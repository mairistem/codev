//! Release notes built from archived changes: which items, in which order,
//! rendered how, and written where in `CHANGELOG.md`.
//!
//! Everything here is a decision, so everything here is pure
//! (`_codev/decisions/0001-functional-core-imperative-shell.md`): the engine
//! reads the archive and asks git which changes are in range, this module
//! turns them into text, and the shell writes it through a plan.

pub mod file;
pub mod release_note;
pub mod version;

pub use release_note::{Category, NoteItem, ReleaseNote};

/// Stable codes of the release note findings and warnings.
pub mod codes {
    /// A proposal without a `## Release note` section.
    pub const RELEASE_NOTE_MISSING: &str = "release_note_missing";
    /// An unknown category, an item without text, an empty section.
    pub const RELEASE_NOTE_INVALID: &str = "release_note_invalid";
    /// Requirements removed without a `Removed` or breaking item.
    pub const RELEASE_NOTE_INCONSISTENT: &str = "release_note_inconsistent";

    /// The codes a validation report can carry, for the uniqueness test.
    pub const ALL: &[&str] = &[
        RELEASE_NOTE_MISSING,
        RELEASE_NOTE_INVALID,
        RELEASE_NOTE_INCONSISTENT,
    ];
}

/// An archived change in range, as read by the engine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeInput {
    /// The archive folder name, `<date>-<name>`.
    pub name: String,
    /// The archive folder, relative to the project root, `/`-separated.
    pub archive_path: String,
    pub proposal: String,
    /// One of its spec deltas has a `## REMOVED Requirements` entry.
    pub removes_requirements: bool,
}

/// Where an entry's text comes from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntrySource {
    ReleaseNote,
    /// The change has no release note: a `## What Changes` bullet.
    WhatChanges,
}

/// One line of the changelog.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub category: Category,
    pub text: String,
    pub breaking: bool,
    pub change: String,
    pub archive_path: String,
    pub source: EntrySource,
}

/// A warning about the content, never blocking.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Notice {
    pub code: &'static str,
    pub message: String,
}

/// The entries in display order, and what the maintainer should look at.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Collected {
    pub entries: Vec<Entry>,
    pub notices: Vec<Notice>,
}

/// Turns the changes in range into ordered entries.
///
/// - `exclude`: archive folder names described by hand, left out silently;
/// - `None` notes contribute nothing;
/// - a change without a note falls back on its What Changes bullets, under
///   `Changed`, with a `release_note_missing` notice;
/// - order: categories in Keep a Changelog order, breaking items first,
///   then archive order (folder names sort by archive date).
pub fn collect(changes: &[ChangeInput], exclude: &[String]) -> Collected {
    let mut sorted: Vec<&ChangeInput> = changes
        .iter()
        .filter(|c| !exclude.iter().any(|e| e == &c.name))
        .collect();
    sorted.sort_by(|a, b| a.name.cmp(&b.name));

    let mut collected = Collected::default();
    for change in sorted {
        let parsed = release_note::parse(&change.proposal);
        let entry = |category, text: String, breaking, source| Entry {
            category,
            text,
            breaking,
            change: change.name.clone(),
            archive_path: change.archive_path.clone(),
            source,
        };
        if !parsed.findings.is_empty() {
            collected.notices.push(Notice {
                code: codes::RELEASE_NOTE_INVALID,
                message: format!(
                    "{}: {} invalid release note item(s) left out; fix them in {}/proposal.md",
                    change.name,
                    parsed.findings.len(),
                    change.archive_path
                ),
            });
        }
        let items = match parsed.value {
            ReleaseNote::Absent => {
                let bullets = release_note::what_changes(&change.proposal);
                collected.notices.push(Notice {
                    code: codes::RELEASE_NOTE_MISSING,
                    message: format!(
                        "{}: no `## Release note`; its {} What Changes bullet(s) are listed under Changed — edit them in CHANGELOG.md if needed",
                        change.name,
                        bullets.len()
                    ),
                });
                for text in bullets {
                    let breaking = release_note::is_breaking(&text);
                    collected.entries.push(entry(
                        Category::Changed,
                        text,
                        breaking,
                        EntrySource::WhatChanges,
                    ));
                }
                continue;
            }
            ReleaseNote::Internal => Vec::new(),
            ReleaseNote::Items(items) => items,
        };
        if change.removes_requirements
            && !items
                .iter()
                .any(|i| i.category == Category::Removed || i.breaking)
        {
            collected.notices.push(Notice {
                code: codes::RELEASE_NOTE_INCONSISTENT,
                message: format!(
                    "{}: its specs remove requirements, but its release note has neither a `Removed` item nor a **BREAKING** one",
                    change.name
                ),
            });
        }
        for item in items {
            collected.entries.push(entry(
                item.category,
                item.text,
                item.breaking,
                EntrySource::ReleaseNote,
            ));
        }
    }
    // Stable: archive order survives within (category, breaking).
    collected.entries.sort_by_key(|e| (e.category, !e.breaking));
    collected
}

/// The heading above the categories.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Heading {
    /// `## [Unreleased]`.
    Unreleased,
    /// `## [<version>] - <date>`.
    Release { version: String, date: String },
    /// No heading: the body of a release on a forge.
    None,
}

impl Heading {
    fn line(&self) -> Option<String> {
        match self {
            Self::Unreleased => Some("## [Unreleased]".into()),
            Self::Release { version, date } => Some(format!("## [{version}] - {date}")),
            Self::None => None,
        }
    }
}

/// Renders the entries as Keep a Changelog markdown: the heading, then one
/// `### <Category>` per non-empty category. Ends with a line break unless
/// there is nothing at all to print.
pub fn render(entries: &[Entry], heading: &Heading) -> String {
    let mut blocks: Vec<String> = Vec::new();
    if let Some(line) = heading.line() {
        blocks.push(line);
    }
    for category in Category::ALL {
        let items: Vec<String> = entries
            .iter()
            .filter(|e| e.category == category)
            .map(|e| format!("- {}", e.text))
            .collect();
        if !items.is_empty() {
            blocks.push(format!("### {}\n\n{}", category.as_str(), items.join("\n")));
        }
    }
    if blocks.is_empty() {
        return String::new();
    }
    format!("{}\n", blocks.join("\n\n"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn change(name: &str, note: Option<&str>) -> ChangeInput {
        let mut proposal =
            String::from("# Proposal: X\n\n## Why\n\nx\n\n## What Changes\n\n- first\n- second\n");
        if let Some(note) = note {
            proposal.push_str(&format!("\n## Release note\n\n{note}\n"));
        }
        ChangeInput {
            name: name.into(),
            archive_path: format!("_codev/changes/archive/{name}"),
            proposal,
            removes_requirements: false,
        }
    }

    fn texts(collected: &Collected) -> Vec<&str> {
        collected.entries.iter().map(|e| e.text.as_str()).collect()
    }

    #[test]
    fn none_and_excluded_changes_contribute_nothing() {
        let changes = [
            change("2026-01-01-internal", Some("None")),
            change("2026-01-02-by-hand", None),
            change("2026-01-03-feature", Some("- Added: a flag.")),
        ];
        let collected = collect(&changes, &["2026-01-02-by-hand".to_string()]);
        assert_eq!(texts(&collected), ["a flag."]);
        assert!(collected.notices.is_empty(), "{:?}", collected.notices);
        assert_eq!(collected.entries[0].change, "2026-01-03-feature");
        assert_eq!(
            collected.entries[0].archive_path,
            "_codev/changes/archive/2026-01-03-feature"
        );
    }

    #[test]
    fn fallback_uses_what_changes_and_warns() {
        let collected = collect(&[change("2026-01-01-old", None)], &[]);
        assert_eq!(texts(&collected), ["first", "second"]);
        assert!(
            collected
                .entries
                .iter()
                .all(|e| e.category == Category::Changed && e.source == EntrySource::WhatChanges)
        );
        assert_eq!(collected.notices.len(), 1);
        assert_eq!(collected.notices[0].code, codes::RELEASE_NOTE_MISSING);
        assert!(collected.notices[0].message.contains("2026-01-01-old"));
    }

    #[test]
    fn removed_requirements_without_removed_item_warn() {
        let mut removing = change("2026-01-01-drop", Some("- Added: a new flag."));
        removing.removes_requirements = true;
        let collected = collect(&[removing.clone()], &[]);
        assert_eq!(collected.notices.len(), 1);
        assert_eq!(collected.notices[0].code, codes::RELEASE_NOTE_INCONSISTENT);

        for note in ["- Removed: the old flag.", "- Changed: **BREAKING** x."] {
            removing.proposal = change("x", Some(note)).proposal;
            assert!(
                collect(&[removing.clone()], &[]).notices.is_empty(),
                "{note}"
            );
        }
    }

    #[test]
    fn breaking_first_then_archive_order_by_category() {
        let changes = [
            change("2026-01-02-b", Some("- Added: **BREAKING** z")),
            change("2026-01-01-a", Some("- Fixed: x\n- Added: y")),
        ];
        let collected = collect(&changes, &[]);
        assert_eq!(texts(&collected), ["**BREAKING** z", "y", "x"]);
        let rendered = render(&collected.entries, &Heading::Unreleased);
        assert_eq!(
            rendered,
            "## [Unreleased]\n\n### Added\n\n- **BREAKING** z\n- y\n\n### Fixed\n\n- x\n"
        );
    }

    #[test]
    fn invalid_items_are_left_out_with_a_notice() {
        let collected = collect(
            &[change("2026-01-01-a", Some("- Added: ok\n- Improved: no"))],
            &[],
        );
        assert_eq!(texts(&collected), ["ok"]);
        assert_eq!(collected.notices[0].code, codes::RELEASE_NOTE_INVALID);
    }

    #[test]
    fn release_heading_carries_version_and_date() {
        let collected = collect(&[change("2026-01-01-a", Some("- Fixed: x"))], &[]);
        let heading = Heading::Release {
            version: "1.2.0".into(),
            date: "2026-10-20".into(),
        };
        assert!(
            render(&collected.entries, &heading)
                .starts_with("## [1.2.0] - 2026-10-20\n\n### Fixed")
        );
        assert_eq!(render(&[], &heading), "## [1.2.0] - 2026-10-20\n");
    }

    #[test]
    fn markdown_has_no_version_heading() {
        let collected = collect(&[change("2026-01-01-a", Some("- Fixed: x"))], &[]);
        assert_eq!(
            render(&collected.entries, &Heading::None),
            "### Fixed\n\n- x\n"
        );
        assert_eq!(render(&[], &Heading::None), "");
    }
}
