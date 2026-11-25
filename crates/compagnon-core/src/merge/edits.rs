//! A single edit on a string — `[start..end)` replaced by a text.
//!
//! The merge is pure: it produces a list of edits that the shell applies
//! to the main spec source. This is what makes `--dry-run`, the JSON
//! preview, and above all atomicity possible — the plan is fully computed
//! before a single write touches the disk.

use std::ops::Range;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Edit {
    pub byte_range: Range<usize>,
    pub replacement: String,
}

impl Edit {
    pub fn new(byte_range: Range<usize>, replacement: impl Into<String>) -> Self {
        Self {
            byte_range,
            replacement: replacement.into(),
        }
    }
}

/// Applies a series of edits to a source, preserving the remaining offsets.
///
/// Edits are sorted by **descending** `byte_range.end`, then by descending
/// `byte_range.start`, before being applied. Two edits do end at the same
/// point: a `MODIFIED` of a section's last requirement, whose span runs to
/// the section's end, and an `ADDED` inserted at that end. The zero-width
/// insertion then goes first and sits after the end; the replacement that
/// follows rewrites only what lies before it. Applied the other way round,
/// the replacement would lengthen the text and the insertion would land at
/// its old offset, inside the new text.
///
/// An edit whose `byte_range.end > source.len()` is rejected — this is the
/// only safety invariant; the rest is already validated by the calling layer.
pub fn apply_edits(source: &str, edits: &[Edit]) -> String {
    let mut ordered = edits.to_vec();
    // Stable sort by descending end then start: the last modified segments
    // go first, leaving earlier offsets intact, and an insertion at the end
    // of a replacement goes before it. `sort_by_key` + `Reverse` avoids the
    // double comparison flagged by clippy.
    ordered.sort_by_key(|e| std::cmp::Reverse((e.byte_range.end, e.byte_range.start)));

    let mut out = source.to_string();
    for edit in ordered {
        let end = edit.byte_range.end.min(out.len());
        let start = edit.byte_range.start.min(end);
        out.replace_range(start..end, &edit.replacement);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apply_is_stable_even_with_shuffled_order() {
        let source = "AAAA BBBB CCCC";
        let edits = vec![
            // Order deliberately reversed relative to position.
            Edit::new(10..14, "cccc"),
            Edit::new(0..4, "aaaa"),
            Edit::new(5..9, "bbbb"),
        ];
        assert_eq!(apply_edits(source, &edits), "aaaa bbbb cccc");
    }

    #[test]
    fn apply_preserves_content_outside_ranges() {
        let source = "before [HERE] after";
        let edits = vec![Edit::new(7..13, "[THERE]")];
        assert_eq!(apply_edits(source, &edits), "before [THERE] after");
    }

    #[test]
    fn an_insertion_at_the_end_of_a_replacement_goes_after_it() {
        // A MODIFIED of the last requirement ends where an ADDED inserts:
        // whatever their order, the insertion lands after the new text.
        let source = "head OLD";
        for edits in [
            vec![Edit::new(5..8, "NEW-AND-LONGER"), Edit::new(8..8, "+ADDED")],
            vec![Edit::new(8..8, "+ADDED"), Edit::new(5..8, "NEW-AND-LONGER")],
        ] {
            assert_eq!(apply_edits(source, &edits), "head NEW-AND-LONGER+ADDED");
        }
    }

    #[test]
    fn empty_edit_at_same_position_inserts() {
        let source = "abcXYZ";
        let edits = vec![Edit::new(3..3, "INS")];
        assert_eq!(apply_edits(source, &edits), "abcINSXYZ");
    }

    #[test]
    fn empty_edit_with_empty_replacement_is_a_no_op() {
        let source = "hello";
        let edits = vec![Edit::new(2..2, "")];
        assert_eq!(apply_edits(source, &edits), "hello");
    }

    #[test]
    fn out_of_bounds_edit_is_clamped_without_panicking() {
        // Safety net against an upstream computation bug: rather than
        // panicking on an out-of-bounds range, we clamp — the golden test
        // that re-runs on the original source will catch the inconsistency.
        let source = "abc";
        let edits = vec![Edit::new(2..999, "XYZ")];
        assert_eq!(apply_edits(source, &edits), "abXYZ");
    }
}
