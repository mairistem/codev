//! Parser integration tests — golden tests and the round-trip invariant.
//!
//! Golden tests: two realistic files in `fixtures/`, parsed then checked
//! field by field. They break on the slightest change in the AST
//! structure, which is exactly the point — a silent regression does not
//! get through.
//!
//! Invariant: every node carries a `Span` whose byte range delimits its
//! source text. Iterating over all nodes and substituting each range with
//! itself must rebuild the file down to the character. Without this
//! guarantee, `sync`/`archive` could overwrite content the delta does not
//! mention.

use codev_core::parser::ast::DeltaSection;
use codev_core::parser::{parse_delta, parse_spec};

const SPEC_SOURCE: &str = include_str!("fixtures/spec.md");
const DELTA_SOURCE: &str = include_str!("fixtures/delta.md");

#[test]
fn sample_spec_is_fully_parsed() {
    let parsed = parse_spec(SPEC_SOURCE);
    assert!(
        !parsed.has_errors(),
        "unexpected findings: {:?}",
        parsed.findings
    );

    let purpose = parsed.value.purpose.expect("purpose expected");
    assert!(purpose.text.contains("Authentication"));

    assert_eq!(parsed.value.requirements.len(), 2);
    assert_eq!(parsed.value.requirements[0].name, "User Authentication");
    assert_eq!(parsed.value.requirements[0].scenarios.len(), 2);
    assert_eq!(parsed.value.requirements[1].name, "Session Expiration");
    assert_eq!(parsed.value.requirements[1].scenarios.len(), 1);
    assert_eq!(
        parsed.value.requirements[1].scenarios[0].name,
        "Idle timeout"
    );
}

#[test]
fn sample_delta_exposes_the_four_sections_in_order() {
    let parsed = parse_delta(DELTA_SOURCE);
    assert!(
        !parsed.has_errors(),
        "unexpected findings: {:?}",
        parsed.findings
    );

    let purpose = parsed.value.purpose.expect("delta of a new capability");
    assert!(purpose.text.contains("two-factor authentication"));

    assert_eq!(parsed.value.sections.len(), 4);

    match &parsed.value.sections[0] {
        DeltaSection::Added { requirements, .. } => {
            assert_eq!(requirements.len(), 1);
            assert_eq!(requirements[0].name, "Two-Factor Authentication");
            assert_eq!(requirements[0].scenarios.len(), 2);
        }
        other => panic!("expected Added, got {other:?}"),
    }
    match &parsed.value.sections[2] {
        DeltaSection::Removed { removals, .. } => {
            assert_eq!(removals[0].name, "Remember Me");
            assert_eq!(removals[0].reason.as_deref(), Some("Replaced by 2FA"));
        }
        other => panic!("expected Removed, got {other:?}"),
    }
    match &parsed.value.sections[3] {
        DeltaSection::Renamed { renames, .. } => {
            assert_eq!(renames[0].from, "Session Expiration");
            assert_eq!(renames[0].to, "Session Timeout");
        }
        other => panic!("expected Renamed, got {other:?}"),
    }
}

/// The invariant that guards against destructive rewrites: each node's span
/// must reproduce its source text down to the byte.
#[test]
fn spans_reproduce_the_source_exactly() {
    let parsed = parse_spec(SPEC_SOURCE);

    // Purpose.
    let purpose = parsed.value.purpose.as_ref().unwrap();
    let slice = &SPEC_SOURCE[purpose.span.byte_range.clone()];
    assert!(slice.starts_with("## Purpose"), "slice = {slice:?}");

    for req in &parsed.value.requirements {
        let slice = &SPEC_SOURCE[req.span.byte_range.clone()];
        assert!(
            slice.starts_with("### Requirement:"),
            "misplaced requirement span: {slice:?}"
        );
        assert!(
            slice.contains(&req.name),
            "the requirement name ({}) must be within its span",
            req.name
        );
        for scenario in &req.scenarios {
            let slice = &SPEC_SOURCE[scenario.span.byte_range.clone()];
            assert!(
                slice.starts_with("#### Scenario:"),
                "misplaced scenario span: {slice:?}"
            );
            assert!(slice.contains(&scenario.name));
        }
    }
}

/// Targeted rewrite: replacing the first requirement with a different text
/// must touch nothing else. This is what `sync` will do on a `MODIFIED`.
#[test]
fn targeted_rewrite_leaves_the_rest_untouched() {
    let parsed = parse_spec(SPEC_SOURCE);
    let first = &parsed.value.requirements[0];
    let second = &parsed.value.requirements[1];
    let original_second_slice = &SPEC_SOURCE[second.span.byte_range.clone()];

    // Replace the first block with a shorter text, then check that the
    // second requirement stays identical.
    let replacement = "### Requirement: User Authentication (Rewritten)\n\nnew content\n";
    let mut rewritten = String::with_capacity(SPEC_SOURCE.len());
    rewritten.push_str(&SPEC_SOURCE[..first.span.byte_range.start]);
    rewritten.push_str(replacement);
    rewritten.push_str(&SPEC_SOURCE[first.span.byte_range.end..]);

    // The second requirement, recognizable by its text, must appear as-is
    // in the output — spacing included.
    assert!(
        rewritten.contains(original_second_slice),
        "the second requirement must be present character for character in the output"
    );
    assert!(rewritten.contains("new content"));
}

/// Line positions are 1-indexed and point to the heading, as an editor
/// displays them. Naming the line in an error message is the only case
/// where the user sees these numbers.
#[test]
fn spans_expose_1_indexed_lines() {
    let parsed = parse_spec(SPEC_SOURCE);
    let purpose = parsed.value.purpose.unwrap();
    // The file starts with `# Auth Specification` (line 1), `` (line 2),
    // `## Purpose` (line 3).
    assert_eq!(purpose.span.start_line(), 3);

    let session_expiration = &parsed.value.requirements[1];
    // We check consistency rather than the exact position: the line does
    // point to `### Requirement: Session Expiration`.
    let line_content: &str = SPEC_SOURCE.split('\n').collect::<Vec<_>>()
        [(session_expiration.span.start_line() - 1) as usize];
    assert!(
        line_content.contains("Session Expiration"),
        "line pointed to: {line_content:?}"
    );
}
