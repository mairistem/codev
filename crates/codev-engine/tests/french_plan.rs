//! A plan written entirely in French, from codev's French templates,
//! validates without a finding and merges into a new French main spec.
//!
//! The templates are filled as an agent would fill them: each `<…>`
//! placeholder replaced by a sentence, the guidance comments removed. Run
//! on an in-memory project with `language: fr`, through the engine's own
//! validation and archive.

use std::path::Path;

use codev_core::{ChangeId, Layout};
use codev_engine::ports::{FileSystem, FixedClock, FixedEnv, MemoryFileSystem};

const PROPOSAL: &str = include_str!("../../../assets/schemas/spec-driven/templates/fr/proposal.md");
const SPEC: &str = include_str!("../../../assets/schemas/spec-driven/templates/fr/spec.md");
const DESIGN: &str = include_str!("../../../assets/schemas/spec-driven/templates/fr/design.md");
const TASKS: &str = include_str!("../../../assets/schemas/spec-driven/templates/fr/tasks.md");

/// `template` without its guidance comments, each `<…>` placeholder
/// replaced by `filler`.
fn fill(template: &str, filler: &str) -> String {
    let mut out = String::new();
    let mut rest = template;
    while let Some(start) = rest.find("<!--") {
        out.push_str(&rest[..start]);
        let end = rest[start..]
            .find("-->")
            .map_or(rest.len(), |e| start + e + 3);
        rest = &rest[end..];
    }
    out.push_str(rest);
    let mut filled = String::new();
    let mut rest = out.as_str();
    while let Some(start) = rest.find('<') {
        filled.push_str(&rest[..start]);
        let end = rest[start..]
            .find('>')
            .map_or(rest.len(), |e| start + e + 1);
        filled.push_str(filler);
        rest = &rest[end..];
    }
    filled.push_str(rest);
    filled
}

fn project() -> MemoryFileSystem {
    let change = "/p/_codev/changes/connexion";
    let proposal = fill(PROPOSAL, "Connexion")
        .replace(
            "### Nouvelles capacités\n",
            "### Nouvelles capacités\n\n- auth\n",
        )
        .replace(
            "## Note de version\n",
            "## Note de version\n\n- Ajouté : la connexion.\n",
        )
        .replace("## Changements\n", "## Changements\n\n- La connexion.\n");
    // A new capability: the agent keeps the template's `## Objet`, which
    // the template holds in a comment for that case.
    let spec = format!(
        "## Objet\n\nConnecter un utilisateur.\n\n{}",
        fill(SPEC, "la connexion")
    );
    let tasks = fill(TASKS, "Écrire la connexion").replace("- [ ]", "- [x]");
    MemoryFileSystem::new()
        .with_file(
            "/p/_codev/config.yaml",
            "schema: spec-driven\nlanguage: fr\nroles:\n  skeptic: never\n  newcomer: never\n",
        )
        .with_file(
            format!("{change}/change.yaml"),
            "schema: spec-driven\ncreated: 2026-10-23\n",
        )
        .with_file(format!("{change}/proposal.md"), proposal)
        .with_file(format!("{change}/specs/auth/spec.md"), spec)
        .with_file(format!("{change}/design.md"), fill(DESIGN, "Connexion"))
        .with_file(format!("{change}/tasks.md"), tasks)
}

fn env() -> FixedEnv {
    let mut env = FixedEnv::at("/p");
    env.vars.insert("HOME".into(), "/home".into());
    env
}

#[test]
fn a_french_plan_from_the_templates_validates_and_archives_into_a_french_spec() {
    let fs = project();
    let layout = Layout::new("/p");
    let config = codev_engine::config::resolve(&fs, &env(), &layout).unwrap();
    let id = ChangeId::parse("connexion").unwrap();

    let report = codev_engine::validate::validate_change(&fs, &layout, &config, &id).unwrap();
    let findings: Vec<String> = report
        .findings
        .iter()
        .map(|f| format!("{}: {}", f.finding.code, f.finding.message))
        .collect();
    assert!(findings.is_empty(), "{findings:#?}");

    let clock = FixedClock("2026-10-23".into());
    let plan = codev_engine::archive::plan_archive(&fs, &layout, &config, &clock, &id).unwrap();
    codev_engine::apply::execute(&plan.plan, &fs).unwrap();
    let spec = fs
        .read_to_string(Path::new("/p/_codev/specs/auth/spec.md"))
        .unwrap();
    assert!(spec.contains("## Exigences"), "{spec}");
    assert!(spec.contains("### Exigence : la connexion"), "{spec}");
    assert!(spec.contains("#### Scénario : la connexion"), "{spec}");
    assert!(spec.contains("DOIT"), "{spec}");
    assert!(!spec.contains("Requirement"), "{spec}");
}
