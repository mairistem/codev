//! ADR parser: YAML frontmatter + markdown sections.
//!
//! Two small leniencies of the format to handle:
//!
//! - `id: 0001` is an integer in YAML terms — we tolerate it and always
//!   serialize to `String`, so that consumers only have one shape to
//!   know about;
//! - `supersedes` can be a single identifier or a list — we normalize
//!   to `Vec<String>`.

use serde::Deserialize;

use crate::parser::ast::{Finding, Parsed, Span};
use crate::parser::codes;

use super::ast::{Decision, DecisionStatus, Section};

/// Validates that a `deviates_from` is indeed a list of strings.
///
/// Kept separate from the rest of the parsing to emit a **specific**
/// finding (`DECISION_FIELD_TYPE_MISMATCH`) rather than the generic
/// `DECISION_MISSING_FIELD` that serde would produce by default.
fn extract_deviates_from(value: &serde_norway::Value, findings: &mut Vec<Finding>) -> Vec<String> {
    let seq = match value {
        serde_norway::Value::Sequence(seq) => seq,
        serde_norway::Value::Null => return Vec::new(),
        _ => {
            findings.push(Finding::error(
                codes::DECISION_FIELD_TYPE_MISMATCH,
                1,
                "the `deviates_from` field must be a list of qualified identifiers \
                 (for example `deviates_from: [path:~/shared/0100]`)",
            ));
            return Vec::new();
        }
    };

    let mut out = Vec::with_capacity(seq.len());
    for (i, item) in seq.iter().enumerate() {
        match item {
            serde_norway::Value::String(s) => out.push(s.clone()),
            _ => {
                findings.push(Finding::error(
                    codes::DECISION_FIELD_TYPE_MISMATCH,
                    1,
                    format!(
                        "entry #{i} of `deviates_from` must be a `<origin>/<id>` \
                         string (for example `path:~/shared/0100`)"
                    ),
                ));
            }
        }
    }
    out
}

/// Parses an ADR file.
///
/// Returns a `Parsed<Option<Decision>>`: if the frontmatter is missing or
/// invalid, the value is `None` but the findings point to where the
/// problem is — the same mechanism as the spec parser.
pub fn parse_decision(source: &str) -> Parsed<Option<Decision>> {
    let mut findings = Vec::new();

    let Some((frontmatter_raw, body_offset)) = extract_frontmatter(source) else {
        findings.push(Finding::error(
            codes::DECISION_MISSING_FRONTMATTER,
            1,
            "this file has no YAML frontmatter delimited by `---` at the start and end",
        ));
        return Parsed {
            value: None,
            findings,
        };
    };

    let frontmatter: RawFrontmatter = match serde_norway::from_str(frontmatter_raw) {
        Ok(v) => v,
        Err(err) => {
            findings.push(Finding::error(
                codes::DECISION_MISSING_FIELD,
                1,
                format!("invalid YAML frontmatter: {err}"),
            ));
            return Parsed {
                value: None,
                findings,
            };
        }
    };

    // `deviates_from` is validated by hand to emit a specific finding
    // when its shape is not a list of strings.
    let deviates_from = match &frontmatter.deviates_from {
        Some(raw) => extract_deviates_from(raw, &mut findings),
        None => Vec::new(),
    };

    let status = DecisionStatus::from_raw(&frontmatter.status);
    if let DecisionStatus::Unknown(raw) = &status {
        findings.push(Finding::error(
            codes::DECISION_UNKNOWN_STATUS,
            1,
            format!(
                "unknown status `{raw}`; use one of: accepted, superseded, proposed, deprecated, rejected"
            ),
        ));
    }

    // The frontmatter runs from the first `---` (line 1) to the second
    // `---` (just before `body_offset`). Its end line is computed from
    // `body_offset`.
    let frontmatter_span = Span::new(0..body_offset, 1..line_at_byte(source, body_offset));
    let sections = split_sections(source, body_offset);

    Parsed {
        value: Some(Decision {
            id: frontmatter.id.into_string(),
            title: frontmatter.title,
            status,
            date: frontmatter.date,
            tags: frontmatter.tags,
            supersedes: frontmatter.supersedes.into_vec(),
            deviates_from,
            sections,
            frontmatter_span,
        }),
        findings,
    }
}

/// Returns `(frontmatter content, offset of the first byte of the body)`.
fn extract_frontmatter(source: &str) -> Option<(&str, usize)> {
    // Must start with `---\n` — `\r\n` is tolerated by normalizing to
    // `\n` at split time.
    let after_open = source.strip_prefix("---\n")?;
    // Look for the closing fence: a lone `---` line.
    let close_relative = find_close_fence(after_open)?;
    let frontmatter = &after_open[..close_relative];
    // `body_offset` in bytes in the original source: `---\n` (4) +
    // frontmatter + `---\n` (4) — unless the file ends right after
    // `---`, in which case the total length is used.
    let open_len = 4;
    let close_len = "---\n".len();
    let raw_offset = open_len + close_relative + close_len;
    let body_offset = raw_offset.min(source.len());
    Some((frontmatter, body_offset))
}

/// Looks for a `---` (closing) line in the content following the
/// opening. Returns the relative offset of the start of that line, or `None`.
fn find_close_fence(after_open: &str) -> Option<usize> {
    let mut cursor = 0;
    for line in after_open.split_inclusive('\n') {
        let trimmed = line.trim_end_matches(['\n', '\r']);
        if trimmed == "---" {
            return Some(cursor);
        }
        cursor += line.len();
    }
    None
}

fn line_at_byte(source: &str, byte: usize) -> u32 {
    let clamped = byte.min(source.len());
    1 + source[..clamped].bytes().filter(|&b| b == b'\n').count() as u32
}

/// Splits the body into top-level `## <title>` sections.
///
/// The preamble (before the first section) is ignored — our ADRs don't
/// have one. A future change could expose it if the need arises.
fn split_sections(source: &str, body_offset: usize) -> Vec<Section> {
    let body = &source[body_offset..];
    let base_line = line_at_byte(source, body_offset);

    // Locate the section starts (`## `, exactly two hashes followed by a
    // space) and their absolute byte position.
    let mut heads: Vec<(usize, usize, String)> = Vec::new(); // (abs_start, rel_start, name)
    let mut rel = 0usize;
    for line in body.split_inclusive('\n') {
        let stripped = line.trim_end_matches(['\n', '\r']);
        if let Some(name) = section_name(stripped) {
            heads.push((body_offset + rel, rel, name));
        }
        rel += line.len();
    }

    let total = source.len();
    let mut sections = Vec::with_capacity(heads.len());
    for (i, (abs_start, rel_start, name)) in heads.iter().enumerate() {
        let end = heads.get(i + 1).map(|(abs, _, _)| *abs).unwrap_or(total);
        let head_line_offset = body[..*rel_start].bytes().filter(|&b| b == b'\n').count() as u32;
        let start_line = base_line + head_line_offset;
        let inner_body = source[*abs_start..end]
            .lines()
            .skip(1) // skip the `## …` heading
            .collect::<Vec<_>>()
            .join("\n")
            .trim()
            .to_string();
        let end_line = line_at_byte(source, end);
        sections.push(Section {
            name: name.clone(),
            body: inner_body,
            span: Span::new(*abs_start..end, start_line..end_line),
        });
    }
    sections
}

fn section_name(line: &str) -> Option<String> {
    let after = line.strip_prefix("## ")?;
    if after.starts_with('#') {
        return None; // this is `###` or deeper
    }
    let trimmed = after.trim();
    if trimmed.is_empty() {
        return None;
    }
    Some(trimmed.to_string())
}

// ─────────────────────────── frontmatter deserialisation ───────────────────────────

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawFrontmatter {
    id: IdValue,
    title: String,
    status: String,
    date: String,
    #[serde(default)]
    tags: Vec<String>,
    #[serde(default)]
    supersedes: SupersedesValue,
    /// The field is read as a `Value` then validated by hand — this lets
    /// us emit `DECISION_FIELD_TYPE_MISMATCH` with a targeted message when
    /// the shape is wrong, rather than the generic `DECISION_MISSING_FIELD`
    /// that serde would produce.
    #[serde(default)]
    deviates_from: Option<serde_norway::Value>,
}

/// The `id` can be `0001` (YAML integer with a leading zero) or
/// `"my-decision"` (string). It is normalized to `String`, preserving the
/// four-digit "0001" look for an integer that starts with zeros.
#[derive(Debug)]
enum IdValue {
    Str(String),
    Int(i64),
}

impl IdValue {
    fn into_string(self) -> String {
        match self {
            Self::Str(s) => s,
            // 4-digit padding — the documentation convention of the
            // repository's ADRs (`0001`… `0006`). An id > 9999 overflows
            // and is written as-is, no truncation.
            Self::Int(n) if (0..10_000).contains(&n) => format!("{n:04}"),
            Self::Int(n) => n.to_string(),
        }
    }
}

impl<'de> Deserialize<'de> for IdValue {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{Error, Visitor};
        use std::fmt;

        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = IdValue;
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("an identifier as an integer or a string")
            }
            fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(IdValue::Str(v.to_string()))
            }
            fn visit_string<E: Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(IdValue::Str(v))
            }
            fn visit_i64<E: Error>(self, v: i64) -> Result<Self::Value, E> {
                Ok(IdValue::Int(v))
            }
            fn visit_u64<E: Error>(self, v: u64) -> Result<Self::Value, E> {
                Ok(IdValue::Int(v as i64))
            }
        }
        d.deserialize_any(V)
    }
}

#[derive(Debug, Default)]
struct SupersedesValue(Vec<String>);

impl SupersedesValue {
    fn into_vec(self) -> Vec<String> {
        self.0
    }
}

impl<'de> Deserialize<'de> for SupersedesValue {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{Error, SeqAccess, Visitor};
        use std::fmt;

        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = SupersedesValue;
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a single identifier or an array of identifiers")
            }
            fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(SupersedesValue(vec![v.to_string()]))
            }
            fn visit_i64<E: Error>(self, v: i64) -> Result<Self::Value, E> {
                Ok(SupersedesValue(vec![IdValue::Int(v).into_string()]))
            }
            fn visit_u64<E: Error>(self, v: u64) -> Result<Self::Value, E> {
                Ok(SupersedesValue(vec![IdValue::Int(v as i64).into_string()]))
            }
            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                let mut out = Vec::new();
                while let Some(id) = seq.next_element::<IdValue>()? {
                    out.push(id.into_string());
                }
                Ok(SupersedesValue(out))
            }
        }
        d.deserialize_any(V)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_well_formed_adr() {
        let source = "---\nid: 0007\ntitle: \"The title\"\nstatus: accepted\ndate: 2026-09-08\ntags: [architecture]\n---\n\n## Context\n\nSome text.\n\n## Decision\n\nAnother one.\n";
        let parsed = parse_decision(source);
        assert!(!parsed.has_errors(), "findings: {:?}", parsed.findings);
        let d = parsed.value.expect("decision expected");
        assert_eq!(d.id, "0007");
        assert_eq!(d.title, "The title");
        assert_eq!(d.status, DecisionStatus::Accepted);
        assert_eq!(d.date, "2026-09-08");
        assert_eq!(d.tags, ["architecture"]);
        assert!(d.supersedes.is_empty());
        assert_eq!(d.sections.len(), 2);
        assert_eq!(d.sections[0].name, "Context");
        assert!(d.sections[0].body.contains("Some text"));
    }

    #[test]
    fn parse_without_frontmatter_is_reported() {
        let source = "# No frontmatter\n\ncontent\n";
        let parsed = parse_decision(source);
        assert!(parsed.value.is_none());
        assert!(
            parsed
                .findings
                .iter()
                .any(|f| f.code == codes::DECISION_MISSING_FRONTMATTER)
        );
    }

    #[test]
    fn parse_without_title_is_reported() {
        // Missing required field: serde emits an error, which is reported
        // as `decision_missing_field`.
        let source = "---\nid: 0001\nstatus: accepted\ndate: 2026-09-08\n---\n\n## X\n";
        let parsed = parse_decision(source);
        assert!(parsed.value.is_none());
        assert!(
            parsed
                .findings
                .iter()
                .any(|f| f.code == codes::DECISION_MISSING_FIELD)
        );
    }

    #[test]
    fn unknown_status_is_reported() {
        let source = "---\nid: 0001\ntitle: T\nstatus: pending\ndate: 2026-09-08\n---\n";
        let parsed = parse_decision(source);
        let d = parsed.value.expect("decision still built");
        assert_eq!(d.status, DecisionStatus::Unknown("pending".into()));
        let f = parsed
            .findings
            .iter()
            .find(|f| f.code == codes::DECISION_UNKNOWN_STATUS)
            .expect("finding expected");
        assert!(f.message.contains("pending"), "{}", f.message);
        assert!(f.message.contains("accepted"), "{}", f.message);
    }

    #[test]
    fn supersedes_accepts_single_id_and_array() {
        let source_single =
            "---\nid: 0007\ntitle: T\nstatus: accepted\ndate: 2026-09-08\nsupersedes: 0003\n---\n";
        let d = parse_decision(source_single).value.unwrap();
        assert_eq!(d.supersedes, ["0003"]);

        let source_list = "---\nid: 0008\ntitle: T\nstatus: accepted\ndate: 2026-09-08\nsupersedes: [0003, 0004]\n---\n";
        let d = parse_decision(source_list).value.unwrap();
        assert_eq!(d.supersedes, ["0003", "0004"]);
    }

    #[test]
    fn string_id_is_accepted_as_is() {
        let source = "---\nid: my-decision\ntitle: T\nstatus: accepted\ndate: 2026-09-08\n---\n";
        let d = parse_decision(source).value.unwrap();
        assert_eq!(d.id, "my-decision");
    }

    #[test]
    fn deviates_from_list_of_strings_is_recognized() {
        let source = "---\nid: 0007\ntitle: T\nstatus: accepted\ndate: 2026-09-08\ndeviates_from: [\"path:~/shared/0100\"]\n---\n";
        let parsed = parse_decision(source);
        assert!(!parsed.has_errors(), "findings: {:?}", parsed.findings);
        let d = parsed.value.unwrap();
        assert_eq!(d.deviates_from, ["path:~/shared/0100"]);
    }

    #[test]
    fn deviates_from_with_several_entries() {
        let source = "---\nid: 0007\ntitle: T\nstatus: accepted\ndate: 2026-09-08\ndeviates_from: [\"path:~/a/0001\", \"git:acme/shared/0002\"]\n---\n";
        let d = parse_decision(source).value.unwrap();
        assert_eq!(d.deviates_from.len(), 2);
    }

    #[test]
    fn missing_deviates_from_gives_empty_list() {
        let source = "---\nid: 0001\ntitle: T\nstatus: accepted\ndate: 2026-09-08\n---\n";
        let d = parse_decision(source).value.unwrap();
        assert!(d.deviates_from.is_empty());
    }

    #[test]
    fn deviates_from_string_instead_of_list_is_reported() {
        // Single string where a list is expected → specific finding.
        let source = "---\nid: 0007\ntitle: T\nstatus: accepted\ndate: 2026-09-08\ndeviates_from: \"not-a-list\"\n---\n";
        let parsed = parse_decision(source);
        // The decision is still built (the error concerns an optional
        // field), but the finding is there.
        assert!(
            parsed
                .findings
                .iter()
                .any(|f| f.code == codes::DECISION_FIELD_TYPE_MISMATCH)
        );
    }

    #[test]
    fn deviates_from_non_string_entry_is_reported() {
        // A list where one of the items is an integer → specific finding
        // on that entry, the others are still exposed.
        let source = "---\nid: 0007\ntitle: T\nstatus: accepted\ndate: 2026-09-08\ndeviates_from: [\"path:~/a/0001\", 42]\n---\n";
        let parsed = parse_decision(source);
        let d = parsed.value.unwrap();
        assert_eq!(d.deviates_from, ["path:~/a/0001"]);
        assert!(
            parsed
                .findings
                .iter()
                .any(|f| f.code == codes::DECISION_FIELD_TYPE_MISMATCH)
        );
    }

    #[test]
    fn unknown_key_in_frontmatter_is_reported() {
        // deny_unknown_fields — consistent with ChangeMetadata and ProjectConfig.
        let source =
            "---\nid: 0001\ntitle: T\nstatus: accepted\ndate: 2026-09-08\nauthor: X\n---\n";

        let parsed = parse_decision(source);
        assert!(
            parsed
                .findings
                .iter()
                .any(|f| f.code == codes::DECISION_MISSING_FIELD)
        );
    }
}
