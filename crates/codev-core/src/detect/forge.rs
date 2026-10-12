//! The forge hosting a repository, read from the text of `.git/config`.
//!
//! Pure: the shell reads the file, this module only looks at its `url =`
//! lines. Reading the file rather than running `git remote -v` keeps the
//! detection free of process spawns.

/// A code forge codev knows how to integrate with.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Forge {
    GitHub,
    GitLab,
}

/// The forge of the first remote URL that names a known host.
///
/// Matches `github.com` and `gitlab.com`, and hosts whose name starts with
/// `gitlab.` (a common self-hosted naming); other hosts yield `None`.
pub fn from_git_config(text: &str) -> Option<Forge> {
    text.lines()
        .filter_map(|line| {
            let (key, value) = line.split_once('=')?;
            (key.trim() == "url").then(|| value.trim().to_ascii_lowercase())
        })
        .find_map(|url| forge_of_url(&url))
}

fn forge_of_url(url: &str) -> Option<Forge> {
    let host = host_of(url)?;
    if host == "github.com" || host.ends_with(".github.com") {
        Some(Forge::GitHub)
    } else if host == "gitlab.com" || host.starts_with("gitlab.") {
        Some(Forge::GitLab)
    } else {
        None
    }
}

/// The host of `scheme://[user@]host[:port]/path` or `user@host:path`.
fn host_of(url: &str) -> Option<&str> {
    let rest = match url.find("://") {
        Some(i) => &url[i + 3..],
        None => url,
    };
    let rest = rest.rsplit_once('@').map_or(rest, |(_, h)| h);
    let end = rest.find(['/', ':']).unwrap_or(rest.len());
    let host = &rest[..end];
    (!host.is_empty()).then_some(host)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn github_and_gitlab_remotes_are_recognized() {
        let github = "[core]\n\tbare = false\n[remote \"origin\"]\n\turl = git@github.com:acme/app.git\n\tfetch = +refs/heads/*:refs/remotes/origin/*\n";
        assert_eq!(from_git_config(github), Some(Forge::GitHub));
        let https = "[remote \"origin\"]\n\turl = https://GitHub.com/acme/app\n";
        assert_eq!(from_git_config(https), Some(Forge::GitHub));
        let gitlab =
            "[remote \"origin\"]\n\turl = ssh://git@gitlab.example.org:2222/acme/app.git\n";
        assert_eq!(from_git_config(gitlab), Some(Forge::GitLab));
        let gitlab_com = "[remote \"upstream\"]\n\turl = https://gitlab.com/acme/app.git\n";
        assert_eq!(from_git_config(gitlab_com), Some(Forge::GitLab));
    }

    #[test]
    fn no_remote_means_no_forge() {
        assert_eq!(from_git_config("[core]\n\tbare = false\n"), None);
        assert_eq!(
            from_git_config("[remote \"origin\"]\n\turl = git@bitbucket.org:acme/app.git\n"),
            None
        );
        assert_eq!(from_git_config(""), None);
    }
}
