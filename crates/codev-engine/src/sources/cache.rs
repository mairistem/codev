//! Disposition du cache local pour les sources git héritées.
//!
//! Racine : `$XDG_CACHE_HOME/codev/` ou `~/.cache/codev/`. Deux sous-arbres :
//! `git/<hash-de-l-url>/` pour les dépôts *bare*, `content/<sha>/` pour le
//! contenu extrait à un SHA donné.

use std::path::PathBuf;

use sha2::{Digest, Sha256};

use crate::ports::Env;

/// Rend la racine du cache.
///
/// Ordre de priorité :
/// 1. `$XDG_CACHE_HOME/codev/` si `XDG_CACHE_HOME` est défini et non vide ;
/// 2. `$HOME/.cache/codev/` sinon ;
/// 3. `/tmp/codev/` en dernier recours, si `$HOME` est aussi absent.
pub fn root(env: &dyn Env) -> PathBuf {
    if let Some(xdg) = env.var("XDG_CACHE_HOME").filter(|v| !v.is_empty()) {
        return PathBuf::from(xdg).join("codev");
    }
    if let Some(home) = env.home_dir() {
        return home.join(".cache").join("codev");
    }
    PathBuf::from("/tmp/codev")
}

/// Hash déterministe d'une URL git — 16 hex de SHA-256, largement suffisant
/// pour éviter les collisions en pratique et pour rester lisible dans un
/// nom de dossier.
pub fn url_hash(url: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(url.as_bytes());
    let digest = hasher.finalize();
    hex_16(&digest[..8])
}

fn hex_16(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(16);
    for byte in bytes {
        out.push_str(&format!("{byte:02x}"));
    }
    out
}

/// Chemin du dépôt bare pour une URL donnée.
pub fn bare_repo_dir(cache_root: &std::path::Path, url_hash: &str) -> PathBuf {
    cache_root.join("git").join(url_hash)
}

/// Chemin du contenu extrait pour un SHA donné.
pub fn content_dir(cache_root: &std::path::Path, sha: &str) -> PathBuf {
    cache_root.join("content").join(sha)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ports::FixedEnv;

    fn env_with(vars: &[(&str, &str)]) -> FixedEnv {
        let mut env = FixedEnv::at("/p");
        for (k, v) in vars {
            env.vars.insert((*k).into(), (*v).into());
        }
        env
    }

    #[test]
    fn respecte_xdg_cache_home() {
        let env = env_with(&[("XDG_CACHE_HOME", "/opt/xdg"), ("HOME", "/home")]);
        assert_eq!(root(&env), PathBuf::from("/opt/xdg/codev"));
    }

    #[test]
    fn retombe_sur_home_cache() {
        let env = env_with(&[("HOME", "/Users/x")]);
        assert_eq!(root(&env), PathBuf::from("/Users/x/.cache/codev"));
    }

    #[test]
    fn xdg_vide_est_ignore() {
        // Une variable définie à la chaîne vide ne compte pas — le comportement
        // XDG standard.
        let env = env_with(&[("XDG_CACHE_HOME", ""), ("HOME", "/h")]);
        assert_eq!(root(&env), PathBuf::from("/h/.cache/codev"));
    }

    #[test]
    fn hash_est_stable_pour_une_meme_url() {
        let a = url_hash("git@github.com:acme/repo.git");
        let b = url_hash("git@github.com:acme/repo.git");
        assert_eq!(a, b);
        assert_eq!(a.len(), 16);
    }

    #[test]
    fn hash_differe_pour_des_url_distinctes() {
        assert_ne!(
            url_hash("git@github.com:a/b.git"),
            url_hash("git@github.com:a/c.git")
        );
    }

    #[test]
    fn chemins_calcules_correctement() {
        let root = std::path::Path::new("/cache");
        assert_eq!(
            bare_repo_dir(root, "abc123"),
            PathBuf::from("/cache/git/abc123")
        );
        assert_eq!(
            content_dir(root, "deadbeef"),
            PathBuf::from("/cache/content/deadbeef")
        );
    }
}
