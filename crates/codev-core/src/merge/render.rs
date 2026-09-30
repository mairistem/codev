//! Markdown rendering of the blocks handled by the merge.
//!
//! A strict canonical form, so that two successive syncs yield the same
//! content down to the character. Blocks are written in a canonical format
//! — without trying to mimic the spacing of a neighboring existing spec,
//! which would be a source of silent drift.

use crate::parser::ast::{Requirement, Scenario};

/// Rendering of a complete requirement.
///
/// Format:
///
/// ```text
/// ### Requirement: <name>
///
/// <description>
///
/// #### Scenario: <name>
///
/// <body>
///
/// #### Scenario: <name>
///
/// <body>
/// ```
///
/// The last line ends without an extra blank line — it is the caller
/// (typically `merge::edits`) that adjusts the separating space with the
/// next block.
pub fn requirement(req: &Requirement) -> String {
    let mut out = String::new();
    out.push_str("### Requirement: ");
    out.push_str(&req.name);
    out.push('\n');

    if !req.description.trim().is_empty() {
        out.push('\n');
        out.push_str(req.description.trim_end());
        out.push('\n');
    }

    for scenario in &req.scenarios {
        out.push('\n');
        out.push_str(&scenario_str(scenario));
    }

    out
}

fn scenario_str(scenario: &Scenario) -> String {
    let mut out = String::new();
    out.push_str("#### Scenario: ");
    out.push_str(&scenario.name);
    out.push('\n');
    if !scenario.body.trim().is_empty() {
        out.push('\n');
        out.push_str(scenario.body.trim_end());
        out.push('\n');
    }
    out
}

/// Title of a main spec derived from the capability path.
///
/// `identity/user-auth` → `User Auth`. We don't try to preserve the full
/// path in the title — the directory name is enough, and the file lives at
/// the location that disambiguates it.
pub fn spec_title_from_capability(capability_path: &str) -> String {
    let last = capability_path
        .rsplit('/')
        .next()
        .unwrap_or(capability_path);
    last.split('-')
        .filter(|s| !s.is_empty())
        .map(title_case)
        .collect::<Vec<_>>()
        .join(" ")
}

fn title_case(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().chain(chars).collect(),
        None => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast::Span;

    fn span() -> Span {
        Span::new(0..0, 1..1)
    }

    #[test]
    fn render_requirement_is_stable() {
        let req = Requirement {
            name: "Login".into(),
            description: "The system SHALL emit a token.".into(),
            scenarios: vec![Scenario {
                name: "OK".into(),
                body: "- **WHEN** login\n- **THEN** token".into(),
                span: span(),
            }],
            span: span(),
        };
        let expected = "### Requirement: Login\n\n\
                       The system SHALL emit a token.\n\n\
                       #### Scenario: OK\n\n\
                       - **WHEN** login\n\
                       - **THEN** token\n";
        assert_eq!(requirement(&req), expected);
    }

    #[test]
    fn render_with_two_scenarios() {
        let req = Requirement {
            name: "Auth".into(),
            description: "The system MUST authenticate.".into(),
            scenarios: vec![
                Scenario {
                    name: "OK".into(),
                    body: "- **WHEN** valid\n- **THEN** ok".into(),
                    span: span(),
                },
                Scenario {
                    name: "KO".into(),
                    body: "- **WHEN** invalid\n- **THEN** ko".into(),
                    span: span(),
                },
            ],
            span: span(),
        };
        let rendered = requirement(&req);
        assert!(rendered.contains("#### Scenario: OK"));
        assert!(rendered.contains("#### Scenario: KO"));
        // One blank line between the two scenarios.
        assert!(rendered.contains("- **THEN** ok\n\n#### Scenario: KO"));
    }

    #[test]
    fn title_from_flat_capability() {
        assert_eq!(spec_title_from_capability("user-auth"), "User Auth");
    }

    #[test]
    fn title_from_nested_capability() {
        // Take the last segment — the full path stays in the directory,
        // no need to duplicate it in the title.
        assert_eq!(
            spec_title_from_capability("identity/user-auth"),
            "User Auth"
        );
    }

    #[test]
    fn title_with_empty_segment_is_robust() {
        assert_eq!(spec_title_from_capability("--x"), "X");
        assert_eq!(spec_title_from_capability(""), "");
    }
}
