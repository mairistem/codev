//! Building blocks shared by `spec.rs` and `delta.rs`: line offsets,
//! heading recognition, extraction of requirements and scenarios.
//!
//! Not publicly exposed (`pub(super)` only): this is implementation
//! machinery, not contract.

use super::ast::{Finding, Requirement, Scenario, Span};

/// A source line, augmented with what needs to be known about it without
/// recomputing.
///
/// - `line_number` is 1-indexed (as an editor displays it).
/// - `byte_offset` is the offset of the line's first byte in the input
///   source — the very one the spans use.
/// - `literal` comes from the fence mask; it is consulted on every heading
///   recognition so as not to fall into the false-positive trap inside a
///   code block.
#[derive(Debug, Clone, Copy)]
pub(super) struct ScannedLine<'a> {
    pub text: &'a str,
    pub line_number: u32,
    pub byte_offset: usize,
    pub literal: bool,
}

pub(super) fn line_starts(source: &str) -> Vec<usize> {
    // Precondition: the length of the returned vector MUST be exactly that
    // of `source.split('\n')`. Otherwise, the indices read by the parsers
    // silently point into the void.
    let mut starts = Vec::with_capacity(source.len() / 40 + 1);
    starts.push(0);
    for (i, byte) in source.bytes().enumerate() {
        if byte == b'\n' {
            starts.push(i + 1);
        }
    }
    starts
}

/// Recognizes a top-level `## <title>` heading, outside a literal zone.
///
/// Returns the cleaned title, or `None` if the line is not a `##` heading.
/// A `##` heading followed by a `#` (hence `###`) is not top-level.
pub(super) fn section_header(line: &str) -> Option<String> {
    let trimmed = line.trim_start();
    let after = trimmed.strip_prefix("## ")?;
    if after.starts_with('#') {
        return None;
    }
    Some(after.trim().to_string())
}

/// Recognizes `## ADDED Requirements`, `## MODIFIED Requirements`, etc.
pub(super) fn is_delta_header(line: &str) -> bool {
    delta_section_kind(line).is_some()
}

pub(super) fn delta_section_kind(line: &str) -> Option<DeltaKind> {
    let title = section_header(line)?;
    let upper = title.to_ascii_uppercase();
    let stripped = upper.strip_suffix("REQUIREMENTS")?.trim_end();
    match stripped {
        "ADDED" => Some(DeltaKind::Added),
        "MODIFIED" => Some(DeltaKind::Modified),
        "REMOVED" => Some(DeltaKind::Removed),
        "RENAMED" => Some(DeltaKind::Renamed),
        _ => None,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum DeltaKind {
    Added,
    Modified,
    Removed,
    Renamed,
}

pub(super) struct RequirementPosition {
    pub name: String,
    pub header_index: usize,
    pub line_number: u32,
}

pub(super) fn find_requirement_positions(scanned: &[ScannedLine]) -> Vec<RequirementPosition> {
    find_requirement_positions_in_range(scanned, 0, scanned.len())
}

pub(super) fn find_requirement_positions_in_range(
    scanned: &[ScannedLine],
    start: usize,
    end: usize,
) -> Vec<RequirementPosition> {
    let bounded_end = end.min(scanned.len());
    scanned[start..bounded_end]
        .iter()
        .enumerate()
        .filter(|(_, entry)| !entry.literal)
        .filter_map(|(offset, entry)| {
            requirement_header(entry.text).map(|name| RequirementPosition {
                name,
                header_index: start + offset,
                line_number: entry.line_number,
            })
        })
        .collect()
}

fn requirement_header(line: &str) -> Option<String> {
    let trimmed = line.trim_start();
    let after = trimmed.strip_prefix("### Requirement:")?;
    if after.starts_with('#') {
        return None; // this is a `####`: not our heading
    }
    Some(after.trim().to_string())
}

/// Extracts a `#### Scenario:` — exactly four hashes. Three hashes is the
/// trap we must detect and report, not silently handle.
fn scenario_header(line: &str) -> Option<String> {
    let trimmed = line.trim_start();
    let after = trimmed.strip_prefix("#### Scenario:")?;
    if after.starts_with('#') {
        return None;
    }
    Some(after.trim().to_string())
}

/// Detects the trap: `### Scenario:` instead of `#### Scenario:`.
fn wrong_level_scenario(line: &str) -> Option<String> {
    let trimmed = line.trim_start();
    let after = trimmed.strip_prefix("### Scenario:")?;
    Some(after.trim().to_string())
}

/// Signature of a requirement body extractor. Naming the type keeps the
/// signature of [`requirement_from_lines`] readable.
pub(super) type BodyCollector =
    dyn Fn(&[ScannedLine], usize, usize, &mut Vec<Finding>) -> (String, Vec<Scenario>);

/// Assembles a complete `Requirement` from a range of lines delimited by its
/// heading and the start of the next requirement (or the end of the
/// section).
pub(super) fn requirement_from_lines(
    scanned: &[ScannedLine],
    header_index: usize,
    end_index: usize,
    name: String,
    collect: &BodyCollector,
) -> (Requirement, Vec<Finding>) {
    let mut findings = Vec::new();
    let (description, scenarios) = collect(scanned, header_index, end_index, &mut findings);
    let span = block_span(scanned, header_index, end_index);
    (
        Requirement {
            name,
            description,
            scenarios,
            span,
        },
        findings,
    )
}

/// Collects the descriptive paragraph up to the first scenario, then the
/// scenarios themselves.
pub(super) fn collect_scenarios(
    scanned: &[ScannedLine],
    header_index: usize,
    end_index: usize,
    findings: &mut Vec<Finding>,
) -> (String, Vec<Scenario>) {
    // Find the first (well-formed) "scenario" line after the heading,
    // reporting the malformed ones along the way (`### Scenario:` — the
    // silent trap, see the spec).
    let mut first_scenario: Option<usize> = None;
    for (offset, entry) in scanned[header_index + 1..end_index].iter().enumerate() {
        if entry.literal {
            continue;
        }
        if scenario_header(entry.text).is_some() {
            first_scenario = Some(header_index + 1 + offset);
            break;
        }
        if wrong_level_scenario(entry.text).is_some() {
            findings.push(Finding::error(
                super::codes::SCENARIO_WRONG_HEADING_LEVEL,
                entry.line_number,
                format!(
                    "line {}: a scenario must have exactly four hashes (`#### Scenario:`), not three",
                    entry.line_number
                ),
            ));
        }
    }

    let description_end = first_scenario.unwrap_or(end_index);
    let description = trim_body(&scanned[header_index + 1..description_end]);

    // Scenario collection: each `#### Scenario:` starts a block, which
    // extends to the next scenario or the end of the requirement.
    let mut scenarios = Vec::new();
    if let Some(first) = first_scenario {
        let positions: Vec<usize> = (first..end_index)
            .filter(|&i| {
                let entry = &scanned[i];
                !entry.literal && scenario_header(entry.text).is_some()
            })
            .collect();
        for (rank, &pos) in positions.iter().enumerate() {
            let stop = positions.get(rank + 1).copied().unwrap_or(end_index);
            let name = scenario_header(scanned[pos].text).expect("filtered above");
            let body = trim_body(&scanned[pos + 1..stop]);
            let span = block_span(scanned, pos, stop);
            scenarios.push(Scenario { name, body, span });
        }
    }
    (description, scenarios)
}

/// Gathers the text of a block, removing leading and trailing blank lines,
/// without altering the inner lines.
pub(super) fn trim_body(lines: &[ScannedLine]) -> String {
    let mut start = 0;
    while start < lines.len() && lines[start].text.trim().is_empty() {
        start += 1;
    }
    let mut end = lines.len();
    while end > start && lines[end - 1].text.trim().is_empty() {
        end -= 1;
    }
    lines[start..end]
        .iter()
        .map(|l| l.text)
        .collect::<Vec<_>>()
        .join("\n")
}

/// The exact span of a block: from the heading (inclusive) to the line just
/// before the next block (exclusive).
///
/// Two useful effects:
/// - `byte_range` covers the bytes from the heading to the end of the block's
///   last character — replacing this range rewrites the whole block.
/// - `line_range` covers the 1-indexed lines of the same zone. `start` is
///   the heading line, `end` is the first line of the next block (or
///   `lines.len() + 1` if it is the last one).
pub(super) fn block_span(scanned: &[ScannedLine], start: usize, end: usize) -> Span {
    let start_byte = scanned[start].byte_offset;
    let end_byte = if end >= scanned.len() {
        // Last block: up to the end of the source. The offset of the
        // "virtual" line added by `split('\n')` covers this case.
        scanned
            .last()
            .map(|l| l.byte_offset + l.text.len())
            .unwrap_or(0)
    } else {
        scanned[end].byte_offset
    };
    let start_line = scanned[start].line_number;
    let end_line = if end >= scanned.len() {
        scanned
            .last()
            .map(|l| l.line_number + 1)
            .unwrap_or(start_line + 1)
    } else {
        scanned[end].line_number
    };
    Span::new(start_byte..end_byte, start_line..end_line)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_starts_gives_the_offset_of_each_line() {
        let source = "a\nbb\n\nccc\n";
        // A trailing empty line (from the final `\n`) is expected —
        // `split('\n')` produces one, `line_starts` must match it.
        assert_eq!(line_starts(source), vec![0, 2, 5, 6, 10]);
        assert_eq!(source.split('\n').count(), line_starts(source).len());
    }

    #[test]
    fn section_header_recognizes_level_2_only() {
        assert_eq!(section_header("## Purpose").as_deref(), Some("Purpose"));
        assert_eq!(
            section_header("##   Requirements  ").as_deref(),
            Some("Requirements")
        );
        assert_eq!(section_header("### Requirement: X"), None);
        assert_eq!(section_header("# Title"), None);
        assert_eq!(section_header("hello"), None);
    }

    #[test]
    fn delta_headers_are_recognized_case_insensitively() {
        assert_eq!(
            delta_section_kind("## ADDED Requirements"),
            Some(DeltaKind::Added)
        );
        assert_eq!(
            delta_section_kind("## modified requirements"),
            Some(DeltaKind::Modified)
        );
        assert_eq!(
            delta_section_kind("## REMOVED Requirements"),
            Some(DeltaKind::Removed)
        );
        assert_eq!(
            delta_section_kind("## RENAMED Requirements"),
            Some(DeltaKind::Renamed)
        );
        assert_eq!(delta_section_kind("## Removed"), None);
    }
}
