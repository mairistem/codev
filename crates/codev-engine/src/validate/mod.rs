//! Validation orchestration: reads the files, calls the parser and the rule
//! registry, produces a report.
//!
//! No rule logic here — the rules are pure and live in
//! `codev-core::validate`. We coordinate, group, and decide an exit code.
//! This is the shell in the sense of the decision
//! `_codev/decisions/0001-functional-core-imperative-shell.md`.

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
    findings.extend(check_change_roles(fs, layout, &ctx));
    findings.extend(check_proposal(fs, layout, &change_dir)?);
    // The record's defects only: an `open` finding is the normal state of
    // a change under review, and the archive gate is `archive`'s business.
    let review = crate::review::check(fs, layout, config, &ctx)?;
    findings.extend(
        review
            .findings
            .into_iter()
            .map(|finding| LocatedFinding::from_finding(finding, review.path.clone())),
    );
    let corrections = crate::corrections::check(fs, layout, change_id)?;
    findings.extend(
        corrections
            .findings
            .into_iter()
            .map(|finding| LocatedFinding::from_finding(finding, corrections.path.clone())),
    );

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
        let main_spec = read_main_spec(fs, layout, &change_dir, &delta_path)?;

        for f in parsed
            .findings
            .iter()
            .chain(core_validate::check_delta(&parsed.value).iter())
            .chain(
                core_validate::check_delta_against_spec(&parsed.value, main_spec.as_ref()).iter(),
            )
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
    if let Some(item) = validate_config(fs, layout, config) {
        items.push(item);
    }
    Ok(ValidateReport {
        root: layout.project_root().to_path_buf(),
        items,
    })
}

/// The findings of `_codev/config.yaml`: one warning per key this codev
/// ignores, one error per invalid entry of `roles:`, each at its line, and
/// the defects of `triggers:` at its key;
/// `None` when there is none, so a clean project's report does not change.
pub fn validate_config(
    fs: &dyn FileSystem,
    layout: &Layout,
    config: &ResolvedConfig,
) -> Option<ItemReport> {
    let path = layout.config_file();
    let relative = path
        .strip_prefix(layout.project_root())
        .unwrap_or(&path)
        .to_path_buf();
    let unknown_keys = config.unknown_keys.iter().map(|unknown| Finding {
        severity: Severity::Warning,
        code: crate::config::UNKNOWN_CONFIG_KEY,
        line: line_number(unknown.line),
        message: crate::config::unknown_key_message(&unknown.key, config.codev_version.as_deref()),
    });
    let roles = crate::roles::project_findings(fs, layout, config)
        .into_iter()
        .map(|finding| {
            let line = config.role_lines.get(&finding.role).copied().unwrap_or(1);
            Finding::error(finding.code, line_number(line), finding.message)
        });
    let triggers_line = line_number(config.triggers_line);
    let triggers = codev_core::triggers::read_groups(config.triggers.as_ref())
        .problems
        .into_iter()
        .map(|problem| {
            let message = format!("line {triggers_line}: {}", problem.message);
            if problem.is_error {
                Finding::error(
                    codev_core::triggers::codes::INVALID_TRIGGER,
                    triggers_line,
                    message,
                )
            } else {
                Finding::warning(
                    codev_core::triggers::codes::INVALID_TRIGGER,
                    triggers_line,
                    message,
                )
            }
        });
    let findings: Vec<LocatedFinding> = unknown_keys
        .chain(roles)
        .chain(triggers)
        .map(|finding| LocatedFinding::from_finding(finding, relative.clone()))
        .collect();
    if findings.is_empty() {
        return None;
    }
    Some(ItemReport {
        kind: ItemKind::Config,
        name: "config".into(),
        path: relative,
        findings,
    })
}

/// The invalid entries of a change's `roles:`, as errors at the line of
/// their key in `change.yaml`.
fn check_change_roles(
    fs: &dyn FileSystem,
    layout: &Layout,
    ctx: &change::ChangeContext,
) -> Vec<LocatedFinding> {
    let path = layout.change_metadata(&ctx.change);
    let relative = path
        .strip_prefix(layout.project_root())
        .unwrap_or(&path)
        .to_path_buf();
    let raw = fs.read_to_string(&path).unwrap_or_default();
    crate::roles::change_findings(ctx)
        .into_iter()
        .map(|finding| {
            let line = crate::config::key_line(&raw, Some("roles"), &finding.role);
            LocatedFinding::from_finding(
                Finding::error(finding.code, line_number(line), finding.message),
                relative.clone(),
            )
        })
        .collect()
}

fn line_number(line: usize) -> u32 {
    u32::try_from(line).unwrap_or(u32::MAX)
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

/// The release note and the domains of an active change's proposal: a
/// warning when a section is missing, the parsers' errors otherwise. A
/// change whose proposal is not written yet has nothing to check. Archived
/// changes never get here — history is not rewritten.
fn check_proposal(
    fs: &dyn FileSystem,
    layout: &Layout,
    change_dir: &std::path::Path,
) -> Result<Vec<LocatedFinding>> {
    use codev_core::changelog::{ReleaseNote, codes, release_note};
    use codev_core::domains::{self, Domains};
    let path = change_dir.join("proposal.md");
    if !fs.exists(&path) {
        return Ok(Vec::new());
    }
    let source = fs
        .read_to_string(&path)
        .map_err(|e| EngineError::Unreadable {
            path: path.clone(),
            reason: e.to_string(),
        })?;
    let relative = path
        .strip_prefix(layout.project_root())
        .unwrap_or(&path)
        .to_path_buf();
    let parsed = release_note::parse(&source);
    let mut findings = parsed.findings;
    if parsed.value == ReleaseNote::Absent {
        findings.push(Finding::warning(
            codes::RELEASE_NOTE_MISSING,
            1,
            "the proposal has no `## Release note` section; add one after `## Impact` \
             (`- <Category>: <text>` items, or `None` for an internal change)",
        ));
    }
    let domains = domains::parse(&source);
    findings.extend(domains.findings);
    if domains.value == Domains::Absent {
        findings.push(Finding::warning(
            domains::codes::DOMAINS_MISSING,
            1,
            "the proposal has no `## Domains` section; add one after `## Impact` \
             (tick the domains the change touches among `security`, `personal-data`, \
             `user-interface`, `performance`, `operations`, `migration`, or write `None`)",
        ));
    }
    Ok(findings
        .into_iter()
        .map(|f| LocatedFinding::from_finding(f, relative.clone()))
        .collect())
}

/// The parsed main spec a delta targets, or `None` for a new capability.
fn read_main_spec(
    fs: &dyn FileSystem,
    layout: &Layout,
    change_dir: &std::path::Path,
    delta_path: &std::path::Path,
) -> Result<Option<codev_core::parser::ast::Spec>> {
    let Some(capability) = crate::sync::capability_from_delta_path(change_dir, delta_path) else {
        return Ok(None);
    };
    let path = layout.spec_file(&capability);
    if !fs.exists(&path) {
        return Ok(None);
    }
    let source = fs
        .read_to_string(&path)
        .map_err(|e| EngineError::Unreadable {
            path: path.clone(),
            reason: e.to_string(),
        })?;
    Ok(Some(parse_spec(&source).value))
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

    #[test]
    fn validate_all_reports_unknown_config_keys() {
        let fs = project_with_well_formed_change().with_file(
            "/p/_codev/config.yaml",
            "schema: spec-driven\nlanguage: en\nreview_policy: strict\n",
        );
        let report = validate_all(&fs, &env(), &Layout::new("/p"), &config(&fs)).unwrap();
        let item = report
            .items
            .iter()
            .find(|i| i.kind == ItemKind::Config)
            .expect("a config item");
        assert_eq!(item.name, "config");
        assert_eq!(
            item.path,
            std::path::Path::new("_codev").join("config.yaml")
        );
        assert_eq!(item.findings.len(), 1);
        let finding = &item.findings[0].finding;
        assert_eq!(finding.code, "unknown_config_key");
        assert_eq!(finding.severity, Severity::Warning);
        assert_eq!(finding.line, 3);
        assert!(
            finding.message.contains("`review_policy`"),
            "{}",
            finding.message
        );
        assert!(report.has_warnings());
        assert!(!report.has_errors());
    }

    #[test]
    fn validate_reports_an_unknown_role_under_config() {
        let fs = MemoryFileSystem::new().with_file(
            "/p/_codev/config.yaml",
            "language: en\nroles:\n  reviewer: never\n",
        );
        let report = validate_all(&fs, &env(), &Layout::new("/p"), &config(&fs)).unwrap();
        let item = report
            .items
            .iter()
            .find(|i| i.kind == ItemKind::Config)
            .expect("a config item");
        let finding = &item.findings[0];
        assert_eq!(finding.finding.code, "unknown_role");
        assert_eq!(finding.finding.severity, Severity::Error);
        assert_eq!(finding.finding.line, 3);
        assert!(finding.finding.message.contains("reviewer"));
    }

    #[test]
    fn validate_reports_an_invalid_role_setting_of_a_change() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file(
                "/p/_codev/changes/add-auth/change.yaml",
                "schema: spec-driven\nskip_specs: true\nroles:\n  developer: never\n",
            );
        let cfg = config(&fs);
        let item = validate_change(
            &fs,
            &Layout::new("/p"),
            &cfg,
            &ChangeId::parse("add-auth").unwrap(),
        )
        .unwrap();
        let finding = item
            .findings
            .iter()
            .find(|f| f.finding.code == "role_not_disableable")
            .expect("a role finding");
        assert_eq!(finding.finding.line, 4);
        assert!(finding.finding.message.contains("developer"));
        assert!(finding.finding.message.contains("change.yaml"));
    }

    #[test]
    fn invalid_project_triggers_are_errors_at_their_key() {
        let fs = MemoryFileSystem::new().with_file(
            "/p/_codev/config.yaml",
            "schema: spec-driven\ntriggers:\n  - domain: secrets\n    paths: [x]\n  - domain: security\n    patterns: ['(unclosed']\n  - domain: security\n    paths: [y]\n    exclude: [z]\n",
        );
        let report = validate_all(&fs, &env(), &Layout::new("/p"), &config(&fs)).unwrap();
        let item = report.items.iter().find(|i| i.name == "config").unwrap();
        let found: Vec<(&str, bool, u32)> = item
            .findings
            .iter()
            .map(|f| (f.finding.code, f.is_error(), f.finding.line))
            .collect();
        assert_eq!(
            found,
            [
                ("invalid_trigger", true, 2),
                ("invalid_trigger", true, 2),
                ("invalid_trigger", false, 2),
            ]
        );
        assert!(item.findings[0].finding.message.contains("secrets"));
        assert!(item.findings[1].finding.message.contains("(unclosed"));
        assert!(item.findings[2].finding.message.contains("exclude"));
    }

    #[test]
    fn validate_all_has_no_config_item_for_a_clean_config() {
        let fs = project_with_well_formed_change();
        let report = validate_all(&fs, &env(), &Layout::new("/p"), &config(&fs)).unwrap();
        assert!(report.items.iter().all(|i| i.kind != ItemKind::Config));
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
                "# Proposal\n\n## Why\n\nTest\n\n## Domains\n\nNone\n\n## Release note\n\n- Added: login.\n",
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
    fn validate_reads_a_french_change_in_an_english_project() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "language: en\n")
            .with_file("/p/_codev/changes/conges/change.yaml", "schema: spec-driven")
            .with_file(
                "/p/_codev/changes/conges/proposal.md",
                "# Proposition : annulation\n\n## Pourquoi\n\nx\n\n## Domaines\n\n- [x] migration\n\n## Note de version\n\n- Ajouté : l'annulation d'une demande.\n",
            )
            .with_file(
                "/p/_codev/changes/conges/specs/conges/spec.md",
                "## Objet\n\nGérer les congés.\n\n## Exigences AJOUTÉES\n\n### Exigence : Annulation\nLe portail DOIT permettre d'annuler une demande en attente.\n\n#### Scénario : Dans les temps\n- **WHEN** l'agent annule\n- **THEN** la demande est annulée\n",
            );
        let cfg = config(&fs);
        let report = validate_change(
            &fs,
            &Layout::new("/p"),
            &cfg,
            &ChangeId::parse("conges").unwrap(),
        )
        .unwrap();
        assert!(
            report.findings.is_empty(),
            "unexpected findings: {:?}",
            report.findings
        );
    }

    #[test]
    fn change_report_flags_a_rename_of_an_unknown_requirement() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file("/p/_codev/changes/ren/change.yaml", "schema: spec-driven")
            .with_file(
                "/p/_codev/specs/user-auth/spec.md",
                "## Purpose\n\nAuth.\n\n## Requirements\n\n### Requirement: Login\nThe system SHALL x.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n",
            )
            .with_file(
                "/p/_codev/changes/ren/specs/user-auth/spec.md",
                "## RENAMED Requirements\n\n- FROM: `### Requirement: Log in`\n- TO: `### Requirement: Sign in`\n",
            );
        let cfg = config(&fs);
        let report = validate_change(
            &fs,
            &Layout::new("/p"),
            &cfg,
            &ChangeId::parse("ren").unwrap(),
        )
        .unwrap();
        assert!(
            report
                .findings
                .iter()
                .any(|f| f.finding.code == rule_codes::RENAME_SOURCE_MISSING),
            "{:?}",
            report.findings
        );
        assert!(report.has_errors());
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

    fn release_note_codes(note: Option<&str>) -> Vec<(&'static str, Severity)> {
        let mut proposal = String::from("# Proposal\n\n## Why\n\nTest\n");
        if let Some(note) = note {
            proposal.push_str(&format!("\n## Release note\n\n{note}\n"));
        }
        let fs = project_with_well_formed_change()
            .with_file("/p/_codev/changes/add-auth/proposal.md", proposal);
        let cfg = config(&fs);
        let report = validate_change(
            &fs,
            &Layout::new("/p"),
            &cfg,
            &ChangeId::parse("add-auth").unwrap(),
        )
        .unwrap();
        report
            .findings
            .iter()
            .filter(|f| f.finding.code.starts_with("release_note"))
            .map(|f| (f.finding.code, f.finding.severity))
            .collect()
    }

    #[test]
    fn validate_reports_a_missing_release_note_as_a_warning() {
        assert_eq!(
            release_note_codes(None),
            vec![("release_note_missing", Severity::Warning)]
        );
    }

    #[test]
    fn validate_accepts_a_valid_or_internal_release_note() {
        assert!(
            release_note_codes(Some("- Added: a flag.\n- Changed: **BREAKING** a default."))
                .is_empty()
        );
        assert!(release_note_codes(Some("None")).is_empty());
    }

    #[test]
    fn validate_reports_an_unknown_category_and_an_empty_bullet() {
        for note in ["- Improved: speed.", "- Fixed:", ""] {
            assert_eq!(
                release_note_codes(Some(note)),
                vec![("release_note_invalid", Severity::Error)],
                "{note:?}"
            );
        }
    }

    #[test]
    fn validate_ignores_archived_changes_without_release_note() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file(
                "/p/_codev/changes/archive/2026-01-01-old/proposal.md",
                "# Proposal\n\n## Why\n\nOld.\n",
            )
            .with_file(
                "/p/_codev/changes/archive/2026-01-01-old/change.yaml",
                "schema: spec-driven",
            );
        let cfg = config(&fs);
        let report = validate_all(&fs, &env(), &Layout::new("/p"), &cfg).unwrap();
        assert!(
            report
                .items
                .iter()
                .flat_map(|i| &i.findings)
                .all(|f| !f.finding.code.starts_with("release_note")),
            "{:?}",
            report.items
        );
    }

    fn domain_codes(domains: Option<&str>) -> Vec<(&'static str, Severity)> {
        let mut proposal = String::from("# Proposal\n\n## Why\n\nTest\n");
        if let Some(domains) = domains {
            proposal.push_str(&format!("\n## Domains\n\n{domains}\n"));
        }
        proposal.push_str("\n## Release note\n\nNone\n");
        let fs = project_with_well_formed_change()
            .with_file("/p/_codev/changes/add-auth/proposal.md", proposal);
        let cfg = config(&fs);
        let report = validate_change(
            &fs,
            &Layout::new("/p"),
            &cfg,
            &ChangeId::parse("add-auth").unwrap(),
        )
        .unwrap();
        report
            .findings
            .iter()
            .filter(|f| f.finding.code.contains("domain"))
            .map(|f| (f.finding.code, f.finding.severity))
            .collect()
    }

    #[test]
    fn validate_accepts_ticked_domains_and_none() {
        assert!(
            domain_codes(Some(
                "- [x] security\n- [ ] performance\n- personal-data — new e-mail field"
            ))
            .is_empty()
        );
        assert!(domain_codes(Some("None")).is_empty());
    }

    #[test]
    fn validate_reports_an_unknown_domain_as_an_error() {
        assert_eq!(
            domain_codes(Some("- [x] securty")),
            vec![("unknown_domain", Severity::Error)]
        );
    }

    #[test]
    fn validate_reports_none_with_a_ticked_domain() {
        assert_eq!(
            domain_codes(Some("None\n- [x] security")),
            vec![("domains_invalid", Severity::Error)]
        );
    }

    #[test]
    fn validate_reports_a_missing_domains_section_as_a_warning() {
        assert_eq!(
            domain_codes(None),
            vec![("domains_missing", Severity::Warning)]
        );
    }

    #[test]
    fn validate_ignores_archived_changes_without_domains() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file(
                "/p/_codev/changes/archive/2026-01-01-old/proposal.md",
                "# Proposal\n\n## Why\n\nOld.\n",
            )
            .with_file(
                "/p/_codev/changes/archive/2026-01-01-old/change.yaml",
                "schema: spec-driven",
            );
        let cfg = config(&fs);
        let report = validate_all(&fs, &env(), &Layout::new("/p"), &cfg).unwrap();
        assert!(
            report
                .items
                .iter()
                .flat_map(|i| &i.findings)
                .all(|f| !f.finding.code.contains("domain")),
            "{:?}",
            report.items
        );
    }

    // ─────────────── review record ───────────────

    fn review_codes(fs: &MemoryFileSystem) -> (Vec<&'static str>, ItemReport) {
        let cfg = config(fs);
        let report = validate_change(
            fs,
            &Layout::new("/p"),
            &cfg,
            &ChangeId::parse("add-auth").unwrap(),
        )
        .unwrap();
        let codes = report
            .findings
            .iter()
            .map(|f| f.finding.code)
            .filter(|code| code.starts_with("review_"))
            .collect();
        (codes, report)
    }

    fn with_review(answers_and_fields: &str) -> MemoryFileSystem {
        project_with_well_formed_change()
            .with_file(
                "/p/src/auth.rs",
                "fn login() {\n    let token = random();\n}\n",
            )
            .with_file(
                "/p/_codev/changes/add-auth/review.md",
                crate::review::tests::record(&crate::review::tests::finding(
                    "newcomer",
                    "high",
                    "src/auth.rs:2",
                    "let token = random();",
                    answers_and_fields,
                )),
            )
    }

    #[test]
    fn validate_reports_nothing_without_a_record() {
        let (codes, _) = review_codes(&project_with_well_formed_change());
        assert!(codes.is_empty(), "{codes:?}");
    }

    #[test]
    fn an_open_high_finding_is_not_a_validation_error() {
        let (codes, report) = review_codes(&with_review(""));
        assert!(codes.is_empty(), "{:?}", report.findings);
        assert!(!report.has_errors());
    }

    #[test]
    fn a_record_error_is_located_in_review_md() {
        let fs = with_review("");
        let broken = fs
            .read("/p/_codev/changes/add-auth/review.md")
            .unwrap()
            .replace("evidence: |\n  let token = random();\n", "");
        let fs = fs.with_file("/p/_codev/changes/add-auth/review.md", broken);
        let (codes, report) = review_codes(&fs);
        assert_eq!(codes, ["review_invalid"]);
        assert!(report.has_errors());
        let finding = report
            .findings
            .iter()
            .find(|f| f.finding.code == "review_invalid")
            .unwrap();
        assert_eq!(
            finding.path,
            PathBuf::from("_codev/changes/add-auth/review.md")
        );
        assert!(finding.finding.line > 1);
    }

    #[test]
    fn archived_records_are_not_checked() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file(
                "/p/_codev/changes/archive/2026-01-01-old/change.yaml",
                "schema: spec-driven",
            )
            .with_file(
                "/p/_codev/changes/archive/2026-01-01-old/review.md",
                "## code.review\n\n### newcomer\n\nnothing usable\n",
            );
        let cfg = config(&fs);
        let report = validate_all(&fs, &env(), &Layout::new("/p"), &cfg).unwrap();
        assert!(
            report
                .items
                .iter()
                .flat_map(|i| &i.findings)
                .all(|f| !f.finding.code.starts_with("review_")),
            "{:?}",
            report.items
        );
    }

    // ─────────────── corrections ───────────────

    const CORRECTION: &str = "<!-- codev-corrections: 1 -->\n\n### C-1 Typed errors\n\n```yaml\nstep: apply\nwhere: src/auth.rs\ndid: Returned anyhow.\nasked: Use thiserror.\n```\n";

    fn corrections_codes(fs: &MemoryFileSystem) -> (Vec<&'static str>, ItemReport) {
        let cfg = config(fs);
        let report = validate_change(
            fs,
            &Layout::new("/p"),
            &cfg,
            &ChangeId::parse("add-auth").unwrap(),
        )
        .unwrap();
        let codes = report
            .findings
            .iter()
            .map(|f| f.finding.code)
            .filter(|code| code.starts_with("corrections_"))
            .collect();
        (codes, report)
    }

    #[test]
    fn a_change_without_corrections_has_no_finding() {
        let (codes, _) = corrections_codes(&project_with_well_formed_change());
        assert!(codes.is_empty(), "{codes:?}");
    }

    #[test]
    fn well_formed_corrections_have_no_finding() {
        let fs = project_with_well_formed_change()
            .with_file("/p/_codev/changes/add-auth/corrections.md", CORRECTION);
        let (codes, _) = corrections_codes(&fs);
        assert!(codes.is_empty(), "{codes:?}");
    }

    #[test]
    fn a_corrections_error_fails_validation() {
        let fs = project_with_well_formed_change().with_file(
            "/p/_codev/changes/add-auth/corrections.md",
            CORRECTION.replace("did: Returned anyhow.\n", ""),
        );
        let (codes, report) = corrections_codes(&fs);
        assert_eq!(codes, ["corrections_invalid"]);
        assert!(report.has_errors());
        let finding = report
            .findings
            .iter()
            .find(|f| f.finding.code == "corrections_invalid")
            .unwrap();
        assert_eq!(
            finding.path,
            PathBuf::from("_codev/changes/add-auth/corrections.md")
        );
        assert_eq!(finding.finding.line, 5);
    }

    #[test]
    fn archived_corrections_are_not_checked() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file(
                "/p/_codev/changes/archive/2026-01-01-old/change.yaml",
                "schema: spec-driven",
            )
            .with_file(
                "/p/_codev/changes/archive/2026-01-01-old/corrections.md",
                "<!-- codev-corrections: 1 -->\n\n### C-1 Broken\n",
            );
        let cfg = config(&fs);
        let report = validate_all(&fs, &env(), &Layout::new("/p"), &cfg).unwrap();
        assert!(
            report
                .items
                .iter()
                .flat_map(|i| &i.findings)
                .all(|f| !f.finding.code.starts_with("corrections_")),
            "{:?}",
            report.items
        );
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
