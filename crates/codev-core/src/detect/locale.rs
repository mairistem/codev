//! Language detection from POSIX locale variables.
//!
//! `codev init` proposes the language in which skills write artifact prose
//! (proposal, design, tasks, specs). The best local signal is the user's
//! locale: `LC_ALL`, then `LC_MESSAGES`, then `LANG` — the precedence POSIX
//! applies to message catalogs. Reading the variables is the shell's job;
//! this module only interprets their values.

/// A language inferred from a locale variable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DetectedLocale {
    /// The variable the value came from, e.g. `LANG`.
    pub var: String,
    /// The raw value, e.g. `fr_FR.UTF-8`.
    pub value: String,
    /// The ISO 639 language code, e.g. `fr`.
    pub language: String,
}

/// Locale variables, in precedence order.
pub const LOCALE_VARS: &[&str] = &["LC_ALL", "LC_MESSAGES", "LANG"];

/// Picks the first locale variable that names a real language.
///
/// `lookup` returns the value of a variable, if set. Empty values and the
/// `C` / `POSIX` locales carry no language preference and are skipped, so a
/// `LC_ALL=C` set for scripting does not hide a meaningful `LANG`.
pub fn detect(lookup: impl Fn(&str) -> Option<String>) -> Option<DetectedLocale> {
    LOCALE_VARS.iter().find_map(|var| {
        let value = lookup(var)?;
        let language = language_from_locale(&value)?;
        Some(DetectedLocale {
            var: (*var).to_string(),
            value,
            language,
        })
    })
}

/// Extracts the language code from a locale string.
///
/// `fr_FR.UTF-8` → `fr`, `en_US` → `en`, `de` → `de`, `pt_BR@euro` → `pt`.
/// Returns `None` for `C`, `POSIX`, `C.UTF-8`, or anything that is not a
/// valid language code.
pub fn language_from_locale(locale: &str) -> Option<String> {
    let head = locale
        .split(['.', '@'])
        .next()
        .unwrap_or_default()
        .split(['_', '-'])
        .next()
        .unwrap_or_default()
        .to_ascii_lowercase();
    if head == "c" || head == "posix" {
        return None;
    }
    is_valid_language_code(&head).then_some(head)
}

/// Whether `code` is an acceptable value for the `language:` key.
///
/// Accepts an ISO 639 language code (two or three lowercase ASCII letters),
/// optionally followed by one subtag such as a region (`pt-BR`, `zh-Hant`).
/// Deliberately stricter than full BCP 47: the value ends up in agent
/// instructions, where a short, unambiguous code is all that is needed.
pub fn is_valid_language_code(code: &str) -> bool {
    let mut parts = code.splitn(2, '-');
    let primary = parts.next().unwrap_or_default();
    let primary_ok =
        (2..=3).contains(&primary.len()) && primary.bytes().all(|b| b.is_ascii_lowercase());
    let subtag_ok = parts.next().is_none_or(|sub| {
        (2..=8).contains(&sub.len()) && sub.bytes().all(|b| b.is_ascii_alphanumeric())
    });
    primary_ok && subtag_ok
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_language_from_common_locales() {
        assert_eq!(language_from_locale("fr_FR.UTF-8").as_deref(), Some("fr"));
        assert_eq!(language_from_locale("en_US").as_deref(), Some("en"));
        assert_eq!(language_from_locale("de").as_deref(), Some("de"));
        assert_eq!(language_from_locale("pt_BR@euro").as_deref(), Some("pt"));
    }

    #[test]
    fn neutral_locales_carry_no_language() {
        assert_eq!(language_from_locale("C"), None);
        assert_eq!(language_from_locale("C.UTF-8"), None);
        assert_eq!(language_from_locale("POSIX"), None);
        assert_eq!(language_from_locale(""), None);
    }

    #[test]
    fn detect_follows_posix_precedence_and_skips_neutral_values() {
        let env = |var: &str| match var {
            "LC_ALL" => Some("C".to_string()),
            "LANG" => Some("fr_FR.UTF-8".to_string()),
            _ => None,
        };
        let found = detect(env).expect("LANG names a language");
        assert_eq!(found.var, "LANG");
        assert_eq!(found.value, "fr_FR.UTF-8");
        assert_eq!(found.language, "fr");
    }

    #[test]
    fn detect_returns_none_without_any_locale() {
        assert_eq!(detect(|_| None), None);
    }

    #[test]
    fn validates_language_codes() {
        for ok in ["en", "fr", "fil", "pt-BR", "zh-Hant"] {
            assert!(is_valid_language_code(ok), "{ok} should be valid");
        }
        for bad in ["", "e", "EN", "english", "fr_FR", "pt-", "en-US-x"] {
            assert!(!is_valid_language_code(bad), "{bad} should be invalid");
        }
    }
}
