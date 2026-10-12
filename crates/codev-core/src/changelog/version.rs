//! Versions, tags and dates: the few rules `codev changelog` needs to pick
//! the default range and to check `--release` and `--date`.

use std::cmp::Ordering;

/// A version as tags spell it: numeric components, and an optional
/// pre-release suffix that sorts before the release itself.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Version {
    numbers: Vec<u64>,
    pre: Option<String>,
}

impl Version {
    fn parse(raw: &str) -> Option<Self> {
        let (core, pre) = match raw.split_once('-') {
            Some((core, pre)) if !pre.is_empty() => (core, Some(pre.to_string())),
            Some(_) => return None,
            None => (raw, None),
        };
        let numbers = core
            .split('.')
            .map(|n| {
                (!n.is_empty() && n.bytes().all(|b| b.is_ascii_digit()))
                    .then(|| n.parse::<u64>().ok())
                    .flatten()
            })
            .collect::<Option<Vec<u64>>>()?;
        Some(Self { numbers, pre })
    }

    fn cmp(&self, other: &Self) -> Ordering {
        let len = self.numbers.len().max(other.numbers.len());
        let at = |v: &Self, i: usize| v.numbers.get(i).copied().unwrap_or(0);
        (0..len)
            .map(|i| at(self, i).cmp(&at(other, i)))
            .find(|o| o.is_ne())
            .unwrap_or_else(|| match (&self.pre, &other.pre) {
                (None, None) => Ordering::Equal,
                (None, Some(_)) => Ordering::Greater,
                (Some(_), None) => Ordering::Less,
                (Some(a), Some(b)) => a.cmp(b),
            })
    }
}

/// The version of a `v*` tag, `None` for any other tag.
fn tag_version(tag: &str) -> Option<Version> {
    tag.strip_prefix('v').and_then(Version::parse)
}

/// The default `--since`: the highest `v*` tag by version order. When
/// `until` is itself a version tag, only tags strictly lower count — the
/// release notes of `v1.2.0` start after `v1.1.x`, whatever was tagged
/// since. Tags that are not versions are ignored.
pub fn default_since(tags: &[String], until: Option<&str>) -> Option<String> {
    let ceiling = until.and_then(tag_version);
    tags.iter()
        .filter_map(|tag| tag_version(tag).map(|v| (tag, v)))
        .filter(|(_, v)| ceiling.as_ref().is_none_or(|c| v.cmp(c) == Ordering::Less))
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .map(|(tag, _)| tag.clone())
}

/// `--release`: `X.Y.Z` with an optional pre-release suffix; a leading `v`
/// is dropped, since Keep a Changelog headings spell versions without it.
pub fn normalize_release(raw: &str) -> Result<String, String> {
    let bare = raw.trim().strip_prefix('v').unwrap_or(raw.trim());
    match Version::parse(bare) {
        Some(v) if v.numbers.len() == 3 => Ok(bare.to_string()),
        _ => Err(format!(
            "`{raw}` is not a version; use `X.Y.Z`, for example `1.2.0`"
        )),
    }
}

/// `--date`: `YYYY-MM-DD`, with a plausible month and day.
pub fn is_valid_date(raw: &str) -> bool {
    let parts: Vec<&str> = raw.split('-').collect();
    let [y, m, d] = parts.as_slice() else {
        return false;
    };
    let digits = |s: &str, n: usize| s.len() == n && s.bytes().all(|b| b.is_ascii_digit());
    if !(digits(y, 4) && digits(m, 2) && digits(d, 2)) {
        return false;
    }
    let (m, d): (u32, u32) = (m.parse().unwrap_or(0), d.parse().unwrap_or(0));
    (1..=12).contains(&m) && (1..=31).contains(&d)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tags(names: &[&str]) -> Vec<String> {
        names.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn default_since_is_the_highest_version_tag() {
        // Version order, not string order: v0.10.0 is above v0.9.0.
        let t = tags(&["v0.9.0", "v0.10.0", "v0.2.1"]);
        assert_eq!(default_since(&t, None).as_deref(), Some("v0.10.0"));
        assert_eq!(default_since(&t, Some("HEAD")).as_deref(), Some("v0.10.0"));
    }

    #[test]
    fn default_since_is_below_a_version_until() {
        let t = tags(&["v0.9.0", "v0.10.0", "v0.11.0-rc.1", "v0.11.0"]);
        assert_eq!(
            default_since(&t, Some("v0.10.0")).as_deref(),
            Some("v0.9.0")
        );
        // A pre-release sorts before its release.
        assert_eq!(
            default_since(&t, Some("v0.11.0")).as_deref(),
            Some("v0.11.0-rc.1")
        );
    }

    #[test]
    fn default_since_ignores_non_version_tags() {
        let t = tags(&["latest", "vnext", "v1.0.0", "release-2"]);
        assert_eq!(default_since(&t, None).as_deref(), Some("v1.0.0"));
    }

    #[test]
    fn no_tag_means_no_since() {
        assert_eq!(default_since(&[], None), None);
        assert_eq!(default_since(&tags(&["v1.0.0"]), Some("v1.0.0")), None);
    }

    #[test]
    fn release_versions_are_normalized_or_refused() {
        assert_eq!(normalize_release("1.2.0").as_deref(), Ok("1.2.0"));
        assert_eq!(normalize_release("v1.2.0").as_deref(), Ok("1.2.0"));
        assert_eq!(normalize_release("1.2.0-rc.1").as_deref(), Ok("1.2.0-rc.1"));
        for bad in ["next", "1.2", "1.2.x", "", "1.2.0-"] {
            assert!(normalize_release(bad).is_err(), "{bad}");
        }
    }

    #[test]
    fn dates_are_checked() {
        assert!(is_valid_date("2026-10-20"));
        for bad in ["2026-13-01", "26-10-20", "2026/10/20", "2026-10-2", "today"] {
            assert!(!is_valid_date(bad), "{bad}");
        }
    }
}
