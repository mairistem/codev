//! Orchestration de la validation : lit les fichiers, appelle le parseur et
//! le registre de règles, produit un rapport.
//!
//! Aucune logique de règle ici — ces règles sont pures et vivent dans
//! `codev-core::validate`. On coordonne, on groupe, on décide un exit code.
//! C'est la coquille au sens de la décision
//! `_codev/decisions/0001-coeur-fonctionnel-coquille-imperative.md`.

pub mod metadata_rules;
pub mod report;

pub use metadata_rules::check_change_metadata;
pub use report::{ItemKind, ItemReport, LocatedFinding, ValidateReport};

use std::path::PathBuf;

use codev_core::decisions::seal::{self, VerificationCase};
use codev_core::parser::ast::{Finding, Severity};
use codev_core::parser::{parse_delta, parse_spec};
use codev_core::validate as core_validate;
use codev_core::{ChangeId, Layout};

use crate::change;
use crate::config::ResolvedConfig;
use crate::decisions as decisions_index;
use crate::decisions::Origin;
use crate::decisions_actions;
use crate::error::{EngineError, Result};
use crate::ports::{Env, FileSystem};
use crate::specs;

/// Codes stables des findings émis par la vérification du scellement.
///
/// Exposés côté `codev-cli::contract` pour que l'agent puisse tester
/// dessus. En cas de changement de nom, c'est un breaking du contrat
/// public — d'où le fait de les figer ici, pas dans une chaîne inline.
pub mod seal_codes {
    pub const DECISION_UNSEALED: &str = "decision_unsealed";
    pub const DECISION_SEAL_MISMATCH: &str = "decision_seal_mismatch";
    pub const DECISION_ORPHAN_SEAL: &str = "decision_orphan_seal";
}

/// Valide un change : ses métadonnées et chacun de ses fichiers de delta.
pub fn validate_change(
    fs: &dyn FileSystem,
    layout: &Layout,
    config: &ResolvedConfig,
    change_id: &ChangeId,
) -> Result<ItemReport> {
    let ctx = change::load(fs, layout, config, change_id.clone())?;
    let change_dir = layout.change_dir(change_id);

    let mut findings = check_change_metadata(fs, layout, &ctx)?;

    // Chaque delta est lu une fois, parsé une fois : les findings du parseur
    // et ceux des règles sortent du même passage.
    for delta_path in locate_deltas(fs, &change_dir)? {
        let relative = delta_path
            .strip_prefix(layout.project_root())
            .unwrap_or(&delta_path)
            .to_path_buf();

        let source = fs.read_to_string(&delta_path).map_err(|e| EngineError::Unreadable {
            path: delta_path.clone(),
            reason: e.to_string(),
        })?;
        let parsed = parse_delta(&source);

        for f in parsed.findings.iter().chain(core_validate::check_delta(&parsed.value).iter()) {
            findings.push(LocatedFinding::from_finding(f.clone(), relative.clone()));
        }
    }

    Ok(ItemReport {
        kind: ItemKind::Change,
        name: change_id.to_string(),
        path: change_dir_relative(layout, change_id),
        findings,
    })
}

/// Valide une spec principale, désignée par son chemin de capacité relatif à
/// `_codev/specs/`.
pub fn validate_spec(
    fs: &dyn FileSystem,
    layout: &Layout,
    capability: &str,
) -> Result<ItemReport> {
    let path = layout.spec_file(capability);
    let relative = path
        .strip_prefix(layout.project_root())
        .unwrap_or(&path)
        .to_path_buf();

    let source = fs.read_to_string(&path).map_err(|e| EngineError::Unreadable {
        path: path.clone(),
        reason: e.to_string(),
    })?;
    let parsed = parse_spec(&source);
    let findings: Vec<LocatedFinding> = parsed
        .findings
        .iter()
        .chain(core_validate::check_spec(&parsed.value).iter())
        .cloned()
        .map(|f| LocatedFinding::from_finding(f, relative.clone()))
        .collect();

    Ok(ItemReport {
        kind: ItemKind::Spec,
        name: capability.to_string(),
        path: relative,
        findings,
    })
}

/// Valide tous les changes actifs et toutes les specs principales du projet.
pub fn validate_all(
    fs: &dyn FileSystem,
    env: &dyn Env,
    layout: &Layout,
    config: &ResolvedConfig,
) -> Result<ValidateReport> {
    let mut items = Vec::new();
    for change_id in change::list(fs, layout) {
        items.push(validate_change(fs, layout, config, &change_id)?);
    }
    for capability in specs::list(fs, layout) {
        items.push(validate_spec(fs, layout, &capability)?);
    }
    items.push(validate_decisions(fs, env, layout, config)?);
    Ok(ValidateReport {
        root: layout.project_root().to_path_buf(),
        items,
    })
}

/// Valide le sous-système « décisions » : compare chaque ADR local à son
/// entrée dans `seal.yaml` et émet des findings pour chaque écart.
///
/// - `decision_unsealed` (warning) : ADR sans entrée de sceau — migration
///   attendue via `codev decision seal`.
/// - `decision_seal_mismatch` (**erreur**) : le corps courant ne
///   correspond plus au hash enregistré.
/// - `decision_orphan_seal` (warning) : entrée de sceau pour un ADR qui
///   n'existe plus.
pub fn validate_decisions(
    fs: &dyn FileSystem,
    env: &dyn Env,
    layout: &Layout,
    config: &ResolvedConfig,
) -> Result<ItemReport> {
    let index = decisions_index::index(fs, env, layout, config)?;
    let seal_file = decisions_actions::read_seal_file(fs, layout).map_err(|e| {
        EngineError::Unreadable {
            path: layout.decisions_seal_file(),
            reason: e.to_string(),
        }
    })?;

    // Seuls les ADR locaux `accepted` ou `superseded` portent un
    // engagement d'immutabilité — les autres statuts (proposed,
    // deprecated, rejected) ne se scellent pas.
    let mut present: Vec<(String, String, PathBuf)> = Vec::new();
    for entry in &index.entries {
        if entry.qualified_id.origin != Origin::Project {
            continue;
        }
        use codev_core::decisions::DecisionStatus::*;
        if !matches!(entry.decision.status, Accepted | Superseded) {
            continue;
        }
        let source =
            fs.read_to_string(&entry.path)
                .map_err(|e| EngineError::Unreadable {
                    path: entry.path.clone(),
                    reason: e.to_string(),
                })?;
        let hash = match seal::body_hash(&source) {
            Ok(h) => h,
            Err(_) => {
                // Sans frontmatter fermant, l'ADR n'aurait même pas été
                // indexé — on peut sauter en silence.
                continue;
            }
        };
        present.push((entry.decision.id.clone(), hash, entry.path.clone()));
    }

    // Pour cartographier chaque cas à son chemin, on garde un index id → path.
    let paths_by_id: std::collections::HashMap<String, PathBuf> = present
        .iter()
        .map(|(id, _, path)| (id.clone(), path.clone()))
        .collect();

    let cases = seal::verify(
        &seal_file,
        &present
            .iter()
            .map(|(id, hash, _)| (id.clone(), hash.clone()))
            .collect::<Vec<_>>(),
    );

    let seal_path = layout.decisions_seal_file();
    let seal_relative = seal_path
        .strip_prefix(layout.project_root())
        .unwrap_or(&seal_path)
        .to_path_buf();

    let mut findings: Vec<LocatedFinding> = Vec::new();
    for case in cases {
        let (finding, path) = match case {
            VerificationCase::Unsealed { id } => {
                let path = paths_by_id.get(&id).cloned().unwrap_or_else(|| seal_path.clone());
                let relative = path
                    .strip_prefix(layout.project_root())
                    .unwrap_or(&path)
                    .to_path_buf();
                (
                    Finding {
                        severity: Severity::Warning,
                        code: seal_codes::DECISION_UNSEALED,
                        line: 1,
                        message: format!(
                            "la décision « {id} » n'est pas scellée ; \
                             lance `codev decision seal {id}` pour enregistrer \
                             le hash de son corps"
                        ),
                    },
                    relative,
                )
            }
            VerificationCase::Mismatch {
                id,
                recorded,
                actual,
            } => {
                let path = paths_by_id.get(&id).cloned().unwrap_or_else(|| seal_path.clone());
                let relative = path
                    .strip_prefix(layout.project_root())
                    .unwrap_or(&path)
                    .to_path_buf();
                (
                    Finding {
                        severity: Severity::Error,
                        code: seal_codes::DECISION_SEAL_MISMATCH,
                        line: 1,
                        message: format!(
                            "le corps de la décision « {id} » ne correspond plus \
                             à son sceau — sceau : {recorded}, corps actuel : {actual}. \
                             Réécris le sceau délibérément avec \
                             `codev decision seal {id} --force`."
                        ),
                    },
                    relative,
                )
            }
            VerificationCase::OrphanSeal { id } => (
                Finding {
                    severity: Severity::Warning,
                    code: seal_codes::DECISION_ORPHAN_SEAL,
                    line: 1,
                    message: format!(
                        "le sceau de « {id} » n'a plus d'ADR correspondant \
                         dans _codev/decisions/"
                    ),
                },
                seal_relative.clone(),
            ),
        };
        findings.push(LocatedFinding::from_finding(finding, path));
    }

    // Le rapport porte aussi les findings du parseur d'ADR (frontmatter
    // manquant, id absent, statut inconnu…) qu'on ne veut pas perdre.
    for f in index.findings {
        // Les findings du parseur ne portent pas de chemin — on utilise
        // le dossier des décisions comme ancre par défaut.
        let anchor = layout
            .decisions_dir()
            .strip_prefix(layout.project_root())
            .unwrap_or(&layout.decisions_dir())
            .to_path_buf();
        findings.push(LocatedFinding::from_finding(f, anchor));
    }

    Ok(ItemReport {
        kind: ItemKind::Decisions,
        name: "decisions".into(),
        path: layout
            .decisions_dir()
            .strip_prefix(layout.project_root())
            .unwrap_or(&layout.decisions_dir())
            .to_path_buf(),
        findings,
    })
}

/// Repère les fichiers `*.md` sous `changes/<name>/specs/**`.
fn locate_deltas(fs: &dyn FileSystem, change_dir: &std::path::Path) -> Result<Vec<PathBuf>> {
    let specs_dir = change_dir.join("specs");
    let files = fs.walk_files(&specs_dir).map_err(|e| EngineError::Unreadable {
        path: specs_dir.clone(),
        reason: e.to_string(),
    })?;
    let mut out: Vec<PathBuf> = files
        .into_iter()
        .filter(|p| p.ends_with(".md"))
        .map(|p| specs_dir.join(p))
        .collect();
    out.sort();
    Ok(out)
}

fn change_dir_relative(layout: &Layout, change: &ChangeId) -> PathBuf {
    let full = layout.change_dir(change);
    full.strip_prefix(layout.project_root())
        .unwrap_or(&full)
        .to_path_buf()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ports::{FixedEnv, MemoryFileSystem};
    use codev_core::parser::codes as parser_codes;
    use codev_core::validate::codes as rule_codes;

    fn env() -> FixedEnv {
        let mut env = FixedEnv::at("/p");
        env.vars.insert("HOME".into(), "/home".into());
        env
    }

    fn config(fs: &dyn FileSystem) -> ResolvedConfig {
        crate::config::resolve(fs, &env(), &Layout::new("/p")).unwrap()
    }

    fn projet_avec_change_bien_forme() -> MemoryFileSystem {
        MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file(
                "/p/_codev/changes/add-auth/change.yaml",
                "schema: spec-driven",
            )
            .with_file(
                "/p/_codev/changes/add-auth/proposal.md",
                "# Proposal\n\n## Pourquoi\n\nTest\n",
            )
            .with_file(
                "/p/_codev/changes/add-auth/specs/user-auth/spec.md",
                "## Purpose\n\nGère l'authentification des utilisateurs.\n\n## ADDED Requirements\n\n### Requirement: Login\nThe system SHALL issue a token.\n\n#### Scenario: OK\n- **WHEN** login\n- **THEN** token\n",
            )
    }

    #[test]
    fn rapport_change_couvre_tous_les_fichiers_de_delta() {
        let fs = projet_avec_change_bien_forme().with_file(
            "/p/_codev/changes/add-auth/specs/second/spec.md",
            "## Purpose\n\nDeuxième capacité.\n\n## ADDED Requirements\n\n### Requirement: Second\nThe system SHALL do.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n",
        );
        let cfg = config(&fs);
        let report = validate_change(
            &fs,
            &Layout::new("/p"),
            &cfg,
            &ChangeId::parse("add-auth").unwrap(),
        )
        .unwrap();

        assert_eq!(report.kind, ItemKind::Change);
        assert_eq!(report.name, "add-auth");
        assert!(report.findings.is_empty(), "findings inattendus : {:?}", report.findings);
    }

    #[test]
    fn rapport_spec_expose_les_findings_du_parseur() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file(
                "/p/_codev/specs/user-auth/spec.md",
                "## Requirements\n\n### Requirement: X\nThe system SHALL x.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n",
            );
        let report = validate_spec(&fs, &Layout::new("/p"), "user-auth").unwrap();
        assert!(report
            .findings
            .iter()
            .any(|f| f.finding.code == parser_codes::SPEC_PURPOSE_MISSING));
    }

    #[test]
    fn rapport_spec_expose_les_findings_du_registre() {
        // Spec avec Purpose mais sans exigence : la règle SpecNoRequirement du
        // registre doit sortir.
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file(
                "/p/_codev/specs/user-auth/spec.md",
                "## Purpose\n\nGérer les utilisateurs.\n\n## Requirements\n",
            );
        let report = validate_spec(&fs, &Layout::new("/p"), "user-auth").unwrap();
        assert!(report
            .findings
            .iter()
            .any(|f| f.finding.code == rule_codes::SPEC_NO_REQUIREMENT));
    }

    #[test]
    fn validate_all_couvre_changes_et_specs() {
        let fs = projet_avec_change_bien_forme().with_file(
            "/p/_codev/specs/user-auth/spec.md",
            "## Purpose\n\nSpec principale existante.\n\n## Requirements\n\n### Requirement: Old\nThe system SHALL persist.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n",
        );
        let cfg = config(&fs);
        let report = validate_all(&fs, &env(), &Layout::new("/p"), &cfg).unwrap();

        // 1 change + 1 spec principale + le rapport décisions (toujours
        // présent, même sans ADR).
        assert_eq!(report.items.len(), 3);
        assert!(report
            .items
            .iter()
            .any(|i| i.kind == ItemKind::Change && i.name == "add-auth"));
        assert!(report
            .items
            .iter()
            .any(|i| i.kind == ItemKind::Spec && i.name == "user-auth"));
        assert!(!report.has_errors());
    }

    // ─────────────── validate_decisions ───────────────

    fn adr_source(id: &str) -> String {
        format!(
            "---\nid: \"{id}\"\ntitle: T\nstatus: accepted\ndate: 2026-09-09\n---\n\n## Contexte\n\nx\n"
        )
    }

    #[test]
    fn decisions_sans_seal_remontent_unsealed() {
        // Migration typique : 3 ADR acceptés, pas de seal.yaml.
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file("/p/_codev/decisions/0001-a.md", adr_source("0001"))
            .with_file("/p/_codev/decisions/0002-b.md", adr_source("0002"))
            .with_file("/p/_codev/decisions/0003-c.md", adr_source("0003"));
        let cfg = config(&fs);
        let report = validate_decisions(&fs, &env(), &Layout::new("/p"), &cfg).unwrap();
        let codes: Vec<&str> = report.findings.iter().map(|f| f.finding.code).collect();
        assert_eq!(
            codes,
            vec![
                seal_codes::DECISION_UNSEALED,
                seal_codes::DECISION_UNSEALED,
                seal_codes::DECISION_UNSEALED,
            ]
        );
        // Warnings, pas erreurs — la migration ne bloque pas les autres flows.
        assert!(!report.has_errors());
    }

    #[test]
    fn corps_modifie_apres_scellement_remonte_mismatch_en_erreur() {
        let adr = adr_source("0001");
        let hash = seal::body_hash(&adr).unwrap();
        let seal_yaml = format!(
            "version: 1\nseals:\n  - id: \"0001\"\n    bodySha256: \"{hash}\"\n    sealedAt: 2026-09-09\n"
        );
        // On corrompt le corps après scellement.
        let modified = adr.replace("\n\nx\n", "\n\nx modifié\n");
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file("/p/_codev/decisions/0001-a.md", &modified)
            .with_file("/p/_codev/decisions/seal.yaml", &seal_yaml);
        let cfg = config(&fs);
        let report = validate_decisions(&fs, &env(), &Layout::new("/p"), &cfg).unwrap();
        assert!(report
            .findings
            .iter()
            .any(|f| f.finding.code == seal_codes::DECISION_SEAL_MISMATCH));
        // C'est bien une erreur : l'index de décisions ne peut plus être
        // considéré fiable.
        assert!(report.has_errors());
    }

    #[test]
    fn sceau_orphelin_remonte_warning() {
        // Un sceau pour 0009 mais l'ADR n'existe plus.
        let seal_yaml = "version: 1\nseals:\n  - id: \"0009\"\n    bodySha256: \"sha256:whatever\"\n    sealedAt: 2026-09-09\n";
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file("/p/_codev/decisions/seal.yaml", seal_yaml);
        let cfg = config(&fs);
        let report = validate_decisions(&fs, &env(), &Layout::new("/p"), &cfg).unwrap();
        assert!(report
            .findings
            .iter()
            .any(|f| f.finding.code == seal_codes::DECISION_ORPHAN_SEAL));
        assert!(!report.has_errors());
    }

    #[test]
    fn validate_remonte_dangling_deviation_en_warning() {
        // Un ADR local `deviates_from` d'une cible qui n'existe pas →
        // warning, exit code nul.
        let adr = "---\nid: \"0007\"\ntitle: T\nstatus: accepted\ndate: 2026-09-09\ndeviates_from: [\"path:~/inconnue/9999\"]\n---\n\n## Contexte\n\nx\n";
        let hash = seal::body_hash(adr).unwrap();
        let seal_yaml = format!(
            "version: 1\nseals:\n  - id: \"0007\"\n    bodySha256: \"{hash}\"\n    sealedAt: 2026-09-09\n"
        );
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file("/p/_codev/decisions/0007-alt.md", adr)
            .with_file("/p/_codev/decisions/seal.yaml", &seal_yaml);
        let cfg = config(&fs);
        let report = validate_decisions(&fs, &env(), &Layout::new("/p"), &cfg).unwrap();
        assert!(report
            .findings
            .iter()
            .any(|f| f.finding.code == codev_core::parser::codes::DECISION_DANGLING_DEVIATION));
        assert!(!report.has_errors(), "dangling deviation reste un warning");
    }

    #[test]
    fn validate_remonte_conflicting_deviations_en_erreur() {
        // Deux ADR locaux qui dévient de la même cible → erreur.
        let adr_a = "---\nid: \"0007\"\ntitle: A\nstatus: accepted\ndate: 2026-09-09\ndeviates_from: [\"path:~/partage/0100\"]\n---\n\n## Contexte\n\na\n";
        let adr_b = "---\nid: \"0008\"\ntitle: B\nstatus: accepted\ndate: 2026-09-09\ndeviates_from: [\"path:~/partage/0100\"]\n---\n\n## Contexte\n\nb\n";
        let adr_h = "---\nid: \"0100\"\ntitle: Source\nstatus: accepted\ndate: 2026-09-09\n---\n\n## Contexte\n\ns\n";
        let hash_a = seal::body_hash(adr_a).unwrap();
        let hash_b = seal::body_hash(adr_b).unwrap();
        let seal_yaml = format!(
            "version: 1\nseals:\n  - id: \"0007\"\n    bodySha256: \"{hash_a}\"\n    sealedAt: 2026-09-09\n  - id: \"0008\"\n    bodySha256: \"{hash_b}\"\n    sealedAt: 2026-09-09\n"
        );
        let fs = MemoryFileSystem::new()
            .with_file(
                "/p/_codev/config.yaml",
                "inherits:\n  - path: ~/partage\n",
            )
            .with_file("/p/_codev/decisions/0007-a.md", adr_a)
            .with_file("/p/_codev/decisions/0008-b.md", adr_b)
            .with_file("/home/partage/_codev/decisions/0100.md", adr_h)
            .with_file("/p/_codev/decisions/seal.yaml", &seal_yaml);
        let cfg = config(&fs);
        let report = validate_decisions(&fs, &env(), &Layout::new("/p"), &cfg).unwrap();
        assert!(report
            .findings
            .iter()
            .any(|f| f.finding.code == codev_core::parser::codes::DECISION_CONFLICTING_DEVIATIONS));
        assert!(report.has_errors(), "conflict must be an error");
    }

    #[test]
    fn adr_herite_nest_pas_verifie_par_le_projet() {
        // Un ADR local scellé (OK) + un ADR hérité (jamais scellé par
        // le consommateur) → aucun finding pour l'hérité.
        let local = adr_source("0001");
        let hash = seal::body_hash(&local).unwrap();
        let seal_yaml = format!(
            "version: 1\nseals:\n  - id: \"0001\"\n    bodySha256: \"{hash}\"\n    sealedAt: 2026-09-09\n"
        );
        let fs = MemoryFileSystem::new()
            .with_file(
                "/p/_codev/config.yaml",
                "inherits:\n  - path: ~/partage\n",
            )
            .with_file("/p/_codev/decisions/0001-a.md", &local)
            .with_file("/p/_codev/decisions/seal.yaml", &seal_yaml)
            .with_file(
                "/home/partage/_codev/decisions/0100-h.md",
                adr_source("0100"),
            );
        let cfg = config(&fs);
        let report = validate_decisions(&fs, &env(), &Layout::new("/p"), &cfg).unwrap();
        // Aucun finding pour 0100 — c'est au projet source de sceller.
        assert!(report.findings.is_empty(), "{:#?}", report.findings);
    }
}
