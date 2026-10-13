//! The `codev-evals` binary, on the committed baseline: scoring needs no
//! model, writes both reports, and gives the same bytes twice.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

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
    let report = fs::read_to_string(committed.join("report.json"))
        .unwrap()
        .replacen("\"format\": 1", "\"format\": 2", 1);
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
        stderr.contains("report.json") && stderr.contains("format 2"),
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
