//! Fichier de verrouillage des sources héritées : `_codev/codev.lock`.
//!
//! Format TOML, à la convention `<outil>.lock`. Une seule commande
//! l'écrit : `codev sources update`.

use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::error::{EngineError, Result};
use crate::ports::FileSystem;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Lockfile {
    /// Version du format — permet une évolution future sans casser les
    /// lecteurs anciens.
    #[serde(default = "default_version")]
    pub version: u32,
    #[serde(default, rename = "source")]
    pub sources: Vec<LockEntry>,
}

fn default_version() -> u32 {
    1
}

impl Default for Lockfile {
    fn default() -> Self {
        Self {
            version: 1,
            sources: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LockEntry {
    pub git: String,
    /// Le champ `ref` est un mot-clé Rust — on l'écrit `ref` dans le TOML
    /// via `#[serde(rename)]`.
    #[serde(rename = "ref")]
    pub git_ref: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subpath: Option<String>,
    pub commit: String,
    pub resolved_at: String,
}

impl LockEntry {
    pub fn find_matching<'a>(
        lock: Option<&'a Lockfile>,
        git: &str,
        git_ref: &str,
    ) -> Option<&'a LockEntry> {
        lock?
            .sources
            .iter()
            .find(|e| e.git == git && e.git_ref == git_ref)
    }
}

/// Charge le lock. Un fichier absent rend `Ok(None)` — le projet n'a pas
/// encore fait de `codev sources update`.
pub fn load(fs: &dyn FileSystem, path: &Path) -> Result<Option<Lockfile>> {
    if !fs.exists(path) {
        return Ok(None);
    }
    let raw = fs.read_to_string(path).map_err(|e| EngineError::Unreadable {
        path: path.to_path_buf(),
        reason: e.to_string(),
    })?;
    parse(&raw).map(Some).map_err(|reason| EngineError::Invalid {
        path: path.to_path_buf(),
        reason,
    })
}

pub fn parse(source: &str) -> std::result::Result<Lockfile, String> {
    toml::from_str::<Lockfile>(source).map_err(|e| e.to_string())
}

pub fn serialize(lock: &Lockfile) -> std::result::Result<String, String> {
    toml::to_string(lock).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ports::MemoryFileSystem;

    fn entry(git: &str, git_ref: &str, commit: &str) -> LockEntry {
        LockEntry {
            git: git.into(),
            git_ref: git_ref.into(),
            subpath: None,
            commit: commit.into(),
            resolved_at: "2026-09-08T00:00:00Z".into(),
        }
    }

    #[test]
    fn le_champ_ref_est_ecrit_sans_backtick() {
        let lock = Lockfile {
            version: 1,
            sources: vec![entry("git@github.com:o/r.git", "main", "abc123")],
        };
        let serialized = serialize(&lock).unwrap();
        // Le nom TOML est `ref`, pas `git_ref` — c'est le rename qui joue.
        assert!(serialized.contains("ref = \"main\""), "{serialized}");
        assert!(!serialized.contains("git_ref"), "{serialized}");
        assert!(serialized.contains("[[source]]"), "{serialized}");
    }

    #[test]
    fn serialise_puis_reparse_est_identite() {
        let lock = Lockfile {
            version: 1,
            sources: vec![
                entry("git@github.com:a/b.git", "main", "1111"),
                LockEntry {
                    subpath: Some("shared/".into()),
                    ..entry("https://github.com/a/c.git", "v1", "2222")
                },
            ],
        };
        let s = serialize(&lock).unwrap();
        let back = parse(&s).unwrap();
        assert_eq!(back, lock);
    }

    #[test]
    fn absent_est_none() {
        let fs = MemoryFileSystem::new();
        let out = load(&fs, Path::new("/p/_codev/codev.lock")).unwrap();
        assert!(out.is_none());
    }

    #[test]
    fn find_matching_reconnait_par_url_et_ref() {
        let lock = Lockfile {
            version: 1,
            sources: vec![
                entry("url1", "main", "aa"),
                entry("url1", "v1", "bb"),
                entry("url2", "main", "cc"),
            ],
        };
        let found = LockEntry::find_matching(Some(&lock), "url1", "v1").unwrap();
        assert_eq!(found.commit, "bb");
    }
}
