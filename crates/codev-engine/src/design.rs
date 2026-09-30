//! Lightweight parser for the `### Decision: ...` blocks of a change's
//! `design.md`.
//!
//! Tightly constrained format: the `## Decisions` section contains N blocks,
//! each starting with `### Decision: <title>` and ending at the next
//! `### ` or `## ` (at H3 or H2 level). A line-by-line scan is enough — no
//! need for a full markdown parser, as the decision
//! `_codev/decisions/0001-coeur-fonctionnel-coquille-imperative.md`
//! reminds us: this module is pure, with no I/O; it lives on the engine side
//! because it is called by functions that need the ports.
//!
//! A block's body is extracted **byte for byte** — consistent with decision
//! sealing (the seal covers the ADR body byte for byte): a consumer that promotes a block
//! to an ADR gets its text back exactly.

use std::ops::Range;

const H2_DECISIONS: &str = "## Decisions";
const H3_DECISION_PREFIX: &str = "### Decision: ";

/// A `### Decision: <title>` block extracted from a `design.md`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionBlock {
    /// What follows `### Decision: `, trimmed.
    pub title: String,
    /// The bytes between the end of the title line and the start of the
    /// next block (or the end of the file), **verbatim**.
    pub body: String,
    /// Position of the block in the source — useful for rewriting the
    /// design during promotion.
    pub byte_range: Range<usize>,
    /// Line of the title (1-indexed) — useful for reporting an ambiguity.
    pub line: u32,
}

/// Extracts every `### Decision: <title>` block under the source's
/// `## Decisions` section.
///
/// Returns an empty list if:
/// - the source does not contain `## Decisions`;
/// - the `## Decisions` section exists but holds no `###
///   Decision: ...` block.
///
/// Blocks are returned in the order they appear.
pub fn extract_decision_blocks(source: &str) -> Vec<DecisionBlock> {
    let Some((section_start, section_end)) = find_decisions_section(source) else {
        return Vec::new();
    };

    let section = &source[section_start..section_end];
    let base_offset = section_start;

    // Step 1: collect the positions (relative to `section`) of the
    // `### Decision: <title>` lines.
    let mut heads: Vec<(usize, String, u32)> = Vec::new();
    let mut rel = 0usize;
    for line in section.split_inclusive('\n') {
        let trimmed = line.trim_end_matches(['\n', '\r']);
        if let Some(after) = trimmed.strip_prefix(H3_DECISION_PREFIX) {
            let title = after.trim().to_string();
            let abs_line = line_at_byte(source, base_offset + rel);
            heads.push((rel, title, abs_line));
        }
        rel += line.len();
    }

    // Step 2: build the blocks — a block spans from the title line up to
    // the start of the next line marked `### ` or `## `
    // (or the end of the section).
    let mut blocks = Vec::with_capacity(heads.len());
    for (i, (head_rel, title, line)) in heads.iter().enumerate() {
        let block_start = *head_rel;
        let block_end = heads
            .get(i + 1)
            .map(|(next, _, _)| *next)
            .unwrap_or(section.len());

        // End of the title line = first `\n` after `head_rel`.
        let title_line_end = section[*head_rel..]
            .find('\n')
            .map(|off| head_rel + off + 1)
            .unwrap_or(section.len());
        // Body = bytes between the end of the title line and the start of the next block.
        let body = section[title_line_end..block_end].to_string();

        blocks.push(DecisionBlock {
            title: title.clone(),
            body,
            byte_range: (base_offset + block_start)..(base_offset + block_end),
            line: *line,
        });
    }

    blocks
}

/// Locates the extent of the `## Decisions` section: from the title line
/// (inclusive) to the line of the next `## ` (exclusive) or the end of the
/// file.
///
/// Returns `None` if the section is missing.
fn find_decisions_section(source: &str) -> Option<(usize, usize)> {
    let mut cursor = 0usize;
    let mut section_start: Option<usize> = None;
    let mut section_end: usize = source.len();
    for line in source.split_inclusive('\n') {
        let trimmed = line.trim_end_matches(['\n', '\r']);
        match section_start {
            None => {
                if trimmed == H2_DECISIONS {
                    section_start = Some(cursor);
                }
            }
            Some(_) => {
                // A new H2 section closes the Decisions section.
                if trimmed.starts_with("## ") && !trimmed.starts_with(H3_DECISION_PREFIX) {
                    section_end = cursor;
                    break;
                }
            }
        }
        cursor += line.len();
    }
    section_start.map(|start| (start, section_end))
}

fn line_at_byte(source: &str, byte: usize) -> u32 {
    let clamped = byte.min(source.len());
    1 + source[..clamped].bytes().filter(|&b| b == b'\n').count() as u32
}

/// Title lookup errors — stable codes carried on the action side.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LookupError {
    NotFound,
    Ambiguous { lines: Vec<u32> },
}

/// Looks up a block by its exact title (after trimming). Returns its index
/// in the slice, or an error.
pub fn find_decision_block(blocks: &[DecisionBlock], title: &str) -> Result<usize, LookupError> {
    let matches: Vec<usize> = blocks
        .iter()
        .enumerate()
        .filter(|(_, b)| b.title == title)
        .map(|(i, _)| i)
        .collect();
    match matches.as_slice() {
        [] => Err(LookupError::NotFound),
        [only] => Ok(*only),
        many => Err(LookupError::Ambiguous {
            lines: many.iter().map(|i| blocks[*i].line).collect(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ─────────────── extract_decision_blocks ───────────────

    #[test]
    fn source_without_decisions_section_yields_empty_list() {
        let source = "# Design\n\n## Context\n\nx\n\n## Other\n\ny\n";
        assert!(extract_decision_blocks(source).is_empty());
    }

    #[test]
    fn empty_section_yields_empty_list() {
        let source = "# Design\n\n## Decisions\n\n## After\n";
        assert!(extract_decision_blocks(source).is_empty());
    }

    #[test]
    fn one_block_extracts_title_and_body() {
        let source = "# Design\n\n## Decisions\n\n### Decision: Use JWT\n\nThe rationale.\n\nSecond paragraph.\n\n## After\n\nz\n";
        let blocks = extract_decision_blocks(source);
        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].title, "Use JWT");
        assert_eq!(blocks[0].body, "\nThe rationale.\n\nSecond paragraph.\n\n");
    }

    #[test]
    fn two_blocks_are_extracted_in_order() {
        let source =
            "## Decisions\n\n### Decision: Alpha\n\nA1\n\n### Decision: Beta\n\nB1\n\n## End\n";
        let blocks = extract_decision_blocks(source);
        assert_eq!(blocks.len(), 2);
        assert_eq!(blocks[0].title, "Alpha");
        assert_eq!(blocks[1].title, "Beta");
        assert_eq!(blocks[0].body, "\nA1\n\n");
        assert_eq!(blocks[1].body, "\nB1\n\n");
    }

    #[test]
    fn body_is_verbatim_even_with_rich_markdown() {
        // Table, code fence, italics — everything stays in the body.
        let source = "## Decisions\n\n### Decision: Table\n\n| A | B |\n|---|---|\n| 1 | 2 |\n\n```\ncode\n```\n\n*emphasis*\n\n## After\n";
        let blocks = extract_decision_blocks(source);
        assert_eq!(blocks.len(), 1);
        assert!(blocks[0].body.contains("| A | B |"));
        assert!(blocks[0].body.contains("```\ncode\n```"));
        assert!(blocks[0].body.contains("*emphasis*"));
    }

    #[test]
    fn byte_range_allows_substitution() {
        let source =
            "BEFORE\n## Decisions\n\n### Decision: X\n\nBody X.\n\n### Decision: Y\n\nBody Y.\n";
        let blocks = extract_decision_blocks(source);
        assert_eq!(blocks.len(), 2);
        // Slicing by byte_range must reproduce each whole block
        // (title line + body).
        let block0 = &source[blocks[0].byte_range.clone()];
        assert!(block0.starts_with("### Decision: X\n"));
        assert!(block0.ends_with("Body X.\n\n"));
        let block1 = &source[blocks[1].byte_range.clone()];
        assert!(block1.starts_with("### Decision: Y\n"));
        assert!(block1.ends_with("Body Y.\n"));
    }

    #[test]
    fn title_line_is_reported() {
        let source = "L1\nL2\n## Decisions\n\n### Decision: X\n\nbody\n";
        let blocks = extract_decision_blocks(source);
        // The title is on line 5 (L1, L2, then ## Decisions, then a blank
        // line, then ###).
        assert_eq!(blocks[0].line, 5);
    }

    #[test]
    fn title_is_trimmed() {
        let source = "## Decisions\n\n### Decision:    With spaces    \n\nbody\n";
        let blocks = extract_decision_blocks(source);
        assert_eq!(blocks[0].title, "With spaces");
    }

    // ─────────────── find_decision_block ───────────────

    #[test]
    fn lookup_finds_by_exact_title() {
        let source = "## Decisions\n\n### Decision: Alpha\n\nA\n\n### Decision: Beta\n\nB\n";
        let blocks = extract_decision_blocks(source);
        assert_eq!(find_decision_block(&blocks, "Beta").unwrap(), 1);
    }

    #[test]
    fn lookup_missing_returns_not_found() {
        let source = "## Decisions\n\n### Decision: Alpha\n\nA\n";
        let blocks = extract_decision_blocks(source);
        assert_eq!(
            find_decision_block(&blocks, "Ghost").unwrap_err(),
            LookupError::NotFound
        );
    }

    #[test]
    fn lookup_ambiguous_returns_positions() {
        let source = "## Decisions\n\n### Decision: X\n\nv1\n\n### Decision: X\n\nv2\n";
        let blocks = extract_decision_blocks(source);
        let err = find_decision_block(&blocks, "X").unwrap_err();
        match err {
            LookupError::Ambiguous { lines } => {
                assert_eq!(lines.len(), 2);
            }
            other => panic!("expected Ambiguous, got {:?}", other),
        }
    }
}
