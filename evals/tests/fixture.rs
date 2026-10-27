//! The fixtures of the cases the runner builds itself, with a stand-in
//! `codev`: the state at a planning, update or sync case's `ref`, the change
//! and the overlay laid on it, the skills of the codev under test, the
//! reference command run on a copy, and a configure case's project.

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
            config: None,
            keyword_language: None,
        }),
        update: None,
        implement: None,
        merge: None,
        configure: None,
    };
    let tools = Tools {
        claude: PathBuf::from("claude"),
        codev,
        git: PathBuf::from("git"),
    };
    let dir = root.join("work").join("plan-x-1");
    runner::setup_plan(&case, &repo, &dir, &tools, None).unwrap();

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
    write(&repo.join("_codev/specs/a/spec.md"), "at the ref\n");
    git(&repo, &["add", "-A"]);
    git(&repo, &["commit", "-q", "-m", "one"]);
    let first = git(&repo, &["rev-parse", "HEAD"]);
    write(&repo.join("_codev/specs/a/spec.md"), "after the ref\n");
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
        implement: None,
        merge: None,
        configure: None,
    };
    let tools = Tools {
        claude: PathBuf::from("claude"),
        codev,
        git: PathBuf::from("git"),
    };
    let dir = root.join("work").join("update-x-1");
    runner::setup_plan(&case, &repo, &dir, &tools, None).unwrap();

    let read = |p: &str| fs::read_to_string(dir.join(p)).unwrap();
    assert_eq!(read("_codev/specs/a/spec.md"), "at the ref\n");
    assert_eq!(read("_codev/changes/add-auth/design.md"), "# Design\n");
    assert_eq!(read("_codev/changes/add-auth/specs/a/spec.md"), "# Delta\n");
    assert_eq!(
        read(".claude/skills/codev-update/SKILL.md"),
        "skill of the codev under test\n"
    );
    assert_eq!(git(&dir, &["status", "--porcelain"]), "");
}

/// A case's `Case`, with only what the fixture needs.
fn bare_case(id: &str, step: Step, change: &str) -> Case {
    Case {
        id: id.into(),
        role: "planner".into(),
        step,
        change: change.into(),
        stacks: Vec::new(),
        kind: Kind::Seeded,
        summary: String::new(),
        expected: Vec::new(),
        fixed: Vec::new(),
        unchanged: Vec::new(),
        planned: Vec::new(),
        plan: None,
        update: None,
        implement: None,
        merge: None,
        configure: None,
    }
}

#[test]
fn the_sync_reference_runs_on_a_copy() {
    use codev_evals::case::{MergeCase, MergeOutcome};
    use std::collections::BTreeMap;

    let root = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("sync-reference");
    let _ = fs::remove_dir_all(&root);
    let repo = root.join("repo");
    fs::create_dir_all(&repo).unwrap();
    git(&repo, &["init", "-q", "-b", "main"]);
    write(&repo.join("_codev/specs/doctor/spec.md"), "at the ref\n");
    write(
        &repo.join("_codev/config.yaml"),
        "codev_version: \">=0.6\"\n",
    );
    git(&repo, &["add", "-A"]);
    git(&repo, &["commit", "-q", "-m", "one"]);
    let first = git(&repo, &["rev-parse", "HEAD"]);

    // The codev under test: `update --force` renders the skills; any other
    // command records its arguments next to the case's directories, merges
    // a requirement into the doctor spec and prints its report.
    let bin = root.join("bin");
    fs::create_dir_all(&bin).unwrap();
    let codev: PathBuf = bin.join("codev");
    fs::write(
        &codev,
        "#!/bin/sh\n\
         if [ \"$1 $2\" = \"update --force\" ]; then\n\
           mkdir -p .claude/skills/codev-sync\n\
           printf 'skill\\n' > .claude/skills/codev-sync/SKILL.md\n\
           exit 0\n\
         fi\n\
         printf '%s\\n' \"$*\" > ../received-args\n\
         printf 'merged\\n' >> _codev/specs/doctor/spec.md\n\
         printf '{\"changeName\":\"add-auth\",\"root\":\"%s\",\"updated\":[\"%s/_codev/specs/doctor/spec.md\"],\"status\":[]}\\n' \"$PWD\" \"$PWD\"\n",
    )
    .unwrap();
    fs::set_permissions(&codev, fs::Permissions::from_mode(0o755)).unwrap();

    let mut case = bare_case("sync-x", Step::ChangeSync, "add-auth");
    case.merge = Some(MergeCase {
        git_ref: first,
        outcome: MergeOutcome::Done,
        code: None,
        contains: Vec::new(),
        absent: Vec::new(),
        files: BTreeMap::from([(
            "specs/doctor/spec.md".to_string(),
            "## MODIFIED Requirements\n".to_string(),
        )]),
        overlay: BTreeMap::from([(
            "_codev/config.yaml".to_string(),
            b"codev_version: \">=99\"\n".to_vec(),
        )]),
    });
    let tools = Tools {
        claude: PathBuf::from("claude"),
        codev,
        git: PathBuf::from("git"),
    };
    let dir = root.join("work").join("sync-x-1");
    runner::setup_plan(&case, &repo, &dir, &tools, None).unwrap();
    let read = |p: &str| fs::read_to_string(dir.join(p)).unwrap();
    assert_eq!(read("_codev/config.yaml"), "codev_version: \">=99\"\n");

    let before = runner::codev_digests(&dir);
    let reference = runner::make_reference(&case, &dir, &before, &tools).unwrap();
    assert_eq!(
        fs::read_to_string(root.join("work").join("received-args")).unwrap(),
        "sync --change add-auth --json\n"
    );
    assert_eq!(reference.exit_code, Some(0));
    let report = reference.report.expect("a report");
    assert_eq!(report["updated"][0], "_codev/specs/doctor/spec.md");
    assert!(report.get("root").is_none());
    assert_eq!(
        reference.changed,
        BTreeMap::from([(
            "_codev/specs/doctor/spec.md".to_string(),
            Some(codev_core::update::sha256_hex(b"at the ref\nmerged\n")),
        )])
    );
    // The case's own directory is as the fixture's commit left it, and the
    // copy is gone.
    assert_eq!(read("_codev/specs/doctor/spec.md"), "at the ref\n");
    assert_eq!(git(&dir, &["status", "--porcelain"]), "");
    assert!(!root.join("work").join("sync-x-1-reference").exists());
}

#[test]
fn the_configure_fixture_without_a_configuration_keeps_the_skills() {
    let root = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("configure-fixture");
    let _ = fs::remove_dir_all(&root);
    let case_dir = root.join("case");
    write(&case_dir.join("base/main.go"), "package main\n");
    // The codev under test: `init` writes a configuration and the skills.
    let bin = root.join("bin");
    fs::create_dir_all(&bin).unwrap();
    let codev: PathBuf = bin.join("codev");
    fs::write(
        &codev,
        "#!/bin/sh\n[ \"$1\" = \"init\" ] || exit 3\n\
         mkdir -p _codev .claude/skills/codev-configure\n\
         printf 'schema: spec-driven\\n' > _codev/config.yaml\n\
         printf 'skill\\n' > .claude/skills/codev-configure/SKILL.md\n",
    )
    .unwrap();
    fs::set_permissions(&codev, fs::Permissions::from_mode(0o755)).unwrap();
    let tools = Tools {
        claude: PathBuf::from("claude"),
        codev,
        git: PathBuf::from("git"),
    };
    let dir = root.join("work").join("configure-x-1");
    runner::setup_configure(&case_dir, &dir, &tools).unwrap();
    assert_eq!(
        fs::read_to_string(dir.join(".claude/skills/codev-configure/SKILL.md")).unwrap(),
        "skill\n"
    );
    assert!(!dir.join("_codev").exists());
    assert_eq!(
        fs::read_to_string(dir.join("main.go")).unwrap(),
        "package main\n"
    );
    assert_eq!(git(&dir, &["status", "--porcelain"]), "");

    // With a configuration, `update --force` installs the skills and the
    // case's file stays as written.
    write(
        &case_dir.join("base/_codev/config.yaml"),
        "# kept\nschema: spec-driven\n",
    );
    fs::write(
        &tools.codev,
        "#!/bin/sh\n[ \"$1 $2\" = \"update --force\" ] || exit 3\n\
         mkdir -p .claude/skills/codev-configure\n\
         printf 'skill\\n' > .claude/skills/codev-configure/SKILL.md\n",
    )
    .unwrap();
    runner::setup_configure(&case_dir, &dir, &tools).unwrap();
    assert_eq!(
        fs::read_to_string(dir.join("_codev/config.yaml")).unwrap(),
        "# kept\nschema: spec-driven\n"
    );
    assert!(dir.join(".claude/skills/codev-configure/SKILL.md").exists());
}
