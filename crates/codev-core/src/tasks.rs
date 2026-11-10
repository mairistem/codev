//! Task progress: the checkboxes of the file a schema tracks
//! (`apply.tracks`), counted from the Markdown structure alone.
//!
//! A line scanner with two states — inside a fenced block, inside an HTML
//! comment — is all it takes; no Markdown crate. A Markdown parser would
//! also decide that a box inside an indented paragraph is not a list item,
//! which the person reading the file would not expect. No word of any
//! language is interpreted: a heading's text is only copied, so a tasks
//! file written in French or Portuguese is counted the same way.

/// The progress of a tracked file.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TaskProgress {
    /// The groups holding at least one box, in file order. Their order is
    /// their identity: two headings may share a number.
    pub groups: Vec<TaskGroup>,
    /// The open boxes, in file order.
    pub unchecked: Vec<UncheckedTask>,
}

/// The boxes under one level-2 heading, or before the first one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskGroup {
    /// The heading's leading number (`## 2. Engine` gives `2`), `None`
    /// without one. A string, so that a later format (`2a`) stays additive.
    pub id: Option<String>,
    /// The heading's text without its number, trimmed; `None` for the boxes
    /// before the first level-2 heading.
    pub title: Option<String>,
    pub checked: usize,
    pub total: usize,
}

/// An open box, as the person reads it in the file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UncheckedTask {
    /// 1-based line number.
    pub line: usize,
    /// The line, trimmed.
    pub text: String,
}

impl TaskGroup {
    fn empty(id: Option<String>, title: Option<String>) -> Self {
        Self {
            id,
            title,
            checked: 0,
            total: 0,
        }
    }

    /// The group of the boxes before the first level-2 heading.
    fn anonymous() -> Self {
        Self::empty(None, None)
    }
}

impl TaskProgress {
    pub fn checked(&self) -> usize {
        self.groups.iter().map(|g| g.checked).sum()
    }

    pub fn total(&self) -> usize {
        self.groups.iter().map(|g| g.total).sum()
    }
}

/// Counts the boxes of a tracked file's text.
pub fn parse(text: &str) -> TaskProgress {
    let mut progress = TaskProgress::default();
    // The heading the next box belongs to, and whether its group is already
    // listed: a heading without a box is never listed.
    let mut heading = TaskGroup::anonymous();
    let mut listed = false;
    let mut fence: Option<(char, usize)> = None;
    let mut in_comment = false;

    for (index, raw) in text.split('\n').enumerate() {
        let line = raw.strip_suffix('\r').unwrap_or(raw);
        let content = line.trim_start();

        if let Some((mark, length)) = fence {
            let closing = line.trim();
            if closing.chars().count() >= length && closing.chars().all(|c| c == mark) {
                fence = None;
            }
            continue;
        }
        if in_comment {
            in_comment = !line.contains("-->");
            continue;
        }
        if let Some(opened) = opening_fence(content) {
            fence = Some(opened);
            continue;
        }
        if let Some(rest) = content.strip_prefix("<!--") {
            in_comment = !rest.contains("-->");
            continue;
        }
        if let Some(text) = line.strip_prefix("## ") {
            heading = group_heading(text);
            listed = false;
            continue;
        }
        let Some(checked) = box_state(line) else {
            continue;
        };
        if !listed {
            progress.groups.push(heading.clone());
            listed = true;
        }
        let group = progress.groups.last_mut().expect("a group was just listed");
        group.total += 1;
        if checked {
            group.checked += 1;
        } else {
            progress.unchecked.push(UncheckedTask {
                line: index + 1,
                text: line.trim().to_string(),
            });
        }
    }
    progress
}

/// The fence a line opens, as its character and length: at least three
/// backticks or three tildes.
fn opening_fence(content: &str) -> Option<(char, usize)> {
    let mark = content.chars().next().filter(|c| *c == '`' || *c == '~')?;
    let length = content.chars().take_while(|c| *c == mark).count();
    (length >= 3).then_some((mark, length))
}

/// The empty group a level-2 heading's text opens: the leading digits
/// followed by a `.` and whitespace (or the end) are the id.
fn group_heading(text: &str) -> TaskGroup {
    let text = text.trim();
    let digits = text.chars().take_while(char::is_ascii_digit).count();
    if digits > 0
        && let Some(rest) = text[digits..].strip_prefix('.')
        && (rest.is_empty() || rest.starts_with(char::is_whitespace))
    {
        return TaskGroup::empty(
            Some(text[..digits].to_string()),
            Some(rest.trim().to_string()),
        );
    }
    TaskGroup::empty(None, Some(text.to_string()))
}

/// `Some(checked)` when the line is a box: `- [ ]`, `- [x]` or `- [X]` after
/// any spaces or tabs, followed by whitespace or the end of the line.
fn box_state(line: &str) -> Option<bool> {
    let rest = line.trim_start_matches([' ', '\t']).strip_prefix("- [")?;
    let mut chars = rest.chars();
    let checked = match chars.next()? {
        ' ' => false,
        'x' | 'X' => true,
        _ => return None,
    };
    let after = chars.as_str().strip_prefix(']')?;
    (after.is_empty() || after.starts_with(char::is_whitespace)).then_some(checked)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn group(id: Option<&str>, title: Option<&str>, checked: usize, total: usize) -> TaskGroup {
        TaskGroup {
            id: id.map(str::to_string),
            title: title.map(str::to_string),
            checked,
            total,
        }
    }

    #[test]
    fn boxes_are_counted_per_numbered_group() {
        let progress = parse(
            "# Tasks\n\n## 1. Core\n\n- [x] 1.1 Parser\n- [ ] 1.2 Engine\n\n## 2. Docs\n\n- [X] 2.1 Guide\n",
        );
        assert_eq!(
            progress.groups,
            [
                group(Some("1"), Some("Core"), 1, 2),
                group(Some("2"), Some("Docs"), 1, 1)
            ]
        );
        assert_eq!((progress.checked(), progress.total()), (2, 3));
        assert_eq!(
            progress.unchecked,
            [UncheckedTask {
                line: 6,
                text: "- [ ] 1.2 Engine".into()
            }]
        );
    }

    #[test]
    fn a_nested_box_counts() {
        let progress = parse("- [x] 1.1 Parser\n    - [ ] edge cases\n\t- [x] tab\n");
        assert_eq!((progress.checked(), progress.total()), (2, 3));
        assert_eq!(progress.unchecked[0].text, "- [ ] edge cases");
    }

    #[test]
    fn uppercase_x_is_checked() {
        let progress = parse("- [X] 1.1 Done\n");
        assert_eq!((progress.checked(), progress.total()), (1, 1));
    }

    #[test]
    fn other_markers_are_not_boxes() {
        let progress = parse(
            "- [-] 9.1 Bump\n* [ ] 9.2 Tag\n-[ ] 9.3 Push\n1. [ ] 9.4 List\n- [x]done\n- [ ] 9.5 Changelog\n- [ ]\n",
        );
        assert_eq!((progress.checked(), progress.total()), (0, 2));
    }

    #[test]
    fn crlf_gives_the_same_progress() {
        let unix = "## 1. Core\n- [x] 1.1 Parser\n- [ ] 1.2 Engine\n";
        let windows = unix.replace('\n', "\r\n");
        assert_eq!(parse(&windows), parse(unix));
        assert_eq!(parse(&windows).groups[0].title.as_deref(), Some("Core"));
    }

    #[test]
    fn boxes_in_an_indented_fence_are_not_counted() {
        // The shape of `2026-09-24-public-launch-polish/tasks.md`: a fence
        // indented inside a list item.
        let progress = parse(
            "- [x] 7.4 Create the PR template:\n\n      ```markdown\n      - [ ] cargo test green\n      - [ ] docs updated\n      ```\n- [x] 7.5 Next\n",
        );
        assert_eq!((progress.checked(), progress.total()), (2, 2));
    }

    #[test]
    fn a_tilde_fence_is_not_closed_by_backticks() {
        let progress = parse("~~~~\n- [ ] a\n```\n- [ ] b\n~~~~~\n- [ ] c\n");
        assert_eq!(progress.total(), 1);
        assert_eq!(progress.unchecked[0].text, "- [ ] c");
    }

    #[test]
    fn an_unclosed_fence_hides_the_rest() {
        let progress = parse("- [ ] a\n````\n- [ ] b\n```\n- [ ] c\n");
        assert_eq!(progress.total(), 1);
    }

    #[test]
    fn boxes_in_an_html_comment_are_not_counted() {
        let progress =
            parse("<!--\n- [ ] example task\n-->\n<!-- - [ ] inline -->\n- [ ] 1.1 Real task\n");
        assert_eq!((progress.checked(), progress.total()), (0, 1));
        assert_eq!(progress.unchecked[0].line, 5);
    }

    #[test]
    fn a_heading_inside_a_fence_opens_no_group() {
        let progress = parse("## 1. Core\n```\n## 2. Fake\n```\n- [ ] 1.1 Parser\n");
        assert_eq!(progress.groups, [group(Some("1"), Some("Core"), 0, 1)]);
    }

    #[test]
    fn boxes_before_the_first_heading_form_an_anonymous_group() {
        let progress = parse("# Tasks\n- [ ] Read the plan\n## Wrap-up\n- [x] Validate\n");
        assert_eq!(
            progress.groups,
            [group(None, None, 0, 1), group(None, Some("Wrap-up"), 1, 1)]
        );
    }

    #[test]
    fn a_heading_without_a_number_has_a_null_id() {
        let progress = parse(
            "## Wrap-up\n- [ ] a\n## 3.Glued\n- [ ] b\n## 2026 plan\n- [ ] c\n## 4.\n- [ ] d\n",
        );
        let ids: Vec<_> = progress
            .groups
            .iter()
            .map(|g| (g.id.as_deref(), g.title.as_deref()))
            .collect();
        assert_eq!(
            ids,
            [
                (None, Some("Wrap-up")),
                (None, Some("3.Glued")),
                (None, Some("2026 plan")),
                (Some("4"), Some(""))
            ]
        );
    }

    #[test]
    fn a_heading_without_a_box_is_not_listed() {
        let progress = parse("## 1. Core\n- [ ] 1.1 Parser\n## Notes\nA paragraph.\n");
        assert_eq!(progress.groups, [group(Some("1"), Some("Core"), 0, 1)]);
    }

    #[test]
    fn a_level_three_heading_stays_in_its_group() {
        let progress = parse("## 1. Core\n- [ ] a\n### Details\n- [x] b\n# Top\n- [ ] c\n");
        assert_eq!(progress.groups, [group(Some("1"), Some("Core"), 1, 3)]);
    }

    #[test]
    fn a_file_without_a_box_has_zero_total() {
        let progress = parse("# Tasks\n\nNothing yet.\n");
        assert_eq!(progress, TaskProgress::default());
        assert_eq!(progress.total(), 0);
    }
}
