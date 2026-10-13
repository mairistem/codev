//! Is the quoted evidence really where the finding says it is?
//!
//! Lines are compared normalized — trimmed, whitespace runs collapsed, `\r`
//! dropped — so that reformatting and CRLF do not break a quote, and each
//! quoted line need only be contained in its file line, so that a reviewer
//! can quote the fragment that matters. Blank lines are skipped on both
//! sides.

use super::TOLERANCE;

/// Where the evidence was found.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvidenceMatch {
    /// At the cited lines, within [`TOLERANCE`], or in the cited section.
    Found,
    /// Only elsewhere in the file, starting at this line.
    Moved {
        line: u32,
    },
    NotFound,
}

/// The number of lines of `content`, as an editor shows them.
pub fn line_count(content: &str) -> u32 {
    let lines = content.split('\n').count();
    // A final newline does not start a line of its own.
    let lines = if content.ends_with('\n') {
        lines - 1
    } else {
        lines
    };
    u32::try_from(lines).unwrap_or(u32::MAX)
}

/// The evidence of a code location, cited at `start..=end` of `content`.
pub fn match_code(content: &str, quote: &str, start: u32, end: u32) -> EvidenceMatch {
    let starts = match_starts(&numbered(content, 1), quote);
    let low = start.saturating_sub(TOLERANCE);
    let high = end.saturating_add(TOLERANCE);
    if starts.iter().any(|line| (low..=high).contains(line)) {
        EvidenceMatch::Found
    } else {
        first_elsewhere(&starts)
    }
}

/// The evidence of a plan location: in the section of `heading`, or
/// `None` when no heading has that text.
pub fn match_plan(content: &str, heading: &str, quote: &str) -> Option<EvidenceMatch> {
    let lines: Vec<&str> = content.split('\n').collect();
    let (index, level) = lines.iter().enumerate().find_map(|(i, line)| {
        heading_of(line)
            .filter(|(_, text)| text.eq_ignore_ascii_case(heading.trim()))
            .map(|(level, _)| (i, level))
    })?;
    let end = lines[index + 1..]
        .iter()
        .position(|line| heading_of(line).is_some_and(|(other, _)| other <= level))
        .map_or(lines.len(), |offset| index + 1 + offset);
    let section = lines[index..end].join("\n");
    let first_line = u32::try_from(index + 1).unwrap_or(u32::MAX);
    if !match_starts(&numbered(&section, first_line), quote).is_empty() {
        return Some(EvidenceMatch::Found);
    }
    Some(first_elsewhere(&match_starts(&numbered(content, 1), quote)))
}

fn first_elsewhere(starts: &[u32]) -> EvidenceMatch {
    starts
        .first()
        .map_or(EvidenceMatch::NotFound, |&line| EvidenceMatch::Moved {
            line,
        })
}

/// `#`-heading level and text; `None` for any other line.
fn heading_of(line: &str) -> Option<(usize, &str)> {
    let line = line.trim_end_matches('\r');
    let level = line.chars().take_while(|c| *c == '#').count();
    if level == 0 || level > 6 {
        return None;
    }
    let text = line[level..].strip_prefix(' ')?;
    Some((level, text.trim()))
}

fn normalize(line: &str) -> String {
    line.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// The non-blank lines of `text`, normalized, with their line number.
fn numbered(text: &str, first_line: u32) -> Vec<(u32, String)> {
    text.split('\n')
        .zip(first_line..)
        .map(|(line, number)| (number, normalize(line)))
        .filter(|(_, line)| !line.is_empty())
        .collect()
}

/// The lines where the quote starts: each of its non-blank lines contained
/// in consecutive non-blank lines of the file.
fn match_starts(file: &[(u32, String)], quote: &str) -> Vec<u32> {
    let quote: Vec<String> = numbered(quote, 1).into_iter().map(|(_, l)| l).collect();
    if quote.is_empty() || quote.len() > file.len() {
        return Vec::new();
    }
    file.windows(quote.len())
        .filter(|window| {
            window
                .iter()
                .zip(&quote)
                .all(|((_, line), wanted)| line.contains(wanted.as_str()))
        })
        .map(|window| window[0].0)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn file(lines: usize, at: usize, text: &str) -> String {
        (1..=lines)
            .map(|n| {
                if n == at {
                    text.to_string()
                } else {
                    format!("let line_{n} = {n};")
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[test]
    fn evidence_matches_with_whitespace_normalized() {
        let content =
            "fn a() {}\n    let   query = format!(\"SELECT {}\", name);\n    run(query);\n";
        let quote = "let query = format!(\"SELECT {}\", name);\n  run(";
        assert_eq!(match_code(content, quote, 2, 3), EvidenceMatch::Found);
    }

    #[test]
    fn evidence_within_five_lines_matches() {
        let content = file(30, 15, "danger()");
        assert_eq!(
            match_code(&content, "danger()", 12, 12),
            EvidenceMatch::Found
        );
        assert_eq!(
            match_code(&content, "danger()", 20, 20),
            EvidenceMatch::Found
        );
    }

    #[test]
    fn evidence_further_away_is_moved() {
        let content = file(50, 40, "danger()");
        assert_eq!(
            match_code(&content, "danger()", 12, 12),
            EvidenceMatch::Moved { line: 40 }
        );
    }

    #[test]
    fn evidence_found_nowhere_is_not_found() {
        let content = file(10, 3, "safe()");
        assert_eq!(
            match_code(&content, "danger()", 3, 3),
            EvidenceMatch::NotFound
        );
        assert_eq!(match_code(&content, "  \n", 3, 3), EvidenceMatch::NotFound);
    }

    #[test]
    fn evidence_ignores_crlf() {
        let content = "a\r\nlet x = 1;\r\nlet y = 2;\r\n";
        assert_eq!(
            match_code(content, "let x = 1;\r\nlet y = 2;", 2, 3),
            EvidenceMatch::Found
        );
        assert_eq!(line_count(content), 3);
    }

    #[test]
    fn a_quote_spanning_lines_must_be_consecutive() {
        let content = "one\ntwo\nthree\n";
        assert_eq!(
            match_code(content, "one\nthree", 1, 3),
            EvidenceMatch::NotFound
        );
        // Blank lines are skipped on both sides.
        assert_eq!(
            match_code("one\n\ntwo\n", "one\ntwo", 1, 3),
            EvidenceMatch::Found
        );
    }

    #[test]
    fn a_plan_section_ends_at_the_next_heading_of_its_level() {
        let design = "# Design\n\n## Decisions\n\n### Decision: Tokens\n\nTokens are stored in clear.\n\n#### Detail\n\nHashed later.\n\n### Decision: Other\n\nSomething else.\n";
        assert_eq!(
            match_plan(design, "decision: tokens", "stored in clear"),
            Some(EvidenceMatch::Found)
        );
        // A deeper heading stays inside the section.
        assert_eq!(
            match_plan(design, "Decision: Tokens", "Hashed later."),
            Some(EvidenceMatch::Found)
        );
        // The next heading of the same level closes it.
        assert_eq!(
            match_plan(design, "Decision: Tokens", "Something else."),
            Some(EvidenceMatch::Moved { line: 15 })
        );
        assert_eq!(match_plan(design, "Decision: Missing", "x"), None);
    }

    #[test]
    fn line_count_does_not_count_the_final_newline() {
        assert_eq!(line_count("a\nb\n"), 2);
        assert_eq!(line_count("a\nb"), 2);
        assert_eq!(line_count(""), 1);
    }
}
