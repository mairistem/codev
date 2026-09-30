//! Recognition of literal zones: ` ``` ` and `~~~` code blocks, plus
//! HTML comments `<!-- … -->`.
//!
//! The logic lives here, not in each parser, for the same reason as
//! upstream in OpenSpec: three independent parsers re-implementing it
//! drift apart and end up accepting false positives — a
//! `### Requirement:` inside a code block taken for real.
//! The silent failure we refuse.

/// A per-line mask: `true` if the line belongs to a literal zone (opening
/// fence, closing fence, content; line of a multi-line HTML comment).
///
/// The spec and delta parsers consult this mask before recognizing a
/// heading. This is what makes a requirement inside a code block
/// invisible.
pub fn build_fence_mask(source: &str) -> Vec<bool> {
    let lines: Vec<&str> = source.split('\n').collect();
    let mut mask = vec![false; lines.len()];

    let mut active_fence: Option<ActiveFence> = None;
    let mut html_comment_open = false;

    for (i, line) in lines.iter().enumerate() {
        // Inside a fence: everything is literal, including a possible HTML
        // comment start — hence handling the fence first.
        if let Some(fence) = &active_fence {
            mask[i] = true;
            if is_closing_fence(line, fence) {
                active_fence = None;
            }
            continue;
        }

        if let Some(fence) = opening_fence(line) {
            mask[i] = true;
            active_fence = Some(fence);
            continue;
        }

        // Outside a fence: handle HTML comments. A comment may span several
        // lines; as long as it is not closed, the following lines are
        // masked.
        if html_comment_open {
            mask[i] = true;
            if line.contains("-->") {
                html_comment_open = false;
            }
            continue;
        }

        // Detection on this line. A `<!--` followed by a `-->` **on the same
        // line** is a closed comment that still masks this line, without
        // opening a window for the following ones.
        if let Some(start) = line.find("<!--") {
            let after = &line[start + 4..];
            if after.contains("-->") {
                mask[i] = true;
            } else {
                mask[i] = true;
                html_comment_open = true;
            }
        }
    }

    mask
}

#[derive(Debug, Clone, Copy)]
struct ActiveFence {
    marker: u8, // b'`' or b'~'
    length: usize,
}

/// Recognizes a fence opening: at least three `\`` or `~`, optional leading
/// whitespace, an optional "info string" (`rust`, `text`, …) after.
fn opening_fence(line: &str) -> Option<ActiveFence> {
    let trimmed_left = line.trim_start();
    if trimmed_left.is_empty() {
        return None;
    }
    let first = trimmed_left.as_bytes()[0];
    if first != b'`' && first != b'~' {
        return None;
    }
    let length = trimmed_left.bytes().take_while(|&b| b == first).count();
    if length < 3 {
        return None;
    }
    Some(ActiveFence {
        marker: first,
        length,
    })
}

/// Recognizes a fence closing: the same marker, at least as long, possibly
/// followed by whitespace — but nothing else. This is the CommonMark rule,
/// and where OpenSpec deviates.
fn is_closing_fence(line: &str, fence: &ActiveFence) -> bool {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return false;
    }
    let bytes = trimmed.as_bytes();
    let run = bytes.iter().take_while(|&&b| b == fence.marker).count();
    run >= fence.length && bytes[run..].iter().all(|b| b.is_ascii_whitespace())
}

#[cfg(test)]
mod tests {
    use super::*;

    // `source.split('\n')` produces a trailing empty line when the source
    // ends with `\n`. The mask must therefore carry a value for that line
    // too. A helper centralizes the check; otherwise each test recounts the
    // `\n` by hand and gets it wrong.
    fn check(source: &str, expected: Vec<bool>) {
        let observed = build_fence_mask(source);
        assert_eq!(
            observed.len(),
            source.split('\n').count(),
            "the mask must have the same length as split('\\n')",
        );
        assert_eq!(observed, expected);
    }

    #[test]
    fn mask_recognizes_backticks_and_tildes() {
        let source = "```\ninside\n```\n\ntop\n~~~\nalso inside\n~~~\n";
        check(
            source,
            vec![true, true, true, false, false, true, true, true, false],
        );
    }

    #[test]
    fn mask_rejects_a_closing_with_a_different_marker() {
        let source = "```\nfoo\n~~~\nbar\n```\nout\n";
        check(source, vec![true, true, true, true, true, false, false]);
    }

    #[test]
    fn mask_handles_two_consecutive_blocks() {
        let source = "```\na\n```\n```\nb\n```\n";
        check(source, vec![true, true, true, true, true, true, false]);
    }

    #[test]
    fn mask_ignores_a_too_short_closing() {
        // Opening ````, closing ```: the closing must be at least as long
        // as the opening.
        let source = "````\nx\n```\ny\n````\n";
        check(source, vec![true, true, true, true, true, false]);
    }

    #[test]
    fn multi_line_comment_is_masked() {
        let source = "before\n<!-- start of the comment\n### Requirement: Fake\nend of the comment -->\nafter\n";
        check(source, vec![false, true, true, true, false, false]);
    }

    #[test]
    fn single_line_comment_is_masked_without_opening_a_window() {
        let source = "before\n<!-- inline -->\nafter\n### Requirement: Real\n";
        check(source, vec![false, true, false, false, false]);
    }

    #[test]
    fn comment_inside_a_fence_stays_literal_via_the_fence() {
        let source = "```\n<!-- not a real comment\n```\nafter\n";
        check(source, vec![true, true, true, false, false]);
    }
}
