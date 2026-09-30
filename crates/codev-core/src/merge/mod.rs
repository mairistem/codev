//! Semantic merge of a delta into a main spec, without any I/O.
//!
//! Two public entry points:
//!
//! - [`merge_into_existing`] for a capability that already has a main spec;
//! - [`build_new_spec`] for a new capability (creates the file from the
//!   delta's `## Purpose` and its `ADDED` requirements).
//!
//! Neither function touches the disk: the first produces a [`MergePlan`]
//! (a list of edits on the existing source), the second returns the full
//! content of the file to create. The shell (`codev-engine::sync`) takes
//! care of executing it.

pub mod edits;
pub mod render;

pub use edits::{Edit, apply_edits};

use std::collections::BTreeMap;

use crate::parser::ast::{Delta, DeltaSection, Spec};

/// The merge plan for **one** existing main spec.
///
/// Once computed, the caller applies the edits to the spec source; the
/// result is the new content to write (or to compare for idempotence).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MergePlan {
    pub edits: Vec<Edit>,
    /// Must the whole spec be deleted from disk?
    ///
    /// `true` when the change carries `retire_capabilities: true` and,
    /// after applying the REMOVED entries, the main spec has no
    /// requirement left. The shell routes this information: the core
    /// does not know the absolute path of the file.
    pub should_delete_spec: bool,
}

impl MergePlan {
    pub fn is_empty(&self) -> bool {
        self.edits.is_empty() && !self.should_delete_spec
    }
}

/// What can go wrong during the merge.
///
/// The variants carry a stable `code` that can be exposed in the JSON
/// contract, with the same rules as in the validator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MergeError {
    /// A `MODIFIED` targets a requirement that does not exist in the main spec.
    ModifiedTargetMissing { name: String },
    /// A `REMOVED` would leave the spec without any requirement.
    ///
    /// This requires `retire_capabilities: true` in `change.yaml`, and
    /// deleting the spec file — two things this batch does not deliver
    /// yet. We refuse rather than write a degenerate spec.
    WouldLeaveSpecWithoutRequirement { name: String },
    /// A delta targets a capability that has no main spec, but carries no
    /// `## Purpose` — which is required to create the file.
    NewCapabilityWithoutPurpose,
    /// The existing main spec has no `## Requirements` section in which to
    /// insert an `ADDED`. A rare case — it indicates a malformed spec that
    /// must be fixed by hand before syncing.
    NoRequirementsSection,
}

impl MergeError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::ModifiedTargetMissing { .. } => "modified_target_missing",
            Self::WouldLeaveSpecWithoutRequirement { .. } => "would_leave_spec_without_requirement",
            Self::NewCapabilityWithoutPurpose => "new_capability_without_purpose",
            Self::NoRequirementsSection => "no_requirements_section",
        }
    }

    pub fn message(&self) -> String {
        match self {
            Self::ModifiedTargetMissing { name } => format!(
                "MODIFIED: requirement `{name}` does not exist in the main spec — \
                 use ADDED if it is new, or fix the name"
            ),
            Self::WouldLeaveSpecWithoutRequirement { name } => format!(
                "REMOVED: removing `{name}` would leave the spec without any requirement; \
                 use `retire_capabilities: true` in `change.yaml` to retire the capability"
            ),
            Self::NewCapabilityWithoutPurpose => {
                "new capability: the delta must carry `## Purpose` to create the main spec".into()
            }
            Self::NoRequirementsSection => {
                "the main spec has no `## Requirements` section in which to insert an ADDED".into()
            }
        }
    }
}

impl std::fmt::Display for MergeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message())
    }
}

impl std::error::Error for MergeError {}

/// Builds the plan for merging a delta into an existing main spec.
///
/// Does not touch the disk. No write happens if the function returns
/// `Err` — the atomicity promised in `_codev/decisions/0001` depends on it.
pub fn merge_into_existing(
    spec_source: &str,
    spec: &Spec,
    delta: &Delta,
    retire_capabilities: bool,
) -> Result<MergePlan, MergeError> {
    let _ = spec_source; // kept for future use (cross-section validation); the AST already carries the spans
    let mut edits = Vec::new();

    // Index the spec's requirements by name, to find MODIFIED and REMOVED
    // targets in O(log n).
    let by_name: BTreeMap<&str, usize> = spec
        .requirements
        .iter()
        .enumerate()
        .map(|(idx, r)| (r.name.as_str(), idx))
        .collect();

    // Count the REMOVED entries to detect a full emptying before writing.
    let mut removed_count = 0usize;

    for section in &delta.sections {
        match section {
            DeltaSection::Modified { requirements, .. } => {
                for new_req in requirements {
                    let Some(&idx) = by_name.get(new_req.name.as_str()) else {
                        return Err(MergeError::ModifiedTargetMissing {
                            name: new_req.name.clone(),
                        });
                    };
                    let target = &spec.requirements[idx];
                    edits.push(Edit::new(
                        target.span.byte_range.clone(),
                        render::requirement(new_req),
                    ));
                }
            }
            DeltaSection::Removed { removals, .. } => {
                for removal in removals {
                    let Some(&idx) = by_name.get(removal.name.as_str()) else {
                        return Err(MergeError::ModifiedTargetMissing {
                            name: removal.name.clone(),
                        });
                    };
                    let target = &spec.requirements[idx];
                    edits.push(Edit::new(target.span.byte_range.clone(), String::new()));
                    removed_count += 1;
                }
            }
            DeltaSection::Renamed { renames, .. } => {
                for rename in renames {
                    // Find the requirement by its old name; retitle *only* the
                    // heading line, leaving the body and scenarios intact.
                    let Some(&idx) = by_name.get(rename.from.as_str()) else {
                        // Renaming a missing requirement is a silent no-op —
                        // the validator (E6, batch 2) reports it separately.
                        continue;
                    };
                    let target = &spec.requirements[idx];
                    let start = target.span.byte_range.start;
                    // End of the heading line: first `\n` after `start`.
                    let header_end = spec_source[start..]
                        .find('\n')
                        .map(|off| start + off)
                        .unwrap_or(target.span.byte_range.end);
                    let new_header = format!("### Requirement: {}", rename.to);
                    edits.push(Edit::new(start..header_end, new_header));
                }
            }
            DeltaSection::Added { requirements, .. } => {
                // Idempotence: an ADDED whose name already exists in the main
                // spec is treated as a silent no-op.
                // Re-applying the same sync must change nothing; otherwise a
                // user who reruns it when in doubt writes a duplicate.
                // An intended-but-conflicting ADDED is what the validator
                // (E4/E6, batch 2) reports upstream, with a dedicated
                // finding — not here.
                let to_add: Vec<_> = requirements
                    .iter()
                    .filter(|r| !by_name.contains_key(r.name.as_str()))
                    .collect();
                if to_add.is_empty() {
                    continue;
                }
                let end = spec
                    .requirements_section_end
                    .ok_or(MergeError::NoRequirementsSection)?;
                // Insert before `end`, preceded by a blank line if the
                // preceding character is not already one — to guarantee the
                // same spacing as between two neighboring requirements.
                let mut inserted = String::new();
                let needs_leading_blank = spec_source[..end]
                    .chars()
                    .last()
                    .map(|c| c != '\n')
                    .unwrap_or(false);
                if needs_leading_blank {
                    inserted.push('\n');
                }
                let already_blank_before = spec_source[..end].ends_with("\n\n");
                if !already_blank_before {
                    inserted.push('\n');
                }
                for (i, new_req) in to_add.iter().enumerate() {
                    if i > 0 {
                        inserted.push('\n');
                    }
                    inserted.push_str(&render::requirement(new_req));
                }
                edits.push(Edit::new(end..end, inserted));
            }
        }
    }

    let would_empty = removed_count > 0 && removed_count >= spec.requirements.len();
    if would_empty && !retire_capabilities {
        // Name of the first REMOVED as the hook of the message — the user
        // then knows where to start.
        let first = delta
            .sections
            .iter()
            .find_map(|s| match s {
                DeltaSection::Removed { removals, .. } => removals.first(),
                _ => None,
            })
            .map(|r| r.name.clone())
            .unwrap_or_default();
        return Err(MergeError::WouldLeaveSpecWithoutRequirement { name: first });
    }

    Ok(MergePlan {
        edits,
        should_delete_spec: would_empty && retire_capabilities,
    })
}

/// Returns the full content of a **new** main spec from a new-capability
/// delta.
///
/// Requires a `## Purpose` — without it, we would refuse to write a file
/// with a placeholder nobody has approved.
pub fn build_new_spec(capability_path: &str, delta: &Delta) -> Result<String, MergeError> {
    let purpose = delta
        .purpose
        .as_ref()
        .ok_or(MergeError::NewCapabilityWithoutPurpose)?;

    let title = render::spec_title_from_capability(capability_path);
    let mut out = String::new();
    out.push_str("# ");
    out.push_str(&title);
    out.push_str(" Specification\n\n");
    out.push_str("## Purpose\n\n");
    out.push_str(purpose.text.trim());
    out.push_str("\n\n");
    out.push_str("## Requirements\n");

    // The ADDED entries (and only those) make up the initial body: REMOVED,
    // MODIFIED and RENAMED make no sense for a blank capability, and are
    // silently ignored — the validator would have reported them.
    for section in &delta.sections {
        if let DeltaSection::Added { requirements, .. } = section {
            for req in requirements {
                out.push('\n');
                out.push_str(&render::requirement(req));
            }
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{parse_delta, parse_spec};

    fn parse_pair(spec_source: &str, delta_source: &str) -> (Spec, Delta) {
        let spec = parse_spec(spec_source);
        assert!(
            !spec.has_errors(),
            "spec source must be valid: {:?}",
            spec.findings
        );
        let delta = parse_delta(delta_source);
        assert!(
            !delta.has_errors(),
            "delta source must be valid: {:?}",
            delta.findings
        );
        (spec.value, delta.value)
    }

    fn merged(spec_source: &str, delta_source: &str) -> String {
        let (spec, delta) = parse_pair(spec_source, delta_source);
        let plan = merge_into_existing(spec_source, &spec, &delta, false).unwrap();
        apply_edits(spec_source, &plan.edits)
    }

    // ─────────────── MODIFIED ───────────────

    #[test]
    fn modified_replaces_the_block() {
        let spec_source = "## Purpose\n\nx.\n\n## Requirements\n\n### Requirement: Login\nThe system SHALL emit a token.\n\n#### Scenario: OK\n- **WHEN** login\n- **THEN** token\n";
        let delta_source = "## MODIFIED Requirements\n\n### Requirement: Login\nThe system MUST emit a JWT token.\n\n#### Scenario: OK\n- **WHEN** login\n- **THEN** JWT\n";
        let out = merged(spec_source, delta_source);
        assert!(out.contains("The system MUST emit a JWT token"));
        assert!(!out.contains("SHALL emit a token"));
        // Purpose intact.
        assert!(out.contains("## Purpose\n\nx.\n"));
    }

    #[test]
    fn modified_without_target_fails() {
        let spec_source = "## Purpose\n\nx.\n\n## Requirements\n\n### Requirement: Login\nThe system SHALL x.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n";
        let delta_source = "## MODIFIED Requirements\n\n### Requirement: Ghost\nThe system SHALL y.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n";
        let (spec, delta) = parse_pair(spec_source, delta_source);
        let err = merge_into_existing(spec_source, &spec, &delta, false).unwrap_err();
        assert_eq!(err.code(), "modified_target_missing");
        assert!(err.to_string().contains("Ghost"));
    }

    // ─────────────── REMOVED ───────────────

    #[test]
    fn removed_deletes_the_block_and_its_spacing() {
        // Two requirements: the first is removed, the second stays identical
        // to the character since the first one's span includes the spacing
        // separating it from the second.
        let spec_source = "## Purpose\n\nx.\n\n## Requirements\n\n### Requirement: Login\nThe system SHALL x.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n\n### Requirement: Session\nThe system SHALL y.\n\n#### Scenario: T\n- **WHEN** c\n- **THEN** d\n";
        let delta_source = "## REMOVED Requirements\n\n### Requirement: Login\n**Reason**: obsolete\n**Migration**: none\n";
        let out = merged(spec_source, delta_source);
        assert!(!out.contains("Login"));
        assert!(out.contains("### Requirement: Session"));
    }

    #[test]
    fn removing_last_requirement_fails_without_flag() {
        let spec_source = "## Purpose\n\nx.\n\n## Requirements\n\n### Requirement: Solo\nThe system SHALL x.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n";
        let delta_source = "## REMOVED Requirements\n\n### Requirement: Solo\n**Reason**: retire\n**Migration**: none\n";
        let (spec, delta) = parse_pair(spec_source, delta_source);
        let err = merge_into_existing(spec_source, &spec, &delta, false).unwrap_err();
        assert_eq!(err.code(), "would_leave_spec_without_requirement");
        // Updated message — points to retire_capabilities.
        assert!(err.to_string().contains("retire_capabilities"));
    }

    #[test]
    fn full_removal_with_flag_retires_the_capability() {
        // Same context as the refusal, but with `retire_capabilities: true`
        // → plan `should_delete_spec` set to `true`, edits applied.
        let spec_source = "## Purpose\n\nx.\n\n## Requirements\n\n### Requirement: Solo\nThe system SHALL x.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n";
        let delta_source = "## REMOVED Requirements\n\n### Requirement: Solo\n**Reason**: retire\n**Migration**: none\n";
        let (spec, delta) = parse_pair(spec_source, delta_source);
        let plan = merge_into_existing(spec_source, &spec, &delta, true).unwrap();
        assert!(plan.should_delete_spec);
    }

    #[test]
    fn partial_removal_with_flag_does_not_delete() {
        // One requirement removed out of two: the flag is a silent no-op.
        let spec_source = "## Purpose\n\nx.\n\n## Requirements\n\n### Requirement: Login\nThe system SHALL x.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n\n### Requirement: Logout\nThe system SHALL y.\n\n#### Scenario: T\n- **WHEN** c\n- **THEN** d\n";
        let delta_source = "## REMOVED Requirements\n\n### Requirement: Login\n**Reason**: obsolete\n**Migration**: none\n";
        let (spec, delta) = parse_pair(spec_source, delta_source);
        let plan = merge_into_existing(spec_source, &spec, &delta, true).unwrap();
        assert!(
            !plan.should_delete_spec,
            "Logout remains, the capability is not retired"
        );
        assert!(!plan.edits.is_empty(), "the REMOVED Login is applied");
    }

    // ─────────────── RENAMED ───────────────

    #[test]
    fn renamed_only_touches_the_heading() {
        let spec_source = "## Purpose\n\nx.\n\n## Requirements\n\n### Requirement: Session Expiration\nThe system MUST expire.\n\n#### Scenario: Idle\n- **WHEN** idle\n- **THEN** expire\n";
        let delta_source =
            "## RENAMED Requirements\n\n- FROM: Session Expiration\n- TO: Session Timeout\n";
        let out = merged(spec_source, delta_source);
        assert!(out.contains("### Requirement: Session Timeout"));
        assert!(!out.contains("### Requirement: Session Expiration"));
        // The body is intact.
        assert!(out.contains("The system MUST expire."));
        assert!(out.contains("#### Scenario: Idle"));
    }

    // ─────────────── ADDED ───────────────

    #[test]
    fn added_is_inserted_after_the_last_requirement() {
        let spec_source = "## Purpose\n\nx.\n\n## Requirements\n\n### Requirement: Login\nThe system SHALL x.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n";
        let delta_source = "## ADDED Requirements\n\n### Requirement: Session\nThe system SHALL y.\n\n#### Scenario: T\n- **WHEN** c\n- **THEN** d\n";
        let out = merged(spec_source, delta_source);
        let login_pos = out.find("### Requirement: Login").unwrap();
        let session_pos = out.find("### Requirement: Session").unwrap();
        assert!(
            login_pos < session_pos,
            "the ADDED comes after the existing one"
        );
    }

    #[test]
    fn added_precedes_a_following_free_section() {
        // ## Notes after ## Requirements: the ADDED must be inserted BEFORE it.
        let spec_source = "## Purpose\n\nx.\n\n## Requirements\n\n### Requirement: Login\nThe system SHALL x.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n\n## Notes\n\nBonus.\n";
        let delta_source = "## ADDED Requirements\n\n### Requirement: Session\nThe system SHALL y.\n\n#### Scenario: T\n- **WHEN** c\n- **THEN** d\n";
        let out = merged(spec_source, delta_source);
        let session_pos = out.find("### Requirement: Session").unwrap();
        let notes_pos = out.find("## Notes").unwrap();
        assert!(
            session_pos < notes_pos,
            "the ADDED is inserted BEFORE the following free section"
        );
        // Notes stays intact.
        assert!(out.contains("## Notes\n\nBonus.\n"));
    }

    // ─────────────── build_new_spec ───────────────

    #[test]
    fn build_new_spec_with_purpose_and_added() {
        let delta_source = "## Purpose\n\nHandles authentication.\n\n## ADDED Requirements\n\n### Requirement: Login\nThe system SHALL x.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n";
        let delta = parse_delta(delta_source).value;
        let content = build_new_spec("user-auth", &delta).unwrap();

        assert!(content.starts_with("# User Auth Specification\n\n"));
        assert!(content.contains("## Purpose\n\nHandles authentication.\n"));
        assert!(content.contains("## Requirements\n"));
        assert!(content.contains("### Requirement: Login"));
    }

    #[test]
    fn build_new_spec_without_purpose_fails() {
        let delta_source = "## ADDED Requirements\n\n### Requirement: X\nThe system SHALL x.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n";
        let delta = parse_delta(delta_source).value;
        let err = build_new_spec("x", &delta).unwrap_err();
        assert_eq!(err.code(), "new_capability_without_purpose");
    }

    // ─────────────── invariants ───────────────

    #[test]
    fn added_with_already_present_name_is_silently_ignored() {
        // Re-application case: the spec already contains `Login`, the delta
        // asks for `ADDED: Login`. It is skipped to preserve idempotence.
        let spec_source = "## Purpose\n\nx.\n\n## Requirements\n\n### Requirement: Login\nThe system SHALL x.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n";
        let delta_source = "## ADDED Requirements\n\n### Requirement: Login\nThe system SHALL x.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n";
        let out = merged(spec_source, delta_source);
        assert_eq!(out, spec_source, "already present ADDED = no-op");
    }

    #[test]
    fn sync_is_idempotent_after_two_passes() {
        // Apply the same delta twice — the second pass must return exactly
        // the same content as the first.
        let spec_source = "## Purpose\n\nx.\n\n## Requirements\n\n### Requirement: Login\nThe system SHALL x.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n";
        let delta_source = "## MODIFIED Requirements\n\n### Requirement: Login\nThe system MUST y.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n";

        let after_one = merged(spec_source, delta_source);
        let after_two = merged(&after_one, delta_source);
        assert_eq!(after_one, after_two, "sync must be idempotent");
    }

    #[test]
    fn edit_construction_is_stable() {
        let edit = Edit::new(0..5, "abc");
        assert_eq!(edit.byte_range, 0..5);
        assert_eq!(edit.replacement, "abc");
    }
}
