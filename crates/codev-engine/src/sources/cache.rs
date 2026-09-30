//! Layout of the local cache for inherited git sources.
//!
//! Root: `$XDG_CACHE_HOME/codev/` or `~/.cache/codev/`. Two subtrees:
//! `git/<url-hash>/` for the *bare* repositories, `content/<sha>/` for the
//! content checked out at a given SHA.

use std::path::PathBuf;

use sha2::{Digest, Sha256};

use crate::ports::Env;

/// Returns the cache root.
///
/// Order of precedence:
/// 1. `$XDG_CACHE_HOME/codev/` if `XDG_CACHE_HOME` is set and non-empty;
/// 2. `$HOME/.cache/codev/` otherwise;
/// 3. `/tmp/codev/` as a last resort, if `$HOME` is missing too.
pub fn root(env: &dyn Env) -> PathBuf {
    if let Some(xdg) = env.var("XDG_CACHE_HOME").filter(|v| !v.is_empty()) {
        return PathBuf::from(xdg).join("codev");
    }
    if let Some(home) = env.home_dir() {
        return home.join(".cache").join("codev");
    }
    PathBuf::from("/tmp/codev")
}

/// Deterministic hash of a git URL — 16 hex digits of SHA-256, more than
/// enough to avoid collisions in practice while staying readable in a
/// directory name.
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

/// Path of the bare repository for a given URL.
pub fn bare_repo_dir(cache_root: &std::path::Path, url_hash: &str) -> PathBuf {
    cache_root.join("git").join(url_hash)
}

/// Path of the checked-out content for a given SHA.
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
    fn honors_xdg_cache_home() {
        let env = env_with(&[("XDG_CACHE_HOME", "/opt/xdg"), ("HOME", "/home")]);
        assert_eq!(root(&env), PathBuf::from("/opt/xdg/codev"));
    }

    #[test]
    fn falls_back_to_home_cache() {
        let env = env_with(&[("HOME", "/Users/x")]);
        assert_eq!(root(&env), PathBuf::from("/Users/x/.cache/codev"));
    }

    #[test]
    fn empty_xdg_is_ignored() {
        // A variable set to the empty string does not count — the standard
        // XDG behavior.
        let env = env_with(&[("XDG_CACHE_HOME", ""), ("HOME", "/h")]);
        assert_eq!(root(&env), PathBuf::from("/h/.cache/codev"));
    }

    #[test]
    fn hash_is_stable_for_the_same_url() {
        let a = url_hash("git@github.com:acme/repo.git");
        let b = url_hash("git@github.com:acme/repo.git");
        assert_eq!(a, b);
        assert_eq!(a.len(), 16);
    }

    #[test]
    fn hash_differs_for_distinct_urls() {
        assert_ne!(
            url_hash("git@github.com:a/b.git"),
            url_hash("git@github.com:a/c.git")
        );
    }

    #[test]
    fn paths_are_computed_correctly() {
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
