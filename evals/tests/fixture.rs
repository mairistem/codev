//! A planning case's fixture, built from a repository's history with a
//! stand-in `codev`: the state at the case's `ref`, the skills of the codev
//! under test.

#![cfg(unix)]

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;

use codev_evals::case::{Case, Kind, PlanCase, Step};
use codev_evals::runner::{self, Tools};

fn git(dir: &Path, args: &[&str]) -> String {
    let output = Command::new("git")
        .args(runner::git_args(args))
        .current_dir(dir)
        .output()
        .expect("git runs");
    assert!(output.status.success(), "git {args:?}: {output:?}");
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn write(path: &Path, text: &str) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, text).unwrap();
}

#[test]
fn the_planning_fixture_holds_the_ref_and_the_skills_of_the_codev_under_test() {
    let root = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("planning-fixture");
    let _ = fs::remove_dir_all(&root);
    let repo = root.join("repo");
    fs::create_dir_all(&repo).unwrap();
    git(&repo, &["init", "-q", "-b", "main"]);
    write(&repo.join("_codev/specs/a/spec.md"), "first state\n");
    write(
        &repo.join(".claude/skills/codev-propose/SKILL.md"),
        "old skill\n",
    );
    git(&repo, &["add", "-A"]);
    git(&repo, &["commit", "-q", "-m", "one"]);
    let first = git(&repo, &["rev-parse", "HEAD"]);
    write(&repo.join("_codev/specs/a/spec.md"), "second state\n");
    git(&repo, &["commit", "-q", "-am", "two"]);

    // The codev under test: `update --force` renders its skills.
    let bin = root.join("bin");
    fs::create_dir_all(&bin).unwrap();
    let codev: PathBuf = bin.join("codev");
    fs::write(
        &codev,
        "#!/bin/sh\n[ \"$1 $2\" = \"update --force\" ] || exit 3\n\
         printf 'skill of the codev under test\\n' > .claude/skills/codev-propose/SKILL.md\n",
    )
    .unwrap();
    fs::set_permissions(&codev, fs::Permissions::from_mode(0o755)).unwrap();

    let case = Case {
        id: "plan-x".into(),
        role: "planner".into(),
        step: Step::PlanWrite,
        change: String::new(),
        stacks: Vec::new(),
        kind: Kind::Seeded,
        summary: String::new(),
        expected: Vec::new(),
        fixed: Vec::new(),
        unchanged: Vec::new(),
        planned: Vec::new(),
        plan: Some(PlanCase {
            git_ref: first,
            request: "r".into(),
            modified: Vec::new(),
            collateral: Vec::new(),
            decisions: Vec::new(),
        }),
        update: None,
    };
    let tools = Tools {
        claude: PathBuf::from("claude"),
        codev,
        git: PathBuf::from("git"),
    };
    let dir = root.join("work").join("plan-x-1");
    runner::setup_plan(&case, &repo, &dir, &tools).unwrap();

    let read = |p: &str| fs::read_to_string(dir.join(p)).unwrap();
    assert_eq!(read("_codev/specs/a/spec.md"), "first state\n");
    assert_eq!(
        read(".claude/skills/codev-propose/SKILL.md"),
        "skill of the codev under test\n"
    );
    assert_eq!(git(&dir, &["status", "--porcelain"]), "");
    assert_eq!(git(&dir, &["rev-list", "--count", "HEAD"]), "2");
}

#[test]
fn the_update_fixture_lays_the_change_on_the_ref() {
    use codev_evals::case::{Artifact, UpdateCase};
    use std::collections::BTreeMap;

    let root = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("update-fixture");
    let _ = fs::remove_dir_all(&root);
    let repo = root.join("repo");
    fs::create_dir_all(&repo).unwrap();
    git(&repo, &["init", "-q", "-b", "main"]);
    write(&repo.join("_codev/specs/a/spec.md"), "before the change\n");
    git(&repo, &["add", "-A"]);
    git(&repo, &["commit", "-q", "-m", "one"]);
    let first = git(&repo, &["rev-parse", "HEAD"]);
    write(&repo.join("_codev/specs/a/spec.md"), "after the change\n");
    git(&repo, &["commit", "-q", "-am", "two"]);

    let bin = root.join("bin");
    fs::create_dir_all(&bin).unwrap();
    let codev: PathBuf = bin.join("codev");
    fs::write(
        &codev,
        "#!/bin/sh\n[ \"$1 $2\" = \"update --force\" ] || exit 3\n\
         mkdir -p .claude/skills/codev-update\n\
         printf 'skill of the codev under test\\n' > .claude/skills/codev-update/SKILL.md\n",
    )
    .unwrap();
    fs::set_permissions(&codev, fs::Permissions::from_mode(0o755)).unwrap();

    let case = Case {
        id: "update-x".into(),
        role: "planner".into(),
        step: Step::PlanUpdate,
        change: "add-auth".into(),
        stacks: Vec::new(),
        kind: Kind::Seeded,
        summary: String::new(),
        expected: Vec::new(),
        fixed: Vec::new(),
        unchanged: Vec::new(),
        planned: Vec::new(),
        plan: None,
        update: Some(UpdateCase {
            git_ref: first,
            artifact: Artifact::Design,
            request: "r".into(),
            revised: vec!["design.md".into()],
            kept: Vec::new(),
            ripple: Vec::new(),
            contains: Vec::new(),
            refuse: false,
            files: BTreeMap::from([
                ("design.md".to_string(), "# Design\n".to_string()),
                ("specs/a/spec.md".to_string(), "# Delta\n".to_string()),
            ]),
        }),
    };
    let tools = Tools {
        claude: PathBuf::from("claude"),
        codev,
        git: PathBuf::from("git"),
    };
    let dir = root.join("work").join("update-x-1");
    runner::setup_plan(&case, &repo, &dir, &tools).unwrap();

    let read = |p: &str| fs::read_to_string(dir.join(p)).unwrap();
    assert_eq!(read("_codev/specs/a/spec.md"), "before the change\n");
    assert_eq!(read("_codev/changes/add-auth/design.md"), "# Design\n");
    assert_eq!(read("_codev/changes/add-auth/specs/a/spec.md"), "# Delta\n");
    assert_eq!(
        read(".claude/skills/codev-update/SKILL.md"),
        "skill of the codev under test\n"
    );
    assert_eq!(git(&dir, &["status", "--porcelain"]), "");
}
