//! Index of architecture decisions — project + inherited sources.
//!
//! The pure parser (`codev-core::decisions`) turns a file into a
//! `Decision`. Here we coordinate reading from disk, merging with the
//! `path:` inherited sources, and computing the decisions "in effect" —
//! those not superseded by any other.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use codev_core::Layout;
use codev_core::decisions::{Decision, DecisionStatus, parse_decision};
use codev_core::parser::ast::{Finding, Severity};
use codev_core::parser::codes;

use crate::config::ResolvedConfig;
use crate::error::{EngineError, Result};
use crate::ports::{Env, FileSystem};

/// The origin of a decision — project or inherited source.
///
/// The format of `git:` inherited sources is reserved — the current batch
/// only handles `path:`. A new variant will follow once `git:` is
/// implemented.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Origin {
    Project,
    Path(String),
    /// URL of a `git:` repository — the content comes from the local cache,
    /// but the `origin` carries the original address so it stays readable
    /// for the agent.
    Git(String),
}

impl Origin {
    pub fn as_str(&self) -> String {
        match self {
            Self::Project => "project".into(),
            Self::Path(raw) => format!("path:{raw}"),
            Self::Git(url) => format!("git:{url}"),
        }
    }
}

/// A qualified identifier — avoids collisions across sources.
///
/// The serialized form `"project/0007"` or `"path:~/shared/0100"` is
/// exposed as-is in the JSON contract.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct QualifiedId {
    pub origin: Origin,
    pub id: String,
}

impl QualifiedId {
    pub fn as_str(&self) -> String {
        format!("{}/{}", self.origin.as_str(), self.id)
    }
}

/// An index entry.
#[derive(Debug, Clone)]
pub struct IndexEntry {
    pub qualified_id: QualifiedId,
    pub decision: Decision,
    /// Absolute path of the file, on disk as it was read.
    pub path: PathBuf,
    /// If this decision is inherited and set aside by a local ADR via
    /// `deviates_from`, the qualified identifier of the ADR that replaces it.
    /// Computed by the index, never persisted to disk.
    pub deviated_by: Option<QualifiedId>,
}

/// The full index.
#[derive(Debug, Clone, Default)]
pub struct DecisionIndex {
    pub entries: Vec<IndexEntry>,
    /// Subset of `entries` that remain in effect: `Accepted` and not
    /// superseded by another `Accepted` decision.
    pub in_effect: Vec<QualifiedId>,
    pub findings: Vec<Finding>,
}

/// Builds the index from the project and its inherited sources.
///
/// `env` is needed to expand `~` in the paths of `inherits: path:`
/// sources — the port used must be the same as the one passed to
/// `config::resolve`, otherwise the resolution warnings and the indexed
/// decisions diverge.
pub fn index(
    fs: &dyn FileSystem,
    env: &dyn Env,
    layout: &Layout,
    config: &ResolvedConfig,
) -> Result<DecisionIndex> {
    let mut entries: Vec<IndexEntry> = Vec::new();
    let mut findings: Vec<Finding> = Vec::new();

    // 1. Project decisions.
    let project_dir = layout.decisions_dir();
    collect_from(
        fs,
        &project_dir,
        Origin::Project,
        &mut entries,
        &mut findings,
    )?;

    // 2. Decisions from inherited sources — path AND locked git.
    let states = crate::sources::list_source_states(fs, env, layout)?;
    for state in states {
        let Some(project_root) = state.project_root() else {
            continue;
        };
        let origin = match state.kind {
            crate::sources::SourceKind::Path => Origin::Path(state.address.clone()),
            crate::sources::SourceKind::Git => Origin::Git(state.address.clone()),
        };
        let decisions_dir = Layout::new(&project_root).decisions_dir();
        collect_from(fs, &decisions_dir, origin, &mut entries, &mut findings)?;
    }
    let _ = config; // kept for signature compatibility

    // 3. Id collision detection — the project wins, the inherited one is
    //    reported.
    detect_id_collisions(&mut entries, &mut findings);

    // 4. Local deviation resolution — every inherited decision referenced
    //    by a local `accepted` ADR via `deviates_from` receives its
    //    `deviated_by`, and will drop out of the `in_effect` computation.
    resolve_deviations(&mut entries, &mut findings);

    // 5. Supersession resolution and `in_effect` computation.
    let in_effect = resolve_in_effect(&entries, &mut findings);

    Ok(DecisionIndex {
        entries,
        in_effect,
        findings,
    })
}

/// Opens every `*.md` in the `dir` directory, calls `parse_decision`, aggregates.
fn collect_from(
    fs: &dyn FileSystem,
    dir: &std::path::Path,
    origin: Origin,
    entries: &mut Vec<IndexEntry>,
    findings: &mut Vec<Finding>,
) -> Result<()> {
    let files = fs.walk_files(dir).map_err(|e| EngineError::Unreadable {
        path: dir.to_path_buf(),
        reason: e.to_string(),
    })?;

    for relative in files.into_iter().filter(|p| p.ends_with(".md")) {
        let path = dir.join(&relative);
        let source = fs
            .read_to_string(&path)
            .map_err(|e| EngineError::Unreadable {
                path: path.clone(),
                reason: e.to_string(),
            })?;
        let parsed = parse_decision(&source);
        findings.extend(parsed.findings);
        if let Some(decision) = parsed.value {
            entries.push(IndexEntry {
                qualified_id: QualifiedId {
                    origin: origin.clone(),
                    id: decision.id.clone(),
                },
                decision,
                path,
                deviated_by: None,
            });
        }
    }
    Ok(())
}

fn detect_id_collisions(entries: &mut [IndexEntry], findings: &mut Vec<Finding>) {
    // Group by bare `id` (not by qualified id). A collision is an id that
    // appears both on the `Project` side and on the `Path(_)` side.
    let mut by_id: BTreeMap<&str, Vec<usize>> = BTreeMap::new();
    for (i, entry) in entries.iter().enumerate() {
        by_id.entry(entry.decision.id.as_str()).or_default().push(i);
    }
    for (id, indices) in by_id {
        if indices.len() < 2 {
            continue;
        }
        let has_project = indices
            .iter()
            .any(|&i| entries[i].qualified_id.origin == Origin::Project);
        let has_source = indices
            .iter()
            .any(|&i| entries[i].qualified_id.origin != Origin::Project);
        if has_project && has_source {
            findings.push(Finding {
                severity: Severity::Warning,
                code: codes::DECISION_ID_COLLISION,
                line: 1,
                message: format!(
                    "identifier `{id}` exists both in the project and in an inherited \
                     source; the project version is the one kept in effect"
                ),
            });
        }
    }
}

/// Computes `deviated_by` on inherited entries, based on the
/// `deviates_from` field of local `accepted` ADRs.
///
/// Emits:
/// - `decision_dangling_deviation` (warning) when the target does not exist.
/// - `decision_conflicting_deviations` (error) when two local `accepted`
///   ADRs deviate from the same target.
///
/// Does nothing for a `deviates_from` carried by a non-`accepted` ADR
/// (a `proposed` one is not binding yet).
fn resolve_deviations(entries: &mut [IndexEntry], findings: &mut Vec<Finding>) {
    // Step 1: build the qualified → position-in-`entries` index.
    let by_qualified: BTreeMap<String, usize> = entries
        .iter()
        .enumerate()
        .map(|(i, e)| (e.qualified_id.as_str(), i))
        .collect();

    // Step 2: for every local `accepted` ADR, collect its targets.
    // `deviations_by_target[target_qualified] = list of <local ADR>s referencing it`.
    let mut deviations_by_target: BTreeMap<String, Vec<QualifiedId>> = BTreeMap::new();
    for entry in entries.iter() {
        if entry.qualified_id.origin != Origin::Project {
            continue;
        }
        if !matches!(entry.decision.status, DecisionStatus::Accepted) {
            continue;
        }
        for target in &entry.decision.deviates_from {
            deviations_by_target
                .entry(target.clone())
                .or_default()
                .push(entry.qualified_id.clone());
        }
    }

    // Step 3: resolve each target.
    for (target, sources) in deviations_by_target {
        // Target missing?
        let Some(&target_idx) = by_qualified.get(&target) else {
            // Every local ADR referencing it raises a warning.
            for source in &sources {
                findings.push(Finding {
                    severity: Severity::Warning,
                    code: codes::DECISION_DANGLING_DEVIATION,
                    line: 1,
                    message: format!(
                        "local decision `{source_qid}` deviates from \
                         `{target}`, which is not indexed (source removed, \
                         SHA moved, or id changed)",
                        source_qid = source.as_str()
                    ),
                });
            }
            continue;
        };

        // Local target? Normally rejected upstream by `plan_deviate`, but a
        // user might have written an ADR by hand. No dedicated finding —
        // the semantics are simply that deviating from a local decision has
        // no effect on the index (nothing is hidden).
        if entries[target_idx].qualified_id.origin == Origin::Project {
            continue;
        }

        // Two or more local ADRs deviating from the same target → conflict.
        if sources.len() > 1 {
            let names = sources
                .iter()
                .map(|q| q.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            findings.push(Finding {
                severity: Severity::Error,
                code: codes::DECISION_CONFLICTING_DEVIATIONS,
                line: 1,
                message: format!(
                    "local decisions {names} all deviate from `{target}` — \
                     the tool will not pick one; remove the extra ADRs or replace \
                     them with local supersessions"
                ),
            });
            // We still mark the target with the first one found (stable
            // `BTreeMap` order) so the inherited decision does not stay
            // in effect if the user ignores the finding.
        }

        // Mark the inherited target.
        entries[target_idx].deviated_by = Some(sources[0].clone());
    }
}

fn resolve_in_effect(entries: &[IndexEntry], findings: &mut Vec<Finding>) -> Vec<QualifiedId> {
    // Cycle detection: a DFS colored by state. Any decision caught in a
    // cycle is excluded from `in_effect`.
    let (cycle_members, cycle_finding) = detect_cycles(entries);
    if let Some(f) = cycle_finding {
        findings.push(f);
    }

    // For every non-project `Accepted` decision colliding with the
    // project, the project version wins — the inherited one is marked as
    // shadowed.
    let project_ids: BTreeSet<&str> = entries
        .iter()
        .filter(|e| e.qualified_id.origin == Origin::Project)
        .map(|e| e.decision.id.as_str())
        .collect();

    // Set of ids superseded by at least one Accepted decision.
    let mut supersedes_source = BTreeSet::new();
    for entry in entries.iter().filter(|e| {
        matches!(e.decision.status, DecisionStatus::Accepted)
            && !cycle_members.contains(&index_of(entries, e))
    }) {
        for target in &entry.decision.supersedes {
            if !entries.iter().any(|e| e.decision.id == *target) {
                findings.push(Finding {
                    severity: Severity::Warning,
                    code: codes::DECISION_SUPERSEDES_UNKNOWN,
                    line: 1,
                    message: format!(
                        "decision `{}` supersedes `{target}`, which does not exist in the index",
                        entry.decision.id
                    ),
                });
                continue;
            }
            supersedes_source.insert(target.clone());
        }
    }

    let mut in_effect = Vec::new();
    for (idx, entry) in entries.iter().enumerate() {
        // Candidate status?
        if !entry.decision.status.is_candidate_for_effect() {
            continue;
        }
        // In a cycle?
        if cycle_members.contains(&idx) {
            continue;
        }
        // Superseded?
        if supersedes_source.contains(entry.decision.id.as_str()) {
            continue;
        }
        // Shadowed by a project version?
        if entry.qualified_id.origin != Origin::Project
            && project_ids.contains(entry.decision.id.as_str())
        {
            continue;
        }
        // Deviated from locally?
        if entry.deviated_by.is_some() {
            continue;
        }
        in_effect.push(entry.qualified_id.clone());
    }
    in_effect
}

fn index_of(entries: &[IndexEntry], entry: &IndexEntry) -> usize {
    entries
        .iter()
        .position(|e| std::ptr::eq(e, entry))
        .expect("the entry comes from the same slice")
}

fn detect_cycles(entries: &[IndexEntry]) -> (BTreeSet<usize>, Option<Finding>) {
    // For cycles we only consider supersession chains between decisions
    // present in the index — a missing target means no cycle; that is
    // reported elsewhere.
    let by_id: BTreeMap<&str, usize> = entries
        .iter()
        .enumerate()
        .map(|(i, e)| (e.decision.id.as_str(), i))
        .collect();

    #[derive(Clone, Copy, PartialEq, Eq)]
    enum State {
        White,
        Gray,
        Black,
    }
    let mut state = vec![State::White; entries.len()];
    let mut members = BTreeSet::new();

    fn visit(
        node: usize,
        entries: &[IndexEntry],
        by_id: &BTreeMap<&str, usize>,
        state: &mut Vec<State>,
        stack: &mut Vec<usize>,
        members: &mut BTreeSet<usize>,
    ) {
        state[node] = State::Gray;
        stack.push(node);
        for target in &entries[node].decision.supersedes {
            let Some(&next) = by_id.get(target.as_str()) else {
                continue;
            };
            match state[next] {
                State::White => visit(next, entries, by_id, state, stack, members),
                State::Gray => {
                    // Cycle: everything on the stack from `next` onward is
                    // part of it.
                    let start = stack.iter().position(|&n| n == next).unwrap_or(0);
                    for &n in &stack[start..] {
                        members.insert(n);
                    }
                }
                State::Black => {}
            }
        }
        stack.pop();
        state[node] = State::Black;
    }

    for start in 0..entries.len() {
        if state[start] == State::White {
            visit(
                start,
                entries,
                &by_id,
                &mut state,
                &mut Vec::new(),
                &mut members,
            );
        }
    }

    let finding = if members.is_empty() {
        None
    } else {
        let names: Vec<String> = members
            .iter()
            .map(|&i| entries[i].decision.id.clone())
            .collect();
        Some(Finding {
            severity: Severity::Warning,
            code: codes::DECISION_SUPERSESSION_CYCLE,
            line: 1,
            message: format!(
                "supersession cycle detected: {} — none of these decisions takes effect",
                names.join(", ")
            ),
        })
    };
    (members, finding)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config;
    use crate::ports::{FixedEnv, MemoryFileSystem};

    fn env() -> FixedEnv {
        let mut env = FixedEnv::at("/p");
        env.vars.insert("HOME".into(), "/home".into());
        env
    }

    fn resolved(fs: &dyn FileSystem) -> ResolvedConfig {
        config::resolve(fs, &env(), &Layout::new("/p")).unwrap()
    }

    fn adr(id: &str, status: &str, supersedes: &[&str]) -> String {
        let sup = if supersedes.is_empty() {
            String::new()
        } else {
            format!(
                "supersedes: [{}]\n",
                supersedes
                    .iter()
                    .map(|s| format!("\"{s}\""))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        };
        format!(
            "---\nid: \"{id}\"\ntitle: \"ADR {id}\"\nstatus: {status}\ndate: 2026-09-08\n{sup}---\n\n## Context\n\nx\n"
        )
    }

    #[test]
    fn git_source_decisions_are_read_under_its_subpath() {
        // Same resolution as the inherited config: `<cache>/<subpath>/_codev`.
        let lock = "version = 1\n\n[[source]]\ngit = \"url\"\nref = \"main\"\nsubpath = \"standards\"\ncommit = \"deadbeef\"\nresolved_at = \"2026-09-08\"\n";
        let fs = MemoryFileSystem::new()
            .with_file(
                "/p/_codev/config.yaml",
                "inherits:\n  - git: url\n    ref: main\n    subpath: standards\n",
            )
            .with_file("/p/_codev/codev.lock", lock)
            .with_file(
                "/home/.cache/codev/content/deadbeef/standards/_codev/config.yaml",
                "",
            )
            .with_file(
                "/home/.cache/codev/content/deadbeef/standards/_codev/decisions/0100-shared.md",
                adr("0100", "accepted", &[]),
            )
            // Outside the subpath: not part of the source.
            .with_file(
                "/home/.cache/codev/content/deadbeef/_codev/decisions/0200-root.md",
                adr("0200", "accepted", &[]),
            );
        let cfg = resolved(&fs);
        let idx = index(&fs, &env(), &Layout::new("/p"), &cfg).unwrap();
        let ids: Vec<String> = idx
            .entries
            .iter()
            .map(|e| e.qualified_id.as_str())
            .collect();
        assert_eq!(ids, vec!["git:url/0100".to_string()], "{ids:?}");
    }

    #[test]
    fn empty_index_for_project_without_adrs() {
        let fs = MemoryFileSystem::new().with_file("/p/_codev/config.yaml", "");
        let cfg = resolved(&fs);
        let idx = index(&fs, &env(), &Layout::new("/p"), &cfg).unwrap();
        assert!(idx.entries.is_empty());
        assert!(idx.in_effect.is_empty());
        assert!(idx.findings.is_empty());
    }

    #[test]
    fn project_and_source_adrs_appear_with_their_origin() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "inherits:\n  - path: ~/shared\n")
            .with_file(
                "/p/_codev/decisions/0001-local.md",
                adr("0001", "accepted", &[]),
            )
            .with_file(
                "/home/shared/_codev/decisions/0100-shared.md",
                adr("0100", "accepted", &[]),
            );
        let cfg = resolved(&fs);
        let idx = index(&fs, &env(), &Layout::new("/p"), &cfg).unwrap();

        assert_eq!(idx.entries.len(), 2);
        assert!(
            idx.entries
                .iter()
                .any(|e| e.qualified_id.as_str() == "project/0001")
        );
        assert!(
            idx.entries
                .iter()
                .any(|e| e.qualified_id.as_str() == "path:~/shared/0100")
        );
    }

    #[test]
    fn direct_supersession_hides_the_superseded_decision() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file("/p/_codev/decisions/0003.md", adr("0003", "accepted", &[]))
            .with_file(
                "/p/_codev/decisions/0007.md",
                adr("0007", "accepted", &["0003"]),
            );
        let cfg = resolved(&fs);
        let idx = index(&fs, &env(), &Layout::new("/p"), &cfg).unwrap();

        let in_effect: Vec<String> = idx.in_effect.iter().map(|q| q.id.clone()).collect();
        assert_eq!(in_effect, ["0007"]);
    }

    #[test]
    fn three_link_chain_leaves_the_last_one() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file("/p/_codev/decisions/A.md", adr("A", "accepted", &[]))
            .with_file("/p/_codev/decisions/B.md", adr("B", "accepted", &["A"]))
            .with_file("/p/_codev/decisions/C.md", adr("C", "accepted", &["B"]));
        let cfg = resolved(&fs);
        let idx = index(&fs, &env(), &Layout::new("/p"), &cfg).unwrap();
        let in_effect: Vec<String> = idx.in_effect.iter().map(|q| q.id.clone()).collect();
        assert_eq!(in_effect, ["C"]);
    }

    #[test]
    fn supersedes_pointing_to_missing_decision_is_reported() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file(
                "/p/_codev/decisions/0007.md",
                adr("0007", "accepted", &["9999"]),
            );
        let cfg = resolved(&fs);
        let idx = index(&fs, &env(), &Layout::new("/p"), &cfg).unwrap();
        assert!(idx
            .findings
            .iter()
            .any(|f| f.code == codes::DECISION_SUPERSEDES_UNKNOWN && f.message.contains("9999")));
        // 0007 stays in effect — the lost target does not disqualify it.
        assert_eq!(
            idx.in_effect
                .iter()
                .map(|q| q.id.clone())
                .collect::<Vec<_>>(),
            ["0007"]
        );
    }

    #[test]
    fn project_source_collision_project_wins() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "inherits:\n  - path: ~/shared\n")
            .with_file("/p/_codev/decisions/0007.md", adr("0007", "accepted", &[]))
            .with_file(
                "/home/shared/_codev/decisions/0007-duplicate.md",
                adr("0007", "accepted", &[]),
            );
        let cfg = resolved(&fs);
        let idx = index(&fs, &env(), &Layout::new("/p"), &cfg).unwrap();

        assert!(
            idx.findings
                .iter()
                .any(|f| f.code == codes::DECISION_ID_COLLISION)
        );

        // A single entry in effect: the project's.
        assert_eq!(idx.in_effect.len(), 1);
        assert_eq!(idx.in_effect[0].origin, Origin::Project);
    }

    #[test]
    fn supersession_cycle_is_reported() {
        // A supersedes B, B supersedes A → cycle. Neither is in effect.
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file("/p/_codev/decisions/A.md", adr("A", "accepted", &["B"]))
            .with_file("/p/_codev/decisions/B.md", adr("B", "accepted", &["A"]));
        let cfg = resolved(&fs);
        let idx = index(&fs, &env(), &Layout::new("/p"), &cfg).unwrap();

        assert!(
            idx.findings
                .iter()
                .any(|f| f.code == codes::DECISION_SUPERSESSION_CYCLE)
        );
        assert!(idx.in_effect.is_empty());
    }

    #[test]
    fn proposed_status_does_not_take_effect() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file("/p/_codev/decisions/0001.md", adr("0001", "proposed", &[]));
        let cfg = resolved(&fs);
        let idx = index(&fs, &env(), &Layout::new("/p"), &cfg).unwrap();
        assert_eq!(idx.entries.len(), 1);
        assert!(idx.in_effect.is_empty());
    }

    // ─────────────── local deviations ───────────────

    fn adr_deviates(id: &str, status: &str, deviates_from: &[&str]) -> String {
        let dev = if deviates_from.is_empty() {
            String::new()
        } else {
            format!(
                "deviates_from: [{}]\n",
                deviates_from
                    .iter()
                    .map(|s| format!("\"{s}\""))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        };
        format!(
            "---\nid: \"{id}\"\ntitle: \"ADR {id}\"\nstatus: {status}\ndate: 2026-09-08\n{dev}---\n\n## Context\n\nx\n"
        )
    }

    #[test]
    fn deviation_marks_inherited_and_removes_it_from_in_effect() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "inherits:\n  - path: ~/shared\n")
            .with_file(
                "/home/shared/_codev/decisions/0100.md",
                adr("0100", "accepted", &[]),
            )
            .with_file(
                "/p/_codev/decisions/0007.md",
                adr_deviates("0007", "accepted", &["path:~/shared/0100"]),
            );
        let cfg = resolved(&fs);
        let idx = index(&fs, &env(), &Layout::new("/p"), &cfg).unwrap();

        // 2 entries, one local, one inherited.
        assert_eq!(idx.entries.len(), 2);
        let inherited = idx
            .entries
            .iter()
            .find(|e| e.qualified_id.origin != Origin::Project)
            .unwrap();
        assert!(inherited.deviated_by.is_some());
        assert_eq!(
            inherited.deviated_by.as_ref().unwrap().as_str(),
            "project/0007"
        );
        // The inherited one is removed from in_effect, the local one stays.
        assert!(!idx.in_effect.iter().any(|q| q.origin != Origin::Project));
        assert!(idx.in_effect.iter().any(|q| q.as_str() == "project/0007"));
    }

    #[test]
    fn deviation_from_unknown_target_raises_warning_and_stays_dangling() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file(
                "/p/_codev/decisions/0007.md",
                adr_deviates("0007", "accepted", &["path:~/unknown/9999"]),
            );
        let cfg = resolved(&fs);
        let idx = index(&fs, &env(), &Layout::new("/p"), &cfg).unwrap();
        assert!(
            idx.findings
                .iter()
                .any(|f| f.code == codes::DECISION_DANGLING_DEVIATION)
        );
        // The finding is a warning, not an error.
        assert!(
            idx.findings
                .iter()
                .filter(|f| f.code == codes::DECISION_DANGLING_DEVIATION)
                .all(|f| f.severity == Severity::Warning)
        );
    }

    #[test]
    fn two_deviations_on_same_target_trigger_conflict() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "inherits:\n  - path: ~/shared\n")
            .with_file(
                "/home/shared/_codev/decisions/0100.md",
                adr("0100", "accepted", &[]),
            )
            .with_file(
                "/p/_codev/decisions/0007.md",
                adr_deviates("0007", "accepted", &["path:~/shared/0100"]),
            )
            .with_file(
                "/p/_codev/decisions/0008.md",
                adr_deviates("0008", "accepted", &["path:~/shared/0100"]),
            );
        let cfg = resolved(&fs);
        let idx = index(&fs, &env(), &Layout::new("/p"), &cfg).unwrap();
        let conflicts: Vec<_> = idx
            .findings
            .iter()
            .filter(|f| f.code == codes::DECISION_CONFLICTING_DEVIATIONS)
            .collect();
        assert_eq!(
            conflicts.len(),
            1,
            "a single finding per conflicting target"
        );
        assert_eq!(conflicts[0].severity, Severity::Error);
        // Both ADRs are named in the message.
        assert!(conflicts[0].message.contains("project/0007"));
        assert!(conflicts[0].message.contains("project/0008"));
    }

    #[test]
    fn deviation_by_proposed_adr_has_no_effect() {
        // A proposed ADR is not binding; its deviation does not hide the target.
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "inherits:\n  - path: ~/shared\n")
            .with_file(
                "/home/shared/_codev/decisions/0100.md",
                adr("0100", "accepted", &[]),
            )
            .with_file(
                "/p/_codev/decisions/0007.md",
                adr_deviates("0007", "proposed", &["path:~/shared/0100"]),
            );
        let cfg = resolved(&fs);
        let idx = index(&fs, &env(), &Layout::new("/p"), &cfg).unwrap();
        let inherited = idx
            .entries
            .iter()
            .find(|e| e.qualified_id.origin != Origin::Project)
            .unwrap();
        assert!(inherited.deviated_by.is_none());
        // The inherited one does stay in_effect.
        assert!(
            idx.in_effect
                .iter()
                .any(|q| q.as_str() == "path:~/shared/0100")
        );
    }
}
