//! Level-2 sections of a markdown document, literal zones left out — the
//! proposal's `## Release note` and `## Domains` are read this way, in
//! every language of the keyword catalog.

use super::fence::build_fence_mask;
use crate::keywords::{self, Key};

/// 1-based line of the heading of `heading`, outside literal zones, in any
/// catalog language: the first one of the file.
pub fn heading_line(source: &str, heading: Key) -> Option<u32> {
    let mask = build_fence_mask(source);
    source
        .split('\n')
        .enumerate()
        .find(|(i, line)| !mask[*i] && is_heading(line, heading))
        .map(|(i, _)| i as u32 + 1)
}

fn is_heading(line: &str, heading: Key) -> bool {
    keywords::section_line(line, heading).is_some()
}

/// The lines of a level-2 section, literal zones left out, each with its
/// 1-based number and without a trailing `\r`. `None` when the heading is
/// absent. When the file holds the section in several languages, the
/// first one is read, as a second section of the same heading always was.
pub fn section(source: &str, heading: Key) -> Option<Vec<(u32, &str)>> {
    let mask = build_fence_mask(source);
    let mut lines = source.split('\n').enumerate();
    lines.find(|(i, line)| !mask[*i] && is_heading(line, heading))?;
    Some(
        lines
            .take_while(|(i, line)| {
                mask[*i] || !(line.starts_with("## ") || line.starts_with("# "))
            })
            .filter(|(i, _)| !mask[*i])
            .map(|(i, line)| (i as u32 + 1, line.trim_end_matches('\r')))
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn section_finds_a_heading_written_in_french() {
        let source = "# Proposition\n\n## Domaines\n\n- [x] migration\n\n## Impact\n";
        assert_eq!(heading_line(source, Key::Domains), Some(3));
        let lines = section(source, Key::Domains).unwrap();
        assert!(
            lines
                .iter()
                .any(|(n, l)| *n == 5 && *l == "- [x] migration")
        );
        assert!(!lines.iter().any(|(_, l)| l.contains("Impact")));
    }

    #[test]
    fn the_first_section_wins_when_both_languages_are_present() {
        let source = "## Domains\n\n- [x] migration\n\n## Domaines\n\n- [x] sécurité\n";
        let lines = section(source, Key::Domains).unwrap();
        assert!(lines.iter().any(|(_, l)| *l == "- [x] migration"));
        assert!(!lines.iter().any(|(_, l)| l.contains("sécurité")));
        assert_eq!(heading_line(source, Key::Domains), Some(1));
    }
}
