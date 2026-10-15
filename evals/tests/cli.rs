//! The `codev-evals` binary, on the committed baseline: scoring needs no
//! model, writes both reports, and gives the same bytes twice.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use codev_evals::observed::FORMAT;

fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("evals/ has a parent")
        .to_path_buf()
}

fn score(out: &Path) -> i32 {
    let status = Command::new(env!("CARGO_BIN_EXE_codev-evals"))
        .arg("score")
        .arg(repo().join("evals").join("baseline"))
        .arg("--out")
        .arg(out)
        .env_remove("CODEV_EVAL_MIN_RECALL")
        .env_remove("CODEV_EVAL_MAX_FALSE_POSITIVES")
        .env_remove("CODEV_EVAL_MIN_EVIDENCE")
        .output()
        .expect("codev-evals runs");
    status.status.code().expect("an exit code")
}

#[test]
fn score_writes_identical_reports_twice() {
    let tmp = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("codev-evals-score");
    let (first, second) = (tmp.join("first"), tmp.join("second"));
    let _ = fs::remove_dir_all(&tmp);
    let code = score(&first);
    assert!(
        code == 0 || code == 1,
        "a scoring exit code, not usage: {code}"
    );
    assert_eq!(score(&second), code);
    for name in ["report.json", "report.md"] {
        let a = fs::read(first.join(name)).expect(name);
        let b = fs::read(second.join(name)).expect(name);
        assert_eq!(a, b, "{name} differs between two scorings");
    }
}

#[test]
fn an_invalid_threshold_exits_with_two() {
    let output = Command::new(env!("CARGO_BIN_EXE_codev-evals"))
        .arg("score")
        .arg(repo().join("evals").join("baseline"))
        .env("CODEV_EVAL_MIN_RECALL", "lots")
        .output()
        .expect("codev-evals runs");
    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("CODEV_EVAL_MIN_RECALL"));
}

#[test]
fn docs_refuses_a_newer_report_and_writes_nothing() {
    let tmp = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("codev-evals-docs");
    let _ = fs::remove_dir_all(&tmp);
    let baseline = tmp.join("baseline");
    fs::create_dir_all(&baseline).unwrap();
    for lang in ["en", "fr"] {
        fs::create_dir_all(tmp.join("docs").join(lang).join("src")).unwrap();
    }
    let committed = repo().join("evals").join("baseline");
    // The committed report's own format, raised past what this binary reads.
    let text = fs::read_to_string(committed.join("report.json")).unwrap();
    let mut value: serde_json::Value = serde_json::from_str(&text).unwrap();
    value["format"] = serde_json::json!(FORMAT + 1);
    let report = serde_json::to_string(&value).unwrap();
    fs::write(baseline.join("report.json"), report).unwrap();
    fs::copy(committed.join("run.json"), baseline.join("run.json")).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_codev-evals"))
        .arg("--repo")
        .arg(&tmp)
        .arg("docs")
        .arg("--baseline")
        .arg(baseline)
        .output()
        .expect("codev-evals runs");
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("report.json") && stderr.contains(&format!("format {}", FORMAT + 1)),
        "{stderr}"
    );
    for lang in ["en", "fr"] {
        assert!(
            !tmp.join("docs")
                .join(lang)
                .join("src")
                .join("reliability.md")
                .exists(),
            "no page written"
        );
    }
}

fn evals(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_codev-evals"))
        .args(args)
        .output()
        .expect("codev-evals runs")
}

#[test]
fn repeat_zero_exits_with_two() {
    // The flag is checked before any tool or case: no model is called.
    let output = evals(&[
        "run",
        "--repeat",
        "0",
        "--claude",
        "codev-evals-no-such-claude",
    ]);
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("--repeat"), "{stderr}");
}

/// A candidate directory holding the baseline's `run.json` and the
/// observations of `cases`, each edited by `edit`.
fn candidate(
    name: &str,
    cases: &[&str],
    edit: impl Fn(&mut serde_json::Value),
    run_edit: impl Fn(&mut serde_json::Value),
) -> PathBuf {
    let committed = repo().join("evals").join("baseline");
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(name);
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    let mut run: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(committed.join("run.json")).unwrap()).unwrap();
    run_edit(&mut run);
    fs::write(dir.join("run.json"), run.to_string()).unwrap();
    for case in cases {
        let flat = committed.join(case).join("observed.json");
        let source = if flat.is_file() {
            flat
        } else {
            committed.join(case).join("1").join("observed.json")
        };
        let mut observed: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(source).unwrap()).unwrap();
        edit(&mut observed);
        let target = dir.join(case).join("1");
        fs::create_dir_all(&target).unwrap();
        fs::write(target.join("observed.json"), observed.to_string()).unwrap();
    }
    dir
}

fn baseline_dir() -> String {
    repo().join("evals").join("baseline").display().to_string()
}

fn path(dir: &Path) -> String {
    dir.display().to_string()
}

#[test]
fn compare_of_the_baseline_with_itself_fails_on_cost_only() {
    let base = baseline_dir();
    let output = evals(&["compare", &base, &base]);
    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("not lower"), "{stderr}");
    assert_eq!(stderr.lines().count(), 1, "only the cost fails: {stderr}");
}

#[test]
fn compare_quality_only_of_the_baseline_with_itself_passes() {
    let base = baseline_dir();
    let output = evals(&["compare", &base, &base, "--quality-only"]);
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("## Quality") && stdout.contains("Cost (USD)"),
        "{stdout}"
    );
}

#[test]
fn compare_on_a_subset_lists_the_others() {
    let cand = candidate(
        "codev-evals-compare-subset",
        &["security-code-sql-injection", "security-control-docs-only"],
        |_| {},
        |_| {},
    );
    let output = evals(&["compare", &baseline_dir(), &path(&cand), "--quality-only"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Cases compared: 2"), "{stdout}");
    assert!(
        stdout.contains("## Not compared") && stdout.contains("`ops-code-no-timeout`"),
        "{stdout}"
    );
}

#[test]
fn compare_with_another_model_exits_with_two() {
    let cand = candidate(
        "codev-evals-compare-model",
        &["security-code-sql-injection"],
        |_| {},
        |run| run["model"] = serde_json::json!("claude-sonnet-4-5"),
    );
    let output = evals(&["compare", &baseline_dir(), &path(&cand)]);
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty(), "no report");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("claude-sonnet-4-5") && stderr.contains("Claude Code's default"),
        "{stderr}"
    );
}

#[test]
fn compare_with_another_model_used_exits_with_two() {
    let used = |model: &'static str| {
        move |o: &mut serde_json::Value| {
            o["format"] = serde_json::json!(FORMAT);
            o["consumption"] = serde_json::json!({"models": [model], "turns": 3});
        }
    };
    let base = candidate(
        "codev-evals-compare-used-a",
        &["security-code-sql-injection"],
        used("claude-opus-5-5"),
        |_| {},
    );
    let cand = candidate(
        "codev-evals-compare-used-b",
        &["security-code-sql-injection"],
        used("claude-sonnet-5-5"),
        |_| {},
    );
    let output = evals(&["compare", &path(&base), &path(&cand)]);
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("claude-opus-5-5") && stderr.contains("claude-sonnet-5-5"),
        "{stderr}"
    );
}

#[test]
fn compare_without_common_case_exits_with_two() {
    let cand = candidate("codev-evals-compare-none", &[], |_| {}, |_| {});
    let output = evals(&["compare", &baseline_dir(), &path(&cand)]);
    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("no case in common"));
}

#[test]
fn compare_with_a_format_1_baseline_compares_costs() {
    // A cheaper candidate of format 2, same findings, against the
    // baseline as committed; its tokens are compared with `—`.
    let cand = candidate(
        "codev-evals-compare-cheaper",
        &["security-code-sql-injection", "security-control-docs-only"],
        |o| {
            o["format"] = serde_json::json!(FORMAT);
            o["costUsd"] = serde_json::json!(o["costUsd"].as_f64().unwrap() / 2.0);
            o["consumption"] = serde_json::json!({"turns": 5,
                "tokens": {"input": 10, "cacheCreation": 20, "cacheRead": 30, "output": 40}});
        },
        |_| {},
    );
    let base = repo().join("evals").join("baseline");
    let base_is_format_1 = fs::read_to_string(base.join("run.json"))
        .unwrap()
        .contains("\"format\": 1");
    let output = evals(&["compare", &baseline_dir(), &path(&cand)]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(
        output.status.code(),
        Some(0),
        "{stdout}{}",
        String::from_utf8_lossy(&output.stderr)
    );
    if base_is_format_1 {
        assert!(stdout.contains("| Turns | — | 10 |"), "{stdout}");
    }
}

#[test]
fn an_unknown_step_exits_with_two() {
    let output = evals(&[
        "run",
        "--step",
        "plan.writ",
        "--claude",
        "codev-evals-no-such-claude",
    ]);
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("plan.writ") && stderr.contains("plan.write"),
        "{stderr}"
    );
}
