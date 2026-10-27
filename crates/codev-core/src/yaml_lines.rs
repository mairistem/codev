//! Where a key is written in a hand-written YAML file.
//!
//! The YAML parser keeps no position on its generic values, and a warning
//! about a key this codev ignores (decision 0013) must point at its line.
//! This is a text scan, by indentation, only used to locate: it never
//! decides what the file means.

/// A key of a hand-written file this codev does not know: ignored, and
/// reported with a warning at its line (decision 0013).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnknownKey {
    /// The key's name, dotted with its parents where they help find it
    /// (`mcp.design_tool`, `review.budget`).
    pub key: String,
    /// 1-based line of the key, 1 when it cannot be located (a flow
    /// mapping, for instance).
    pub line: usize,
}

/// The keys of `mapping` not in `known`, in file order, each named
/// `<prefix><key>` and located at `path` + the key in `raw`.
pub fn unknown_keys_of(
    raw: &str,
    mapping: &serde_norway::Mapping,
    known: &[&str],
    path: &[Step],
    prefix: &str,
) -> Vec<UnknownKey> {
    mapping
        .keys()
        .filter_map(serde_norway::Value::as_str)
        .filter(|key| !known.contains(key))
        .map(|key| {
            let mut at = path.to_vec();
            at.push(Step::Key(key));
            UnknownKey {
                key: format!("{prefix}{key}"),
                line: line_of(raw, &at),
            }
        })
        .collect()
}

/// One step of the path to a key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Step<'a> {
    /// A key of the current mapping.
    Key(&'a str),
    /// The n-th item (0-based) of the current sequence.
    Item(usize),
}

/// A line as the scan sees it: a list item `- id: x` is a marker at the
/// dash's indentation followed by the line `id: x` two columns deeper.
struct Virtual<'t> {
    /// 1-based line in the file.
    line: usize,
    indent: usize,
    /// `None` for an item marker.
    text: Option<&'t str>,
}

fn virtual_lines(raw: &str) -> Vec<Virtual<'_>> {
    let mut out = Vec::new();
    for (i, line) in raw.lines().enumerate() {
        let mut text = line.trim_start();
        let mut indent = line.len() - text.len();
        if text.is_empty() || text.starts_with('#') || text == "---" {
            continue;
        }
        while let Some(rest) = text
            .strip_prefix('-')
            .filter(|r| r.is_empty() || r.starts_with(' '))
        {
            out.push(Virtual {
                line: i + 1,
                indent,
                text: None,
            });
            let trimmed = rest.trim_start();
            indent += 1 + rest.len() - trimmed.len();
            text = trimmed;
        }
        if !text.is_empty() {
            out.push(Virtual {
                line: i + 1,
                indent,
                text: Some(text),
            });
        }
    }
    out
}

fn starts_with_key(text: &str, key: &str) -> bool {
    let unquoted = text
        .strip_prefix('"')
        .and_then(|t| t.strip_prefix(key))
        .and_then(|t| t.strip_prefix('"'))
        .or_else(|| {
            text.strip_prefix('\'')
                .and_then(|t| t.strip_prefix(key))
                .and_then(|t| t.strip_prefix('\''))
        })
        .or_else(|| text.strip_prefix(key));
    unquoted.is_some_and(|rest| rest.trim_start().starts_with(':'))
}

/// The 1-based line of the key at `path`. When a step cannot be found —
/// a flow mapping, for instance — the line of the deepest step found, or 1.
pub fn line_of(raw: &str, path: &[Step]) -> usize {
    let lines = virtual_lines(raw);
    let mut start = 0;
    let mut parent: Option<usize> = None;
    let mut deepest_found_line = 1;
    for step in path {
        let mut child: Option<usize> = None;
        let mut items = 0;
        let mut hit = None;
        for (j, v) in lines.iter().enumerate().skip(start) {
            if let Some(p) = parent {
                // A sequence may sit at its key's indentation (`key:\n- a`).
                let item_at_key_level =
                    v.indent == p && v.text.is_none() && matches!(step, Step::Item(_));
                if v.indent < p || (v.indent == p && !item_at_key_level) {
                    break;
                }
            }
            // The first line under the parent sets its children's
            // indentation; deeper lines belong to a grandchild.
            let level = *child.get_or_insert(v.indent);
            if v.indent != level {
                continue;
            }
            match (step, v.text) {
                (Step::Key(key), Some(text)) if starts_with_key(text, key) => {
                    hit = Some(j);
                    break;
                }
                (Step::Item(n), None) => {
                    if items == *n {
                        hit = Some(j);
                        break;
                    }
                    items += 1;
                }
                _ => {}
            }
        }
        let Some(j) = hit else {
            return deepest_found_line;
        };
        deepest_found_line = lines[j].line;
        parent = Some(lines[j].indent);
        start = j + 1;
    }
    deepest_found_line
}

#[cfg(test)]
mod tests {
    use super::*;
    use Step::{Item, Key};

    #[test]
    fn a_top_level_key() {
        assert_eq!(line_of("a: 1\nb: 2\n", &[Key("b")]), 2);
    }

    #[test]
    fn a_nested_key_skips_comments_and_blank_lines() {
        let raw = "roles:\n  # note\n\n  skeptic:\n    when: always\n    cadence: weekly\n  newcomer: never\n";
        assert_eq!(
            line_of(raw, &[Key("roles"), Key("skeptic"), Key("cadence")]),
            6
        );
        assert_eq!(line_of(raw, &[Key("roles"), Key("newcomer")]), 7);
    }

    #[test]
    fn a_key_of_a_sibling_block_is_not_taken() {
        let raw = "a:\n  x: 1\nb:\n  y: 2\n";
        assert_eq!(line_of(raw, &[Key("a"), Key("y")]), 1);
    }

    #[test]
    fn items_of_a_sequence() {
        let raw = "artifacts:\n  - id: a\n    generates: a.md\n  - id: b\n    require: a\n";
        assert_eq!(
            line_of(raw, &[Key("artifacts"), Item(1), Key("require")]),
            5
        );
        assert_eq!(line_of(raw, &[Key("artifacts"), Item(0), Key("id")]), 2);
    }

    #[test]
    fn a_sequence_at_its_key_s_indentation() {
        let raw = "artifacts:\n- id: a\n  role:\n    model: x\n- id: b\n";
        assert_eq!(
            line_of(raw, &[Key("artifacts"), Item(0), Key("role"), Key("model")]),
            4
        );
        assert_eq!(line_of(raw, &[Key("artifacts"), Item(1), Key("id")]), 5);
    }

    #[test]
    fn a_flow_mapping_points_at_its_key() {
        let raw = "review: { mode: isolated, budget: 3 }\n";
        assert_eq!(line_of(raw, &[Key("review"), Key("budget")]), 1);
    }

    #[test]
    fn an_absent_key_is_line_one() {
        assert_eq!(line_of("a: 1\n", &[Key("b")]), 1);
    }

    #[test]
    fn a_frontmatter_after_its_fence() {
        assert_eq!(
            line_of("---\nid: 1\nreview_by: x\n", &[Key("review_by")]),
            3
        );
    }
}
