use std::fmt;

use crate::error::{CoreError, Result};

/// Change identifier, in strict kebab-case.
///
/// A leading digit is allowed: it lets changes be prefixed to
/// order or stagger them (`100-add-billing`, `00001-add-auth`).
///
/// The identifier is used as a directory name on three different file
/// systems, one of which is case-insensitive (macOS): hence the rejection of
/// uppercase letters, which would make `Add-Auth` and `add-auth` indistinguishable there and
/// distinct on a Linux machine.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ChangeId(String);

impl ChangeId {
    pub fn parse(raw: &str) -> Result<Self> {
        let reason = kebab_violation(raw);
        match reason {
            Some(reason) => Err(CoreError::InvalidChangeId {
                raw: raw.to_string(),
                reason,
            }),
            None => Ok(Self(raw.to_string())),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ChangeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl AsRef<str> for ChangeId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// True if `raw` is an acceptable kebab-case identifier.
pub fn is_kebab_case(raw: &str) -> bool {
    kebab_violation(raw).is_none()
}

/// Returns the first violated rule, phrased to be read by a human.
///
/// A boolean would not be enough: "invalid name" without saying which rule
/// was violated forces the user to guess.
fn kebab_violation(raw: &str) -> Option<String> {
    if raw.is_empty() {
        return Some("it is empty".into());
    }
    if raw.starts_with('-') || raw.ends_with('-') {
        return Some("it starts or ends with a hyphen".into());
    }
    if raw.contains("--") {
        return Some("it contains two consecutive hyphens".into());
    }
    if raw.contains(' ') {
        return Some("it contains a space; use hyphens".into());
    }
    if raw.contains('_') {
        return Some("it contains an underscore; use hyphens".into());
    }
    if raw.chars().any(|c| c.is_ascii_uppercase()) {
        return Some("it contains an uppercase letter; use lowercase only".into());
    }
    if let Some(bad) = raw
        .chars()
        .find(|c| !(c.is_ascii_lowercase() || c.is_ascii_digit() || *c == '-'))
    {
        return Some(format!(
            "it contains the character `{bad}`; only lowercase letters, digits and hyphens are allowed"
        ));
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_kebab_case() {
        for ok in [
            "add-auth",
            "fix-bug",
            "a",
            "100-add-feature",
            "00001-add-auth",
            "v2-migration",
        ] {
            assert!(ChangeId::parse(ok).is_ok(), "{ok} should be accepted");
        }
    }

    #[test]
    fn rejects_and_explains() {
        let cases = [
            ("", "empty"),
            ("-lead", "hyphen"),
            ("trail-", "hyphen"),
            ("double--hyphen", "consecutive"),
            ("with space", "space"),
            ("with_underscore", "underscore"),
            ("AddAuth", "uppercase"),
            ("caf\u{e9}", "character"),
        ];
        for (raw, expected) in cases {
            let err = ChangeId::parse(raw)
                .err()
                .unwrap_or_else(|| panic!("`{raw}` should be rejected"));
            let message = err.to_string();
            assert!(
                message.contains(expected),
                "`{raw}`: message `{message}` should mention `{expected}`"
            );
        }
    }

    #[test]
    fn error_code_is_stable() {
        let err = ChangeId::parse("Nope").unwrap_err();
        assert_eq!(err.code(), "invalid_change_id");
    }
}
