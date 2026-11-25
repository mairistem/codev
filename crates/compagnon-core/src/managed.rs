//! Managed sections: compagnon's text inside a file that belongs to the user.
//!
//! `CLAUDE.md` and a pull request template are the team's files. compagnon
//! only owns the lines between two HTML comments — invisible once the
//! markdown is rendered — and never touches a byte outside them. The
//! decision is pure: [`splice`] receives the current content and returns
//! the new one; the shell writes it through a plan
//! (`_compagnon/decisions/0001-functional-core-imperative-shell.md`).
//!
//! A file compagnon owns entirely (the generated CI workflow) uses a simpler
//! convention: a first line starting with [`MANAGED_YAML_MARKER`]. Without
//! it, the file belongs to the user.

/// Prefix of the line that opens a managed section. Only the prefix is
/// matched, so the line can carry a short explanation for the reader.
pub const BEGIN_PREFIX: &str = "<!-- compagnon:begin";

/// The full opening line compagnon writes.
pub const BEGIN_LINE: &str = "<!-- compagnon:begin — managed by compagnon; `compagnon update` rewrites this block, edit outside it -->";

/// The line that closes a managed section.
pub const END_LINE: &str = "<!-- compagnon:end -->";

/// First-line marker of a file compagnon owns entirely.
pub const MANAGED_YAML_MARKER: &str = "# compagnon:managed";

/// Whether a missing section may be added.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpliceMode {
    /// `compagnon init`: create the file or append the section when absent.
    Init,
    /// `compagnon update`: only refresh a section that already exists. A
    /// project that opted out has no section, and must stay that way —
    /// compagnon keeps no other record of the opt-out.
    Refresh,
}

/// What writing the section does to the file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpliceAction {
    Created,
    Appended,
    Refreshed,
}

/// The outcome of [`splice`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Splice {
    /// Write `contents` to the file.
    Write {
        contents: String,
        action: SpliceAction,
    },
    /// The section is already current: nothing to write.
    Unchanged,
    /// No section, and the mode forbids adding one.
    Absent,
    /// Unbalanced or repeated markers: the file is left alone.
    Malformed(String),
}

/// Where the markers sit in a text.
enum Markers {
    None,
    /// Byte range from the start of the begin line to the end of the end
    /// line, its line break included when there is one.
    One(std::ops::Range<usize>),
    Malformed(String),
}

fn find_markers(text: &str) -> Markers {
    let mut begins = Vec::new();
    let mut ends = Vec::new();
    let mut offset = 0;
    for line in text.split_inclusive('\n') {
        let content = line.trim_end_matches(['\n', '\r']);
        if content.trim_start().starts_with(BEGIN_PREFIX) {
            begins.push(offset);
        } else if content.trim() == END_LINE {
            ends.push(offset + line.len());
        }
        offset += line.len();
    }
    match (begins.as_slice(), ends.as_slice()) {
        ([], []) => Markers::None,
        ([begin], [end]) if begin < end => Markers::One(*begin..*end),
        ([_], [_]) => Markers::Malformed("the end marker comes before the begin marker".into()),
        (b, e) => Markers::Malformed(format!(
            "{} begin marker(s) and {} end marker(s); exactly one of each is expected",
            b.len(),
            e.len()
        )),
    }
}

/// Renders a whole managed section around `body`, line break included.
pub fn render_section(body: &str) -> String {
    render_section_with(BEGIN_LINE, body)
}

/// Renders a managed section whose opening line is `begin_line` — a file
/// refreshed by another command than `compagnon update` says so on that line.
/// `begin_line` must start with [`BEGIN_PREFIX`].
pub fn render_section_with(begin_line: &str, body: &str) -> String {
    debug_assert!(begin_line.starts_with(BEGIN_PREFIX));
    format!("{begin_line}\n{}\n{END_LINE}\n", body.trim_matches('\n'))
}

/// Where the managed section sits: `Ok(None)` without markers, the byte
/// range from the begin line to the end line (its line break included)
/// otherwise, `Err` with the reason when the markers are malformed.
pub fn section_range(text: &str) -> Result<Option<std::ops::Range<usize>>, String> {
    match find_markers(text) {
        Markers::None => Ok(None),
        Markers::One(range) => Ok(Some(range)),
        Markers::Malformed(reason) => Err(reason),
    }
}

/// The text between the markers of the only well-formed section, without
/// the marker lines; `None` without exactly one section.
pub fn section_body(text: &str) -> Option<&str> {
    let Markers::One(range) = find_markers(text) else {
        return None;
    };
    let section = &text[range];
    let after_begin = section.find('\n').map_or(section.len(), |i| i + 1);
    let inner = &section[after_begin..];
    let before_end = inner.rfind(END_LINE).unwrap_or(inner.len());
    Some(&inner[..before_end])
}

/// Whether `text` holds exactly one well-formed managed section.
pub fn has_section(text: &str) -> bool {
    matches!(find_markers(text), Markers::One(_))
}

/// Whether a file compagnon owns entirely carries its marker on the first line.
pub fn is_managed_file(text: &str) -> bool {
    text.lines()
        .next()
        .is_some_and(|line| line.trim_start().starts_with(MANAGED_YAML_MARKER))
}

/// Computes the new content of a file whose managed section must hold
/// `body`.
///
/// - `existing` is `None` when the file does not exist.
/// - Everything outside the markers is kept byte for byte; an appended
///   section is separated from the user's content by a blank line.
pub fn splice(existing: Option<&str>, body: &str, mode: SpliceMode) -> Splice {
    let section = render_section(body);
    let Some(text) = existing else {
        return match mode {
            SpliceMode::Init => Splice::Write {
                contents: section,
                action: SpliceAction::Created,
            },
            SpliceMode::Refresh => Splice::Absent,
        };
    };
    match find_markers(text) {
        Markers::Malformed(reason) => Splice::Malformed(reason),
        Markers::None => match mode {
            SpliceMode::Refresh => Splice::Absent,
            SpliceMode::Init => {
                let mut contents = text.to_string();
                if !contents.is_empty() {
                    if !contents.ends_with('\n') {
                        contents.push('\n');
                    }
                    contents.push('\n');
                }
                contents.push_str(&section);
                Splice::Write {
                    contents,
                    action: SpliceAction::Appended,
                }
            }
        },
        Markers::One(range) => {
            // Keep the user's choice of a final line break after the end
            // marker, so a file that ended on it stays that way.
            let replacement = if text[..range.end].ends_with('\n') {
                section
            } else {
                section.trim_end_matches('\n').to_string()
            };
            let contents = format!(
                "{}{replacement}{}",
                &text[..range.start],
                &text[range.end..]
            );
            if contents == text {
                Splice::Unchanged
            } else {
                Splice::Write {
                    contents,
                    action: SpliceAction::Refreshed,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BODY: &str = "## compagnon\n\nUse /compagnon-propose.";

    fn written(splice: Splice) -> (String, SpliceAction) {
        match splice {
            Splice::Write { contents, action } => (contents, action),
            other => panic!("expected a write, got {other:?}"),
        }
    }

    #[test]
    fn splice_creates_a_missing_file() {
        let (contents, action) = written(splice(None, BODY, SpliceMode::Init));
        assert_eq!(action, SpliceAction::Created);
        assert_eq!(contents, format!("{BEGIN_LINE}\n{BODY}\n{END_LINE}\n"));
    }

    #[test]
    fn splice_appends_after_user_content_with_a_blank_line() {
        let user = "# House rules\n\nBe kind.";
        let (contents, action) = written(splice(Some(user), BODY, SpliceMode::Init));
        assert_eq!(action, SpliceAction::Appended);
        assert!(contents.starts_with("# House rules\n\nBe kind.\n\n<!-- compagnon:begin"));
        assert!(contents.ends_with(&format!("{END_LINE}\n")));
    }

    #[test]
    fn splice_refreshes_only_between_the_markers() {
        let before = "Intro by the team.\n\n";
        let after = "\nOutro by the team, no final newline";
        let stale = format!("{BEGIN_LINE}\nold text\n{END_LINE}\n");
        let file = format!("{before}{stale}{after}");
        for mode in [SpliceMode::Init, SpliceMode::Refresh] {
            let (contents, action) = written(splice(Some(&file), BODY, mode));
            assert_eq!(action, SpliceAction::Refreshed);
            assert_eq!(contents, format!("{before}{}{after}", render_section(BODY)));
        }
    }

    #[test]
    fn splice_keeps_a_custom_begin_line_explanation_out_of_the_match() {
        let file = "<!-- compagnon:begin hand-written note -->\nold\n<!-- compagnon:end -->";
        let (contents, _) = written(splice(Some(file), BODY, SpliceMode::Refresh));
        // No final newline in the original: none is added.
        assert_eq!(contents, render_section(BODY).trim_end_matches('\n'));
    }

    #[test]
    fn splice_is_unchanged_when_the_section_is_current() {
        let file = format!("User text\n\n{}", render_section(BODY));
        assert_eq!(
            splice(Some(&file), BODY, SpliceMode::Init),
            Splice::Unchanged
        );
        assert_eq!(
            splice(Some(&file), BODY, SpliceMode::Refresh),
            Splice::Unchanged
        );
    }

    #[test]
    fn splice_refuses_unbalanced_or_repeated_markers() {
        let only_begin = format!("{BEGIN_LINE}\ntext\n");
        let only_end = format!("text\n{END_LINE}\n");
        let reversed = format!("{END_LINE}\n{BEGIN_LINE}\n");
        let twice = format!("{}{}", render_section("a"), render_section("b"));
        for file in [only_begin, only_end, reversed, twice] {
            assert!(
                matches!(
                    splice(Some(&file), BODY, SpliceMode::Init),
                    Splice::Malformed(_)
                ),
                "{file}"
            );
            assert!(!has_section(&file));
        }
    }

    #[test]
    fn refresh_mode_never_creates_nor_appends() {
        assert_eq!(splice(None, BODY, SpliceMode::Refresh), Splice::Absent);
        assert_eq!(
            splice(Some("User text\n"), BODY, SpliceMode::Refresh),
            Splice::Absent
        );
    }

    #[test]
    fn an_empty_file_gets_the_section_alone() {
        let (contents, action) = written(splice(Some(""), BODY, SpliceMode::Init));
        assert_eq!(action, SpliceAction::Appended);
        assert_eq!(contents, render_section(BODY));
    }

    #[test]
    fn section_body_and_range_expose_the_managed_text() {
        let file = format!(
            "Intro\n{}Outro\n",
            render_section_with("<!-- compagnon:begin custom -->", "a\nb")
        );
        assert_eq!(section_body(&file), Some("a\nb\n"));
        let range = section_range(&file).unwrap().unwrap();
        assert_eq!(&file[..range.start], "Intro\n");
        assert_eq!(&file[range.end..], "Outro\n");
        assert_eq!(section_range("plain\n"), Ok(None));
        assert!(section_range(&format!("{BEGIN_LINE}\n")).is_err());
        assert_eq!(section_body("plain\n"), None);
    }

    #[test]
    fn managed_files_are_recognized_by_their_first_line() {
        assert!(is_managed_file(
            "# compagnon:managed — generated\nname: compagnon\n"
        ));
        assert!(!is_managed_file("name: ci\n# compagnon:managed\n"));
        assert!(!is_managed_file(""));
    }
}
