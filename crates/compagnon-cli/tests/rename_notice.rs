//! The binary run under the product's former name: one notice on stderr,
//! the command otherwise unchanged. `argv[0]` is set without copying the
//! binary, which only Unix allows.
#![cfg(unix)]

use std::os::unix::process::CommandExt;
use std::process::Command;

use compagnon_core::legacy::BINARY_STEM;

fn run_as_former_name(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_compagnon"))
        .arg0(BINARY_STEM)
        .args(args)
        // A directory that is no project: the command answers the same way
        // wherever the test suite runs.
        .current_dir("/")
        .env("COMPAGNON_NO_UPDATE_CHECK", "1")
        .output()
        .expect("the binary runs")
}

#[test]
fn running_as_codev_prints_the_rename_notice_on_stderr() {
    let out = run_as_former_name(&["--version"]);
    assert!(out.status.success());
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(stderr.lines().count(), 1, "{stderr}");
    assert!(stderr.contains("compagnon migrate"), "{stderr}");
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains(env!("CARGO_PKG_VERSION")), "{stdout}");
}

#[test]
fn running_as_codev_keeps_json_stdout_parseable() {
    let out = run_as_former_name(&["status", "--json"]);
    let stdout = String::from_utf8(out.stdout).expect("UTF-8");
    let document: serde_json::Value = serde_json::from_str(&stdout).expect("one JSON document");
    assert!(document.get("status").is_some(), "{stdout}");
    assert!(String::from_utf8_lossy(&out.stderr).contains("compagnon"));
}

#[test]
fn running_as_compagnon_prints_no_notice() {
    let out = Command::new(env!("CARGO_BIN_EXE_compagnon"))
        .arg("--version")
        .output()
        .expect("the binary runs");
    assert!(
        out.stderr.is_empty(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
}
