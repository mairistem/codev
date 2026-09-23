//! La sortie destinée à un humain dans un terminal.
//!
//! Séparée du contrat JSON : celui-ci est une API stable, celle-ci est faite
//! pour être relue et améliorée librement.

use std::fmt::Write as _;

use codev_core::parser::ast::Severity;
use codev_core::{ArtifactState, ChangeStatus};
use codev_engine::archive::ArchiveOutcome;
use codev_engine::instructions::Instructions;
use codev_engine::sync::SyncOutcome;
use codev_engine::validate::{ItemKind, ValidateReport};
use codev_engine::Warning;

use crate::commands::{
    ChangesOutcome, DecisionCreatedOutcome, DecisionDeviatedOutcome, DecisionListOutcome,
    DecisionPromotedOutcome, DecisionSealedOutcome, DecisionShowOutcome,
    DecisionSupersededOutcome, NewChangeOutcome, SchemasOutcome, SetupOutcome,
    SourcesListOutcome,
    SourcesShowOutcome, SourcesUpdateOutcome, SpecsOutcome,
};

/// Les avertissements vont sur stderr, jamais sur stdout : un `codev list`
/// redirigé dans un fichier ne doit pas s'en retrouver pollué.
pub fn warnings(warnings: &[Warning]) {
    for warning in warnings {
        eprintln!("Avertissement : {}", warning.message);
    }
}

pub fn setup(outcome: &SetupOutcome, initialise: bool) -> String {
    let mut out = String::new();
    let verbe = if initialise { "initialisé" } else { "mis à jour" };
    let _ = writeln!(out, "codev {verbe} dans {}", outcome.root.display());
    let _ = writeln!(out);

    let _ = writeln!(
        out,
        "  Structure   _codev/ — specs, decisions, changes, schemas"
    );
    if outcome.skills.is_empty() {
        let _ = writeln!(out, "  Skills      aucune (aucun workflow sélectionné)");
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
            "  {} fichier(s) créé(s), {} mis à jour",
            outcome.created.len(),
            outcome.updated.len()
        );
    } else {
        let _ = writeln!(out, "  Rien à faire, tout est déjà en place.");
    }

    if !outcome.preserved.is_empty() {
        let _ = writeln!(out);
        let _ = writeln!(
            out,
            "  Laissé(s) en place car modifié(s) à la main — relance avec --force pour écraser :"
        );
        for path in &outcome.preserved {
            let _ = writeln!(out, "    {}", path.display());
        }
    }

    if initialise && !outcome.skills.is_empty() {
        let premiere = outcome
            .skills
            .iter()
            .find(|s| s.ends_with("propose"))
            .or_else(|| outcome.skills.first());
        let _ = writeln!(out);
        let _ = writeln!(
            out,
            "Redémarre Claude Code pour qu'il découvre les skills, puis tape /{}.",
            premiere.map(String::as_str).unwrap_or("codev-propose")
        );
    }
    out
}

pub fn new_change(outcome: &NewChangeOutcome) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "Change « {} » créé", outcome.change);
    let _ = writeln!(out, "  Emplacement  {}", outcome.change_root.display());
    let _ = writeln!(out, "  Schéma       {}", outcome.schema_name);
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "Prochaine étape : `codev instructions --change {}` donne l'artefact à écrire.",
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
    let _ = writeln!(out, "Change : {}", status.change);
    let _ = writeln!(out, "Schéma : {}", status.schema_name);
    let _ = writeln!(out);

    for artifact in &status.artifacts {
        let mut ligne = format!("  {} {}", symbol(artifact.state), artifact.id);
        match artifact.state {
            ArtifactState::Blocked if !artifact.missing_deps.is_empty() => {
                let _ = write!(ligne, " (attend : {})", artifact.missing_deps.join(", "));
            }
            ArtifactState::Skipped => {
                let _ = write!(ligne, " (neutralisé par skip_specs)");
            }
            _ => {}
        }
        let _ = writeln!(out, "{ligne}");
    }

    let faits = status
        .artifacts
        .iter()
        .filter(|a| a.state.satisfies_dependency())
        .count();
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "Planification : {faits}/{} artefacts",
        status.artifacts.len()
    );

    match status
        .artifacts
        .iter()
        .find(|a| a.state == ArtifactState::Ready)
    {
        Some(prochain) => {
            let _ = writeln!(out, "Prochain      : {}", prochain.id);
        }
        None if status.planning_complete => {
            let _ = writeln!(out, "La planification est complète.");
        }
        None => {
            let _ = writeln!(
                out,
                "Rien de prêt : un artefact requis est bloqué, voir ci-dessus."
            );
        }
    }
    out
}

pub fn instructions(instructions: &Instructions) -> String {
    let mut out = String::new();
    let _ = writeln!(
        out,
        "Artefact : {} (change « {} », schéma {})",
        instructions.artifact_id, instructions.change, instructions.schema_name
    );
    let _ = writeln!(
        out,
        "Écrire dans : {}",
        instructions.resolved_output_path.display()
    );

    if instructions.skipped {
        let _ = writeln!(out);
        let _ = writeln!(
            out,
            "Cet artefact est neutralisé par `skip_specs` : ne le crée pas."
        );
        return out;
    }

    if !instructions.dependencies.is_empty() {
        let _ = writeln!(out);
        let _ = writeln!(out, "À relire avant d'écrire :");
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

    for (titre, blocs) in [
        ("Contexte projet", &instructions.context),
        ("Règles de cet artefact", &instructions.rules),
    ] {
        if blocs.is_empty() {
            continue;
        }
        let _ = writeln!(out);
        let _ = writeln!(out, "{titre} :");
        for bloc in blocs {
            let _ = writeln!(out, "  ({}) {}", bloc.origin, bloc.text);
        }
    }

    // Décisions en vigueur — une ligne par entrée. Section absente quand la
    // liste est vide (le champ existe côté JSON, mais un affichage humain
    // n'a pas à porter un titre vide).
    if !instructions.decisions.is_empty() {
        let _ = writeln!(out);
        let _ = writeln!(out, "Décisions en vigueur :");
        for decision in &instructions.decisions {
            let _ = writeln!(out, "  - {} {}", decision.id, decision.title);
        }
    }

    if let Some(instruction) = &instructions.instruction {
        let _ = writeln!(out);
        let _ = writeln!(out, "Consigne :");
        let _ = writeln!(out);
        let _ = writeln!(out, "{}", instruction.trim_end());
    }

    if let Some(template) = &instructions.template {
        let _ = writeln!(out);
        let _ = writeln!(out, "Template :");
        let _ = writeln!(out);
        let _ = writeln!(out, "{}", template.trim_end());
    }

    if !instructions.unlocks.is_empty() {
        let _ = writeln!(out);
        let _ = writeln!(out, "Débloquera : {}", instructions.unlocks.join(", "));
    }
    out
}

/// Sortie humaine du rapport de validation.
///
/// Trois principes : une ligne par finding (pas de tableau ASCII fragile),
/// chaque ligne préfixée de `path:line:` pour tenir sous 80 colonnes, et un
/// résumé final qui distingue « rien trouvé » de « rien à valider ».
pub fn validate(report: &ValidateReport) -> String {
    let mut out = String::new();

    if report.items.is_empty() {
        let _ = writeln!(
            out,
            "Rien à valider — aucun change actif et aucune spec principale."
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
        let _ = writeln!(
            out,
            "\n{} {} — {}",
            label,
            item.name,
            item.path.display(),
        );
        if item.findings.is_empty() {
            let _ = writeln!(out, "  ✓ aucun défaut");
            continue;
        }
        for finding in &item.findings {
            total_findings += 1;
            if finding.finding.severity == Severity::Error {
                total_errors += 1;
            }
            let severity_marker = match finding.finding.severity {
                Severity::Error => "erreur ",
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
        let _ = writeln!(
            out,
            "{} item(s) validé(s), aucun défaut.",
            report.items.len()
        );
    } else {
        let _ = writeln!(
            out,
            "{} item(s) validé(s), {} défaut(s) dont {} erreur(s).",
            report.items.len(),
            total_findings,
            total_errors
        );
    }
    out
}

/// Rendu humain du résultat d'un `codev sync`.
///
/// Regroupe par catégorie — créés, mis à jour, inchangés — plutôt qu'une
/// ligne mêlée. Un « rien à faire » explicite quand tout est inchangé.
pub fn sync(outcome: &SyncOutcome) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "Sync du change « {} »", outcome.change);
    render_categories(
        &mut out,
        &outcome.created,
        &outcome.updated,
        &outcome.unchanged,
        &outcome.deleted,
    );
    if !outcome.changed_anything() {
        let _ = writeln!(out, "\nRien à faire — les specs principales sont déjà à jour.");
    }
    out
}

/// Rendu humain du résultat d'un `codev archive`.
pub fn archive(outcome: &ArchiveOutcome) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "Archive du change « {} »", outcome.change);
    render_categories(
        &mut out,
        &outcome.created,
        &outcome.updated,
        &outcome.unchanged,
        &outcome.deleted,
    );
    let _ = writeln!(out, "\nDéplacé vers : {}", outcome.moved_to.display());
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
        ("Créé(s)", created),
        ("Mis à jour", updated),
        ("Inchangé(s)", unchanged),
        ("Supprimé(s)", deleted),
    ] {
        if paths.is_empty() {
            continue;
        }
        let _ = writeln!(out);
        let _ = writeln!(out, "{label} :");
        for p in paths {
            let _ = writeln!(out, "  {}", p.display());
        }
    }
}

/// Rendu humain de `codev decision list`.
pub fn decision_list(outcome: &DecisionListOutcome) -> String {
    if outcome.decisions.is_empty() {
        return "Aucune décision. `codev decision new <titre>` pour en créer une.\n".to_string();
    }
    let mut out = String::from("Décisions :\n");
    for d in &outcome.decisions {
        let marker = if d.in_effect { "•" } else { "–" };
        let _ = writeln!(out, "  {marker} {} {}  [{}]", d.id, d.title, d.status);
        if let Some(by) = &d.superseded_by {
            let _ = writeln!(out, "      supersedée par {by}");
        }
    }
    out
}

/// Rendu humain de `codev decision show`.
pub fn decision_show(outcome: &DecisionShowOutcome) -> String {
    let d = &outcome.decision;
    let mut out = String::new();
    let _ = writeln!(out, "ID     : {} ({})", d.id, d.qualified_id);
    let _ = writeln!(out, "Titre  : {}", d.title);
    let _ = writeln!(out, "Statut : {}", d.status);
    let _ = writeln!(out, "Date   : {}", d.date);
    if !d.tags.is_empty() {
        let _ = writeln!(out, "Tags   : {}", d.tags.join(", "));
    }
    if !d.supersedes.is_empty() {
        let _ = writeln!(out, "Supersede : {}", d.supersedes.join(", "));
    }
    if let Some(by) = &d.superseded_by {
        let _ = writeln!(out, "Supersedée par : {by}");
    }
    let _ = writeln!(out, "Fichier : {}", d.path.display());
    let _ = writeln!(out);
    out.push_str(&outcome.content);
    out
}

/// Rendu humain de `codev decision new`.
pub fn decision_created(outcome: &DecisionCreatedOutcome) -> String {
    let mut out = String::new();
    let _ = writeln!(
        out,
        "✓ Décision « {} {} » créée",
        outcome.decision.id, outcome.decision.title
    );
    let _ = writeln!(out, "  Fichier : {}", outcome.path.display());
    let _ = writeln!(
        out,
        "\nOuvre le fichier pour rédiger le Contexte, la Décision et les Conséquences."
    );
    out
}

/// Rendu humain de `codev decision promote`.
pub fn decision_promoted(outcome: &DecisionPromotedOutcome) -> String {
    let mut out = String::new();
    let _ = writeln!(
        out,
        "✓ Décision « {} » promue en ADR {} : {}",
        outcome.decision.title, outcome.decision.id, outcome.decision.title
    );
    let _ = writeln!(out, "  Fichier    : {}", outcome.path.display());
    let _ = writeln!(out, "  Hash       : {}", outcome.body_sha256);
    let _ = writeln!(out, "  Source     : {}", outcome.design_path.display());
    let _ = writeln!(
        out,
        "\nVentile le corps en Contexte / Décision / Conséquences / Alternatives\n\
         écartées avant d'archiver le change « {} ».",
        outcome.source_change
    );
    out
}

/// Rendu humain de `codev decision deviate`.
pub fn decision_deviated(outcome: &DecisionDeviatedOutcome) -> String {
    let mut out = String::new();
    let _ = writeln!(
        out,
        "✓ Dérive locale de « {} » créée : {} {}",
        outcome.target_qualified_id, outcome.decision.id, outcome.decision.title
    );
    let _ = writeln!(out, "  Fichier : {}", outcome.path.display());
    let _ = writeln!(out, "  Hash    : {}", outcome.body_sha256);
    let _ = writeln!(
        out,
        "\nL'ADR est scellé. Ouvre-le pour rédiger le Contexte et la Décision."
    );
    out
}

/// Rendu humain de `codev decision seal`.
pub fn decision_sealed(outcome: &DecisionSealedOutcome) -> String {
    let mut out = String::new();
    if outcome.was_noop {
        let _ = writeln!(out, "Déjà à jour : {} ({})", outcome.id, outcome.body_sha256);
    } else if outcome.was_forced {
        let _ = writeln!(
            out,
            "✓ Re-scellé : {} ({})",
            outcome.id, outcome.body_sha256
        );
    } else {
        let _ = writeln!(out, "✓ Scellé : {} ({})", outcome.id, outcome.body_sha256);
    }
    out
}

/// Rendu humain de `codev decision supersede`.
pub fn decision_superseded(outcome: &DecisionSupersededOutcome) -> String {
    let mut out = String::new();
    let _ = writeln!(
        out,
        "✓ Décision « {} » supersedée par « {} {} »",
        outcome.old_qualified_id, outcome.new_decision.id, outcome.new_decision.title
    );
    let _ = writeln!(out, "  Nouvel ADR       : {}", outcome.new_path.display());
    let _ = writeln!(
        out,
        "  Ancien retitré   : {} (status: superseded)",
        outcome.old_path.display()
    );
    let _ = writeln!(
        out,
        "\nOuvre le nouveau fichier pour rédiger la Décision et ce qui change."
    );
    out
}

/// Rendu humain de `codev sources list`.
pub fn sources_list(outcome: &SourcesListOutcome) -> String {
    if outcome.sources.is_empty() {
        return "Aucune source héritée déclarée dans _codev/config.yaml.\n".to_string();
    }
    let mut out = String::from("Sources héritées :\n");
    for s in &outcome.sources {
        let _ = writeln!(
            out,
            "  {} {}  [{}]",
            s.kind.as_str(),
            s.address,
            s.state.as_str()
        );
        if let Some(sha) = &s.sha {
            let _ = writeln!(out, "      commit : {sha}");
        }
        if let Some(path) = &s.resolved_path {
            let _ = writeln!(out, "      chemin : {}", path.display());
        }
    }
    out
}

/// Rendu humain de `codev sources update`.
pub fn sources_update(outcome: &SourcesUpdateOutcome) -> String {
    let mut out = String::new();
    if outcome.diff.is_empty() {
        let _ = writeln!(out, "Aucune source `git:` à mettre à jour.");
        return out;
    }
    let _ = writeln!(out, "Changements :");
    use codev_engine::sources::PinChange::*;
    for change in &outcome.diff {
        match change {
            Added { url, git_ref, to } => {
                let _ = writeln!(out, "  + {url} @{git_ref}  → {to}");
            }
            Moved { url, git_ref, from, to } => {
                let _ = writeln!(out, "  ~ {url} @{git_ref}  {from} → {to}");
            }
            Unchanged { url, git_ref, sha } => {
                let _ = writeln!(out, "  = {url} @{git_ref}  ({sha})");
            }
        }
    }
    let _ = writeln!(out);
    if outcome.lock_written {
        let _ = writeln!(out, "codev.lock mis à jour.");
    } else {
        let _ = writeln!(out, "codev.lock inchangé.");
    }
    out
}

/// Rendu humain de `codev sources show`.
pub fn sources_show(outcome: &SourcesShowOutcome) -> String {
    let mut out = String::new();
    let s = &outcome.source;
    let _ = writeln!(out, "Type    : {}", s.kind.as_str());
    let _ = writeln!(out, "Adresse : {}", s.address);
    if let Some(git_ref) = &s.git_ref {
        let _ = writeln!(out, "Ref     : {git_ref}");
    }
    if let Some(sha) = &s.sha {
        let _ = writeln!(out, "SHA     : {sha}");
    }
    if let Some(subpath) = &s.subpath {
        let _ = writeln!(out, "Subpath : {subpath}");
    }
    let _ = writeln!(out, "État    : {}", s.state.as_str());
    if let Some(path) = &s.resolved_path {
        let _ = writeln!(out, "Chemin  : {}", path.display());
    }
    if !outcome.files_exposed.is_empty() {
        let _ = writeln!(out);
        let _ = writeln!(
            out,
            "Fichiers exposés ({} au total) :",
            outcome.files_exposed.len()
        );
        for file in outcome.files_exposed.iter().take(20) {
            let _ = writeln!(out, "  {file}");
        }
        if outcome.files_exposed.len() > 20 {
            let _ = writeln!(out, "  … ({} de plus)", outcome.files_exposed.len() - 20);
        }
    }
    out
}

pub fn changes(outcome: &ChangesOutcome) -> String {
    if outcome.changes.is_empty() {
        return "Aucun change actif. `codev new change <nom>` pour en créer un.\n".to_string();
    }
    let mut out = String::from("Changes actifs :\n");
    for change in &outcome.changes {
        let _ = writeln!(out, "  {change}");
    }
    out
}

pub fn specs(outcome: &SpecsOutcome) -> String {
    if outcome.specs.is_empty() {
        return "Aucune spec. Elles apparaissent quand un change est synchronisé ou archivé.\n"
            .to_string();
    }
    let mut out = String::from("Capacités spécifiées :\n");
    for spec in &outcome.specs {
        let _ = writeln!(out, "  {spec}");
    }
    out
}

pub fn schemas(outcome: &SchemasOutcome) -> String {
    let mut out = String::from("Schémas disponibles :\n");
    for schema in &outcome.schemas {
        let _ = writeln!(out, "  {} ({})", schema.name, schema.origin);
        let _ = writeln!(out, "    {}", schema.flow.join(" → "));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use codev_core::{status as core_status, ArtifactGraph, ChangeId};
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

    fn statut(existing: &[&str], skipped: &[&str]) -> ChangeStatus {
        let graph = ArtifactGraph::from_yaml(SPEC_DRIVEN).unwrap();
        let change = ChangeId::parse("add-auth").unwrap();
        let to_set = |ids: &[&str]| -> BTreeSet<String> {
            ids.iter().map(|s| s.to_string()).collect()
        };
        core_status::compute(&graph, &change, &to_set(existing), &to_set(skipped))
    }

    #[test]
    fn instructions_design_liste_les_decisions_en_vigueur() {
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
                qualified_id: "projet/0001".into(),
                title: "Fondation".into(),
                status: "accepted".into(),
                tags: vec![],
                path: PathBuf::from("_codev/decisions/0001-fondation.md"),
                origin: "projet".into(),
            }],
            skipped: false,
            warnings: Vec::new(),
        };
        let rendu = super::instructions(&instr);
        assert!(rendu.contains("Décisions en vigueur"), "{rendu}");
        assert!(rendu.contains("- 0001 Fondation"), "{rendu}");
    }

    #[test]
    fn instructions_sans_decisions_nest_pas_seche_de_titre_vide() {
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
        let rendu = super::instructions(&instr);
        assert!(
            !rendu.contains("Décisions en vigueur"),
            "aucun titre attendu quand la liste est vide : {rendu}"
        );
    }

    #[test]
    fn le_statut_montre_letat_et_le_prochain() {
        let rendu = status(&statut(&["proposal"], &[]));
        assert!(rendu.contains("[x] proposal"), "{rendu}");
        assert!(rendu.contains("[ ] specs"), "{rendu}");
        assert!(rendu.contains("[-] tasks"), "{rendu}");
        assert!(rendu.contains("(attend : specs)"), "{rendu}");
        assert!(rendu.contains("Prochain      : specs"), "{rendu}");
        assert!(rendu.contains("1/3 artefacts"), "{rendu}");
    }

    #[test]
    fn le_statut_dit_quand_la_planification_est_complete() {
        let rendu = status(&statut(&["proposal", "specs", "tasks"], &[]));
        assert!(rendu.contains("complète"), "{rendu}");
        assert!(!rendu.contains("Prochain"), "{rendu}");
    }

    #[test]
    fn le_statut_signale_un_artefact_neutralise() {
        let rendu = status(&statut(&["proposal"], &["specs"]));
        assert!(rendu.contains("[~] specs"), "{rendu}");
        assert!(rendu.contains("neutralisé"), "{rendu}");
    }

    #[test]
    fn une_liste_vide_dit_quoi_faire_ensuite() {
        let outcome = ChangesOutcome {
            root: "/p".into(),
            changes: Vec::new(),
        };
        assert!(changes(&outcome).contains("codev new change"));
    }
}
