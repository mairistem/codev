//! The corrections of a change, `corrections.md`: the instructions the
//! author gave the agent about how the project does things, one entry per
//! correction, so that they outlive the session.
//!
//! Pure (decision 0001): the file comes in as a string, its entries and
//! located findings come out. The layout follows the review record's: a
//! format marker, one heading per entry and one fenced `yaml` block of
//! fields, read field by field so that a field added by a newer codev is a
//! warning, not a broken file.

use std::collections::BTreeSet;

use serde_norway::Value;

use crate::parser::ast::{Finding, Parsed};

use codes::{
    CORRECTIONS_FORMAT_UNSUPPORTED, CORRECTIONS_INVALID, CORRECTIONS_TOO_LONG,
    CORRECTIONS_UNKNOWN_FIELD,
};

/// The file's name, in the change's directory.
pub const FILE: &str = "corrections.md";

/// The format this codev reads and writes. A file marked with a greater
/// number comes from a newer codev and is not checked.
pub const FORMAT: u32 = 1;

/// The skills that note corrections, as an entry's `step` names them.
pub const STEPS: &[&str] = &["propose", "apply", "update"];

/// Above this length, `did` or `asked` is a transcript rather than an
/// instruction.
pub const MAX_TEXT_CHARS: usize = 300;

const FIELDS: &[&str] = &["step", "where", "did", "asked"];

/// Stable codes of the file's findings.
pub mod codes {
    /// A structural defect: a heading, an id, a block or a field.
    pub const CORRECTIONS_INVALID: &str = "corrections_invalid";
    /// A field this codev does not know (warning).
    pub const CORRECTIONS_UNKNOWN_FIELD: &str = "corrections_unknown_field";
    /// A `did` or `asked` longer than an instruction needs (warning).
    pub const CORRECTIONS_TOO_LONG: &str = "corrections_too_long";
    /// A file written in a newer format (warning).
    pub const CORRECTIONS_FORMAT_UNSUPPORTED: &str = "corrections_format_unsupported";
}

/// One correction, as its entry states it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Correction {
    pub id: String,
    pub title: String,
    /// The line of the entry's heading.
    pub line: u32,
    pub step: String,
    pub r#where: String,
    pub did: String,
    pub asked: String,
}

/// What a `corrections.md` holds.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Corrections {
    /// The format marker; `None` when the file has none, which makes it
    /// some other document of the same name, neither checked nor listed.
    pub format: Option<u32>,
    pub entries: Vec<Correction>,
}

/// Parses a corrections file. The value holds the well-formed entries; the
/// findings say what is wrong, each at its line.
pub fn parse(source: &str) -> Parsed<Corrections> {
    let lines: Vec<&str> = source
        .split('\n')
        .map(|line| line.trim_end_matches('\r'))
        .collect();
    let Some(format) = format_marker(&lines) else {
        return Parsed::new(Corrections::default());
    };
    let mut parsed = Parsed::new(Corrections {
        format: Some(format),
        entries: Vec::new(),
    });
    if format > FORMAT {
        parsed.findings.push(Finding::warning(
            CORRECTIONS_FORMAT_UNSUPPORTED,
            1,
            format!(
                "the corrections file is in format {format}, written by a newer codev; this codev reads format {FORMAT} and checks nothing in it — update codev"
            ),
        ));
        return parsed;
    }
    let mut reader = Reader {
        ids: BTreeSet::new(),
        open: None,
        parsed,
    };
    reader.run(&lines);
    reader.parsed
}

/// `<!-- codev-corrections: <n> -->`, anywhere in the file.
fn format_marker(lines: &[&str]) -> Option<u32> {
    lines.iter().find_map(|line| {
        line.trim()
            .strip_prefix("<!--")?
            .strip_suffix("-->")?
            .trim()
            .strip_prefix("codev-corrections:")?
            .trim()
            .parse()
            .ok()
    })
}

fn line_no(index: usize) -> u32 {
    u32::try_from(index + 1).unwrap_or(u32::MAX)
}

/// An entry heading whose block has not been read yet.
struct OpenEntry {
    id: String,
    title: String,
    line: u32,
    has_block: bool,
}

struct Reader {
    ids: BTreeSet<String>,
    open: Option<OpenEntry>,
    parsed: Parsed<Corrections>,
}

impl Reader {
    fn run(&mut self, lines: &[&str]) {
        let mut index = 0;
        while index < lines.len() {
            let trimmed = lines[index].trim();
            if let Some(info) = trimmed.strip_prefix("```") {
                let close = lines[index + 1..]
                    .iter()
                    .position(|l| l.trim().starts_with("```"))
                    .map(|offset| index + 1 + offset);
                let body_end = close.unwrap_or(lines.len());
                if info.trim() == "yaml" {
                    self.block(&lines[index + 1..body_end], index);
                }
                index = close.map_or(lines.len(), |c| c + 1);
                continue;
            }
            if let Some(text) = lines[index].strip_prefix("### ") {
                self.heading(text.trim(), index);
            }
            index += 1;
        }
        self.close();
    }

    fn error(&mut self, line: u32, message: String) {
        self.parsed
            .findings
            .push(Finding::error(CORRECTIONS_INVALID, line, message));
    }

    fn heading(&mut self, text: &str, index: usize) {
        self.close();
        let line = line_no(index);
        let (id, title) = text.split_once(char::is_whitespace).unwrap_or((text, ""));
        let (id, title) = (id.to_string(), title.trim().to_string());
        let number = id.strip_prefix("C-").and_then(|n| n.parse::<u32>().ok());
        if !matches!(number, Some(n) if n > 0) {
            self.error(
                line,
                format!(
                    "`### {text}` is not a correction: an entry is `### C-<n> <title>`, n from 1"
                ),
            );
        } else if !self.ids.insert(id.clone()) {
            self.error(
                line,
                format!("`{id}` is already used; take the next free number"),
            );
        } else if title.is_empty() {
            self.error(
                line,
                format!("`### {id}` has no title; write `### {id} <the instruction>`"),
            );
        }
        self.open = Some(OpenEntry {
            id,
            title,
            line,
            has_block: false,
        });
    }

    /// Reports an entry heading that never got its block.
    fn close(&mut self) {
        if let Some(open) = self.open.take()
            && !open.has_block
        {
            self.error(
                open.line,
                format!(
                    "{} has no `yaml` block; give it `step`, `where`, `did` and `asked`",
                    open.id
                ),
            );
        }
    }

    /// A field the format defines, as trimmed text; `None`, reported, when
    /// it is missing, empty or not text.
    fn required_text(
        &mut self,
        mapping: &serde_norway::Mapping,
        field: &str,
        id: &str,
        line: u32,
    ) -> Option<String> {
        match mapping.get(field) {
            None => self.error(line, format!("{id}: `{field}` is missing")),
            Some(Value::String(text)) if text.trim().is_empty() => {
                self.error(line, format!("{id}: `{field}` is empty"));
            }
            Some(Value::String(text)) => return Some(text.trim().to_string()),
            Some(_) => self.error(line, format!("{id}: `{field}` must be text")),
        }
        None
    }

    fn warn_unknown_fields(&mut self, mapping: &serde_norway::Mapping, id: &str, line: u32) {
        for key in mapping.keys() {
            let name = key.as_str().unwrap_or("?");
            if !FIELDS.contains(&name) {
                self.parsed.findings.push(Finding::warning(
                    CORRECTIONS_UNKNOWN_FIELD,
                    line,
                    format!(
                        "{id}: `{name}` is not a field of a correction ({}); it is ignored",
                        FIELDS.join(", ")
                    ),
                ));
            }
        }
    }

    fn warn_long_text(&mut self, field: &str, text: &str, id: &str, line: u32) {
        let length = text.chars().count();
        if length > MAX_TEXT_CHARS {
            self.parsed.findings.push(Finding::warning(
                CORRECTIONS_TOO_LONG,
                line,
                format!(
                    "{id}: `{field}` holds {length} characters; state the instruction in one or two lines, at most {MAX_TEXT_CHARS}, never the conversation"
                ),
            ));
        }
    }

    fn block(&mut self, body: &[&str], fence_index: usize) {
        let line = line_no(fence_index);
        // A block outside an entry is free text.
        let Some(open) = self.open.as_mut() else {
            return;
        };
        if open.has_block {
            let id = open.id.clone();
            self.error(
                line,
                format!("{id} has a second `yaml` block; keep one per entry"),
            );
            return;
        }
        open.has_block = true;
        let (id, title, heading_line) = (open.id.clone(), open.title.clone(), open.line);
        let mapping = match serde_norway::from_str::<Value>(&body.join("\n")) {
            Ok(Value::Mapping(mapping)) => mapping,
            Ok(_) => {
                self.error(
                    line,
                    format!("{id}: the block must be a YAML mapping of `step`, `where`, `did` and `asked`"),
                );
                return;
            }
            Err(error) => {
                self.error(
                    line,
                    format!(
                        "{id}: the block is not valid YAML ({error}); quote a value that holds `: `"
                    ),
                );
                return;
            }
        };
        self.warn_unknown_fields(&mapping, &id, line);
        let (Some(step), Some(r#where), Some(did), Some(asked)) = (
            self.required_text(&mapping, "step", &id, line),
            self.required_text(&mapping, "where", &id, line),
            self.required_text(&mapping, "did", &id, line),
            self.required_text(&mapping, "asked", &id, line),
        ) else {
            // Each missing or malformed field has been reported.
            return;
        };
        if !STEPS.contains(&step.as_str()) {
            self.error(
                line,
                format!(
                    "{id}: `step: {step}` is not a skill that notes corrections ({})",
                    STEPS.join(", ")
                ),
            );
            return;
        }
        self.warn_long_text("did", &did, &id, line);
        self.warn_long_text("asked", &asked, &id, line);
        self.parsed.value.entries.push(Correction {
            id,
            title,
            line: heading_line,
            step,
            r#where,
            did,
            asked,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ENTRY: &str = "### C-1 Typed errors in the libraries\n\n```yaml\nstep: apply\nwhere: crates/codev-engine/src/search.rs\ndid: Returned an anyhow error from an engine function.\nasked: Use a thiserror type; anyhow stays in the CLI.\n```\n";

    fn file(body: &str) -> String {
        format!("# Corrections: add-auth\n\n<!-- codev-corrections: 1 -->\n\n{body}")
    }

    fn codes(source: &str) -> Vec<&'static str> {
        parse(source).findings.iter().map(|f| f.code).collect()
    }

    #[test]
    fn a_well_formed_corrections_file_has_no_finding() {
        let parsed = parse(&file(ENTRY));
        assert!(parsed.findings.is_empty(), "{:?}", parsed.findings);
        let entry = &parsed.value.entries[0];
        assert_eq!(entry.id, "C-1");
        assert_eq!(entry.title, "Typed errors in the libraries");
        assert_eq!(entry.step, "apply");
        assert_eq!(entry.r#where, "crates/codev-engine/src/search.rs");
        assert_eq!(entry.line, 5);
    }

    #[test]
    fn windows_line_endings_are_read() {
        let parsed = parse(&file(ENTRY).replace('\n', "\r\n"));
        assert!(parsed.findings.is_empty(), "{:?}", parsed.findings);
        assert_eq!(parsed.value.entries.len(), 1);
    }

    #[test]
    fn a_file_without_the_marker_is_not_read() {
        let parsed = parse("# Notes\n\n### Meeting\n\nFree text.\n");
        assert!(parsed.findings.is_empty());
        assert_eq!(parsed.value, Corrections::default());
    }

    #[test]
    fn a_missing_field_is_invalid() {
        let parsed = parse(&file(&ENTRY.replace(
            "asked: Use a thiserror type; anyhow stays in the CLI.\n",
            "",
        )));
        assert_eq!(parsed.findings.len(), 1);
        assert_eq!(parsed.findings[0].code, CORRECTIONS_INVALID);
        assert!(parsed.findings[0].message.contains("`asked`"));
        assert_eq!(parsed.findings[0].line, 7);
        assert!(parsed.value.entries.is_empty());
    }

    #[test]
    fn an_unknown_step_names_the_accepted_ones() {
        let source = file(&ENTRY.replace("step: apply", "step: archive"));
        let parsed = parse(&source);
        assert_eq!(codes(&source), [CORRECTIONS_INVALID]);
        assert!(
            parsed.findings[0]
                .message
                .contains("propose, apply, update")
        );
    }

    #[test]
    fn an_identifier_used_twice_is_invalid() {
        let source = file(&format!("{ENTRY}\n{ENTRY}"));
        let parsed = parse(&source);
        assert_eq!(codes(&source), [CORRECTIONS_INVALID]);
        assert!(parsed.findings[0].message.contains("already used"));
        assert_eq!(parsed.findings[0].line, 14);
    }

    #[test]
    fn an_unknown_field_is_a_warning() {
        let source = file(&ENTRY.replace("step: apply", "step: apply\ncategory: style"));
        let parsed = parse(&source);
        assert_eq!(codes(&source), [CORRECTIONS_UNKNOWN_FIELD]);
        assert!(parsed.findings[0].message.contains("`category`"));
        assert_eq!(parsed.value.entries.len(), 1);
    }

    #[test]
    fn a_long_entry_is_a_warning() {
        let long = "x".repeat(450);
        let source = file(&ENTRY.replace(
            "asked: Use a thiserror type; anyhow stays in the CLI.",
            &format!("asked: {long}"),
        ));
        assert_eq!(codes(&source), [CORRECTIONS_TOO_LONG]);
    }

    #[test]
    fn a_newer_format_is_not_checked() {
        let source = "<!-- codev-corrections: 2 -->\n\n### not an entry\n";
        let parsed = parse(source);
        assert_eq!(codes(source), [CORRECTIONS_FORMAT_UNSUPPORTED]);
        assert!(parsed.value.entries.is_empty());
    }

    #[test]
    fn a_heading_without_its_block_is_invalid() {
        let source = file("### C-1 A rule\n\nNo block.\n");
        assert_eq!(codes(&source), [CORRECTIONS_INVALID]);
    }

    #[test]
    fn a_heading_that_is_not_an_entry_is_invalid() {
        let source = file(&ENTRY.replace("### C-1", "### Note"));
        assert!(codes(&source).contains(&CORRECTIONS_INVALID));
    }
}
