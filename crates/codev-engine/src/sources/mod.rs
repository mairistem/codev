//! Inherited sources — local cache, lock, resolution of `inherits: git:`.
//!
//! Three layers:
//!
//! - `cache`: on-disk layout and URL hash, pure;
//! - `lockfile`: reading/writing `_codev/codev.lock` as TOML;
//! - `update`: plan + execution of `codev sources update`.
//!
//! No everyday command (`status`, `instructions`, `validate`, `sync`,
//! `archive`) ever goes down here. Only `codev sources update` does.

pub mod cache;
pub mod lockfile;
pub mod update;

pub use lockfile::{LockEntry, Lockfile};
pub use update::{
    GitSourceInput, PinChange, SourcesUpdatePlan, UpdateOutcome, plan_sources_update,
    run_sources_update,
};

use std::path::PathBuf;

use crate::config::{InheritSource, load};
use crate::error::Result;
use crate::ports::{Env, FileSystem};
use codev_core::Layout;

/// Extensions allowed for inherited content — the concrete enforcement of
/// the "no inherited executable content" principle from decision 0005.
///
/// Used by `list_files_exposed` — the other readers
/// (`decisions::index`, `config::resolve`) already filter by exact name,
/// which is a special case of this rule.
pub const ALLOWED_EXTENSIONS: &[&str] = &["md", "yaml"];

/// Kind of a declared source.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SourceKind {
    Path,
    Git,
}

impl SourceKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Path => "path",
            Self::Git => "git",
        }
    }
}

/// Resolution state of a source.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SourceState {
    Resolved,
    Locked,
    Unlocked,
    NeedsUpdate,
    Unreadable,
}

impl SourceState {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Resolved => "resolved",
            Self::Locked => "locked",
            Self::Unlocked => "unlocked",
            Self::NeedsUpdate => "needs_update",
            Self::Unreadable => "unreadable",
        }
    }
}

/// The complete state of a source declared in `inherits`, for the
/// `codev sources list` command.
#[derive(Debug, Clone)]
pub struct SourceStatus {
    pub kind: SourceKind,
    /// For `Path`: the declared path; for `Git`: the URL.
    pub address: String,
    pub state: SourceState,
    pub git_ref: Option<String>,
    pub subpath: Option<String>,
    pub sha: Option<String>,
    /// Resolved path on disk, if applicable.
    pub resolved_path: Option<PathBuf>,
}

impl SourceStatus {
    /// The root of the source's codev project: `resolved_path`, joined
    /// with `subpath` when the project lives in a folder of the repository.
    ///
    /// Everything read from a source (config, decisions) goes through it,
    /// so that `subpath` applies to all of them alike.
    pub fn project_root(&self) -> Option<PathBuf> {
        let root = self.resolved_path.as_ref()?;
        Some(project_root(root, self.subpath.as_deref()))
    }
}

/// `content_root`, joined with `subpath` when one is declared.
pub fn project_root(content_root: &std::path::Path, subpath: Option<&str>) -> PathBuf {
    match subpath {
        Some(sub) => content_root.join(sub),
        None => content_root.to_path_buf(),
    }
}

/// Gathers the state of every source declared by the project.
pub fn list_source_states(
    fs: &dyn FileSystem,
    env: &dyn Env,
    layout: &Layout,
) -> Result<Vec<SourceStatus>> {
    let config_path = layout.config_file();
    let project = load(fs, &config_path)?.unwrap_or_default();
    let lock = lockfile::load(fs, &layout.planning_dir().join("codev.lock"))?;

    let mut out = Vec::new();
    for source in &project.inherits {
        out.push(status_of(source, fs, env, lock.as_ref()));
    }
    Ok(out)
}

fn status_of(
    source: &InheritSource,
    fs: &dyn FileSystem,
    env: &dyn Env,
    lock: Option<&Lockfile>,
) -> SourceStatus {
    if let Some(raw_path) = &source.path {
        let resolved = crate::config::expand_tilde(raw_path, env);
        let state = if fs.exists(&resolved.join("_codev").join("config.yaml")) {
            SourceState::Resolved
        } else {
            SourceState::Unreadable
        };
        return SourceStatus {
            kind: SourceKind::Path,
            address: raw_path.clone(),
            state,
            git_ref: None,
            subpath: source.subpath.clone(),
            sha: None,
            resolved_path: Some(resolved),
        };
    }
    if let Some(url) = &source.git {
        let git_ref = source.git_ref.clone().unwrap_or_else(|| "?".to_string());
        let entry = LockEntry::find_matching(lock, url, &git_ref);
        let (state, sha, resolved_path) = match entry {
            None => (SourceState::Unlocked, None, None),
            Some(entry) => {
                let cache_root = cache::root(env);
                let content = cache::content_dir(&cache_root, &entry.commit);
                if fs.exists(&content) {
                    (
                        SourceState::Locked,
                        Some(entry.commit.clone()),
                        Some(content),
                    )
                } else {
                    (
                        SourceState::NeedsUpdate,
                        Some(entry.commit.clone()),
                        Some(content),
                    )
                }
            }
        };
        return SourceStatus {
            kind: SourceKind::Git,
            address: url.clone(),
            state,
            git_ref: Some(git_ref),
            subpath: source.subpath.clone(),
            sha,
            resolved_path,
        };
    }
    SourceStatus {
        kind: SourceKind::Path,
        address: "(malformed)".to_string(),
        state: SourceState::Unreadable,
        git_ref: None,
        subpath: None,
        sha: None,
        resolved_path: None,
    }
}

/// Lists the **exposed** files under a root path — only those whose
/// extension is in `ALLOWED_EXTENSIONS`.
pub fn list_files_exposed(fs: &dyn FileSystem, root: &std::path::Path) -> Vec<String> {
    let files = fs.walk_files(root).unwrap_or_default();
    files
        .into_iter()
        .filter(|p| {
            let ext = p.rsplit('.').next().unwrap_or("");
            ALLOWED_EXTENSIONS.contains(&ext)
        })
        .collect()
}
