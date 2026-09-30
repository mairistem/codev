//! Merge integration tests — on the core's pure function.
//!
//! No `FileSystem`, no engine: parse the spec and the delta, compute the
//! `MergePlan`, apply it. That is what keeps these tests fast and
//! readable; the invariants they guard are what protects `sync` and
//! `archive` from a destructive rewrite.

use codev_core::merge::{apply_edits, merge_into_existing};
use codev_core::parser::{parse_delta, parse_spec};

const SPEC_BEFORE: &str = include_str!("fixtures/sync/spec_before.md");

fn merge(spec_source: &str, delta_source: &str) -> String {
    let spec = parse_spec(spec_source);
    assert!(
        !spec.has_errors(),
        "spec must be valid: {:?}",
        spec.findings
    );
    let delta = parse_delta(delta_source);
    assert!(
        !delta.has_errors(),
        "delta must be valid: {:?}",
        delta.findings
    );
    let plan = merge_into_existing(spec_source, &spec.value, &delta.value, false)
        .expect("merge must succeed");
    apply_edits(spec_source, &plan.edits)
}

#[test]
fn added_adds() {
    let delta = "## ADDED Requirements\n\n### Requirement: Two-Factor\nThe system MUST support 2FA.\n\n#### Scenario: Enrolment\n- **WHEN** user enables\n- **THEN** QR code shown\n";
    let out = merge(SPEC_BEFORE, delta);
    // New block present.
    assert!(out.contains("### Requirement: Two-Factor"));
    // It comes AFTER the existing one, BEFORE the Notes section.
    let login_pos = out.find("### Requirement: Login").unwrap();
    let twofactor_pos = out.find("### Requirement: Two-Factor").unwrap();
    let notes_pos = out.find("## Notes").unwrap();
    assert!(login_pos < twofactor_pos);
    assert!(twofactor_pos < notes_pos);
}

#[test]
fn modified_replaces() {
    let delta = "## MODIFIED Requirements\n\n### Requirement: Session Expiration\nThe system MUST expire sessions after 15 minutes.\n\n#### Scenario: Idle timeout\n- **WHEN** 15 minutes pass without activity\n- **THEN** the session is invalidated\n";
    let out = merge(SPEC_BEFORE, delta);
    assert!(out.contains("15 minutes"));
    assert!(!out.contains("30 minutes"));
    // Login untouched.
    assert!(out.contains("### Requirement: Login"));
    assert!(out.contains("The system SHALL emit a token upon successful login."));
}

#[test]
fn removed_deletes() {
    let delta = "## REMOVED Requirements\n\n### Requirement: Session Expiration\n**Reason**: obsolete\n**Migration**: replaced by 2FA\n";
    let out = merge(SPEC_BEFORE, delta);
    assert!(!out.contains("### Requirement: Session Expiration"));
    // Login remains.
    assert!(out.contains("### Requirement: Login"));
}

#[test]
fn renamed_retitle() {
    let delta = "## RENAMED Requirements\n\n- FROM: Session Expiration\n- TO: Session Timeout\n";
    let out = merge(SPEC_BEFORE, delta);
    assert!(out.contains("### Requirement: Session Timeout"));
    assert!(!out.contains("### Requirement: Session Expiration"));
    // The requirement body is intact down to the character.
    assert!(out.contains("The system MUST expire sessions after 30 minutes."));
    assert!(out.contains("- **WHEN** 30 minutes pass without activity"));
}

#[test]
fn multi_ops_is_deterministic() {
    // A delta spanning several operations: ADDED + MODIFIED. The result
    // must be stable and consistent.
    let delta = "## ADDED Requirements\n\n### Requirement: Two-Factor\nThe system MUST support 2FA.\n\n#### Scenario: Enrolment\n- **WHEN** user enables\n- **THEN** QR code shown\n\n## MODIFIED Requirements\n\n### Requirement: Session Expiration\nThe system MUST expire sessions after 15 minutes.\n\n#### Scenario: Idle timeout\n- **WHEN** 15 minutes pass without activity\n- **THEN** the session is invalidated\n";
    let out = merge(SPEC_BEFORE, delta);
    assert!(out.contains("Two-Factor"));
    assert!(out.contains("15 minutes"));
    // A second pass gives the same result.
    let out2 = merge(&out, delta);
    assert_eq!(out, out2, "multi-ops must be idempotent");
}

#[test]
fn a_free_form_section_after_requirements_survives() {
    // This is the invariant that guarantees unmentioned content is
    // preserved: the `## Notes` section must stay intact down to the character.
    let delta = "## MODIFIED Requirements\n\n### Requirement: Login\nThe system SHALL emit a JWT.\n\n#### Scenario: Valid credentials\n- **WHEN** the user submits valid credentials\n- **THEN** a JWT is returned\n";
    let out = merge(SPEC_BEFORE, delta);
    assert!(out.contains("## Notes\n\nFree-form notes that must survive any sync.\n"));
}

#[test]
fn html_comment_in_untouched_requirement_survives() {
    // A requirement contains an HTML comment in its text. If a MODIFIED
    // touches another requirement, the comment must stay intact.
    let spec_with_comment = "## Purpose\n\nCap.\n\n## Requirements\n\n### Requirement: R1\nThe system SHALL x.\n<!-- internal note to preserve -->\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n\n### Requirement: R2\nThe system SHALL y.\n\n#### Scenario: T\n- **WHEN** c\n- **THEN** d\n";
    let delta = "## MODIFIED Requirements\n\n### Requirement: R2\nThe system MUST z.\n\n#### Scenario: T\n- **WHEN** c\n- **THEN** d\n";
    let out = merge(spec_with_comment, delta);
    assert!(out.contains("<!-- internal note to preserve -->"));
}

#[test]
fn two_successive_syncs_give_the_same_result() {
    // The idempotency invariant, on a richer case.
    let delta = "## MODIFIED Requirements\n\n### Requirement: Login\nThe system SHALL emit a JWT token.\n\n#### Scenario: Valid credentials\n- **WHEN** the user submits valid credentials\n- **THEN** a JWT token is returned\n";
    let after_one = merge(SPEC_BEFORE, delta);
    let after_two = merge(&after_one, delta);
    assert_eq!(after_one, after_two);
}
