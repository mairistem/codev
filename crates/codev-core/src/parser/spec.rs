//! Parser for a main spec: `## Purpose`, `## Requirements`, with its
//! `### Requirement:` entries and their `#### Scenario:` entries.
//!
//! Line by line, a single pass over the source, with the precomputed fence
//! mask. See `design.md` of `parse-specs-and-deltas` for the rationale.

use super::ast::{Finding, Parsed, PurposeBlock, Requirement, Spec};
use super::codes;
use super::fence;
use super::shared::{
    ScannedLine, block_span, collect_scenarios, is_delta_header, line_starts,
    requirement_from_lines, section_header, trim_body,
};

pub fn parse_spec(source: &str) -> Parsed<Spec> {
    let lines: Vec<&str> = source.split('\n').collect();
    let mask = fence::build_fence_mask(source);
    let starts = line_starts(source);

    let mut findings = Vec::new();
    let mut purpose: Option<PurposeBlock> = None;
    let mut requirements: Vec<Requirement> = Vec::new();

    // Locate the top-level `##` sections, outside literal zones. We work on
    // `ScannedLine` to carry the 1-indexed line and the byte offset.
    let scanned: Vec<ScannedLine> = (0..lines.len())
        .map(|i| ScannedLine {
            text: lines[i],
            line_number: (i as u32) + 1,
            byte_offset: starts[i],
            literal: mask[i],
        })
        .collect();

    // Locate the top-level sections.
    #[derive(Debug)]
    struct H2 {
        title: String,
        header_index: usize,
    }
    let mut h2: Vec<H2> = Vec::new();
    for (i, entry) in scanned.iter().enumerate() {
        if entry.literal {
            continue;
        }
        if let Some(title) = section_header(entry.text) {
            h2.push(H2 {
                title,
                header_index: i,
            });
        }
    }

    // Detect delta headings that strayed into a main spec.
    for entry in &scanned {
        if entry.literal {
            continue;
        }
        if is_delta_header(entry.text) {
            findings.push(Finding::error(
                codes::DELTA_HEADER_IN_MAIN_SPEC,
                entry.line_number,
                format!(
                    "line {}: `{}` only belongs in change files; \
                     remove it from the main spec",
                    entry.line_number,
                    entry.text.trim()
                ),
            ));
        }
    }

    // Purpose: first top-level `## Purpose`.
    let purpose_idx = h2
        .iter()
        .position(|h| h.title.eq_ignore_ascii_case("Purpose"));
    if let Some(idx) = purpose_idx {
        let start_line = h2[idx].header_index;
        let end_line = h2
            .get(idx + 1)
            .map(|next| next.header_index)
            .unwrap_or(lines.len());
        let body = trim_body(&scanned[start_line + 1..end_line]);
        let span = block_span(&scanned, start_line, end_line);
        purpose = Some(PurposeBlock { text: body, span });
    } else {
        findings.push(Finding::error(
            codes::SPEC_PURPOSE_MISSING,
            1,
            "the main spec has no `## Purpose` section",
        ));
    }

    // Requirements: first `## Requirements` — requirements outside this
    // section are reported.
    let requirements_idx = h2
        .iter()
        .position(|h| h.title.eq_ignore_ascii_case("Requirements"));

    // `requirements_section_end`: insertion point for ADDED entries, in
    // bytes. It is the start of the next top-level `##` section after
    // `## Requirements`, or the length of the source if none follows.
    let requirements_section_end = requirements_idx.map(|idx| {
        h2.get(idx + 1)
            .map(|next| scanned[next.header_index].byte_offset)
            .unwrap_or(source.len())
    });

    let (req_start, req_end) = match requirements_idx {
        Some(idx) => {
            let start = h2[idx].header_index;
            let end = h2
                .get(idx + 1)
                .map(|next| next.header_index)
                .unwrap_or(lines.len());
            (Some(start), end)
        }
        None => (None, lines.len()),
    };

    // Locate all requirements in the source (even outside the section) so
    // that stray ones can be reported.
    let requirement_positions = super::shared::find_requirement_positions(&scanned);

    for pos in &requirement_positions {
        let inside_requirements = match req_start {
            Some(start) => pos.header_index > start && pos.header_index < req_end,
            None => false,
        };
        if !inside_requirements {
            findings.push(Finding::error(
                codes::REQUIREMENT_OUTSIDE_SECTION,
                pos.line_number,
                format!(
                    "line {}: `### Requirement:` outside `## Requirements`",
                    pos.line_number
                ),
            ));
        }
    }

    // Extract the valid requirements.
    if let Some(start) = req_start {
        let inside: Vec<_> = requirement_positions
            .iter()
            .filter(|p| p.header_index > start && p.header_index < req_end)
            .collect();
        for (rank, pos) in inside.iter().enumerate() {
            let next_start = inside
                .get(rank + 1)
                .map(|next| next.header_index)
                .unwrap_or(req_end);
            let (requirement, sub_findings) = requirement_from_lines(
                &scanned,
                pos.header_index,
                next_start,
                pos.name.clone(),
                &collect_scenarios,
            );
            findings.extend(sub_findings);
            requirements.push(requirement);
        }
    }

    Parsed {
        value: Spec {
            purpose,
            requirements,
            requirements_section_end,
        },
        findings,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast::Severity;

    fn purpose_only() -> &'static str {
        "# Auth Specification\n\n## Purpose\n\nAuthentication for the app.\n\n## Requirements\n\n### Requirement: Session Expiration\nThe system MUST expire sessions after inactivity.\n\n#### Scenario: Idle\n\n- **WHEN** the user is idle\n- **THEN** the session expires\n"
    }

    #[test]
    fn extracts_purpose_and_a_requirement_with_scenario() {
        let parsed = parse_spec(purpose_only());
        assert!(!parsed.has_errors(), "findings: {:?}", parsed.findings);

        let purpose = parsed.value.purpose.expect("purpose expected");
        assert!(purpose.text.contains("Authentication"));

        assert_eq!(parsed.value.requirements.len(), 1);
        let req = &parsed.value.requirements[0];
        assert_eq!(req.name, "Session Expiration");
        assert_eq!(req.scenarios.len(), 1);
        assert_eq!(req.scenarios[0].name, "Idle");
    }

    #[test]
    fn missing_purpose_is_a_located_finding() {
        let source = "## Requirements\n\n### Requirement: X\nThe system SHALL x.\n\n#### Scenario: Y\n- **WHEN** a\n- **THEN** b\n";
        let parsed = parse_spec(source);
        assert!(
            parsed
                .findings
                .iter()
                .any(|f| f.code == "spec_purpose_missing" && f.severity == Severity::Error)
        );
        // The requirement remains extractable.
        assert_eq!(parsed.value.requirements.len(), 1);
    }

    #[test]
    fn requirement_outside_section_is_reported() {
        // Requirement placed BEFORE `## Requirements`.
        let source = "## Purpose\n\nx\n\n### Requirement: Stray\nThe system MUST y.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n\n## Requirements\n";
        let parsed = parse_spec(source);
        let f = parsed
            .findings
            .iter()
            .find(|f| f.code == "requirement_outside_section")
            .expect("finding expected");
        assert!(f.message.contains("outside"), "{}", f.message);
    }

    #[test]
    fn delta_header_in_main_spec_is_reported() {
        let source = "## Purpose\n\nx\n\n## ADDED Requirements\n\n### Requirement: X\nThe system SHALL x.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n";
        let parsed = parse_spec(source);
        let f = parsed
            .findings
            .iter()
            .find(|f| f.code == "delta_header_in_main_spec")
            .expect("finding expected");
        assert!(f.message.contains("ADDED"), "{}", f.message);
    }

    #[test]
    fn requirement_inside_fence_is_ignored() {
        // The trap: a `### Requirement: Example` in a code block is not a
        // real requirement, and therefore does not appear in the result.
        let source =
            "## Purpose\n\nExample:\n\n```\n### Requirement: Fake\n```\n\n## Requirements\n";
        let parsed = parse_spec(source);
        assert!(parsed.value.requirements.is_empty());
        // And above all no "requirement_outside_section" finding: masking
        // must happen first.
        assert!(
            !parsed
                .findings
                .iter()
                .any(|f| f.code == "requirement_outside_section")
        );
    }

    #[test]
    fn requirements_section_end_is_before_following_free_section() {
        // The Requirements section is followed by a `## Notes`: the
        // insertion point for ADDED entries must be just before that
        // section, not at the end of the file.
        let source = "## Purpose\n\nx.\n\n## Requirements\n\n### Requirement: R\nThe system SHALL r.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n\n## Notes\n\nBonus.\n";
        let parsed = parse_spec(source);
        let end = parsed
            .value
            .requirements_section_end
            .expect("Requirements present");
        assert_eq!(
            &source[end..end + 8],
            "## Notes",
            "the insertion must land right at the start of the next section"
        );
    }

    #[test]
    fn requirements_section_end_goes_to_the_end_without_next_section() {
        let source = "## Purpose\n\nx.\n\n## Requirements\n\n### Requirement: R\nThe system SHALL r.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n";
        let parsed = parse_spec(source);
        assert_eq!(
            parsed.value.requirements_section_end,
            Some(source.len()),
            "without a next section, it goes to the end of the file"
        );
    }

    #[test]
    fn three_hash_scenario_is_reported() {
        // `### Scenario:` instead of `#### Scenario:`: this is the silent
        // failure we refuse. The requirement still appears, without this
        // scenario.
        let source = "## Purpose\n\nx\n\n## Requirements\n\n### Requirement: R\nThe system SHALL r.\n\n### Scenario: Fake\n- **WHEN** a\n- **THEN** b\n";
        let parsed = parse_spec(source);
        assert!(
            parsed
                .findings
                .iter()
                .any(|f| f.code == "scenario_wrong_heading_level")
        );
        assert_eq!(parsed.value.requirements.len(), 1);
        assert!(parsed.value.requirements[0].scenarios.is_empty());
    }
}
