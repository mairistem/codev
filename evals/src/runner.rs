//! Runs the cases with Claude Code. The pure parts — the change's
//! `change.yaml`, the prompts, the command line, what is left to run, what
//! strayed — come first and are unit-tested; the shell parts below them
//! touch the disk and start processes, and are exercised by real runs.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crate::case::{Case, Step};
use crate::observed::{FORMAT, Observed, RunInfo};

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
                 check the record with `codev review --change {change} --json`).\n\
                 \n\
                 Evaluation mode: skip the author's answer and the reviewer's re-check — leave every finding `open`. \
                 Modify no file but `_codev/changes/{change}/review.md`: do not fix, simplify or edit the code or the plan. \
                 Ask no question. When the record is written and checked, stop with a one-line summary.",
                step = case.step.as_str(),
            )
        }
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
    pub budget_usd: f64,
    pub timeout: Duration,
}

pub const DEFAULT_BUDGET_USD: f64 = 3.0;
pub const DEFAULT_TIMEOUT_SECS: u64 = 600;

impl ClaudeSettings {
    /// `CODEV_EVAL_MODEL`, `CODEV_EVAL_BUDGET_USD`,
    /// `CODEV_EVAL_TIMEOUT_SECS`; a value that does not parse names its
    /// variable.
    pub fn from_env(env: &dyn Fn(&str) -> Option<String>) -> Result<ClaudeSettings, String> {
        let model = env("CODEV_EVAL_MODEL").filter(|m| !m.trim().is_empty());
        let budget_usd = match env("CODEV_EVAL_BUDGET_USD") {
            None => DEFAULT_BUDGET_USD,
            Some(text) => text
                .trim()
                .parse::<f64>()
                .ok()
                .filter(|v| *v > 0.0)
                .ok_or_else(|| {
                    format!("CODEV_EVAL_BUDGET_USD: `{text}` is not a positive number")
                })?,
        };
        let timeout = match env("CODEV_EVAL_TIMEOUT_SECS") {
            None => DEFAULT_TIMEOUT_SECS,
            Some(text) => text
                .trim()
                .parse::<u64>()
                .ok()
                .filter(|v| *v > 0)
                .ok_or_else(|| {
                    format!("CODEV_EVAL_TIMEOUT_SECS: `{text}` is not a positive whole number")
                })?,
        };
        Ok(ClaudeSettings {
            model,
            budget_usd,
            timeout: Duration::from_secs(timeout),
        })
    }
}

/// The tools the agent may use. git by read-only subcommand only:
/// `Bash(git:*)` would let it run any program through git's configuration.
pub const ALLOWED_TOOLS: &str = "Bash(codev:*) Bash(git diff:*) Bash(git log:*) Bash(git show:*) Bash(git status:*) Read Write Edit Glob Grep Skill";

/// The arguments of `claude` for one case.
pub fn claude_args(prompt: &str, settings: &ClaudeSettings) -> Vec<String> {
    let mut args: Vec<String> = [
        "-p",
        prompt,
        "--output-format",
        "json",
        "--no-session-persistence",
        "--permission-mode",
        "acceptEdits",
        "--allowedTools",
        ALLOWED_TOOLS,
        "--max-budget-usd",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    args.push(format!("{}", settings.budget_usd));
    if let Some(model) = &settings.model {
        args.push("--model".into());
        args.push(model.clone());
    }
    args
}

/// The cases still to run: those without an observation, or all of them
/// with `rerun`.
pub fn pending<'a>(cases: &'a [Case], recorded: &BTreeSet<String>, rerun: bool) -> Vec<&'a Case> {
    cases
        .iter()
        .filter(|c| rerun || !recorded.contains(&c.id))
        .collect()
}

/// The files `git status --porcelain -z --untracked-files=all` lists that
/// the step does not allow: `review.md` at the review steps, the change's
/// directory at `plan.trace`.
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
            Step::PlanTrace => path.starts_with(&change_dir),
            _ => path == review,
        };
        if !allowed {
            out.push(path.to_string());
        }
    }
    out.sort();
    out
}

/// What Claude Code's `--output-format json` reports: cost, duration, and
/// whether it ended in error.
pub fn parse_claude_output(stdout: &str) -> (Option<f64>, Option<u64>, Option<String>) {
    let Ok(value) = serde_json::from_str::<serde_json::Value>(stdout.trim()) else {
        return (
            None,
            None,
            Some(format!(
                "Claude Code's output is not JSON: {}",
                excerpt(stdout)
            )),
        );
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
    (cost, duration, error)
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
    Ok((claude, codev))
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

fn run_tool(tool: &Path, args: &[&str], dir: &Path, codev_dir: &Path) -> Result<String, String> {
    let output = Command::new(tool)
        .args(args)
        .current_dir(dir)
        .env("CODEV_NO_UPDATE_CHECK", "1")
        .env("PATH", path_with(codev_dir))
        .stdin(Stdio::null())
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

fn git(tools: &Tools, dir: &Path, args: &[&str]) -> Result<String, String> {
    let mut all = vec![
        "-c",
        "user.name=codev-evals",
        "-c",
        "user.email=evals@codev.invalid",
        "-c",
        "commit.gpgsign=false",
        "-c",
        "core.hooksPath=.git/no-hooks",
    ];
    all.extend_from_slice(args);
    run_tool(&tools.git, &all, dir, codev_dir(tools))
}

fn codev_dir(tools: &Tools) -> &Path {
    tools.codev.parent().unwrap_or(Path::new("."))
}

/// Builds the case's project in `dir`: `base/` on `main` with codev
/// initialized, `change/` on the branch `change`, the evaluated role alone.
pub fn setup(
    case: &Case,
    case_dir: &Path,
    dir: &Path,
    tools: &Tools,
    date: &str,
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

/// Runs Claude Code in `dir` with a timeout; returns its standard output.
fn run_claude(
    dir: &Path,
    tools: &Tools,
    args: &[String],
    timeout: Duration,
    log: &Path,
) -> Result<String, String> {
    let stdout_path = log.join("claude.json");
    let stderr_path = log.join("claude.stderr");
    let stdout = fs::File::create(&stdout_path).map_err(|e| e.to_string())?;
    let stderr = fs::File::create(&stderr_path).map_err(|e| e.to_string())?;
    let mut command = Command::new(&tools.claude);
    command
        .args(args)
        .current_dir(dir)
        .env("CODEV_NO_UPDATE_CHECK", "1")
        .env("PATH", path_with(codev_dir(tools)))
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

fn terminate(child: &mut std::process::Child) {
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

/// Runs one case and returns its observation; never fails, a failure is
/// the observation's `error`.
pub fn run_case(
    case: &Case,
    case_dir: &Path,
    work: &Path,
    log: &Path,
    tools: &Tools,
    settings: &ClaudeSettings,
    date: &str,
) -> Observed {
    let mut observed = Observed {
        format: FORMAT,
        case: case.id.clone(),
        error: None,
        cost_usd: None,
        duration_ms: None,
        strayed: Vec::new(),
        review: None,
        files: BTreeMap::new(),
    };
    let dir = work.join(&case.id);
    if let Err(e) = fs::create_dir_all(log) {
        observed.error = Some(format!("cannot create {}: {e}", log.display()));
        return observed;
    }
    if let Err(e) = setup(case, case_dir, &dir, tools, date) {
        observed.error = Some(e);
        return observed;
    }
    let args = claude_args(&prompt(case), settings);
    match run_claude(&dir, tools, &args, settings.timeout, log) {
        Err(e) => observed.error = Some(e),
        Ok(stdout) => {
            let (cost, duration, error) = parse_claude_output(&stdout);
            observed.cost_usd = cost;
            observed.duration_ms = duration;
            observed.error = error;
        }
    }
    if observed.error.is_some() {
        return observed;
    }
    if let Ok(status) = git(
        tools,
        &dir,
        &["status", "--porcelain", "-z", "--untracked-files=all"],
    ) {
        observed.strayed = strayed(&status, case);
    }
    if case.step.records_findings() {
        // codev exits with 1 when the record has an error: its JSON is
        // still the observation, only output that is not JSON is an error.
        let output = Command::new(&tools.codev)
            .args(["review", "--change", &case.change, "--json"])
            .current_dir(&dir)
            .env("CODEV_NO_UPDATE_CHECK", "1")
            .output();
        match output {
            Ok(out) => match serde_json::from_slice::<serde_json::Value>(&out.stdout) {
                Ok(value) => observed.review = Some(value),
                Err(e) => observed.error = Some(format!("`codev review --json` output: {e}")),
            },
            Err(e) => observed.error = Some(format!("cannot run codev review: {e}")),
        }
        if let Ok(record) = fs::read(dir.join(case.change_dir()).join("review.md")) {
            let _ = fs::write(log.join("review.md"), record);
        }
    }
    for path in case.checked_paths() {
        let content = fs::read_to_string(dir.join(&path)).ok();
        observed.files.insert(path, content);
    }
    observed
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
}

/// Runs `cases` `jobs` at a time, writing each `observed.json` as soon as
/// it is known, so an interrupted run keeps what it measured.
pub fn run_cases(cases: &[&Case], options: &RunOptions, date: &str) {
    let queue: Arc<Mutex<Vec<&Case>>> = Arc::new(Mutex::new(cases.iter().rev().copied().collect()));
    std::thread::scope(|scope| {
        for _ in 0..options.jobs.max(1) {
            let queue = Arc::clone(&queue);
            scope.spawn(move || {
                loop {
                    let Some(case) = queue.lock().expect("queue lock").pop() else {
                        break;
                    };
                    eprintln!("[codev-evals] {} — running", case.id);
                    let case_dir = options.repo.join("evals").join("cases").join(&case.id);
                    let log = options.out.join(&case.id);
                    let observed = run_case(
                        case,
                        &case_dir,
                        &options.work,
                        &log,
                        &options.tools,
                        &options.settings,
                        date,
                    );
                    let text = serde_json::to_string_pretty(&observed).expect("serializes") + "\n";
                    if let Err(e) = fs::write(log.join("observed.json"), text) {
                        eprintln!(
                            "[codev-evals] {} — cannot write observed.json: {e}",
                            case.id
                        );
                    }
                    eprintln!(
                        "[codev-evals] {} — {}",
                        case.id,
                        observed.error.as_deref().unwrap_or("done")
                    );
                    if !options.keep {
                        let _ = fs::remove_dir_all(options.work.join(&case.id));
                    }
                }
            });
        }
    });
}

/// Refuses to resume a run recorded with another model: its observations
/// would be scored under the first model's name.
pub fn check_resume(recorded: &RunInfo, model: Option<&str>) -> Result<(), String> {
    if recorded.model.as_deref() == model {
        return Ok(());
    }
    let name = |m: Option<&str>| m.unwrap_or("Claude Code's default").to_string();
    Err(format!(
        "this output directory was recorded with the model {}, and CODEV_EVAL_MODEL asks for {}; \
         use another --out directory, or the same model",
        name(recorded.model.as_deref()),
        name(model)
    ))
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
        let args = claude_args("p", &settings);
        let after = |flag: &str| {
            args.iter()
                .position(|a| a == flag)
                .map(|i| args[i + 1].clone())
        };
        assert_eq!(after("--model").as_deref(), Some("claude-sonnet-4-5"));
        assert_eq!(after("--max-budget-usd").as_deref(), Some("1.5"));
        assert_eq!(after("-p").as_deref(), Some("p"));

        let defaults = ClaudeSettings::from_env(&|_| None).unwrap();
        let args = claude_args("p", &defaults);
        assert!(!args.iter().any(|a| a == "--model"));
        let budget = args.iter().position(|a| a == "--max-budget-usd").unwrap();
        assert_eq!(args[budget + 1], "3");
        assert_eq!(defaults.timeout, Duration::from_secs(600));

        let bad = |name: &str| (name == "CODEV_EVAL_TIMEOUT_SECS").then(|| "soon".to_string());
        assert!(
            ClaudeSettings::from_env(&bad)
                .unwrap_err()
                .contains("CODEV_EVAL_TIMEOUT_SECS")
        );
    }

    #[test]
    fn git_is_allowed_by_read_only_subcommand_only() {
        assert!(!ALLOWED_TOOLS.contains("Bash(git:*)"));
        assert!(ALLOWED_TOOLS.contains("Bash(git diff:*)"));
    }

    #[test]
    fn a_resumed_run_runs_only_unrecorded_cases() {
        let cases: Vec<Case> = ["a", "b", "c", "d", "e"]
            .iter()
            .map(|id| {
                let mut c = a_case("security", Kind::Control, &[]);
                c.id = (*id).into();
                c
            })
            .collect();
        let recorded: BTreeSet<String> = ["a", "c", "e"].iter().map(|s| s.to_string()).collect();
        let ids: Vec<&str> = pending(&cases, &recorded, false)
            .iter()
            .map(|c| c.id.as_str())
            .collect();
        assert_eq!(ids, ["b", "d"]);
        assert_eq!(pending(&cases, &recorded, true).len(), 5);
    }

    #[test]
    fn a_resumed_run_with_another_model_is_refused() {
        let recorded = RunInfo {
            format: FORMAT,
            model: None,
            date: "2026-10-13".into(),
            claude_version: None,
            codev_version: None,
        };
        assert!(check_resume(&recorded, None).is_ok());
        let err = check_resume(&recorded, Some("claude-sonnet-4-5")).unwrap_err();
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
        assert_eq!(parse_claude_output(ok), (Some(0.42), Some(81234), None));
        let budget = r#"{"subtype":"error_max_budget_usd","is_error":true,"total_cost_usd":3.01}"#;
        let (_, _, error) = parse_claude_output(budget);
        assert!(error.unwrap().contains("error_max_budget_usd"));
        assert!(parse_claude_output("oops").2.unwrap().contains("not JSON"));
    }

    #[test]
    fn civil_dates_are_right() {
        assert_eq!(civil_date(0), "1970-01-01");
        assert_eq!(civil_date(20_739), "2026-10-13");
        assert_eq!(civil_date(11_016), "2000-02-29");
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
}
