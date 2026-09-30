//! `codev docs`: the embedded documentation, rendered as standalone HTML.
//!
//! Three forms:
//! - default: writes to `temp_dir()/codev-docs-<version>.html` and opens
//!   it in the system browser;
//! - `--write <PATH>`: writes to the given path, does not open;
//! - `--print`: prints the markdown source to stdout.
//!
//! The HTML is **standalone** — inline CSS, no CDN dependency, no required
//! JS, viewable offline. In line with the codev philosophy: the binary
//! embeds what it needs to work without assuming network access at
//! runtime.

use std::io;
use std::path::{Path, PathBuf};

/// The markdown source of the docs. Embedded at build time via
/// `include_str!`: the docs always match the binary's version, with no
/// dependency on an external file at runtime.
pub const MARKDOWN_SOURCE: &str = include_str!("../../../docs/codev.md");

/// The embedded CSS, injected inline into every rendered HTML page.
const CSS: &str = include_str!("../assets/docs.css");

/// Converts the markdown source into standalone HTML, wrapped in the HTML5
/// template (doctype, `<head>`, inline CSS, version `<header>`).
pub fn render_html(md: &str, version: &str) -> String {
    use pulldown_cmark::{Options, Parser, html};

    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);

    let parser = Parser::new_ext(md, options);
    let mut body_html = String::new();
    html::push_html(&mut body_html, parser);

    format!(
        "<!doctype html>\n\
         <html lang=\"en\">\n\
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
        css = CSS,
        version = version,
        body_html = body_html,
    )
}

/// Default path to write the HTML to — in the system temp directory, with
/// the version in the name to tell apart several binaries installed side by
/// side.
pub fn default_output_path(version: &str) -> PathBuf {
    std::env::temp_dir().join(format!("codev-docs-{version}.html"))
}

/// Writes the HTML to the given path. The path's parent must exist — we do
/// not create a deep directory tree on the user's behalf.
pub fn write_to(path: &Path, version: &str) -> io::Result<()> {
    let html = render_html(MARKDOWN_SOURCE, version);
    std::fs::write(path, html)
}

/// Writes to `default_output_path(version)`, then delegates to
/// `open::that(...)` to open it in the system browser.
///
/// Returns the written path — the CLI shows it to the user so they know
/// where to find it if they want to share it again.
pub fn open_default(version: &str) -> io::Result<PathBuf> {
    let path = default_output_path(version);
    write_to(&path, version)?;
    open::that(&path)?;
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn markdown_source_is_not_empty() {
        // Tracer for an include_str! that would point to an empty file.
        assert!(
            MARKDOWN_SOURCE.len() > 1000,
            "MARKDOWN_SOURCE is only {} bytes — the docs look empty",
            MARKDOWN_SOURCE.len()
        );
        assert!(
            MARKDOWN_SOURCE.starts_with("# codev"),
            "MARKDOWN_SOURCE must start with the title"
        );
    }

    #[test]
    fn render_html_produces_standalone_html5() {
        let out = render_html("# Title\n\nParagraph.", "9.9.9");
        assert!(out.starts_with("<!doctype html>"));
        assert!(out.contains("<html lang=\"en\">"));
        assert!(out.contains("codev"));
        assert!(out.contains("v9.9.9"));
        assert!(out.contains("<h1>Title</h1>"));
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
    fn render_html_handles_tables_and_code() {
        let md = "\
| A | B |\n\
|---|---|\n\
| 1 | 2 |\n\
\n\
```\n\
let x = 1;\n\
```\n";
        let out = render_html(md, "0.1.0");
        assert!(out.contains("<table>"));
        assert!(out.contains("<code>"));
    }

    #[test]
    fn write_to_writes_the_html_file() {
        let dir = std::env::temp_dir().join("codev-docs-test-write");
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("docs.html");
        write_to(&path, "0.1.0").unwrap();
        let content = std::fs::read_to_string(&path).unwrap();
        assert!(content.starts_with("<!doctype html>"));
        assert!(content.contains("codev"));
        // Cleanup.
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn default_output_path_carries_the_version_in_its_name() {
        let p = default_output_path("1.2.3");
        let name = p.file_name().unwrap().to_string_lossy();
        assert_eq!(name, "codev-docs-1.2.3.html");
    }
}
