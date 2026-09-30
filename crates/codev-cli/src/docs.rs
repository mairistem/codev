//! `codev docs`: the embedded documentation, rendered as standalone HTML.
//!
//! Three forms:
//! - default: writes to `temp_dir()/codev-docs-<version>.html` and opens
//!   it in the system browser;
//! - `--write <PATH>`: writes to the given path, does not open;
//! - `--print`: prints the markdown source to stdout.
//!
//! Each form takes `--lang en|fr`. The chapters of a language are the ones
//! its `docs/<lang>/src/SUMMARY.md` lists — the same source the website is
//! built from — concatenated in order into a single page (see `build.rs`).
//!
//! The HTML is **standalone** — inline CSS, no CDN dependency, no required
//! JS, viewable offline. In line with the codev philosophy: the binary
//! embeds what it needs to work without assuming network access at
//! runtime.

use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};

/// The English chapters, as `(path relative to docs/en/src, markdown)`.
const CHAPTERS_EN: &[(&str, &str)] = include!(concat!(env!("OUT_DIR"), "/docs_en.rs"));

/// The French chapters, same shape as [`CHAPTERS_EN`].
const CHAPTERS_FR: &[(&str, &str)] = include!(concat!(env!("OUT_DIR"), "/docs_fr.rs"));

/// The embedded CSS, injected inline into every rendered HTML page.
const CSS: &str = include_str!("../assets/docs.css");

/// A documentation language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Lang {
    #[default]
    En,
    Fr,
}

impl Lang {
    /// The BCP 47 tag, used for `<html lang>` and file names.
    pub fn tag(self) -> &'static str {
        match self {
            Lang::En => "en",
            Lang::Fr => "fr",
        }
    }

    fn chapters(self) -> &'static [(&'static str, &'static str)] {
        match self {
            Lang::En => CHAPTERS_EN,
            Lang::Fr => CHAPTERS_FR,
        }
    }
}

/// The whole documentation of a language as one markdown document.
///
/// Links between chapters (`concepts.md#delta`, `../faq.md`) are rewritten
/// to in-page anchors, so they keep working once everything sits on a
/// single page.
pub fn markdown(lang: Lang) -> String {
    let chapters = lang.chapters();

    // Anchor of each chapter's title, to rewrite links to a whole chapter.
    // Every heading goes through the slugger, in page order, exactly as
    // `render_html` will do: repeated titles get the same `-N` suffixes.
    let mut slugger = Slugger::default();
    let mut chapter_anchors: HashMap<&str, String> = HashMap::new();
    for (path, source) in chapters {
        for (index, heading) in heading_texts(source).into_iter().enumerate() {
            let slug = slugger.slug(&heading);
            if index == 0 {
                chapter_anchors.insert(path, slug);
            }
        }
    }

    let mut out = String::new();
    for (path, source) in chapters {
        if !out.is_empty() {
            out.push_str("\n\n");
        }
        out.push_str(&rewrite_links(source, path, &chapter_anchors));
    }
    out
}

/// Converts markdown into standalone HTML, wrapped in the HTML5 template
/// (doctype, `<head>`, inline CSS, version `<header>`).
///
/// Headings get `id` attributes computed like mdBook's, so the anchors that
/// work on the website work in the single-page HTML too.
pub fn render_html(md: &str, lang: Lang, version: &str) -> String {
    use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd, html};

    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);

    // Collect events so each heading can receive the id computed from its
    // full text, which is only known once its end is reached.
    let mut events: Vec<Event> = Parser::new_ext(md, options).collect();
    let mut slugger = Slugger::default();
    let mut i = 0;
    while i < events.len() {
        if let Event::Start(Tag::Heading { .. }) = &events[i] {
            let mut text = String::new();
            let mut j = i + 1;
            while j < events.len() && !matches!(events[j], Event::End(TagEnd::Heading(_))) {
                if let Event::Text(t) | Event::Code(t) = &events[j] {
                    text.push_str(t);
                }
                j += 1;
            }
            let slug = slugger.slug(&text);
            if let Event::Start(Tag::Heading { id, .. }) = &mut events[i] {
                *id = Some(slug.into());
            }
            i = j;
        }
        i += 1;
    }

    let mut body_html = String::new();
    html::push_html(&mut body_html, events.into_iter());
    let body_html = mark_warning_callouts(&body_html);

    format!(
        "<!doctype html>\n\
         <html lang=\"{lang}\">\n\
         <head>\n\
         <meta charset=\"utf-8\">\n\
         <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n\
         <title>codev — Documentation</title>\n\
         <style>\n{css}\n</style>\n\
         </head>\n\
         <body>\n\
         <header>\n\
         <span class=\"title\">codev</span>\n\
         <span class=\"version\">v{version}</span>\n\
         </header>\n\
         <main>\n\
         {body_html}\n\
         </main>\n\
         </body>\n\
         </html>\n",
        lang = lang.tag(),
        css = CSS,
        version = version,
        body_html = body_html,
    )
}

/// Titles that turn a blockquote into a warning callout, in both languages —
/// the same set the website's theme recognizes.
const WARNING_TITLES: &[&str] = &["Warning", "Attention"];

/// Gives `class="warning"` to the blockquotes that open on a warning title
/// (`> **Warning**`, `> **Attention**`), so the stylesheet can tell them apart
/// from notes. Other blockquotes are left as they are.
fn mark_warning_callouts(html: &str) -> String {
    let mut out = html.to_string();
    for title in WARNING_TITLES {
        out = out.replace(
            &format!("<blockquote>\n<p><strong>{title}</strong>"),
            &format!("<blockquote class=\"warning\">\n<p><strong>{title}</strong>"),
        );
    }
    out
}

/// Default path to write the HTML to — in the system temp directory, with
/// the version in the name to tell apart several binaries installed side by
/// side. Non-English pages also carry their language.
pub fn default_output_path(lang: Lang, version: &str) -> PathBuf {
    let name = match lang {
        Lang::En => format!("codev-docs-{version}.html"),
        other => format!("codev-docs-{version}-{}.html", other.tag()),
    };
    std::env::temp_dir().join(name)
}

/// Writes the HTML to the given path. The path's parent must exist — we do
/// not create a deep directory tree on the user's behalf.
pub fn write_to(path: &Path, lang: Lang, version: &str) -> io::Result<()> {
    let html = render_html(&markdown(lang), lang, version);
    std::fs::write(path, html)
}

/// Writes to `default_output_path(lang, version)`, then delegates to
/// `open::that(...)` to open it in the system browser.
///
/// Returns the written path — the CLI shows it to the user so they know
/// where to find it if they want to share it again.
pub fn open_default(lang: Lang, version: &str) -> io::Result<PathBuf> {
    let path = default_output_path(lang, version);
    write_to(&path, lang, version)?;
    open::that(&path)?;
    Ok(path)
}

/// The text of every ATX heading of a chapter, outside code fences, with
/// inline code and emphasis markers removed — the text the HTML renderer
/// sees.
fn heading_texts(source: &str) -> Vec<String> {
    let mut in_fence = false;
    let mut out = Vec::new();
    for line in source.lines() {
        if line.trim_start().starts_with("```") {
            in_fence = !in_fence;
            continue;
        }
        if in_fence {
            continue;
        }
        let hashes = line.bytes().take_while(|&b| b == b'#').count();
        if (1..=6).contains(&hashes) && line[hashes..].starts_with(' ') {
            out.push(line[hashes..].replace(['`', '*'], "").trim().to_string());
        }
    }
    out
}

/// Rewrites the inline links of one chapter that point to another chapter.
///
/// `other.md#anchor` becomes `#anchor`; `other.md` becomes the anchor of
/// that chapter's title. Paths are resolved relative to the current
/// chapter, like mdBook does. External links and plain anchors are left
/// alone.
fn rewrite_links(source: &str, chapter: &str, anchors: &HashMap<&str, String>) -> String {
    let base = Path::new(chapter).parent().unwrap_or(Path::new(""));
    let mut out = String::with_capacity(source.len());
    let mut rest = source;
    while let Some(start) = rest.find("](") {
        out.push_str(&rest[..start + 2]);
        rest = &rest[start + 2..];
        let Some(end) = rest.find(')') else { break };
        let target = &rest[..end];
        out.push_str(&rewrite_target(target, base, anchors));
        rest = &rest[end..];
    }
    out.push_str(rest);
    out
}

fn rewrite_target(target: &str, base: &Path, anchors: &HashMap<&str, String>) -> String {
    if target.contains("://") || target.starts_with('#') || target.starts_with("mailto:") {
        return target.to_string();
    }
    let (path, fragment) = match target.split_once('#') {
        Some((p, f)) => (p, Some(f)),
        None => (target, None),
    };
    if !path.ends_with(".md") {
        return target.to_string();
    }
    let resolved = normalize(&base.join(path));
    match (fragment, anchors.get(resolved.as_str())) {
        (Some(f), _) => format!("#{f}"),
        (None, Some(anchor)) => format!("#{anchor}"),
        // A link to a file outside the book (e.g. the repository README):
        // there is no page to point to, keep it as written.
        (None, None) => target.to_string(),
    }
}

/// Resolves `.` and `..` components without touching the file system.
fn normalize(path: &Path) -> String {
    let mut parts: Vec<&str> = Vec::new();
    for component in path.to_str().unwrap_or_default().split(['/', '\\']) {
        match component {
            "" | "." => {}
            ".." => {
                parts.pop();
            }
            other => parts.push(other),
        }
    }
    parts.join("/")
}

/// Heading ids in the style of mdBook: lowercase, alphanumerics kept,
/// spaces turned into `-`, other punctuation dropped, and a `-N` suffix on
/// repeats.
#[derive(Default)]
struct Slugger {
    seen: HashMap<String, usize>,
}

impl Slugger {
    fn slug(&mut self, text: &str) -> String {
        let base: String = text
            .trim()
            .chars()
            .filter_map(|c| {
                if c.is_alphanumeric() || c == '_' || c == '-' {
                    Some(c.to_lowercase().next().unwrap_or(c))
                } else if c.is_whitespace() {
                    Some('-')
                } else {
                    None
                }
            })
            .collect();
        let count = self.seen.entry(base.clone()).or_insert(0);
        let slug = if *count == 0 {
            base
        } else {
            format!("{base}-{count}")
        };
        *count += 1;
        slug
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_language_embeds_a_non_empty_manual_starting_with_the_title() {
        for lang in [Lang::En, Lang::Fr] {
            let md = markdown(lang);
            assert!(
                md.len() > 1000,
                "{} docs are only {} bytes — they look empty",
                lang.tag(),
                md.len()
            );
            assert!(
                md.starts_with("# codev"),
                "{} docs must start with the `# codev` title",
                lang.tag()
            );
        }
    }

    #[test]
    fn both_languages_have_the_same_chapters() {
        let en: Vec<&str> = CHAPTERS_EN.iter().map(|(p, _)| *p).collect();
        let fr: Vec<&str> = CHAPTERS_FR.iter().map(|(p, _)| *p).collect();
        assert_eq!(en, fr, "docs/en and docs/fr must list the same chapters");
    }

    #[test]
    fn every_chapter_has_exactly_one_top_level_title() {
        for (lang, chapters) in [("en", CHAPTERS_EN), ("fr", CHAPTERS_FR)] {
            for (path, source) in chapters {
                let mut in_fence = false;
                let mut titles = 0;
                for line in source.lines() {
                    if line.trim_start().starts_with("```") {
                        in_fence = !in_fence;
                    } else if !in_fence && line.starts_with("# ") {
                        titles += 1;
                    }
                }
                assert_eq!(titles, 1, "{lang}/{path} must have exactly one `# ` title");
            }
        }
    }

    #[test]
    fn links_between_chapters_become_in_page_anchors() {
        let mut anchors = HashMap::new();
        anchors.insert("concepts.md", "concepts".to_string());
        anchors.insert("reference/cli.md", "cli-reference".to_string());
        let source = "See [deltas](concepts.md#delta), [the CLI](../reference/cli.md), \
                      [the site](https://example.com/a.md) and [above](#top).";
        let out = rewrite_links(source, "guides/mcp.md", &anchors);
        assert_eq!(
            out,
            "See [deltas](#delta), [the CLI](#cli-reference), \
             [the site](https://example.com/a.md) and [above](#top)."
        );
    }

    #[test]
    fn slugs_follow_mdbook_and_deduplicate() {
        let mut s = Slugger::default();
        assert_eq!(s.slug("Getting started"), "getting-started");
        assert_eq!(
            s.slug("`codev init` — what it does"),
            "codev-init--what-it-does"
        );
        assert_eq!(s.slug("Getting started"), "getting-started-1");
    }

    #[test]
    fn render_html_produces_standalone_html5() {
        let out = render_html("# Title\n\nParagraph.", Lang::En, "9.9.9");
        assert!(out.starts_with("<!doctype html>"));
        assert!(out.contains("<html lang=\"en\">"));
        assert!(out.contains("codev"));
        assert!(out.contains("v9.9.9"));
        assert!(out.contains("<h1 id=\"title\">Title</h1>"));
        assert!(out.contains("<p>Paragraph.</p>"));
        // Standalone: no reference to an external URL (http://, https://).
        // URLs in the rendered docs body are still tolerated — the test
        // specifically targets `<link>` and `<script src>` tags.
        assert!(
            !out.contains("<link href=\"http"),
            "no external <link> expected (standalone HTML)"
        );
        assert!(
            !out.contains("<script src=\"http"),
            "no external <script src> expected"
        );
    }

    #[test]
    fn warning_callouts_get_their_own_class() {
        let md =
            "> **Warning**\n> Careful.\n\n> **Attention**\n> Prudence.\n\n> **Note**\n> Fine.\n";
        let out = render_html(md, Lang::En, "0.1.0");
        assert_eq!(
            out.matches("<blockquote class=\"warning\">").count(),
            2,
            "{out}"
        );
        assert_eq!(
            out.matches("<blockquote>").count(),
            1,
            "the note stays a plain blockquote"
        );
    }

    #[test]
    fn render_html_marks_the_page_language() {
        let out = render_html("# Titre", Lang::Fr, "0.1.0");
        assert!(out.contains("<html lang=\"fr\">"));
    }

    #[test]
    fn render_html_handles_tables_and_code() {
        let md = "\
| A | B |\n\
|---|---|\n\
| 1 | 2 |\n\
\n\
```\n\
let x = 1;\n\
```\n";
        let out = render_html(md, Lang::En, "0.1.0");
        assert!(out.contains("<table>"));
        assert!(out.contains("<code>"));
    }

    #[test]
    fn write_to_writes_the_html_file() {
        let dir = std::env::temp_dir().join("codev-docs-test-write");
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("docs.html");
        write_to(&path, Lang::En, "0.1.0").unwrap();
        let content = std::fs::read_to_string(&path).unwrap();
        assert!(content.starts_with("<!doctype html>"));
        assert!(content.contains("codev"));
        // Cleanup.
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn default_output_path_carries_the_version_in_its_name() {
        let p = default_output_path(Lang::En, "1.2.3");
        let name = p.file_name().unwrap().to_string_lossy();
        assert_eq!(name, "codev-docs-1.2.3.html");
        let p = default_output_path(Lang::Fr, "1.2.3");
        let name = p.file_name().unwrap().to_string_lossy();
        assert_eq!(name, "codev-docs-1.2.3-fr.html");
    }
}
