//! The `## Domains` section of a proposal: which of a closed list of
//! domains the change touches. The experts of the role catalog are
//! activated by them (see [`crate::roles`]).
//!
//! Pure: a `&str` in, the ticked domains and their located defects out.

use crate::parser::ast::{Finding, Parsed};
use crate::parser::section::{heading_line, section};

/// The domains a proposal can tick, in the template's order.
pub const DOMAINS: &[&str] = &[
    "security",
    "personal-data",
    "user-interface",
    "performance",
    "operations",
    "migration",
];

/// The heading of the section, matched case-insensitively.
pub const HEADING: &str = "## Domains";

/// Stable codes of the domain findings.
pub mod codes {
    /// A bullet naming a domain outside [`super::DOMAINS`].
    pub const UNKNOWN_DOMAIN: &str = "unknown_domain";
    /// A line that is neither a bullet nor `None`, or `None` next to a
    /// ticked domain.
    pub const DOMAINS_INVALID: &str = "domains_invalid";
    /// An active change's proposal without a `## Domains` section.
    pub const DOMAINS_MISSING: &str = "domains_missing";

    /// The codes a validation report can carry, for the uniqueness test.
    pub const ALL: &[&str] = &[UNKNOWN_DOMAIN, DOMAINS_INVALID, DOMAINS_MISSING];
}

/// What a proposal says about its domains.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Domains {
    /// No `## Domains` section.
    Absent,
    /// The ticked domains, known ones only, in the order written. Empty for
    /// `None` or a list with nothing ticked.
    Listed(Vec<String>),
}

impl Domains {
    /// The ticked domains; empty when the section is absent.
    pub fn ticked(&self) -> &[String] {
        match self {
            Self::Absent => &[],
            Self::Listed(domains) => domains,
        }
    }
}

/// Parses the `## Domains` section of `proposal`.
///
/// `- [x] security` and `- security` tick a domain, `- [ ] security`
/// leaves it unticked; text after the domain name is a free comment.
pub fn parse(proposal: &str) -> Parsed<Domains> {
    let Some(lines) = section(proposal, HEADING) else {
        return Parsed::new(Domains::Absent);
    };
    let mut ticked: Vec<String> = Vec::new();
    let mut findings = Vec::new();
    let mut none_line = None;

    for (line, raw) in lines {
        let text = raw.trim();
        if text.is_empty() {
            continue;
        }
        let Some(item) = text.strip_prefix("- ").or_else(|| text.strip_prefix("* ")) else {
            if is_none(text) {
                none_line.get_or_insert(line);
            } else {
                findings.push(Finding::error(
                    codes::DOMAINS_INVALID,
                    line,
                    format!("line {line}: `{text}` is neither a `- [x] <domain>` item nor `None`"),
                ));
            }
            continue;
        };
        let (is_ticked, rest) = checkbox(item.trim_start());
        let domain = first_word(rest);
        if domain.is_empty() {
            findings.push(Finding::error(
                codes::DOMAINS_INVALID,
                line,
                format!(
                    "line {line}: the item names no domain; use one of {}",
                    known()
                ),
            ));
        } else if !DOMAINS.contains(&domain) {
            findings.push(Finding::error(
                codes::UNKNOWN_DOMAIN,
                line,
                format!(
                    "line {line}: `{domain}` is not a domain; use one of {}",
                    known()
                ),
            ));
        } else if is_ticked && !ticked.iter().any(|d| d == domain) {
            ticked.push(domain.to_string());
        }
    }

    if let Some(line) = none_line
        && !ticked.is_empty()
    {
        findings.push(Finding::error(
            codes::DOMAINS_INVALID,
            line,
            format!(
                "line {line}: `None` cannot be combined with ticked domains; keep one or the other"
            ),
        ));
    }
    Parsed {
        value: Domains::Listed(ticked),
        findings,
    }
}

/// The 1-based line of the section, for a finding about it.
pub fn heading(proposal: &str) -> Option<u32> {
    heading_line(proposal, HEADING)
}

/// Splits an item's checkbox off: `[x] security` is ticked, `[ ] security`
/// is not, a plain `security` is ticked.
fn checkbox(item: &str) -> (bool, &str) {
    if let Some(rest) = item
        .strip_prefix("[x]")
        .or_else(|| item.strip_prefix("[X]"))
    {
        return (true, rest);
    }
    if let Some(rest) = item.strip_prefix("[ ]") {
        return (false, rest);
    }
    (true, item)
}

/// The domain name: the first word, without backticks or a trailing
/// colon or comma.
fn first_word(text: &str) -> &str {
    text.split_whitespace()
        .next()
        .unwrap_or("")
        .trim_end_matches([':', ','])
        .trim_matches('`')
}

fn is_none(text: &str) -> bool {
    matches!(text.trim_end_matches('.'), "None" | "none" | "NONE")
}

fn known() -> String {
    DOMAINS
        .iter()
        .map(|d| format!("`{d}`"))
        .collect::<Vec<_>>()
        .join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn proposal(domains: &str) -> String {
        format!(
            "# Proposal: X\n\n## Impact\n\n- code\n\n## Domains\n\n{domains}\n\n## Release note\n\nNone\n"
        )
    }

    fn codes_of(parsed: &Parsed<Domains>) -> Vec<&'static str> {
        parsed.findings.iter().map(|f| f.code).collect()
    }

    #[test]
    fn ticked_and_plain_bullets_list_their_domain() {
        let parsed = parse(&proposal(
            "- [x] security\n- [ ] performance\n- personal-data — new e-mail field\n* [X] `migration`: renamed key",
        ));
        assert!(parsed.findings.is_empty(), "{:?}", parsed.findings);
        assert_eq!(
            parsed.value.ticked(),
            ["security", "personal-data", "migration"]
        );
    }

    #[test]
    fn none_and_unticked_lists_mean_no_domain() {
        for body in ["None", "None.", "- [ ] security\n- [ ] operations"] {
            let parsed = parse(&proposal(body));
            assert!(parsed.findings.is_empty(), "{body}: {:?}", parsed.findings);
            assert_eq!(parsed.value, Domains::Listed(Vec::new()), "{body}");
        }
    }

    #[test]
    fn the_template_comment_is_ignored() {
        let parsed = parse(&proposal(
            "<!-- Tick every domain the change touches,\n     or write None. -->\n- [x] operations",
        ));
        assert!(parsed.findings.is_empty(), "{:?}", parsed.findings);
        assert_eq!(parsed.value.ticked(), ["operations"]);
    }

    #[test]
    fn an_unknown_domain_is_an_error_with_its_line() {
        let source = proposal("- [x] securty\n- [ ] perf");
        let parsed = parse(&source);
        assert_eq!(codes_of(&parsed), ["unknown_domain", "unknown_domain"]);
        let first = &parsed.findings[0];
        assert_eq!(first.line, 9);
        assert!(first.message.contains("securty"), "{}", first.message);
        assert!(
            first.message.contains("`personal-data`"),
            "{}",
            first.message
        );
        assert!(parsed.value.ticked().is_empty());
    }

    #[test]
    fn none_with_a_ticked_domain_is_invalid() {
        let parsed = parse(&proposal("None\n- [x] security"));
        assert_eq!(codes_of(&parsed), ["domains_invalid"]);
        assert_eq!(parsed.findings[0].line, 9);
    }

    #[test]
    fn a_stray_line_is_invalid() {
        let parsed = parse(&proposal("Security mostly.\n- [ ]"));
        assert_eq!(codes_of(&parsed), ["domains_invalid", "domains_invalid"]);
    }

    #[test]
    fn a_missing_section_is_absent() {
        let parsed = parse("# Proposal: X\n\n## Why\n\nx\n");
        assert_eq!(parsed.value, Domains::Absent);
        assert!(parsed.findings.is_empty());
        assert_eq!(heading("# Proposal: X\n\n## Domains\n\nNone\n"), Some(3));
    }

    #[test]
    fn a_heading_inside_a_fence_is_not_the_section() {
        let parsed = parse("# Proposal: X\n\n```\n## Domains\n- [x] securty\n```\n");
        assert_eq!(parsed.value, Domains::Absent);
    }
}
