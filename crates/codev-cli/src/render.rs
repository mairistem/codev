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
    ChangelogOutcome, ChangesOutcome, DecisionAcceptedOutcome, DecisionCreatedOutcome,
    DecisionDeviatedOutcome, DecisionListOutcome, DecisionPromotedOutcome, DecisionSealedOutcome,
    DecisionShowOutcome, DecisionSupersededOutcome, DoctorOutcome, NewChangeOutcome,
    SchemasOutcome, SetupOutcome, SourcesListOutcome, SourcesShowOutcome, SourcesUpdateOutcome,
    SpecsOutcome,
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
    for pin in &outcome.pins {
        if let codev_engine::sources::PinChange::Added { url, git_ref, to } = pin {
            let short = &to[..to.len().min(12)];
            let _ = writeln!(out, "  Inherits    {url} @{git_ref} → pinned {short}");
        }
    }
    let _ = writeln!(out);

    if !outcome.integration.is_empty() {
        let _ = writeln!(out, "  Repository integration");
        for step in &outcome.integration {
            integration_line(&mut out, step);
        }
        let _ = writeln!(out);
    }

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
    if initialized && let Some(missing) = outcome.doctor_missing.filter(|n| *n > 0) {
        let _ = writeln!(out);
        let _ = writeln!(
            out,
            "Run `codev doctor` to see the {missing} setup check(s) still to fix."
        );
    }
    out
}

/// One line per integration step: what happened to which file, and why
/// when it was skipped. The GitLab job follows its line, ready to paste.
fn integration_line(out: &mut String, step: &codev_engine::integration::IntegrationStep) {
    use codev_engine::integration::{Step, StepAction};
    let label = match (&step.path, step.step) {
        (Some(path), _) => path.display().to_string(),
        (None, Step::ClaudeMd) => "CLAUDE.md".to_string(),
        (None, Step::Ci) => "CI".to_string(),
        (None, Step::PrTemplate) => "pull request template".to_string(),
    };
    let (mark, verb) = match step.action {
        StepAction::Created => ("+", "created"),
        StepAction::Appended => ("+", "codev section appended"),
        StepAction::Refreshed => ("~", "codev section refreshed"),
        StepAction::Unchanged => ("=", "up to date"),
        StepAction::Skipped => ("-", "skipped"),
    };
    let verb = match (step.step, step.action) {
        (Step::Ci, StepAction::Refreshed) => "refreshed",
        _ => verb,
    };
    match &step.reason {
        Some(reason) => {
            let _ = writeln!(out, "    {mark} {label:<34} {verb}: {reason}");
        }
        None => {
            let _ = writeln!(out, "    {mark} {label:<34} {verb}");
        }
    }
    if let Some(snippet) = &step.snippet {
        let _ = writeln!(out);
        for line in snippet.lines() {
            let _ = writeln!(out, "        {line}");
        }
        let _ = writeln!(out);
    }
}

/// Human rendering of `codev doctor`: the checks grouped, the fix under
/// each one that is not ok, and the count on the last line.
/// `codev self-update`, for a human.
pub fn self_update(outcome: &codev_engine::update::Outcome) -> String {
    use codev_engine::update::Action;
    let latest = outcome.latest.as_deref().unwrap_or("unknown");
    let mut out = String::new();
    match outcome.action {
        Action::Updated => {
            let _ = writeln!(
                out,
                "codev {} → {latest}: installed {}",
                outcome.current,
                outcome.path.display()
            );
            if let Some(project) = &outcome.project_updated {
                let _ = writeln!(out, "Ran `codev update` in {}", project.display());
            }
            if let Some(url) = &outcome.url {
                let _ = writeln!(out, "Release notes: {url}");
            }
        }
        Action::UpToDate => {
            let _ = writeln!(out, "codev {} is up to date", outcome.current);
        }
        Action::Available => {
            let _ = writeln!(
                out,
                "codev {latest} is available (you have {})",
                outcome.current
            );
            if let Some(url) = &outcome.url {
                let _ = writeln!(out, "Release notes: {url}");
            }
            let _ = writeln!(
                out,
                "Installed by {} at {}",
                outcome.method.label(),
                outcome.path.display()
            );
            match outcome.method.upgrade_hint() {
                Some(hint) => {
                    let _ = writeln!(out, "To upgrade: {hint}");
                }
                None => {
                    let _ = writeln!(out, "To upgrade: codev self-update");
                }
            }
        }
        Action::Snoozed => {
            let _ = writeln!(
                out,
                "codev {latest} snoozed for 7 days — `codev self-update` installs it any time"
            );
        }
        Action::NothingToSnooze => {
            let _ = writeln!(
                out,
                "No update to snooze: codev {} is the latest known release",
                outcome.current
            );
        }
        Action::ManagedElsewhere => {
            let _ = writeln!(
                out,
                "codev at {} was installed by {}; codev self-update leaves it alone.",
                outcome.path.display(),
                outcome.method.label()
            );
            if let Some(hint) = outcome.method.upgrade_hint() {
                let _ = writeln!(out, "To upgrade: {hint}");
            }
        }
        Action::Cancelled => {
            let _ = writeln!(out, "Update cancelled; nothing changed");
        }
    }
    out
}

pub fn doctor(outcome: &DoctorOutcome) -> String {
    use codev_core::doctor::{CheckGroup, CheckStatus};
    let mut out = String::new();
    let _ = writeln!(out, "codev doctor — {}", outcome.root.display());
    for group in [
        CheckGroup::Configuration,
        CheckGroup::Planning,
        CheckGroup::Repository,
    ] {
        let checks: Vec<_> = outcome.checks.iter().filter(|c| c.group == group).collect();
        if checks.is_empty() {
            continue;
        }
        let _ = writeln!(out);
        let _ = writeln!(out, "{}", group.label());
        for check in checks {
            let mark = match check.status {
                CheckStatus::Ok => "✓",
                CheckStatus::Missing => "✗",
                CheckStatus::Warning => "!",
            };
            let _ = writeln!(out, "  {mark} {:<16} {}", check.id, check.message);
            if let Some(fix) = &check.fix {
                let _ = writeln!(out, "      fix: {fix}");
            }
        }
    }
    let (total, missing, warnings) = codev_core::doctor::summary(&outcome.checks);
    let _ = writeln!(out);
    if warnings > 0 {
        let _ = writeln!(
            out,
            "{total} check(s), {missing} to fix, {warnings} warning(s)"
        );
    } else {
        let _ = writeln!(out, "{total} check(s), {missing} to fix");
    }
    out
}

/// Human rendering of `codev changelog --write`: what happened to the file.
/// Without `--write`, the changelog text itself is the output.
pub fn changelog_written(outcome: &ChangelogOutcome) -> String {
    let Some((path, action)) = &outcome.written else {
        return outcome.rendered.clone();
    };
    let range = match (&outcome.since, &outcome.until) {
        (Some(since), Some(until)) => format!("{since}..{until}"),
        (Some(since), None) => format!("since {since}"),
        (None, Some(until)) => format!("up to {until}"),
        (None, None) => "all archived changes".to_string(),
    };
    format!(
        "{} {} — {} item(s), {range}\n",
        path.display(),
        action.as_str(),
        outcome.entries.len()
    )
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
    let _ = writeln!(
        out,
        "Language: {} — prose only; template keywords stay in English",
        instructions.language
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
            ItemKind::Config => "config",
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
    match &outcome.body_sha256 {
        Some(hash) => {
            let _ = writeln!(out, "  Hash: {hash}");
            let _ = writeln!(
                out,
                "\nThe decision is accepted and sealed: its body can no longer be\n\
                 edited without `codev decision seal {} --force`.",
                outcome.decision.id
            );
        }
        None => {
            let _ = writeln!(
                out,
                "\nOpen the file to write the Context, Decision and Consequences sections,\n\
                 {}.",
                accept_next_step(&outcome.decision.id)
            );
        }
    }
    out
}

/// Human rendering of `codev decision accept`.
pub fn decision_accepted(outcome: &DecisionAcceptedOutcome) -> String {
    let mut out = String::new();
    let _ = writeln!(
        out,
        "✓ Accepted {} — sealed ({})",
        outcome.decision.id, outcome.body_sha256
    );
    let _ = writeln!(out, "  File: {}", outcome.path.display());
    for predecessor in &outcome.superseded {
        let _ = writeln!(
            out,
            "  Superseded: {} (status: superseded)",
            predecessor.qualified_id
        );
    }
    out
}

/// The next step shared by every command that creates a `proposed`
/// decision — the same wording as `decision new`.
fn accept_next_step(id: &str) -> String {
    format!("then run `codev decision accept {id}` to accept and seal it")
}

/// Human rendering of `codev decision promote`.
pub fn decision_promoted(outcome: &DecisionPromotedOutcome) -> String {
    let mut out = String::new();
    let _ = writeln!(
        out,
        "✓ Decision `{}` promoted to proposed ADR {}",
        outcome.decision.title, outcome.decision.id
    );
    let _ = writeln!(out, "  File:    {}", outcome.path.display());
    let _ = writeln!(out, "  Source:  {}", outcome.design_path.display());
    let _ = writeln!(
        out,
        "\nReview the ADR promoted from change `{}`: split its body into Context,\n\
         Decision, Consequences and Alternatives considered,\n\
         {}.",
        outcome.source_change,
        accept_next_step(&outcome.decision.id)
    );
    out
}

/// Human rendering of `codev decision deviate`.
pub fn decision_deviated(outcome: &DecisionDeviatedOutcome) -> String {
    let mut out = String::new();
    let _ = writeln!(
        out,
        "✓ Local deviation from `{}` created: {} {} (proposed)",
        outcome.target_qualified_id, outcome.decision.id, outcome.decision.title
    );
    let _ = writeln!(out, "  File: {}", outcome.path.display());
    let _ = writeln!(
        out,
        "\nOpen the file to write the Context, Decision and Consequences sections,\n\
         {}.\n\
         `{}` stays in effect until then.",
        accept_next_step(&outcome.decision.id),
        outcome.target_qualified_id
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
        "✓ Decision `{} {}` created (proposed), to supersede `{}`",
        outcome.new_decision.id, outcome.new_decision.title, outcome.old_qualified_id
    );
    let _ = writeln!(out, "  New ADR:     {}", outcome.new_path.display());
    let _ = writeln!(
        out,
        "  Old ADR:     {} (unchanged, still in effect)",
        outcome.old_path.display()
    );
    let _ = writeln!(
        out,
        "\nOpen the new file to write the Context, Decision and Consequences sections,\n\
         {}.\n\
         Accepting it marks `{}` superseded; until then it stays in effect.",
        accept_next_step(&outcome.new_decision.id),
        outcome.old_qualified_id
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
            language: "en".into(),
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
            language: "en".into(),
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
            integration: Vec::new(),
            pins: Vec::new(),
            doctor_missing: None,
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

    // ─────────────── decisions created proposed ───────────────

    fn proposed_summary(id: &str, title: &str) -> crate::commands::DecisionSummary {
        crate::commands::DecisionSummary {
            id: id.into(),
            qualified_id: format!("project/{id}"),
            title: title.into(),
            status: "proposed".into(),
            date: "2026-09-30".into(),
            tags: Vec::new(),
            supersedes: Vec::new(),
            deviates_from: Vec::new(),
            path: std::path::PathBuf::from(format!("_codev/decisions/{id}-x.md")),
            origin: "project".into(),
            in_effect: false,
            superseded_by: None,
            deviated_by: None,
        }
    }

    #[test]
    fn superseded_output_names_the_accept_step() {
        let out = decision_superseded(&crate::commands::DecisionSupersededOutcome {
            root: "/p".into(),
            new_decision: proposed_summary("0007", "New choice"),
            new_path: "/p/_codev/decisions/0007-new-choice.md".into(),
            old_id: "0003".into(),
            old_qualified_id: "project/0003".into(),
            old_path: "/p/_codev/decisions/0003-old.md".into(),
        });
        assert!(out.contains("(proposed)"), "{out}");
        assert!(out.contains("still in effect"), "{out}");
        assert!(out.contains("`codev decision accept 0007`"), "{out}");
    }

    #[test]
    fn deviated_output_names_the_accept_step() {
        let out = decision_deviated(&crate::commands::DecisionDeviatedOutcome {
            root: "/p".into(),
            decision: proposed_summary("0001", "Our alternative"),
            path: "/p/_codev/decisions/0001-our-alternative.md".into(),
            target_qualified_id: "path:~/shared/0100".into(),
        });
        assert!(out.contains("(proposed)"), "{out}");
        assert!(!out.contains("Hash"), "nothing is sealed: {out}");
        assert!(out.contains("`codev decision accept 0001`"), "{out}");
    }

    #[test]
    fn promoted_output_names_the_accept_step() {
        let out = decision_promoted(&crate::commands::DecisionPromotedOutcome {
            root: "/p".into(),
            decision: proposed_summary("0001", "Use JWT"),
            path: "/p/_codev/decisions/0001-use-jwt.md".into(),
            source_change: "add-auth".into(),
            design_path: "/p/_codev/changes/add-auth/design.md".into(),
        });
        assert!(out.contains("proposed ADR 0001"), "{out}");
        assert!(!out.contains("Hash"), "nothing is sealed: {out}");
        assert!(out.contains("Review the ADR"), "{out}");
        assert!(out.contains("`codev decision accept 0001`"), "{out}");
    }

    #[test]
    fn setup_lists_integration_steps_and_reasons() {
        use codev_engine::integration::{IntegrationStep, Step, StepAction};
        let mut outcome = setup_outcome(&["codev-propose"], false);
        outcome.integration = vec![
            IntegrationStep {
                step: Step::ClaudeMd,
                action: StepAction::Created,
                path: Some("CLAUDE.md".into()),
                reason: None,
                snippet: None,
            },
            IntegrationStep {
                snippet: Some("codev:\n  script:\n    - codev validate --strict\n".into()),
                ..IntegrationStep::skipped(
                    Step::Ci,
                    Some(".gitlab-ci.yml".into()),
                    "codev never edits .gitlab-ci.yml",
                )
            },
            IntegrationStep::skipped(Step::PrTemplate, None, "not a GitHub repository"),
        ];
        let rendered = setup(&outcome, true);
        assert!(rendered.contains("Repository integration"));
        assert!(rendered.contains("+ CLAUDE.md"));
        assert!(rendered.contains("created"));
        assert!(rendered.contains("- .gitlab-ci.yml"));
        assert!(rendered.contains("skipped: codev never edits .gitlab-ci.yml"));
        assert!(rendered.contains("        codev:\n"));
        assert!(rendered.contains("- pull request template"));
        assert!(rendered.contains("skipped: not a GitHub repository"));
    }

    #[test]
    fn setup_ends_with_the_doctor_hint_when_something_is_missing() {
        let mut outcome = setup_outcome(&["codev-propose"], false);
        outcome.doctor_missing = Some(2);
        let rendered = setup(&outcome, true);
        assert!(
            rendered
                .trim_end()
                .ends_with("Run `codev doctor` to see the 2 setup check(s) still to fix."),
            "{rendered}"
        );
        outcome.doctor_missing = Some(0);
        assert!(!setup(&outcome, true).contains("codev doctor"));
        outcome.doctor_missing = Some(2);
        // `codev update` keeps its short summary.
        assert!(!setup(&outcome, false).contains("codev doctor"));
    }

    #[test]
    fn doctor_output_groups_checks_and_ends_with_the_count() {
        use codev_core::doctor::{DoctorFacts, evaluate};
        let facts = DoctorFacts {
            language_set: true,
            claude_md_section: true,
            ..DoctorFacts::default()
        };
        let outcome = DoctorOutcome {
            root: "/home/you/acme-app".into(),
            checks: evaluate(&facts),
        };
        let rendered = doctor(&outcome);
        let configuration = rendered.find("\nConfiguration\n").unwrap();
        let planning = rendered.find("\nPlanning\n").unwrap();
        let repository = rendered.find("\nRepository\n").unwrap();
        assert!(configuration < planning && planning < repository);
        assert!(rendered.contains("  ✗ config.rules"));
        assert!(rendered.contains("      fix: /codev-configure"));
        assert!(rendered.contains("  ✓ config.language"));
        assert!(rendered.contains("  ✓ config.keys"));
        assert!(rendered.contains("  ! decisions"));
        assert!(rendered.contains("  ! changelog"));
        assert!(rendered.contains("      fix: codev changelog --write"));
        assert!(
            rendered
                .trim_end()
                .ends_with("12 check(s), 3 to fix, 2 warning(s)"),
            "{rendered}"
        );
    }
}
