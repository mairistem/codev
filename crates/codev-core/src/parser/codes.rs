//! Stable codes of the `Finding`s emitted by the parser.
//!
//! Naming each code once — rather than writing it as a literal at the
//! emission sites — serves two purposes:
//!
//! 1. The `validate` module can iterate over this list to check that its
//!    own codes create no duplicates.
//! 2. A rename has to go through this constant and its documentation,
//!    as a reminder that the code is a public contract.

/// The main spec contains no `## Purpose`.
pub const SPEC_PURPOSE_MISSING: &str = "spec_purpose_missing";

/// A `### Requirement:` appears outside the `## Requirements` section.
pub const REQUIREMENT_OUTSIDE_SECTION: &str = "requirement_outside_section";

/// A delta heading (`## ADDED Requirements`, etc.) appears in a main
/// spec.
pub const DELTA_HEADER_IN_MAIN_SPEC: &str = "delta_header_in_main_spec";

/// A scenario is written with three hashes (`### Scenario:`) instead of
/// four.
pub const SCENARIO_WRONG_HEADING_LEVEL: &str = "scenario_wrong_heading_level";

/// Two requirements with the same name in the same section of a delta.
pub const DUPLICATE_REQUIREMENT: &str = "duplicate_requirement";

// ─────────────────────────── decision parser codes ───────────────────────────

/// The decision file has no YAML frontmatter delimited by `---`.
pub const DECISION_MISSING_FRONTMATTER: &str = "decision_missing_frontmatter";

/// The frontmatter lacks a required field (`id`, `title`, `status`,
/// `date`), or carries an unknown key.
pub const DECISION_MISSING_FIELD: &str = "decision_missing_field";

/// The `status` carries a value that is not one of the five recognized ones.
pub const DECISION_UNKNOWN_STATUS: &str = "decision_unknown_status";

// The codes below are emitted by the engine-side index, not by the
// parser. They are registered here to keep a single point of uniqueness
// checked by `validate::finding_codes_are_unique`.

/// A `supersedes` points to an identifier missing from the index.
pub const DECISION_SUPERSEDES_UNKNOWN: &str = "decision_supersedes_unknown";

/// The same identifier appears in the project and in an inherited source.
pub const DECISION_ID_COLLISION: &str = "decision_id_collision";

/// A supersession chain forms a cycle — no decision in the cycle takes
/// effect.
pub const DECISION_SUPERSESSION_CYCLE: &str = "decision_supersession_cycle";

/// A typed frontmatter field (`deviates_from`, `tags`…) carries a value
/// of the wrong shape (for example a string instead of a list).
pub const DECISION_FIELD_TYPE_MISMATCH: &str = "decision_field_type_mismatch";

/// A local ADR's `deviates_from` points to a `qualified-id` that is no
/// longer present in the index (source removed, SHA moved, id changed).
pub const DECISION_DANGLING_DEVIATION: &str = "decision_dangling_deviation";

/// Two local `accepted` ADRs reference the same target in their
/// `deviates_from` — the tool does not arbitrate and refuses.
pub const DECISION_CONFLICTING_DEVIATIONS: &str = "decision_conflicting_deviations";

/// All parser codes, in order of first appearance.
///
/// The `validate` module runs a uniqueness check on them in its tests; a
/// new rule whose code would collide is rejected before it is even
/// written.
pub const ALL: &[&str] = &[
    SPEC_PURPOSE_MISSING,
    REQUIREMENT_OUTSIDE_SECTION,
    DELTA_HEADER_IN_MAIN_SPEC,
    SCENARIO_WRONG_HEADING_LEVEL,
    DUPLICATE_REQUIREMENT,
    DECISION_MISSING_FRONTMATTER,
    DECISION_MISSING_FIELD,
    DECISION_UNKNOWN_STATUS,
    DECISION_SUPERSEDES_UNKNOWN,
    DECISION_ID_COLLISION,
    DECISION_SUPERSESSION_CYCLE,
    DECISION_FIELD_TYPE_MISMATCH,
    DECISION_DANGLING_DEVIATION,
    DECISION_CONFLICTING_DEVIATIONS,
];
