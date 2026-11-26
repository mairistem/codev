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

/// The language of a fence line (```` ``` ```` or `~~~`), empty when it
/// names none; `None` for any other line.
fn fence_marker(trimmed: &str) -> Option<&str> {
    trimmed
        .strip_prefix("```")
        .or_else(|| trimmed.strip_prefix("~~~"))
        .map(str::trim)
}

/// Walks a chapter, line by line, keeping track of fences and `<details>`.
/// Fence lines and `<details>`/`</details>` lines themselves are skipped.
fn lines(md: &str) -> Vec<Line<'_>> {
    let mut out = Vec::new();
    let mut fence: Option<&str> = None;
    let mut details = 0usize;
    for text in md.lines() {
        let trimmed = text.trim_start();
        if let Some(lang) = fence_marker(trimmed) {
            fence = match fence {
                Some(_) => None,
                None => Some(lang),
            };
            continue;
        }
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
/// matching entry of `targets`. Used on Choose your path, the
/// introduction and the READMEs, whose targets differ only by prefix.
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
    let after = labels
        .last()
        .and_then(|l| md.find(&format!("[{l}]")))
        .map_or("", |i| &md[i..]);
    if !link_pairs(after)
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
        .filter(|(_, l)| {
            let digits = l.chars().take_while(char::is_ascii_digit).count();
            digits > 0 && l[digits..].starts_with(". ")
        })
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
        let path = target
            .split('#')
            .next()
            .unwrap_or_default()
            .trim_start_matches("../");
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
    let mut open = false;
    for line in md.lines() {
        if let Some(lang) = fence_marker(line.trim_start()) {
            if !open && SHELL_LANGS.contains(&lang) {
                out.push(lang.to_string());
            }
            open = !open;
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
        for (at, word) in words(&without_link_targets(&paragraph)) {
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

/// `text` with the target of every link, `](…)`, replaced by spaces of
/// the same length: a path such as `workflow.md` is not prose. So is the
/// text of a link to another chapter, which names that chapter as the
/// table of contents does ("The workflow"); a link to the glossary keeps
/// its text, the term it explains.
fn without_link_targets(text: &str) -> String {
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
/// prose and headings, outside code, `<details>` blocks and parentheses;
/// "change" counts only as a noun, after a determiner.
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
    ];
    let mut found: Vec<String> = Vec::new();
    let add = |w: &str, found: &mut Vec<String>| {
        if !found.iter().any(|f| f == w) {
            found.push(w.to_string());
        }
    };
    // Headings count: a French heading must be French too.
    for paragraph in prose_paragraphs(md, Headings::Keep) {
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
            if (word == "change" || word == "changes")
                && i > 0
                && DETERMINERS.contains(&words[i - 1].as_str())
            {
                add(word, &mut found);
            }
            if word == "pull" && words.get(i + 1).is_some_and(|n| n.starts_with("request")) {
                add("pull request", &mut found);
            }
        }
    }
    found
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
                let words = prose_words(chapter(lang, path));
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

    /// Each chapter's path and title, as the table of contents shows it in
    /// the sidebar: the name a reader sees and looks for.
    fn chapter_titles(lang: Lang) -> Vec<(String, String)> {
        summary(lang)
            .lines()
            .filter_map(|l| {
                let l = l.trim_start().strip_prefix("- [")?;
                let (title, rest) = l.split_once("](")?;
                Some((rest.trim_end_matches(')').to_string(), title.to_string()))
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
                let path = target
                    .split('#')
                    .next()
                    .unwrap_or_default()
                    .trim_start_matches("../");
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
            (Lang::En, ["proposal", "specs", "design", "tasks"]),
            (
                Lang::Fr,
                ["proposition", "spécification", "conception", "tâches"],
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
    const EVERYDAY: [&str; 14] = [
        "introduction.md",
        "choose-your-path.md",
        "paths/review.md",
        "paths/build.md",
        "paths/develop.md",
        "paths/team.md",
        "glossary.md",
        "quickstart.md",
        "installation.md",
        "workflow.md",
        "skills.md",
        "guides/review-a-plan.md",
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
    fn the_french_everyday_pages_hold_no_anglicism() {
        let mut found = Vec::new();
        for path in EVERYDAY {
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
}
