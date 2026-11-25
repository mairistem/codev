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

/// Walks a chapter, line by line, keeping track of fences and `<details>`.
/// Fence lines and `<details>`/`</details>` lines themselves are skipped.
fn lines(md: &str) -> Vec<Line<'_>> {
    let mut out = Vec::new();
    let mut fence: Option<&str> = None;
    let mut details = 0usize;
    for text in md.lines() {
        let trimmed = text.trim_start();
        if let Some(rest) = trimmed
            .strip_prefix("```")
            .or_else(|| trimmed.strip_prefix("~~~"))
        {
            fence = match fence {
                Some(_) => None,
                None => Some(rest.trim()),
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

/// The chapters of the Start here path, in order.
pub const START_HERE: [&str; 4] = [
    "introduction.md",
    "choose-your-path.md",
    "quickstart.md",
    "glossary.md",
];

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

/// The `## ` sections of a chapter: heading and body.
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

/// The local chapter paths a text links to, anchors removed.
fn chapter_links(md: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = md;
    while let Some(i) = rest.find("](") {
        rest = &rest[i + 2..];
        let Some(end) = rest.find(')') else { break };
        let target = &rest[..end];
        if !target.contains("://") {
            let path = target.split('#').next().unwrap_or(target);
            if path.ends_with(".md") {
                out.push(path.trim_start_matches("./").to_string());
            }
        }
        rest = &rest[end..];
    }
    out
}

/// What is wrong with Choose your path: not five profile sections with
/// their expected headings in order, a profile with fewer than two links,
/// or a link outside `chapters`.
pub fn profile_problems(md: &str, headings: &[&str; 5], chapters: &[&str]) -> Vec<String> {
    let sections = sections(md, "## ");
    let mut problems = Vec::new();
    let found: Vec<&str> = sections.iter().map(|(h, _)| h.as_str()).collect();
    if found != headings {
        problems.push(format!("profiles {found:?}, expected {headings:?}"));
    }
    for (heading, body) in &sections {
        let links = chapter_links(body);
        if links.len() < 2 {
            problems.push(format!("{heading}: {} link(s)", links.len()));
        }
        for link in links {
            if !chapters.contains(&link.as_str()) {
                problems.push(format!("{heading}: {link} is not a chapter"));
            }
        }
    }
    problems
}

/// The shell languages of a chapter's fenced blocks.
pub fn shell_blocks(md: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut open = false;
    for line in md.lines() {
        let t = line.trim_start();
        if let Some(lang) = t.strip_prefix("```") {
            if !open && SHELL_LANGS.contains(&lang.trim()) {
                out.push(lang.trim().to_string());
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

/// The headings of the profile sections, per edition.
pub fn profile_headings(lang: Lang) -> [&'static str; 5] {
    match lang {
        Lang::En => [
            "I am discovering AI agents",
            "I do not code, I read and approve plans",
            "I develop with Compagnon",
            "I set Compagnon up for my team",
            "I contribute to Compagnon",
        ],
        Lang::Fr => [
            "Je découvre les agents IA",
            "Je ne code pas, je relis et valide des plans",
            "Je développe avec Compagnon",
            "Je configure Compagnon pour mon équipe",
            "Je contribue à Compagnon",
        ],
    }
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
        ],
        Lang::Fr => &[
            "Agent",
            "Archive",
            "Capacité",
            "Change",
            "Claude Code",
            "Conception",
            "Décision",
            "Delta",
            "Exigence",
            "MCP",
            "Plan",
            "Proposition",
            "Revue",
            "Rôle",
            "Scénario",
            "Skill",
            "Spec",
            "Tâches",
        ],
    }
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

    const SUMMARY: &str = "# Summary\n\n# Start here\n\n- [A](introduction.md)\n- [B](choose-your-path.md)\n- [C](quickstart.md)\n- [D](glossary.md)\n\n# Everyday use\n\n- [E](concepts.md)\n";

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

    #[test]
    fn a_link_outside_the_table_of_contents_is_caught() {
        let headings = ["A", "B", "C", "D", "E"];
        let mut md = String::from("# Choose\n");
        for h in headings {
            md.push_str(&format!(
                "\n## {h}\n\n1. [x](quickstart.md)\n2. [y](glossary.md#plan)\n"
            ));
        }
        assert!(profile_problems(&md, &headings, &["quickstart.md", "glossary.md"]).is_empty());
        let broken = md.replacen("glossary.md#plan", "guides/missing.md", 1);
        assert_eq!(
            profile_problems(&broken, &headings, &["quickstart.md", "glossary.md"]),
            ["A: guides/missing.md is not a chapter"]
        );
    }

    #[test]
    fn a_shell_block_in_review_a_plan_is_caught() {
        assert_eq!(
            shell_blocks("```markdown\n## Why\n```\n\n```bash\nls\n```\n"),
            ["bash"]
        );
        assert!(shell_blocks("```text\nWHEN the user…\n```\n").is_empty());
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
            for c in ["origins.md", "architecture.md", "reliability.md"] {
                assert!(
                    parts[4].1.iter().any(|p| p == c),
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
                ("choose-your-path.md", (400, 500)),
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
            assert!(
                p + 1 < a && a < r,
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

    #[test]
    fn choose_your_path_links_five_profiles_to_chapters() {
        for lang in LANGS {
            let problems = profile_problems(
                chapter(lang, "choose-your-path.md"),
                &profile_headings(lang),
                &paths(lang),
            );
            assert!(problems.is_empty(), "{}: {problems:?}", lang.tag());
        }
    }

    #[test]
    fn review_a_plan_covers_the_plan_without_a_terminal() {
        for (lang, words) in [
            (Lang::En, ["proposal", "specs", "design", "tasks"]),
            (Lang::Fr, ["proposition", "specs", "conception", "tâches"]),
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
            assert_eq!(&paths(lang)[..4], START_HERE, "{}", lang.tag());
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
}
