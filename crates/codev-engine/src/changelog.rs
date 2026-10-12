//! What `codev changelog` needs from the outside world: which archived
//! changes are in range, according to git, and their proposals and deltas,
//! from the working tree.
//!
//! Git is reached through the [`ProcessRunner`] port only
//! (`_codev/decisions/0002-crate-graph-as-dependency-rule.md`), and only for
//! local, read-only queries: `rev-parse`, `tag --list`, `ls-tree -d`. Deciding
//! the default range, the content and the order is pure, in
//! `codev_core::changelog` (`_codev/decisions/0001-functional-core-imperative-shell.md`).

use std::path::Path;

use codev_core::Layout;
use codev_core::changelog::{ChangeInput, version};
use codev_core::parser::{DeltaSection, parse_delta};

use crate::error::{EngineError, Result, Warning};
use crate::ports::{FileSystem, ProcessRunner};

/// No git repository around the project: every archived change is listed.
pub const NOT_A_GIT_REPOSITORY: &str = "not_a_git_repository";
/// A change in range whose folder is missing from the working tree.
pub const ARCHIVED_CHANGE_UNREADABLE: &str = "archived_change_unreadable";

/// The changes in range, with the refs actually used.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Gathered {
    /// The `--since` used — explicit or the default tag; `None` when every
    /// archived change is in range.
    pub since: Option<String>,
    /// The `--until` used; `None` for the working tree.
    pub until: Option<String>,
    /// In archive order.
    pub changes: Vec<ChangeInput>,
    pub warnings: Vec<Warning>,
}

/// Reads the changes archived between `since` and `until`.
///
/// - Outside a git repository: every archived change of the working tree,
///   with a `not_a_git_repository` warning; explicit refs are refused.
/// - `until` absent: the working tree; present: `git ls-tree` at that ref.
/// - `since` absent: the highest `v*` tag below `until`
///   ([`version::default_since`]); no tag means everything is in range.
pub fn gather(
    fs: &dyn FileSystem,
    runner: &dyn ProcessRunner,
    layout: &Layout,
    since: Option<&str>,
    until: Option<&str>,
) -> Result<Gathered> {
    let root = layout.project_root();
    let archive_rel = archive_relative(layout);
    let mut gathered = Gathered {
        until: until.map(str::to_string),
        ..Gathered::default()
    };

    let names = if is_git_work_tree(runner, root) {
        let until_names = match until {
            Some(git_ref) => ls_tree(runner, root, git_ref, &archive_rel)?,
            None => working_tree_names(fs, layout),
        };
        let since = match since {
            Some(git_ref) => Some(git_ref.to_string()),
            None => version::default_since(&tags(runner, root), until),
        };
        let since_names = match &since {
            Some(git_ref) => ls_tree(runner, root, git_ref, &archive_rel)?,
            None => Vec::new(),
        };
        gathered.since = since;
        until_names
            .into_iter()
            .filter(|name| !since_names.contains(name))
            .collect::<Vec<_>>()
    } else {
        if since.is_some() || until.is_some() {
            return Err(EngineError::NotAGitRepository {
                root: root.to_path_buf(),
            });
        }
        gathered.warnings.push(Warning::new(
            NOT_A_GIT_REPOSITORY,
            "not a git repository: every archived change is listed",
        ));
        working_tree_names(fs, layout)
    };

    let mut names = names;
    names.sort();
    names.dedup();
    for name in names {
        let dir = layout.archive_dir().join(&name);
        let proposal_path = dir.join("proposal.md");
        let Ok(proposal) = fs.read_to_string(&proposal_path) else {
            gathered.warnings.push(Warning::new(
                ARCHIVED_CHANGE_UNREADABLE,
                format!(
                    "{name}: in range, but {archive_rel}{name}/proposal.md is missing from the working tree; skipped"
                ),
            ));
            continue;
        };
        gathered.changes.push(ChangeInput {
            archive_path: format!("{archive_rel}{name}"),
            removes_requirements: removes_requirements(fs, &dir)?,
            name,
            proposal,
        });
    }
    Ok(gathered)
}

/// `_codev/changes/archive/`, relative to the project root, `/`-separated:
/// git takes it relative to the working directory, on every platform.
fn archive_relative(layout: &Layout) -> String {
    let archive = layout.archive_dir();
    let relative = archive
        .strip_prefix(layout.project_root())
        .unwrap_or(&archive);
    let mut text = codev_core::layout::slashed(relative);
    if !text.ends_with('/') {
        text.push('/');
    }
    text
}

fn is_git_work_tree(runner: &dyn ProcessRunner, root: &Path) -> bool {
    runner
        .run("git", &["rev-parse", "--is-inside-work-tree"], Some(root))
        .is_ok_and(|out| out.is_ok() && out.stdout_str().trim() == "true")
}

/// The `v*` tags; none when git cannot list them.
fn tags(runner: &dyn ProcessRunner, root: &Path) -> Vec<String> {
    runner
        .run("git", &["tag", "--list", "v*"], Some(root))
        .ok()
        .filter(|out| out.is_ok())
        .map(|out| {
            out.stdout_str()
                .lines()
                .map(str::trim)
                .filter(|l| !l.is_empty())
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default()
}

/// The archive folder names present at `git_ref`. `-d` keeps folders only:
/// the archive's `.gitkeep` is not a change.
fn ls_tree(
    runner: &dyn ProcessRunner,
    root: &Path,
    git_ref: &str,
    archive_rel: &str,
) -> Result<Vec<String>> {
    let out = runner
        .run(
            "git",
            &["ls-tree", "-d", "--name-only", git_ref, "--", archive_rel],
            Some(root),
        )
        .map_err(|e| EngineError::UnknownRef {
            git_ref: git_ref.to_string(),
            reason: e.to_string(),
        })?;
    if !out.is_ok() {
        return Err(EngineError::UnknownRef {
            git_ref: git_ref.to_string(),
            reason: out.stderr_str().trim().to_string(),
        });
    }
    Ok(out
        .stdout_str()
        .lines()
        .map(|line| line.trim().trim_end_matches('/'))
        .filter(|line| !line.is_empty())
        .map(|line| line.rsplit('/').next().unwrap_or(line).to_string())
        .filter(|name| !name.starts_with('.'))
        .collect())
}

/// The archived changes of the working tree: folders holding a proposal.
fn working_tree_names(fs: &dyn FileSystem, layout: &Layout) -> Vec<String> {
    let archive = layout.archive_dir();
    fs.list_dir(&archive)
        .unwrap_or_default()
        .into_iter()
        .filter(|name| !name.starts_with('.'))
        .filter(|name| fs.exists(&archive.join(name).join("proposal.md")))
        .collect()
}

/// Whether one of the change's deltas removes a requirement.
fn removes_requirements(fs: &dyn FileSystem, change_dir: &Path) -> Result<bool> {
    let specs = change_dir.join("specs");
    for file in fs.walk_files(&specs).unwrap_or_default() {
        if !file.ends_with(".md") {
            continue;
        }
        let path = specs.join(&file);
        let source = fs
            .read_to_string(&path)
            .map_err(|e| EngineError::Unreadable {
                path: path.clone(),
                reason: e.to_string(),
            })?;
        let removes =
            parse_delta(&source).value.sections.iter().any(
                |s| matches!(s, DeltaSection::Removed { removals, .. } if !removals.is_empty()),
            );
        if removes {
            return Ok(true);
        }
    }
    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ports::{MemoryFileSystem, MockProcessRunner, ProcessOutput};

    fn ok(stdout: &str) -> ProcessOutput {
        ProcessOutput {
            stdout: stdout.as_bytes().to_vec(),
            stderr: Vec::new(),
            exit_code: 0,
        }
    }

    fn failed(stderr: &str) -> ProcessOutput {
        ProcessOutput {
            stdout: Vec::new(),
            stderr: stderr.as_bytes().to_vec(),
            exit_code: 128,
        }
    }

    fn listing(names: &[&str]) -> String {
        names
            .iter()
            .map(|n| format!("_codev/changes/archive/{n}\n"))
            .collect()
    }

    /// Three archived changes: `a` before v0.9.0, `b` between v0.9.0 and
    /// v0.10.0, `c` after v0.10.0, only in the working tree.
    fn project() -> MemoryFileSystem {
        let mut fs = MemoryFileSystem::new().with_file("/p/_codev/config.yaml", "");
        for name in ["2026-01-01-a", "2026-02-01-b", "2026-03-01-c"] {
            fs = fs.with_file(
                format!("/p/_codev/changes/archive/{name}/proposal.md"),
                format!("# Proposal: {name}\n\n## Release note\n\n- Added: {name}.\n"),
            );
        }
        fs
    }

    fn git() -> MockProcessRunner {
        MockProcessRunner::new()
            .with_response("git", &["rev-parse"], ok("true\n"))
            .with_response("git", &["tag"], ok("v0.9.0\nv0.10.0\n"))
            .with_response(
                "git",
                &["ls-tree", "-d", "--name-only", "v0.9.0"],
                ok(&listing(&["2026-01-01-a"])),
            )
            .with_response(
                "git",
                &["ls-tree", "-d", "--name-only", "v0.10.0"],
                ok(&listing(&["2026-01-01-a", "2026-02-01-b"])),
            )
            .with_response(
                "git",
                &["ls-tree", "-d", "--name-only", "v9.9.9"],
                failed("fatal: Not a valid object name v9.9.9"),
            )
    }

    fn names(gathered: &Gathered) -> Vec<&str> {
        gathered.changes.iter().map(|c| c.name.as_str()).collect()
    }

    #[test]
    fn gather_uses_the_latest_tag_by_default() {
        let runner = git();
        let gathered = gather(&project(), &runner, &Layout::new("/p"), None, None).unwrap();
        assert_eq!(names(&gathered), ["2026-03-01-c"]);
        assert_eq!(gathered.since.as_deref(), Some("v0.10.0"));
        assert!(gathered.warnings.is_empty());
        assert_eq!(
            gathered.changes[0].archive_path,
            "_codev/changes/archive/2026-03-01-c"
        );
        // Only local, read-only git commands.
        for (program, args) in runner.calls() {
            assert_eq!(program, "git");
            assert!(
                ["rev-parse", "tag", "ls-tree"].contains(&args[0].as_str()),
                "{args:?}"
            );
        }
    }

    #[test]
    fn gather_between_two_tags() {
        let gathered = gather(
            &project(),
            &git(),
            &Layout::new("/p"),
            None,
            Some("v0.10.0"),
        )
        .unwrap();
        assert_eq!(names(&gathered), ["2026-02-01-b"]);
        assert_eq!(gathered.since.as_deref(), Some("v0.9.0"));
        assert_eq!(gathered.until.as_deref(), Some("v0.10.0"));
    }

    #[test]
    fn gather_without_tag_lists_everything() {
        let runner = MockProcessRunner::new()
            .with_response("git", &["rev-parse"], ok("true\n"))
            .with_response("git", &["tag"], ok(""));
        let gathered = gather(&project(), &runner, &Layout::new("/p"), None, None).unwrap();
        assert_eq!(
            names(&gathered),
            ["2026-01-01-a", "2026-02-01-b", "2026-03-01-c"]
        );
        assert_eq!(gathered.since, None);
    }

    #[test]
    fn gather_outside_git_lists_everything_with_a_warning() {
        let runner = MockProcessRunner::new().with_response(
            "git",
            &["rev-parse"],
            failed("fatal: not a git repository"),
        );
        let gathered = gather(&project(), &runner, &Layout::new("/p"), None, None).unwrap();
        assert_eq!(names(&gathered).len(), 3);
        assert_eq!(gathered.warnings[0].code, NOT_A_GIT_REPOSITORY);
        // No git installed at all behaves the same.
        let gathered = gather(
            &project(),
            &MockProcessRunner::new(),
            &Layout::new("/p"),
            None,
            None,
        )
        .unwrap();
        assert_eq!(gathered.warnings[0].code, NOT_A_GIT_REPOSITORY);
    }

    #[test]
    fn gather_outside_git_refuses_explicit_refs() {
        let err = gather(
            &project(),
            &MockProcessRunner::new(),
            &Layout::new("/p"),
            Some("v0.9.0"),
            None,
        )
        .unwrap_err();
        assert_eq!(err.code(), "not_a_git_repository");
    }

    #[test]
    fn gather_unknown_ref_fails() {
        let err = gather(&project(), &git(), &Layout::new("/p"), Some("v9.9.9"), None).unwrap_err();
        assert_eq!(err.code(), "unknown_ref");
        assert!(err.to_string().contains("v9.9.9"));
    }

    #[test]
    fn gather_skips_a_change_missing_from_the_tree() {
        let runner = git().with_response(
            "git",
            &["ls-tree", "-d", "--name-only", "HEAD"],
            ok(&listing(&[
                ".gitkeep",
                "2026-01-01-a",
                "2026-02-01-b",
                "2026-02-15-gone",
            ])),
        );
        let gathered = gather(
            &project(),
            &runner,
            &Layout::new("/p"),
            Some("v0.9.0"),
            Some("HEAD"),
        )
        .unwrap();
        assert_eq!(names(&gathered), ["2026-02-01-b"]);
        // A dotfile is not a change: one warning, for the missing folder.
        assert_eq!(gathered.warnings.len(), 1, "{:?}", gathered.warnings);
        assert_eq!(gathered.warnings[0].code, ARCHIVED_CHANGE_UNREADABLE);
        assert!(gathered.warnings[0].message.contains("2026-02-15-gone"));
    }

    #[test]
    fn gather_flags_removed_requirements() {
        let fs = project().with_file(
            "/p/_codev/changes/archive/2026-03-01-c/specs/x/spec.md",
            "## REMOVED Requirements\n\n### Requirement: Old\n**Reason**: gone\n**Migration**: none\n",
        );
        let gathered = gather(&fs, &git(), &Layout::new("/p"), None, None).unwrap();
        assert!(gathered.changes[0].removes_requirements);
        let gathered = gather(&project(), &git(), &Layout::new("/p"), None, None).unwrap();
        assert!(!gathered.changes[0].removes_requirements);
    }
}
