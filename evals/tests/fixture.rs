//! The fixtures of the cases the runner builds itself, with a stand-in
//! `compagnon`: the state at a planning, update or sync case's `ref`, the change
//! and the overlay laid on it, the skills of the compagnon under test, the
//! reference command run on a copy, and a configure case's project.

#![cfg(unix)]

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;

use compagnon_core::decisions::seal;
use compagnon_evals::case::{Case, Kind, PlanCase, Step};
use compagnon_evals::runner::{self, Tools};

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
fn the_planning_fixture_holds_the_ref_and_the_skills_of_the_compagnon_under_test() {
    let root = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("planning-fixture");
    let _ = fs::remove_dir_all(&root);
    let repo = root.join("repo");
    fs::create_dir_all(&repo).unwrap();
    git(&repo, &["init", "-q", "-b", "main"]);
    write(&repo.join("_compagnon/specs/a/spec.md"), "first state\n");
    write(
        &repo.join(".claude/skills/compagnon-propose/SKILL.md"),
        "old skill\n",
    );
    git(&repo, &["add", "-A"]);
    git(&repo, &["commit", "-q", "-m", "one"]);
    let first = git(&repo, &["rev-parse", "HEAD"]);
    write(&repo.join("_compagnon/specs/a/spec.md"), "second state\n");
    git(&repo, &["commit", "-q", "-am", "two"]);

    // The compagnon under test: `update --force` renders its skills.
    let bin = root.join("bin");
    fs::create_dir_all(&bin).unwrap();
    let compagnon: PathBuf = bin.join("compagnon");
    fs::write(
        &compagnon,
        "#!/bin/sh\n[ \"$1 $2\" = \"update --force\" ] || exit 3\n\
         printf 'skill of the compagnon under test\\n' > .claude/skills/compagnon-propose/SKILL.md\n",
    )
    .unwrap();
    fs::set_permissions(&compagnon, fs::Permissions::from_mode(0o755)).unwrap();

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
        read_only: None,
    };
    let tools = Tools {
        claude: PathBuf::from("claude"),
        compagnon,
        git: PathBuf::from("git"),
    };
    let dir = root.join("work").join("plan-x-1");
    runner::setup_plan(&case, &repo, &dir, &tools, None).unwrap();

    let read = |p: &str| fs::read_to_string(dir.join(p)).unwrap();
    assert_eq!(read("_compagnon/specs/a/spec.md"), "first state\n");
    assert_eq!(
        read(".claude/skills/compagnon-propose/SKILL.md"),
        "skill of the compagnon under test\n"
    );
    assert_eq!(git(&dir, &["status", "--porcelain"]), "");
    assert_eq!(git(&dir, &["rev-list", "--count", "HEAD"]), "2");
}

#[test]
fn the_update_fixture_lays_the_change_on_the_ref() {
    use compagnon_evals::case::{Artifact, UpdateCase};
    use std::collections::BTreeMap;

    let root = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("update-fixture");
    let _ = fs::remove_dir_all(&root);
    let repo = root.join("repo");
    fs::create_dir_all(&repo).unwrap();
    git(&repo, &["init", "-q", "-b", "main"]);
    write(&repo.join("_compagnon/specs/a/spec.md"), "at the ref\n");
    git(&repo, &["add", "-A"]);
    git(&repo, &["commit", "-q", "-m", "one"]);
    let first = git(&repo, &["rev-parse", "HEAD"]);
    write(&repo.join("_compagnon/specs/a/spec.md"), "after the ref\n");
    git(&repo, &["commit", "-q", "-am", "two"]);

    let bin = root.join("bin");
    fs::create_dir_all(&bin).unwrap();
    let compagnon: PathBuf = bin.join("compagnon");
    fs::write(
        &compagnon,
        "#!/bin/sh\n[ \"$1 $2\" = \"update --force\" ] || exit 3\n\
         mkdir -p .claude/skills/compagnon-update\n\
         printf 'skill of the compagnon under test\\n' > .claude/skills/compagnon-update/SKILL.md\n",
    )
    .unwrap();
    fs::set_permissions(&compagnon, fs::Permissions::from_mode(0o755)).unwrap();

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
        read_only: None,
    };
    let tools = Tools {
        claude: PathBuf::from("claude"),
        compagnon,
        git: PathBuf::from("git"),
    };
    let dir = root.join("work").join("update-x-1");
    runner::setup_plan(&case, &repo, &dir, &tools, None).unwrap();

    let read = |p: &str| fs::read_to_string(dir.join(p)).unwrap();
    assert_eq!(read("_compagnon/specs/a/spec.md"), "at the ref\n");
    assert_eq!(read("_compagnon/changes/add-auth/design.md"), "# Design\n");
    assert_eq!(
        read("_compagnon/changes/add-auth/specs/a/spec.md"),
        "# Delta\n"
    );
    assert_eq!(
        read(".claude/skills/compagnon-update/SKILL.md"),
        "skill of the compagnon under test\n"
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
        read_only: None,
    }
}

#[test]
fn the_sync_reference_runs_on_a_copy() {
    use compagnon_evals::case::{MergeCase, MergeOutcome};
    use std::collections::BTreeMap;

    let root = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("sync-reference");
    let _ = fs::remove_dir_all(&root);
    let repo = root.join("repo");
    fs::create_dir_all(&repo).unwrap();
    git(&repo, &["init", "-q", "-b", "main"]);
    write(
        &repo.join("_compagnon/specs/doctor/spec.md"),
        "at the ref\n",
    );
    write(
        &repo.join("_compagnon/config.yaml"),
        "compagnon_version: \">=0.6\"\n",
    );
    git(&repo, &["add", "-A"]);
    git(&repo, &["commit", "-q", "-m", "one"]);
    let first = git(&repo, &["rev-parse", "HEAD"]);

    // The compagnon under test: `update --force` renders the skills; any other
    // command records its arguments next to the case's directories, merges
    // a requirement into the doctor spec and prints its report.
    let bin = root.join("bin");
    fs::create_dir_all(&bin).unwrap();
    let compagnon: PathBuf = bin.join("compagnon");
    fs::write(
        &compagnon,
        "#!/bin/sh\n\
         if [ \"$1 $2\" = \"update --force\" ]; then\n\
           mkdir -p .claude/skills/compagnon-sync\n\
           printf 'skill\\n' > .claude/skills/compagnon-sync/SKILL.md\n\
           exit 0\n\
         fi\n\
         printf '%s\\n' \"$*\" > ../received-args\n\
         printf 'merged\\n' >> _compagnon/specs/doctor/spec.md\n\
         printf '{\"changeName\":\"add-auth\",\"root\":\"%s\",\"updated\":[\"%s/_compagnon/specs/doctor/spec.md\"],\"status\":[]}\\n' \"$PWD\" \"$PWD\"\n",
    )
    .unwrap();
    fs::set_permissions(&compagnon, fs::Permissions::from_mode(0o755)).unwrap();

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
            "_compagnon/config.yaml".to_string(),
            b"compagnon_version: \">=99\"\n".to_vec(),
        )]),
    });
    let tools = Tools {
        claude: PathBuf::from("claude"),
        compagnon,
        git: PathBuf::from("git"),
    };
    let dir = root.join("work").join("sync-x-1");
    runner::setup_plan(&case, &repo, &dir, &tools, None).unwrap();
    let read = |p: &str| fs::read_to_string(dir.join(p)).unwrap();
    assert_eq!(
        read("_compagnon/config.yaml"),
        "compagnon_version: \">=99\"\n"
    );

    let before = runner::compagnon_digests(&dir);
    let reference = runner::make_reference(&case, &dir, &before, &tools).unwrap();
    assert_eq!(
        fs::read_to_string(root.join("work").join("received-args")).unwrap(),
        "sync --change add-auth --json\n"
    );
    assert_eq!(reference.exit_code, Some(0));
    let report = reference.report.expect("a report");
    assert_eq!(report["updated"][0], "_compagnon/specs/doctor/spec.md");
    assert!(report.get("root").is_none());
    assert_eq!(
        reference.changed,
        BTreeMap::from([(
            "_compagnon/specs/doctor/spec.md".to_string(),
            Some(compagnon_core::update::sha256_hex(b"at the ref\nmerged\n")),
        )])
    );
    // The case's own directory is as the fixture's commit left it, and the
    // copy is gone.
    assert_eq!(read("_compagnon/specs/doctor/spec.md"), "at the ref\n");
    assert_eq!(git(&dir, &["status", "--porcelain"]), "");
    assert!(!root.join("work").join("sync-x-1-reference").exists());
}

#[test]
fn the_configure_fixture_without_a_configuration_keeps_the_skills() {
    let root = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("configure-fixture");
    let _ = fs::remove_dir_all(&root);
    let case_dir = root.join("case");
    write(&case_dir.join("base/main.go"), "package main\n");
    // The compagnon under test: `init` writes a configuration and the skills.
    let bin = root.join("bin");
    fs::create_dir_all(&bin).unwrap();
    let compagnon: PathBuf = bin.join("compagnon");
    fs::write(
        &compagnon,
        "#!/bin/sh\n[ \"$1\" = \"init\" ] || exit 3\n\
         mkdir -p _compagnon .claude/skills/compagnon-configure\n\
         printf 'schema: spec-driven\\n' > _compagnon/config.yaml\n\
         printf 'skill\\n' > .claude/skills/compagnon-configure/SKILL.md\n",
    )
    .unwrap();
    fs::set_permissions(&compagnon, fs::Permissions::from_mode(0o755)).unwrap();
    let tools = Tools {
        claude: PathBuf::from("claude"),
        compagnon,
        git: PathBuf::from("git"),
    };
    let dir = root.join("work").join("configure-x-1");
    runner::setup_configure(&case_dir, &dir, &tools).unwrap();
    assert_eq!(
        fs::read_to_string(dir.join(".claude/skills/compagnon-configure/SKILL.md")).unwrap(),
        "skill\n"
    );
    assert!(!dir.join("_compagnon").exists());
    assert_eq!(
        fs::read_to_string(dir.join("main.go")).unwrap(),
        "package main\n"
    );
    assert_eq!(git(&dir, &["status", "--porcelain"]), "");

    // With a configuration, `update --force` installs the skills and the
    // case's file stays as written.
    write(
        &case_dir.join("base/_compagnon/config.yaml"),
        "# kept\nschema: spec-driven\n",
    );
    fs::write(
        &tools.compagnon,
        "#!/bin/sh\n[ \"$1 $2\" = \"update --force\" ] || exit 3\n\
         mkdir -p .claude/skills/compagnon-configure\n\
         printf 'skill\\n' > .claude/skills/compagnon-configure/SKILL.md\n",
    )
    .unwrap();
    runner::setup_configure(&case_dir, &dir, &tools).unwrap();
    assert_eq!(
        fs::read_to_string(dir.join("_compagnon/config.yaml")).unwrap(),
        "# kept\nschema: spec-driven\n"
    );
    assert!(
        dir.join(".claude/skills/compagnon-configure/SKILL.md")
            .exists()
    );
}

#[test]
fn an_onboard_case_without_compagnon() {
    let root = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("onboard-fixture");
    let _ = fs::remove_dir_all(&root);
    let case_dir = root.join("case");
    write(&case_dir.join("base/README.md"), "# Notes\n");
    // The compagnon under test: `init` writes a configuration and the skills.
    let bin = root.join("bin");
    fs::create_dir_all(&bin).unwrap();
    let compagnon: PathBuf = bin.join("compagnon");
    fs::write(
        &compagnon,
        "#!/bin/sh\n[ \"$1\" = \"init\" ] || exit 3\n\
         mkdir -p _compagnon/changes .claude/skills/compagnon-onboard\n\
         printf 'schema: spec-driven\\n' > _compagnon/config.yaml\n\
         printf 'skill\\n' > .claude/skills/compagnon-onboard/SKILL.md\n",
    )
    .unwrap();
    fs::set_permissions(&compagnon, fs::Permissions::from_mode(0o755)).unwrap();
    let tools = Tools {
        claude: PathBuf::from("claude"),
        compagnon,
        git: PathBuf::from("git"),
    };
    let dir = root.join("work").join("onboard-x-1");
    runner::setup_configure(&case_dir, &dir, &tools).unwrap();
    assert_eq!(
        fs::read_to_string(dir.join(".claude/skills/compagnon-onboard/SKILL.md")).unwrap(),
        "skill\n"
    );
    assert!(!dir.join("_compagnon").exists());
    assert_eq!(
        fs::read_to_string(dir.join("README.md")).unwrap(),
        "# Notes\n"
    );
    assert_eq!(git(&dir, &["status", "--porcelain"]), "");
    assert!(runner::change_folders(&dir).is_empty());
}

#[test]
fn a_case_pinned_before_the_rename_is_migrated_before_it_runs() {
    use compagnon_core::legacy;
    use compagnon_evals::case::{Artifact, UpdateCase};
    use std::collections::BTreeMap;

    let root = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("pinned-before-rename");
    let _ = fs::remove_dir_all(&root);
    let repo = root.join("repo");
    fs::create_dir_all(&repo).unwrap();
    git(&repo, &["init", "-q", "-b", "main"]);
    // The state at the ref: the former planning root, the former names.
    let stem = legacy::BINARY_STEM;
    write(
        &repo.join(legacy::PLANNING_DIR).join("specs/a/spec.md"),
        &format!("### Requirement: {stem} validates\n\nRun `{stem} validate`.\n"),
    );
    write(
        &repo.join(format!("crates/{stem}-core/Cargo.toml")),
        &format!("[package]\nname = \"{stem}-core\"\n"),
    );
    // Two sealed decisions naming the former name: the first matching its
    // seal, the second edited after it.
    let decisions = repo.join(legacy::PLANNING_DIR).join("decisions");
    let adr =
        |id: &str| format!("---\nid: {id}\nstatus: accepted\n---\n\nRun `{stem} validate`.\n");
    write(&decisions.join("0001-a.md"), &adr("0001"));
    write(&decisions.join("0002-b.md"), &adr("0002"));
    let stale = "sha256:0000000000000000000000000000000000000000000000000000000000000000";
    write(
        &decisions.join("seal.yaml"),
        &seal::render_seal_file(&seal::SealFile {
            version: seal::SEAL_FILE_VERSION,
            seals: vec![
                seal::Seal {
                    id: "0001".into(),
                    body_sha256: seal::body_hash(&adr("0001")).unwrap(),
                    sealed_at: "2026-09-30".into(),
                },
                seal::Seal {
                    id: "0002".into(),
                    body_sha256: stale.into(),
                    sealed_at: "2026-09-30".into(),
                },
            ],
        }),
    );
    // A workspace whose formatted line the longer name pushes past the
    // width, and a module it declares, not formatted, the rename leaves.
    write(
        &repo.join("Cargo.toml"),
        "[workspace]\nmembers = [\"crates/tool\"]\n\n[workspace.package]\nedition = \"2024\"\n",
    );
    write(
        &repo.join("crates/tool/Cargo.toml"),
        "[package]\nname = \"tool\"\nedition.workspace = true\n",
    );
    let lib = format!(
        "mod untouched;\n\npub fn f() -> std::path::PathBuf {{\n    \
         std::path::Path::new(\"/p/.claude/skills/{stem}-propose/SKILL.md\").join(\"aaaaaaaaaaaaaaaaaaaa\")\n}}\n"
    );
    write(&repo.join("crates/tool/src/lib.rs"), &lib);
    let untouched = "fn  g( ) {}\n";
    write(&repo.join("crates/tool/src/untouched.rs"), untouched);
    assert!(rustfmt_checks(&repo.join("crates/tool/src/lib.rs")));
    git(&repo, &["add", "-A"]);
    git(&repo, &["commit", "-q", "-m", "before the rename"]);
    let pinned = git(&repo, &["rev-parse", "HEAD"]);

    let bin = root.join("bin");
    fs::create_dir_all(&bin).unwrap();
    let compagnon: PathBuf = bin.join("compagnon");
    fs::write(&compagnon, "#!/bin/sh\nexit 0\n").unwrap();
    fs::set_permissions(&compagnon, fs::Permissions::from_mode(0o755)).unwrap();

    let mut case = bare_case("update-pinned", Step::PlanUpdate, "add-auth");
    case.update = Some(UpdateCase {
        git_ref: pinned,
        artifact: Artifact::Design,
        request: "r".into(),
        revised: vec!["design.md".into()],
        kept: Vec::new(),
        ripple: Vec::new(),
        contains: Vec::new(),
        refuse: false,
        files: BTreeMap::from([("design.md".to_string(), "# Design\n".to_string())]),
    });
    let tools = Tools {
        claude: PathBuf::from("claude"),
        compagnon,
        git: PathBuf::from("git"),
    };
    let dir = root.join("work").join("update-pinned-1");
    runner::setup_plan(&case, &repo, &dir, &tools, None).unwrap();

    let read = |p: &str| fs::read_to_string(dir.join(p)).unwrap();
    assert_eq!(
        read("_compagnon/specs/a/spec.md"),
        "### Requirement: compagnon validates\n\nRun `compagnon validate`.\n"
    );
    assert_eq!(
        read("crates/compagnon-core/Cargo.toml"),
        "[package]\nname = \"compagnon-core\"\n"
    );
    assert_eq!(read("_compagnon/changes/add-auth/design.md"), "# Design\n");
    assert!(!dir.join(legacy::PLANNING_DIR).exists());
    assert!(!dir.join(format!("crates/{stem}-core")).exists());

    // The seal the rename alone broke holds again; the one already broken
    // stays as the commit held it.
    let seals = seal::parse_seal_file(&read("_compagnon/decisions/seal.yaml")).unwrap();
    let renamed = read("_compagnon/decisions/0001-a.md");
    assert!(renamed.contains("compagnon validate"));
    assert_eq!(
        seals.find("0001").unwrap().body_sha256,
        seal::body_hash(&renamed).unwrap()
    );
    assert_eq!(seals.find("0002").unwrap().body_sha256, stale);

    // The renamed file is formatted again; the module it declares is not
    // touched.
    assert!(rustfmt_checks(&dir.join("crates/tool/src/lib.rs")));
    assert_eq!(read("crates/tool/src/untouched.rs"), untouched);
}

/// Whether `rustfmt --check` passes on `file` alone, in edition 2024.
fn rustfmt_checks(file: &Path) -> bool {
    let text = fs::read_to_string(file).unwrap();
    let mut child = Command::new("rustfmt")
        .args(["--edition", "2024", "--check"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::null())
        .spawn()
        .expect("rustfmt runs");
    use std::io::Write;
    child
        .stdin
        .take()
        .unwrap()
        .write_all(text.as_bytes())
        .unwrap();
    child.wait().unwrap().success()
}

#[test]
fn an_unparsable_seal_file_is_left_as_it_is() {
    use compagnon_core::legacy;
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("unparsable-seal");
    let _ = fs::remove_dir_all(&dir);
    let decisions = dir.join(legacy::PLANNING_DIR).join("decisions");
    write(
        &decisions.join("0001-a.md"),
        "---\nid: 0001\n---\n\nA body.\n",
    );
    write(
        &decisions.join("seal.yaml"),
        "version: 1\nseals: [ not a list\n",
    );
    runner::bring_through_rename(&dir).unwrap();
    assert_eq!(
        fs::read_to_string(dir.join("_compagnon/decisions/seal.yaml")).unwrap(),
        "version: 1\nseals: [ not a list\n"
    );
}

#[test]
fn a_tree_under_the_current_names_is_left_as_it_is() {
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("current-names");
    let _ = fs::remove_dir_all(&dir);
    let seal =
        "version: 1\nseals:\n- id: '0001'\n  bodySha256: sha256:00\n  sealedAt: 2026-09-30\n";
    write(&dir.join("_compagnon/decisions/seal.yaml"), seal);
    write(
        &dir.join("_compagnon/decisions/0001-a.md"),
        "---\nid: 0001\n---\n\nBody.\n",
    );
    write(&dir.join("Cargo.toml"), "[package]\nname = \"x\"\n");
    write(&dir.join("src/lib.rs"), "fn  g( ) {}\n");
    runner::bring_through_rename(&dir).unwrap();
    let read = |p: &str| fs::read_to_string(dir.join(p)).unwrap();
    assert_eq!(read("_compagnon/decisions/seal.yaml"), seal);
    assert_eq!(read("src/lib.rs"), "fn  g( ) {}\n");
}
