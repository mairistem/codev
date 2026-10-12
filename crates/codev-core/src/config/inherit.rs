//! The value of `codev init --inherit <SOURCE>`.
//!
//! Pure: it only classifies and splits the text. Whether a local path
//! exists is a question for the file system, asked by the shell before any
//! write.

/// The ref written when a git source has no `@<ref>` suffix. The existing
/// sources code cannot resolve a remote's default branch, and `ref:` is
/// mandatory in `inherits:`.
pub const DEFAULT_GIT_REF: &str = "main";

/// A parsed `--inherit` value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InheritArg {
    Git { url: String, git_ref: String },
    Path(String),
}

const GIT_PREFIXES: &[&str] = &["git@", "https://", "http://", "ssh://", "file://"];

/// Parses a `--inherit` value.
///
/// A git source starts with one of `git@`, `https://`, `http://`, `ssh://`,
/// `file://`, or ends with `.git` once an `@<ref>` suffix is removed;
/// anything else is a local path, kept as written. The ref separator is the
/// last `@` of the *path part* — after `scheme://host/` or after the `:` of
/// `user@host:path` — so the user name before the host is never taken for
/// a ref.
pub fn parse(raw: &str) -> Result<InheritArg, String> {
    let value = raw.trim();
    if value.is_empty() {
        return Err("the source is empty; give a git URL or a local path".into());
    }

    let has_git_prefix = GIT_PREFIXES.iter().any(|p| value.starts_with(p));
    let path_start = path_part_start(value);
    let (candidate_url, explicit_ref) = match value[path_start..].rfind('@') {
        Some(at) => (
            &value[..path_start + at],
            Some(&value[path_start + at + 1..]),
        ),
        None => (value, None),
    };
    let ends_with_git = candidate_url.trim_end_matches('/').ends_with(".git");

    if !has_git_prefix && !ends_with_git {
        // A local path may legitimately contain `@`: keep it whole.
        return Ok(InheritArg::Path(value.to_string()));
    }

    let git_ref = match explicit_ref {
        Some("") => {
            return Err(format!(
                "`{value}` ends with `@` but names no ref; write `<url>@<branch-or-tag>` or drop the `@`"
            ));
        }
        Some(r) => r.to_string(),
        None => DEFAULT_GIT_REF.to_string(),
    };
    if !has_host_or_path(candidate_url) {
        return Err(format!("`{value}` names no repository"));
    }
    Ok(InheritArg::Git {
        url: candidate_url.to_string(),
        git_ref,
    })
}

/// Byte index where the repository path starts, i.e. after any user name
/// and host.
fn path_part_start(value: &str) -> usize {
    if let Some(scheme_end) = value.find("://") {
        let after = scheme_end + 3;
        return value[after..]
            .find('/')
            .map_or(value.len(), |slash| after + slash);
    }
    // scp-like `user@host:path`: a `:` before any `/`.
    match (value.find(':'), value.find('/')) {
        (Some(colon), Some(slash)) if colon < slash => colon + 1,
        (Some(colon), None) => colon + 1,
        _ => 0,
    }
}

/// Whether a git URL has something after its scheme or its `user@host:`.
fn has_host_or_path(url: &str) -> bool {
    if let Some(scheme_end) = url.find("://") {
        return !url[scheme_end + 3..].trim_matches('/').is_empty();
    }
    if let Some(rest) = url.strip_prefix("git@") {
        return rest
            .split_once(':')
            .is_some_and(|(host, path)| !host.is_empty() && !path.trim_matches('/').is_empty());
    }
    !url.trim_end_matches('/')
        .trim_end_matches(".git")
        .is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn git(url: &str, git_ref: &str) -> InheritArg {
        InheritArg::Git {
            url: url.into(),
            git_ref: git_ref.into(),
        }
    }

    #[test]
    fn parses_scp_https_ssh_file_and_dot_git_sources() {
        assert_eq!(
            parse("git@github.com:acme/conventions.git").unwrap(),
            git("git@github.com:acme/conventions.git", "main")
        );
        assert_eq!(
            parse("https://gitlab.example.com/acme/conventions").unwrap(),
            git("https://gitlab.example.com/acme/conventions", "main")
        );
        assert_eq!(
            parse("ssh://git@host.example/acme/c.git@v2").unwrap(),
            git("ssh://git@host.example/acme/c.git", "v2")
        );
        assert_eq!(
            parse("file:///srv/git/conventions.git@release/1.x").unwrap(),
            git("file:///srv/git/conventions.git", "release/1.x")
        );
        assert_eq!(
            parse("/srv/git/conventions.git").unwrap(),
            git("/srv/git/conventions.git", "main")
        );
        assert_eq!(
            parse("../shared.git@dev").unwrap(),
            git("../shared.git", "dev")
        );
    }

    #[test]
    fn splits_the_ref_after_the_path_not_the_user() {
        assert_eq!(
            parse("git@github.com:acme/conventions.git@main").unwrap(),
            git("git@github.com:acme/conventions.git", "main")
        );
        assert_eq!(
            parse("https://user@example.com/acme/c.git").unwrap(),
            git("https://user@example.com/acme/c.git", "main")
        );
        assert_eq!(
            parse("https://user@example.com/acme/c.git@v1.2").unwrap(),
            git("https://user@example.com/acme/c.git", "v1.2")
        );
    }

    #[test]
    fn defaults_the_ref_to_main() {
        assert_eq!(
            parse("https://example.com/acme/conventions.git").unwrap(),
            git("https://example.com/acme/conventions.git", DEFAULT_GIT_REF)
        );
    }

    #[test]
    fn local_paths_are_kept_whole() {
        assert_eq!(
            parse("~/codev/shared").unwrap(),
            InheritArg::Path("~/codev/shared".into())
        );
        assert_eq!(
            parse("/home/me/team@2026").unwrap(),
            InheritArg::Path("/home/me/team@2026".into())
        );
        assert_eq!(
            parse(" ./conventions ").unwrap(),
            InheritArg::Path("./conventions".into())
        );
    }

    #[test]
    fn refuses_empty_values_and_empty_refs() {
        assert!(parse("").is_err());
        assert!(parse("   ").is_err());
        assert!(parse("git@github.com:acme/c.git@").is_err());
        assert!(parse("https://").is_err());
        assert!(parse("git@github.com:").is_err());
        assert!(parse(".git").is_err());
    }
}
