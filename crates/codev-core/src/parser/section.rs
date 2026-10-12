//! Level-2 sections of a markdown document, literal zones left out — the
//! proposal's `## Release note` and `## Domains` are read this way.

use super::fence::build_fence_mask;

/// 1-based line of `heading`, outside literal zones, matched
/// case-insensitively.
pub fn heading_line(source: &str, heading: &str) -> Option<u32> {
    let mask = build_fence_mask(source);
    source
        .split('\n')
        .enumerate()
        .find(|(i, line)| !mask[*i] && is_heading(line, heading))
        .map(|(i, _)| i as u32 + 1)
}

fn is_heading(line: &str, heading: &str) -> bool {
    line.trim_end().eq_ignore_ascii_case(heading)
}

/// The lines of a level-2 section, literal zones left out, each with its
/// 1-based number and without a trailing `\r`. `None` when the heading is
/// absent.
pub fn section<'a>(source: &'a str, heading: &str) -> Option<Vec<(u32, &'a str)>> {
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
