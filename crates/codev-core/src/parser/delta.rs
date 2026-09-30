//! Parser for a delta: `## ADDED | MODIFIED | REMOVED | RENAMED Requirements`.

use std::collections::BTreeMap;

use super::ast::{
    Delta, DeltaSection, Finding, Parsed, PurposeBlock, Removal, Rename, Requirement, Span,
};
use super::fence;
use super::shared::{
    DeltaKind, ScannedLine, block_span, collect_scenarios, delta_section_kind, line_starts,
    requirement_from_lines, section_header, trim_body,
};

pub fn parse_delta(source: &str) -> Parsed<Delta> {
    let lines: Vec<&str> = source.split('\n').collect();
    let mask = fence::build_fence_mask(source);
    let starts = line_starts(source);

    let scanned: Vec<ScannedLine> = (0..lines.len())
        .map(|i| ScannedLine {
            text: lines[i],
            line_number: (i as u32) + 1,
            byte_offset: starts[i],
            literal: mask[i],
        })
        .collect();

    let mut findings = Vec::new();

    // Locate all top-level `##` sections (Purpose + delta).
    #[derive(Debug)]
    struct H2 {
        kind: SectionKind,
        header_index: usize,
    }
    #[derive(Debug)]
    enum SectionKind {
        Purpose,
        Delta(DeltaKind),
        Other,
    }

    let mut h2: Vec<H2> = Vec::new();
    for (i, entry) in scanned.iter().enumerate() {
        if entry.literal {
            continue;
        }
        if let Some(kind) = delta_section_kind(entry.text) {
            h2.push(H2 {
                kind: SectionKind::Delta(kind),
                header_index: i,
            });
            continue;
        }
        if let Some(title) = section_header(entry.text) {
            let kind = if title.eq_ignore_ascii_case("Purpose") {
                SectionKind::Purpose
            } else {
                SectionKind::Other
            };
            h2.push(H2 {
                kind,
                header_index: i,
            });
        }
    }

    // Optional Purpose — the "new capability only" semantics are the
    // validator's business, not the parser's.
    let mut purpose: Option<PurposeBlock> = None;
    if let Some(idx) = h2
        .iter()
        .position(|h| matches!(h.kind, SectionKind::Purpose))
    {
        let start_line = h2[idx].header_index;
        let end_line = h2
            .get(idx + 1)
            .map(|next| next.header_index)
            .unwrap_or(lines.len());
        let text = trim_body(&scanned[start_line + 1..end_line]);
        let span = block_span(&scanned, start_line, end_line);
        purpose = Some(PurposeBlock { text, span });
    }

    // Rebuild the delta sections.
    let mut sections: Vec<DeltaSection> = Vec::new();
    for (i, h) in h2.iter().enumerate() {
        let SectionKind::Delta(kind) = &h.kind else {
            continue;
        };
        let start = h.header_index;
        let end = h2
            .get(i + 1)
            .map(|next| next.header_index)
            .unwrap_or(lines.len());
        let span = block_span(&scanned, start, end);
        findings.extend(unexpected_headings(&scanned, start + 1, end, *kind));

        match kind {
            DeltaKind::Added | DeltaKind::Modified => {
                let (requirements, section_findings) =
                    parse_requirement_blocks(&scanned, start + 1, end, kind_label(*kind));
                findings.extend(section_findings);
                sections.push(match kind {
                    DeltaKind::Added => DeltaSection::Added { requirements, span },
                    DeltaKind::Modified => DeltaSection::Modified { requirements, span },
                    _ => unreachable!(),
                });
            }
            DeltaKind::Removed => {
                let removals = parse_removals(&scanned, start + 1, end);
                sections.push(DeltaSection::Removed { removals, span });
            }
            DeltaKind::Renamed => {
                let renames = parse_renames(&scanned, start + 1, end);
                sections.push(DeltaSection::Renamed { renames, span });
            }
        }
    }

    for section in &sections {
        let entries = match section {
            DeltaSection::Added { requirements, .. }
            | DeltaSection::Modified { requirements, .. } => requirements.len(),
            DeltaSection::Removed { removals, .. } => removals.len(),
            DeltaSection::Renamed { renames, .. } => renames.len(),
        };
        if entries == 0 {
            let line = section.span().start_line();
            let expected = match section {
                DeltaSection::Renamed { .. } => "a `FROM:` / `TO:` pair",
                _ => "a `### Requirement: <name>` block",
            };
            findings.push(Finding::warning(
                super::codes::DELTA_SECTION_EMPTY,
                line,
                format!(
                    "line {line}: `## {} Requirements` has no entry, so nothing of it \
                     will be merged; add {expected}, or remove the section",
                    kind_label(section_kind(section))
                ),
            ));
        }
    }

    Parsed {
        value: Delta { purpose, sections },
        findings,
    }
}

fn section_kind(section: &DeltaSection) -> DeltaKind {
    match section {
        DeltaSection::Added { .. } => DeltaKind::Added,
        DeltaSection::Modified { .. } => DeltaKind::Modified,
        DeltaSection::Removed { .. } => DeltaKind::Removed,
        DeltaSection::Renamed { .. } => DeltaKind::Renamed,
    }
}

/// Reports every `###` heading of a delta section that the merge would not
/// understand.
///
/// The keywords are English whatever the artifact language: a
/// `### Exigence :` would otherwise be skipped silently, leaving the
/// section empty and the validation green. `### Scenario:` is left to
/// `scenario_wrong_heading_level`, which already names the fix.
fn unexpected_headings(
    scanned: &[ScannedLine],
    start: usize,
    end: usize,
    kind: DeltaKind,
) -> Vec<Finding> {
    let mut findings = Vec::new();
    for entry in &scanned[start..end.min(scanned.len())] {
        if entry.literal {
            continue;
        }
        let trimmed = entry.text.trim_start();
        let Some(title) = trimmed.strip_prefix("### ") else {
            continue;
        };
        let title = title.trim();
        let is_requirement = title.starts_with("Requirement:");
        let expected = match kind {
            DeltaKind::Renamed => {
                // RENAMED takes `FROM:` / `TO:` lines, no heading at all.
                "`- FROM:` / `- TO:` lines, not headings"
            }
            _ if is_requirement => continue,
            _ if title.starts_with("Scenario:") => continue,
            _ => {
                "`### Requirement: <name>` — the keyword stays in English whatever the artifact language"
            }
        };
        findings.push(Finding::error(
            super::codes::DELTA_UNEXPECTED_HEADING,
            entry.line_number,
            format!(
                "line {}: `### {title}` is not understood in `## {} Requirements`; \
                 expected {expected}",
                entry.line_number,
                kind_label(kind)
            ),
        ));
    }
    findings
}

fn kind_label(k: DeltaKind) -> &'static str {
    match k {
        DeltaKind::Added => "ADDED",
        DeltaKind::Modified => "MODIFIED",
        DeltaKind::Removed => "REMOVED",
        DeltaKind::Renamed => "RENAMED",
    }
}

/// Gathers the `### Requirement:` blocks of a given range into complete
/// `Requirement`s, with duplicate detection.
fn parse_requirement_blocks(
    scanned: &[ScannedLine],
    start: usize,
    end: usize,
    section_label: &'static str,
) -> (Vec<Requirement>, Vec<Finding>) {
    let mut findings = Vec::new();
    let all = super::shared::find_requirement_positions_in_range(scanned, start, end);
    let mut requirements = Vec::with_capacity(all.len());

    let mut seen: BTreeMap<String, u32> = BTreeMap::new();
    for (rank, pos) in all.iter().enumerate() {
        if let Some(previous_line) = seen.insert(pos.name.clone(), pos.line_number) {
            findings.push(Finding::error(
                super::codes::DUPLICATE_REQUIREMENT,
                pos.line_number,
                format!(
                    "{section_label} section: duplicate requirement `{}` (first seen at line {previous_line}, redefined at line {})",
                    pos.name, pos.line_number
                ),
            ));
        }
        let next_start = all.get(rank + 1).map(|p| p.header_index).unwrap_or(end);
        let (requirement, sub_findings) = requirement_from_lines(
            scanned,
            pos.header_index,
            next_start,
            pos.name.clone(),
            &collect_scenarios,
        );
        findings.extend(sub_findings);
        requirements.push(requirement);
    }
    (requirements, findings)
}

fn parse_removals(scanned: &[ScannedLine], start: usize, end: usize) -> Vec<Removal> {
    let positions = super::shared::find_requirement_positions_in_range(scanned, start, end);
    let mut removals = Vec::with_capacity(positions.len());
    for (rank, pos) in positions.iter().enumerate() {
        let next_start = positions
            .get(rank + 1)
            .map(|p| p.header_index)
            .unwrap_or(end);
        let body = &scanned[pos.header_index + 1..next_start];
        let reason = find_labelled_line(body, "Reason");
        let migration = find_labelled_line(body, "Migration");
        let span = block_span(scanned, pos.header_index, next_start);
        removals.push(Removal {
            name: pos.name.clone(),
            reason,
            migration,
            span,
        });
    }
    removals
}

/// Looks for a "**Label**: …" line in a block, tolerant of whitespace.
fn find_labelled_line(lines: &[ScannedLine], label: &str) -> Option<String> {
    let needle_star = format!("**{label}**:");
    let needle_bare = format!("{label}:");
    for entry in lines {
        if entry.literal {
            continue;
        }
        let trimmed = entry.text.trim_start();
        for needle in [&needle_star, &needle_bare] {
            if let Some(rest) = trimmed.strip_prefix(needle) {
                return Some(rest.trim().to_string());
            }
        }
    }
    None
}

fn parse_renames(scanned: &[ScannedLine], start: usize, end: usize) -> Vec<Rename> {
    let mut renames = Vec::new();
    let mut current_from: Option<(String, usize)> = None;
    for i in start..end {
        let entry = &scanned[i];
        if entry.literal {
            continue;
        }
        let trimmed = entry.text.trim_start();

        // A "- FROM: X" line is accepted just like "FROM: X": both forms
        // come naturally when typing, and the validator normalizes.
        let payload = trimmed
            .strip_prefix("- ")
            .or_else(|| trimmed.strip_prefix("* "))
            .unwrap_or(trimmed);

        if let Some(rest) = payload.strip_prefix("FROM:") {
            current_from = Some((rename_target(rest), i));
        } else if let Some(rest) = payload.strip_prefix("TO:")
            && let Some((from, from_line)) = current_from.take()
        {
            let span = span_across(scanned, from_line, i + 1);
            renames.push(Rename {
                from,
                to: rename_target(rest),
                span,
            });
        }
    }
    renames
}

/// The requirement name after `FROM:` or `TO:`.
///
/// Two spellings are accepted: the bare name, and the full heading in
/// backticks — `` `### Requirement: <name>` ``, the form the template
/// shows. Both must name the same requirement, otherwise a rename copied
/// from the template would silently match nothing.
fn rename_target(raw: &str) -> String {
    let trimmed = raw.trim();
    let unquoted = trimmed
        .strip_prefix('`')
        .and_then(|rest| rest.strip_suffix('`'))
        .unwrap_or(trimmed)
        .trim();
    unquoted
        .strip_prefix("### Requirement:")
        .unwrap_or(unquoted)
        .trim()
        .to_string()
}

fn span_across(scanned: &[ScannedLine], start_line: usize, end_line: usize) -> Span {
    let start_byte = scanned[start_line].byte_offset;
    let end_byte = scanned
        .get(end_line)
        .map(|e| e.byte_offset)
        .unwrap_or_else(|| {
            let last = scanned.last().expect("at least one line");
            last.byte_offset + last.text.len()
        });
    let start_ln = scanned[start_line].line_number;
    let end_ln = scanned
        .get(end_line)
        .map(|e| e.line_number)
        .unwrap_or(start_ln + (end_line - start_line) as u32);
    Span::new(start_byte..end_byte, start_ln..end_ln)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast::DeltaOp;

    #[test]
    fn recognizes_the_four_sections() {
        let source = "## ADDED Requirements\n\n### Requirement: A\nThe system SHALL a.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n\n## MODIFIED Requirements\n\n### Requirement: B\nThe system SHALL b.\n\n#### Scenario: T\n- **WHEN** c\n- **THEN** d\n\n## REMOVED Requirements\n\n### Requirement: C\n**Reason**: obsolete\n**Migration**: use D\n\n## RENAMED Requirements\n\n- FROM: Old\n- TO: New\n";
        let parsed = parse_delta(source);
        assert!(!parsed.has_errors(), "findings: {:?}", parsed.findings);
        let ops: Vec<DeltaOp> = parsed.value.sections.iter().map(|s| s.op()).collect();
        assert_eq!(
            ops,
            vec![
                DeltaOp::Added,
                DeltaOp::Modified,
                DeltaOp::Removed,
                DeltaOp::Renamed
            ]
        );
    }

    #[test]
    fn added_block_carries_requirement_and_scenario() {
        let source = "## ADDED Requirements\n\n### Requirement: Two-Factor Authentication\nThe system MUST support TOTP.\n\n#### Scenario: Enrolment\n- **WHEN** a user enables 2FA\n- **THEN** a QR code is displayed\n";
        let parsed = parse_delta(source);
        let DeltaSection::Added { requirements, .. } = &parsed.value.sections[0] else {
            panic!("expected Added");
        };
        assert_eq!(requirements.len(), 1);
        assert_eq!(requirements[0].name, "Two-Factor Authentication");
        assert_eq!(requirements[0].scenarios.len(), 1);
        assert_eq!(requirements[0].scenarios[0].name, "Enrolment");
    }

    #[test]
    fn removed_block_carries_reason_and_migration() {
        let source = "## REMOVED Requirements\n\n### Requirement: Remember Me\n**Reason**: obsolete\n**Migration**: use 2FA\n";
        let parsed = parse_delta(source);
        let DeltaSection::Removed { removals, .. } = &parsed.value.sections[0] else {
            panic!("expected Removed");
        };
        assert_eq!(removals.len(), 1);
        assert_eq!(removals[0].name, "Remember Me");
        assert_eq!(removals[0].reason.as_deref(), Some("obsolete"));
        assert_eq!(removals[0].migration.as_deref(), Some("use 2FA"));
    }

    #[test]
    fn renamed_block_pairs_from_and_to() {
        let source = "## RENAMED Requirements\n\n- FROM: Old Name\n- TO: New Name\n";
        let parsed = parse_delta(source);
        let DeltaSection::Renamed { renames, .. } = &parsed.value.sections[0] else {
            panic!("expected Renamed");
        };
        assert_eq!(renames.len(), 1);
        assert_eq!(renames[0].from, "Old Name");
        assert_eq!(renames[0].to, "New Name");
    }

    #[test]
    fn renamed_accepts_the_backticked_heading_of_the_template() {
        let source = "## RENAMED Requirements\n\n- FROM: `### Requirement: Old Name`\n- TO: `### Requirement: New Name`\n";
        let parsed = parse_delta(source);
        let DeltaSection::Renamed { renames, .. } = &parsed.value.sections[0] else {
            panic!("expected Renamed");
        };
        assert_eq!(renames.len(), 1);
        assert_eq!(renames[0].from, "Old Name");
        assert_eq!(renames[0].to, "New Name");
    }

    #[test]
    fn translated_requirement_heading_is_reported() {
        let source = "## ADDED Requirements\n\n### Exigence : Connexion\nLe système DOIT x.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n";
        let parsed = parse_delta(source);
        let heading = parsed
            .findings
            .iter()
            .find(|f| f.code == "delta_unexpected_heading")
            .expect("the translated heading must be reported");
        assert_eq!(heading.severity, crate::parser::ast::Severity::Error);
        assert_eq!(heading.line, 3);
        assert!(
            heading.message.contains("### Exigence : Connexion"),
            "{}",
            heading.message
        );
        // The section it leaves empty is reported too.
        assert!(
            parsed
                .findings
                .iter()
                .any(|f| f.code == "delta_section_empty"
                    && f.severity == crate::parser::ast::Severity::Warning)
        );
    }

    #[test]
    fn well_formed_sections_report_no_heading_or_emptiness() {
        let source = "## ADDED Requirements\n\n### Requirement: A\nThe system SHALL a.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n\n## RENAMED Requirements\n\n- FROM: Old\n- TO: New\n";
        let parsed = parse_delta(source);
        assert!(parsed.findings.is_empty(), "{:?}", parsed.findings);
    }

    #[test]
    fn heading_in_renamed_section_is_reported() {
        let source = "## RENAMED Requirements\n\n### Requirement: Old\n";
        let parsed = parse_delta(source);
        assert!(
            parsed
                .findings
                .iter()
                .any(|f| f.code == "delta_unexpected_heading")
        );
        assert!(
            parsed
                .findings
                .iter()
                .any(|f| f.code == "delta_section_empty")
        );
    }

    #[test]
    fn three_hash_scenario_is_reported_once() {
        // `### Scenario:` keeps its dedicated code, without a second finding.
        let source = "## ADDED Requirements\n\n### Requirement: A\nThe system SHALL a.\n\n### Scenario: S\n- **WHEN** a\n- **THEN** b\n";
        let parsed = parse_delta(source);
        let codes: Vec<&str> = parsed.findings.iter().map(|f| f.code).collect();
        assert_eq!(codes, vec!["scenario_wrong_heading_level"]);
    }

    #[test]
    fn new_capability_purpose_is_extracted() {
        let source = "## Purpose\n\nLets users export their data.\n\n## ADDED Requirements\n\n### Requirement: R\nThe system SHALL r.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n";
        let parsed = parse_delta(source);
        let p = parsed.value.purpose.expect("purpose expected");
        assert!(p.text.contains("export their data"));
    }

    #[test]
    fn duplicate_in_added_is_reported() {
        let source = "## ADDED Requirements\n\n### Requirement: X\nSHALL x.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n\n### Requirement: X\nSHALL x again.\n\n#### Scenario: T\n- **WHEN** c\n- **THEN** d\n";
        let parsed = parse_delta(source);
        let f = parsed
            .findings
            .iter()
            .find(|f| f.code == "duplicate_requirement")
            .expect("finding expected");
        assert!(f.message.contains("ADDED"), "{}", f.message);
        assert!(f.message.contains("duplicate"), "{}", f.message);
    }

    #[test]
    fn header_inside_html_comment_is_ignored() {
        // An `## ADDED Requirements` inside a comment does not count; only
        // the real one further down does.
        let source = "<!--\n## ADDED Requirements\n\n### Requirement: Fake\n-->\n\n## ADDED Requirements\n\n### Requirement: Real\nSHALL r.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n";
        let parsed = parse_delta(source);
        assert_eq!(parsed.value.sections.len(), 1, "a single ADDED");
        let DeltaSection::Added { requirements, .. } = &parsed.value.sections[0] else {
            panic!();
        };
        assert_eq!(requirements.len(), 1);
        assert_eq!(requirements[0].name, "Real");
    }
}
