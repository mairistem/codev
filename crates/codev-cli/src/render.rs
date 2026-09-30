//! Output meant for a human in a terminal.
//!
//! Kept separate from the JSON contract: that one is a stable API, this one
//! is made to be read and freely improved.

use std::fmt::Write as _;

use codev_core::parser::ast::Severity;
use codev_core::{ArtifactState, ChangeStatus};
use codev_engine::Warning;
use codev_engine::archive::ArchiveOutcome;
use codev_engine::instructions::Instructions;
use codev_engine::sync::SyncOutcome;
use codev_engine::validate::{ItemKind, ValidateReport};

use crate::commands::{
    ChangesOutcome, DecisionCreatedOutcome, DecisionDeviatedOutcome, DecisionListOutcome,
    DecisionPromotedOutcome, DecisionSealedOutcome, DecisionShowOutcome, DecisionSupersededOutcome,
    NewChangeOutcome, SchemasOutcome, SetupOutcome, SourcesListOutcome, SourcesShowOutcome,
    SourcesUpdateOutcome, SpecsOutcome,
};

/// Warnings go to stderr, never to stdout: a `codev list` redirected to a
/// file must not get polluted by them.
pub fn warnings(warnings: &[Warning]) {
    for warning in warnings {
        eprintln!("warning: {}", warning.message);
    }
}

pub fn setup(outcome: &SetupOutcome, initialized: bool) -> String {
    let mut out = String::new();
    let verb = if initialized {
        "initialized"
    } else {
        "updated"
    };
    let _ = writeln!(out, "codev {verb} in {}", outcome.root.display());
    let _ = writeln!(out);

    let _ = writeln!(
        out,
        "  Structure   _codev/ — specs, decisions, changes, schemas"
    );
    if outcome.skills.is_empty() {
        let _ = writeln!(out, "  Skills      none (no workflow selected)");
    } else {
        let _ = writeln!(
            out,
            "  Skills      {} → .claude/skills/",
            outcome.skills.join(", ")
        );
    }
    let _ = writeln!(out);

    if outcome.changed_anything() {
        let _ = writeln!(
            out,
            "  {} file(s) created, {} updated",
            outcome.created.len(),
            outcome.updated.len()
        );
    } else {
        let _ = writeln!(out, "  Nothing to do, everything is already in place.");
    }

    if !outcome.preserved.is_empty() {
        let _ = writeln!(out);
        let _ = writeln!(
            out,
            "  Left in place because edited by hand — rerun with --force to overwrite:"
        );
        for path in &outcome.preserved {
            let _ = writeln!(out, "    {}", path.display());
        }
    }

    if initialized && !outcome.skills.is_empty() {
        let has_configure = outcome.skills.iter().any(|s| s.ends_with("configure"));
        let _ = writeln!(out);
        if outcome.config_thin && has_configure {
            // Call to action for the empty config — it is the recommended
            // entry point right after an init on a new project.
            let _ = writeln!(
                out,
                "→ Recommended next step: in Claude Code, run /codev-configure."
            );
            let _ = writeln!(
                out,
                "  Claude will analyze the project and enrich _codev/config.yaml"
            );
            let _ = writeln!(out, "  (context, per-artifact rules) — ~30 seconds.");
            let _ = writeln!(out);
            let _ = writeln!(
                out,
                "Or skip this step and run /codev-propose <an-idea> directly."
            );
        } else {
            let first = outcome
                .skills
                .iter()
                .find(|s| s.ends_with("propose"))
                .or_else(|| outcome.skills.first());
            let _ = writeln!(
                out,
                "Restart Claude Code so it discovers the skills, then run /{}.",
                first.map(String::as_str).unwrap_or("codev-propose")
            );
        }
    }
    out
}

pub fn new_change(outcome: &NewChangeOutcome) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "Change `{}` created", outcome.change);
    let _ = writeln!(out, "  Location  {}", outcome.change_root.display());
    let _ = writeln!(out, "  Schema    {}", outcome.schema_name);
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "Next step: `codev instructions --change {}` shows the artifact to write.",
        outcome.change
    );
    out
}

fn symbol(state: ArtifactState) -> &'static str {
    match state {
        ArtifactState::Done => "[x]",
        ArtifactState::Ready => "[ ]",
        ArtifactState::Blocked => "[-]",
        ArtifactState::Skipped => "[~]",
    }
}

pub fn status(status: &ChangeStatus) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "Change: {}", status.change);
    let _ = writeln!(out, "Schema: {}", status.schema_name);
    let _ = writeln!(out);

    for artifact in &status.artifacts {
        let mut line = format!("  {} {}", symbol(artifact.state), artifact.id);
        match artifact.state {
            ArtifactState::Blocked if !artifact.missing_deps.is_empty() => {
                let _ = write!(line, " (waiting on: {})", artifact.missing_deps.join(", "));
            }
            ArtifactState::Skipped => {
                let _ = write!(line, " (disabled by skip_specs)");
            }
            _ => {}
        }
        let _ = writeln!(out, "{line}");
    }

    let done_count = status
        .artifacts
        .iter()
        .filter(|a| a.state.satisfies_dependency())
        .count();
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "Planning: {done_count}/{} artifacts",
        status.artifacts.len()
    );

    match status
        .artifacts
        .iter()
        .find(|a| a.state == ArtifactState::Ready)
    {
        Some(next) => {
            let _ = writeln!(out, "Next:     {}", next.id);
        }
        None if status.planning_complete => {
            let _ = writeln!(out, "Planning is complete.");
        }
        None => {
            let _ = writeln!(
                out,
                "Nothing ready: a required artifact is blocked, see above."
            );
        }
    }
    out
}

pub fn instructions(instructions: &Instructions) -> String {
    let mut out = String::new();
    let _ = writeln!(
        out,
        "Artifact: {} (change `{}`, schema {})",
        instructions.artifact_id, instructions.change, instructions.schema_name
    );
    let _ = writeln!(
        out,
        "Write to: {}",
        instructions.resolved_output_path.display()
    );

    if instructions.skipped {
        let _ = writeln!(out);
        let _ = writeln!(
            out,
            "This artifact is disabled by `skip_specs`: do not create it."
        );
        return out;
    }

    if !instructions.dependencies.is_empty() {
        let _ = writeln!(out);
        let _ = writeln!(out, "Read before writing:");
        for dep in &instructions.dependencies {
            let _ = writeln!(
                out,
                "  {} {} — {}",
                if dep.done { "[x]" } else { "[ ]" },
                dep.id,
                dep.path.display()
            );
        }
    }

    for (title, blocks) in [
        ("Project context", &instructions.context),
        ("Rules for this artifact", &instructions.rules),
    ] {
        if blocks.is_empty() {
            continue;
        }
        let _ = writeln!(out);
        let _ = writeln!(out, "{title}:");
        for block in blocks {
            let _ = writeln!(out, "  ({}) {}", block.origin, block.text);
        }
    }

    // Decisions in effect — one line per entry. The section is absent when
    // the list is empty (the field exists on the JSON side, but a human
    // display has no reason to carry an empty heading).
    if !instructions.decisions.is_empty() {
        let _ = writeln!(out);
        let _ = writeln!(out, "Decisions in effect:");
        for decision in &instructions.decisions {
            let _ = writeln!(out, "  - {} {}", decision.id, decision.title);
        }
    }

    if let Some(instruction) = &instructions.instruction {
        let _ = writeln!(out);
        let _ = writeln!(out, "Instruction:");
        let _ = writeln!(out);
        let _ = writeln!(out, "{}", instruction.trim_end());
    }

    if let Some(template) = &instructions.template {
        let _ = writeln!(out);
        let _ = writeln!(out, "Template:");
        let _ = writeln!(out);
        let _ = writeln!(out, "{}", template.trim_end());
    }

    if !instructions.unlocks.is_empty() {
        let _ = writeln!(out);
        let _ = writeln!(out, "Unlocks: {}", instructions.unlocks.join(", "));
    }
    out
}

/// Human output of the validation report.
///
/// Three principles: one line per finding (no fragile ASCII table), each
/// line prefixed with `path:line:` to fit within 80 columns, and a final
/// summary that distinguishes "nothing found" from "nothing to validate".
pub fn validate(report: &ValidateReport) -> String {
    let mut out = String::new();

    if report.items.is_empty() {
        let _ = writeln!(
            out,
            "Nothing to validate — no active change and no main spec."
        );
        return out;
    }

    let mut total_findings = 0;
    let mut total_errors = 0;

    for item in &report.items {
        let label = match item.kind {
            ItemKind::Change => "change",
            ItemKind::Spec => "spec",
            ItemKind::Decisions => "decisions",
        };
        let _ = writeln!(out, "\n{} {} — {}", label, item.name, item.path.display(),);
        if item.findings.is_empty() {
            let _ = writeln!(out, "  ✓ no issues");
            continue;
        }
        for finding in &item.findings {
            total_findings += 1;
            if finding.finding.severity == Severity::Error {
                total_errors += 1;
            }
            let severity_marker = match finding.finding.severity {
                Severity::Error => "error  ",
                Severity::Warning => "warning",
                Severity::Info => "info   ",
            };
            let _ = writeln!(
                out,
                "  {} {}:{}: {} — {}",
                severity_marker,
                finding.path.display(),
                finding.finding.line,
                finding.finding.code,
                finding.finding.message,
            );
        }
    }

    let _ = writeln!(out);
    if total_findings == 0 {
        let _ = writeln!(out, "{} item(s) validated, no issues.", report.items.len());
    } else {
        let _ = writeln!(
            out,
            "{} item(s) validated, {} finding(s) including {} error(s).",
            report.items.len(),
            total_findings,
            total_errors
        );
    }
    out
}

/// Human rendering of a `codev sync` result.
///
/// Groups by category — created, updated, unchanged — rather than a single
/// mixed line. An explicit "nothing to do" when everything is unchanged.
pub fn sync(outcome: &SyncOutcome) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "Synced change `{}`", outcome.change);
    render_categories(
        &mut out,
        &outcome.created,
        &outcome.updated,
        &outcome.unchanged,
        &outcome.deleted,
    );
    if !outcome.changed_anything() {
        let _ = writeln!(
            out,
            "\nNothing to do — the main specs are already up to date."
        );
    }
    out
}

/// Human rendering of a `codev archive` result.
pub fn archive(outcome: &ArchiveOutcome) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "Archived change `{}`", outcome.change);
    render_categories(
        &mut out,
        &outcome.created,
        &outcome.updated,
        &outcome.unchanged,
        &outcome.deleted,
    );
    let _ = writeln!(out, "\nMoved to: {}", outcome.moved_to.display());
    out
}

fn render_categories(
    out: &mut String,
    created: &[std::path::PathBuf],
    updated: &[std::path::PathBuf],
    unchanged: &[std::path::PathBuf],
    deleted: &[std::path::PathBuf],
) {
    for (label, paths) in [
        ("Created", created),
        ("Updated", updated),
        ("Unchanged", unchanged),
        ("Deleted", deleted),
    ] {
        if paths.is_empty() {
            continue;
        }
        let _ = writeln!(out);
        let _ = writeln!(out, "{label}:");
        for p in paths {
            let _ = writeln!(out, "  {}", p.display());
        }
    }
}

/// Human rendering of `codev decision list`.
pub fn decision_list(outcome: &DecisionListOutcome) -> String {
    if outcome.decisions.is_empty() {
        return "No decisions. Run `codev decision new <title>` to create one.\n".to_string();
    }
    let mut out = String::from("Decisions:\n");
    for d in &outcome.decisions {
        let marker = if d.in_effect { "•" } else { "–" };
        let _ = writeln!(out, "  {marker} {} {}  [{}]", d.id, d.title, d.status);
        if let Some(by) = &d.superseded_by {
            let _ = writeln!(out, "      superseded by {by}");
        }
    }
    out
}

/// Human rendering of `codev decision show`.
pub fn decision_show(outcome: &DecisionShowOutcome) -> String {
    let d = &outcome.decision;
    let mut out = String::new();
    let _ = writeln!(out, "ID:     {} ({})", d.id, d.qualified_id);
    let _ = writeln!(out, "Title:  {}", d.title);
    let _ = writeln!(out, "Status: {}", d.status);
    let _ = writeln!(out, "Date:   {}", d.date);
    if !d.tags.is_empty() {
        let _ = writeln!(out, "Tags:   {}", d.tags.join(", "));
    }
    if !d.supersedes.is_empty() {
        let _ = writeln!(out, "Supersedes:    {}", d.supersedes.join(", "));
    }
    if let Some(by) = &d.superseded_by {
        let _ = writeln!(out, "Superseded by: {by}");
    }
    let _ = writeln!(out, "File:   {}", d.path.display());
    let _ = writeln!(out);
    out.push_str(&outcome.content);
    out
}

/// Human rendering of `codev decision new`.
pub fn decision_created(outcome: &DecisionCreatedOutcome) -> String {
    let mut out = String::new();
    let _ = writeln!(
        out,
        "✓ Decision `{} {}` created",
        outcome.decision.id, outcome.decision.title
    );
    let _ = writeln!(out, "  File: {}", outcome.path.display());
    let _ = writeln!(
        out,
        "\nOpen the file to write the Context, Decision and Consequences sections."
    );
    out
}

/// Human rendering of `codev decision promote`.
pub fn decision_promoted(outcome: &DecisionPromotedOutcome) -> String {
    let mut out = String::new();
    let _ = writeln!(
        out,
        "✓ Decision `{}` promoted to ADR {}: {}",
        outcome.decision.title, outcome.decision.id, outcome.decision.title
    );
    let _ = writeln!(out, "  File:    {}", outcome.path.display());
    let _ = writeln!(out, "  Hash:    {}", outcome.body_sha256);
    let _ = writeln!(out, "  Source:  {}", outcome.design_path.display());
    let _ = writeln!(
        out,
        "\nSplit the body into Context / Decision / Consequences / Alternatives\n\
         considered before archiving change `{}`.",
        outcome.source_change
    );
    out
}

/// Human rendering of `codev decision deviate`.
pub fn decision_deviated(outcome: &DecisionDeviatedOutcome) -> String {
    let mut out = String::new();
    let _ = writeln!(
        out,
        "✓ Local deviation from `{}` created: {} {}",
        outcome.target_qualified_id, outcome.decision.id, outcome.decision.title
    );
    let _ = writeln!(out, "  File: {}", outcome.path.display());
    let _ = writeln!(out, "  Hash: {}", outcome.body_sha256);
    let _ = writeln!(
        out,
        "\nThe ADR is sealed. Open it to write the Context and Decision sections."
    );
    out
}

/// Human rendering of `codev decision seal`.
pub fn decision_sealed(outcome: &DecisionSealedOutcome) -> String {
    let mut out = String::new();
    if outcome.was_noop {
        let _ = writeln!(
            out,
            "Already up to date: {} ({})",
            outcome.id, outcome.body_sha256
        );
    } else if outcome.was_forced {
        let _ = writeln!(out, "✓ Resealed: {} ({})", outcome.id, outcome.body_sha256);
    } else {
        let _ = writeln!(out, "✓ Sealed: {} ({})", outcome.id, outcome.body_sha256);
    }
    out
}

/// Human rendering of `codev decision supersede`.
pub fn decision_superseded(outcome: &DecisionSupersededOutcome) -> String {
    let mut out = String::new();
    let _ = writeln!(
        out,
        "✓ Decision `{}` superseded by `{} {}`",
        outcome.old_qualified_id, outcome.new_decision.id, outcome.new_decision.title
    );
    let _ = writeln!(out, "  New ADR:     {}", outcome.new_path.display());
    let _ = writeln!(
        out,
        "  Old ADR:     {} (status: superseded)",
        outcome.old_path.display()
    );
    let _ = writeln!(
        out,
        "\nOpen the new file to write the Decision and what changes."
    );
    out
}

/// Human rendering of `codev sources list`.
pub fn sources_list(outcome: &SourcesListOutcome) -> String {
    if outcome.sources.is_empty() {
        return "No inherited source declared in _codev/config.yaml.\n".to_string();
    }
    let mut out = String::from("Inherited sources:\n");
    for s in &outcome.sources {
        let _ = writeln!(
            out,
            "  {} {}  [{}]",
            s.kind.as_str(),
            s.address,
            s.state.as_str()
        );
        if let Some(sha) = &s.sha {
            let _ = writeln!(out, "      commit: {sha}");
        }
        if let Some(path) = &s.resolved_path {
            let _ = writeln!(out, "      path:   {}", path.display());
        }
    }
    out
}

/// Human rendering of `codev sources update`.
pub fn sources_update(outcome: &SourcesUpdateOutcome) -> String {
    let mut out = String::new();
    if outcome.diff.is_empty() {
        let _ = writeln!(out, "No `git:` source to update.");
        return out;
    }
    let _ = writeln!(out, "Changes:");
    use codev_engine::sources::PinChange::*;
    for change in &outcome.diff {
        match change {
            Added { url, git_ref, to } => {
                let _ = writeln!(out, "  + {url} @{git_ref}  → {to}");
            }
            Moved {
                url,
                git_ref,
                from,
                to,
            } => {
                let _ = writeln!(out, "  ~ {url} @{git_ref}  {from} → {to}");
            }
            Unchanged { url, git_ref, sha } => {
                let _ = writeln!(out, "  = {url} @{git_ref}  ({sha})");
            }
        }
    }
    let _ = writeln!(out);
    if outcome.lock_written {
        let _ = writeln!(out, "codev.lock updated.");
    } else {
        let _ = writeln!(out, "codev.lock unchanged.");
    }
    out
}

/// Human rendering of `codev sources show`.
pub fn sources_show(outcome: &SourcesShowOutcome) -> String {
    let mut out = String::new();
    let s = &outcome.source;
    let _ = writeln!(out, "Type:    {}", s.kind.as_str());
    let _ = writeln!(out, "Address: {}", s.address);
    if let Some(git_ref) = &s.git_ref {
        let _ = writeln!(out, "Ref:     {git_ref}");
    }
    if let Some(sha) = &s.sha {
        let _ = writeln!(out, "SHA:     {sha}");
    }
    if let Some(subpath) = &s.subpath {
        let _ = writeln!(out, "Subpath: {subpath}");
    }
    let _ = writeln!(out, "State:   {}", s.state.as_str());
    if let Some(path) = &s.resolved_path {
        let _ = writeln!(out, "Path:    {}", path.display());
    }
    if !outcome.files_exposed.is_empty() {
        let _ = writeln!(out);
        let _ = writeln!(
            out,
            "Exposed files ({} in total):",
            outcome.files_exposed.len()
        );
        for file in outcome.files_exposed.iter().take(20) {
            let _ = writeln!(out, "  {file}");
        }
        if outcome.files_exposed.len() > 20 {
            let _ = writeln!(out, "  … ({} more)", outcome.files_exposed.len() - 20);
        }
    }
    out
}

pub fn changes(outcome: &ChangesOutcome) -> String {
    if outcome.changes.is_empty() {
        return "No active change. Run `codev new change <name>` to create one.\n".to_string();
    }
    let mut out = String::from("Active changes:\n");
    for change in &outcome.changes {
        let _ = writeln!(out, "  {change}");
    }
    out
}

pub fn specs(outcome: &SpecsOutcome) -> String {
    if outcome.specs.is_empty() {
        return "No specs. They appear when a change is synced or archived.\n".to_string();
    }
    let mut out = String::from("Specified capabilities:\n");
    for spec in &outcome.specs {
        let _ = writeln!(out, "  {spec}");
    }
    out
}

pub fn schemas(outcome: &SchemasOutcome) -> String {
    let mut out = String::from("Available schemas:\n");
    for schema in &outcome.schemas {
        let _ = writeln!(out, "  {} ({})", schema.name, schema.origin);
        let _ = writeln!(out, "    {}", schema.flow.join(" → "));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use codev_core::{ArtifactGraph, ChangeId, status as core_status};
    use std::collections::BTreeSet;

    const SPEC_DRIVEN: &str = r#"
name: spec-driven
artifacts:
  - id: proposal
    generates: proposal.md
  - id: specs
    generates: "specs/**/*.md"
    requires: [proposal]
  - id: tasks
    generates: tasks.md
    requires: [specs]
apply:
  requires: [tasks]
  tracks: tasks.md
"#;

    fn status_of(existing: &[&str], skipped: &[&str]) -> ChangeStatus {
        let graph = ArtifactGraph::from_yaml(SPEC_DRIVEN).unwrap();
        let change = ChangeId::parse("add-auth").unwrap();
        let to_set =
            |ids: &[&str]| -> BTreeSet<String> { ids.iter().map(|s| s.to_string()).collect() };
        core_status::compute(&graph, &change, &to_set(existing), &to_set(skipped))
    }

    #[test]
    fn design_instructions_list_the_decisions_in_effect() {
        use codev_engine::instructions::{DecisionRef, Dependency, Instructions};
        use std::path::PathBuf;
        let instr = Instructions {
            change: ChangeId::parse("add-auth").unwrap(),
            schema_name: "spec-driven".into(),
            artifact_id: "design".into(),
            description: None,
            resolved_output_path: PathBuf::from("/p/_codev/changes/add-auth/design.md"),
            instruction: None,
            template: None,
            context: Vec::new(),
            rules: Vec::new(),
            dependencies: Vec::<Dependency>::new(),
            unlocks: Vec::new(),
            decisions: vec![DecisionRef {
                id: "0001".into(),
                qualified_id: "project/0001".into(),
                title: "Foundation".into(),
                status: "accepted".into(),
                tags: vec![],
                path: PathBuf::from("_codev/decisions/0001-foundation.md"),
                origin: "project".into(),
            }],
            skipped: false,
            warnings: Vec::new(),
        };
        let rendered = super::instructions(&instr);
        assert!(rendered.contains("Decisions in effect"), "{rendered}");
        assert!(rendered.contains("- 0001 Foundation"), "{rendered}");
    }

    #[test]
    fn instructions_without_decisions_have_no_empty_heading() {
        use codev_engine::instructions::{Dependency, Instructions};
        use std::path::PathBuf;
        let instr = Instructions {
            change: ChangeId::parse("add-auth").unwrap(),
            schema_name: "spec-driven".into(),
            artifact_id: "design".into(),
            description: None,
            resolved_output_path: PathBuf::from("/p/_codev/changes/add-auth/design.md"),
            instruction: None,
            template: None,
            context: Vec::new(),
            rules: Vec::new(),
            dependencies: Vec::<Dependency>::new(),
            unlocks: Vec::new(),
            decisions: Vec::new(),
            skipped: false,
            warnings: Vec::new(),
        };
        let rendered = super::instructions(&instr);
        assert!(
            !rendered.contains("Decisions in effect"),
            "no heading expected when the list is empty: {rendered}"
        );
    }

    #[test]
    fn status_shows_the_state_and_the_next_artifact() {
        let rendered = status(&status_of(&["proposal"], &[]));
        assert!(rendered.contains("[x] proposal"), "{rendered}");
        assert!(rendered.contains("[ ] specs"), "{rendered}");
        assert!(rendered.contains("[-] tasks"), "{rendered}");
        assert!(rendered.contains("(waiting on: specs)"), "{rendered}");
        assert!(rendered.contains("Next:     specs"), "{rendered}");
        assert!(rendered.contains("1/3 artifacts"), "{rendered}");
    }

    #[test]
    fn status_says_when_planning_is_complete() {
        let rendered = status(&status_of(&["proposal", "specs", "tasks"], &[]));
        assert!(rendered.contains("Planning is complete."), "{rendered}");
        assert!(!rendered.contains("Next:"), "{rendered}");
    }

    #[test]
    fn status_flags_a_disabled_artifact() {
        let rendered = status(&status_of(&["proposal"], &["specs"]));
        assert!(rendered.contains("[~] specs"), "{rendered}");
        assert!(rendered.contains("disabled by skip_specs"), "{rendered}");
    }

    #[test]
    fn an_empty_list_says_what_to_do_next() {
        let outcome = ChangesOutcome {
            root: "/p".into(),
            changes: Vec::new(),
        };
        assert!(changes(&outcome).contains("codev new change"));
    }

    fn setup_outcome(skills: &[&str], config_thin: bool) -> SetupOutcome {
        SetupOutcome {
            root: "/p".into(),
            created: Vec::new(),
            updated: Vec::new(),
            untouched: Vec::new(),
            preserved: Vec::new(),
            skills: skills.iter().map(|s| s.to_string()).collect(),
            warnings: Vec::new(),
            config_thin,
        }
    }

    #[test]
    fn setup_thin_config_with_configure_points_to_configure() {
        let outcome = setup_outcome(&["codev-propose", "codev-configure", "codev-apply"], true);
        let rendered = setup(&outcome, true);
        assert!(
            rendered.contains("/codev-configure"),
            "thin output must cite /codev-configure: {rendered}"
        );
        assert!(
            rendered.contains("~30 seconds"),
            "thin output must cite the estimated time: {rendered}"
        );
        assert!(
            rendered.contains("/codev-propose"),
            "output must keep /codev-propose as an alternative: {rendered}"
        );
    }

    #[test]
    fn setup_non_thin_config_does_not_cite_configure() {
        let outcome = setup_outcome(&["codev-propose", "codev-configure"], false);
        let rendered = setup(&outcome, true);
        assert!(
            !rendered.contains("/codev-configure"),
            "non-thin output must NOT cite /codev-configure: {rendered}"
        );
        assert!(
            rendered.contains("Restart Claude Code"),
            "non-thin output keeps the short line: {rendered}"
        );
    }

    #[test]
    fn setup_thin_without_configure_installed_falls_back_to_the_short_message() {
        // A project that explicitly removed `configure` from its workflows
        // must not see the hint — it would be broken (missing skill).
        let outcome = setup_outcome(&["codev-propose", "codev-explore"], true);
        let rendered = setup(&outcome, true);
        assert!(!rendered.contains("/codev-configure"), "{rendered}");
        assert!(rendered.contains("Restart Claude Code"), "{rendered}");
    }
}
