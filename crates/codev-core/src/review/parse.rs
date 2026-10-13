//! Reads `review.md`: headings give the structure (step, role, finding),
//! one fenced `yaml` block per finding gives its fields.
//!
//! The block is read as a YAML value, field by field, rather than into a
//! struct that refuses unknown fields: a field added by a newer codev is a
//! warning here, not a broken record.

use std::collections::BTreeSet;

use serde_norway::{Mapping, Value};

use crate::parser::ast::{Finding, Parsed};

use super::codes::{
    REVIEW_FORMAT_UNSUPPORTED, REVIEW_INVALID, REVIEW_UNKNOWN_FIELD, REVIEW_UNKNOWN_SECTION,
};
use super::{
    Confidence, Entry, FORMAT, Location, Record, ReviewFinding, RoleSection, RoleState, STEPS,
    Severity, Status, StepSection, history,
};

const FINDING_FIELDS: &[&str] = &[
    "severity",
    "confidence",
    "location",
    "evidence",
    "recommendation",
    "history",
];
const ENTRY_FIELDS: &[&str] = &["status", "by", "note", "ref", "to"];

/// Parses a record. The value holds what could be read; the findings say
/// what is wrong, each at its line.
pub fn parse(source: &str) -> Parsed<Record> {
    let lines: Vec<&str> = source
        .split('\n')
        .map(|line| line.trim_end_matches('\r'))
        .collect();
    let format = format_marker(&lines).unwrap_or(1);
    if format > FORMAT {
        let mut parsed = Parsed::new(Record {
            format,
            steps: Vec::new(),
        });
        parsed.findings.push(Finding::warning(
            REVIEW_FORMAT_UNSUPPORTED,
            1,
            format!(
                "the review record is in format {format}, written by a newer codev; this codev reads format {FORMAT} and checks nothing in it — update codev"
            ),
        ));
        return parsed;
    }
    let mut parser = Parser {
        record: Record {
            format,
            steps: Vec::new(),
        },
        findings: Vec::new(),
        ids: BTreeSet::new(),
        skipping: false,
        role: None,
        finding: None,
    };
    parser.run(&lines);
    Parsed {
        value: parser.record,
        findings: parser.findings,
    }
}

/// `<!-- codev-review: <n> -->`, anywhere in the file.
fn format_marker(lines: &[&str]) -> Option<u32> {
    lines.iter().find_map(|line| {
        line.trim()
            .strip_prefix("<!--")?
            .strip_suffix("-->")?
            .trim()
            .strip_prefix("codev-review:")?
            .trim()
            .parse()
            .ok()
    })
}

fn line_no(index: usize) -> u32 {
    u32::try_from(index + 1).unwrap_or(u32::MAX)
}

/// A role section being read.
struct OpenRole {
    role: String,
    line: u32,
    no_finding: bool,
    disabled: bool,
    /// Finding headings seen, even those whose block could not be read.
    headings: usize,
    findings: Vec<ReviewFinding>,
}

/// A finding heading whose block has not been read yet.
struct OpenFinding {
    id: String,
    title: String,
    line: u32,
    has_block: bool,
}

struct Parser {
    record: Record,
    findings: Vec<Finding>,
    ids: BTreeSet<String>,
    /// Inside an unknown or duplicate step: its content is ignored.
    skipping: bool,
    role: Option<OpenRole>,
    finding: Option<OpenFinding>,
}

impl Parser {
    fn run(&mut self, lines: &[&str]) {
        let mut index = 0;
        let mut in_comment = false;
        while index < lines.len() {
            let line = lines[index];
            let trimmed = line.trim();
            if in_comment {
                in_comment = !trimmed.contains("-->");
                index += 1;
                continue;
            }
            if trimmed.starts_with("<!--") && !trimmed.contains("-->") {
                in_comment = true;
                index += 1;
                continue;
            }
            if let Some(info) = trimmed.strip_prefix("```") {
                let close = lines[index + 1..]
                    .iter()
                    .position(|l| l.trim().starts_with("```"))
                    .map(|offset| index + 1 + offset);
                let body_end = close.unwrap_or(lines.len());
                if info.trim() == "yaml" && !self.skipping {
                    self.block(&lines[index + 1..body_end], index);
                }
                index = close.map_or(lines.len(), |c| c + 1);
                continue;
            }
            if let Some(text) = line.strip_prefix("#### ") {
                self.finding_heading(text, index);
            } else if let Some(text) = line.strip_prefix("### ") {
                self.role_heading(text.trim(), index);
            } else if let Some(text) = line.strip_prefix("## ") {
                self.step_heading(text.trim(), index);
            } else if line.starts_with("# ") {
                // The title: free text.
            } else if !self.skipping {
                self.body_line(trimmed);
            }
            index += 1;
        }
        self.close_role();
    }

    fn step_heading(&mut self, step: &str, index: usize) {
        self.close_role();
        let line = line_no(index);
        if !STEPS.contains(&step) {
            self.findings.push(Finding::warning(
                REVIEW_UNKNOWN_SECTION,
                line,
                format!(
                    "`## {step}` is not a reviewing step ({}); its content is ignored",
                    STEPS.join(", ")
                ),
            ));
            self.skipping = true;
        } else if self.record.step(step).is_some() {
            self.findings.push(Finding::error(
                REVIEW_INVALID,
                line,
                format!("`## {step}` is written twice; keep one section per step"),
            ));
            self.skipping = true;
        } else {
            self.skipping = false;
            self.record.steps.push(StepSection {
                step: step.to_string(),
                line,
                roles: Vec::new(),
            });
        }
    }

    fn role_heading(&mut self, role: &str, index: usize) {
        self.close_role();
        if self.skipping {
            return;
        }
        let line = line_no(index);
        let Some(step) = self.record.steps.last() else {
            self.findings.push(Finding::error(
                REVIEW_INVALID,
                line,
                format!("`### {role}` is outside a step; put it under `## plan.challenge` or `## code.review`"),
            ));
            return;
        };
        if role.is_empty() || role.contains(char::is_whitespace) {
            self.findings.push(Finding::error(
                REVIEW_INVALID,
                line,
                format!("`### {role}` must be a role id, such as `### skeptic`"),
            ));
            return;
        }
        if step.role(role).is_some() {
            self.findings.push(Finding::error(
                REVIEW_INVALID,
                line,
                format!("`### {role}` is written twice under `## {}`", step.step),
            ));
            return;
        }
        self.role = Some(OpenRole {
            role: role.to_string(),
            line,
            no_finding: false,
            disabled: false,
            headings: 0,
            findings: Vec::new(),
        });
    }

    fn finding_heading(&mut self, text: &str, index: usize) {
        self.close_finding();
        if self.skipping {
            return;
        }
        let line = line_no(index);
        let Some(role) = &self.role else {
            self.findings.push(Finding::error(
                REVIEW_INVALID,
                line,
                format!(
                    "`#### {}` is outside a role; put it under `### <role-id>`",
                    text.trim()
                ),
            ));
            return;
        };
        let role_id = role.role.clone();
        let text = text.trim();
        let (id, title) = text.split_once(char::is_whitespace).unwrap_or((text, ""));
        let (id, title) = (id.to_string(), title.trim().to_string());
        let prefix = format!("R-{role_id}-");
        let number = id.strip_prefix(&prefix).and_then(|n| n.parse::<u32>().ok());
        if !matches!(number, Some(n) if n > 0) {
            self.findings.push(Finding::error(
                REVIEW_INVALID,
                line,
                format!(
                    "`{id}` is not a finding id of `{role_id}`: it must be `{prefix}<n>`, n from 1"
                ),
            ));
        } else if !self.ids.insert(id.clone()) {
            self.findings.push(Finding::error(
                REVIEW_INVALID,
                line,
                format!(
                    "`{id}` is already used; an id is never reused — take the next free number"
                ),
            ));
        }
        if title.is_empty() {
            self.findings.push(Finding::error(
                REVIEW_INVALID,
                line,
                format!("`#### {id}` has no title; write `#### {id} <what is wrong>`"),
            ));
        }
        if let Some(role) = self.role.as_mut() {
            role.headings += 1;
        }
        self.finding = Some(OpenFinding {
            id,
            title,
            line,
            has_block: false,
        });
    }

    fn body_line(&mut self, trimmed: &str) {
        let Some(role) = &mut self.role else {
            return;
        };
        let sentence = trimmed.trim_end_matches('.').to_ascii_lowercase();
        if sentence == "no finding" {
            role.no_finding = true;
        } else if sentence == "disabled by configuration" {
            role.disabled = true;
        }
    }

    fn block(&mut self, body: &[&str], fence_index: usize) {
        let line = line_no(fence_index);
        let Some(open) = self.finding.as_mut() else {
            return;
        };
        if open.has_block {
            self.findings.push(Finding::error(
                REVIEW_INVALID,
                line,
                format!(
                    "{} has a second `yaml` block; keep one per finding",
                    open.id
                ),
            ));
            return;
        }
        open.has_block = true;
        let heading = Heading {
            id: open.id.clone(),
            title: open.title.clone(),
            line: open.line,
            role: self
                .role
                .as_ref()
                .map(|r| r.role.clone())
                .unwrap_or_default(),
            step: self
                .record
                .steps
                .last()
                .map(|s| s.step.clone())
                .unwrap_or_default(),
        };
        let id = heading.id.clone();
        let mapping = match serde_norway::from_str::<Value>(&body.join("\n")) {
            Ok(Value::Mapping(mapping)) => mapping,
            Ok(_) => {
                self.findings.push(Finding::error(
                    REVIEW_INVALID,
                    line,
                    format!("{id}: the block must be a YAML mapping of the finding's fields"),
                ));
                return;
            }
            Err(error) => {
                self.findings.push(Finding::error(
                    REVIEW_INVALID,
                    line,
                    format!(
                        "{id}: the block is not valid YAML ({error}); quote a value that holds `: ` or starts with a special character, such as `location: \"design.md#Decision: X\"`"
                    ),
                ));
                return;
            }
        };
        let mut fields = Fields {
            id: &id,
            line,
            findings: &mut self.findings,
            history_readable: true,
        };
        let finding = fields.finding(&mapping, heading);
        // An unreadable entry was reported already; judging the
        // transitions of what is left would only add noise.
        if fields.history_readable {
            self.findings.extend(history::check(&finding, line));
        }
        if let Some(role) = self.role.as_mut() {
            role.findings.push(finding);
        }
    }

    fn close_finding(&mut self) {
        if let Some(open) = self.finding.take()
            && !open.has_block
        {
            self.findings.push(Finding::error(
                REVIEW_INVALID,
                open.line,
                format!("{} has no ```yaml block with its fields", open.id),
            ));
        }
    }

    fn close_role(&mut self) {
        self.close_finding();
        let Some(open) = self.role.take() else {
            return;
        };
        let has_findings = open.headings > 0;
        let says =
            usize::from(open.no_finding) + usize::from(open.disabled) + usize::from(has_findings);
        let state = if says > 1 {
            self.findings.push(Finding::error(
                REVIEW_INVALID,
                open.line,
                format!(
                    "`### {}` mixes findings, `No finding.` and `Disabled by configuration.`; keep one",
                    open.role
                ),
            ));
            RoleState::Empty
        } else if has_findings {
            RoleState::Findings
        } else if open.no_finding {
            RoleState::NoFinding
        } else if open.disabled {
            RoleState::Disabled
        } else {
            self.findings.push(Finding::error(
                REVIEW_INVALID,
                open.line,
                format!(
                    "`### {}` holds no finding; write `No finding.` when the role found nothing, or `Disabled by configuration.`",
                    open.role
                ),
            ));
            RoleState::Empty
        };
        if let Some(step) = self.record.steps.last_mut() {
            step.roles.push(RoleSection {
                role: open.role,
                line: open.line,
                state,
                findings: open.findings,
            });
        }
    }
}

/// Where a finding sits: what its `####` heading and the sections above it
/// say.
struct Heading {
    id: String,
    title: String,
    step: String,
    role: String,
    line: u32,
}

/// Reads the fields of one block, reporting at the block's line.
struct Fields<'a> {
    id: &'a str,
    line: u32,
    findings: &'a mut Vec<Finding>,
    history_readable: bool,
}

impl Fields<'_> {
    fn finding(&mut self, mapping: &Mapping, heading: Heading) -> ReviewFinding {
        self.unknown_keys(mapping, FINDING_FIELDS, "");
        let severity = self.required(mapping, "severity").and_then(|text| {
            let parsed = Severity::parse(&text);
            if parsed.is_none() {
                self.invalid(format!(
                    "`severity: {text}` is not a severity; use high, medium or low"
                ));
            }
            parsed
        });
        let confidence = self.required(mapping, "confidence").and_then(|text| {
            let parsed = Confidence::parse(&text);
            if parsed.is_none() {
                self.invalid(format!(
                    "`confidence: {text}` is not a confidence; use high or low"
                ));
            }
            parsed
        });
        let location_text = self
            .required(mapping, "location")
            .map(|text| text.replace('\\', "/"))
            .unwrap_or_default();
        let location = if location_text.is_empty() {
            None
        } else {
            let parsed = parse_location(&location_text);
            if let Err(reason) = &parsed {
                self.invalid(format!("`location: {location_text}` {reason}"));
            }
            parsed.ok()
        };
        let evidence = self.required(mapping, "evidence").unwrap_or_default();
        let recommendation = self.required(mapping, "recommendation").unwrap_or_default();
        let history = self.history(mapping);
        ReviewFinding {
            id: heading.id,
            title: heading.title,
            step: heading.step,
            role: heading.role,
            line: heading.line,
            severity,
            confidence,
            location_text,
            location,
            evidence,
            recommendation,
            history,
        }
    }

    fn invalid(&mut self, message: String) {
        self.findings.push(Finding::error(
            REVIEW_INVALID,
            self.line,
            format!("{}: {message}", self.id),
        ));
    }

    fn unknown_keys(&mut self, mapping: &Mapping, known: &[&str], within: &str) {
        for key in mapping.keys() {
            let name = key.as_str().unwrap_or("?");
            if !known.contains(&name) {
                self.findings.push(Finding::warning(
                    REVIEW_UNKNOWN_FIELD,
                    self.line,
                    format!(
                        "{}: `{within}{name}` is not a field this codev knows; it is ignored",
                        self.id
                    ),
                ));
            }
        }
    }

    /// A non-empty text field; reports it missing otherwise.
    fn required(&mut self, mapping: &Mapping, key: &str) -> Option<String> {
        let text = scalar(mapping.get(key));
        if text.as_deref().is_none_or(|t| t.trim().is_empty()) {
            self.invalid(format!("`{key}` is missing or empty"));
            return None;
        }
        text
    }

    fn history(&mut self, mapping: &Mapping) -> Vec<Entry> {
        let entries = self.entries(mapping);
        self.history_readable = entries.is_some();
        entries.unwrap_or_default()
    }

    /// `None` once a defect is reported.
    fn entries(&mut self, mapping: &Mapping) -> Option<Vec<Entry>> {
        let Some(value) = mapping.get("history") else {
            self.invalid("`history` is missing".into());
            return None;
        };
        let Value::Sequence(items) = value else {
            self.invalid("`history` must be a list of entries".into());
            return None;
        };
        let mut entries = Vec::new();
        for (n, item) in items.iter().enumerate() {
            let Value::Mapping(entry) = item else {
                self.invalid(format!("history entry {} is not a mapping", n + 1));
                return None;
            };
            self.unknown_keys(entry, ENTRY_FIELDS, "history.");
            let status_text = scalar(entry.get("status")).unwrap_or_default();
            let Some(status) = Status::parse(status_text.trim()) else {
                self.invalid(format!(
                    "history entry {}: `status: {status_text}` is not a status; use {}",
                    n + 1,
                    Status::ALL.map(Status::as_str).join(", ")
                ));
                return None;
            };
            let by = scalar(entry.get("by"))
                .unwrap_or_default()
                .trim()
                .to_string();
            if by.is_empty() {
                self.invalid(format!(
                    "history entry {} has no `by`: a role id, `human` or `human:<name>`",
                    n + 1
                ));
                return None;
            }
            entries.push(Entry {
                status,
                by,
                note: scalar(entry.get("note")),
                reference: scalar(entry.get("ref")),
                to: scalar(entry.get("to")),
            });
        }
        Some(entries)
    }
}

/// A scalar as text: YAML reads `12` or `true` as non-strings.
fn scalar(value: Option<&Value>) -> Option<String> {
    match value? {
        Value::String(text) => Some(text.clone()),
        Value::Number(number) => Some(number.to_string()),
        Value::Bool(flag) => Some(flag.to_string()),
        _ => None,
    }
}

/// `<path>:<line>`, `<path>:<start>-<end>` or `<file>#<heading>`, `\`
/// already read as `/`. The error completes "`location: …` …".
fn parse_location(text: &str) -> Result<Location, String> {
    if let Some((file, heading)) = text.split_once('#') {
        check_path(file)?;
        if heading.trim().is_empty() {
            return Err("names no heading after `#`".into());
        }
        return Ok(Location::Plan {
            file: file.to_string(),
            heading: heading.trim().to_string(),
        });
    }
    let shape = "must be `<path>:<line>`, `<path>:<start>-<end>` or `<file>#<heading>`";
    let (path, range) = text.rsplit_once(':').ok_or_else(|| shape.to_string())?;
    let (start, end) = match range.split_once('-') {
        Some((start, end)) => (start.parse::<u32>(), end.parse::<u32>()),
        None => (range.parse::<u32>(), range.parse::<u32>()),
    };
    let (Ok(start), Ok(end)) = (start, end) else {
        return Err(shape.into());
    };
    if start == 0 || end < start {
        return Err("has a line range that is not `start <= end`, from line 1".into());
    }
    check_path(path)?;
    Ok(Location::Code {
        path: path.to_string(),
        start,
        end,
    })
}

/// A path judged as text, the same on every OS: relative to its base, and
/// unable to leave it. `std::path` would read `C:x` as relative on Unix.
fn check_path(path: &str) -> Result<(), String> {
    if path.is_empty() {
        return Err("names no file".into());
    }
    if path.starts_with('/') {
        return Err("is absolute; cite a path relative to the project root".into());
    }
    let segments: Vec<&str> = path.split('/').collect();
    if segments[0].contains(':') {
        return Err(
            "starts with a drive or a scheme; cite a path relative to the project root".into(),
        );
    }
    if segments
        .iter()
        .any(|s| s.is_empty() || *s == "." || *s == "..")
    {
        return Err("holds an empty, `.` or `..` segment; cite a path inside the project".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast::Severity as FindingSeverity;
    use crate::review::codes::REVIEW_DECISION_INCOMPLETE;

    const FINDING: &str = "#### R-security-1 Tokens in clear\n\n```yaml\nseverity: high\nconfidence: high\nlocation: src/db.rs:12-13\nevidence: |\n  store(token)\nrecommendation: Hash it.\nhistory:\n  - status: open\n    by: security\n```\n";

    fn record(body: &str) -> String {
        format!("# Review record: x\n\n<!-- codev-review: 1 -->\n\n{body}")
    }

    fn codes(source: &str) -> Vec<&'static str> {
        parse(source).findings.iter().map(|f| f.code).collect()
    }

    fn messages(source: &str) -> String {
        parse(source)
            .findings
            .iter()
            .map(|f| format!("{} {}: {}", f.code, f.line, f.message))
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn with_block(fields: &str) -> String {
        record(&format!(
            "## code.review\n\n### security\n\n#### R-security-1 Title\n\n```yaml\n{fields}```\n"
        ))
    }

    const HISTORY_OPEN: &str = "history:\n  - status: open\n    by: security\n";

    fn fields(severity: &str, location: &str, history: &str) -> String {
        format!(
            "severity: {severity}\nconfidence: high\nlocation: {location}\nevidence: x\nrecommendation: y\n{history}"
        )
    }

    #[test]
    fn a_well_formed_record_parses_steps_roles_and_findings() {
        let source = record(&format!(
            "## plan.challenge\n\n### skeptic\n\nNo finding.\n\n### security\n\n{FINDING}\n### performance\n\nDisabled by configuration.\n"
        ));
        let parsed = parse(&source);
        assert!(parsed.findings.is_empty(), "{}", messages(&source));
        let step = parsed.value.step("plan.challenge").unwrap();
        assert_eq!(step.role("skeptic").unwrap().state, RoleState::NoFinding);
        assert_eq!(step.role("performance").unwrap().state, RoleState::Disabled);
        let security = step.role("security").unwrap();
        assert_eq!(security.state, RoleState::Findings);
        let finding = &security.findings[0];
        assert_eq!(finding.id, "R-security-1");
        assert_eq!(finding.title, "Tokens in clear");
        assert_eq!(finding.step, "plan.challenge");
        assert_eq!(finding.severity, Some(Severity::High));
        assert_eq!(
            finding.location,
            Some(Location::Code {
                path: "src/db.rs".into(),
                start: 12,
                end: 13
            })
        );
        assert_eq!(finding.evidence, "store(token)\n");
        assert_eq!(finding.status(), Some(Status::Open));
        assert_eq!(parsed.value.format, 1);
    }

    #[test]
    fn a_crlf_record_parses_like_lf() {
        let source =
            record(&format!("## code.review\n\n### security\n\n{FINDING}")).replace('\n', "\r\n");
        let parsed = parse(&source);
        assert!(parsed.findings.is_empty(), "{}", messages(&source));
        let finding = parsed.value.findings().next().unwrap();
        assert_eq!(finding.evidence.trim(), "store(token)");
    }

    #[test]
    fn an_unknown_severity_is_invalid() {
        let source = with_block(&fields("critical", "src/a.rs:1", HISTORY_OPEN));
        assert_eq!(codes(&source), [REVIEW_INVALID]);
        let text = messages(&source);
        assert!(
            text.contains("critical") && text.contains("high, medium or low"),
            "{text}"
        );
    }

    #[test]
    fn a_role_with_nothing_in_it_is_invalid() {
        let source = record("## plan.challenge\n\n### skeptic\n\nI looked at it.\n");
        let parsed = parse(&source);
        assert_eq!(parsed.findings.len(), 1);
        assert_eq!(parsed.findings[0].code, REVIEW_INVALID);
        assert_eq!(parsed.findings[0].line, 7);
        assert!(parsed.findings[0].message.contains("No finding."));
    }

    #[test]
    fn a_block_that_is_not_yaml_is_invalid() {
        let source = with_block("severity: [high\n");
        let parsed = parse(&source);
        assert_eq!(codes(&source), [REVIEW_INVALID]);
        assert_eq!(parsed.findings[0].line, 11);
        assert!(parsed.findings[0].message.contains("not valid YAML"));
    }

    #[test]
    fn an_unknown_step_is_a_warning() {
        let source = record(&format!("## code.reveiw\n\n### security\n\n{FINDING}"));
        let parsed = parse(&source);
        assert_eq!(codes(&source), [REVIEW_UNKNOWN_SECTION]);
        assert_eq!(parsed.findings[0].severity, FindingSeverity::Warning);
        assert!(parsed.findings[0].message.contains("code.reveiw"));
        assert!(parsed.value.steps.is_empty());
    }

    #[test]
    fn structure_errors_are_located() {
        let outside_role =
            record("## code.review\n\n#### R-x-1 T\n\n```yaml\nseverity: low\n```\n");
        assert!(messages(&outside_role).contains("outside a role"));
        let outside_step = record("### skeptic\n\nNo finding.\n");
        assert!(messages(&outside_step).contains("outside a step"));
        let twice = record(
            "## plan.challenge\n\n### skeptic\n\nNo finding.\n\n### skeptic\n\nNo finding.\n",
        );
        assert!(messages(&twice).contains("written twice"));
        let step_twice =
            record("## plan.challenge\n\n### skeptic\n\nNo finding.\n\n## plan.challenge\n");
        assert!(messages(&step_twice).contains("written twice"));
        let no_block =
            record("## code.review\n\n### newcomer\n\n#### R-newcomer-1 Title\n\nprose\n");
        assert!(messages(&no_block).contains("no ```yaml block"));
        let mixed = record(&format!(
            "## code.review\n\n### security\n\nNo finding.\n\n{FINDING}"
        ));
        assert!(messages(&mixed).contains("mixes findings"));
    }

    #[test]
    fn a_finding_without_evidence_is_invalid() {
        let source = with_block(
            "severity: low\nconfidence: high\nlocation: a.rs:1\nrecommendation: y\nhistory:\n  - status: open\n    by: security\n",
        );
        assert_eq!(codes(&source), [REVIEW_INVALID]);
        let text = messages(&source);
        assert!(
            text.contains("R-security-1") && text.contains("`evidence`"),
            "{text}"
        );
    }

    #[test]
    fn an_id_must_match_its_role() {
        let source = record(&format!(
            "## code.review\n\n### privacy\n\n{}",
            FINDING.replace("by: security", "by: privacy")
        ));
        let text = messages(&source);
        assert!(text.contains("R-privacy-"), "{text}");
        assert_eq!(codes(&source)[0], REVIEW_INVALID);
    }

    #[test]
    fn a_duplicate_id_is_invalid() {
        let source = record(&format!(
            "## plan.challenge\n\n### security\n\n{FINDING}\n## code.review\n\n### security\n\n{FINDING}"
        ));
        let parsed = parse(&source);
        assert_eq!(codes(&source), [REVIEW_INVALID]);
        assert!(parsed.findings[0].message.contains("already used"));
        assert!(parsed.findings[0].line > 20);
    }

    #[test]
    fn both_kinds_of_location_parse() {
        assert_eq!(
            parse_location("src/db.rs:12-14"),
            Ok(Location::Code {
                path: "src/db.rs".into(),
                start: 12,
                end: 14
            })
        );
        assert_eq!(
            parse_location("specs/auth/spec.md#Requirement: Login"),
            Ok(Location::Plan {
                file: "specs/auth/spec.md".into(),
                heading: "Requirement: Login".into()
            })
        );
        assert!(parse_location("src/db.rs").is_err());
        assert!(parse_location("src/db.rs:0").is_err());
        assert!(parse_location("src/db.rs:9-3").is_err());
        assert!(parse_location("design.md#").is_err());
    }

    #[test]
    fn a_location_outside_the_project_is_invalid() {
        for location in [
            "../secrets.txt:1",
            "/etc/passwd:1",
            "C:/secrets.txt:1",
            "src/../../x.rs:1",
            "src//x.rs:1",
            "../design.md#Why",
        ] {
            assert!(parse_location(location).is_err(), "{location}");
        }
        let source = with_block(&fields("low", "C:\\secrets.txt:1", HISTORY_OPEN));
        assert_eq!(codes(&source), [REVIEW_INVALID]);
        assert_eq!(
            parse(&source).value.findings().next().unwrap().location,
            None
        );
    }

    #[test]
    fn backslashes_in_a_location_read_as_slashes() {
        let source = with_block(&fields("low", "src\\db.rs:3", HISTORY_OPEN));
        let parsed = parse(&source);
        assert!(parsed.findings.is_empty(), "{}", messages(&source));
        let finding = parsed.value.findings().next().unwrap();
        assert_eq!(finding.location_text, "src/db.rs:3");
    }

    #[test]
    fn an_unknown_field_is_a_warning() {
        let source = with_block(&format!(
            "{}category: injection\n",
            fields("low", "a.rs:1", HISTORY_OPEN)
        ));
        let parsed = parse(&source);
        assert_eq!(codes(&source), [REVIEW_UNKNOWN_FIELD]);
        assert!(parsed.findings[0].message.contains("category"));
        assert_eq!(parsed.value.findings().count(), 1);
    }

    #[test]
    fn a_newer_format_is_not_checked() {
        let source = "<!-- codev-review: 2 -->\n\n## code.review\n\n### whatever\n\nnonsense\n";
        let parsed = parse(source);
        assert_eq!(codes(source), [REVIEW_FORMAT_UNSUPPORTED]);
        assert_eq!(parsed.value.format, 2);
        assert!(!parsed.value.is_supported());
        assert!(parsed.value.steps.is_empty());
    }

    #[test]
    fn a_record_without_marker_is_format_one() {
        let parsed = parse("## plan.challenge\n\n### skeptic\n\nNo finding.\n");
        assert!(parsed.findings.is_empty());
        assert_eq!(parsed.value.format, 1);
    }

    #[test]
    fn headings_inside_comments_and_other_fences_are_ignored() {
        let source = record(
            "<!--\n## code.review\n-->\n\n## plan.challenge\n\n### skeptic\n\n```text\n### not-a-role\n```\n\nNo finding.\n",
        );
        let parsed = parse(&source);
        assert!(parsed.findings.is_empty(), "{}", messages(&source));
        assert_eq!(parsed.value.steps.len(), 1);
        assert_eq!(parsed.value.steps[0].roles.len(), 1);
    }

    // ─────────────── history ───────────────

    fn history_codes(severity: &str, history: &str) -> (Vec<&'static str>, String) {
        let source = with_block(&fields(severity, "a.rs:1", history));
        (codes(&source), messages(&source))
    }

    #[test]
    fn an_accepted_risk_needs_a_human() {
        let (codes, text) = history_codes(
            "high",
            "history:\n  - status: open\n    by: security\n  - status: accepted-risk\n    by: developer\n    note: internal tool\n",
        );
        assert_eq!(codes, [REVIEW_DECISION_INCOMPLETE]);
        assert!(text.contains("decided by a human"), "{text}");
        let (codes, _) = history_codes(
            "high",
            "history:\n  - status: open\n    by: security\n  - status: accepted-risk\n    by: human:Ludovic\n    note: internal tool\n",
        );
        assert!(codes.is_empty());
        let (codes, text) = history_codes(
            "high",
            "history:\n  - status: open\n    by: security\n  - status: accepted-risk\n    by: human\n",
        );
        assert_eq!(codes, [REVIEW_DECISION_INCOMPLETE]);
        assert!(text.contains("justification"), "{text}");
    }

    #[test]
    fn a_fix_needs_a_reference() {
        let (codes, text) = history_codes(
            "medium",
            "history:\n  - status: open\n    by: security\n  - status: fixed\n    by: developer\n",
        );
        assert_eq!(codes, [REVIEW_DECISION_INCOMPLETE]);
        assert!(text.contains("`ref`"), "{text}");
    }

    #[test]
    fn a_high_finding_is_deferred_by_a_human() {
        let (codes, text) = history_codes(
            "high",
            "history:\n  - status: open\n    by: security\n  - status: deferred\n    by: developer\n    to: harden-sessions\n",
        );
        assert_eq!(codes, [REVIEW_DECISION_INCOMPLETE]);
        assert!(text.contains("deferred by a human"), "{text}");
        // A medium one, the author may defer.
        let (codes, _) = history_codes(
            "medium",
            "history:\n  - status: open\n    by: security\n  - status: deferred\n    by: developer\n    to: harden-sessions\n",
        );
        assert!(codes.is_empty());
        let (codes, text) = history_codes(
            "medium",
            "history:\n  - status: open\n    by: security\n  - status: deferred\n    by: developer\n",
        );
        assert_eq!(codes, [REVIEW_DECISION_INCOMPLETE]);
        assert!(text.contains("`to`"), "{text}");
    }

    #[test]
    fn only_the_reviewer_or_a_human_rejects() {
        let (codes, text) = history_codes(
            "low",
            "history:\n  - status: open\n    by: security\n  - status: disputed\n    by: developer\n    note: not reachable\n  - status: rejected\n    by: developer\n",
        );
        assert_eq!(codes, [REVIEW_INVALID]);
        assert!(text.contains("only the reviewer"), "{text}");
    }

    #[test]
    fn a_complete_loop_is_valid() {
        let (codes, text) = history_codes(
            "high",
            "history:\n  - status: open\n    by: security\n  - status: disputed\n    by: developer\n    note: the value is a constant (src/a.rs:3)\n  - status: rejected\n    by: security\n",
        );
        assert!(codes.is_empty(), "{text}");
        let (codes, text) = history_codes(
            "high",
            "history:\n  - status: open\n    by: security\n  - status: fixed\n    by: developer\n    ref: abc1234\n  - status: open\n    by: security\n    note: still concatenated on line 9\n  - status: fixed\n    by: developer\n    ref: def5678\n  - status: fixed\n    by: security\n    note: verified\n",
        );
        assert!(codes.is_empty(), "{text}");
    }

    #[test]
    fn a_history_must_start_open_by_its_role() {
        let (codes, text) = history_codes("low", "history:\n  - status: open\n    by: developer\n");
        assert_eq!(codes, [REVIEW_INVALID]);
        assert!(
            text.contains("starts with `status: open`, `by: security`"),
            "{text}"
        );
        let (codes, _) = history_codes("low", "history: []\n");
        assert_eq!(codes, [REVIEW_INVALID]);
        let (codes, text) =
            history_codes("low", "history:\n  - status: wontfix\n    by: security\n");
        assert_eq!(codes, [REVIEW_INVALID]);
        assert!(text.contains("wontfix"), "{text}");
    }

    #[test]
    fn a_transition_outside_the_table_is_invalid() {
        let (codes, text) = history_codes(
            "low",
            "history:\n  - status: open\n    by: security\n  - status: rejected\n    by: security\n",
        );
        assert_eq!(codes, [REVIEW_INVALID]);
        assert!(text.contains("cannot follow `open`"), "{text}");
        let (codes, text) = history_codes(
            "low",
            "history:\n  - status: open\n    by: security\n  - status: fixed\n    by: developer\n    ref: x\n  - status: fixed\n    by: developer\n",
        );
        assert_eq!(codes, [REVIEW_INVALID]);
        assert!(text.contains("verifies or reopens"), "{text}");
    }

    #[test]
    fn awaiting_follows_the_last_answer() {
        let awaiting = |history: &str| {
            let source = with_block(&fields("high", "a.rs:1", history));
            parse(&source).value.findings().next().unwrap().awaiting()
        };
        use super::super::Awaiting;
        assert_eq!(awaiting(HISTORY_OPEN), Some(Awaiting::Author));
        let disputed = "history:\n  - status: open\n    by: security\n  - status: disputed\n    by: developer\n    note: n\n";
        assert_eq!(awaiting(disputed), Some(Awaiting::Reviewer));
        let maintained =
            format!("{disputed}  - status: disputed\n    by: security\n    note: still\n");
        assert_eq!(awaiting(&maintained), Some(Awaiting::Human));
        let fixed = "history:\n  - status: open\n    by: security\n  - status: fixed\n    by: developer\n    ref: r\n";
        assert_eq!(awaiting(fixed), Some(Awaiting::Reviewer));
        let verified = format!("{fixed}  - status: fixed\n    by: security\n");
        assert_eq!(awaiting(&verified), None);
    }
}
