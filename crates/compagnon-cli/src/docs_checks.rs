//! Checks on the documentation's shape — the Start here path, its word
//! caps, the glossary, the profiles, the folded blocks, the plain-words
//! requests. Pure functions returning what they find wrong, tested on
//! synthetic inputs, then applied to the embedded chapters of both
//! editions.

use crate::docs::Lang;

/// The fence languages that run commands in a terminal.
const SHELL_LANGS: &[&str] = &["bash", "sh", "shell", "console", "powershell", "pwsh"];

/// The skills, as their commands name them.
pub const SKILLS: [&str; 8] = [
    "compagnon-propose",
    "compagnon-explore",
    "compagnon-onboard",
    "compagnon-configure",
    "compagnon-apply",
    "compagnon-update",
    "compagnon-sync",
    "compagnon-archive",
];

/// One line of a chapter with what surrounds it: inside a fenced block
/// (and its language), inside a `<details>` block.
struct Line<'a> {
    text: &'a str,
    fence: Option<&'a str>,
    details: bool,
}

/// A fence line: its character (a backtick or `~`), how many of them, and
/// the language it names, empty when none.
#[derive(Clone, Copy)]
struct Fence<'a> {
    ch: char,
    len: usize,
    lang: &'a str,
}

/// The fence a line opens or closes, `None` for any other line.
fn fence_marker(trimmed: &str) -> Option<Fence<'_>> {
    let ch = trimmed.chars().next().filter(|c| *c == '`' || *c == '~')?;
    let len = trimmed.chars().take_while(|c| *c == ch).count();
    (len >= 3).then(|| Fence {
        ch,
        len,
        lang: trimmed[len..].trim(),
    })
}

/// Moves the fence state past one line: `open` is the fence the walk is in.
/// As in CommonMark, only a bare fence of the same character, at least as
/// long, closes it, so a four-backtick block may quote a three-backtick
/// one. Returns whether the line was a fence that opened or closed.
fn step_fence<'a>(open: &mut Option<Fence<'a>>, trimmed: &'a str) -> bool {
    let Some(fence) = fence_marker(trimmed) else {
        return false;
    };
    match *open {
        None => {
            *open = Some(fence);
            true
        }
        Some(o) if fence.ch == o.ch && fence.len >= o.len && fence.lang.is_empty() => {
            *open = None;
            true
        }
        Some(_) => false,
    }
}

/// Walks a chapter, line by line, keeping track of fences and `<details>`.
/// Fence lines and `<details>`/`</details>` lines themselves are skipped.
fn lines(md: &str) -> Vec<Line<'_>> {
    let mut out = Vec::new();
    let mut open: Option<Fence> = None;
    let mut details = 0usize;
    for text in md.lines() {
        let trimmed = text.trim_start();
        if step_fence(&mut open, trimmed) {
            continue;
        }
        let fence = open.map(|f| f.lang);
        if fence.is_none() {
            if trimmed.starts_with("<details") {
                details += 1;
                continue;
            }
            if trimmed.starts_with("</details>") {
                details = details.saturating_sub(1);
                continue;
            }
        }
        out.push(Line {
            text,
            fence,
            details: details > 0,
        });
    }
    out
}

/// The words of a chapter's prose: outside fenced code and `<details>`
/// blocks, a word being a run of non-space characters holding a letter or
/// a digit.
pub fn prose_words(md: &str) -> usize {
    lines(md)
        .iter()
        .filter(|l| l.fence.is_none() && !l.details)
        .flat_map(|l| l.text.split_whitespace())
        .filter(|w| w.chars().any(char::is_alphanumeric))
        .count()
}

/// The parts of a `SUMMARY.md`: each `# ` heading but the first (the
/// summary's own title), with the chapter paths listed under it.
pub fn parts(summary: &str) -> Vec<(String, Vec<String>)> {
    let mut parts: Vec<(String, Vec<String>)> = Vec::new();
    let mut seen_title = false;
    for line in summary.lines() {
        if let Some(title) = line.strip_prefix("# ") {
            if seen_title {
                parts.push((title.trim().to_string(), Vec::new()));
            }
            seen_title = true;
            continue;
        }
        if let (Some(start), Some(part)) = (line.find("]("), parts.last_mut()) {
            let path = &line[start + 2..];
            if let Some(end) = path.find(')') {
                part.1.push(path[..end].to_string());
            }
        }
    }
    parts
}

/// The path pages, one per profile, in the order of Choose your path.
pub const PATHS: [&str; 4] = [
    "paths/review.md",
    "paths/build.md",
    "paths/develop.md",
    "paths/team.md",
];

/// The chapters of the Start here path, in order: the introduction, Choose
/// your path and its four path pages, the glossary.
pub const START_HERE: [&str; 7] = [
    "introduction.md",
    "choose-your-path.md",
    PATHS[0],
    PATHS[1],
    PATHS[2],
    PATHS[3],
    "glossary.md",
];

/// The four profile labels, per edition, in the order of [`PATHS`]: the
/// answers to "What will you do with Compagnon?", and the titles of the
/// path pages, word for word.
pub fn profile_labels(lang: Lang) -> [&'static str; 4] {
    match lang {
        Lang::En => [
            "I review and approve plans — I do not code",
            "I do not code, but I want Claude to build something for me",
            "I am a developer and I have Claude write code",
            "I set Compagnon up for my team",
        ],
        Lang::Fr => [
            "Je relis et je valide des plans — je ne code pas",
            "Je ne code pas, mais je veux que Claude construise quelque chose pour moi",
            "Je suis développeur et je fais coder Claude",
            "J'installe Compagnon pour mon équipe",
        ],
    }
}

/// The heading of a path page's "Next" section, per edition.
pub fn next_heading(lang: Lang) -> &'static str {
    match lang {
        Lang::En => "Next, if you need it",
        Lang::Fr => "Ensuite, si besoin",
    }
}

/// The chapter a link opens, anchor and leading `../` removed: a path page
/// links `../glossary.md#delta`, which opens `glossary.md`.
fn chapter_of(target: &str) -> &str {
    target
        .split('#')
        .next()
        .unwrap_or_default()
        .trim_start_matches("../")
}

/// The links of a text as `(text, target)`, the text with its inner
/// whitespace collapsed (a link may wrap across lines).
fn link_pairs(text: &str) -> Vec<(String, String)> {
    let flat = text.split_whitespace().collect::<Vec<_>>().join(" ");
    links(&flat)
        .into_iter()
        .map(|(range, target)| (flat[range].to_string(), target))
        .collect()
}

/// What is wrong with the profile links of a text: for each label, in
/// order, a link whose text is the label and whose target ends with the
/// matching entry of `targets`. Used on Choose your path and the
/// introduction; the READMEs and the home pages are checked by its mirror
/// in `tests/repository.rs`, which also reads HTML links.
pub fn profile_link_problems(text: &str, labels: &[&str; 4], targets: &[String; 4]) -> Vec<String> {
    let pairs = link_pairs(text);
    let mut problems = Vec::new();
    let mut last = None;
    for (label, target) in labels.iter().zip(targets) {
        match pairs.iter().position(|(t, _)| t == label) {
            None => problems.push(format!("no link \"{label}\"")),
            Some(i) => {
                let found = pairs[i].1.split('#').next().unwrap_or_default();
                if !found.ends_with(target.as_str()) {
                    problems.push(format!("\"{label}\" links {found}, not {target}"));
                }
                if last.is_some_and(|l| l > i) {
                    problems.push(format!("\"{label}\" out of order"));
                }
                last = Some(i);
            }
        }
    }
    problems
}

/// What is wrong with Choose your path: the four profiles, each a list
/// item whose only link opens its path page, in order, then a link to
/// `origins.md` for contributors.
pub fn choose_path_problems(md: &str, labels: &[&str; 4]) -> Vec<String> {
    let targets = PATHS.map(String::from);
    let mut problems = profile_link_problems(md, labels, &targets);
    for label in labels {
        if let Some(item) = md
            .lines()
            .find(|l| l.trim_start().starts_with("- ") && l.contains(&format!("[{label}]")))
        {
            let count = links(item).len();
            if count != 1 {
                problems.push(format!("\"{label}\": {count} links in its item"));
            }
        }
    }
    let from_last_profile = labels
        .last()
        .and_then(|l| md.find(&format!("[{l}]")))
        .map_or("", |i| &md[i..]);
    // The first link from there is the last profile's own.
    if !link_pairs(from_last_profile)
        .iter()
        .skip(1)
        .any(|(_, t)| t.split('#').next() == Some("origins.md"))
    {
        problems.push("no link to origins.md after the profiles".into());
    }
    problems
}

/// What is wrong with a path page: its title not `title`, no sentence
/// before the first step, not one to three numbered steps of exactly one
/// link each whose text is the title of the chapter it opens (`titles`,
/// by chapter path), no `next` section, or no link back to Choose your
/// path.
pub fn path_page_problems(
    md: &str,
    title: &str,
    next: &str,
    titles: &[(String, String)],
) -> Vec<String> {
    let mut problems = Vec::new();
    let lines: Vec<&str> = md.lines().collect();
    if lines.first().map(|l| l.trim()) != Some(&format!("# {title}") as &str) {
        problems.push(format!("not titled \"{title}\""));
    }
    let steps: Vec<(usize, &str)> = lines
        .iter()
        .enumerate()
        .filter(|(_, l)| is_step_line(l))
        .map(|(i, l)| (i, *l))
        .collect();
    if steps.is_empty() || steps.len() > 3 {
        problems.push(format!("{} steps", steps.len()));
    }
    if let Some((first, _)) = steps.first()
        && !lines[1..*first].iter().any(|l| {
            let t = l.trim();
            !t.is_empty() && !t.starts_with('#')
        })
    {
        problems.push("no sentence before the first step".into());
    }
    for (_, step) in &steps {
        let pairs = link_pairs(step);
        if pairs.len() != 1 {
            problems.push(format!("step with {} links: {step}", pairs.len()));
            continue;
        }
        let (text, target) = &pairs[0];
        let path = chapter_of(target);
        match titles.iter().find(|(p, _)| p == path) {
            Some((_, t)) if t == text => {}
            Some((_, t)) => problems.push(format!("step \"{text}\" opens \"{t}\"")),
            None => problems.push(format!("step \"{text}\" opens no chapter: {path}")),
        }
    }
    if !lines.iter().any(|l| l.trim() == format!("## {next}")) {
        problems.push(format!("no \"{next}\" section"));
    }
    if !link_pairs(md)
        .iter()
        .any(|(_, t)| t.split('#').next() == Some("../choose-your-path.md"))
    {
        problems.push("no link back to Choose your path".into());
    }
    problems
}

/// Whether a line of a path page is one of its numbered steps: digits, then
/// `. `.
fn is_step_line(line: &str) -> bool {
    let digits = line.chars().take_while(char::is_ascii_digit).count();
    digits > 0 && line[digits..].starts_with(". ")
}

/// The links a path page lacks. `next_heading` is the title of its "Next"
/// section; `steps` are the chapters its numbered steps must open, and
/// `next_links` the chapters that section must link.
pub fn missing_path_links(
    md: &str,
    next_heading: &str,
    steps: &[&str],
    next_links: &[&str],
) -> Vec<String> {
    let (body, next_section) = md
        .split_once(&format!("## {next_heading}"))
        .unwrap_or((md, ""));
    let step_lines: String = body
        .lines()
        .filter(|l| is_step_line(l))
        .collect::<Vec<_>>()
        .join("\n");
    let opens = |text: &str, chapter: &str| {
        link_pairs(text)
            .iter()
            .any(|(_, t)| chapter_of(t) == chapter)
    };
    let mut missing: Vec<String> = steps
        .iter()
        .filter(|c| !opens(&step_lines, c))
        .map(|c| format!("no step {c}"))
        .collect();
    missing.extend(
        next_links
            .iter()
            .filter(|c| !opens(next_section, c))
            .map(|c| format!("no {c} under \"{next_heading}\"")),
    );
    missing
}

/// What is wrong with an annotated example: one of `parts` (`## ` headings)
/// missing or out of order, or a part without a quoted `markdown` block, or
/// without its `### note` heading after that block.
pub fn example_problems(md: &str, parts: &[&str], note: &str) -> Vec<String> {
    // The quoted plan holds `## ` headings of its own: a part runs from its
    // heading to the next part's, not to the next `## `.
    let starts: Vec<Option<usize>> = parts
        .iter()
        .map(|p| md.find(&format!("## {p}\n")))
        .collect();
    parts
        .iter()
        .enumerate()
        .filter_map(|(i, part)| {
            let Some(start) = starts[i] else {
                return Some(format!("no part \"{part}\""));
            };
            // The part ends where the next part present begins.
            let end = starts[i + 1..].iter().find_map(|s| *s).unwrap_or(md.len());
            if end < start {
                return Some(format!("\"{part}\" comes after the part that follows it"));
            }
            let body = &md[start..end];
            let Some(fence) = body.find("```markdown") else {
                return Some(format!("\"{part}\" quotes no plan"));
            };
            (!body[fence..].contains(&format!("### {note}")))
                .then(|| format!("\"{part}\" has no \"{note}\" after its quoted plan"))
        })
        .collect()
}

/// The text of the first fenced `markdown` block under the `## part`
/// heading of a chapter.
pub fn quoted_block<'a>(md: &'a str, part: &str) -> Option<&'a str> {
    let start = md.find(&format!("## {part}\n"))?;
    let rest = &md[start..];
    let open = rest.find("```markdown\n")? + "```markdown\n".len();
    let close = rest[open..].find("\n```")?;
    Some(&rest[open..open + close])
}

/// The explanations of everyday developer words that pages read by
/// developers leave out.
const GLOSSES: [&str; 4] = [
    "the window where you type commands",
    "a small program that checks",
    "la fenêtre où l'on tape des commandes",
    "un petit programme qui vérifie",
];

/// The glosses of [`GLOSSES`] a chapter holds.
pub fn developer_glosses(md: &str) -> Vec<&'static str> {
    let flat = md
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase();
    GLOSSES.into_iter().filter(|g| flat.contains(g)).collect()
}

/// What is wrong with a summary's first part: a chapter that does not
/// belong to the Start here path, or one of its chapters missing or out of
/// order.
pub fn start_here_problems(summary: &str) -> Vec<String> {
    let parts = parts(summary);
    let Some((_, first)) = parts.first() else {
        return vec!["no part".into()];
    };
    let mut problems: Vec<String> = first
        .iter()
        .filter(|c| !START_HERE.contains(&c.as_str()))
        .map(|c| format!("unexpected {c}"))
        .collect();
    problems.extend(
        START_HERE
            .iter()
            .filter(|c| !first.iter().any(|f| f == *c))
            .map(|c| format!("missing {c}")),
    );
    if problems.is_empty() && first.as_slice() != START_HERE {
        problems.push(format!("out of order: {first:?}"));
    }
    problems
}

/// The chapters of the parts after "Start here", in order: Everyday use,
/// Going further, Reference, Contributing.
pub const PARTS_AFTER_START: [&[&str]; 4] = [
    &[
        "quickstart.md",
        "guides/start-from-nothing.md",
        "installation.md",
        "workflow.md",
        "skills.md",
        "guides/mcp.md",
        "guides/review-a-plan.md",
        "guides/example-plan.md",
        "guides/approve-a-plan.md",
        "concepts.md",
        "faq.md",
    ],
    &[
        "guides/decisions.md",
        "guides/review-record.md",
        "roles.md",
        "guides/existing-project.md",
        "guides/team-rollout.md",
        "guides/repository-integration.md",
        "guides/release-notes.md",
        "guides/inherited-sources.md",
        "guides/artifact-language.md",
        "guides/custom-schemas.md",
        "guides/moving-from-codev.md",
    ],
    &[
        "reference/cli.md",
        "reference/configuration.md",
        "reference/file-formats.md",
        "reference/json-output.md",
        "reference/errors.md",
    ],
    &[
        "origins.md",
        "contributing.md",
        "architecture.md",
        "reliability.md",
    ],
];

/// How a `SUMMARY.md`'s parts after "Start here" depart from
/// [`PARTS_AFTER_START`]: a chapter in the wrong part, a missing one, or
/// a part out of order, each named.
pub fn toc_problems(summary: &str) -> Vec<String> {
    let parts = parts(summary);
    let mut problems = Vec::new();
    for (i, expected) in PARTS_AFTER_START.iter().enumerate() {
        let Some((title, listed)) = parts.get(i + 1) else {
            problems.push(format!("part {} is missing", i + 2));
            continue;
        };
        for c in listed {
            if !expected.contains(&c.as_str()) {
                problems.push(format!("{c} does not belong in \"{title}\""));
            }
        }
        for c in *expected {
            if !listed.iter().any(|l| l == c) {
                problems.push(format!("{c} is missing from \"{title}\""));
            }
        }
        if problems.is_empty()
            && listed
                .iter()
                .map(String::as_str)
                .ne(expected.iter().copied())
        {
            problems.push(format!("\"{title}\" is out of order: {listed:?}"));
        }
    }
    problems
}

/// The entries of a `SUMMARY.md` whose title is not the `# ` heading of
/// their chapter, as `path: entry ≠ heading`.
pub fn title_problems(summary: &str, heading_of: impl Fn(&str) -> Option<String>) -> Vec<String> {
    let mut problems = Vec::new();
    for line in summary.lines() {
        let line = line.trim_start();
        let Some(rest) = line.strip_prefix("- [") else {
            continue;
        };
        let Some((title, target)) = rest.split_once("](") else {
            continue;
        };
        let path = target.trim_end_matches(')');
        match heading_of(path) {
            Some(h) if h == title => {}
            Some(h) => problems.push(format!("{path}: entry \"{title}\" ≠ heading \"{h}\"")),
            None => problems.push(format!("{path}: no `# ` heading")),
        }
    }
    problems
}

/// `md` without its `<figure>…</figure>` blocks: a diagram's words are
/// not the chapter's prose.
pub fn without_figures(md: &str) -> String {
    let mut out = String::with_capacity(md.len());
    let mut rest = md;
    while let Some(start) = rest.find("<figure") {
        out.push_str(&rest[..start]);
        match rest[start..].find("</figure>") {
            Some(end) => rest = &rest[start + end + "</figure>".len()..],
            None => return out,
        }
    }
    out.push_str(rest);
    out
}

/// What keeps an entry point from putting Jira forward: it must name Jira
/// and link the Jira guide (`guides/mcp.md`, or `guides/mcp.html` once
/// published).
pub fn jira_problems(text: &str) -> Vec<String> {
    let mut problems = Vec::new();
    if !text.contains("Jira") {
        problems.push("never names Jira".to_string());
    }
    if !text.contains("guides/mcp.md") && !text.contains("guides/mcp.html") {
        problems.push("does not link the Jira guide".to_string());
    }
    problems
}

/// Whether a home page names Jira only after the last of its four profile
/// labels, so that the question still comes first.
pub fn jira_after_profiles(page: &str, labels: &[&str; 4]) -> bool {
    let last = labels.iter().filter_map(|l| page.find(l)).max();
    match (last, page.find("Jira")) {
        (Some(last), Some(jira)) => jira > last,
        _ => false,
    }
}

/// What is wrong with the Jira guide's shape: its diagram must come before
/// its first `## ` heading — what the reader gets before how to set it up.
pub fn jira_guide_problems(md: &str) -> Vec<String> {
    let figure = md.find("<figure class=\"diagram\"");
    let heading = md.find("\n## ");
    match (figure, heading) {
        (None, _) => vec!["has no diagram".to_string()],
        (Some(f), Some(h)) if h < f => {
            vec!["shows its first `## ` section before its diagram".to_string()]
        }
        _ => Vec::new(),
    }
}

/// The error codes still headed in the FAQ, and those `codes` the errors
/// chapter does not head with `### \`code\``.
pub fn faq_code_problems(faq: &str, errors: &str, codes: &[&str]) -> Vec<String> {
    let mut problems = Vec::new();
    for line in faq.lines() {
        let Some(title) = line.strip_prefix("### ") else {
            continue;
        };
        let title = title.trim();
        if title.starts_with('`') && title.ends_with('`') && !title.contains(' ') {
            problems.push(format!("the FAQ still explains {title}"));
        }
    }
    for code in codes {
        let heading = format!("### `{code}`");
        if !errors.lines().any(|l| l.trim() == heading) {
            problems.push(format!("the errors chapter has no {heading}"));
        }
    }
    problems
}

/// The title of an everyday chapter's last section.
pub fn go_further_heading(lang: Lang) -> &'static str {
    match lang {
        Lang::En => "Go further",
        Lang::Fr => "Pour aller plus loin",
    }
}

/// What keeps an everyday chapter short and open: its prose (code,
/// `<details>` and `<figure>` blocks not counted) over `cap` words, or a
/// last `## ` section other than "Go further", or one that links no
/// chapter by its title. `title_of` gives a chapter's title from a link
/// target relative to the chapter (`../roles.md#x`).
pub fn everyday_problems(
    md: &str,
    cap: usize,
    lang: Lang,
    title_of: impl Fn(&str) -> Option<String>,
) -> Vec<String> {
    let mut problems = Vec::new();
    let words = prose_words(&without_figures(md));
    if words > cap {
        problems.push(format!("{words} words, over {cap}"));
    }
    let heading = go_further_heading(lang);
    let last = sections(md, "## ").into_iter().last();
    match last {
        Some((title, body)) if title == heading => {
            let mut chapters = 0;
            for (text, target) in link_pairs(&body) {
                if target.starts_with("http") || !chapter_of(&target).ends_with(".md") {
                    continue;
                }
                chapters += 1;
                match title_of(&target) {
                    Some(t) if t == text => {}
                    Some(t) => problems.push(format!("\"{text}\" links {target}, titled \"{t}\"")),
                    None => {
                        problems.push(format!("\"{text}\" links {target}, which is no chapter"))
                    }
                }
            }
            if chapters == 0 {
                problems.push(format!("\"{heading}\" links no chapter"));
            }
        }
        Some((title, _)) => problems.push(format!("ends with \"{title}\", not \"{heading}\"")),
        None => problems.push(format!("has no \"{heading}\" section")),
    }
    problems
}

/// The commands of shell blocks outside `<details>` that neither install
/// Compagnon nor run `compagnon init`.
pub fn stray_commands(md: &str) -> Vec<String> {
    lines(md)
        .iter()
        .filter(|l| !l.details && l.fence.is_some_and(|f| SHELL_LANGS.contains(&f)))
        .map(|l| l.text.trim())
        .filter(|t| !t.is_empty() && !t.starts_with('#'))
        .filter(|t| {
            !(t.contains("install.sh")
                || t.contains("install.ps1")
                || *t == "compagnon init"
                || t.starts_with("compagnon init "))
        })
        .map(str::to_string)
        .collect()
}

/// The sections of a chapter opened by headings starting with `level`
/// (`"## "`, `"### "`): heading and body. A deeper heading stays in the
/// body; text before the first heading is dropped.
fn sections(md: &str, level: &str) -> Vec<(String, String)> {
    let mut out: Vec<(String, String)> = Vec::new();
    for line in md.lines() {
        if let Some(h) = line.strip_prefix(level) {
            out.push((h.trim().to_string(), String::new()));
        } else if let Some(last) = out.last_mut() {
            last.1.push_str(line);
            last.1.push('\n');
        }
    }
    out
}

/// What is wrong with a glossary: a required term without its `## `
/// section, or a definition over `cap` words.
pub fn glossary_problems(md: &str, terms: &[&str], cap: usize) -> Vec<String> {
    let sections = sections(md, "## ");
    let mut problems = Vec::new();
    for term in terms {
        match sections.iter().find(|(h, _)| h.eq_ignore_ascii_case(term)) {
            None => problems.push(format!("missing {term}")),
            Some((_, body)) => {
                let words = prose_words(body);
                if words > cap {
                    problems.push(format!("{term}: {words} words"));
                }
            }
        }
    }
    problems
}

/// The shell languages of a chapter's fenced blocks, one entry per block,
/// fences recognised as in [`lines`].
pub fn shell_blocks(md: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut open: Option<Fence> = None;
    for line in md.lines() {
        let was_open = open.is_some();
        if step_fence(&mut open, line.trim_start())
            && !was_open
            && let Some(f) = open
            && SHELL_LANGS.contains(&f.lang)
        {
            out.push(f.lang.to_string());
        }
    }
    out
}

/// What is wrong with a chapter's `<details>` blocks: open by default, a
/// summary not starting with `prefix`, or a summary used twice.
pub fn details_problems(md: &str, prefix: &str) -> Vec<String> {
    let mut problems = Vec::new();
    let mut summaries: Vec<String> = Vec::new();
    for line in md.lines() {
        let t = line.trim();
        if t.starts_with("<details") && t.contains("open") {
            problems.push(format!("open by default: {t}"));
        }
        if let Some(rest) = t.strip_prefix("<summary>") {
            let summary = rest.trim_end_matches("</summary>").trim().to_string();
            if !summary.starts_with(prefix) {
                problems.push(format!(
                    "summary \"{summary}\" does not start with \"{prefix}\""
                ));
            }
            if summaries.contains(&summary) {
                problems.push(format!("summary \"{summary}\" used twice"));
            }
            summaries.push(summary);
        }
    }
    problems
}

/// What is wrong with a "How to ask" section: a skill whose `### `
/// subsection holds fewer than two quoted example requests (list items
/// starting with `"` or `«`).
pub fn how_to_ask_problems(section: &str) -> Vec<String> {
    let subsections = sections(section, "### ");
    SKILLS
        .iter()
        .filter_map(|skill| {
            let examples = subsections
                .iter()
                .find(|(h, _)| h.contains(&format!("/{skill}`")))
                .map(|(_, body)| {
                    body.lines()
                        .filter(|l| {
                            let l = l.trim_start();
                            l.starts_with("- \"") || l.starts_with("- «")
                        })
                        .count()
                })
                .unwrap_or(0);
            (examples < 2).then(|| format!("{skill}: {examples} example(s)"))
        })
        .collect()
}

/// The sections of a quickstart that name a skill in their heading but do
/// not quote a plain request (a `> ` line) before naming the command in
/// their body.
pub fn requests_after_commands(md: &str) -> Vec<String> {
    sections(md, "## ")
        .into_iter()
        .filter(|(h, _)| h.contains("/compagnon-"))
        .filter(|(_, body)| {
            let request = body.lines().position(|l| l.trim_start().starts_with("> "));
            let command = body.lines().position(|l| l.contains("/compagnon-"));
            match (request, command) {
                (Some(r), Some(c)) => r > c,
                (None, _) => true,
                (Some(_), None) => false,
            }
        })
        .map(|(h, _)| h)
        .collect()
}

/// The glossary's required terms, per edition.
pub fn glossary_terms(lang: Lang) -> &'static [&'static str] {
    match lang {
        Lang::En => &[
            "Agent",
            "Archive",
            "Capability",
            "Change",
            "Claude Code",
            "Decision",
            "Delta",
            "Design",
            "MCP",
            "Plan",
            "Proposal",
            "Requirement",
            "Review",
            "Role",
            "Scenario",
            "Skill",
            "Spec",
            "Tasks",
            "Artifact",
            "Schema",
            "Workflow",
            "Frontmatter",
            "Kebab-case",
        ],
        Lang::Fr => &[
            "Agent",
            "Archive",
            "Capacité",
            "Modification",
            "Claude Code",
            "Compétence",
            "Conception",
            "Décision",
            "Delta",
            "Exigence",
            "MCP",
            "Plan",
            "Proposition",
            "Relecture",
            "Rôle",
            "Scénario",
            "Spécification",
            "Tâches",
            "Artefact",
            "Schéma",
            "Déroulé",
            "Frontmatter",
            "Kebab-case",
        ],
    }
}

/// The technical terms an everyday chapter links to the glossary on first
/// use, each with the forms it takes, per edition.
pub fn explained_terms(lang: Lang) -> &'static [(&'static str, &'static [&'static str])] {
    match lang {
        Lang::En => &[
            ("delta", &["delta", "deltas"]),
            ("artifact", &["artifact", "artifacts"]),
            ("capability", &["capability", "capabilities"]),
            ("schema", &["schema", "schemas"]),
            ("workflow", &["workflow", "workflows"]),
            ("frontmatter", &["frontmatter"]),
            ("kebab-case", &["kebab-case"]),
        ],
        Lang::Fr => &[
            ("delta", &["delta", "deltas"]),
            ("artefact", &["artefact", "artefacts"]),
            ("capacité", &["capacité", "capacités"]),
            ("schéma", &["schéma", "schémas"]),
            ("déroulé", &["déroulé", "déroulés"]),
            ("frontmatter", &["frontmatter"]),
            ("kebab-case", &["kebab-case"]),
        ],
    }
}

/// The phrase an everyday chapter never uses: the project's specs say it
/// in everyday words.
fn banned_phrase(lang: Lang) -> &'static str {
    match lang {
        Lang::En => "main spec",
        Lang::Fr => "specs principales",
    }
}

/// Whether [`prose_paragraphs`] keeps the heading lines.
#[derive(Clone, Copy, PartialEq)]
enum Headings {
    Keep,
    Skip,
}

/// The paragraphs of a chapter's prose, outside fenced code and `<details>`
/// blocks, with inline code blanked out; with [`Headings::Keep`], each
/// heading line is a paragraph of its own.
fn prose_paragraphs(md: &str, headings: Headings) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut current = String::new();
    let flush = |current: &mut String, out: &mut Vec<String>| {
        if !current.trim().is_empty() {
            out.push(std::mem::take(current));
        }
        current.clear();
    };
    for line in lines(md) {
        if line.fence.is_some() || line.details {
            flush(&mut current, &mut out);
            continue;
        }
        let text = without_inline_code(line.text);
        if text.trim().is_empty() {
            flush(&mut current, &mut out);
        } else if text.trim_start().starts_with('#') {
            flush(&mut current, &mut out);
            if headings == Headings::Keep {
                out.push(text);
            }
        } else {
            current.push_str(&text);
            current.push(' ');
        }
    }
    flush(&mut current, &mut out);
    out
}

/// `text` with every inline code span replaced by spaces of the same
/// length, so that positions are kept.
fn without_inline_code(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut in_code = false;
    for c in text.chars() {
        if c == '`' {
            in_code = !in_code;
            out.push(' ');
        } else if in_code {
            out.push(' ');
        } else {
            out.push(c);
        }
    }
    out
}

/// The words of `text`, lowercased, with their byte offsets; a word keeps
/// its letters, digits and inner hyphens.
fn words(text: &str) -> Vec<(usize, String)> {
    let mut out = Vec::new();
    let mut start: Option<usize> = None;
    // A trailing space closes the last word; a hyphen is kept inside a word
    // ("kebab-case") and dropped at its end.
    for (i, c) in text
        .char_indices()
        .chain(std::iter::once((text.len(), ' ')))
    {
        let part = c.is_alphanumeric() || (c == '-' && start.is_some());
        match (part, start) {
            (true, None) => start = Some(i),
            (false, Some(s)) => {
                let word = text[s..i].trim_end_matches('-').to_lowercase();
                out.push((s, word));
                start = None;
            }
            _ => {}
        }
    }
    out
}

/// The links of a paragraph: the byte range of their text and their
/// target. A simple scanner for single-level `[text](target)` links: no
/// `[` inside the text, no `)` inside the target.
fn links(text: &str) -> Vec<(std::ops::Range<usize>, String)> {
    let mut out = Vec::new();
    let mut from = 0;
    while let Some(open) = text[from..].find('[').map(|i| i + from) {
        let Some(close) = text[open..].find("](").map(|i| i + open) else {
            break;
        };
        let Some(end) = text[close..].find(')').map(|i| i + close) else {
            break;
        };
        out.push((open + 1..close, text[close + 2..end].to_string()));
        from = end;
    }
    out
}

/// The anchors the `## ` headings of a glossary produce, as mdBook writes
/// them.
pub fn glossary_anchors(glossary: &str) -> Vec<String> {
    let mut slugger = crate::docs::Slugger::default();
    glossary
        .lines()
        .filter_map(|l| l.strip_prefix("## "))
        .map(|h| slugger.slug(h))
        .collect()
}

/// Whether the word at byte `at` of a paragraph sits in the text of a link
/// to one of the glossary's entries (`anchors`).
fn links_to_glossary_entry(
    links: &[(std::ops::Range<usize>, String)],
    at: usize,
    anchors: &[String],
) -> bool {
    links.iter().any(|(range, target)| {
        range.contains(&at)
            && target.split_once('#').is_some_and(|(path, anchor)| {
                path.ends_with("glossary.md") && anchors.iter().any(|a| a == anchor)
            })
    })
}

/// What is wrong with the vocabulary of an everyday chapter: a technical
/// term whose first use in prose is not a link to its own glossary entry
/// (one of `anchors`), and the banned phrase ("main specs") wherever it
/// appears. Headings are skipped: a heading cannot hold a link.
pub fn vocabulary_problems(md: &str, lang: Lang, anchors: &[String]) -> Vec<String> {
    let mut problems = Vec::new();
    let mut seen: Vec<&str> = Vec::new();
    let banned = banned_phrase(lang);
    for paragraph in prose_paragraphs(md, Headings::Skip) {
        let lower = paragraph.to_lowercase();
        if lower.contains(banned) && !problems.iter().any(|p: &String| p == banned) {
            problems.push(banned.to_string());
        }
        let links = links(&paragraph);
        // The offsets match `links`: the targets are blanked to the same
        // length, only so that a path such as `workflow.md` is not prose.
        for (at, word) in words(&prose_without_links(&paragraph)) {
            for (term, forms) in explained_terms(lang) {
                if seen.contains(term) || !forms.contains(&word.as_str()) {
                    continue;
                }
                seen.push(term);
                if !links_to_glossary_entry(&links, at, anchors) {
                    problems.push((*term).to_string());
                }
            }
        }
    }
    problems
}

/// `text` with every link blanked to spaces of the same length, so that
/// offsets are kept: the target always (a path such as `workflow.md` is
/// not prose), and the visible text of every link except one to the
/// glossary — a link elsewhere names a page as its title does ("The
/// workflow"), while a link to the glossary holds the term it explains.
fn prose_without_links(text: &str) -> String {
    let mut out = text.to_string();
    for (range, target) in links(text) {
        let start = range.end + 2;
        let end = start + target.len();
        out.replace_range(start..end, &" ".repeat(target.len()));
        if !target.contains("glossary.md") {
            out.replace_range(range.clone(), &" ".repeat(range.len()));
        }
    }
    out
}

/// The words after which "change" is the English noun, not the verb
/// *changer*: determiners, adjectives that come before a noun (premier,
/// nouveau…), and the elided articles d' and l', which [`words`] leaves as
/// "d" and "l".
const DETERMINERS: &[&str] = &[
    "le", "la", "les", "un", "une", "des", "du", "au", "aux", "ce", "cet", "cette", "ces",
    "chaque", "votre", "vos", "son", "sa", "ses", "leur", "leurs", "quel", "quelle", "d", "l",
    "mon", "ma", "mes", "notre", "nos", "premier", "dernier", "nouveau", "autre",
];

/// The anglicisms of a French chapter: the English words of the lexicon in
/// prose and headings, outside code, `<details>` blocks, `<svg>` blocks,
/// identifier headings and parentheses. "change" counts only as a noun:
/// after a determiner or a word of [`NOUN_MARKERS`], or as a heading's
/// first word.
pub fn anglicisms(md: &str) -> Vec<String> {
    const ENGLISH: &[&str] = &[
        "skill",
        "skills",
        "spec",
        "specs",
        "workflow",
        "workflows",
        "review",
        "reviews",
        "commit",
        "commits",
        "design",
        "template",
        "templates",
    ];
    let mut found: Vec<String> = Vec::new();
    let add = |w: &str, found: &mut Vec<String>| {
        if !found.iter().any(|f| f == w) {
            found.push(w.to_string());
        }
    };
    // Headings count: a French heading must be French too.
    for paragraph in prose_paragraphs(&without_svg(md), Headings::Keep) {
        let heading = paragraph.trim_start().starts_with('#');
        if heading && is_identifier_heading(&paragraph) {
            continue;
        }
        let text = without_parentheses(&paragraph);
        let words: Vec<String> = words(&text).into_iter().map(|(_, w)| w).collect();
        for (i, word) in words.iter().enumerate() {
            let parts: Vec<&str> = word.split('-').collect();
            // A product's name is not an anglicism: "Spec Kit".
            let product = word == "spec" && words.get(i + 1).is_some_and(|n| n == "kit");
            for part in &parts {
                if ENGLISH.contains(part) && !product {
                    add(part, &mut found);
                }
            }
            if is_noun_change(&words, i, heading) {
                add(word, &mut found);
            }
            for (first, name) in [("pull", "pull request"), ("merge", "merge request")] {
                if word == first && words.get(i + 1).is_some_and(|n| n.starts_with("request")) {
                    add(name, &mut found);
                }
            }
        }
    }
    found
}

/// Words after which "change" is a noun, beyond [`DETERMINERS`]: numerals,
/// quantifiers and the plural adjectives that come before a noun.
const NOUN_MARKERS: &[&str] = &[
    "aucun",
    "seul",
    "seuls",
    "deux",
    "trois",
    "plusieurs",
    "nombre",
    "de",
    "autres",
    "tous",
    "certains",
    "quelques",
    "nouveaux",
    "derniers",
    "premiers",
    "différents",
];

/// Whether `words[i]` is "change(s)" used as a noun: a heading's first word,
/// or after a determiner or a word of [`NOUN_MARKERS`]. « ce qui change »
/// stays a verb.
fn is_noun_change(words: &[String], i: usize, heading: bool) -> bool {
    let word = words[i].as_str();
    if word != "change" && word != "changes" {
        return false;
    }
    if heading && i == 0 {
        return true;
    }
    i > 0 && {
        let before = words[i - 1].as_str();
        DETERMINERS.contains(&before) || NOUN_MARKERS.contains(&before)
    }
}

/// Whether a heading names an identifier rather than saying something: a
/// file name (`## design.md`), a configuration key alone in lower case
/// (`## review`), or a command (`### compagnon review`).
fn is_identifier_heading(heading: &str) -> bool {
    let text = heading.trim().trim_start_matches('#').trim();
    let file = !text.contains(' ') && text.contains('.');
    let key = !text.is_empty()
        && text
            .chars()
            .all(|c| c.is_ascii_lowercase() || c == '_' || c == '-');
    file || key || text.starts_with("compagnon ")
}

/// `md` with each `<svg>` block replaced by what it says in words (its
/// `aria-label` and its `<text>`), each as its own paragraph: a French
/// diagram speaks French too.
fn without_svg(md: &str) -> String {
    let mut out = String::with_capacity(md.len());
    let mut from = 0;
    for (start, svg) in crate::docs::svg_elements(md) {
        out.push_str(&md[from..start]);
        out.push_str("\n\n");
        out.push_str(&svg_words(svg));
        out.push_str("\n\n");
        from = start + svg.len();
    }
    out.push_str(&md[from..]);
    out
}

/// The value of attribute `name` in the opening tag `tag`.
fn attribute<'a>(tag: &'a str, name: &str) -> Option<&'a str> {
    let key = format!(" {name}=\"");
    let start = tag.find(&key)? + key.len();
    let len = tag[start..].find('"')?;
    Some(&tag[start..start + len])
}

/// What a diagram says in words: its `aria-label` and the contents of its
/// `<text>` elements, one per line.
fn svg_words(svg: &str) -> String {
    let open = &svg[..svg.find('>').unwrap_or(svg.len())];
    let mut out: Vec<String> = attribute(open, "aria-label")
        .map(|l| l.to_string())
        .into_iter()
        .collect();
    let mut rest = svg;
    while let Some(i) = rest.find("<text") {
        let after = &rest[i..];
        let Some(gt) = after.find('>') else { break };
        let body = &after[gt + 1..];
        let end = body.find("</text>").unwrap_or(body.len());
        let inner = &body[..end];
        // Strip nested tags such as <tspan>.
        let mut plain = String::new();
        let mut in_tag = false;
        for c in inner.chars() {
            match c {
                '<' => in_tag = true,
                '>' => in_tag = false,
                _ if !in_tag => plain.push(c),
                _ => {}
            }
        }
        // A label that is an identifier — a token source, a command — is
        // not prose, as an identifier heading is not.
        if !is_identifier_heading(&plain) {
            out.push(plain);
        }
        rest = &body[end..];
    }
    out.join("\n\n")
}

/// The problems of the diagrams of a page — a chapter's markdown or a
/// home page's HTML. A diagram is any `<svg>` that is neither hidden from
/// assistive technology (`aria-hidden="true"`, the sprites and the marks
/// beside a visible name) nor a logo (`class="logo"`). Each must sit in a
/// `<figure>`, carry `role="img"` and a name (`aria-label`, or
/// `aria-labelledby` pointing at its `<title>`), be followed by a
/// `<figcaption>` before its figure closes, point at no other file, and
/// hold no blank line — a blank line ends a markdown HTML block, and the
/// rest of the drawing would be printed as text.
pub fn diagram_problems(text: &str) -> Vec<String> {
    let mut problems = Vec::new();
    for (n, (start, svg)) in crate::docs::svg_elements(text).into_iter().enumerate() {
        let open = &svg[..svg.find('>').map_or(svg.len(), |i| i + 1)];
        if attribute(open, "aria-hidden") == Some("true")
            || attribute(open, "class").is_some_and(|c| c.split(' ').any(|c| c == "logo"))
        {
            continue;
        }
        let name = attribute(open, "aria-label")
            .filter(|l| !l.trim().is_empty())
            .or_else(|| attribute(open, "aria-labelledby"));
        let label = name.map_or_else(
            || format!("diagram {}", n + 1),
            |l| {
                let short: String = l.chars().take(40).collect();
                format!("diagram \"{short}\"")
            },
        );
        let before = &text[..start];
        let in_figure = before
            .rfind("<figure")
            .is_some_and(|f| before.rfind("</figure>").is_none_or(|c| c < f));
        if !in_figure {
            problems.push(format!("{label} is not inside a <figure>"));
        }
        if attribute(open, "role") != Some("img") {
            problems.push(format!("{label} lacks role=\"img\""));
        }
        if name.is_none() {
            problems.push(format!("{label} has no aria-label"));
        }
        let after = &text[start + svg.len()..];
        let closes = after.find("</figure>").unwrap_or(after.len());
        if in_figure && !after[..closes].contains("<figcaption") {
            problems.push(format!("{label} is not followed by a <figcaption>"));
        }
        for attr in ["href=\"", "src=\""] {
            let mut rest = svg;
            while let Some(i) = rest.find(attr) {
                let value = &rest[i + attr.len()..];
                if !value.starts_with('#') {
                    problems.push(format!("{label} points at another file"));
                    break;
                }
                rest = value;
            }
        }
        if svg.contains("\n\n")
            || svg.contains("\n\r\n")
            || svg.lines().any(|l| l.trim().is_empty())
        {
            problems.push(format!("{label} holds a blank line"));
        }
    }
    problems
}

/// The relative luminance of a `#rrggbb` color (WCAG 2.2).
fn luminance(hex: &str) -> Option<f64> {
    let hex = hex.strip_prefix('#')?;
    if hex.len() != 6 {
        return None;
    }
    let channel = |i: usize| -> Option<f64> {
        let v = f64::from(u8::from_str_radix(&hex[i..i + 2], 16).ok()?) / 255.0;
        Some(if v <= 0.040_45 {
            v / 12.92
        } else {
            ((v + 0.055) / 1.055).powf(2.4)
        })
    };
    Some(0.2126 * channel(0)? + 0.7152 * channel(2)? + 0.0722 * channel(4)?)
}

/// The contrast ratio of two `#rrggbb` colors, from 1 to 21.
pub fn contrast(a: &str, b: &str) -> Option<f64> {
    let (la, lb) = (luminance(a)?, luminance(b)?);
    let (hi, lo) = if la > lb { (la, lb) } else { (lb, la) };
    Some((hi + 0.05) / (lo + 0.05))
}

/// The diagram themes of a style sheet: each rule block that declares
/// `--dg-ink`, as its selector and its `--dg-*` declarations.
pub fn diagram_themes(css: &str) -> Vec<(String, Vec<(String, String)>)> {
    let mut out = Vec::new();
    let mut from = 0;
    while let Some(open) = css[from..].find('{').map(|i| from + i) {
        let close = css[open..].find('}').map_or(css.len(), |i| open + i);
        let body = &css[open + 1..close];
        if body.contains("--dg-ink:") && !body.contains('{') {
            let head = &css[from..open];
            let selector = head
                .rsplit(['}', ';', '{'])
                .next()
                .unwrap_or(head)
                .lines()
                .filter(|l| !l.trim().starts_with("/*") && !l.trim().starts_with('*'))
                .collect::<Vec<_>>()
                .join(" ");
            let tokens = body
                .split(';')
                .filter_map(|d| {
                    let (k, v) = d.split_once(':')?;
                    let k = k.trim();
                    k.starts_with("--dg-")
                        .then(|| (k.to_string(), v.trim().to_string()))
                })
                .collect();
            out.push((
                selector.split_whitespace().collect::<Vec<_>>().join(" "),
                tokens,
            ));
        }
        from = open + 1;
    }
    out
}

/// The diagram color pairs of one theme that miss their contrast: text
/// (ink, muted, the digits on a step's dot) at least 4.5:1 on what it is
/// drawn on, lines, borders and dots at least 3:1 against the page and the
/// boxes.
pub fn contrast_problems(theme: &str, tokens: &[(String, String)]) -> Vec<String> {
    const TEXT: [(&str, &str); 8] = [
        ("--dg-ink", "--dg-box"),
        ("--dg-muted", "--dg-box"),
        ("--dg-ink", "--dg-file"),
        ("--dg-muted", "--dg-file"),
        ("--dg-ink", "--dg-bg"),
        ("--dg-muted", "--dg-bg"),
        ("--dg-on-you", "--dg-you"),
        ("--dg-box", "--dg-claude"),
    ];
    const LINES: [(&str, &str); 6] = [
        ("--dg-line", "--dg-bg"),
        ("--dg-you", "--dg-bg"),
        ("--dg-claude", "--dg-bg"),
        ("--dg-line", "--dg-box"),
        ("--dg-you", "--dg-box"),
        ("--dg-claude", "--dg-box"),
    ];
    let value = |k: &str| tokens.iter().find(|(n, _)| n == k).map(|(_, v)| v.as_str());
    let mut problems = Vec::new();
    for (pairs, min) in [(&TEXT[..], 4.5), (&LINES[..], 3.0)] {
        for (fg, bg) in pairs {
            match (value(fg), value(bg)) {
                (Some(a), Some(b)) => match contrast(a, b) {
                    Some(r) if r >= min => {}
                    Some(r) => problems.push(format!(
                        "{theme}: {fg} {a} on {bg} {b} is {r:.2}:1, under {min}:1"
                    )),
                    None => problems.push(format!("{theme}: {fg} or {bg} is not a #rrggbb color")),
                },
                _ => problems.push(format!("{theme}: {fg} or {bg} is missing")),
            }
        }
    }
    problems
}

/// `text` without what stands between parentheses.
fn without_parentheses(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut depth = 0usize;
    for c in text.chars() {
        match c {
            '(' => depth += 1,
            ')' => depth = depth.saturating_sub(1),
            _ if depth == 0 => out.push(c),
            _ => {}
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn words_outside_code_and_under_the_hood_are_counted() {
        let prose = "word ".repeat(480);
        let folded = format!(
            "<details>\n<summary>Under the hood: x</summary>\n\n{}\n</details>\n",
            "hidden ".repeat(400)
        );
        let code = format!("```bash\n{}\n```\n", "code ".repeat(100));
        assert_eq!(prose_words(&format!("{prose}\n{folded}{code}")), 480);
        let figure = format!(
            "<figure class=\"diagram\">\n{}</figure>\n",
            "drawn ".repeat(200)
        );
        assert_eq!(
            prose_words(&without_figures(&format!(
                "{prose}\n{folded}{code}{figure}"
            ))),
            480
        );
    }

    const SUMMARY: &str = "# Summary\n\n# Start here\n\n- [A](introduction.md)\n- [B](choose-your-path.md)\n  - [R](paths/review.md)\n  - [U](paths/build.md)\n  - [V](paths/develop.md)\n  - [T](paths/team.md)\n- [D](glossary.md)\n\n# Everyday use\n\n- [E](concepts.md)\n";

    #[test]
    fn a_well_formed_start_here_part_passes() {
        assert!(start_here_problems(SUMMARY).is_empty());
        assert_eq!(
            parts(SUMMARY)[1],
            ("Everyday use".into(), vec!["concepts.md".into()])
        );
    }

    #[test]
    fn a_chapter_moved_into_start_here_is_caught() {
        let summary = SUMMARY.replace(
            "- [D](glossary.md)\n",
            "- [D](glossary.md)\n- [E](concepts.md)\n",
        );
        assert_eq!(start_here_problems(&summary), ["unexpected concepts.md"]);
    }

    #[test]
    fn a_chapter_missing_from_start_here_is_caught() {
        let summary = SUMMARY.replace("- [D](glossary.md)\n", "");
        assert_eq!(start_here_problems(&summary), ["missing glossary.md"]);
    }

    #[test]
    fn a_terminal_command_outside_under_the_hood_is_caught() {
        let md = "Step.\n\n```bash\ncompagnon validate\n```\n\n<details>\n<summary>Under the hood: y</summary>\n\n```bash\ncompagnon status\n```\n</details>\n";
        assert_eq!(stray_commands(md), ["compagnon validate"]);
    }

    #[test]
    fn installing_and_initializing_are_allowed_outside_the_blocks() {
        let md = "```bash\ncurl -sSL https://example.test/install.sh | sh\ncompagnon init\n```\n";
        assert!(stray_commands(md).is_empty());
    }

    #[test]
    fn a_missing_or_overlong_glossary_entry_is_caught() {
        let md = format!(
            "# Glossary\n\n## Spec\n\n{}\n\n## Plan\n\nA plan.\n",
            "word ".repeat(90)
        );
        assert_eq!(
            glossary_problems(&md, &["Delta", "Spec", "Plan"], 60),
            ["missing Delta", "Spec: 90 words"]
        );
    }

    fn titles() -> Vec<(String, String)> {
        [
            ("introduction.md", "Compagnon in two minutes"),
            ("glossary.md", "Glossary"),
            ("workflow.md", "The workflow"),
        ]
        .map(|(p, t)| (p.to_string(), t.to_string()))
        .to_vec()
    }

    const PAGE: &str = "# Review plans\n\nFor you if you do not code; ten minutes.\n\n\
        1. [Compagnon in two minutes](../introduction.md)\n2. [Glossary](../glossary.md)\n\n\
        ## Next, if you need it\n\n- [The workflow](../workflow.md)\n\n\
        [Choose another path](../choose-your-path.md)\n";

    #[test]
    fn a_step_labelled_differently_from_its_page_is_caught() {
        assert!(
            path_page_problems(PAGE, "Review plans", "Next, if you need it", &titles()).is_empty()
        );
        let renamed = PAGE.replace(
            "2. [Glossary](../glossary.md)",
            "2. [Step by step](../workflow.md)",
        );
        assert_eq!(
            path_page_problems(&renamed, "Review plans", "Next, if you need it", &titles()),
            ["step \"Step by step\" opens \"The workflow\""]
        );
    }

    #[test]
    fn a_path_page_with_four_steps_is_caught() {
        let four = PAGE.replace(
            "2. [Glossary](../glossary.md)\n",
            "2. [Glossary](../glossary.md)\n3. [Glossary](../glossary.md)\n4. [Glossary](../glossary.md)\n",
        );
        assert_eq!(
            path_page_problems(&four, "Review plans", "Next, if you need it", &titles()),
            ["4 steps"]
        );
    }

    #[test]
    fn a_profile_with_two_links_or_the_wrong_page_is_caught() {
        let labels = ["A", "B", "C", "D"];
        let good = "- [A](paths/review.md)\n- [B](paths/build.md)\n- [C](paths/develop.md)\n\
                    - [D](paths/team.md)\n\nTo change Compagnon, see [Origins](origins.md).\n";
        assert!(
            choose_path_problems(good, &labels).is_empty(),
            "{:?}",
            choose_path_problems(good, &labels)
        );
        let two = good.replace(
            "- [A](paths/review.md)",
            "- [A](paths/review.md), see [the glossary](glossary.md)",
        );
        assert_eq!(
            choose_path_problems(&two, &labels),
            ["\"A\": 2 links in its item"]
        );
        let wrong = good.replace("[B](paths/build.md)", "[B](paths/team.md)");
        assert_eq!(
            choose_path_problems(&wrong, &labels),
            ["\"B\" links paths/team.md, not paths/build.md"]
        );
    }

    #[test]
    fn a_path_page_that_does_not_link_its_new_page_is_caught() {
        let page = "# Review\n\nFor you.\n\n1. [A](../introduction.md)\n\n## Next, if you need it\n\n\
                    - [Example](../guides/example-plan.md)\n";
        assert_eq!(
            missing_path_links(
                page,
                "Next, if you need it",
                &["introduction.md"],
                &["guides/example-plan.md", "guides/approve-a-plan.md"]
            ),
            ["no guides/approve-a-plan.md under \"Next, if you need it\""]
        );
    }

    #[test]
    fn a_part_of_the_example_without_its_note_is_caught() {
        let parts = ["The proposal", "The specs", "The design", "The tasks"];
        let mut md = String::from("# Example\n");
        for part in parts {
            md.push_str(&format!(
                "\n## {part}\n\n```markdown\n## Why\n```\n\n### What to check here\n\nCheck.\n"
            ));
        }
        assert!(example_problems(&md, &parts, "What to check here").is_empty());
        let bare = md.replacen(
            "## The design\n\n```markdown\n## Why\n```\n\n### What to check here",
            "## The design\n\n```markdown\n## Why\n```\n\n### Elsewhere",
            1,
        );
        assert_eq!(
            example_problems(&bare, &parts, "What to check here"),
            ["\"The design\" has no \"What to check here\" after its quoted plan"]
        );
    }

    #[test]
    fn a_command_in_a_page_for_non_developers_is_caught() {
        assert_eq!(shell_blocks("```bash\ncompagnon status\n```\n"), ["bash"]);
        assert_eq!(
            stray_commands("```bash\ncompagnon validate\n```\n"),
            ["compagnon validate"]
        );
    }

    #[test]
    fn a_gloss_for_developers_is_caught() {
        assert_eq!(
            developer_glosses("a terminal (the window where you\ntype commands)"),
            ["the window where you type commands"]
        );
        assert!(developer_glosses("a [terminal](glossary.md#terminal)").is_empty());
    }

    #[test]
    fn a_shell_block_in_review_a_plan_is_caught() {
        assert_eq!(
            shell_blocks("```markdown\n## Why\n```\n\n```bash\nls\n```\n"),
            ["bash"]
        );
        assert!(shell_blocks("```text\nWHEN the user…\n```\n").is_empty());
        assert_eq!(shell_blocks("~~~sh\nls\n~~~\n"), ["sh"]);
    }

    #[test]
    fn a_malformed_under_the_hood_block_is_caught() {
        let md = "<details open>\n<summary>Sous le capot : a</summary>\n\nx\n</details>\n<details>\n<summary>Details</summary>\n\ny\n</details>\n";
        let problems = details_problems(md, "Sous le capot");
        assert_eq!(problems.len(), 2, "{problems:?}");
        assert!(problems[0].starts_with("open by default"));
        assert!(problems[1].contains("\"Details\""));
    }

    #[test]
    fn two_blocks_with_the_same_name_are_caught() {
        let block = "<details>\n<summary>Under the hood</summary>\n\nx\n</details>\n";
        assert_eq!(
            details_problems(&block.repeat(2), "Under the hood"),
            ["summary \"Under the hood\" used twice"]
        );
    }

    #[test]
    fn a_skill_without_examples_is_caught() {
        let mut md = String::new();
        for skill in SKILLS {
            let examples = if skill == "compagnon-sync" { 1 } else { 2 };
            md.push_str(&format!("### X (`/{skill}`)\n\n"));
            for i in 0..examples {
                md.push_str(&format!("- \"ask {i}\"\n"));
            }
        }
        assert_eq!(how_to_ask_problems(&md), ["compagnon-sync: 1 example(s)"]);
    }

    #[test]
    fn a_step_naming_the_command_before_the_request_is_caught() {
        let md = "## 1. Propose (`/compagnon-propose`)\n\n> \"Add a dark mode\"\n\nOr `/compagnon-propose`.\n\n## 2. Apply (`/compagnon-apply`)\n\nRun `/compagnon-apply`.\n\n> \"Implement it\"\n";
        assert_eq!(
            requests_after_commands(md),
            ["2. Apply (`/compagnon-apply`)"]
        );
    }

    fn anchors() -> Vec<String> {
        glossary_anchors("## Delta\n\nThe part…\n\n## Schema\n\nThe shape…\n")
    }

    #[test]
    fn a_term_used_without_a_link_is_caught() {
        let md = "A delta lists the change.\n\nA [delta](glossary.md#delta) again.\n";
        assert_eq!(vocabulary_problems(md, Lang::En, &anchors()), ["delta"]);
        let linked = "A [delta](../glossary.md#delta) lists it; the delta stays.\n";
        assert!(vocabulary_problems(linked, Lang::En, &anchors()).is_empty());
    }

    #[test]
    fn a_link_target_is_not_prose() {
        let md = "See [Step by step](workflow.md) and [the plan](concepts.md#delta).\n";
        assert!(vocabulary_problems(md, Lang::En, &anchors()).is_empty());
    }

    #[test]
    fn a_chapter_named_in_a_link_is_not_prose() {
        let md = "Then read [The workflow](workflow.md).\n";
        assert!(vocabulary_problems(md, Lang::En, &anchors()).is_empty());
    }

    #[test]
    fn a_link_to_the_top_of_the_glossary_is_caught() {
        let bare = "A [delta](glossary.md) lists the change.\n";
        assert_eq!(vocabulary_problems(bare, Lang::En, &anchors()), ["delta"]);
        let unknown = "A [delta](glossary.md#deltas) lists the change.\n";
        assert_eq!(
            vocabulary_problems(unknown, Lang::En, &anchors()),
            ["delta"]
        );
    }

    #[test]
    fn a_term_in_code_or_under_the_hood_is_not_concerned() {
        let md = "## The schema\n\nSet `schema: spec-driven`.\n\n```yaml\nschema: x\n```\n\n\
                  <details>\n<summary>Under the hood: the schema</summary>\n\nThe schema.\n\n</details>\n";
        assert!(vocabulary_problems(md, Lang::En, &anchors()).is_empty());
    }

    #[test]
    fn the_phrase_main_specs_is_caught() {
        let md = "It merges the deltas into the main specs.\n";
        let problems = vocabulary_problems(md, Lang::En, &anchors());
        assert!(problems.contains(&"main spec".to_string()), "{problems:?}");
    }

    #[test]
    fn an_anglicism_in_prose_is_caught() {
        assert_eq!(
            anglicisms("Relisez le change avant de l'archiver.\n"),
            ["change"]
        );
        assert_eq!(
            anglicisms("## Les skills\n\nUne spec.\n"),
            ["skills", "spec"]
        );
        assert_eq!(anglicisms("Ouvrez une pull request.\n"), ["pull request"]);
    }

    #[test]
    fn the_french_verb_changer_is_not_an_anglicism() {
        assert!(anglicisms("Voici ce qui change.\n").is_empty());
        assert!(anglicisms("Rien ne change.\n").is_empty());
        assert_eq!(anglicisms("Relisez ce change.\n"), ["change"]);
    }

    #[test]
    fn the_english_word_in_parentheses_code_or_path_is_allowed() {
        let md = "Une compétence (skill) se lance avec `/compagnon-apply`, \
                  dans `_compagnon/changes/` ; voir [le guide](guides/review-a-plan.md).\n";
        assert!(anglicisms(md).is_empty(), "{:?}", anglicisms(md));
        assert!(anglicisms("Spec Kit vous conviendra mieux.\n").is_empty());
    }

    #[test]
    fn an_anglicism_of_a_technical_page_is_caught() {
        assert_eq!(
            anglicisms("Lisez le design et le template du schéma.\n"),
            ["design", "template"]
        );
        assert_eq!(anglicisms("Ouvrez une merge request.\n"), ["merge request"]);
        assert_eq!(
            anglicisms("## Changes\n\nAucun change n'est actif.\n"),
            ["changes", "change"]
        );
    }

    #[test]
    fn a_fence_quoted_in_a_longer_fence_stays_code() {
        let md = "````markdown\n```yaml\nlocation: design.md\n```\n````\n\nLa relecture.\n";
        assert!(anglicisms(md).is_empty(), "{:?}", anglicisms(md));
        let quoted = "````markdown\n```bash\nls\n```\n````\n\n```sh\npwd\n```\n";
        assert_eq!(shell_blocks(quoted), ["sh"]);
    }

    #[test]
    fn identifier_headings_are_not_prose() {
        let md = "## review\n\n## design.md\n\n### compagnon review\n\n\
                  <svg><title>skill</title></svg>\n\nLa relecture.\n";
        assert!(anglicisms(md).is_empty(), "{:?}", anglicisms(md));
    }

    const GOOD_DIAGRAM: &str = "<figure class=\"diagram\">\n\
        <svg viewBox=\"0 0 10 10\" role=\"img\" aria-label=\"Two steps: you ask, Claude plans\">\n\
        <defs><marker id=\"a\"/></defs><line marker-end=\"url(#a)\"/><use href=\"#a\"/>\n\
        <text x=\"1\">Ask</text>\n\
        </svg>\n<figcaption>The workflow.</figcaption>\n</figure>\n";

    #[test]
    fn a_complete_diagram_passes() {
        assert!(
            diagram_problems(GOOD_DIAGRAM).is_empty(),
            "{:?}",
            diagram_problems(GOOD_DIAGRAM)
        );
        let hidden = "<svg aria-hidden=\"true\"><use href=\"#cp-mark\"/></svg>\n\
                      <svg class=\"logo\" role=\"img\" aria-label=\"Mairistem\"></svg>";
        assert!(
            diagram_problems(hidden).is_empty(),
            "{:?}",
            diagram_problems(hidden)
        );
    }

    #[test]
    fn a_diagram_without_label_is_caught() {
        let md = GOOD_DIAGRAM.replace(" aria-label=\"Two steps: you ask, Claude plans\"", "");
        let problems = diagram_problems(&md);
        assert!(
            problems.iter().any(|p| p.contains("no aria-label")),
            "{problems:?}"
        );
    }

    #[test]
    fn a_diagram_split_by_a_blank_line_is_caught() {
        let md = GOOD_DIAGRAM.replace("<text x", "\n<text x");
        let problems = diagram_problems(&md);
        assert!(
            problems.iter().any(|p| p.contains("blank line")),
            "{problems:?}"
        );
    }

    #[test]
    fn a_diagram_outside_a_figure_or_pointing_out_is_caught() {
        let bare = "<svg role=\"img\" aria-label=\"x\"><image href=\"chart.png\"/></svg>";
        let problems = diagram_problems(bare);
        assert!(
            problems.iter().any(|p| p.contains("<figure>")),
            "{problems:?}"
        );
        assert!(
            problems.iter().any(|p| p.contains("another file")),
            "{problems:?}"
        );
        let no_caption = GOOD_DIAGRAM.replace("<figcaption>The workflow.</figcaption>\n", "");
        assert!(
            diagram_problems(&no_caption)
                .iter()
                .any(|p| p.contains("figcaption"))
        );
    }

    #[test]
    fn contrast_follows_wcag() {
        let r = contrast("#ffffff", "#000000").unwrap();
        assert!((r - 21.0).abs() < 0.01, "{r}");
        let coral = contrast("#ffffff", "#ff5331").unwrap();
        assert!((3.0..3.4).contains(&coral), "{coral}");
    }

    #[test]
    fn a_diagram_pair_under_its_threshold_is_caught() {
        let css = ".light {\n  --dg-bg: #ffffff;\n  --dg-box: #f4f8f9;\n  --dg-file: #ffffff;\n\
                   --dg-ink: #00263a;\n  --dg-muted: #b0b0b0;\n  --dg-line: #6b8590;\n\
                   --dg-you: #e0421f;\n  --dg-on-you: #1a0a05;\n  --dg-claude: #004c66;\n}\n";
        let themes = diagram_themes(css);
        assert_eq!(themes.len(), 1);
        assert_eq!(themes[0].0, ".light");
        let problems = contrast_problems(&themes[0].0, &themes[0].1);
        assert!(
            problems
                .iter()
                .any(|p| p.contains(".light") && p.contains("--dg-muted #b0b0b0")),
            "{problems:?}"
        );
    }

    #[test]
    fn an_entry_point_without_jira_is_caught() {
        assert!(
            jira_problems(
                "Say \"Prepare a plan for PROJ-123\": [Start from a Jira ticket](guides/mcp.md)."
            )
            .is_empty()
        );
        let problems = jira_problems("# Compagnon in two minutes\n\nPlan with Claude.\n");
        assert_eq!(
            problems,
            ["never names Jira", "does not link the Jira guide"]
        );
    }

    #[test]
    fn jira_between_two_profiles_is_caught() {
        let labels = ["A", "B", "C", "D"];
        assert!(jira_after_profiles("A B C D … Jira", &labels));
        assert!(!jira_after_profiles("A B Jira C D", &labels));
    }

    #[test]
    fn a_guide_that_configures_before_it_shows_is_caught() {
        let good = "# Start\n\nSay it.\n\n<figure class=\"diagram\">…</figure>\n\n## Set it up\n";
        assert!(jira_guide_problems(good).is_empty());
        let bad = "# Start\n\n## Set it up\n\n<figure class=\"diagram\">…</figure>\n";
        assert_eq!(jira_guide_problems(bad).len(), 1);
    }

    #[test]
    fn a_code_left_in_the_faq_is_caught() {
        let faq = "# FAQ\n\n### `decision_seal_mismatch`\n\nFix it.\n";
        let errors = "# Error codes\n\n### `legacy_source`\n";
        let problems = faq_code_problems(faq, errors, &["decision_seal_mismatch", "legacy_source"]);
        assert_eq!(
            problems,
            [
                "the FAQ still explains `decision_seal_mismatch`",
                "the errors chapter has no ### `decision_seal_mismatch`"
            ]
        );
    }

    #[test]
    fn a_long_chapter_or_one_without_go_further_is_caught() {
        let title = |t: &str| (t == "roles.md").then(|| "Roles".to_string());
        let good = "# FAQ\n\nShort.\n\n<figure>many many words</figure>\n\n## Go further\n\n- [Roles](roles.md)\n";
        assert!(
            everyday_problems(good, 5, Lang::En, title).is_empty(),
            "{:?}",
            everyday_problems(good, 5, Lang::En, title)
        );
        let long = format!(
            "# FAQ\n\n{}\n\n## Go further\n\n- [Roles](roles.md)\n",
            "word ".repeat(1300)
        );
        assert!(everyday_problems(&long, 1200, Lang::En, title)[0].contains("over 1200"));
        let ends = "# Concepts\n\n## Seal\n\nText.\n";
        assert_eq!(
            everyday_problems(ends, 1200, Lang::En, title),
            ["ends with \"Seal\", not \"Go further\""]
        );
        let wrong = "# FAQ\n\n## Go further\n\n- [The roles](roles.md)\n";
        assert_eq!(everyday_problems(wrong, 1200, Lang::En, title).len(), 1);
    }

    #[test]
    fn a_french_diagram_speaks_french() {
        let fr = GOOD_DIAGRAM.replace("The workflow.", "Le déroulé.");
        let md = fr
            .replace(
                "Two steps: you ask, Claude plans",
                "Le workflow en deux étapes",
            )
            .replace(">Ask<", ">Demander<");
        assert_eq!(anglicisms(&md), ["workflow"]);
        let text = fr
            .replace("Two steps: you ask, Claude plans", "Deux étapes")
            .replace(">Ask<", ">La review<");
        assert_eq!(anglicisms(&text), ["review"]);
    }
}

/// The checks applied to the embedded chapters of both editions.
#[cfg(test)]
mod chapter_tests {
    use super::*;

    fn summary(lang: Lang) -> &'static str {
        match lang {
            Lang::En => include_str!("../../../docs/en/src/SUMMARY.md"),
            Lang::Fr => include_str!("../../../docs/fr/src/SUMMARY.md"),
        }
    }

    fn chapter(lang: Lang, path: &str) -> &'static str {
        lang.chapters()
            .iter()
            .find(|(p, _)| *p == path)
            .map(|(_, md)| *md)
            .unwrap_or_else(|| panic!("{} has no {path}", lang.tag()))
    }

    fn paths(lang: Lang) -> Vec<&'static str> {
        lang.chapters().iter().map(|(p, _)| *p).collect()
    }

    const LANGS: [Lang; 2] = [Lang::En, Lang::Fr];

    /// `(English, French)` word caps.
    fn cap(lang: Lang, (en, fr): (usize, usize)) -> usize {
        if lang == Lang::En { en } else { fr }
    }

    #[test]
    fn both_editions_open_with_the_start_here_path() {
        let titles = [
            [
                "Start here",
                "Everyday use",
                "Going further",
                "Reference",
                "Contributing",
            ],
            [
                "Commencer ici",
                "Au quotidien",
                "Aller plus loin",
                "Référence",
                "Contribuer",
            ],
        ];
        for (lang, expected) in LANGS.into_iter().zip(titles) {
            let summary = summary(lang);
            assert!(
                start_here_problems(summary).is_empty(),
                "{}: {:?}",
                lang.tag(),
                start_here_problems(summary)
            );
            let parts = parts(summary);
            let found: Vec<&str> = parts.iter().map(|(t, _)| t.as_str()).collect();
            assert_eq!(found, expected, "{}", lang.tag());
            assert_eq!(
                parts[1].1.first().map(String::as_str),
                Some("quickstart.md"),
                "{}: Everyday use opens with the quickstart",
                lang.tag()
            );
            // The fifth part is Contributing: its chapter paths.
            let (_, contributing) = &parts[4];
            for c in ["origins.md", "architecture.md", "reliability.md"] {
                assert!(
                    contributing.iter().any(|p| p == c),
                    "{}: Contributing lacks {c}",
                    lang.tag()
                );
            }
        }
    }

    #[test]
    fn both_editions_follow_the_refactored_order() {
        for lang in LANGS {
            let problems = toc_problems(summary(lang));
            assert!(problems.is_empty(), "{}: {problems:#?}", lang.tag());
            let titles = title_problems(summary(lang), |path| {
                lang.chapters()
                    .iter()
                    .find(|(p, _)| *p == path)
                    .and_then(|(_, md)| md.lines().find_map(|l| l.strip_prefix("# ")))
                    .map(str::to_string)
            });
            assert!(titles.is_empty(), "{}: {titles:#?}", lang.tag());
        }
    }

    #[test]
    fn a_chapter_out_of_its_part_is_caught() {
        let moved = summary(Lang::En)
            .replace("- [Roles](roles.md)\n", "")
            .replace(
                "- [FAQ](faq.md)\n",
                "- [FAQ](faq.md)\n- [Roles](roles.md)\n",
            );
        let problems = toc_problems(&moved);
        assert!(
            problems
                .iter()
                .any(|p| p.contains("roles.md does not belong")),
            "{problems:?}"
        );
        let renamed = summary(Lang::En).replace("[FAQ](faq.md)", "[Questions](faq.md)");
        let titles = title_problems(&renamed, |_| Some("FAQ".into()));
        assert!(titles.iter().any(|p| p.contains("faq.md")), "{titles:?}");
    }

    #[test]
    fn no_chapter_is_removed() {
        let before = [
            "introduction.md",
            "installation.md",
            "quickstart.md",
            "workflow.md",
            "skills.md",
            "roles.md",
            "reliability.md",
            "concepts.md",
            "guides/moving-from-codev.md",
            "guides/inherited-sources.md",
            "guides/repository-integration.md",
            "guides/release-notes.md",
            "guides/review-record.md",
            "guides/mcp.md",
            "guides/artifact-language.md",
            "guides/custom-schemas.md",
            "reference/cli.md",
            "reference/configuration.md",
            "reference/file-formats.md",
            "reference/json-output.md",
            "faq.md",
            "architecture.md",
            "guides/decisions.md",
            "reference/errors.md",
        ];
        for lang in LANGS {
            let now = paths(lang);
            for c in before {
                assert!(now.contains(&c), "{}: {c} is no longer listed", lang.tag());
            }
        }
    }

    #[test]
    fn the_start_here_path_stays_under_its_caps() {
        for lang in LANGS {
            for (path, caps) in [
                ("introduction.md", (450, 560)),
                ("choose-your-path.md", (200, 250)),
                (PATHS[0], (250, 310)),
                (PATHS[1], (250, 310)),
                (PATHS[2], (250, 310)),
                (PATHS[3], (250, 310)),
                ("quickstart.md", (500, 625)),
            ] {
                let words = prose_words(&without_figures(chapter(lang, path)));
                let cap = cap(lang, caps);
                assert!(
                    words <= cap,
                    "{} {path}: {words} words, cap {cap}",
                    lang.tag()
                );
            }
        }
    }

    #[test]
    fn the_introduction_says_who_decides() {
        for (lang, heading) in [(Lang::En, "## Who decides"), (Lang::Fr, "## Qui décide")] {
            assert!(
                chapter(lang, "introduction.md")
                    .lines()
                    .any(|l| l.trim() == heading),
                "{} introduction lacks `{heading}`",
                lang.tag()
            );
        }
    }

    #[test]
    fn the_quickstart_walks_the_four_skills_in_order() {
        for lang in LANGS {
            let headings: Vec<&str> = chapter(lang, "quickstart.md")
                .lines()
                .filter(|l| l.starts_with("## "))
                .collect();
            let position = |skill: &str| {
                headings
                    .iter()
                    .position(|h| h.contains(&format!("/{skill}`")))
                    .unwrap_or_else(|| panic!("{}: no step heading for {skill}", lang.tag()))
            };
            let (p, a, r) = (
                position("compagnon-propose"),
                position("compagnon-apply"),
                position("compagnon-archive"),
            );
            // Four steps: propose, then reading the plan — a step of its
            // own, with no skill — then apply, then archive.
            let reading_the_plan_comes_between = p + 1 < a;
            assert!(
                reading_the_plan_comes_between && a < r,
                "{}: steps out of order: {headings:?}",
                lang.tag()
            );
        }
    }

    #[test]
    fn the_quickstart_runs_no_command_outside_under_the_hood() {
        for lang in LANGS {
            let stray = stray_commands(chapter(lang, "quickstart.md"));
            assert!(stray.is_empty(), "{}: {stray:?}", lang.tag());
        }
    }

    #[test]
    fn each_quickstart_step_leads_with_a_plain_request() {
        for lang in LANGS {
            let late = requests_after_commands(chapter(lang, "quickstart.md"));
            assert!(late.is_empty(), "{}: {late:?}", lang.tag());
        }
    }

    #[test]
    fn every_skill_has_examples_of_requests() {
        for (lang, heading) in [
            (Lang::En, "## How to ask"),
            (Lang::Fr, "## Comment demander"),
        ] {
            let skills = chapter(lang, "skills.md");
            let start = skills
                .find(&format!("{heading}\n"))
                .unwrap_or_else(|| panic!("{} skills chapter lacks `{heading}`", lang.tag()));
            let rest = &skills[start + heading.len()..];
            let section = &rest[..rest.find("\n## ").unwrap_or(rest.len())];
            let problems = how_to_ask_problems(section);
            assert!(problems.is_empty(), "{}: {problems:?}", lang.tag());
        }
    }

    #[test]
    fn the_glossary_defines_every_required_term() {
        for lang in LANGS {
            let problems = glossary_problems(
                chapter(lang, "glossary.md"),
                glossary_terms(lang),
                cap(lang, (60, 75)),
            );
            assert!(problems.is_empty(), "{}: {problems:?}", lang.tag());
        }
    }

    /// Each chapter's path and `# ` title: what a reader hears on arriving
    /// at the page. The path pages also match the sidebar, checked below.
    fn chapter_titles(lang: Lang) -> Vec<(String, String)> {
        lang.chapters()
            .iter()
            .map(|(p, md)| {
                let title = md
                    .lines()
                    .find_map(|l| l.strip_prefix("# "))
                    .unwrap_or_default()
                    .trim()
                    .to_string();
                (p.to_string(), title)
            })
            .collect()
    }

    #[test]
    fn both_editions_ask_the_question_with_the_four_profiles() {
        for lang in LANGS {
            let problems =
                choose_path_problems(chapter(lang, "choose-your-path.md"), &profile_labels(lang));
            assert!(problems.is_empty(), "{}: {problems:?}", lang.tag());
        }
    }

    #[test]
    fn the_introduction_links_the_four_paths() {
        for lang in LANGS {
            let problems = profile_link_problems(
                chapter(lang, "introduction.md"),
                &profile_labels(lang),
                &PATHS.map(String::from),
            );
            assert!(problems.is_empty(), "{}: {problems:?}", lang.tag());
        }
    }

    #[test]
    fn every_path_page_has_at_most_three_steps_titled_as_their_pages() {
        for lang in LANGS {
            let titles = chapter_titles(lang);
            let summary = summary(lang);
            for (path, label) in PATHS.iter().zip(profile_labels(lang)) {
                let problems =
                    path_page_problems(chapter(lang, path), label, next_heading(lang), &titles);
                assert!(problems.is_empty(), "{} {path}: {problems:?}", lang.tag());
                assert!(
                    summary.contains(&format!("[{label}]({path})")),
                    "{}: SUMMARY.md does not title {path} \"{label}\"",
                    lang.tag()
                );
            }
        }
    }

    #[test]
    fn the_build_path_asks_no_command_beyond_installing() {
        for lang in LANGS {
            let page = chapter(lang, PATHS[1]);
            let (steps, next) = page
                .split_once(&format!("## {}", next_heading(lang)))
                .unwrap_or((page, ""));
            for (_, target) in link_pairs(steps) {
                let path = chapter_of(&target);
                if path.ends_with(".md") && path != "choose-your-path.md" {
                    let stray = stray_commands(chapter(lang, path));
                    assert!(stray.is_empty(), "{} {path}: {stray:?}", lang.tag());
                }
            }
            for developer in ["workflow.md", "skills.md", "concepts.md", "faq.md"] {
                assert!(
                    !next.contains(developer),
                    "{}: the build path's Next names {developer}",
                    lang.tag()
                );
            }
        }
    }

    #[test]
    fn every_link_of_the_paths_resolves() {
        for lang in LANGS {
            let chapters = paths(lang);
            let pages = std::iter::once("choose-your-path.md").chain(PATHS);
            for page in pages {
                let base = if page.starts_with("paths/") {
                    "paths/"
                } else {
                    ""
                };
                for (_, target) in link_pairs(chapter(lang, page)) {
                    if target.contains("://") {
                        continue;
                    }
                    let (path, anchor) = target.split_once('#').unwrap_or((&target, ""));
                    let resolved = match path.strip_prefix("../") {
                        Some(up) => up.to_string(),
                        None => format!("{base}{path}"),
                    };
                    assert!(
                        chapters.contains(&resolved.as_str()),
                        "{} {page}: {target} is not a chapter",
                        lang.tag()
                    );
                    if !anchor.is_empty() {
                        let mut slugger = crate::docs::Slugger::default();
                        let slugs: Vec<String> =
                            crate::docs::heading_texts(chapter(lang, &resolved))
                                .iter()
                                .map(|h| slugger.slug(h))
                                .collect();
                        assert!(
                            slugs.iter().any(|s| s == anchor),
                            "{} {page}: {target} names no heading",
                            lang.tag()
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn developer_pages_do_not_gloss_everyday_words() {
        for lang in LANGS {
            for path in ["workflow.md", "skills.md", "concepts.md", "faq.md"] {
                let glosses = developer_glosses(chapter(lang, path));
                assert!(glosses.is_empty(), "{} {path}: {glosses:?}", lang.tag());
            }
        }
    }

    #[test]
    fn review_a_plan_covers_the_plan_without_a_terminal() {
        for (lang, words) in [
            (
                Lang::En,
                [
                    "proposal", "specs", "design", "tasks", "added", "modified", "removed",
                    "renamed",
                ],
            ),
            (
                Lang::Fr,
                [
                    "proposition",
                    "spécification",
                    "conception",
                    "tâches",
                    "ajoutées",
                    "modifiées",
                    "supprimées",
                    "renommées",
                ],
            ),
        ] {
            let guide = chapter(lang, "guides/review-a-plan.md");
            assert!(
                shell_blocks(guide).is_empty(),
                "{}: {:?}",
                lang.tag(),
                shell_blocks(guide)
            );
            let lower = guide.to_lowercase();
            for w in words {
                assert!(
                    lower.contains(w),
                    "{}: the guide does not name {w}",
                    lang.tag()
                );
            }
        }
    }

    #[test]
    fn under_the_hood_blocks_are_well_formed() {
        for (lang, prefix) in [(Lang::En, "Under the hood"), (Lang::Fr, "Sous le capot")] {
            for (path, md) in lang.chapters() {
                let problems = details_problems(md, prefix);
                assert!(problems.is_empty(), "{} {path}: {problems:?}", lang.tag());
            }
        }
    }

    #[test]
    fn the_manual_starts_with_the_start_here_path() {
        for lang in LANGS {
            assert_eq!(
                &paths(lang)[..START_HERE.len()],
                START_HERE,
                "{}",
                lang.tag()
            );
        }
    }

    #[test]
    fn the_offline_manual_renders_a_folded_block() {
        let md = "# T\n\n<details>\n<summary>Under the hood: what init writes</summary>\n\n```bash\ncompagnon status\n```\n\n</details>\n";
        let html = crate::docs::render_html(md, Lang::En, "0.7.0");
        assert!(html.contains("<details>"), "{html}");
        assert!(!html.contains("<details open"), "{html}");
        assert!(
            html.contains("<summary>Under the hood: what init writes</summary>"),
            "{html}"
        );
        assert!(
            html.contains("<pre><code class=\"language-bash\">compagnon status"),
            "{html}"
        );
    }

    /// The chapters of the "Start here" and "Everyday use" parts, as both
    /// `SUMMARY.md` list them: `the_everyday_list_follows_the_summary`
    /// fails if a chapter is added to those parts and not here.
    const EVERYDAY: [&str; 18] = [
        "introduction.md",
        "choose-your-path.md",
        "paths/review.md",
        "paths/build.md",
        "paths/develop.md",
        "paths/team.md",
        "glossary.md",
        "quickstart.md",
        "guides/start-from-nothing.md",
        "installation.md",
        "workflow.md",
        "skills.md",
        "guides/mcp.md",
        "guides/review-a-plan.md",
        "guides/example-plan.md",
        "guides/approve-a-plan.md",
        "concepts.md",
        "faq.md",
    ];

    #[test]
    fn the_everyday_list_follows_the_summary() {
        for lang in LANGS {
            let parts = parts(summary(lang));
            let listed: Vec<&str> = parts[..2]
                .iter()
                .flat_map(|(_, chapters)| chapters.iter().map(String::as_str))
                .collect();
            assert_eq!(listed, EVERYDAY, "{}", lang.tag());
        }
    }

    #[test]
    fn every_everyday_chapter_explains_its_terms() {
        let mut found = Vec::new();
        for lang in LANGS {
            let anchors = glossary_anchors(chapter(lang, "glossary.md"));
            for path in EVERYDAY.iter().filter(|p| **p != "glossary.md") {
                let problems = vocabulary_problems(chapter(lang, path), lang, &anchors);
                if !problems.is_empty() {
                    found.push(format!("{} {path}: {problems:?}", lang.tag()));
                }
            }
        }
        assert!(found.is_empty(), "{found:#?}");
    }

    #[test]
    fn the_jira_guide_leads_with_what_it_changes() {
        for (lang, title) in [
            (Lang::En, "Start from a Jira ticket"),
            (Lang::Fr, "Partir d'un ticket Jira"),
        ] {
            let md = chapter(lang, "guides/mcp.md");
            assert!(
                md.starts_with(&format!("# {title}\n")),
                "{}: title",
                lang.tag()
            );
            let everyday = &parts(summary(lang))[EVERYDAY_PART].1;
            assert!(
                everyday.iter().any(|c| c == "guides/mcp.md"),
                "{}: part",
                lang.tag()
            );
            let problems = jira_guide_problems(md);
            assert!(problems.is_empty(), "{}: {problems:?}", lang.tag());
        }
    }

    /// The codes and messages the FAQ explained before the refactor, all
    /// moved to "Error codes and fixes".
    const FAQ_CODES: [&str; 22] = [
        "no compagnon project found",
        "skills_review_mode",
        "zero_delta_without_marker",
        "decision_seal_mismatch",
        "predecessor_not_accepted",
        "decision_missing_frontmatter",
        "validation_failed",
        "rename_source_missing",
        "delta_unexpected_heading",
        "inherit_pin_failed",
        "managed_section_malformed",
        "unlocked",
        "needs_update",
        "invalid",
        "unknown_config_key",
        "change_unknown_key",
        "schema_unknown_key",
        "decision_unknown_field",
        "compagnon_version_unsatisfied",
        "update_checksum_mismatch",
        "update_not_writable",
        "compagnon: command not found",
    ];

    #[test]
    fn error_codes_moved_out_of_the_faq() {
        for lang in LANGS {
            let faq = chapter(lang, "faq.md");
            let problems = faq_code_problems(faq, chapter(lang, "reference/errors.md"), &FAQ_CODES);
            assert!(problems.is_empty(), "{}: {problems:#?}", lang.tag());
            assert!(
                faq.contains("reference/errors.md"),
                "{}: the FAQ links the errors chapter",
                lang.tag()
            );
        }
    }

    /// `target`, a link written in chapter `from`, as a chapter path of the
    /// book (`../roles.md#x` from `guides/mcp.md` is `roles.md`).
    fn resolve(from: &str, target: &str) -> String {
        let target = target.split('#').next().unwrap_or_default();
        let mut parts: Vec<&str> = from.split('/').collect();
        parts.pop();
        for segment in target.split('/') {
            match segment {
                ".." => {
                    parts.pop();
                }
                "." | "" => {}
                s => parts.push(s),
            }
        }
        parts.join("/")
    }

    /// The title `SUMMARY.md` gives a chapter.
    fn summary_title(lang: Lang, path: &str) -> Option<String> {
        summary(lang).lines().find_map(|l| {
            let rest = l.trim_start().strip_prefix("- [")?;
            let (title, target) = rest.split_once("](")?;
            (target.trim_end_matches(')') == path).then(|| title.to_string())
        })
    }

    #[test]
    fn every_entry_point_puts_jira_forward() {
        let mut found = Vec::new();
        for lang in LANGS {
            for path in ["introduction.md", "quickstart.md"]
                .into_iter()
                .chain(PATHS)
            {
                for p in jira_problems(chapter(lang, path)) {
                    found.push(format!("{}/{path}: {p}", lang.tag()));
                }
            }
        }
        for (name, readme) in [
            ("README.md", include_str!("../../../README.md")),
            ("README.fr.md", include_str!("../../../README.fr.md")),
        ] {
            for p in jira_problems(readme) {
                found.push(format!("{name}: {p}"));
            }
        }
        assert!(found.is_empty(), "{found:#?}");
    }

    #[test]
    fn everyday_chapters_are_short_and_end_with_go_further() {
        let mut found = Vec::new();
        for lang in LANGS {
            let everyday = &parts(summary(lang))[EVERYDAY_PART].1;
            for path in everyday.iter().filter(|p| *p != "guides/example-plan.md") {
                let problems = everyday_problems(
                    chapter(lang, path),
                    cap(lang, (1200, 1500)),
                    lang,
                    |target| summary_title(lang, &resolve(path, target)),
                );
                for p in problems {
                    found.push(format!("{}/{path}: {p}", lang.tag()));
                }
            }
        }
        assert!(found.is_empty(), "{found:#?}");
    }

    #[test]
    fn every_diagram_theme_is_contrasted_enough() {
        let sheets = [
            (
                "docs/theme/mairistem.css",
                include_str!("../../../docs/theme/mairistem.css"),
            ),
            ("assets/docs.css", include_str!("../assets/docs.css")),
        ];
        let mut found = Vec::new();
        for (file, css) in sheets {
            let themes = diagram_themes(css);
            assert_eq!(
                themes.len(),
                2,
                "{file} should define a light and a dark diagram theme"
            );
            for (selector, tokens) in themes {
                found.extend(contrast_problems(&format!("{file} {selector}"), &tokens));
            }
        }
        assert!(found.is_empty(), "{found:#?}");
    }

    #[test]
    fn every_diagram_has_a_text_alternative() {
        let mut found = Vec::new();
        for lang in LANGS {
            for (path, md) in lang.chapters() {
                for p in diagram_problems(md) {
                    found.push(format!("{}/{path}: {p}", lang.tag()));
                }
            }
        }
        for (page, html) in [
            (
                "docs/site/index.html",
                include_str!("../../../docs/site/index.html"),
            ),
            (
                "docs/site/fr.html",
                include_str!("../../../docs/site/fr.html"),
            ),
        ] {
            for p in diagram_problems(html) {
                found.push(format!("{page}: {p}"));
            }
        }
        assert!(found.is_empty(), "{found:#?}");
    }

    #[test]
    fn the_french_book_holds_no_anglicism() {
        let mut found = Vec::new();
        let chapters: Vec<String> = parts(summary(Lang::Fr))
            .into_iter()
            .flat_map(|(_, chapters)| chapters)
            .collect();
        for path in &chapters {
            let words = anglicisms(chapter(Lang::Fr, path));
            if !words.is_empty() {
                found.push(format!("{path}: {words:?}"));
            }
        }
        let readme = anglicisms(include_str!("../../../README.fr.md"));
        if !readme.is_empty() {
            found.push(format!("README.fr.md: {readme:?}"));
        }
        assert!(found.is_empty(), "{found:#?}");
    }

    /// The parts of `SUMMARY.md`, by their order (`# ` headings).
    const EVERYDAY_PART: usize = 1;
    const GOING_FURTHER_PART: usize = 2;
    const CONTRIBUTING_PART: usize = 4;

    /// The guides the path pages send their readers to, each with the part
    /// of `SUMMARY.md` that lists it.
    const PROFILE_GUIDES: [(&str, usize); 6] = [
        ("guides/example-plan.md", EVERYDAY_PART),
        ("guides/approve-a-plan.md", EVERYDAY_PART),
        ("guides/start-from-nothing.md", EVERYDAY_PART),
        ("guides/existing-project.md", GOING_FURTHER_PART),
        ("guides/team-rollout.md", GOING_FURTHER_PART),
        ("contributing.md", CONTRIBUTING_PART),
    ];

    #[test]
    fn both_editions_list_the_new_pages_where_their_readers_look() {
        for lang in LANGS {
            let parts = parts(summary(lang));
            for (page, part) in PROFILE_GUIDES {
                assert!(
                    parts[part].1.iter().any(|p| p == page),
                    "{}: {page} not in part {}",
                    lang.tag(),
                    parts[part].0
                );
            }
            let next = next_heading(lang);
            // (path page, chapters its steps open, chapters its "Next" links)
            for (path, steps, next_links) in [
                (
                    "paths/review.md",
                    &[][..],
                    &["guides/example-plan.md", "guides/approve-a-plan.md"][..],
                ),
                (
                    "paths/build.md",
                    &["guides/start-from-nothing.md"][..],
                    &["guides/example-plan.md"][..],
                ),
                (
                    "paths/develop.md",
                    &[][..],
                    &["guides/existing-project.md"][..],
                ),
                (
                    "paths/team.md",
                    &["guides/team-rollout.md"][..],
                    &["roles.md"][..],
                ),
            ] {
                let missing = missing_path_links(chapter(lang, path), next, steps, next_links);
                assert!(missing.is_empty(), "{} {path}: {missing:?}", lang.tag());
            }
            assert!(
                chapter(lang, "origins.md").contains("](contributing.md)"),
                "{}: origins.md does not link contributing.md",
                lang.tag()
            );
        }
    }

    /// The parts of the example plan and the heading of their notes.
    fn example_parts(lang: Lang) -> ([&'static str; 4], &'static str) {
        match lang {
            Lang::En => (
                ["The proposal", "The specs", "The design", "The tasks"],
                "What to check here",
            ),
            Lang::Fr => (
                [
                    "La proposition",
                    "Les spécifications",
                    "La conception",
                    "Les tâches",
                ],
                "Ce qu'on vérifie ici",
            ),
        }
    }

    #[test]
    fn every_part_of_the_example_is_annotated() {
        for lang in LANGS {
            let (parts, note) = example_parts(lang);
            let page = chapter(lang, "guides/example-plan.md");
            let problems = example_problems(page, &parts, note);
            assert!(problems.is_empty(), "{}: {problems:?}", lang.tag());
        }
        let french = chapter(Lang::Fr, "guides/example-plan.md");
        for keyword in ["### Exigence :", "#### Scénario :"] {
            assert!(
                french.contains(keyword),
                "the French example lacks `{keyword}`"
            );
        }
    }

    #[test]
    fn the_example_plan_parses_as_compagnon_writes_it() {
        for lang in LANGS {
            let (parts, _) = example_parts(lang);
            let spec = quoted_block(chapter(lang, "guides/example-plan.md"), parts[1])
                .unwrap_or_else(|| panic!("{}: no quoted spec", lang.tag()));
            let parsed = compagnon_core::parser::parse_delta(spec);
            assert!(
                parsed.findings.is_empty(),
                "{}: {:?}",
                lang.tag(),
                parsed.findings
            );
            assert!(
                !parsed.value.sections.is_empty(),
                "{}: no delta section",
                lang.tag()
            );
        }
    }

    #[test]
    fn the_two_pages_keep_to_what_a_non_developer_can_do() {
        for lang in LANGS {
            let approve = chapter(lang, "guides/approve-a-plan.md");
            assert!(
                shell_blocks(approve).is_empty(),
                "{}: {:?}",
                lang.tag(),
                shell_blocks(approve)
            );
            for name in ["GitHub", "GitLab", "_compagnon/changes/"] {
                assert!(
                    approve.contains(name),
                    "{}: the approving page lacks {name}",
                    lang.tag()
                );
            }
            let start = chapter(lang, "guides/start-from-nothing.md");
            let stray = stray_commands(start);
            assert!(stray.is_empty(), "{}: {stray:?}", lang.tag());
            let first_block = start.find("```").unwrap_or(start.len());
            assert!(
                start[..first_block].contains("Claude Code"),
                "{}: the starting page does not name Claude Code before its first command",
                lang.tag()
            );
        }
        // French labels between parentheses, never as code.
        let approve = chapter(Lang::Fr, "guides/approve-a-plan.md");
        for label in ["Approve", "Request changes"] {
            assert!(
                !approve.contains(&format!("`{label}`")),
                "the French approving page writes « {label} » as code"
            );
        }
    }
}
