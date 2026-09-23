//! Sources héritées — cache local, lock, résolution de `inherits: git:`.
//!
//! Trois couches :
//!
//! - `cache` : disposition disque et hash d'URL, pur ;
//! - `lockfile` : lecture/écriture de `_codev/codev.lock` en TOML ;
//! - `update` : plan + exécution de `codev sources update`.
//!
//! Aucune commande courante (`status`, `instructions`, `validate`, `sync`,
//! `archive`) ne descend jamais ici. Seul `codev sources update` le fait.

pub mod cache;
pub mod lockfile;
pub mod update;

pub use lockfile::{LockEntry, Lockfile};
pub use update::{
    plan_sources_update, run_sources_update, GitSourceInput, PinChange, SourcesUpdatePlan,
    UpdateOutcome,
};

use std::path::PathBuf;

use crate::config::{load, InheritSource};
use crate::error::Result;
use crate::ports::{Env, FileSystem};
use codev_core::Layout;

/// Extensions autorisées pour du contenu hérité — mise en œuvre matérielle
/// du principe « aucun contenu exécutable hérité » de la décision 0005.
///
/// Utilisé par `list_files_exposed` — les autres lecteurs
/// (`decisions::index`, `config::resolve`) filtrent déjà par nom précis,
/// c'est-à-dire un cas particulier de cette règle.
pub const ALLOWED_EXTENSIONS: &[&str] = &["md", "yaml"];

/// Type d'une source déclarée.
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

/// État de résolution d'une source.
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

/// L'état complet d'une source déclarée dans `inherits`, pour la commande
/// `codev sources list`.
#[derive(Debug, Clone)]
pub struct SourceStatus {
    pub kind: SourceKind,
    /// Pour `Path` : le chemin déclaré ; pour `Git` : l'URL.
    pub address: String,
    pub state: SourceState,
    pub git_ref: Option<String>,
    pub subpath: Option<String>,
    pub sha: Option<String>,
    /// Chemin résolu sur le disque, si applicable.
    pub resolved_path: Option<PathBuf>,
}

/// Rassemble l'état de chaque source déclarée par le projet.
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
        address: "(malformée)".to_string(),
        state: SourceState::Unreadable,
        git_ref: None,
        subpath: None,
        sha: None,
        resolved_path: None,
    }
}

/// Liste les fichiers **exposés** sous un chemin racine — seuls ceux dont
/// l'extension est dans `ALLOWED_EXTENSIONS`.
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
