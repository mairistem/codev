//! The repository's own suite: its cases, its coverage, its baseline, its
//! workflow and the guides that ask contributors to run it.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use codev_evals::case::coverage_gaps;
use codev_evals::checks::{
    missing_from_baseline, missing_mention, repetitions_in_error, too_few_repetitions,
};
use codev_evals::reliability::{self, Baseline, Lang};
use codev_evals::report;
use codev_evals::score::Thresholds;
use codev_evals::suite;

fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("evals/ has a parent")
        .to_path_buf()
}

fn read(relative: &str) -> String {
    fs::read_to_string(repo().join(relative))
        .unwrap_or_else(|e| panic!("{relative}: {e}"))
        .replace("\r\n", "\n")
}

#[test]
fn every_case_of_the_suite_loads() {
    let cases = suite::load_cases(&repo()).unwrap_or_else(|e| panic!("{e}"));
    assert!(cases.len() >= 40, "{} cases", cases.len());
}

#[test]
fn the_suite_covers_every_reviewing_role() {
    let cases = suite::load_cases(&repo()).unwrap();
    assert_eq!(coverage_gaps(&cases), Vec::<String>::new());
}

#[test]
fn the_suite_covers_the_implementation_of_past_changes() {
    let cases = suite::load_cases(&repo()).unwrap();
    let implementing: Vec<_> = cases
        .iter()
        .filter_map(|c| c.implement.as_ref().map(|i| (c.id.as_str(), i)))
        .collect();
    assert!(implementing.len() >= 4, "{implementing:?}");
    let refusing: Vec<&str> = implementing
        .iter()
        .filter(|(_, i)| i.refuse)
        .map(|(id, _)| *id)
        .collect();
    assert_eq!(refusing, ["implement-refuse-missing-tasks"]);
    for (id, implement) in &implementing {
        assert!(
            implement.refuse || !implement.acceptance.is_empty(),
            "{id} has no acceptance assertion"
        );
    }
}

#[test]
fn categories_of_one_case_share_no_term() {
    let cases = suite::load_cases(&repo()).unwrap();
    let mut overlaps = Vec::new();
    for case in &cases {
        for (n, a) in case.expected.iter().enumerate() {
            for b in &case.expected[n + 1..] {
                if a.category == b.category {
                    continue;
                }
                for term in a.terms.iter().filter(|t| b.terms.contains(t)) {
                    overlaps.push(format!(
                        "{}: `{}` and `{}` share `{term}`",
                        case.id, a.category, b.category
                    ));
                }
            }
        }
    }
    assert_eq!(overlaps, Vec::<String>::new());
}

#[test]
fn every_case_has_a_baseline_observation() {
    let cases: Vec<String> = suite::load_cases(&repo())
        .unwrap()
        .into_iter()
        .map(|c| c.id)
        .collect();
    let observed: BTreeSet<String> = suite::recorded(&repo().join("evals").join("baseline"))
        .into_keys()
        .map(|(id, _)| id)
        .collect();
    assert_eq!(
        missing_from_baseline(&cases, &observed),
        Vec::<String>::new()
    );
}

#[test]
fn the_baseline_agrees_with_the_scoring() {
    let cases = suite::load_cases(&repo()).unwrap();
    let score = suite::score_dir(
        &repo(),
        &cases,
        &repo().join("evals").join("baseline"),
        Thresholds::default(),
    )
    .unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(
        report::json(&score),
        read("evals/baseline/report.json"),
        "scoring the baseline's observations must give its committed report; \
         run `cargo run -p codev-evals -- score evals/baseline` after changing the scoring rules"
    );
    assert_eq!(report::markdown(&score), read("evals/baseline/report.md"));
}

#[test]
fn the_reliability_pages_follow_the_baseline() {
    let baseline = Baseline::parse(
        &read("evals/baseline/report.json"),
        &read("evals/baseline/run.json"),
    )
    .unwrap_or_else(|e| panic!("{e}"));
    for lang in Lang::ALL {
        let path = format!("docs/{}/src/reliability.md", lang.tag());
        assert!(
            read(&path) == reliability::page(&baseline, lang),
            "{path} differs from the baseline; run `cargo run -p codev-evals -- docs`"
        );
    }
}

/// The `on:` block of the workflow, up to the next top-level key.
fn triggers(workflow: &str) -> Vec<String> {
    let mut lines = workflow.lines().skip_while(|l| !l.starts_with("on:"));
    lines.next();
    lines
        .take_while(|l| l.is_empty() || l.starts_with(' '))
        .filter(|l| l.starts_with("  ") && !l.starts_with("   ") && l.trim_end().ends_with(':'))
        .map(|l| l.trim().trim_end_matches(':').to_string())
        .collect()
}

#[test]
fn the_workflow_runs_only_on_dispatch() {
    let workflow = read(".github/workflows/evals.yml");
    assert_eq!(triggers(&workflow), ["workflow_dispatch"]);
    let wider = workflow.replace("on:\n", "on:\n  pull_request:\n");
    assert_ne!(
        triggers(&wider),
        ["workflow_dispatch"],
        "the check sees a second trigger"
    );
}

#[test]
fn the_workflow_checks_the_secret_before_running() {
    let workflow = read(".github/workflows/evals.yml");
    let check = workflow
        .find("if [ -z \"$ANTHROPIC_API_KEY\" ]")
        .expect("the workflow tests the secret");
    let run = workflow
        .find("codev-evals -- run")
        .expect("the workflow runs the suite");
    assert!(check < run, "the secret is tested before the suite runs");
    assert!(workflow.contains("Evaluations skipped"));
    assert!(
        !workflow.contains("run: ${{") && !workflow.contains("$EVAL_ROLES }}"),
        "inputs are never interpolated into a script"
    );
}

#[test]
fn contributing_and_roles_chapters_name_the_baseline() {
    let names = [
        "CONTRIBUTING.md",
        "CONTRIBUTING.fr.md",
        "docs/en/src/roles.md",
        "docs/fr/src/roles.md",
    ];
    let texts: Vec<String> = names.iter().map(|n| read(n)).collect();
    let files: Vec<(&str, &str)> = names
        .iter()
        .zip(&texts)
        .map(|(n, t)| (*n, t.as_str()))
        .collect();
    let mut gaps = missing_mention(&files, "evals/baseline");
    gaps.extend(missing_mention(&files[..2], "assets/roles/"));
    gaps.extend(missing_mention(&files[..2], "assets/workflows/"));
    gaps.extend(missing_mention(&files[..2], "codev-evals -- docs"));
    gaps.extend(missing_mention(&files, "codev-evals"));
    gaps.extend(missing_mention(&files[..2], "codev-evals -- compare"));
    gaps.extend(missing_mention(&files[2..], "codev-evals compare"));
    gaps.extend(missing_mention(&files[..2], "--repeat 3"));
    assert_eq!(gaps, Vec::<String>::new());
}

#[test]
fn contributing_chapters_document_the_update_cases() {
    let names = ["CONTRIBUTING.md", "CONTRIBUTING.fr.md"];
    let texts: Vec<String> = names.iter().map(|n| read(n)).collect();
    let files: Vec<(&str, &str)> = names
        .iter()
        .zip(&texts)
        .map(|(n, t)| (*n, t.as_str()))
        .collect();
    let mut gaps = Vec::new();
    for needle in [
        "step: plan.update",
        "--step plan.update",
        "`revised`",
        "`kept`",
        "`ripple`",
        "`contains`",
        "`refuse`",
        "/codev-propose",
        "codev validate <change> --json",
        // A new step's cases recorded alone into the baseline.
        "run --step plan.update --out evals/baseline --repeat 5",
    ] {
        gaps.extend(missing_mention(&files, needle));
    }
    assert_eq!(gaps, Vec::<String>::new());
}

#[test]
fn contributing_chapters_document_the_implementation_cases() {
    let names = ["CONTRIBUTING.md", "CONTRIBUTING.fr.md"];
    let texts: Vec<String> = names.iter().map(|n| read(n)).collect();
    let files: Vec<(&str, &str)> = names
        .iter()
        .zip(&texts)
        .map(|(n, t)| (*n, t.as_str()))
        .collect();
    let mut gaps = Vec::new();
    for needle in [
        "step: code.implement",
        "--step code.implement",
        "`merged`",
        "`acceptance`",
        "acceptance.sh",
        "`ok <",
        "codev-evals -- calibrate",
        "`mdbook`",
        // The commands the runner starts itself have a timeout.
        "200 second",
    ] {
        gaps.extend(missing_mention(&files, needle));
    }
    assert_eq!(gaps, Vec::<String>::new());
}

#[test]
fn contributing_chapters_say_the_frozen_binary_keeps_its_name() {
    let names = ["CONTRIBUTING.md", "CONTRIBUTING.fr.md"];
    let texts: Vec<String> = names.iter().map(|n| read(n)).collect();
    let files: Vec<(&str, &str)> = names
        .iter()
        .zip(&texts)
        .map(|(n, t)| (*n, t.as_str()))
        .collect();
    let mut gaps = missing_mention(&files, "--codev <");
    gaps.extend(missing_mention(&files, "/codev`"));
    assert_eq!(gaps, Vec::<String>::new());
}

#[test]
fn contributing_chapters_document_the_isolated_mode() {
    let names = ["CONTRIBUTING.md", "CONTRIBUTING.fr.md"];
    let texts: Vec<String> = names.iter().map(|n| read(n)).collect();
    let files: Vec<(&str, &str)> = names
        .iter()
        .zip(&texts)
        .map(|(n, t)| (*n, t.as_str()))
        .collect();
    let mut gaps = Vec::new();
    for needle in [
        "--review-mode isolated",
        "review: { mode: isolated }",
        "`Agent`",
        "`Fallback:`",
        "`sub-agents`",
        "--fixture-config",
        "--across-models",
        "run.json",
    ] {
        gaps.extend(missing_mention(&files, needle));
    }
    assert_eq!(gaps, Vec::<String>::new());
}

/// The `run:` scripts of a workflow, each with its following indented lines.
fn scripts(workflow: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut lines = workflow.lines().peekable();
    while let Some(line) = lines.next() {
        let trimmed = line.trim_start();
        if let Some(rest) = trimmed.strip_prefix("run:") {
            let indent = line.len() - trimmed.len();
            let mut script = rest.to_string();
            while let Some(next) = lines.peek() {
                let next_indent = next.len() - next.trim_start().len();
                if !next.trim().is_empty() && next_indent <= indent {
                    break;
                }
                script.push('\n');
                script.push_str(next);
                lines.next();
            }
            out.push(script);
        }
    }
    out
}

#[test]
fn no_workflow_input_is_interpolated_in_a_script() {
    let workflow = read(".github/workflows/evals.yml");
    for script in scripts(&workflow) {
        assert!(
            !script.contains("${{"),
            "a script interpolates an expression: {script}"
        );
    }
}

#[test]
fn the_workflow_passes_repeat_through_the_environment() {
    let workflow = read(".github/workflows/evals.yml");
    assert!(workflow.contains("EVAL_REPEAT: ${{ inputs.repeat }}"));
    assert!(workflow.contains("--repeat \"$EVAL_REPEAT\" --jobs 4"));
    let bound = workflow
        .find("^[1-3]$")
        .expect("the run step bounds repeat");
    let run = workflow.find("codev-evals -- run").unwrap();
    assert!(bound < run, "repeat is checked before the suite runs");
}

/// The jobs of a workflow, each from its `  <id>:` line to the next.
fn jobs(workflow: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let body = workflow.split_once("\njobs:\n").map_or("", |(_, b)| b);
    for line in body.lines() {
        let is_job = line.starts_with("  ") && !line.starts_with("   ") && line.ends_with(':');
        if is_job || out.is_empty() {
            out.push(String::new());
        }
        let job = out.last_mut().expect("a job");
        job.push_str(line);
        job.push('\n');
    }
    out
}

#[test]
fn the_workflows_check_out_the_full_history() {
    for path in [".github/workflows/evals.yml", ".github/workflows/ci.yml"] {
        let workflow = read(path);
        // Every job that runs `cargo test` or the suite checks out with
        // `fetch-depth: 0` before it.
        let running: Vec<String> = jobs(&workflow)
            .into_iter()
            .filter(|j| j.contains("cargo test") || j.contains("codev-evals -- run"))
            .collect();
        assert!(!running.is_empty(), "{path}: no job runs the suite");
        for job in &running {
            let checkout = job.find("actions/checkout").expect("a checkout");
            let depth = job.find("fetch-depth: 0").unwrap_or_else(|| {
                panic!("{path}: a job checks out without the full history: {job}")
            });
            assert!(
                checkout < depth,
                "{path}: fetch-depth belongs to the checkout"
            );
        }
    }
}

#[test]
fn the_workflow_passes_step_through_the_environment() {
    let workflow = read(".github/workflows/evals.yml");
    assert!(workflow.contains("EVAL_STEP: ${{ inputs.step }}"));
    assert!(workflow.contains("--step \"$EVAL_STEP\""));
    assert!(workflow.contains("default: \"review\""));
}

#[test]
fn the_workflow_bounds_a_planning_run() {
    let workflow = read(".github/workflows/evals.yml");
    let bound = workflow
        .find("[ \"$EVAL_STEP\" = \"plan.write\" ] && [ \"$EVAL_REPEAT\" != \"1\" ]")
        .expect("the run step bounds a planning run to one repetition");
    let run = workflow.find("codev-evals -- run").unwrap();
    assert!(bound < run, "the bound is checked before the suite runs");
}

#[test]
fn the_workflow_installs_the_sandbox() {
    let workflow = read(".github/workflows/evals.yml");
    let install = workflow
        .find("apt-get install -y bubblewrap socat")
        .expect("the workflow installs the sandbox");
    assert!(workflow.contains("profile bwrap /usr/bin/bwrap"));
    let run = workflow.find("codev-evals -- run").unwrap();
    assert!(
        install < run,
        "the sandbox is installed before the suite runs"
    );
}

#[test]
fn no_raw_log_is_tracked_in_the_baseline() {
    let output = std::process::Command::new("git")
        .args(["ls-files", "evals/baseline"])
        .current_dir(repo())
        .output()
        .expect("git runs");
    let tracked = String::from_utf8_lossy(&output.stdout);
    let raw: Vec<&str> = tracked
        .lines()
        .filter(|l| l.ends_with("/claude.json") || l.ends_with("/claude.stderr"))
        .collect();
    assert_eq!(raw, Vec::<&str>::new(), "raw Claude Code logs are tracked");
    let ignored = std::process::Command::new("git")
        .args([
            "check-ignore",
            "-q",
            "evals/baseline/some-case/2/claude.json",
        ])
        .current_dir(repo())
        .status()
        .expect("git runs");
    assert!(ignored.success(), "a repetition's raw log is ignored");
}

#[test]
fn every_baseline_case_has_five_repetitions() {
    let recorded = suite::recorded(&repo().join("evals").join("baseline"));
    assert_eq!(too_few_repetitions(&recorded, 5), Vec::<String>::new());
}

#[test]
fn no_baseline_repetition_is_in_error() {
    let recorded = suite::recorded(&repo().join("evals").join("baseline"));
    assert_eq!(repetitions_in_error(&recorded), Vec::<String>::new());
}

#[test]
fn the_baseline_is_recorded_with_the_pinned_claude_code() {
    let workflow = read(".github/workflows/evals.yml");
    let marker = "@anthropic-ai/claude-code@";
    let at = workflow
        .find(marker)
        .expect("the workflow pins Claude Code")
        + marker.len();
    let pin: String = workflow[at..]
        .chars()
        .take_while(|c| c.is_ascii_digit() || *c == '.')
        .collect();
    let run: serde_json::Value = serde_json::from_str(&read("evals/baseline/run.json")).unwrap();
    let recorded = run["claudeVersion"].as_str().unwrap_or("");
    assert!(
        recorded.starts_with(&format!("{pin} ")),
        "the baseline was recorded with Claude Code `{recorded}`, the workflow pins {pin}: \
         raise the pin and record the baseline again together"
    );
}

#[test]
fn the_planning_suite_covers_past_changes_and_decisions() {
    use codev_evals::case::{DecisionExpect, Step};
    let cases = suite::load_cases(&repo()).unwrap();
    assert!(
        cases.iter().any(|c| c.step.is_review()) && cases.iter().any(|c| c.step == Step::PlanWrite),
        "review and planning cases load side by side"
    );
    let plans: Vec<_> = cases.iter().filter_map(|c| c.plan.as_ref()).collect();
    let historical = plans
        .iter()
        .filter(|p| p.decisions.is_empty() && !p.modified.is_empty())
        .count();
    let with_decision = plans.iter().filter(|p| !p.decisions.is_empty()).count();
    let to_supersede = plans
        .iter()
        .flat_map(|p| &p.decisions)
        .filter(|d| d.expect == DecisionExpect::Supersede)
        .count();
    assert!(
        historical >= 17,
        "{historical} planning cases with a modified requirement, 17 needed"
    );
    assert!(
        with_decision >= 4,
        "{with_decision} planning cases with a decision, 4 needed"
    );
    assert!(
        to_supersede >= 1,
        "no planning case expects a decision to be superseded"
    );
}

#[test]
fn the_suite_covers_the_update_behaviors() {
    use codev_evals::case::{Artifact, Step};
    let cases = suite::load_cases(&repo()).unwrap();
    let updates: Vec<_> = cases
        .iter()
        .filter(|c| c.step == Step::PlanUpdate)
        .filter_map(|c| c.update.as_ref().map(|u| (c, u)))
        .collect();
    assert_eq!(updates.len(), 6, "six plan.update cases");
    for artifact in [
        Artifact::Proposal,
        Artifact::Specs,
        Artifact::Design,
        Artifact::Tasks,
    ] {
        assert!(
            updates
                .iter()
                .any(|(_, u)| u.artifact == artifact && !u.refuse),
            "no case revises the {}",
            artifact.as_str()
        );
    }
    assert!(
        updates.iter().any(|(_, u)| u.refuse),
        "no case expects a refusal"
    );
    let ripple_to = |test: fn(&str) -> bool| {
        updates
            .iter()
            .any(|(_, u)| u.ripple.iter().any(|p| test(p)))
    };
    assert!(
        ripple_to(|p| p.starts_with("specs/") && p.ends_with("/spec.md")),
        "no case expects a ripple to a delta spec"
    );
    assert!(
        ripple_to(|p| p == "tasks.md"),
        "no case expects a ripple to the tasks"
    );
    // Each case's change is an archived change, copied as the archive
    // keeps it.
    let archive = repo().join("_codev").join("changes").join("archive");
    let archived: Vec<String> = fs::read_dir(&archive)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    for (case, update) in &updates {
        let dir = archived
            .iter()
            .find(|d| d.len() > 11 && d[11..] == case.change)
            .unwrap_or_else(|| panic!("{}: `{}` is not an archived change", case.id, case.change));
        for (path, text) in &update.files {
            let original = read(&format!("_codev/changes/archive/{dir}/{path}"));
            assert!(
                original == *text,
                "{}: `change/{path}` differs from the archive",
                case.id
            );
        }
    }
}

/// A run of this many words shared with an archived proposal is a copy,
/// not a coincidence: "more than twelve words", as the spec says.
const COPIED_RUN: usize = 13;

/// Every run of `n` consecutive words of `text`, lowercased.
fn word_runs(text: &str, n: usize) -> BTreeSet<String> {
    let words: Vec<String> = text
        .split(|c: char| !c.is_alphanumeric() && c != '\'')
        .filter(|w| !w.is_empty())
        .map(str::to_lowercase)
        .collect();
    words.windows(n).map(|w| w.join(" ")).collect()
}

#[test]
fn no_request_copies_its_proposal() {
    let cases = suite::load_cases(&repo()).unwrap();
    let archive = repo().join("_codev").join("changes").join("archive");
    let mut proposals = BTreeSet::new();
    for entry in fs::read_dir(&archive).unwrap().filter_map(|e| e.ok()) {
        if let Ok(text) = fs::read_to_string(entry.path().join("proposal.md")) {
            proposals.extend(word_runs(&text, COPIED_RUN));
        }
    }
    let copied: Vec<String> = cases
        .iter()
        .filter_map(|c| c.plan.as_ref().map(|p| (c, p)))
        .filter(|(_, p)| {
            word_runs(&p.request, COPIED_RUN)
                .iter()
                .any(|r| proposals.contains(r))
        })
        .map(|(c, _)| c.id.clone())
        .collect();
    assert_eq!(
        copied,
        Vec::<String>::new(),
        "requests copied from an archived proposal"
    );
}

/// The main spec of role evaluations as the next sync leaves it: with this
/// change's delta merged while the change is active, as it stands after.
fn role_evaluation_spec() -> String {
    use codev_core::merge::{apply_edits, merge_into_existing};
    use codev_core::parser::{parse_delta, parse_spec};
    let main = read("_codev/specs/role-evaluation/spec.md");
    let delta = repo().join("_codev/changes/evaluate-planning/specs/role-evaluation/spec.md");
    let Ok(delta) = fs::read_to_string(delta) else {
        return main;
    };
    let spec = parse_spec(&main).value;
    let plan = merge_into_existing(&main, &spec, &parse_delta(&delta).value, false)
        .unwrap_or_else(|e| panic!("the delta merges: {e}"));
    apply_edits(&main, &plan.edits)
}

#[test]
fn the_role_evaluation_spec_has_no_cut_scenario() {
    let text = role_evaluation_spec();
    let spec = codev_core::parser::parse_spec(&text).value;
    let mut cut = Vec::new();
    for requirement in &spec.requirements {
        for scenario in &requirement.scenarios {
            let body = &scenario.body;
            let starts = body.contains("**GIVEN**") || body.contains("**WHEN**");
            if !starts || !body.contains("**THEN**") {
                cut.push(format!("{} / {}", requirement.name, scenario.name));
            }
        }
    }
    assert_eq!(
        cut,
        Vec::<String>::new(),
        "scenarios without GIVEN/WHEN and THEN"
    );
    let stray: Vec<&str> = text.lines().filter(|l| l.starts_with("` ")).collect();
    assert_eq!(
        stray,
        Vec::<&str>::new(),
        "lines starting with a stray backquote"
    );
    assert!(text.contains("#### Scenario: A report of another run"));
}
