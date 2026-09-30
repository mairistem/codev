//! Stable codes emitted by the `validate` rules.
//!
//! Kept apart from the parser codes to make provenance obvious: a
//! `code` of `spec_purpose_missing` comes from the parser, a `code` of
//! `requirement_no_shall` comes from here.

/// A requirement whose description contains neither `SHALL` nor `MUST`.
pub const REQUIREMENT_NO_SHALL: &str = "requirement_no_shall";

/// A requirement without any scenario.
pub const REQUIREMENT_NO_SCENARIO: &str = "requirement_no_scenario";

/// A main spec without any extractable requirement.
pub const SPEC_NO_REQUIREMENT: &str = "spec_no_requirement";

/// The same requirement appears in two `ADDED`/`MODIFIED`/`REMOVED` sections
/// of the same delta.
pub const CROSS_SECTION_CONFLICT: &str = "cross_section_conflict";

/// A `RENAMED.TO` coincides with an `ADDED` of the same name in the same delta.
pub const RENAME_TARGET_COLLISION: &str = "rename_target_collision";

/// A `MODIFIED` references a `RENAMED.FROM` — the new name must be
/// used instead.
pub const MODIFIED_USES_OLD_NAME: &str = "modified_uses_old_name";

/// A change with no delta at all and no `skip_specs: true` in its metadata.
pub const ZERO_DELTA_WITHOUT_MARKER: &str = "zero_delta_without_marker";

/// `skip_specs: true` is declared, but delta files exist.
pub const SKIP_SPECS_CONFLICT: &str = "skip_specs_conflict";
