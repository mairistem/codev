//! Runs the cases with Claude Code. The pure parts — the change's
//! `change.yaml`, the prompts, the command line, what is left to run, what
//! strayed — come first and are unit-tested; the shell parts below them
//! touch the disk and start processes, and are exercised by real runs.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::Mutex;
use std::time::{Duration, Instant};

use crate::case::{Case, Step};
use crate::observed::{Consumption, FORMAT, Observed, ReviewMode, RunInfo, Tokens};
use crate::suite::{Recorded, RunState};

/// The `change.yaml` the runner writes: every role in `disable` set to
/// `never`, so the evaluated role reviews alone.
pub fn change_yaml(created: &str, skip_specs: bool, disable: &[String]) -> String {
    let mut text = format!("schema: spec-driven\ncreated: {created}\n");
    if skip_specs {
        text.push_str("skip_specs: true\n");
    }
    if !disable.is_empty() {
        text.push_str("roles:\n");
        for role in disable {
            text.push_str(&format!("  {role}: never\n"));
        }
    }
    text
}

/// The roles `codev roles --at <step> --change <name> --json` returns,
/// other than the evaluated one, that can be disabled.
pub fn roles_to_disable(roles_json: &serde_json::Value, evaluated: &str) -> Vec<String> {
    let mut ids: Vec<String> = roles_json
        .get("roles")
        .and_then(|r| r.as_array())
        .into_iter()
        .flatten()
        .filter(|r| r.get("disableable").and_then(|d| d.as_bool()) == Some(true))
        .filter_map(|r| r.get("id").and_then(|i| i.as_str()))
        .filter(|id| *id != evaluated)
        .map(str::to_string)
        .collect();
    ids.sort();
    ids.dedup();
    ids
}

/// The prompt that runs one skill step on the case's change.
pub fn prompt(case: &Case) -> String {
    let change = &case.change;
    match case.step {
        Step::PlanChallenge | Step::CodeReview => {
            let (skill, heading) = if case.step == Step::PlanChallenge {
                ("codev-propose", "7. Challenge the plan — plan.challenge")
            } else {
                ("codev-apply", "5. Review the change — code.review")
            };
            let material = if case.step == Step::PlanChallenge {
                "Its planning artifacts are complete."
            } else {
                "Its tasks are all done and its code is committed on the current branch; its diff is against `main`."
            };
            format!(
                "This is a codev role evaluation, run without a user. The change `{change}` exists. {material}\n\
                 \n\
                 Run only step \"{heading}\" of the `{skill}` skill (.claude/skills/{skill}/SKILL.md) on the change `{change}`: \
                 ask for the roles of the step with `codev roles --at {step} --change {change} --json`, apply every role returned, \
                 and write every finding to the review record `_codev/changes/{change}/review.md` as the skill's review-record \
                 procedure says (resume first, a section per role, each finding with its location and verbatim evidence, then \
                 check the record as the skill says).\n\
                 \n\
                 Evaluation mode: skip the author's answer and the reviewer's re-check — leave every finding `open`. \
                 Modify no file but `_codev/changes/{change}/review.md`: do not fix, simplify or edit the code or the plan. \
                 Ask no question. When the record is written and checked, stop with a one-line summary.",
                step = case.step.as_str(),
            )
        }
        Step::PlanWrite => {
            let request = case.plan.as_ref().map_or("", |p| p.request.as_str());
            format!(
                "Run the `codev-propose` skill (read .claude/skills/codev-propose/SKILL.md and follow it) for this \
                 user request:\n\
                 \n\
                 « {request} »\n\
                 \n\
                 This is a codev evaluation run, without a user: ask no question — wherever the skill says to ask, \
                 make the most reasonable assumption and state it in the proposal. Write the proposal, the delta specs \
                 and the design, then stop: skip tasks.md, the traceability check (plan.trace) and the challenge \
                 (plan.challenge). Modify no file outside `_codev/changes/`. End with a one-line summary."
            )
        }
        Step::PlanUpdate => {
            let (artifact, request) = case
                .update
                .as_ref()
                .map_or(("", ""), |u| (u.artifact.as_str(), u.request.as_str()));
            // The prompt does not ask for the ripple in the final reply: the
            // check reads every message the agent wrote, so it measures what
            // the skill makes the agent report, not what the prompt asks.
            format!(
                "Run the `codev-update` skill (read .claude/skills/codev-update/SKILL.md and follow it) on the \
                 change `{change}`, artifact `{artifact}`, for this request of the author:\n\
                 \n\
                 « {request} »\n\
                 \n\
                 This is a codev evaluation run, without a user: ask no question — wherever the skill says to ask, \
                 make the most reasonable assumption. No one will confirm anything: report a ripple as the skill \
                 says, without applying it. Modify no file outside `_codev/changes/{change}/`."
            )
        }
        Step::CodeImplement => format!(
            "Run the `codev-apply` skill (read .claude/skills/codev-apply/SKILL.md and follow it) on the \
             change `{change}`.\n\
             \n\
             This is a codev evaluation run, without a user: ask no question — wherever the skill says to ask, \
             make the most reasonable assumption. Carry the skill through to its end, its review (code.review) \
             included. Do not archive the change and do not sync it. Cargo works offline: the dependencies are \
             already fetched and built."
        ),
        Step::PlanTrace => format!(
            "This is a codev role evaluation, run without a user. The change `{change}` exists and its planning \
             artifacts are written.\n\
             \n\
             Run only step \"6. Check traceability — plan.trace\" of the `codev-propose` skill \
             (.claude/skills/codev-propose/SKILL.md) on the change `{change}`: ask for the roles of the step with \
             `codev roles --at plan.trace --change {change} --json`, re-read every artifact from disk, check it against \
             the tracer's criteria, and fix the gaps directly in the artifacts, as the step says.\n\
             \n\
             Evaluation mode: modify only files under `_codev/changes/{change}/`; write no review record; touch no code. \
             Ask no question. When done, stop with a one-line summary."
        ),
    }
}

/// How Claude Code is run, from the environment.
#[derive(Debug, Clone, PartialEq)]
pub struct ClaudeSettings {
    pub model: Option<String>,
    /// `CODEV_EVAL_BUDGET_USD`, overriding every step's default.
    pub budget_usd: Option<f64>,
    /// `CODEV_EVAL_TIMEOUT_SECS`, overriding every step's default.
    pub timeout: Option<Duration>,
}

pub const DEFAULT_BUDGET_USD: f64 = 3.0;
pub const DEFAULT_TIMEOUT_SECS: u64 = 600;
/// A planning run writes three artifacts from scratch: about 1.65 USD and
/// several minutes at the median, measured by the study of 44 runs.
pub const PLAN_BUDGET_USD: f64 = 6.0;
pub const PLAN_TIMEOUT_SECS: u64 = 1800;
/// An implementation run writes about a thousand lines of code and builds
/// and tests them several times, then reviews them: many times a planning
/// run, with room left for the slowest model.
pub const IMPLEMENT_BUDGET_USD: f64 = 30.0;
pub const IMPLEMENT_TIMEOUT_SECS: u64 = 5400;

impl ClaudeSettings {
    /// `CODEV_EVAL_MODEL`, `CODEV_EVAL_BUDGET_USD`,
    /// `CODEV_EVAL_TIMEOUT_SECS`; a value that does not parse names its
    /// variable.
    pub fn from_env(env: &dyn Fn(&str) -> Option<String>) -> Result<ClaudeSettings, String> {
        let model = env("CODEV_EVAL_MODEL").filter(|m| !m.trim().is_empty());
        let budget_usd = match env("CODEV_EVAL_BUDGET_USD") {
            None => None,
            Some(text) => text
                .trim()
                .parse::<f64>()
                .ok()
                .filter(|v| *v > 0.0)
                .ok_or_else(|| format!("CODEV_EVAL_BUDGET_USD: `{text}` is not a positive number"))
                .map(Some)?,
        };
        let timeout = match env("CODEV_EVAL_TIMEOUT_SECS") {
            None => None,
            Some(text) => text
                .trim()
                .parse::<u64>()
                .ok()
                .filter(|v| *v > 0)
                .ok_or_else(|| {
                    format!("CODEV_EVAL_TIMEOUT_SECS: `{text}` is not a positive whole number")
                })
                .map(|secs| Some(Duration::from_secs(secs)))?,
        };
        Ok(ClaudeSettings {
            model,
            budget_usd,
            timeout,
        })
    }

    /// The budget of a run at `step`: the environment's, else the step's.
    pub fn budget_for(&self, step: Step) -> f64 {
        self.budget_usd.unwrap_or(match step {
            Step::PlanWrite | Step::PlanUpdate => PLAN_BUDGET_USD,
            Step::CodeImplement => IMPLEMENT_BUDGET_USD,
            _ => DEFAULT_BUDGET_USD,
        })
    }

    /// How long a run at `step` may take: the environment's, else the step's.
    pub fn timeout_for(&self, step: Step) -> Duration {
        self.timeout.unwrap_or(Duration::from_secs(match step {
            Step::PlanWrite | Step::PlanUpdate => PLAN_TIMEOUT_SECS,
            Step::CodeImplement => IMPLEMENT_TIMEOUT_SECS,
            _ => DEFAULT_TIMEOUT_SECS,
        }))
    }
}

/// Claude Code's sandbox, as a user who works with it would set it: every
/// shell command is approved without a prompt — compound ones included —
/// and runs where it can write only in the working directory and the
/// temporary directory, reach no host but the API, and not read the key.
/// `failIfUnavailable`: a machine without the sandbox fails the case
/// rather than run it unconfined. The file tools are outside the sandbox;
/// `acceptEdits` accepts their edits inside the working directory only.
pub const SANDBOX_SETTINGS: &str = r#"{"sandbox":{"enabled":true,"autoAllowBashIfSandboxed":true,"allowUnsandboxedCommands":false,"failIfUnavailable":true,"network":{"allowedDomains":["api.anthropic.com"]},"credentials":{"envVars":["ANTHROPIC_API_KEY"]}}}"#;

/// Tools that reach the network outside the sandbox's reach.
pub const DISALLOWED_TOOLS: &str = "WebFetch WebSearch";

/// What the skills grant in their `allowed-tools`: the sandbox approves
/// only the commands Claude Code knows (`ls`, `git`, `cat`…) without a
/// prompt, and a user's session approves `codev` through the skill. The
/// evaluation's agent reads the skill file instead of invoking the skill,
/// so the runner grants it the same.
pub const ALLOWED_TOOLS: &str = "Bash(codev:*)";

/// The sub-agent tool, granted besides `codev` to a review case in the
/// isolated mode: the skill would grant it in its `allowed-tools`, which
/// the evaluation's agent does not invoke.
pub const SUB_AGENT_TOOL: &str = "Agent";

/// What the review cases are played with: their mode, and a configuration
/// appended to each of them. Planning and update cases ignore both.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ReviewSettings {
    pub mode: ReviewMode,
    /// The text of `--fixture-config`.
    pub fixture_config: Option<String>,
}

impl ReviewSettings {
    /// Whether a case at `step` runs in the isolated mode.
    pub fn isolated_at(&self, step: Step) -> bool {
        self.mode == ReviewMode::Isolated && step.is_review()
    }

    /// Records the settings in a run's `run.json`: the mode only when it is
    /// not the default, so that a run of the default mode reads as before.
    pub fn record(&self, info: &mut RunInfo) {
        info.review_mode = (self.mode == ReviewMode::Isolated).then_some(self.mode);
        info.fixture_config = self.fixture_config.clone();
    }
}

/// The review settings from `--review-mode` and `--fixture-config`, the
/// latter as its path and what reading it gave; an error names the cause:
/// an unknown mode, a file missing or not a YAML mapping, or one setting
/// `review:` while `--review-mode` sets the mode.
pub fn review_settings(
    mode: Option<&str>,
    fixture_config: Option<(&Path, std::io::Result<String>)>,
) -> Result<ReviewSettings, String> {
    let parsed = match mode {
        None => ReviewMode::Inline,
        Some(text) => ReviewMode::parse(text).ok_or_else(|| {
            format!("--review-mode: unknown mode `{text}`; expected `inline` or `isolated`")
        })?,
    };
    let fixture_config = match fixture_config {
        None => None,
        Some((path, read)) => {
            let text = read.map_err(|e| format!("--fixture-config {}: {e}", path.display()))?;
            let value: serde_norway::Value = serde_norway::from_str(&text).map_err(|e| {
                format!("--fixture-config {}: not valid YAML ({e})", path.display())
            })?;
            let Some(mapping) = value.as_mapping() else {
                return Err(format!(
                    "--fixture-config {}: not a YAML mapping of configuration keys",
                    path.display()
                ));
            };
            // The review mode comes from --review-mode alone, so that
            // run.json records the mode the cases ran in.
            if mapping.contains_key("review") {
                return Err(format!(
                    "--fixture-config {}: sets `review:`; give the review mode with --review-mode",
                    path.display()
                ));
            }
            Some(text)
        }
    };
    Ok(ReviewSettings {
        mode: parsed,
        fixture_config,
    })
}

/// A review case's `_codev/config.yaml` with what the settings append:
/// `review: { mode: isolated }` in the isolated mode, then the text of
/// `--fixture-config`; `None` when there is nothing to append.
pub fn config_with_review_settings(existing: &str, review: &ReviewSettings) -> Option<String> {
    if review.mode == ReviewMode::Inline && review.fixture_config.is_none() {
        return None;
    }
    let mut text = existing.to_string();
    let mut append = |part: &str| {
        if !text.is_empty() && !text.ends_with('\n') {
            text.push('\n');
        }
        text.push_str(part);
    };
    if review.mode == ReviewMode::Isolated {
        append("review: { mode: isolated }\n");
    }
    if let Some(extra) = &review.fixture_config {
        append(extra);
    }
    Some(text)
}

/// The `allowed-tools` of a skill's frontmatter, as the skill grants them
/// when a user invokes it; `None` without the key.
pub fn skill_allowed_tools(skill: &str) -> Option<String> {
    let text = skill.replace("\r\n", "\n");
    let front = text.strip_prefix("---\n")?;
    let front = &front[..front.find("\n---")?];
    front.lines().find_map(|line| {
        let value = line.strip_prefix("allowed-tools:")?.trim();
        Some(value.trim_matches('"').to_string())
    })
}

/// `args` with the tools of `--allowedTools` replaced by `tools`.
pub fn with_allowed_tools(mut args: Vec<String>, tools: &str) -> Vec<String> {
    if let Some(at) = args.iter().position(|a| a == "--allowedTools") {
        args[at + 1] = tools.to_string();
    }
    args
}

/// The tools granted without a prompt to a case at `step`.
pub fn allowed_tools(step: Step, review: &ReviewSettings) -> String {
    if review.isolated_at(step) {
        format!("{ALLOWED_TOOLS} {SUB_AGENT_TOOL}")
    } else {
        ALLOWED_TOOLS.to_string()
    }
}

/// What is missing for an isolated case's role to have run alone, from
/// `codev review --json` and the sub-agents its session started: the
/// step's `Mode: isolated.`, no `Fallback:` line, a sub-agent; `None`
/// when nothing is.
pub fn isolation_missing(
    review: Option<&serde_json::Value>,
    step: Step,
    sub_agents_started: u64,
) -> Option<String> {
    let review: crate::observed::Review = review
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .unwrap_or_default();
    let section = review.steps.iter().find(|s| s.step == step.as_str());
    let mut missing = Vec::new();
    match section.and_then(|s| s.mode.as_deref()) {
        Some("isolated") => {}
        Some(other) => missing.push(format!(
            "the review record's step {} says `Mode: {other}.`, not `Mode: isolated.`",
            step.as_str()
        )),
        None => missing.push(format!(
            "the review record's step {} has no `Mode: isolated.` line",
            step.as_str()
        )),
    }
    for role in section.into_iter().flat_map(|s| &s.roles) {
        if let Some(reason) = &role.fallback {
            missing.push(format!(
                "the role {} fell back to the inline mode (`Fallback: {reason}`)",
                role.role
            ));
        }
    }
    if sub_agents_started == 0 {
        missing.push("the session started no sub-agent".to_string());
    }
    (!missing.is_empty()).then(|| {
        format!(
            "the case did not run in the isolated mode: {}",
            missing.join("; ")
        )
    })
}

/// Sets `sub_agents` to the tokens of an isolated run's totals beyond its
/// own calls, kind by kind; a run whose totals fall below its own calls is
/// inconsistent, and an error.
pub fn record_sub_agent_tokens(consumption: &mut Consumption) -> Result<(), String> {
    let Some(totals) = consumption.tokens else {
        return Ok(());
    };
    let own = consumption
        .calls
        .iter()
        .fold(Tokens::default(), |t, c| Tokens {
            input: t.input + c.input,
            cache_creation: t.cache_creation + c.cache_creation,
            cache_read: t.cache_read + c.cache_read,
            output: t.output + c.output,
        });
    let beyond = |kind: &str, total: u64, calls: u64| {
        total.checked_sub(calls).ok_or_else(|| {
            format!(
                "the session's totals hold {total} {kind} tokens, fewer than its own calls' {calls}; \
                 its consumption is inconsistent"
            )
        })
    };
    consumption.sub_agents = Some(Tokens {
        input: beyond("input", totals.input, own.input)?,
        cache_creation: beyond("cache write", totals.cache_creation, own.cache_creation)?,
        cache_read: beyond("cache read", totals.cache_read, own.cache_read)?,
        output: beyond("output", totals.output, own.output)?,
    });
    Ok(())
}

/// The working folder when `--work` is not given: named after the second
/// and the process, so that two runs started in the same second never
/// build their fixtures in the same place.
pub fn default_work_dir(temp: &Path, secs: u64, pid: u32) -> PathBuf {
    temp.join(format!("codev-evals-{secs}-{pid}"))
}

/// The arguments of `claude` for one case.
pub fn claude_args(
    prompt: &str,
    settings: &ClaudeSettings,
    step: Step,
    review: &ReviewSettings,
) -> Vec<String> {
    let mut args: Vec<String> = [
        "-p",
        prompt,
        "--output-format",
        "stream-json",
        "--verbose",
        "--no-session-persistence",
        "--permission-mode",
        "acceptEdits",
        "--settings",
        SANDBOX_SETTINGS,
        "--allowedTools",
        &allowed_tools(step, review),
        "--disallowedTools",
        DISALLOWED_TOOLS,
        "--max-budget-usd",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    args.push(format!("{}", settings.budget_for(step)));
    if let Some(model) = &settings.model {
        args.push("--model".into());
        args.push(model.clone());
    }
    args
}

/// The steps `--step` accepts: `review` for the three review steps, or a
/// step's own name.
pub const STEP_SELECTORS: [&str; 7] = [
    "review",
    "plan.challenge",
    "code.review",
    "plan.trace",
    "plan.write",
    "plan.update",
    "code.implement",
];

/// The cases of the steps named, all of them when none is; an unknown name
/// is an error naming the valid ones.
pub fn select_steps<'a>(cases: &'a [Case], steps: &[String]) -> Result<Vec<&'a Case>, String> {
    if let Some(unknown) = steps.iter().find(|s| !STEP_SELECTORS.contains(&s.as_str())) {
        return Err(format!(
            "--step: unknown step `{unknown}`; expected one of {}",
            STEP_SELECTORS.join(", ")
        ));
    }
    Ok(cases
        .iter()
        .filter(|c| {
            steps.is_empty()
                || steps
                    .iter()
                    .any(|s| s == c.step.as_str() || (s == "review" && c.step.is_review()))
        })
        .collect())
}

/// One run of a case: its repetition `n`, from 1.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Job<'a> {
    pub case: &'a Case,
    pub n: u32,
}

/// The runs still to make for `repeat` repetitions of each case: those
/// without an observation or whose observation is in error, or all of them
/// with `rerun`.
pub fn pending<'a>(
    cases: &'a [Case],
    recorded: &Recorded,
    repeat: u32,
    rerun: bool,
) -> Vec<Job<'a>> {
    cases
        .iter()
        .flat_map(|case| (1..=repeat).map(move |n| Job { case, n }))
        .filter(|job| {
            rerun
                || recorded
                    .get(&(job.case.id.clone(), job.n))
                    .is_none_or(|state| *state == RunState::InError)
        })
        .collect()
}

/// The files `git status --porcelain -z --untracked-files=all` lists that
/// the step does not allow: `review.md` at the review steps, the change's
/// directory at `plan.trace` and `plan.update`, anything under
/// `_codev/changes/` at `plan.write` but a `corrections.md`: no author
/// corrects the agent in an evaluation run, so a correction noted there is
/// one it invented.
pub fn strayed(porcelain: &str, case: &Case) -> Vec<String> {
    let change_dir = format!("{}/", case.change_dir());
    let review = format!("{change_dir}review.md");
    let mut out = Vec::new();
    let mut entries = porcelain.split('\0').filter(|e| !e.is_empty());
    while let Some(entry) = entries.next() {
        if entry.len() < 4 {
            continue;
        }
        let (status, path) = entry.split_at(3);
        if status.starts_with('R') || status.starts_with('C') {
            // A rename is followed by its original path.
            entries.next();
        }
        let allowed = match case.step {
            Step::PlanTrace | Step::PlanUpdate => path.starts_with(&change_dir),
            Step::PlanWrite => path.starts_with(CHANGES_DIR) && !path.ends_with("/corrections.md"),
            // The code is the run's to change; `_codev/` only in its change,
            // which an author's correction never reaches in an evaluation.
            Step::CodeImplement => {
                !path.starts_with("_codev/")
                    || (path.starts_with(&change_dir) && !path.ends_with("/corrections.md"))
            }
            _ => path == review,
        };
        if !allowed {
            out.push(path.to_string());
        }
    }
    out.sort();
    out
}

const CHANGES_DIR: &str = "_codev/changes/";

/// The changes a planning run created: the directories of
/// `_codev/changes/` (archive aside) that hold a file git does not track,
/// with those files, from `git status --porcelain -z --untracked-files=all`.
pub fn created_changes(porcelain: &str) -> BTreeMap<String, Vec<String>> {
    let mut changes: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for entry in porcelain.split('\0').filter(|e| e.len() > 3) {
        let (status, path) = entry.split_at(3);
        if status != "?? " {
            continue;
        }
        let Some((name, _)) = path
            .strip_prefix(CHANGES_DIR)
            .and_then(|rest| rest.split_once('/'))
        else {
            continue;
        };
        if name != "archive" {
            changes
                .entry(name.to_string())
                .or_default()
                .push(path.to_string());
        }
    }
    changes
}

/// What Claude Code's `--output-format json` reports about a run.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ClaudeOutput {
    pub cost_usd: Option<f64>,
    pub duration_ms: Option<u64>,
    pub consumption: Option<Consumption>,
    /// Why the run ended in error, if it did.
    pub error: Option<String>,
    /// The text of every message the agent wrote; empty without the event
    /// stream.
    pub messages: Vec<String>,
    /// The sub-agents the session started: its calls to the sub-agent tool.
    pub sub_agents_started: u64,
    /// The paths the tool calls read or searched, and the shell commands
    /// they ran, the sub-agents' included.
    pub reads: Vec<String>,
}

/// What Claude Code's event stream (`--output-format stream-json`) reports:
/// its final `result` event, read as `--output-format json` prints it, plus
/// the tool calls and every call to the model, read from the events.
pub fn parse_claude_stream(stdout: &str) -> ClaudeOutput {
    let events: Vec<serde_json::Value> = stdout
        .lines()
        .filter_map(|line| serde_json::from_str(line.trim()).ok())
        .collect();
    let Some(result) = events
        .iter()
        .rev()
        .find(|e| e.get("type").and_then(|t| t.as_str()) == Some("result"))
    else {
        return ClaudeOutput {
            error: Some(format!(
                "Claude Code's event stream has no result: {}",
                excerpt(stdout)
            )),
            ..ClaudeOutput::default()
        };
    };
    let mut output = parse_claude_output(&result.to_string());
    output.messages = agent_messages(&events);
    output.reads = crate::implement_run::tool_reads(&events);
    if let Some(consumption) = output.consumption.as_mut() {
        let stream = read_calls(&events);
        consumption.tool_calls = Some(stream.tool_calls);
        consumption.orientation_calls = stream.orientation_calls;
        consumption.calls = stream.calls;
        output.sub_agents_started = stream.sub_agents_started;
    }
    output
}

/// The text of every message the agent wrote, in order: the text blocks
/// of the assistant events, those of one message joined. Tool calls and
/// their outputs are left out, so a path the agent only read does not
/// count as named.
pub fn agent_messages(events: &[serde_json::Value]) -> Vec<String> {
    let mut messages: Vec<(Option<String>, String)> = Vec::new();
    for event in events {
        if event.get("type").and_then(|t| t.as_str()) != Some("assistant") {
            continue;
        }
        let message = &event["message"];
        let id = message
            .get("id")
            .and_then(|i| i.as_str())
            .map(str::to_string);
        let text: Vec<&str> = message["content"]
            .as_array()
            .into_iter()
            .flatten()
            .filter(|b| b.get("type").and_then(|t| t.as_str()) == Some("text"))
            .filter_map(|b| b.get("text").and_then(|t| t.as_str()))
            .collect();
        if text.is_empty() {
            continue;
        }
        let text = text.join("\n");
        match messages.last_mut() {
            Some((last, body)) if id.is_some() && *last == id => {
                body.push('\n');
                body.push_str(&text);
            }
            _ => messages.push((id, text)),
        }
    }
    messages.into_iter().map(|(_, text)| text).collect()
}

/// What the events say about each call to the model.
struct StreamCalls {
    tool_calls: u64,
    sub_agents_started: u64,
    orientation_calls: Option<u64>,
    calls: Vec<crate::observed::Call>,
}

/// The source a tool result is counted under, from the tool call that
/// asked for it.
pub fn source_of(tool: &str, input: &serde_json::Value) -> String {
    let field = |k: &str| input.get(k).and_then(|v| v.as_str()).unwrap_or("");
    match tool {
        "Read" if field("file_path").contains(".claude/skills/") => "skill".into(),
        "Read" | "NotebookRead" => "file".into(),
        "Grep" | "Glob" => "search".into(),
        "Write" | "Edit" | "MultiEdit" | "NotebookEdit" => "edit".into(),
        "Bash" => {
            let command = without_cd(field("command"));
            if command.contains(".claude/skills/") && !command.starts_with("codev ") {
                return "skill".into();
            }
            let mut words = command.split(|c: char| c.is_whitespace() || ";&|)".contains(c));
            match words.next().unwrap_or("") {
                "codev" => format!("codev {}", words.find(|w| !w.is_empty()).unwrap_or("")),
                "grep" | "rg" | "find" | "fd" | "ag" => "search".into(),
                "cat" | "sed" | "head" | "tail" | "less" | "awk" | "nl" => "file".into(),
                _ => "shell".into(),
            }
        }
        other => other.to_lowercase(),
    }
}

/// A command without the `cd <dir> &&` (or `;`) that opens it: what runs
/// is what follows.
fn without_cd(command: &str) -> &str {
    let mut rest = command.trim_start();
    while let Some(after) = rest.strip_prefix("cd ") {
        let Some(at) = after.find(['&', ';']) else {
            break;
        };
        rest = after[at..].trim_start_matches(['&', ';']).trim_start();
    }
    rest
}

fn is_planning_write(tool: &str, input: &serde_json::Value) -> bool {
    matches!(tool, "Write" | "Edit" | "MultiEdit")
        && input
            .get("file_path")
            .and_then(|v| v.as_str())
            .is_some_and(|p| p.contains("_codev/changes/"))
}

/// Groups the assistant events by message — one call to the model may
/// span several events — and shares each call's cache writes among what
/// entered the context since the previous call: the tool results returned
/// in between, by the length of their text, and the previous call's own
/// text. The first call's writes are Claude Code's prompt and the request.
/// Only the session's own calls are read: an event a sub-agent produced
/// names the tool call that started it in `parent_tool_use_id`, and its
/// tokens are counted from the session's totals instead.
fn read_calls(events: &[serde_json::Value]) -> StreamCalls {
    let mut reader = CallReader::default();
    for event in events {
        if event
            .get("parent_tool_use_id")
            .is_some_and(|p| !p.is_null())
        {
            continue;
        }
        match event.get("type").and_then(|t| t.as_str()) {
            Some("assistant") => reader.read_assistant(&event["message"]),
            Some("user") => reader.read_tool_results(&event["message"]),
            _ => {}
        }
    }
    StreamCalls {
        tool_calls: reader.tool_calls,
        sub_agents_started: reader.sub_agents_started,
        orientation_calls: reader.orientation_calls,
        calls: reader.calls,
    }
}

/// Shares a call's cache writes among what entered its context since the
/// previous call, `(source, length)`, in proportion to the length. The
/// first call's writes are Claude Code's own prompt and the request. A
/// later call that received no tool result and follows no text still
/// wrote something: what the previous call produced and Claude Code
/// carries forward (its tool calls, its thinking), so it goes to `output`.
fn share_writes(writes: u64, entered: &[(String, usize)], first: bool) -> BTreeMap<String, u64> {
    let mut sources = BTreeMap::new();
    let total: usize = entered.iter().map(|(_, len)| *len).sum();
    if first {
        sources.insert("prompt".to_string(), writes);
    } else if total == 0 {
        sources.insert("output".to_string(), writes);
    } else {
        for (source, len) in entered {
            let share = (writes as f64 * *len as f64 / total as f64).round() as u64;
            *sources.entry(source.clone()).or_insert(0) += share;
        }
    }
    sources
}

/// The length of a message's text, whether a string or a list of blocks.
fn text_len(value: &serde_json::Value) -> usize {
    match value {
        serde_json::Value::String(s) => s.len(),
        serde_json::Value::Array(items) => items
            .iter()
            .map(|i| i.get("text").and_then(|t| t.as_str()).map_or(0, str::len))
            .sum(),
        _ => 0,
    }
}

/// The state of a stream being read, one event at a time.
#[derive(Default)]
struct CallReader {
    calls: Vec<crate::observed::Call>,
    /// The message id of the call being read: one call spans several
    /// events with the same id.
    last_id: Option<String>,
    /// The source of each tool call, by its id, for its result.
    sources_of_tools: BTreeMap<String, String>,
    /// What entered since the previous call: (source, length).
    pending: Vec<(String, usize)>,
    /// The length of the text the current call produced.
    previous_text: usize,
    tool_calls: u64,
    sub_agents_started: u64,
    orientation_calls: Option<u64>,
}

impl CallReader {
    /// An assistant event: starts a call when its message id is new, then
    /// records its usage and reads its blocks.
    fn read_assistant(&mut self, message: &serde_json::Value) {
        let id = message
            .get("id")
            .and_then(|i| i.as_str())
            .map(str::to_string);
        let usage = &message["usage"];
        let get = |k: &str| usage.get(k).and_then(|v| v.as_u64()).unwrap_or(0);
        if id.is_none() || id != self.last_id {
            self.start_call(get("cache_creation_input_tokens"));
            self.last_id = id;
        }
        let call = self.calls.last_mut().expect("a call was started");
        // The last event of a message carries its final usage.
        call.cache_read = get("cache_read_input_tokens");
        call.input = get("input_tokens");
        call.output = call.output.max(get("output_tokens"));
        for block in message["content"].as_array().into_iter().flatten() {
            self.read_block(block);
        }
    }

    fn start_call(&mut self, writes: u64) {
        let mut entered = std::mem::take(&mut self.pending);
        if self.previous_text > 0 {
            entered.push(("output".into(), self.previous_text));
        }
        let sources = share_writes(writes, &entered, self.calls.is_empty());
        self.calls.push(crate::observed::Call {
            cache_creation: writes,
            sources,
            ..crate::observed::Call::default()
        });
        self.previous_text = 0;
    }

    /// One block of the current call: a tool call, or text it produced.
    fn read_block(&mut self, block: &serde_json::Value) {
        match block.get("type").and_then(|t| t.as_str()) {
            Some("tool_use") => {
                let tool = block.get("name").and_then(|n| n.as_str()).unwrap_or("");
                let input = &block["input"];
                if self.orientation_calls.is_none() && is_planning_write(tool, input) {
                    self.orientation_calls = Some(self.tool_calls);
                }
                self.tool_calls += 1;
                if tool == SUB_AGENT_TOOL {
                    self.sub_agents_started += 1;
                }
                if let Some(call) = self.calls.last_mut() {
                    call.tool_calls += 1;
                }
                if let Some(tid) = block.get("id").and_then(|i| i.as_str()) {
                    self.sources_of_tools
                        .insert(tid.to_string(), source_of(tool, input));
                }
            }
            Some("text") => self.previous_text += text_len(&block["text"]),
            Some("thinking") => self.previous_text += text_len(&block["thinking"]),
            _ => {}
        }
    }

    /// A user event: the tool results it returns enter the next call.
    fn read_tool_results(&mut self, message: &serde_json::Value) {
        for block in message["content"].as_array().into_iter().flatten() {
            if block.get("type").and_then(|t| t.as_str()) != Some("tool_result") {
                continue;
            }
            let source = block
                .get("tool_use_id")
                .and_then(|i| i.as_str())
                .and_then(|i| self.sources_of_tools.get(i))
                .cloned()
                .unwrap_or_else(|| "other".into());
            self.pending.push((source, text_len(&block["content"])));
        }
    }
}

/// Reads one `result` document, as `--output-format json` prints it: cost,
/// duration, what the run consumed, and whether it ended in error.
pub fn parse_claude_output(stdout: &str) -> ClaudeOutput {
    let Ok(value) = serde_json::from_str::<serde_json::Value>(stdout.trim()) else {
        return ClaudeOutput {
            error: Some(format!(
                "Claude Code's output is not JSON: {}",
                excerpt(stdout)
            )),
            ..ClaudeOutput::default()
        };
    };
    let cost = value.get("total_cost_usd").and_then(|v| v.as_f64());
    let duration = value.get("duration_ms").and_then(|v| v.as_u64());
    let error = (value.get("is_error").and_then(|v| v.as_bool()) == Some(true)).then(|| {
        let subtype = value
            .get("subtype")
            .and_then(|v| v.as_str())
            .unwrap_or("error");
        let result = value.get("result").and_then(|v| v.as_str()).unwrap_or("");
        format!(
            "Claude Code reported an error ({subtype}): {}",
            excerpt(result)
        )
    });
    ClaudeOutput {
        cost_usd: cost,
        duration_ms: duration,
        consumption: Some(consumption(&value)),
        error,
        messages: Vec::new(),
        sub_agents_started: 0,
        reads: Vec::new(),
    }
}

/// The tokens, summed over `modelUsage` — every model the run used, the
/// housekeeping one included, as the cost is — or read from `usage` when
/// `modelUsage` is absent; the turns, the API duration and the denials.
fn consumption(value: &serde_json::Value) -> Consumption {
    let u64_at = |v: &serde_json::Value, key: &str| v.get(key).and_then(|x| x.as_u64());
    let by_model = value.get("modelUsage").and_then(|m| m.as_object());
    let (tokens, models) = match by_model {
        Some(models) if !models.is_empty() => {
            let mut sum = Tokens::default();
            for usage in models.values() {
                sum.input += u64_at(usage, "inputTokens").unwrap_or(0);
                sum.cache_creation += u64_at(usage, "cacheCreationInputTokens").unwrap_or(0);
                sum.cache_read += u64_at(usage, "cacheReadInputTokens").unwrap_or(0);
                sum.output += u64_at(usage, "outputTokens").unwrap_or(0);
            }
            (Some(sum), models.keys().cloned().collect())
        }
        _ => {
            let tokens = value.get("usage").map(|usage| Tokens {
                input: u64_at(usage, "input_tokens").unwrap_or(0),
                cache_creation: u64_at(usage, "cache_creation_input_tokens").unwrap_or(0),
                cache_read: u64_at(usage, "cache_read_input_tokens").unwrap_or(0),
                output: u64_at(usage, "output_tokens").unwrap_or(0),
            });
            (tokens, Vec::new())
        }
    };
    let mut models: Vec<String> = models;
    models.sort();
    Consumption {
        duration_api_ms: u64_at(value, "duration_api_ms"),
        turns: u64_at(value, "num_turns"),
        permission_denials: value
            .get("permission_denials")
            .and_then(|d| d.as_array())
            .map(|d| d.len() as u64),
        models,
        tokens,
        ..Consumption::default()
    }
}

fn excerpt(text: &str) -> String {
    let text = text.trim();
    let cut: String = text.chars().take(300).collect();
    if cut.len() < text.len() {
        format!("{cut}…")
    } else {
        cut
    }
}

/// The civil date of a Unix day number (Howard Hinnant's algorithm), so
/// the runner needs no date crate.
pub fn civil_date(days_since_epoch: i64) -> String {
    let z = days_since_epoch + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = yoe + era * 400 + i64::from(month <= 2);
    format!("{year:04}-{month:02}-{day:02}")
}

// ---------------------------------------------------------------------
// Shell: files, git, codev, Claude Code.
// ---------------------------------------------------------------------

/// Where the tools are.
#[derive(Debug, Clone)]
pub struct Tools {
    pub claude: PathBuf,
    pub codev: PathBuf,
    pub git: PathBuf,
}

/// The first line of `<tool> --version`, or an error naming the tool.
fn version(tool: &Path, name: &str) -> Result<String, String> {
    let output = Command::new(tool)
        .arg("--version")
        .env("CODEV_NO_UPDATE_CHECK", "1")
        .output()
        .map_err(|e| format!("cannot execute `{name}` ({}): {e}", tool.display()))?;
    if !output.status.success() {
        return Err(format!(
            "`{name} --version` failed ({}): {}",
            tool.display(),
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    Ok(String::from_utf8_lossy(&output.stdout)
        .lines()
        .next()
        .unwrap_or("")
        .trim()
        .to_string())
}

/// Checks each tool runs before any case; the versions go to `run.json`.
pub fn preflight(tools: &Tools) -> Result<(String, String), String> {
    let claude = version(&tools.claude, "claude")?;
    version(&tools.git, "git")?;
    let codev = version(&tools.codev, "codev")?;
    let runs = |program: &str, arg: &str| {
        Command::new(program)
            .arg(arg)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .is_ok_and(|s| s.success())
    };
    if let Some(missing) = sandbox_missing(std::env::consts::OS, &runs) {
        return Err(missing);
    }
    Ok((claude, codev))
}

/// What Claude Code's sandbox needs and this machine lacks, by operating
/// system, so that a run is refused once rather than every case failing
/// one by one; `None` when nothing is missing. `runs(program, arg)` says
/// whether the program runs.
pub fn sandbox_missing(os: &str, runs: &dyn Fn(&str, &str) -> bool) -> Option<String> {
    match os {
        "linux" => {
            let missing: Vec<&str> = [
                ("bwrap", "--version", "bubblewrap"),
                ("socat", "-V", "socat"),
            ]
            .into_iter()
            .filter(|(program, arg, _)| !runs(program, arg))
            .map(|(_, _, package)| package)
            .collect();
            (!missing.is_empty()).then(|| {
                format!(
                    "Claude Code's sandbox needs {} on Linux (`sudo apt-get install bubblewrap socat`, \
                     and on Ubuntu 24.04 the AppArmor profile for bwrap, see CONTRIBUTING.md); \
                     the suite does not run the agent without it",
                    missing.join(" and ")
                )
            })
        }
        "macos" => None,
        other => Some(format!(
            "Claude Code's sandbox does not run on {other}; run the suite on macOS, Linux or WSL2"
        )),
    }
}

/// Reads a directory into a map of relative `/`-separated paths to bytes.
pub fn read_tree(root: &Path) -> std::io::Result<BTreeMap<String, Vec<u8>>> {
    let mut files = BTreeMap::new();
    if !root.exists() {
        return Ok(files);
    }
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        for entry in fs::read_dir(&dir)? {
            let entry = entry?;
            let path = entry.path();
            if entry.file_type()?.is_dir() {
                stack.push(path);
            } else {
                let relative = path
                    .strip_prefix(root)
                    .expect("under the root")
                    .components()
                    .map(|c| c.as_os_str().to_string_lossy().into_owned())
                    .collect::<Vec<_>>()
                    .join("/");
                files.insert(relative, fs::read(&path)?);
            }
        }
    }
    Ok(files)
}

fn write_tree(root: &Path, files: &BTreeMap<String, Vec<u8>>) -> std::io::Result<()> {
    for (path, bytes) in files {
        let target = root.join(path);
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(target, bytes)?;
    }
    Ok(())
}

/// A command the runner starts itself, outside the sandbox: once the agent
/// has run, the case's directory is untrusted, so the command gets neither
/// the API key nor any git configuration but the repository's own.
fn tool_command(tool: &Path, dir: &Path, codev_dir: &Path) -> Command {
    let mut command = Command::new(tool);
    command
        .current_dir(dir)
        .env("CODEV_NO_UPDATE_CHECK", "1")
        .env("PATH", path_with(codev_dir))
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", null_device())
        .env_remove("ANTHROPIC_API_KEY")
        .stdin(Stdio::null());
    command
}

fn null_device() -> &'static str {
    if cfg!(windows) { "NUL" } else { "/dev/null" }
}

fn run_tool(tool: &Path, args: &[&str], dir: &Path, codev_dir: &Path) -> Result<String, String> {
    let output = tool_command(tool, dir, codev_dir)
        .args(args)
        .output()
        .map_err(|e| format!("cannot run {}: {e}", tool.display()))?;
    if !output.status.success() {
        return Err(format!(
            "`{} {}` failed: {}",
            tool.display(),
            args.join(" "),
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

/// `PATH` with the directory of the `codev` under test first, so the
/// skills call the binary built from this tree.
fn path_with(dir: &Path) -> std::ffi::OsString {
    let mut paths = vec![dir.to_path_buf()];
    if let Some(existing) = std::env::var_os("PATH") {
        paths.extend(std::env::split_paths(&existing));
    }
    std::env::join_paths(paths).unwrap_or_default()
}

/// The arguments of every git call: no hook, no file-system monitor — the
/// two settings of a repository's configuration that make git run a
/// program — and a fixed identity.
pub fn git_args<'a>(args: &[&'a str]) -> Vec<&'a str> {
    let mut all = vec![
        "-c",
        "user.name=codev-evals",
        "-c",
        "user.email=evals@codev.invalid",
        "-c",
        "commit.gpgsign=false",
        "-c",
        "core.hooksPath=.git/no-hooks",
        "-c",
        "core.fsmonitor=false",
    ];
    all.extend_from_slice(args);
    all
}

fn git(tools: &Tools, dir: &Path, args: &[&str]) -> Result<String, String> {
    run_tool(&tools.git, &git_args(args), dir, codev_dir(tools))
}

/// Why a run cannot be trusted when its agent changed the repository's
/// configuration, which git would read afterwards outside the sandbox.
pub fn config_tampered(before: &[u8], after: Option<&[u8]>) -> Option<String> {
    (after != Some(before)).then(|| {
        "the run modified .git/config, which the runner reads outside the sandbox; \
         its observation is not trusted"
            .to_string()
    })
}

fn codev_dir(tools: &Tools) -> &Path {
    tools.codev.parent().unwrap_or(Path::new("."))
}

/// Builds the case's project in `dir`: `base/` on `main` with codev
/// initialized — its configuration with what `review` appends, the skills
/// rendered again for it —, `change/` on the branch `change`, the
/// evaluated role alone.
pub fn setup(
    case: &Case,
    case_dir: &Path,
    dir: &Path,
    tools: &Tools,
    date: &str,
    review: &ReviewSettings,
) -> Result<(), String> {
    let io = |e: std::io::Error| format!("setting up the fixture: {e}");
    if dir.exists() {
        fs::remove_dir_all(dir).map_err(io)?;
    }
    fs::create_dir_all(dir).map_err(io)?;
    write_tree(dir, &read_tree(&case_dir.join("base")).map_err(io)?).map_err(io)?;
    git(tools, dir, &["init", "-q", "-b", "main"])?;
    run_tool(
        &tools.codev,
        &[
            "init",
            "--yes",
            "--no-detect",
            "--no-integration",
            "--preset",
            "full",
            "--language",
            "en",
            ".",
        ],
        dir,
        codev_dir(tools),
    )?;
    let config_path = dir.join("_codev").join("config.yaml");
    let config = fs::read_to_string(&config_path).map_err(io)?;
    if let Some(text) = config_with_review_settings(&config, review) {
        fs::write(&config_path, text).map_err(io)?;
        run_tool(&tools.codev, &["update", "--force"], dir, codev_dir(tools))?;
    }
    git(tools, dir, &["add", "-A"])?;
    git(tools, dir, &["commit", "-q", "-m", "base"])?;
    git(tools, dir, &["switch", "-q", "-c", "change"])?;
    let change = read_tree(&case_dir.join("change")).map_err(io)?;
    write_tree(dir, &change).map_err(io)?;

    let change_dir = dir.join(case.change_dir());
    let skip_specs = !change_dir.join("specs").exists();
    // Write a first change.yaml so `codev roles` resolves the change, then
    // the final one with the other roles turned off.
    fs::write(
        change_dir.join("change.yaml"),
        change_yaml(date, skip_specs, &[]),
    )
    .map_err(io)?;
    let roles = run_tool(
        &tools.codev,
        &[
            "roles",
            "--at",
            case.step.as_str(),
            "--change",
            &case.change,
            "--json",
        ],
        dir,
        codev_dir(tools),
    )?;
    let roles: serde_json::Value =
        serde_json::from_str(&roles).map_err(|e| format!("`codev roles` output: {e}"))?;
    let disable = roles_to_disable(&roles, &case.role);
    fs::write(
        change_dir.join("change.yaml"),
        change_yaml(date, skip_specs, &disable),
    )
    .map_err(io)?;
    git(tools, dir, &["add", "-A"])?;
    git(tools, dir, &["commit", "-q", "-m", "change"])?;
    Ok(())
}

/// Builds a planning or update case's project in `dir`: the tree of `repo`
/// at the case's `ref` — for an update case, with its `change/` written to
/// `_codev/changes/<change>/`, the change as it stood while active — one
/// commit, then the skills the codev under test renders (`codev update
/// --force`, over skills edited since), committed.
pub fn setup_plan(case: &Case, repo: &Path, dir: &Path, tools: &Tools) -> Result<(), String> {
    let io = |e: std::io::Error| format!("setting up the fixture: {e}");
    let git_ref = case
        .plan
        .as_ref()
        .map(|p| p.git_ref.as_str())
        .or_else(|| case.update.as_ref().map(|u| u.git_ref.as_str()))
        .or_else(|| case.implement.as_ref().map(|i| i.git_ref.as_str()))
        .ok_or_else(|| "not a planning, update or implementation case".to_string())?;
    if dir.exists() {
        fs::remove_dir_all(dir).map_err(io)?;
    }
    fs::create_dir_all(dir).map_err(io)?;
    let archive = dir.with_extension("tar");
    let archive_arg = archive.to_string_lossy().into_owned();
    git(
        tools,
        repo,
        &["archive", "--format=tar", "-o", &archive_arg, git_ref],
    )?;
    let untar = Command::new("tar")
        .arg("-xf")
        .arg(&archive)
        .arg("-C")
        .arg(dir)
        .output()
        .map_err(|e| format!("cannot run tar: {e}"));
    let _ = fs::remove_file(&archive);
    let untar = untar?;
    if !untar.status.success() {
        return Err(format!(
            "extracting `{git_ref}`: {}",
            String::from_utf8_lossy(&untar.stderr).trim()
        ));
    }
    let laid = case
        .update
        .as_ref()
        .map(|u| &u.files)
        .or(case.implement.as_ref().map(|i| &i.files));
    if let Some(files) = laid {
        let change: BTreeMap<String, Vec<u8>> = files
            .iter()
            .map(|(path, text)| (path.clone(), text.clone().into_bytes()))
            .collect();
        write_tree(&dir.join(case.change_dir()), &change).map_err(io)?;
    }
    git(tools, dir, &["init", "-q", "-b", "main"])?;
    git(tools, dir, &["add", "-A"])?;
    git(
        tools,
        dir,
        &["commit", "-q", "-m", &format!("state at {git_ref}")],
    )?;
    run_tool(&tools.codev, &["update", "--force"], dir, codev_dir(tools))?;
    git(tools, dir, &["add", "-A"])?;
    git(
        tools,
        dir,
        &[
            "commit",
            "-q",
            "--allow-empty",
            "-m",
            "skills of the codev under test",
        ],
    )?;
    Ok(())
}

/// Runs Claude Code in `dir` with a timeout; returns its standard output.
fn run_claude(
    dir: &Path,
    tools: &Tools,
    args: &[String],
    timeout: Duration,
    log: &Path,
    extra_env: &[(String, String)],
) -> Result<String, String> {
    let stdout_path = log.join("claude.jsonl");
    let stderr_path = log.join("claude.stderr");
    let stdout = fs::File::create(&stdout_path).map_err(|e| e.to_string())?;
    let stderr = fs::File::create(&stderr_path).map_err(|e| e.to_string())?;
    let mut command = Command::new(&tools.claude);
    command
        .args(args)
        .current_dir(dir)
        .env("CODEV_NO_UPDATE_CHECK", "1")
        .env("PATH", path_with(codev_dir(tools)))
        .envs(extra_env.iter().map(|(k, v)| (k, v)))
        .stdin(Stdio::null())
        .stdout(stdout)
        .stderr(stderr);
    #[cfg(unix)]
    {
        // Its own process group, so a timeout stops the shells and tools it
        // spawned too.
        use std::os::unix::process::CommandExt;
        command.process_group(0);
    }
    let mut child = command
        .spawn()
        .map_err(|e| format!("cannot start claude: {e}"))?;
    let started = Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                let out = fs::read_to_string(&stdout_path).unwrap_or_default();
                // Claude Code exits non-zero on a budget or turn limit and
                // still prints its JSON, whose `is_error` the caller reads:
                // only an exit without output is a failure here.
                if !status.success() && out.trim().is_empty() {
                    let err = fs::read_to_string(&stderr_path).unwrap_or_default();
                    return Err(format!("claude exited with {status}: {}", excerpt(&err)));
                }
                return Ok(out);
            }
            Ok(None) if started.elapsed() > timeout => {
                terminate(&mut child);
                return Err(format!("timeout after {} s", timeout.as_secs()));
            }
            Ok(None) => std::thread::sleep(Duration::from_millis(500)),
            Err(e) => return Err(format!("waiting for claude: {e}")),
        }
    }
}

pub(crate) fn terminate(child: &mut std::process::Child) {
    #[cfg(unix)]
    {
        let group = format!("-{}", child.id());
        let _ = Command::new("kill").args(["-TERM", "--", &group]).status();
        let deadline = Instant::now() + Duration::from_secs(5);
        while Instant::now() < deadline {
            if let Ok(Some(_)) = child.try_wait() {
                let _ = Command::new("kill").args(["-KILL", "--", &group]).status();
                return;
            }
            std::thread::sleep(Duration::from_millis(200));
        }
        let _ = Command::new("kill").args(["-KILL", "--", &group]).status();
    }
    let _ = child.kill();
    let _ = child.wait();
}

/// A stream without its `system/init` event: the session's settings, whose
/// paths name the operator's home directory, and nothing a run is
/// measured on.
pub fn without_init_event(stream: &str) -> String {
    stream
        .lines()
        .filter(|line| {
            serde_json::from_str::<serde_json::Value>(line).map_or(true, |event| {
                !(event["type"] == "system" && event["subtype"] == "init")
            })
        })
        .map(|line| format!("{line}\n"))
        .collect()
}

/// A `codev validate --json` report without its `root`: the absolute path
/// of the case's directory on the machine that ran it, which the scoring
/// does not read.
pub fn without_root(mut report: serde_json::Value) -> serde_json::Value {
    if let Some(object) = report.as_object_mut() {
        object.remove("root");
    }
    report
}

/// The JSON document a `codev … --json` command prints in `dir`. codev
/// exits with 1 when what it reports holds an error — an invalid plan, a
/// review record with an error — and still prints its document, which is
/// the observation: only output that is not JSON is an error.
fn codev_json(tools: &Tools, dir: &Path, args: &[&str]) -> Result<serde_json::Value, String> {
    let command = format!("codev {}", args.join(" "));
    let out = tool_command(&tools.codev, dir, codev_dir(tools))
        .args(args)
        .output()
        .map_err(|e| format!("cannot run `{command}`: {e}"))?;
    serde_json::from_slice(&out.stdout).map_err(|e| format!("`{command}` output: {e}"))
}

/// Runs one repetition of a case in `work_dir`, its working copy, set up
/// from the fixture in `case_dir`; returns its observation and never fails:
/// a failure is the observation's `error`.
#[allow(clippy::too_many_arguments)]
pub fn run_case(
    case: &Case,
    repo: &Path,
    case_dir: &Path,
    work_dir: &Path,
    log: &Path,
    tools: &Tools,
    settings: &ClaudeSettings,
    date: &str,
    review: &ReviewSettings,
) -> Observed {
    let mut observed = Observed {
        format: FORMAT,
        case: case.id.clone(),
        error: None,
        cost_usd: None,
        duration_ms: None,
        consumption: None,
        strayed: Vec::new(),
        review: None,
        files: BTreeMap::new(),
        validate: BTreeMap::new(),
        validate_before: None,
        messages: Vec::new(),
        implement: None,
    };
    if let Err(e) = fs::create_dir_all(log) {
        observed.error = Some(format!("cannot create {}: {e}", log.display()));
        return observed;
    }
    let built = if case.step.replays_history() {
        setup_plan(case, repo, work_dir, tools)
    } else {
        setup(case, case_dir, work_dir, tools, date, review)
    };
    if let Err(e) = built {
        observed.error = Some(e);
        return observed;
    }
    let implementing = case.step == Step::CodeImplement;
    if implementing {
        let observed = run_implementation(case, repo, work_dir, log, tools, settings, observed);
        crate::implement_run::remove_build_outputs(work_dir);
        return observed;
    }
    if case.step == Step::PlanUpdate {
        // The errors the archived change already has under the codev being
        // measured: the run is judged only on those it adds.
        match codev_json(tools, work_dir, &["validate", &case.change, "--json"]) {
            Ok(value) => observed.validate_before = Some(without_root(value)),
            Err(e) => {
                observed.error = Some(e);
                return observed;
            }
        }
    }
    let config_before = fs::read(work_dir.join(".git").join("config")).unwrap_or_default();
    let args = claude_args(&prompt(case), settings, case.step, review);
    let isolated = review.isolated_at(case.step);
    let mut sub_agents_started = 0;
    match run_claude(
        work_dir,
        tools,
        &args,
        settings.timeout_for(case.step),
        log,
        &[],
    ) {
        Err(e) => observed.error = Some(e),
        Ok(stdout) => {
            let output = parse_claude_stream(&stdout);
            // The stream is kept for reading a run again; its first event,
            // which names the operator's home directory, is not needed.
            let _ = fs::write(log.join("claude.jsonl"), without_init_event(&stdout));
            observed.cost_usd = output.cost_usd;
            observed.duration_ms = output.duration_ms;
            observed.consumption = output.consumption;
            observed.error = output.error;
            sub_agents_started = output.sub_agents_started;
            if isolated
                && observed.error.is_none()
                && let Some(consumption) = observed.consumption.as_mut()
                && let Err(e) = record_sub_agent_tokens(consumption)
            {
                observed.error = Some(e);
            }
            // Only the update score reads the agent's messages; keeping them
            // for the other steps would only make `observed.json` larger.
            if case.step == Step::PlanUpdate {
                observed.messages = output.messages;
            }
        }
    }
    if observed.error.is_none() {
        let after = fs::read(work_dir.join(".git").join("config")).ok();
        observed.error = config_tampered(&config_before, after.as_deref());
    }
    if observed.error.is_some() {
        return observed;
    }
    let status = git(
        tools,
        work_dir,
        &["status", "--porcelain", "-z", "--untracked-files=all"],
    )
    .unwrap_or_default();
    observed.strayed = strayed(&status, case);
    if case.step == Step::PlanWrite {
        for (change, files) in created_changes(&status) {
            for path in files {
                let content = fs::read_to_string(work_dir.join(&path)).ok();
                observed.files.insert(path, content);
            }
            match codev_json(tools, work_dir, &["validate", &change, "--json"]) {
                Ok(value) => {
                    observed.validate.insert(change, without_root(value));
                }
                Err(e) => observed.error = Some(e),
            }
        }
    }
    if case.step == Step::PlanUpdate {
        observed.files = change_files(case, work_dir);
        match codev_json(tools, work_dir, &["validate", &case.change, "--json"]) {
            Ok(value) => {
                observed
                    .validate
                    .insert(case.change.clone(), without_root(value));
            }
            Err(e) => observed.error = Some(e),
        }
    }
    if case.step.records_findings() {
        match codev_json(
            tools,
            work_dir,
            &["review", "--change", &case.change, "--json"],
        ) {
            Ok(value) => observed.review = Some(value),
            Err(e) => observed.error = Some(e),
        }
        if let Ok(record) = fs::read(work_dir.join(case.change_dir()).join("review.md")) {
            let _ = fs::write(log.join("review.md"), record);
        }
        // The skill falls back to the inline mode by design (decision
        // 0007): an isolated case that did is not scored as one.
        if isolated && observed.error.is_none() {
            observed.error =
                isolation_missing(observed.review.as_ref(), case.step, sub_agents_started);
        }
    }
    for path in case.checked_paths() {
        let content = fs::read_to_string(work_dir.join(&path)).ok();
        observed.files.insert(path, content);
    }
    observed
}

/// An implementation run, once its fixture is built: the build prepared,
/// Claude Code on `/codev-apply`, then the runner's own checks, the binary
/// and the acceptance script. The caller removes the build outputs, error
/// or not.
fn run_implementation(
    case: &Case,
    repo: &Path,
    work_dir: &Path,
    log: &Path,
    tools: &Tools,
    settings: &ClaudeSettings,
    mut observed: Observed,
) -> Observed {
    use crate::implement_run as build;
    if let Err(e) = build::prepare(work_dir, log) {
        observed.error = Some(e);
        return observed;
    }
    // The errors the archived change already has under the codev being
    // measured: the run is judged only on those it adds.
    match codev_json(tools, work_dir, &["validate", &case.change, "--json"]) {
        Ok(value) => observed.validate_before = Some(without_root(value)),
        Err(e) => {
            observed.error = Some(e);
            return observed;
        }
    }
    let config_before = fs::read(work_dir.join(".git").join("config")).unwrap_or_default();
    // An implementation run grants what the installed `codev-apply` skill
    // grants a user who invokes it — the shell, to build and test, and the
    // sub-agent tool of an isolated review —: the agent reads the skill
    // file instead of invoking it.
    let skill = fs::read_to_string(
        work_dir
            .join(".claude")
            .join("skills")
            .join("codev-apply")
            .join("SKILL.md"),
    )
    .unwrap_or_default();
    let mut args = claude_args(
        &prompt(case),
        settings,
        case.step,
        &ReviewSettings::default(),
    );
    if let Some(tools) = skill_allowed_tools(&skill) {
        args = with_allowed_tools(args, &tools);
    }
    let mut reads = Vec::new();
    match run_claude(
        work_dir,
        tools,
        &args,
        settings.timeout_for(case.step),
        log,
        &build::agent_env(work_dir),
    ) {
        Err(e) => observed.error = Some(e),
        Ok(stdout) => {
            let output = parse_claude_stream(&stdout);
            let _ = fs::write(log.join("claude.jsonl"), without_init_event(&stdout));
            observed.cost_usd = output.cost_usd;
            observed.duration_ms = output.duration_ms;
            observed.consumption = output.consumption;
            observed.error = output.error;
            observed.messages = output.messages;
            reads = output.reads;
        }
    }
    if observed.error.is_none() {
        let after = fs::read(work_dir.join(".git").join("config")).ok();
        observed.error = config_tampered(&config_before, after.as_deref());
    }
    if observed.error.is_some() {
        return observed;
    }
    let status = git(
        tools,
        work_dir,
        &["status", "--porcelain", "-z", "--untracked-files=all"],
    )
    .unwrap_or_default();
    observed.strayed = strayed(&status, case);
    observed.files = change_files(case, work_dir);
    for (args, key) in [
        (vec!["validate", case.change.as_str(), "--json"], "validate"),
        (
            vec!["review", "--change", case.change.as_str(), "--json"],
            "review",
        ),
    ] {
        match codev_json(tools, work_dir, &args) {
            Ok(value) if key == "validate" => {
                observed
                    .validate
                    .insert(case.change.clone(), without_root(value));
            }
            Ok(value) => observed.review = Some(value),
            Err(e) => {
                observed.error = Some(e);
                return observed;
            }
        }
    }
    if let Ok(record) = fs::read(work_dir.join(case.change_dir()).join("review.md")) {
        let _ = fs::write(log.join("review.md"), record);
    }
    let roots: Vec<PathBuf> = [repo.to_path_buf()]
        .into_iter()
        .chain(fs::canonicalize(repo).ok())
        .collect();
    let mut implement = crate::observed::ImplementObserved {
        changed: build::changed_paths(&status),
        read_repository: build::reads_under(&reads, &roots),
        ..Default::default()
    };
    let refuse = case.implement.as_ref().is_some_and(|i| i.refuse);
    if !refuse {
        implement.checks = build::run_checks(work_dir, log);
        let built = implement
            .checks
            .iter()
            .any(|c| c.name == "build" && c.passed);
        let script = case.implement.as_ref().and_then(|i| i.script.as_deref());
        if let (true, Some(script)) = (built, script) {
            match build::run_acceptance(script, &build::built_binary(work_dir), log) {
                Ok((printed, end)) => {
                    implement.printed = printed;
                    implement.script = Some(end);
                }
                Err(e) => {
                    observed.error = Some(e);
                    return observed;
                }
            }
        }
    }
    observed.implement = Some(implement);
    observed
}

/// The files of an update case's change folder after the run, by path from
/// the project root; a file the case laid and the run removed is `None`.
fn change_files(case: &Case, work_dir: &Path) -> BTreeMap<String, Option<String>> {
    let change_dir = case.change_dir();
    let mut files: BTreeMap<String, Option<String>> = read_tree(&work_dir.join(&change_dir))
        .unwrap_or_default()
        .into_iter()
        .map(|(path, bytes)| {
            (
                format!("{change_dir}/{path}"),
                Some(String::from_utf8_lossy(&bytes).into_owned()),
            )
        })
        .collect();
    let laid = case
        .update
        .iter()
        .flat_map(|u| u.files.keys())
        .chain(case.implement.iter().flat_map(|i| i.files.keys()));
    for path in laid {
        files.entry(format!("{change_dir}/{path}")).or_insert(None);
    }
    files
}

/// Everything a suite run needs.
pub struct RunOptions {
    pub repo: PathBuf,
    pub out: PathBuf,
    pub work: PathBuf,
    pub keep: bool,
    pub jobs: usize,
    pub tools: Tools,
    pub settings: ClaudeSettings,
    pub review: ReviewSettings,
}

/// Whether a run's error says the account's usage limit is reached: every
/// run after it would fail the same way, at once.
pub fn is_usage_limit(error: &str) -> bool {
    let error = error.to_lowercase();
    error.contains("session limit") || error.contains("usage limit")
}

/// The runs left to make, shared by the jobs; once stopped, it hands out
/// no more.
pub struct Queue<'a> {
    jobs: Mutex<Vec<Job<'a>>>,
    stopped: std::sync::atomic::AtomicBool,
}

impl<'a> Queue<'a> {
    pub fn new(pending: &[Job<'a>]) -> Self {
        Queue {
            jobs: Mutex::new(pending.iter().rev().copied().collect()),
            stopped: std::sync::atomic::AtomicBool::new(false),
        }
    }

    /// The next run, in the order given; `None` once empty or stopped.
    pub fn pop(&self) -> Option<Job<'a>> {
        if self.is_stopped() {
            return None;
        }
        self.jobs.lock().expect("queue lock").pop()
    }

    pub fn stop(&self) {
        self.stopped
            .store(true, std::sync::atomic::Ordering::SeqCst);
    }

    pub fn is_stopped(&self) -> bool {
        self.stopped.load(std::sync::atomic::Ordering::SeqCst)
    }
}

/// Runs the repetitions `jobs` at a time, each case's repetition `n` set up
/// in `<work>/<case>-<n>` and logged in `<out>/<case>/<n>/`, writing each
/// `observed.json` as soon as it is known, so an interrupted run keeps what
/// it measured. Returns `true` when a run hit the account's usage limit,
/// after which no other run started.
pub fn run_cases(pending: &[Job], options: &RunOptions, date: &str) -> bool {
    let queue = Queue::new(pending);
    std::thread::scope(|scope| {
        for _ in 0..options.jobs.max(1) {
            let queue = &queue;
            scope.spawn(move || {
                while let Some(Job { case, n }) = queue.pop() {
                    eprintln!("[codev-evals] {} #{n} — running", case.id);
                    let case_dir = options.repo.join("evals").join("cases").join(&case.id);
                    let log = options.out.join(&case.id).join(n.to_string());
                    let work = options.work.join(format!("{}-{n}", case.id));
                    let observed = run_case(
                        case,
                        &options.repo,
                        &case_dir,
                        &work,
                        &log,
                        &options.tools,
                        &options.settings,
                        date,
                        &options.review,
                    );
                    let text = serde_json::to_string_pretty(&observed).expect("serializes") + "\n";
                    if let Err(e) = fs::write(log.join("observed.json"), text) {
                        eprintln!(
                            "[codev-evals] {} #{n} — cannot write observed.json: {e}",
                            case.id
                        );
                    }
                    eprintln!(
                        "[codev-evals] {} #{n} — {}",
                        case.id,
                        observed.error.as_deref().unwrap_or("done")
                    );
                    if observed.error.as_deref().is_some_and(is_usage_limit) {
                        queue.stop();
                    }
                    if !options.keep {
                        let _ = fs::remove_dir_all(&work);
                    }
                }
            });
        }
    });
    queue.is_stopped()
}

/// Refuses to resume a run recorded with another model, whose observations
/// would be scored under the first model's name, or with another Claude
/// Code version, whose runs would be credited to the wrong version.
pub fn check_resume(
    recorded: &RunInfo,
    model: Option<&str>,
    claude_version: Option<&str>,
) -> Result<(), String> {
    let name = |m: Option<&str>| m.unwrap_or("Claude Code's default").to_string();
    if recorded.model.as_deref() != model {
        return Err(format!(
            "this output directory was recorded with the model {}, and CODEV_EVAL_MODEL asks for {}; \
             use another --out directory, or the same model",
            name(recorded.model.as_deref()),
            name(model)
        ));
    }
    if let (Some(recorded_version), Some(installed)) =
        (recorded.claude_version.as_deref(), claude_version)
        && recorded_version != installed
    {
        return Err(format!(
            "this output directory was recorded with {recorded_version}, and {installed} is installed; \
             runs added to it would be credited to the wrong version: install the same Claude Code, or use \
             another --out directory"
        ));
    }
    Ok(())
}

/// Refuses to resume a run recorded in another review mode or with another
/// appended configuration: its observations would be scored as one run. A
/// `run.json` that records neither reads as `inline` with nothing appended.
pub fn check_resume_review(recorded: &RunInfo, review: &ReviewSettings) -> Result<(), String> {
    let mode = recorded.review_mode_or_default();
    if mode != review.mode {
        return Err(format!(
            "this output directory was recorded in the {} review mode, and --review-mode asks for {}; \
             use another --out directory, or the same mode",
            mode.as_str(),
            review.mode.as_str()
        ));
    }
    if recorded.fixture_config != review.fixture_config {
        let name = |text: &Option<String>| {
            text.as_deref()
                .map_or_else(|| "none".to_string(), |t| format!("`{}`", t.trim()))
        };
        return Err(format!(
            "this output directory was recorded with the appended configuration {}, and \
             --fixture-config gives {}; use another --out directory, or the same file",
            name(&recorded.fixture_config),
            name(&review.fixture_config)
        ));
    }
    Ok(())
}

/// Writes `run.json` unless the output directory already has one (a
/// resumed run keeps its date and versions).
pub fn ensure_run_info(out: &Path, info: &RunInfo) -> std::io::Result<()> {
    let path = out.join("run.json");
    if path.exists() {
        return Ok(());
    }
    fs::create_dir_all(out)?;
    fs::write(
        path,
        serde_json::to_string_pretty(info).expect("serializes") + "\n",
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::case::Kind;
    use crate::score::tests::a_case;
    use serde_json::json;

    #[test]
    fn change_yaml_disables_every_other_reviewing_role() {
        let roles = json!({"roles": [
            {"id": "newcomer", "disableable": true},
            {"id": "security", "disableable": true},
            {"id": "developer", "disableable": false}
        ]});
        let disable = roles_to_disable(&roles, "security");
        assert_eq!(disable, ["newcomer"]);
        assert_eq!(
            change_yaml("2026-10-13", true, &disable),
            "schema: spec-driven\ncreated: 2026-10-13\nskip_specs: true\nroles:\n  newcomer: never\n"
        );
        assert_eq!(
            change_yaml("2026-10-13", false, &[]),
            "schema: spec-driven\ncreated: 2026-10-13\n"
        );
    }

    #[test]
    fn the_review_prompt_names_the_skill_step_and_the_evaluation_mode() {
        let mut case = a_case("security", Kind::Seeded, &[]);
        case.step = Step::PlanChallenge;
        case.change = "add-sessions".into();
        let text = prompt(&case);
        for needle in [
            "`codev-propose`",
            "plan.challenge",
            "`add-sessions`",
            "leave every finding `open`",
            "Modify no file but `_codev/changes/add-sessions/review.md`",
        ] {
            assert!(text.contains(needle), "missing {needle} in {text}");
        }
        assert!(
            !text.contains("--json`)"),
            "the prompt leaves the record's check to the skill: {text}"
        );
        case.step = Step::CodeReview;
        assert!(prompt(&case).contains("`codev-apply`"));
        assert!(prompt(&case).contains("code.review"));
    }

    #[test]
    fn the_trace_prompt_asks_to_fix_the_plan() {
        let mut case = a_case("tracer", Kind::Seeded, &[]);
        case.step = Step::PlanTrace;
        let text = prompt(&case);
        assert!(text.contains("plan.trace") && text.contains("fix the gaps directly"));
        assert!(text.contains("write no review record"));
    }

    #[test]
    fn the_claude_command_takes_model_and_budget_from_the_environment() {
        let env = |name: &str| match name {
            "CODEV_EVAL_MODEL" => Some("claude-sonnet-4-5".to_string()),
            "CODEV_EVAL_BUDGET_USD" => Some("1.5".to_string()),
            _ => None,
        };
        let settings = ClaudeSettings::from_env(&env).unwrap();
        let args = claude_args("p", &settings, Step::CodeReview, &ReviewSettings::default());
        let after = |flag: &str| {
            args.iter()
                .position(|a| a == flag)
                .map(|i| args[i + 1].clone())
        };
        assert_eq!(after("--model").as_deref(), Some("claude-sonnet-4-5"));
        assert_eq!(after("--max-budget-usd").as_deref(), Some("1.5"));
        assert_eq!(after("-p").as_deref(), Some("p"));

        let defaults = ClaudeSettings::from_env(&|_| None).unwrap();
        let args = claude_args("p", &defaults, Step::CodeReview, &ReviewSettings::default());
        assert!(!args.iter().any(|a| a == "--model"));
        let budget = args.iter().position(|a| a == "--max-budget-usd").unwrap();
        assert_eq!(args[budget + 1], "3");
        assert_eq!(
            defaults.timeout_for(Step::CodeReview),
            Duration::from_secs(600)
        );

        let bad = |name: &str| (name == "CODEV_EVAL_TIMEOUT_SECS").then(|| "soon".to_string());
        assert!(
            ClaudeSettings::from_env(&bad)
                .unwrap_err()
                .contains("CODEV_EVAL_TIMEOUT_SECS")
        );
    }

    #[test]
    fn git_runs_no_hook_and_no_file_system_monitor() {
        let args = git_args(&["status"]);
        let config: Vec<&str> = args
            .windows(2)
            .filter(|w| w[0] == "-c")
            .map(|w| w[1])
            .collect();
        assert!(config.contains(&"core.hooksPath=.git/no-hooks"), "{args:?}");
        assert!(config.contains(&"core.fsmonitor=false"), "{args:?}");
        assert_eq!(args.last(), Some(&"status"));
    }

    #[test]
    fn the_runner_starts_its_tools_without_the_key_or_a_global_configuration() {
        let command = tool_command(Path::new("git"), Path::new("."), Path::new("."));
        let envs: Vec<(String, Option<String>)> = command
            .get_envs()
            .map(|(k, v)| {
                (
                    k.to_string_lossy().into_owned(),
                    v.map(|v| v.to_string_lossy().into_owned()),
                )
            })
            .collect();
        assert!(
            envs.contains(&("ANTHROPIC_API_KEY".into(), None)),
            "{envs:?}"
        );
        assert!(envs.contains(&("GIT_CONFIG_NOSYSTEM".into(), Some("1".into()))));
        assert!(
            envs.iter()
                .any(|(k, v)| k == "GIT_CONFIG_GLOBAL" && v.is_some())
        );
    }

    #[test]
    fn a_modified_git_config_is_an_error() {
        let before = b"[core]\n\tbare = false\n";
        assert_eq!(config_tampered(before, Some(before)), None);
        let after = b"[core]\n\tbare = false\n\tfsmonitor = curl evil\n";
        assert!(
            config_tampered(before, Some(after))
                .unwrap()
                .contains(".git/config")
        );
        assert!(
            config_tampered(before, None).is_some(),
            "a removed config too"
        );
    }

    #[test]
    fn claude_runs_in_the_sandbox() {
        let args = claude_args(
            "p",
            &ClaudeSettings::from_env(&|_| None).unwrap(),
            Step::CodeReview,
            &ReviewSettings::default(),
        );
        let after = |flag: &str| {
            args.iter()
                .position(|a| a == flag)
                .map(|i| args[i + 1].clone())
        };
        assert_eq!(
            after("--allowedTools").as_deref(),
            Some("Bash(codev:*)"),
            "exactly what the skills grant"
        );
        assert!(!args.iter().any(|a| a.contains("bypassPermissions")));
        assert_eq!(after("--permission-mode").as_deref(), Some("acceptEdits"));
        assert_eq!(
            after("--disallowedTools").as_deref(),
            Some("WebFetch WebSearch")
        );
        let settings: serde_json::Value =
            serde_json::from_str(&after("--settings").unwrap()).unwrap();
        let sandbox = &settings["sandbox"];
        assert_eq!(sandbox["enabled"], true);
        assert_eq!(sandbox["autoAllowBashIfSandboxed"], true);
        assert_eq!(sandbox["allowUnsandboxedCommands"], false);
        assert_eq!(sandbox["failIfUnavailable"], true);
        assert_eq!(
            sandbox["network"]["allowedDomains"],
            json!(["api.anthropic.com"])
        );
        assert_eq!(
            sandbox["credentials"]["envVars"],
            json!(["ANTHROPIC_API_KEY"])
        );
    }

    fn cases(ids: &[&str]) -> Vec<Case> {
        ids.iter()
            .map(|id| {
                let mut c = a_case("security", Kind::Control, &[]);
                c.id = (*id).into();
                c
            })
            .collect()
    }

    fn jobs(pending: &[Job]) -> Vec<String> {
        pending
            .iter()
            .map(|j| format!("{}#{}", j.case.id, j.n))
            .collect()
    }

    #[test]
    fn a_resumed_run_runs_only_unrecorded_cases() {
        let cases = cases(&["a", "b", "c", "d", "e"]);
        let recorded: Recorded = ["a", "c", "e"]
            .iter()
            .map(|s| ((s.to_string(), 1), RunState::Done))
            .collect();
        assert_eq!(jobs(&pending(&cases, &recorded, 1, false)), ["b#1", "d#1"]);
        assert_eq!(pending(&cases, &recorded, 1, true).len(), 5);
    }

    #[test]
    fn each_case_is_repeated() {
        let cases = cases(&["a", "b"]);
        let all = pending(&cases, &Recorded::new(), 3, false);
        assert_eq!(jobs(&all), ["a#1", "a#2", "a#3", "b#1", "b#2", "b#3"]);
    }

    #[test]
    fn pending_runs_only_missing_repetitions() {
        let cases = cases(&["a"]);
        let recorded: Recorded = [
            (("a".to_string(), 1), RunState::Done),
            (("a".to_string(), 2), RunState::Done),
        ]
        .into();
        assert_eq!(
            jobs(&pending(&cases, &recorded, 5, false)),
            ["a#3", "a#4", "a#5"]
        );
    }

    #[test]
    fn a_repetition_in_error_is_pending() {
        let cases = cases(&["ops-code-no-timeout"]);
        let recorded: Recorded = [
            (("ops-code-no-timeout".to_string(), 1), RunState::Done),
            (("ops-code-no-timeout".to_string(), 2), RunState::InError),
            (("ops-code-no-timeout".to_string(), 3), RunState::Done),
        ]
        .into();
        assert_eq!(
            jobs(&pending(&cases, &recorded, 3, false)),
            ["ops-code-no-timeout#2"]
        );
    }

    #[test]
    fn a_resumed_run_with_another_model_is_refused() {
        let recorded = RunInfo {
            format: FORMAT,
            model: None,
            date: "2026-10-13".into(),
            claude_version: None,
            codev_version: None,
            repeat: None,
            jobs: None,
            review_mode: None,
            fixture_config: None,
        };
        assert!(check_resume(&recorded, None, None).is_ok());
        let err = check_resume(&recorded, Some("claude-sonnet-4-5"), None).unwrap_err();
        assert!(
            err.contains("Claude Code's default") && err.contains("claude-sonnet-4-5"),
            "{err}"
        );
    }

    #[test]
    fn files_modified_outside_the_step_are_strayed() {
        let case = a_case("security", Kind::Seeded, &[]);
        let status =
            " M src/users.ts\0?? _codev/changes/add-search/review.md\0R  src/new.ts\0src/old.ts\0";
        assert_eq!(strayed(status, &case), ["src/new.ts", "src/users.ts"]);
        let mut trace = case.clone();
        trace.step = Step::PlanTrace;
        let status = " M _codev/changes/add-search/tasks.md\0";
        assert!(strayed(status, &trace).is_empty());
    }

    #[test]
    fn claude_output_gives_cost_duration_and_errors() {
        let ok = r#"{"type":"result","subtype":"success","is_error":false,"duration_ms":81234,"total_cost_usd":0.42,"result":"done"}"#;
        let output = parse_claude_output(ok);
        assert_eq!(
            (output.cost_usd, output.duration_ms, output.error),
            (Some(0.42), Some(81234), None)
        );
        let budget = r#"{"subtype":"error_max_budget_usd","is_error":true,"total_cost_usd":3.01}"#;
        assert!(
            parse_claude_output(budget)
                .error
                .unwrap()
                .contains("error_max_budget_usd")
        );
        assert!(
            parse_claude_output("oops")
                .error
                .unwrap()
                .contains("not JSON")
        );
    }

    /// The figures of the baseline's `security-code-sql-injection` run.
    const SQL_INJECTION_OUTPUT: &str = r#"{"type":"result","subtype":"success","is_error":false,
        "duration_ms":28004,"duration_api_ms":27747,"num_turns":9,"total_cost_usd":0.3511748,
        "usage":{"input_tokens":14,"cache_creation_input_tokens":29349,"cache_read_input_tokens":304094,"output_tokens":2713},
        "modelUsage":{
          "claude-haiku-4-5-20251001":{"inputTokens":1173,"outputTokens":15,"cacheReadInputTokens":0,"cacheCreationInputTokens":0},
          "claude-opus-5-5":{"inputTokens":14,"outputTokens":2713,"cacheReadInputTokens":304094,"cacheCreationInputTokens":29349}},
        "permission_denials":[{"tool_name":"Bash"}],"result":"done"}"#;

    #[test]
    fn claude_output_gives_consumption() {
        let c = parse_claude_output(SQL_INJECTION_OUTPUT)
            .consumption
            .unwrap();
        assert_eq!(c.turns, Some(9));
        assert_eq!(c.duration_api_ms, Some(27747));
        assert_eq!(c.permission_denials, Some(1));
        assert_eq!(
            c.tokens,
            Some(Tokens {
                input: 1187,
                cache_creation: 29349,
                cache_read: 304094,
                output: 2728,
            })
        );
        assert_eq!(c.models, ["claude-haiku-4-5-20251001", "claude-opus-5-5"]);
    }

    #[test]
    fn consumption_sums_every_model_or_falls_back_on_usage() {
        let only_usage = r#"{"is_error":false,"num_turns":2,
            "usage":{"input_tokens":3,"cache_creation_input_tokens":4,"cache_read_input_tokens":5,"output_tokens":6}}"#;
        let c = parse_claude_output(only_usage).consumption.unwrap();
        assert_eq!(
            c.tokens,
            Some(Tokens {
                input: 3,
                cache_creation: 4,
                cache_read: 5,
                output: 6,
            })
        );
        assert!(c.models.is_empty());
    }

    #[test]
    fn a_missing_usage_is_absent_not_zero() {
        let c = parse_claude_output(r#"{"is_error":true,"subtype":"error_during_execution"}"#)
            .consumption
            .unwrap();
        assert_eq!(c.tokens, None);
        assert_eq!(c.turns, None);
        assert_eq!(c.permission_denials, None);
        assert_eq!(parse_claude_output("oops").consumption, None);
    }

    #[test]
    fn civil_dates_are_right() {
        assert_eq!(civil_date(0), "1970-01-01");
        assert_eq!(civil_date(20_739), "2026-10-13");
        assert_eq!(civil_date(11_016), "2000-02-29");
    }

    #[test]
    fn a_machine_without_the_sandbox_is_refused_once() {
        let none = |_: &str, _: &str| false;
        let all = |_: &str, _: &str| true;
        let err = sandbox_missing("linux", &none).unwrap();
        assert!(err.contains("bubblewrap and socat"), "{err}");
        let only_bwrap = |p: &str, _: &str| p == "bwrap";
        let err = sandbox_missing("linux", &only_bwrap).unwrap();
        assert!(
            err.contains("needs socat") && !err.contains("bubblewrap and"),
            "{err}"
        );
        assert_eq!(sandbox_missing("linux", &all), None);
        assert_eq!(sandbox_missing("macos", &none), None);
        assert!(sandbox_missing("windows", &all).unwrap().contains("WSL2"));
    }

    #[test]
    fn preflight_names_a_missing_tool() {
        let tools = Tools {
            claude: PathBuf::from("codev-evals-no-such-claude"),
            codev: PathBuf::from("codev-evals-no-such-codev"),
            git: PathBuf::from("git"),
        };
        let err = preflight(&tools).unwrap_err();
        assert!(err.contains("`claude`"), "{err}");
    }

    /// An assistant event of message `id`, with its usage and content.
    fn assistant(id: &str, writes: u64, output: u64, content: serde_json::Value) -> String {
        json!({"type": "assistant", "message": {"id": id, "content": content,
            "usage": {"input_tokens": 2, "cache_creation_input_tokens": writes,
                      "cache_read_input_tokens": 1000, "output_tokens": output}}})
        .to_string()
    }

    fn tool_use(id: &str, name: &str, input: serde_json::Value) -> serde_json::Value {
        json!({"type": "tool_use", "id": id, "name": name, "input": input})
    }

    fn tool_result(id: &str, chars: usize) -> String {
        json!({"type": "user", "message": {"content": [
            {"type": "tool_result", "tool_use_id": id, "content": "x".repeat(chars)}]}})
        .to_string()
    }

    const RESULT: &str = r#"{"type":"result","subtype":"success","is_error":false,"duration_ms":1000,"num_turns":3,"total_cost_usd":0.1,"usage":{"input_tokens":1,"cache_creation_input_tokens":1,"cache_read_input_tokens":1,"output_tokens":1}}"#;

    #[test]
    fn a_calls_writes_are_shared_by_length_among_its_sources() {
        let stream = [
            assistant(
                "m1",
                9000,
                10,
                json!([
                    tool_use(
                        "t1",
                        "Bash",
                        json!({"command": "codev roles --at code.review --json"})
                    ),
                    tool_use("t2", "Read", json!({"file_path": "/p/src/users.ts"})),
                ]),
            ),
            tool_result("t1", 9000),
            tool_result("t2", 3000),
            assistant("m2", 6000, 5, json!([{"type": "text", "text": "Done."}])),
            RESULT.to_string(),
        ]
        .join("\n");
        let c = parse_claude_stream(&stream).consumption.unwrap();
        assert_eq!(c.calls.len(), 2);
        assert_eq!(c.calls[1].sources["codev roles"], 4500);
        assert_eq!(c.calls[1].sources["file"], 1500);
    }

    #[test]
    fn the_first_call_goes_to_the_prompt() {
        let stream = [
            assistant("m1", 9696, 3, json!([{"type": "text", "text": "Hi"}])),
            RESULT.to_string(),
        ]
        .join("\n");
        let c = parse_claude_stream(&stream).consumption.unwrap();
        assert_eq!(
            c.calls[0].sources,
            BTreeMap::from([("prompt".to_string(), 9696)])
        );
    }

    #[test]
    fn each_call_is_recorded() {
        // One call spread over two events of the same message, then another.
        let stream = [
            assistant(
                "m1",
                500,
                4,
                json!([{"type": "thinking", "thinking": "hm"}]),
            ),
            assistant(
                "m1",
                500,
                9,
                json!([tool_use("t1", "Grep", json!({"pattern": "x"}))]),
            ),
            tool_result("t1", 100),
            assistant("m2", 80, 7, json!([{"type": "text", "text": "ok"}])),
            RESULT.to_string(),
        ]
        .join("\n");
        let c = parse_claude_stream(&stream).consumption.unwrap();
        assert_eq!(c.calls.len(), 2);
        assert_eq!(
            (
                c.calls[0].cache_creation,
                c.calls[0].output,
                c.calls[0].tool_calls
            ),
            (500, 9, 1)
        );
        assert_eq!(c.calls[0].cache_read, 1000);
        assert!(
            c.calls[1].sources.contains_key("search"),
            "{:?}",
            c.calls[1].sources
        );
    }

    #[test]
    fn a_codev_command_is_named_by_its_subcommand() {
        assert_eq!(
            source_of("Bash", &json!({"command": "codev review --change x"})),
            "codev review"
        );
        assert_eq!(
            source_of("Bash", &json!({"command": "codev validate; git status"})),
            "codev validate"
        );
        assert_eq!(
            source_of(
                "Bash",
                &json!({"command": "cd \"$PWD\" && codev roles --json"})
            ),
            "codev roles"
        );
        assert_eq!(
            source_of("Bash", &json!({"command": "grep -rn x _codev/specs"})),
            "search"
        );
        assert_eq!(
            source_of("Bash", &json!({"command": "cd /tmp/x; sed -n 1,80p a.md"})),
            "file"
        );
        assert_eq!(source_of("Bash", &json!({"command": "ls _codev"})), "shell");
        assert_eq!(
            source_of("Bash", &json!({"command": "codev --version&&ls"})),
            "codev --version"
        );
        assert_eq!(
            source_of(
                "Bash",
                &json!({"command": "cat .claude/skills/codev-apply/SKILL.md"})
            ),
            "skill"
        );
        assert_eq!(
            source_of(
                "Read",
                &json!({"file_path": "/p/.claude/skills/codev-apply/SKILL.md"})
            ),
            "skill"
        );
        assert_eq!(
            source_of("Bash", &json!({"command": "git diff main"})),
            "shell"
        );
        assert_eq!(source_of("Edit", &json!({"file_path": "/p/a.rs"})), "edit");
    }

    #[test]
    fn a_stream_gives_the_result_tool_calls_and_orientation() {
        let mut lines = Vec::new();
        for i in 0..27 {
            let input = if i == 11 {
                json!({"file_path": "/p/_codev/changes/add-search/proposal.md", "content": "x"})
            } else {
                json!({"file_path": "/p/src/a.rs"})
            };
            let tool = if i == 11 { "Write" } else { "Read" };
            lines.push(assistant(
                &format!("m{i}"),
                10,
                1,
                json!([tool_use(&format!("t{i}"), tool, input)]),
            ));
            lines.push(tool_result(&format!("t{i}"), 10));
        }
        lines.push(RESULT.to_string());
        let out = parse_claude_stream(&lines.join("\n"));
        assert_eq!(out.cost_usd, Some(0.1));
        let c = out.consumption.unwrap();
        assert_eq!(c.tool_calls, Some(27));
        assert_eq!(c.orientation_calls, Some(11));
    }

    #[test]
    fn a_stream_without_a_write_has_no_orientation() {
        let stream = [
            assistant(
                "m1",
                10,
                1,
                json!([tool_use("t1", "Read", json!({"file_path": "/p/a.rs"}))]),
            ),
            tool_result("t1", 10),
            RESULT.to_string(),
        ]
        .join("\n");
        let c = parse_claude_stream(&stream).consumption.unwrap();
        assert_eq!(c.tool_calls, Some(1));
        assert_eq!(c.orientation_calls, None);
    }

    #[test]
    fn a_stream_without_a_result_is_an_error() {
        let stream = assistant("m1", 10, 1, json!([]));
        let out = parse_claude_stream(&stream);
        assert!(out.error.unwrap().contains("no result"));
        assert_eq!(out.consumption, None);
    }

    #[test]
    fn a_resumed_run_with_another_claude_code_is_refused() {
        let recorded = RunInfo {
            format: FORMAT,
            model: None,
            date: "2026-10-15".into(),
            claude_version: Some("2.1.289 (Claude Code)".into()),
            codev_version: None,
            repeat: None,
            jobs: None,
            review_mode: None,
            fixture_config: None,
        };
        assert!(check_resume(&recorded, None, Some("2.1.289 (Claude Code)")).is_ok());
        let err = check_resume(&recorded, None, Some("2.1.301 (Claude Code)")).unwrap_err();
        assert!(err.contains("2.1.289") && err.contains("2.1.301"), "{err}");
    }

    #[test]
    fn cases_are_selected_by_step() {
        let mut review = a_case("security", Kind::Seeded, &[]);
        review.id = "r".into();
        let mut plan = a_case("planner", Kind::Seeded, &[]);
        plan.id = "p".into();
        plan.step = Step::PlanWrite;
        let cases = vec![review, plan];
        let ids = |steps: &[&str]| -> Vec<String> {
            select_steps(
                &cases,
                &steps.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
            )
            .unwrap()
            .iter()
            .map(|c| c.id.clone())
            .collect()
        };
        assert_eq!(ids(&["plan.write"]), ["p"]);
        assert_eq!(ids(&["review"]), ["r"]);
        assert_eq!(ids(&[]), ["r", "p"]);
        let err = select_steps(&cases, &["plan.writ".to_string()]).unwrap_err();
        assert!(err.contains("plan.writ") && err.contains("plan.write") && err.contains("review"));
    }

    fn planning_case(request: &str) -> Case {
        let mut case = a_case("planner", Kind::Seeded, &[]);
        case.step = Step::PlanWrite;
        case.plan = Some(crate::case::PlanCase {
            git_ref: "abc123".into(),
            request: request.into(),
            modified: Vec::new(),
            collateral: Vec::new(),
            decisions: Vec::new(),
        });
        case
    }

    #[test]
    fn the_planning_prompt_names_the_skill_the_request_and_the_mode() {
        let text = prompt(&planning_case(
            "Je voudrais qu'une décision naisse en brouillon",
        ));
        assert!(text.contains("`codev-propose`"), "{text}");
        assert!(text.contains("« Je voudrais qu'une décision naisse en brouillon »"));
        assert!(text.contains("ask no question"));
        assert!(
            text.contains("skip tasks.md")
                && text.contains("plan.trace")
                && text.contains("plan.challenge")
        );
        assert!(text.contains("Modify no file outside `_codev/changes/`"));
    }

    #[test]
    fn a_planning_case_has_its_own_budget_and_timeout() {
        let defaults = ClaudeSettings::from_env(&|_| None).unwrap();
        let budget = |args: Vec<String>| {
            let i = args.iter().position(|a| a == "--max-budget-usd").unwrap();
            args[i + 1].clone()
        };
        assert_eq!(
            budget(claude_args(
                "p",
                &defaults,
                Step::PlanWrite,
                &ReviewSettings::default()
            )),
            "6"
        );
        assert_eq!(
            defaults.timeout_for(Step::PlanWrite),
            Duration::from_secs(1800)
        );
        assert_eq!(
            budget(claude_args(
                "p",
                &defaults,
                Step::PlanChallenge,
                &ReviewSettings::default()
            )),
            "3"
        );
        assert_eq!(
            defaults.timeout_for(Step::PlanChallenge),
            Duration::from_secs(600)
        );

        let env = |name: &str| match name {
            "CODEV_EVAL_BUDGET_USD" => Some("2".to_string()),
            "CODEV_EVAL_TIMEOUT_SECS" => Some("900".to_string()),
            _ => None,
        };
        let set = ClaudeSettings::from_env(&env).unwrap();
        assert_eq!(
            budget(claude_args(
                "p",
                &set,
                Step::PlanWrite,
                &ReviewSettings::default()
            )),
            "2"
        );
        assert_eq!(set.timeout_for(Step::PlanWrite), Duration::from_secs(900));
    }

    #[test]
    fn an_invented_correction_is_modified_outside_the_step() {
        let case = planning_case("r");
        let status =
            "?? _codev/changes/draft/proposal.md\0?? _codev/changes/draft/corrections.md\0";
        assert_eq!(
            strayed(status, &case),
            ["_codev/changes/draft/corrections.md"]
        );
    }

    #[test]
    fn a_planning_run_strays_outside_the_changes() {
        let case = planning_case("r");
        let status = "?? _codev/changes/draft/proposal.md\0?? _codev/changes/draft/specs/a/spec.md\0 M crates/codev-core/src/lib.rs\0";
        assert_eq!(strayed(status, &case), ["crates/codev-core/src/lib.rs"]);
        let created = created_changes(status);
        assert_eq!(created.keys().collect::<Vec<_>>(), ["draft"]);
        assert_eq!(created["draft"].len(), 2);
        assert!(
            created_changes(" M _codev/changes/old/tasks.md\0?? _codev/changes/archive/x/a.md\0")
                .is_empty()
        );
    }

    fn update_case(change: &str, artifact: crate::case::Artifact, request: &str) -> Case {
        let mut case = a_case("planner", Kind::Seeded, &[]);
        case.id = "update-x".into();
        case.step = Step::PlanUpdate;
        case.change = change.into();
        case.update = Some(crate::case::UpdateCase {
            git_ref: "abc123".into(),
            artifact,
            request: request.into(),
            revised: vec!["design.md".into()],
            kept: vec![],
            ripple: vec![],
            contains: vec![],
            refuse: false,
            files: BTreeMap::from([("design.md".to_string(), "# Design\n".to_string())]),
        });
        case
    }

    #[test]
    fn the_update_prompt_names_the_skill_and_the_request() {
        let text = prompt(&update_case(
            "add-auth",
            crate::case::Artifact::Design,
            "remplace la décision X par Y",
        ));
        for needle in [
            "`codev-update`",
            ".claude/skills/codev-update/SKILL.md",
            "change `add-auth`",
            "artifact `design`",
            "« remplace la décision X par Y »",
            "ask no question",
            "report a ripple as the skill says, without applying it",
            "Modify no file outside `_codev/changes/add-auth/`",
        ] {
            assert!(text.contains(needle), "missing {needle} in {text}");
        }
        assert!(
            !text.to_lowercase().contains("summary"),
            "the prompt asks for no summary of the ripple: {text}"
        );
    }

    #[test]
    fn an_update_run_that_edits_the_code_strays() {
        let case = update_case("add-auth", crate::case::Artifact::Design, "r");
        let status = " M _codev/changes/add-auth/design.md\0?? _codev/changes/add-auth/notes.md\0 M src/lib.rs\0?? _codev/changes/other/proposal.md\0";
        assert_eq!(
            strayed(status, &case),
            ["_codev/changes/other/proposal.md", "src/lib.rs"]
        );
    }

    #[test]
    fn review_does_not_select_update_cases() {
        let mut review = a_case("security", Kind::Seeded, &[]);
        review.id = "r".into();
        let update = update_case("add-auth", crate::case::Artifact::Design, "r");
        let cases = vec![review, update];
        let ids = |steps: &[&str]| -> Vec<String> {
            select_steps(
                &cases,
                &steps.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
            )
            .unwrap()
            .iter()
            .map(|c| c.id.clone())
            .collect()
        };
        assert_eq!(ids(&["review"]), ["r"]);
        assert_eq!(ids(&["plan.update"]), ["update-x"]);
        assert_eq!(ids(&["plan.write"]), Vec::<String>::new());
    }

    #[test]
    fn an_update_case_runs_under_the_planning_budget_and_timeout() {
        let defaults = ClaudeSettings::from_env(&|_| None).unwrap();
        assert_eq!(defaults.budget_for(Step::PlanUpdate), PLAN_BUDGET_USD);
        assert_eq!(
            defaults.timeout_for(Step::PlanUpdate),
            Duration::from_secs(PLAN_TIMEOUT_SECS)
        );
    }

    #[test]
    fn the_agent_messages_leave_out_tool_calls_and_outputs() {
        let stream = [
            assistant(
                "m1",
                10,
                1,
                json!([{"type": "text", "text": "I read the change."},
                       tool_use("t1", "Read", json!({"file_path": "/p/specs/doctor/spec.md"}))]),
            ),
            json!({"type": "user", "message": {"content": [
                {"type": "tool_result", "tool_use_id": "t1", "content": "specs/doctor/spec.md"}]}})
            .to_string(),
            assistant(
                "m2",
                10,
                1,
                json!([{"type": "thinking", "thinking": "specs/tasks.md"}]),
            ),
            assistant(
                "m2",
                10,
                1,
                json!([{"type": "text", "text": "Ripple: `specs/doctor/spec.md` is orphaned."}]),
            ),
            RESULT.to_string(),
        ]
        .join("\n");
        let out = parse_claude_stream(&stream);
        assert_eq!(
            out.messages,
            [
                "I read the change.",
                "Ripple: `specs/doctor/spec.md` is orphaned."
            ]
        );
    }

    #[test]
    fn a_usage_limit_is_recognized() {
        assert!(is_usage_limit(
            "Claude Code reported an error (success): You've hit your session limit · resets 5:40pm"
        ));
        assert!(is_usage_limit("Usage limit reached"));
        assert!(!is_usage_limit("timeout after 600 s"));
        assert!(!is_usage_limit("claude exited with 1: budget exceeded"));
    }

    #[test]
    fn the_queue_stops_after_a_usage_limit() {
        let cases: Vec<Case> = (0..40)
            .map(|i| {
                let mut c = a_case("security", Kind::Seeded, &[]);
                c.id = format!("case-{i:02}");
                c
            })
            .collect();
        let pending: Vec<Job> = cases.iter().map(|case| Job { case, n: 1 }).collect();
        let queue = Queue::new(&pending);
        let order: Vec<String> = (0..3)
            .map(|_| queue.pop().unwrap().case.id.clone())
            .collect();
        assert_eq!(order, ["case-00", "case-01", "case-02"]);
        queue.stop();
        assert!(queue.pop().is_none(), "no fourth run starts");
        assert!(queue.is_stopped());
    }

    #[test]
    fn a_call_with_nothing_new_charges_its_writes_to_output() {
        assert_eq!(
            share_writes(900, &[], true),
            BTreeMap::from([("prompt".to_string(), 900)])
        );
        assert_eq!(
            share_writes(900, &[], false),
            BTreeMap::from([("output".to_string(), 900)])
        );
        let entered = [("file".to_string(), 300), ("skill".to_string(), 600)];
        assert_eq!(
            share_writes(900, &entered, false),
            BTreeMap::from([("file".to_string(), 300), ("skill".to_string(), 600)])
        );
    }

    #[test]
    fn the_stream_keeps_no_init_event() {
        let stream = "{\"type\":\"system\",\"subtype\":\"init\",\"cwd\":\"/Users/someone/x\"}\n\
{\"type\":\"assistant\",\"message\":{}}\n{\"type\":\"result\"}\n";
        let kept = without_init_event(stream);
        assert!(!kept.contains("/Users/someone"), "{kept}");
        assert_eq!(kept.lines().count(), 2);
        // A stream it cannot read is kept as it is.
        assert_eq!(without_init_event("not json\n"), "not json\n");
    }

    #[test]
    fn a_validate_report_keeps_no_root() {
        let report = json!({"root": "/private/var/folders/x/T/case", "items": [], "status": []});
        let kept = without_root(report);
        assert!(kept.get("root").is_none());
        assert_eq!(kept["items"], json!([]));
    }

    fn isolated() -> ReviewSettings {
        ReviewSettings {
            mode: ReviewMode::Isolated,
            fixture_config: None,
        }
    }

    fn recorded_run(review: &ReviewSettings) -> RunInfo {
        let mut info = RunInfo {
            format: FORMAT,
            model: None,
            date: "2026-10-20".into(),
            claude_version: None,
            codev_version: None,
            repeat: Some(1),
            jobs: Some(2),
            review_mode: None,
            fixture_config: None,
        };
        review.record(&mut info);
        info
    }

    const INIT_CONFIG: &str = "schema: spec-driven\nlanguage: en\n";

    #[test]
    fn an_isolated_review_case_says_so_in_its_configuration() {
        let review = review_settings(Some("isolated"), None).unwrap();
        assert_eq!(review, isolated());
        assert_eq!(
            config_with_review_settings(INIT_CONFIG, &review).unwrap(),
            "schema: spec-driven\nlanguage: en\nreview: { mode: isolated }\n"
        );
        // The sub-agent tool is granted to the review cases alone.
        let tools = |step| {
            let args = claude_args(
                "p",
                &ClaudeSettings::from_env(&|_| None).unwrap(),
                step,
                &review,
            );
            let at = args.iter().position(|a| a == "--allowedTools").unwrap();
            args[at + 1].clone()
        };
        assert_eq!(tools(Step::PlanChallenge), "Bash(codev:*) Agent");
        assert_eq!(tools(Step::PlanWrite), "Bash(codev:*)");
        assert!(review.isolated_at(Step::CodeReview) && !review.isolated_at(Step::PlanUpdate));
        // Inline, the configuration is left as `codev init` wrote it.
        assert_eq!(
            config_with_review_settings(INIT_CONFIG, &ReviewSettings::default()),
            None
        );
        let text = serde_json::to_string(&recorded_run(&review)).unwrap();
        assert!(text.contains(r#""reviewMode":"isolated""#), "{text}");
        let inline = serde_json::to_string(&recorded_run(&ReviewSettings::default())).unwrap();
        assert!(!inline.contains("reviewMode"), "{inline}");
    }

    #[test]
    fn a_fixture_config_is_appended_to_every_case() {
        let file = "roles: { skeptic: { done_when: [\"The rollback plan is stated\"] } }\n";
        let review =
            review_settings(None, Some((Path::new("extra.yaml"), Ok(file.to_string())))).unwrap();
        assert_eq!(review.mode, ReviewMode::Inline);
        let config = config_with_review_settings("schema: spec-driven", &review).unwrap();
        assert!(
            config.ends_with(&format!("spec-driven\n{file}")),
            "{config}"
        );
        // With the isolated mode, the mode comes first and the file ends it.
        let both = ReviewSettings {
            fixture_config: review.fixture_config.clone(),
            ..isolated()
        };
        let config = config_with_review_settings(INIT_CONFIG, &both).unwrap();
        assert!(config.contains("review: { mode: isolated }\n"));
        assert!(config.ends_with(file), "{config}");
        assert_eq!(
            recorded_run(&review).fixture_config.as_deref(),
            Some(file),
            "run.json records the text"
        );
    }

    #[test]
    fn options_that_cannot_be_applied_stop_the_run() {
        let err = review_settings(Some("sometimes"), None).unwrap_err();
        assert!(
            err.contains("`sometimes`") && err.contains("--review-mode"),
            "{err}"
        );
        let missing = std::io::Error::new(std::io::ErrorKind::NotFound, "No such file");
        let err = review_settings(None, Some((Path::new("gone.yaml"), Err(missing)))).unwrap_err();
        assert!(
            err.contains("gone.yaml") && err.contains("No such file"),
            "{err}"
        );
        let file = |text: &str| Some((Path::new("extra.yaml"), Ok(text.to_string())));
        let err = review_settings(None, file("- a list\n")).unwrap_err();
        assert!(err.contains("not a YAML mapping"), "{err}");
        let err = review_settings(None, file("roles: [unclosed\n")).unwrap_err();
        assert!(err.contains("not valid YAML"), "{err}");
        let err = review_settings(None, file("review:\n  mode: isolated\n")).unwrap_err();
        assert!(
            err.contains("`review:`") && err.contains("--review-mode"),
            "{err}"
        );
        let err = review_settings(Some("isolated"), file("review:\n  mode: inline\n")).unwrap_err();
        assert!(
            err.contains("`review:`") && err.contains("--review-mode"),
            "{err}"
        );
    }

    #[test]
    fn resuming_in_another_mode_stops() {
        let recorded = recorded_run(&ReviewSettings::default());
        let err = check_resume_review(&recorded, &isolated()).unwrap_err();
        assert!(err.contains("inline") && err.contains("isolated"), "{err}");
        assert!(check_resume_review(&recorded_run(&isolated()), &isolated()).is_ok());
    }

    #[test]
    fn resuming_with_another_fixture_config_stops() {
        let with = |text: &str| ReviewSettings {
            fixture_config: Some(text.to_string()),
            ..ReviewSettings::default()
        };
        let recorded = recorded_run(&with("roles: { a: never }\n"));
        let err = check_resume_review(&recorded, &with("roles: { b: never }\n")).unwrap_err();
        assert!(
            err.contains("a: never") && err.contains("b: never"),
            "{err}"
        );
        assert!(check_resume_review(&recorded, &ReviewSettings::default()).is_err());
        assert!(check_resume_review(&recorded, &with("roles: { a: never }\n")).is_ok());
    }

    #[test]
    fn an_earlier_run_json_reads_as_inline() {
        let earlier = crate::observed::parse_run_info(
            r#"{"format": 2, "model": null, "date": "2026-10-15",
            "claudeVersion": "2.1.289 (Claude Code)", "codevVersion": "0.6.1",
            "repeat": 5, "jobs": 4}"#,
        )
        .unwrap();
        assert_eq!(earlier.review_mode_or_default(), ReviewMode::Inline);
        assert_eq!(earlier.fixture_config, None);
        assert!(check_resume_review(&earlier, &ReviewSettings::default()).is_ok());
        assert!(check_resume_review(&earlier, &isolated()).is_err());
    }

    /// `codev review --json` for a step: its `Mode:` line and each role's
    /// `Fallback:` line.
    fn review_json(mode: Option<&str>, fallback: Option<&str>) -> serde_json::Value {
        json!({"exists": true, "steps": [{"step": "plan.challenge", "recorded": true,
            "mode": mode, "summarized": [],
            "roles": [{"role": "skeptic", "state": "findings", "fallback": fallback,
                       "credited": []}]}],
            "findings": [], "issues": []})
    }

    #[test]
    fn a_case_that_fell_back_inline_is_an_error() {
        let started =
            |stream: &[String]| parse_claude_stream(&stream.join("\n")).sub_agents_started;
        let with_agent = started(&[
            assistant(
                "m1",
                10,
                1,
                json!([tool_use(
                    "t1",
                    "Agent",
                    json!({"prompt": "You are the skeptic"})
                )]),
            ),
            tool_result("t1", 50),
            assistant("m2", 10, 1, json!([{"type": "text", "text": "Done."}])),
            RESULT.to_string(),
        ]);
        assert_eq!(with_agent, 1);
        let without_agent = started(&[
            assistant("m1", 10, 1, json!([{"type": "text", "text": "Done."}])),
            RESULT.to_string(),
        ]);
        assert_eq!(without_agent, 0);

        let step = Step::PlanChallenge;
        assert_eq!(
            isolation_missing(Some(&review_json(Some("isolated"), None)), step, with_agent),
            None
        );
        let fell_back = review_json(Some("isolated"), Some("the sub-agent timed out twice."));
        let err = isolation_missing(Some(&fell_back), step, with_agent).unwrap();
        assert!(
            err.contains("Fallback: the sub-agent timed out twice."),
            "{err}"
        );
        assert!(err.contains("skeptic"), "{err}");
        let err = isolation_missing(
            Some(&review_json(Some("isolated"), None)),
            step,
            without_agent,
        )
        .unwrap();
        assert!(err.contains("no sub-agent"), "{err}");
        let err =
            isolation_missing(Some(&review_json(Some("inline"), None)), step, with_agent).unwrap();
        assert!(err.contains("`Mode: inline.`"), "{err}");
        let err = isolation_missing(None, step, with_agent).unwrap();
        assert!(err.contains("no `Mode: isolated.` line"), "{err}");
    }

    /// An assistant event with its input and cache writes, made by the
    /// session or, with `parent`, by the sub-agent that tool call started.
    fn call_event(id: &str, input: u64, writes: u64, parent: Option<&str>) -> String {
        json!({"type": "assistant", "parent_tool_use_id": parent,
            "message": {"id": id, "content": [{"type": "text", "text": "x"}],
            "usage": {"input_tokens": input, "cache_creation_input_tokens": writes,
                      "cache_read_input_tokens": 0, "output_tokens": 10}}})
        .to_string()
    }

    fn result_with_totals(input: u64, writes: u64, output: u64) -> String {
        json!({"type": "result", "subtype": "success", "is_error": false,
            "duration_ms": 1000, "num_turns": 3, "total_cost_usd": 0.4,
            "modelUsage": {"claude-opus-5-5": {"inputTokens": input,
                "cacheCreationInputTokens": writes, "cacheReadInputTokens": 0,
                "outputTokens": output}}})
        .to_string()
    }

    #[test]
    fn the_sub_agents_tokens_are_a_source() {
        // The session's own calls hold 70,000 input tokens; a sub-agent's
        // call, streamed under the tool call that started it, is not one.
        let stream = [
            call_event("m1", 30_000, 4_000, None),
            call_event("s1", 45_000, 9_000, Some("t1")),
            call_event("m2", 40_000, 1_000, None),
            result_with_totals(120_000, 15_000, 50),
        ]
        .join("\n");
        let mut consumption = parse_claude_stream(&stream).consumption.unwrap();
        assert_eq!(consumption.calls.len(), 2);
        record_sub_agent_tokens(&mut consumption).unwrap();
        assert_eq!(
            consumption.sub_agents,
            Some(Tokens {
                input: 50_000,
                cache_creation: 10_000,
                cache_read: 0,
                output: 30,
            })
        );

        // The sub-agents are a source of their own, and the per-call
        // measures count the session's calls alone.
        let case = a_case("security", Kind::Control, &[]);
        let mut observed = crate::score::tests::observed("no-finding", &[], json!([]));
        observed.consumption = Some(consumption.clone());
        let score = crate::score::suite(
            crate::score::tests::run(),
            vec![crate::score::case(&case, &observed)],
            crate::score::Thresholds::default(),
        );
        let source = score
            .sources
            .iter()
            .find(|s| s.source == crate::score::SUB_AGENTS_SOURCE)
            .expect("a sub-agents source");
        assert_eq!(source.tokens, 10_000.0);
        assert_eq!(
            score.case_consumption[0].measures.input_per_call,
            Some(37_500.0)
        );

        // Totals below the session's own calls are inconsistent.
        let stream = [
            call_event("m1", 30_000, 4_000, None),
            result_with_totals(20_000, 4_000, 10),
        ]
        .join("\n");
        let mut consumption = parse_claude_stream(&stream).consumption.unwrap();
        let err = record_sub_agent_tokens(&mut consumption).unwrap_err();
        assert!(err.contains("20000") && err.contains("30000"), "{err}");
    }

    #[test]
    fn two_runs_started_in_the_same_second_get_their_own_folder() {
        let temp = Path::new("/tmp");
        let a = default_work_dir(temp, 1_792_148_804, 101);
        let b = default_work_dir(temp, 1_792_148_804, 202);
        assert_ne!(a, b);
        assert_eq!(a, Path::new("/tmp/codev-evals-1792148804-101"));
    }

    fn implement_case(change: &str) -> Case {
        let mut case = a_case("planner", Kind::Seeded, &[]);
        case.id = "implement-x".into();
        case.step = Step::CodeImplement;
        case.change = change.into();
        case.implement = Some(crate::case::ImplementCase {
            git_ref: "abc123".into(),
            merged: "def456".into(),
            refuse: false,
            docs: true,
            acceptance: vec!["works".into()],
            script: Some("echo ok works\n".into()),
            files: BTreeMap::from([("tasks.md".to_string(), "- [ ] 1.1 A\n".to_string())]),
        });
        case
    }

    #[test]
    fn the_prompt_of_an_implementation_case() {
        let text = prompt(&implement_case("decision-accept-command"));
        for needle in [
            "`codev-apply`",
            "`decision-accept-command`",
            "ask no question",
            "code.review",
            "Do not archive the change and do not sync it",
        ] {
            assert!(text.contains(needle), "missing {needle}: {text}");
        }
    }

    #[test]
    fn the_budget_of_an_implementation_case() {
        let defaults = ClaudeSettings::from_env(&|_| None).unwrap();
        assert_eq!(defaults.budget_for(Step::CodeImplement), 30.0);
        assert_eq!(
            defaults.timeout_for(Step::CodeImplement),
            Duration::from_secs(5400)
        );
        let args = claude_args(
            "p",
            &defaults,
            Step::CodeImplement,
            &ReviewSettings::default(),
        );
        let at = args.iter().position(|a| a == "--max-budget-usd").unwrap();
        assert_eq!(args[at + 1], "30");
    }

    #[test]
    fn only_the_implementation_cases_run() {
        let cases = vec![
            a_case("security", Kind::Seeded, &[]),
            planning_case("r"),
            implement_case("c"),
        ];
        let selected = select_steps(&cases, &["code.implement".to_string()]).unwrap();
        assert_eq!(selected.len(), 1);
        assert_eq!(selected[0].step, Step::CodeImplement);
        let review = select_steps(&cases, &["review".to_string()]).unwrap();
        assert!(review.iter().all(|c| c.step != Step::CodeImplement));
    }

    #[test]
    fn a_run_that_syncs_its_change() {
        let case = implement_case("decision-accept-command");
        let status = " M _codev/specs/decisions/spec.md\0 M crates/codev-cli/src/main.rs\0";
        assert_eq!(strayed(status, &case), ["_codev/specs/decisions/spec.md"]);
        let archived = " D _codev/changes/decision-accept-command/tasks.md\0?? _codev/changes/archive/2026-10-20-decision-accept-command/tasks.md\0";
        assert_eq!(
            strayed(archived, &case),
            ["_codev/changes/archive/2026-10-20-decision-accept-command/tasks.md"]
        );
    }

    #[test]
    fn an_implementation_run_may_change_the_code() {
        let case = implement_case("decision-accept-command");
        let status = " M crates/codev-cli/src/main.rs\0?? docs/fr/src/cli.md\0 M _codev/changes/decision-accept-command/tasks.md\0?? _codev/changes/decision-accept-command/review.md\0";
        assert!(strayed(status, &case).is_empty());
        let corrected = "?? _codev/changes/decision-accept-command/corrections.md\0";
        assert_eq!(
            strayed(corrected, &case),
            ["_codev/changes/decision-accept-command/corrections.md"]
        );
    }

    #[test]
    fn an_implementation_run_grants_what_the_apply_skill_grants() {
        let skill = "---\nname: codev-apply\nallowed-tools: \"Bash(codev:*), Read, Write, Edit, Glob, Grep, Bash, Agent\"\nlicense: MIT\n---\n\nallowed-tools: not this\n";
        let tools = skill_allowed_tools(skill).unwrap();
        assert_eq!(
            tools,
            "Bash(codev:*), Read, Write, Edit, Glob, Grep, Bash, Agent"
        );
        let settings = ClaudeSettings::from_env(&|_| None).unwrap();
        let args = with_allowed_tools(
            claude_args(
                "p",
                &settings,
                Step::CodeImplement,
                &ReviewSettings::default(),
            ),
            &tools,
        );
        let at = args.iter().position(|a| a == "--allowedTools").unwrap();
        assert_eq!(args[at + 1], tools);
        assert!(skill_allowed_tools("no frontmatter").is_none());
    }
}
