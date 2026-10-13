//! The repository's own suite: its cases, its coverage, its baseline, its
//! workflow and the guides that ask contributors to run it.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use codev_evals::case::coverage_gaps;
use codev_evals::checks::{missing_from_baseline, missing_mention};
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
    assert!(cases.len() >= 26, "{} cases", cases.len());
}

#[test]
fn the_suite_covers_every_reviewing_role() {
    let cases = suite::load_cases(&repo()).unwrap();
    assert_eq!(coverage_gaps(&cases), Vec::<String>::new());
}

#[test]
fn every_case_has_a_baseline_observation() {
    let cases: Vec<String> = suite::load_cases(&repo())
        .unwrap()
        .into_iter()
        .map(|c| c.id)
        .collect();
    let observed: BTreeSet<String> = suite::recorded(&repo().join("evals").join("baseline"));
    assert_eq!(
        missing_from_baseline(&cases, &observed),
        Vec::<String>::new()
    );
}

#[test]
fn the_baseline_agrees_with_the_scoring() {
    let cases = suite::load_cases(&repo()).unwrap();
    let score = suite::score_dir(
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
    assert_eq!(gaps, Vec::<String>::new());
}
