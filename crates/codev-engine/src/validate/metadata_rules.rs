//! The two rules that depend on the disk: presence of deltas and consistency
//! with the `skip_specs` marker. They are not in `codev-core::validate`
//! because they need to count files, not just an AST.

use std::path::PathBuf;

use codev_core::Layout;
use codev_core::parser::ast::{Finding, Severity};
use codev_core::validate::codes;

use crate::change::ChangeContext;
use crate::error::{EngineError, Result};
use crate::ports::FileSystem;

use super::report::LocatedFinding;

/// Checks a change's metadata against its content.
///
/// Two rejected cases:
///
/// - `zero_delta_without_marker`: no delta under `specs/` and
///   `skip_specs: true` not declared. The validator requires an explicit
///   decision; without it, the user could forget a capability.
/// - `skip_specs_conflict`: `skip_specs: true` declared but `.md` files
///   exist under `specs/` anyway. This is contradictory; the archive
///   would not know whether to merge them or ignore them.
pub fn check_change_metadata(
    fs: &dyn FileSystem,
    layout: &Layout,
    ctx: &ChangeContext,
) -> Result<Vec<LocatedFinding>> {
    let change_dir = layout.change_dir(&ctx.change);
    let specs_dir = change_dir.join("specs");
    let metadata_path = layout.change_metadata(&ctx.change);
    let metadata_relative = metadata_path
        .strip_prefix(layout.project_root())
        .unwrap_or(&metadata_path)
        .to_path_buf();

    let deltas = list_delta_files(fs, &specs_dir)?;
    let has_deltas = !deltas.is_empty();

    let mut out = Vec::new();

    if !has_deltas && !ctx.metadata.skip_specs {
        out.push(zero_delta_finding(metadata_relative.clone()));
    }
    if has_deltas && ctx.metadata.skip_specs {
        out.push(skip_specs_conflict_finding(
            metadata_relative,
            &deltas,
            layout,
        ));
    }

    Ok(out)
}

fn zero_delta_finding(metadata_relative: PathBuf) -> LocatedFinding {
    LocatedFinding::from_finding(
        Finding {
            severity: Severity::Error,
            code: codes::ZERO_DELTA_WITHOUT_MARKER,
            // Line 1 by default: the finding concerns the change as a
            // whole, not a specific spot in `change.yaml`.
            line: 1,
            message: "no delta file under `specs/` and `skip_specs: true` is not declared; \
                      add a delta or set `skip_specs: true` in change.yaml"
                .into(),
        },
        metadata_relative,
    )
}

fn skip_specs_conflict_finding(
    metadata_relative: PathBuf,
    deltas: &[PathBuf],
    layout: &Layout,
) -> LocatedFinding {
    let previews: Vec<String> = deltas
        .iter()
        .take(3)
        .map(|p| {
            p.strip_prefix(layout.project_root())
                .unwrap_or(p)
                .display()
                .to_string()
        })
        .collect();
    let suffix = if deltas.len() > 3 {
        format!(", … ({} in total)", deltas.len())
    } else {
        String::new()
    };
    LocatedFinding::from_finding(
        Finding {
            severity: Severity::Error,
            code: codes::SKIP_SPECS_CONFLICT,
            line: 1,
            message: format!(
                "`skip_specs: true` is declared but delta files exist: {}{}. \
                 Remove `skip_specs` or delete the files in the `specs/` directory",
                previews.join(", "),
                suffix
            ),
        },
        metadata_relative,
    )
}

fn list_delta_files(fs: &dyn FileSystem, specs_dir: &std::path::Path) -> Result<Vec<PathBuf>> {
    let files = fs
        .walk_files(specs_dir)
        .map_err(|e| EngineError::Unreadable {
            path: specs_dir.to_path_buf(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::change;
    use crate::config;
    use crate::ports::{FixedEnv, MemoryFileSystem};
    use codev_core::ChangeId;

    fn env() -> FixedEnv {
        let mut env = FixedEnv::at("/p");
        env.vars.insert("HOME".into(), "/home".into());
        env
    }

    fn load_change(fs: &dyn FileSystem, name: &str) -> ChangeContext {
        let layout = Layout::new("/p");
        let cfg = config::resolve(fs, &env(), &layout).unwrap();
        change::load(fs, &layout, &cfg, ChangeId::parse(name).unwrap()).unwrap()
    }

    #[test]
    fn zero_delta_without_marker_fails() {
        // Active change, no delta, `skip_specs` not declared: rejected.
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file(
                "/p/_codev/changes/refactor/change.yaml",
                "schema: spec-driven",
            )
            .with_file("/p/_codev/changes/refactor/proposal.md", "x");

        let ctx = load_change(&fs, "refactor");
        let findings = check_change_metadata(&fs, &Layout::new("/p"), &ctx).unwrap();

        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].finding.code, codes::ZERO_DELTA_WITHOUT_MARKER);
        assert!(
            findings[0].finding.message.contains("skip_specs"),
            "{}",
            findings[0].finding.message
        );
    }

    #[test]
    fn zero_delta_with_marker_is_accepted() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file(
                "/p/_codev/changes/refactor/change.yaml",
                "schema: spec-driven\nskip_specs: true\n",
            )
            .with_file("/p/_codev/changes/refactor/proposal.md", "x");

        let ctx = load_change(&fs, "refactor");
        let findings = check_change_metadata(&fs, &Layout::new("/p"), &ctx).unwrap();
        assert!(findings.is_empty(), "no finding expected: {findings:?}");
    }

    #[test]
    fn skip_specs_with_delta_is_a_conflict() {
        // The marker is declared AND a delta exists: contradictory.
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file(
                "/p/_codev/changes/refactor/change.yaml",
                "schema: spec-driven\nskip_specs: true\n",
            )
            .with_file(
                "/p/_codev/changes/refactor/specs/x/spec.md",
                "## Purpose\n\na spec\n",
            );

        let ctx = load_change(&fs, "refactor");
        let findings = check_change_metadata(&fs, &Layout::new("/p"), &ctx).unwrap();

        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].finding.code, codes::SKIP_SPECS_CONFLICT);
        assert!(
            findings[0].finding.message.contains("Remove"),
            "{}",
            findings[0].finding.message
        );
    }
}
