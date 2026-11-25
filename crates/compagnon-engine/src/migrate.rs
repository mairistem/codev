//! The effects of `compagnon migrate`: finding the project under either
//! planning root, reading the facts the core's planner decides from, and
//! executing its operations one by one, stopping at the first failure.

use std::path::{Path, PathBuf};

use compagnon_core::layout::{CONFIG_FILE, PLANNING_DIR};
use compagnon_core::legacy;
use compagnon_core::migrate::{Facts, FormerSkill, Op, relative};

use crate::error::{EngineError, Result};
use crate::integration::RepoFacts;
use crate::ports::FileSystem;

/// The first ancestor of `start` holding a planning root, current or
/// former. Unlike `root::discover`, the former root is what this command
/// looks for.
pub fn discover(fs: &dyn FileSystem, start: &Path) -> Option<PathBuf> {
    start
        .ancestors()
        .find(|dir| {
            fs.exists(&dir.join(PLANNING_DIR)) || fs.exists(&dir.join(legacy::PLANNING_DIR))
        })
        .map(Path::to_path_buf)
}

/// Reads what the planner needs. `is_generated` tells a skill still
/// rendered by the former version from one edited by hand (it lives with
/// the skills' renderer, above this crate); `skills_need_refresh` says
/// whether a configured skill is missing or out of date.
pub fn gather(
    fs: &dyn FileSystem,
    project_root: &Path,
    running_version: &str,
    is_generated: &dyn Fn(&str) -> bool,
    skills_need_refresh: bool,
) -> Facts {
    let former_root = fs.exists(&project_root.join(legacy::PLANNING_DIR));
    let current_root = fs.exists(&project_root.join(PLANNING_DIR));
    let planning = if current_root {
        project_root.join(PLANNING_DIR)
    } else {
        project_root.join(legacy::PLANNING_DIR)
    };
    let config_path = planning.join(CONFIG_FILE);
    let config = fs.read_to_string(&config_path).ok();
    let has_git_sources = crate::config::load(fs, &config_path)
        .ok()
        .flatten()
        .is_some_and(|c| c.inherits.iter().any(|s| s.git.is_some()));

    let records = fs
        .walk_files(&planning)
        .unwrap_or_default()
        .into_iter()
        .filter(|f| {
            let name = f.rsplit('/').next().unwrap_or(f);
            name == "review.md" || name == "corrections.md"
        })
        .filter_map(|f| {
            let text = fs.read_to_string(&planning.join(&f)).ok()?;
            Some((PathBuf::from(f), text))
        })
        .collect();

    let mut managed_markdown = Vec::new();
    if let Ok(text) = fs.read_to_string(&project_root.join("CLAUDE.md")) {
        managed_markdown.push((PathBuf::from("CLAUDE.md"), text));
    }
    if let Some((path, text)) = RepoFacts::probe(fs, project_root).pr_template {
        managed_markdown.push((path, text));
    }

    let skills_root = project_root.join(".claude").join("skills");
    let prefix = format!("{}-", legacy::SKILL_PREFIX);
    let former_skills = fs
        .list_dir(&skills_root)
        .unwrap_or_default()
        .into_iter()
        .filter(|name| name.starts_with(&prefix))
        .map(|name| {
            let dir = skills_root.join(&name);
            let generated = fs
                .read_to_string(&dir.join("SKILL.md"))
                .is_ok_and(|text| is_generated(&text));
            FormerSkill { dir, generated }
        })
        .collect();

    Facts {
        project_root: project_root.to_path_buf(),
        former_root,
        current_root,
        former_lock: fs.exists(&planning.join(legacy::LOCK_FILE)),
        config,
        running_version: running_version.to_string(),
        records,
        managed_markdown,
        former_workflow: fs.read_to_string(&project_root.join(legacy::WORKFLOW)).ok(),
        current_workflow: fs.exists(&project_root.join(compagnon_core::migrate::WORKFLOW)),
        former_skills,
        has_git_sources,
        skills_need_refresh,
    }
}

/// Runs the operations in order. On the first failure, nothing more is
/// attempted and the error names the path, the system error and what was
/// already applied: running `compagnon migrate` again completes it.
pub fn execute(fs: &dyn FileSystem, project_root: &Path, ops: &[Op]) -> Result<()> {
    let mut applied = Vec::new();
    for op in ops {
        let (path, result) = match op {
            Op::Rename { from, to } => (from, fs.rename(from, to)),
            Op::Write { path, contents } => (path, write(fs, path, contents)),
            Op::Remove { path } => (path, fs.remove_file(path)),
            // A directory already gone is done: some file systems drop an
            // emptied folder by themselves.
            Op::RemoveDir { path } if !fs.exists(path) => (path, Ok(())),
            Op::RemoveDir { path } => (path, fs.remove_dir(path)),
            Op::Keep { .. } => continue,
        };
        if let Err(err) = result {
            return Err(EngineError::MigrationIncomplete {
                path: path.clone(),
                reason: err.to_string(),
                applied,
            });
        }
        applied.push(describe(project_root, op));
    }
    Ok(())
}

fn write(fs: &dyn FileSystem, path: &Path, contents: &str) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        fs.create_dir_all(parent)?;
    }
    fs.write(path, contents)
}

/// One line per operation, paths relative to the project root.
pub fn describe(project_root: &Path, op: &Op) -> String {
    let rel = |p: &Path| compagnon_core::layout::slashed(relative(project_root, p));
    match op {
        Op::Rename { from, to } => format!("moved {} to {}", rel(from), rel(to)),
        Op::Write { path, .. } => format!("wrote {}", rel(path)),
        Op::Remove { path } => format!("removed {}", rel(path)),
        Op::RemoveDir { path } => format!("removed {}/", rel(path)),
        Op::Keep { path } => format!("kept {} (edited by hand)", rel(path)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ports::MemoryFileSystem;
    use compagnon_core::migrate::{Outcome, plan};

    fn former(path: &str) -> String {
        format!("/p/{}/{path}", legacy::PLANNING_DIR)
    }

    #[test]
    fn discovers_a_former_root_from_below() {
        let fs = MemoryFileSystem::new().with_file(former("config.yaml"), "");
        assert_eq!(
            discover(&fs, Path::new("/p/src")),
            Some(PathBuf::from("/p"))
        );
        assert_eq!(discover(&MemoryFileSystem::new(), Path::new("/p")), None);
    }

    #[test]
    fn a_failure_names_the_path_and_the_applied_operations() {
        let fs = MemoryFileSystem::new()
            .with_file(former("config.yaml"), "schema: spec-driven\n")
            .with_file(former(legacy::LOCK_FILE), "version = 1\n");
        let ops = [
            Op::Rename {
                from: PathBuf::from(former("")),
                to: PathBuf::from("/p/_compagnon"),
            },
            Op::Remove {
                path: PathBuf::from("/p/missing.md"),
            },
        ];
        let err = execute(&fs, Path::new("/p"), &ops).unwrap_err();
        assert_eq!(err.code(), "migration_incomplete");
        let message = err.to_string();
        assert!(message.contains("missing.md"), "{message}");
        assert!(message.contains("moved"), "{message}");
        assert!(message.contains("compagnon migrate"), "{message}");
    }

    #[test]
    fn gather_then_execute_moves_a_codev_project() {
        let fs = MemoryFileSystem::new()
            .with_file(
                former("config.yaml"),
                format!("schema: spec-driven\n{}: \">=0.6\"\n", legacy::VERSION_KEY),
            )
            .with_file(former(legacy::LOCK_FILE), "version = 1\n");
        let facts = gather(&fs, Path::new("/p"), "0.7.0", &|_| true, false);
        let Outcome::Migrate { ops, .. } = plan(&facts) else {
            panic!("a migration");
        };
        execute(&fs, Path::new("/p"), &ops).unwrap();
        assert_eq!(
            fs.read("/p/_compagnon/config.yaml").as_deref(),
            Some("schema: spec-driven\ncompagnon_version: \">=0.6\"\n")
        );
        assert!(fs.read("/p/_compagnon/compagnon.lock").is_some());
        assert!(!fs.exists(Path::new(&former(""))));
    }
}
