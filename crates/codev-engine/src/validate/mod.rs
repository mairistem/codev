//! Validation orchestration: reads the files, calls the parser and the rule
//! registry, produces a report.
//!
//! No rule logic here — the rules are pure and live in
//! `codev-core::validate`. We coordinate, group, and decide an exit code.
//! This is the shell in the sense of the decision
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

/// Stable codes of the findings emitted by seal verification.
///
/// Exposed through `codev-cli::contract` so that the agent can test
/// against them. Renaming one is a breaking change to the public
/// contract — hence freezing them here, not in an inline string.
pub mod seal_codes {
    pub const DECISION_UNSEALED: &str = "decision_unsealed";
    pub const DECISION_SEAL_MISMATCH: &str = "decision_seal_mismatch";
    pub const DECISION_ORPHAN_SEAL: &str = "decision_orphan_seal";
}

/// Validates a change: its metadata and each of its delta files.
pub fn validate_change(
    fs: &dyn FileSystem,
    layout: &Layout,
    config: &ResolvedConfig,
    change_id: &ChangeId,
) -> Result<ItemReport> {
    let ctx = change::load(fs, layout, config, change_id.clone())?;
    let change_dir = layout.change_dir(change_id);

    let mut findings = check_change_metadata(fs, layout, &ctx)?;

    // Each delta is read once and parsed once: the parser's findings and
    // the rules' findings come out of the same pass.
    for delta_path in locate_deltas(fs, &change_dir)? {
        let relative = delta_path
            .strip_prefix(layout.project_root())
            .unwrap_or(&delta_path)
            .to_path_buf();

        let source = fs
            .read_to_string(&delta_path)
            .map_err(|e| EngineError::Unreadable {
                path: delta_path.clone(),
                reason: e.to_string(),
            })?;
        let parsed = parse_delta(&source);

        for f in parsed
            .findings
            .iter()
            .chain(core_validate::check_delta(&parsed.value).iter())
        {
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

/// Validates a main spec, designated by its capability path relative to
/// `_codev/specs/`.
pub fn validate_spec(fs: &dyn FileSystem, layout: &Layout, capability: &str) -> Result<ItemReport> {
    let path = layout.spec_file(capability);
    let relative = path
        .strip_prefix(layout.project_root())
        .unwrap_or(&path)
        .to_path_buf();

    let source = fs
        .read_to_string(&path)
        .map_err(|e| EngineError::Unreadable {
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

/// Validates all active changes and all main specs of the project.
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

/// Validates the "decisions" subsystem: compares each local ADR with its
/// entry in `seal.yaml` and emits a finding for each discrepancy.
///
/// - `decision_unsealed` (warning): ADR with no seal entry — migration
///   expected via `codev decision seal`.
/// - `decision_seal_mismatch` (**error**): the current body no longer
///   matches the recorded hash.
/// - `decision_orphan_seal` (warning): seal entry for an ADR that no
///   longer exists.
pub fn validate_decisions(
    fs: &dyn FileSystem,
    env: &dyn Env,
    layout: &Layout,
    config: &ResolvedConfig,
) -> Result<ItemReport> {
    let index = decisions_index::index(fs, env, layout, config)?;
    let seal_file =
        decisions_actions::read_seal_file(fs, layout).map_err(|e| EngineError::Unreadable {
            path: layout.decisions_seal_file(),
            reason: e.to_string(),
        })?;

    // Only local `accepted` or `superseded` ADRs carry an immutability
    // commitment — the other statuses (proposed, deprecated,
    // rejected) are not sealed.
    let mut present: Vec<(String, String, PathBuf)> = Vec::new();
    for entry in &index.entries {
        if entry.qualified_id.origin != Origin::Project {
            continue;
        }
        use codev_core::decisions::DecisionStatus::*;
        if !matches!(entry.decision.status, Accepted | Superseded) {
            continue;
        }
        let source = fs
            .read_to_string(&entry.path)
            .map_err(|e| EngineError::Unreadable {
                path: entry.path.clone(),
                reason: e.to_string(),
            })?;
        let hash = match seal::body_hash(&source) {
            Ok(h) => h,
            Err(_) => {
                // Without a closing frontmatter, the ADR would not even have
                // been indexed — it can be skipped silently.
                continue;
            }
        };
        present.push((entry.decision.id.clone(), hash, entry.path.clone()));
    }

    // To map each case to its path, we keep an id → path index.
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
                let path = paths_by_id
                    .get(&id)
                    .cloned()
                    .unwrap_or_else(|| seal_path.clone());
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
                            "decision `{id}` is not sealed; \
                             run `codev decision seal {id}` to record \
                             the hash of its body"
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
                let path = paths_by_id
                    .get(&id)
                    .cloned()
                    .unwrap_or_else(|| seal_path.clone());
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
                            "the body of decision `{id}` no longer matches \
                             its seal — seal: {recorded}, current body: {actual}. \
                             Rewrite the seal deliberately with \
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
                        "the seal for `{id}` no longer has a matching ADR \
                         in _codev/decisions/"
                    ),
                },
                seal_relative.clone(),
            ),
        };
        findings.push(LocatedFinding::from_finding(finding, path));
    }

    // The report also carries the ADR parser's findings (missing
    // frontmatter, missing id, unknown status…), which we do not want to lose.
    for f in index.findings {
        // Parser findings carry no path — the decisions directory is
        // used as the default anchor.
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

/// Locates the `*.md` files under `changes/<name>/specs/**`.
fn locate_deltas(fs: &dyn FileSystem, change_dir: &std::path::Path) -> Result<Vec<PathBuf>> {
    let specs_dir = change_dir.join("specs");
    let files = fs
        .walk_files(&specs_dir)
        .map_err(|e| EngineError::Unreadable {
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

    fn project_with_well_formed_change() -> MemoryFileSystem {
        MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file(
                "/p/_codev/changes/add-auth/change.yaml",
                "schema: spec-driven",
            )
            .with_file(
                "/p/_codev/changes/add-auth/proposal.md",
                "# Proposal\n\n## Why\n\nTest\n",
            )
            .with_file(
                "/p/_codev/changes/add-auth/specs/user-auth/spec.md",
                "## Purpose\n\nHandles user authentication.\n\n## ADDED Requirements\n\n### Requirement: Login\nThe system SHALL issue a token.\n\n#### Scenario: OK\n- **WHEN** login\n- **THEN** token\n",
            )
    }

    #[test]
    fn change_report_covers_all_delta_files() {
        let fs = project_with_well_formed_change().with_file(
            "/p/_codev/changes/add-auth/specs/second/spec.md",
            "## Purpose\n\nSecond capability.\n\n## ADDED Requirements\n\n### Requirement: Second\nThe system SHALL do.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n",
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
        assert!(
            report.findings.is_empty(),
            "unexpected findings: {:?}",
            report.findings
        );
    }

    #[test]
    fn spec_report_exposes_parser_findings() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file(
                "/p/_codev/specs/user-auth/spec.md",
                "## Requirements\n\n### Requirement: X\nThe system SHALL x.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n",
            );
        let report = validate_spec(&fs, &Layout::new("/p"), "user-auth").unwrap();
        assert!(
            report
                .findings
                .iter()
                .any(|f| f.finding.code == parser_codes::SPEC_PURPOSE_MISSING)
        );
    }

    #[test]
    fn spec_report_exposes_registry_findings() {
        // A spec with a Purpose but no requirement: the registry's
        // SpecNoRequirement rule must fire.
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file(
                "/p/_codev/specs/user-auth/spec.md",
                "## Purpose\n\nManage users.\n\n## Requirements\n",
            );
        let report = validate_spec(&fs, &Layout::new("/p"), "user-auth").unwrap();
        assert!(
            report
                .findings
                .iter()
                .any(|f| f.finding.code == rule_codes::SPEC_NO_REQUIREMENT)
        );
    }

    #[test]
    fn validate_all_covers_changes_and_specs() {
        let fs = project_with_well_formed_change().with_file(
            "/p/_codev/specs/user-auth/spec.md",
            "## Purpose\n\nExisting main spec.\n\n## Requirements\n\n### Requirement: Old\nThe system SHALL persist.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n",
        );
        let cfg = config(&fs);
        let report = validate_all(&fs, &env(), &Layout::new("/p"), &cfg).unwrap();

        // 1 change + 1 main spec + the decisions report (always
        // present, even without ADRs).
        assert_eq!(report.items.len(), 3);
        assert!(
            report
                .items
                .iter()
                .any(|i| i.kind == ItemKind::Change && i.name == "add-auth")
        );
        assert!(
            report
                .items
                .iter()
                .any(|i| i.kind == ItemKind::Spec && i.name == "user-auth")
        );
        assert!(!report.has_errors());
    }

    // ─────────────── validate_decisions ───────────────

    fn adr_source(id: &str) -> String {
        format!(
            "---\nid: \"{id}\"\ntitle: T\nstatus: accepted\ndate: 2026-09-09\n---\n\n## Context\n\nx\n"
        )
    }

    #[test]
    fn decisions_without_seal_report_unsealed() {
        // Typical migration: 3 accepted ADRs, no seal.yaml.
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
        // Warnings, not errors — the migration does not block other flows.
        assert!(!report.has_errors());
    }

    #[test]
    fn body_modified_after_sealing_reports_mismatch_as_error() {
        let adr = adr_source("0001");
        let hash = seal::body_hash(&adr).unwrap();
        let seal_yaml = format!(
            "version: 1\nseals:\n  - id: \"0001\"\n    bodySha256: \"{hash}\"\n    sealedAt: 2026-09-09\n"
        );
        // Corrupt the body after sealing.
        let modified = adr.replace("\n\nx\n", "\n\nx modified\n");
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file("/p/_codev/decisions/0001-a.md", &modified)
            .with_file("/p/_codev/decisions/seal.yaml", &seal_yaml);
        let cfg = config(&fs);
        let report = validate_decisions(&fs, &env(), &Layout::new("/p"), &cfg).unwrap();
        assert!(
            report
                .findings
                .iter()
                .any(|f| f.finding.code == seal_codes::DECISION_SEAL_MISMATCH)
        );
        // It is indeed an error: the decisions index can no longer be
        // considered reliable.
        assert!(report.has_errors());
    }

    #[test]
    fn orphan_seal_reports_warning() {
        // A seal for 0009, but the ADR no longer exists.
        let seal_yaml = "version: 1\nseals:\n  - id: \"0009\"\n    bodySha256: \"sha256:whatever\"\n    sealedAt: 2026-09-09\n";
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file("/p/_codev/decisions/seal.yaml", seal_yaml);
        let cfg = config(&fs);
        let report = validate_decisions(&fs, &env(), &Layout::new("/p"), &cfg).unwrap();
        assert!(
            report
                .findings
                .iter()
                .any(|f| f.finding.code == seal_codes::DECISION_ORPHAN_SEAL)
        );
        assert!(!report.has_errors());
    }

    #[test]
    fn validate_reports_dangling_deviation_as_warning() {
        // A local ADR that `deviates_from` a target that does not exist →
        // warning, zero exit code.
        let adr = "---\nid: \"0007\"\ntitle: T\nstatus: accepted\ndate: 2026-09-09\ndeviates_from: [\"path:~/unknown/9999\"]\n---\n\n## Context\n\nx\n";
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
        assert!(
            report
                .findings
                .iter()
                .any(|f| f.finding.code == codev_core::parser::codes::DECISION_DANGLING_DEVIATION)
        );
        assert!(!report.has_errors(), "dangling deviation stays a warning");
    }

    #[test]
    fn validate_reports_conflicting_deviations_as_error() {
        // Two local ADRs deviating from the same target → error.
        let adr_a = "---\nid: \"0007\"\ntitle: A\nstatus: accepted\ndate: 2026-09-09\ndeviates_from: [\"path:~/shared/0100\"]\n---\n\n## Context\n\na\n";
        let adr_b = "---\nid: \"0008\"\ntitle: B\nstatus: accepted\ndate: 2026-09-09\ndeviates_from: [\"path:~/shared/0100\"]\n---\n\n## Context\n\nb\n";
        let adr_h = "---\nid: \"0100\"\ntitle: Source\nstatus: accepted\ndate: 2026-09-09\n---\n\n## Context\n\ns\n";
        let hash_a = seal::body_hash(adr_a).unwrap();
        let hash_b = seal::body_hash(adr_b).unwrap();
        let seal_yaml = format!(
            "version: 1\nseals:\n  - id: \"0007\"\n    bodySha256: \"{hash_a}\"\n    sealedAt: 2026-09-09\n  - id: \"0008\"\n    bodySha256: \"{hash_b}\"\n    sealedAt: 2026-09-09\n"
        );
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "inherits:\n  - path: ~/shared\n")
            .with_file("/p/_codev/decisions/0007-a.md", adr_a)
            .with_file("/p/_codev/decisions/0008-b.md", adr_b)
            .with_file("/home/shared/_codev/decisions/0100.md", adr_h)
            .with_file("/p/_codev/decisions/seal.yaml", &seal_yaml);
        let cfg = config(&fs);
        let report = validate_decisions(&fs, &env(), &Layout::new("/p"), &cfg).unwrap();
        assert!(
            report
                .findings
                .iter()
                .any(|f| f.finding.code
                    == codev_core::parser::codes::DECISION_CONFLICTING_DEVIATIONS)
        );
        assert!(report.has_errors(), "conflict must be an error");
    }

    #[test]
    fn inherited_adr_is_not_verified_by_the_project() {
        // A sealed local ADR (OK) + an inherited ADR (never sealed by
        // the consumer) → no finding for the inherited one.
        let local = adr_source("0001");
        let hash = seal::body_hash(&local).unwrap();
        let seal_yaml = format!(
            "version: 1\nseals:\n  - id: \"0001\"\n    bodySha256: \"{hash}\"\n    sealedAt: 2026-09-09\n"
        );
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "inherits:\n  - path: ~/shared\n")
            .with_file("/p/_codev/decisions/0001-a.md", &local)
            .with_file("/p/_codev/decisions/seal.yaml", &seal_yaml)
            .with_file(
                "/home/shared/_codev/decisions/0100-h.md",
                adr_source("0100"),
            );
        let cfg = config(&fs);
        let report = validate_decisions(&fs, &env(), &Layout::new("/p"), &cfg).unwrap();
        // No finding for 0100 — sealing is up to the source project.
        assert!(report.findings.is_empty(), "{:#?}", report.findings);
    }
}
