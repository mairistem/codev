//! The `## Release note` section of a proposal, and the `## What Changes`
//! bullets used when a change was archived without one.
//!
//! Pure: a `&str` in, the parsed note and its located defects out. The
//! parser is tolerant on form — a bullet may wrap, `*` works like `-`, the
//! template's HTML comment and fenced blocks are ignored — and strict on
//! content: a category outside the six of Keep a Changelog, or an item
//! without text, is a finding, never a silent guess.

use crate::keywords::{self, Key};
use crate::parser::ast::{Finding, Parsed};
use crate::parser::section::{heading_line, section};

use super::codes;

/// The six categories of Keep a Changelog, in its order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Category {
    Added,
    Changed,
    Deprecated,
    Removed,
    Fixed,
    Security,
}

impl Category {
    pub const ALL: [Category; 6] = [
        Self::Added,
        Self::Changed,
        Self::Deprecated,
        Self::Removed,
        Self::Fixed,
        Self::Security,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Added => "Added",
            Self::Changed => "Changed",
            Self::Deprecated => "Deprecated",
            Self::Removed => "Removed",
            Self::Fixed => "Fixed",
            Self::Security => "Security",
        }
    }

    /// The keyword of the category in the catalog.
    fn key(self) -> Key {
        match self {
            Self::Added => Key::CategoryAdded,
            Self::Changed => Key::CategoryChanged,
            Self::Deprecated => Key::CategoryDeprecated,
            Self::Removed => Key::CategoryRemoved,
            Self::Fixed => Key::CategoryFixed,
            Self::Security => Key::CategorySecurity,
        }
    }

    /// In any catalog language (`Ajouté` is `Added`), and case-insensitive:
    /// `added` is plainly meant as `Added`, and refusing it would teach
    /// nothing. [`Category::as_str`] stays English whatever was written.
    pub fn parse(raw: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|c| keywords::word(raw.trim(), c.key()).is_some())
    }
}

/// One item of a release note.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NoteItem {
    pub category: Category,
    /// The item's text as written, wrapped lines joined, `**BREAKING**`
    /// marker included.
    pub text: String,
    pub breaking: bool,
    /// 1-based line of the bullet in the proposal.
    pub line: u32,
}

/// What a proposal says about its release note.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReleaseNote {
    /// No `## Release note` section.
    Absent,
    /// `None`: users will not notice the change.
    Internal,
    /// The valid items; invalid ones are reported as findings only.
    Items(Vec<NoteItem>),
}

/// The heading of the section, in any language of the keyword catalog.
pub const HEADING: Key = Key::ReleaseNote;

/// Whether an item's text marks a breaking change. `**BREAKING:**` is
/// accepted too: it is how breaking items were written by hand so far.
pub fn is_breaking(text: &str) -> bool {
    text.starts_with("**BREAKING")
}

/// Parses the `## Release note` section of `proposal`.
pub fn parse(proposal: &str) -> Parsed<ReleaseNote> {
    let Some(lines) = section(proposal, HEADING) else {
        return Parsed::new(ReleaseNote::Absent);
    };
    let (bullets, strays) = bullets(&lines);
    let mut findings = Vec::new();

    let none_lines: Vec<u32> = strays
        .iter()
        .filter(|(_, text)| is_none(text))
        .map(|(line, _)| *line)
        .collect();
    for (line, text) in strays.iter().filter(|(_, text)| !is_none(text)) {
        findings.push(Finding::error(
            codes::RELEASE_NOTE_INVALID,
            *line,
            format!("line {line}: `{text}` is neither a `- <Category>: <text>` item nor `None`"),
        ));
    }
    if !none_lines.is_empty() && !bullets.is_empty() {
        findings.push(Finding::error(
            codes::RELEASE_NOTE_INVALID,
            none_lines[0],
            format!(
                "line {}: `None` cannot be combined with release note items; keep one or the other",
                none_lines[0]
            ),
        ));
    }
    if bullets.is_empty() && strays.is_empty() {
        let line = heading_line(proposal, HEADING).unwrap_or(1);
        findings.push(Finding::error(
            codes::RELEASE_NOTE_INVALID,
            line,
            format!(
                "line {line}: the release note is empty; write `- <Category>: <text>` items, \
                 or `None` for an internal change"
            ),
        ));
    }

    let mut items = Vec::new();
    for (line, content) in bullets {
        match item(line, &content) {
            Ok(item) => items.push(item),
            Err(message) => findings.push(Finding::error(
                codes::RELEASE_NOTE_INVALID,
                line,
                format!("line {line}: {message}"),
            )),
        }
    }

    let value = if items.is_empty() && !none_lines.is_empty() {
        ReleaseNote::Internal
    } else {
        ReleaseNote::Items(items)
    };
    Parsed { value, findings }
}

/// The bullets of `## What Changes`, wrapped lines and nested bullets
/// joined into their parent, for a change archived without a release note.
pub fn what_changes(proposal: &str) -> Vec<String> {
    section(proposal, Key::WhatChanges)
        .map(|lines| bullets(&lines).0.into_iter().map(|(_, t)| t).collect())
        .unwrap_or_default()
}

/// `None`, or its translation in a catalog language (`Aucune`).
fn is_none(text: &str) -> bool {
    keywords::word(text.trim().trim_end_matches('.'), Key::None).is_some()
}

fn item(line: u32, content: &str) -> Result<NoteItem, String> {
    let Some((raw_category, text)) = content.split_once(':') else {
        return Err(format!(
            "`{content}` has no category; write `- <Category>: <text>` with one of {}",
            categories()
        ));
    };
    let Some(category) = Category::parse(raw_category) else {
        return Err(format!(
            "unknown category `{}`; use one of {}",
            raw_category.trim(),
            categories()
        ));
    };
    let text = text.trim();
    if text.is_empty() {
        return Err(format!(
            "the `{}` item has no text; describe the change for end users",
            category.as_str()
        ));
    }
    Ok(NoteItem {
        category,
        text: text.to_string(),
        breaking: is_breaking(text),
        line,
    })
}

fn categories() -> String {
    Category::ALL
        .iter()
        .map(|c| format!("`{}`", c.as_str()))
        .collect::<Vec<_>>()
        .join(", ")
}

/// A text with the 1-based line it starts on.
type Numbered = (u32, String);

/// Splits section lines into bullets — text joined across wrapped lines —
/// and stray non-bullet lines.
fn bullets(lines: &[(u32, &str)]) -> (Vec<Numbered>, Vec<Numbered>) {
    let mut bullets: Vec<Numbered> = Vec::new();
    let mut strays = Vec::new();
    let mut after_blank = false;
    for (line, raw) in lines {
        if raw.trim().is_empty() {
            after_blank = true;
            continue;
        }
        let indented = raw.starts_with(' ') || raw.starts_with('\t');
        if !indented && let Some(content) = bullet_content(raw) {
            bullets.push((*line, content.trim().to_string()));
        } else if let Some((_, text)) = bullets.last_mut()
            && (indented || !after_blank)
        {
            // A wrapped line, or a nested bullet folded into its parent.
            let continuation = bullet_content(raw.trim_start()).unwrap_or(raw).trim();
            if !text.is_empty() {
                text.push(' ');
            }
            text.push_str(continuation);
        } else {
            strays.push((*line, raw.trim().to_string()));
        }
        after_blank = false;
    }
    (bullets, strays)
}

fn bullet_content(line: &str) -> Option<&str> {
    if line == "-" || line == "*" {
        return Some("");
    }
    line.strip_prefix("- ")
        .or_else(|| line.strip_prefix("* "))
        .or_else(|| line.strip_prefix("-\t"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn proposal(note: &str) -> String {
        format!(
            "# Proposal: X\n\n## Why\n\nBecause.\n\n## Impact\n\n- code\n\n## Release note\n\n{note}\n"
        )
    }

    fn items(source: &str) -> Vec<NoteItem> {
        match parse(source).value {
            ReleaseNote::Items(items) => items,
            other => panic!("expected items, got {other:?}"),
        }
    }

    #[test]
    fn parses_wrapped_and_breaking_items() {
        let source =
            proposal("- Removed: **BREAKING** the legacy\n  flag is gone.\n* Fixed: a crash.");
        let parsed = parse(&source);
        assert!(parsed.findings.is_empty(), "{:?}", parsed.findings);
        let items = items(&source);
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].category, Category::Removed);
        assert_eq!(items[0].text, "**BREAKING** the legacy flag is gone.");
        assert!(items[0].breaking);
        assert_eq!(items[0].line, 13);
        assert_eq!(items[1].category, Category::Fixed);
        assert!(!items[1].breaking);
    }

    #[test]
    fn none_is_an_internal_change() {
        for none in ["None", "None.", "none"] {
            let parsed = parse(&proposal(none));
            assert_eq!(parsed.value, ReleaseNote::Internal, "{none}");
            assert!(parsed.findings.is_empty());
        }
    }

    #[test]
    fn absent_section_is_absent() {
        let parsed = parse("# Proposal: X\n\n## Why\n\nx\n");
        assert_eq!(parsed.value, ReleaseNote::Absent);
        assert!(parsed.findings.is_empty());
    }

    #[test]
    fn unknown_category_is_invalid_with_its_line() {
        let parsed = parse(&proposal("- Added: ok.\n- Improved: speed."));
        assert_eq!(parsed.findings.len(), 1);
        let finding = &parsed.findings[0];
        assert_eq!(finding.code, codes::RELEASE_NOTE_INVALID);
        assert_eq!(finding.line, 14);
        assert!(finding.message.contains("Improved"));
        // The valid item is kept.
        assert_eq!(
            items(&proposal("- Added: ok.\n- Improved: speed.")).len(),
            1
        );
    }

    #[test]
    fn empty_bullet_is_invalid() {
        for bullet in ["- Fixed:", "- Fixed:   ", "-"] {
            let parsed = parse(&proposal(bullet));
            assert_eq!(parsed.findings.len(), 1, "{bullet}");
            assert_eq!(parsed.findings[0].code, codes::RELEASE_NOTE_INVALID);
        }
    }

    #[test]
    fn empty_section_is_invalid() {
        let parsed = parse("## Release note\n\n## Other\n");
        assert_eq!(parsed.findings.len(), 1);
        assert_eq!(parsed.findings[0].line, 1);
        assert!(parsed.findings[0].message.contains("empty"));
    }

    #[test]
    fn stray_text_and_none_mixed_with_items_are_invalid() {
        let parsed = parse(&proposal("Some prose.\n\n- Added: x."));
        assert_eq!(parsed.findings.len(), 1);
        assert!(parsed.findings[0].message.contains("Some prose."));
        let parsed = parse(&proposal("None\n\n- Added: x."));
        assert_eq!(parsed.findings.len(), 1);
        assert!(parsed.findings[0].message.contains("cannot be combined"));
    }

    #[test]
    fn template_comment_is_ignored() {
        let source = "## Release note\n\n<!-- What end users read.\n     - Added: example -->\n\n- Added: real.\n";
        let parsed = parse(source);
        assert!(parsed.findings.is_empty(), "{:?}", parsed.findings);
        assert_eq!(items(source).len(), 1);
        // A section holding only the template comment is empty.
        let parsed = parse("## Release note\n\n<!-- comment -->\n");
        assert_eq!(parsed.findings.len(), 1);
    }

    #[test]
    fn windows_line_endings_are_accepted() {
        let source = proposal("- Added: a\r\n  b.").replace('\n', "\r\n");
        let items = items(&source);
        assert_eq!(items[0].text, "a b.");
    }

    #[test]
    fn what_changes_bullets_are_joined() {
        let source = "## Why\n\nx\n\n## What Changes\n\n- **BREAKING** first\n  wrapped.\n- second, with:\n  - a nested point\n\n## Capabilities\n\n- not this\n";
        assert_eq!(
            what_changes(source),
            vec![
                "**BREAKING** first wrapped.".to_string(),
                "second, with: a nested point".to_string()
            ]
        );
        assert!(what_changes("## Why\n").is_empty());
    }

    fn french(note: &str) -> String {
        format!("# Proposition : X\n\n## Impact\n\n- code\n\n## Note de version\n\n{note}\n")
    }

    #[test]
    fn french_release_note_is_valid() {
        let source = french("- Ajouté : un export.\n- Modifié : **BREAKING** un défaut.");
        let parsed = parse(&source);
        assert!(parsed.findings.is_empty(), "{:?}", parsed.findings);
        let found = items(&source);
        assert_eq!(found[0].category, Category::Added);
        assert_eq!(found[0].text, "un export.");
        assert!(found[1].breaking);
    }

    #[test]
    fn an_unknown_french_category_is_reported() {
        let parsed = parse(&french("- Amélioré : la vitesse."));
        let f = &parsed.findings[0];
        assert_eq!(f.code, codes::RELEASE_NOTE_INVALID);
        assert_eq!(f.line, 9);
        assert!(f.message.contains("`Amélioré`"), "{}", f.message);
    }

    #[test]
    fn aucune_release_note_is_an_internal_change() {
        let parsed = parse(&french("Aucune"));
        assert!(parsed.findings.is_empty(), "{:?}", parsed.findings);
        assert_eq!(parsed.value, ReleaseNote::Internal);
    }

    #[test]
    fn french_categories_parse_to_the_english_category() {
        let pairs = [
            ("Ajouté", Category::Added),
            ("modifie", Category::Changed),
            ("Obsolète", Category::Deprecated),
            ("Supprimé", Category::Removed),
            ("Corrigé", Category::Fixed),
            ("Sécurité", Category::Security),
        ];
        for (raw, expected) in pairs {
            assert_eq!(Category::parse(raw), Some(expected), "{raw}");
        }
        assert_eq!(Category::parse("Ajouté").unwrap().as_str(), "Added");
    }

    #[test]
    fn french_what_changes_bullets_are_read() {
        let source = "# P\n\n## Changements\n\n- Un export mensuel.\n";
        assert_eq!(what_changes(source), ["Un export mensuel."]);
    }
}
