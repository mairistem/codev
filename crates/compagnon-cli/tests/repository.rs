//! Checks on the repository's own files: what the product ships besides
//! its code — sources, workflows, installers, documentation — read from
//! the checkout, never written.

use std::fs;
use std::path::{Path, PathBuf};

use compagnon_core::legacy;

fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("crates/compagnon-cli sits two levels below the root")
        .to_path_buf()
}

fn read(relative: &str) -> String {
    fs::read_to_string(repo().join(relative)).unwrap_or_else(|e| panic!("{relative}: {e}"))
}

/// Every `.rs` file under `dir`, as the paths `read_dir` gives.
fn rust_sources(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).expect("a readable directory") {
        let path = entry.expect("an entry").path();
        if path.is_dir() {
            rust_sources(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
}

/// The double-quoted string literals of a Rust source that look like a
/// JSON code: lowercase words joined by `_`.
fn code_like_literals(source: &str) -> Vec<String> {
    source
        .split('"')
        .skip(1)
        .step_by(2)
        .filter(|s| {
            s.contains('_')
                && !s.starts_with('_')
                && s.chars().all(|c| c.is_ascii_lowercase() || c == '_')
        })
        .map(str::to_string)
        .collect()
}

#[test]
fn no_json_code_contains_the_former_name() {
    let mut files = Vec::new();
    rust_sources(&repo().join("crates"), &mut files);
    assert!(files.len() > 20, "the sources were found: {}", files.len());
    let mut offenders = Vec::new();
    for file in files {
        if file.ends_with("legacy.rs") {
            continue;
        }
        let source = fs::read_to_string(&file).expect("a readable source");
        for literal in code_like_literals(&source) {
            if literal.contains(legacy::BINARY_STEM) {
                offenders.push(format!("{}: {literal}", file.display()));
            }
        }
    }
    assert!(
        offenders.is_empty(),
        "codes naming the former product: {offenders:#?}"
    );
}

#[test]
fn code_like_literals_finds_snake_case_codes() {
    let source = r#"let a = "no_compagnon_root"; let b = "a message, not a code";"#;
    assert_eq!(code_like_literals(source), ["no_compagnon_root"]);
}

#[test]
fn the_contract_lists_the_renamed_codes() {
    let contract = read("crates/compagnon-cli/src/contract.rs");
    for code in [
        "no_compagnon_root",
        "compagnon_version_unsatisfied",
        "legacy_root",
        "legacy_source",
    ] {
        assert!(
            contract.contains(code),
            "contract.rs does not list `{code}`"
        );
    }
}

/// The release workflow, whitespace collapsed to single spaces.
fn release_workflow() -> String {
    read(".github/workflows/release.yml")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

#[test]
fn release_workflow_publishes_legacy_archives_only_for_0_7() {
    let flat = release_workflow();
    let former = legacy::ASSET_PREFIX;
    let stem = legacy::BINARY_STEM;
    assert_eq!(
        flat.matches("startsWith(github.ref_name, 'v0.7.')").count(),
        2,
        "both legacy packaging steps are limited to the 0.7 series"
    );
    for needle in [
        format!("legacy=\"{former}-${{version}}-${{target}}\""),
        format!("\"${{legacy}}/{stem}\""),
        format!("$legacy = \"{former}-$version-$target\""),
        format!("\"$legacy/{stem}.exe\""),
        format!(
            "{former}-${{{{ steps.version.outputs.version }}}}-${{{{ matrix.target }}}}.${{{{ matrix.ext }}}}"
        ),
        // Every archive, legacy ones included, is listed in the sums.
        "shasum -a 256 *.tar.gz *.zip > SHA256SUMS".to_string(),
    ] {
        assert!(
            flat.contains(&needle),
            "release.yml must contain `{needle}`"
        );
    }
}

#[test]
fn release_workflow_stages_compagnon_exe_with_the_documents_on_windows() {
    let flat = release_workflow();
    for needle in [
        "$stage = \"compagnon-$version-$target\"",
        "Copy-Item \"target/$target/release/compagnon.exe\" $stage",
        "Copy-Item README.md, README.fr.md, LICENSE, CHANGELOG.md $stage",
        "Compress-Archive -Path $stage -DestinationPath \"$stage.zip\"",
        "cargo build --release --bin compagnon",
    ] {
        assert!(flat.contains(needle), "release.yml must contain `{needle}`");
    }
}

#[test]
fn release_workflow_runs_only_on_version_tags() {
    let flat = release_workflow();
    assert!(
        flat.contains("on: push: tags: - \"v*.*.*\" permissions:"),
        "the only trigger is a version tag"
    );
    assert!(!flat.contains("branches:"), "no branch triggers a release");
}

#[test]
fn installers_honor_a_codev_pin() {
    let former = legacy::ASSET_PREFIX;
    let sh = read("install.sh");
    for needle in [
        "REPO=\"mairistem/compagnon\"".to_string(),
        "version=\"${COMPAGNON_VERSION:-}\"".to_string(),
        format!("version=\"${{{}}}\"", legacy::VERSION_ENV),
        format!("BIN_NAME=\"{}\"", legacy::BINARY_STEM),
        format!("0.[0-6].*) release_name=\"{former}\""),
        "archive=\"${release_name}-${version}-${target}.tar.gz\"".to_string(),
    ] {
        assert!(sh.contains(&needle), "install.sh must contain `{needle}`");
    }
    let ps1 = read("install.ps1");
    for needle in [
        "$repo = 'mairistem/compagnon'".to_string(),
        "$version = $env:COMPAGNON_VERSION".to_string(),
        format!("$env:{}", legacy::VERSION_ENV),
        "if ($version -match '^0\\.[0-6]\\.')".to_string(),
        format!("$releaseName = '{former}'"),
        "$archive     = \"$releaseName-$version-$target.zip\"".to_string(),
    ] {
        assert!(ps1.contains(&needle), "install.ps1 must contain `{needle}`");
    }
    assert!(
        ps1.is_ascii(),
        "install.ps1 stays pure ASCII for PowerShell 5.1"
    );
}

/// The colors an SVG paints with: every `fill="#…"` and `stroke="#…"`.
fn painted_colors(svg: &str) -> Vec<String> {
    ["fill=\"#", "stroke=\"#"]
        .iter()
        .flat_map(|attr| {
            svg.match_indices(attr).map(move |(i, _)| {
                svg[i + attr.len() - 1..]
                    .split('"')
                    .next()
                    .unwrap_or("")
                    .to_lowercase()
            })
        })
        .collect()
}

const MARKS: [&str; 5] = [
    "docs/assets/compagnon-mark.svg",
    "docs/assets/compagnon-mark-dark.svg",
    "docs/assets/compagnon-logo.svg",
    "docs/assets/compagnon-logo-dark.svg",
    "docs/theme/favicon.svg",
];

#[test]
fn compagnon_marks_use_only_the_mairistem_palette() {
    let palette = [
        "#ff5331", "#ffab03", "#004c66", "#00c39b", "#000000", "#ffffff",
    ];
    for file in MARKS {
        let svg = read(file);
        let colors = painted_colors(&svg);
        assert!(!colors.is_empty(), "{file} paints nothing");
        for color in colors {
            assert!(
                palette.contains(&color.as_str()),
                "{file} uses {color}, outside the palette"
            );
        }
        // The logos carry the byline, the marks the name alone.
        let title = if file.contains("-logo") {
            "<title id=\"t\">Compagnon by Mairistem</title>"
        } else {
            "<title id=\"t\">Compagnon</title>"
        };
        assert!(svg.contains(title), "{file} must hold `{title}`");
    }
}

#[test]
fn a_mark_outside_the_palette_is_rejected() {
    let colors = painted_colors(r##"<svg><circle fill="#123456"/><path stroke="#FF5331"/></svg>"##);
    assert_eq!(colors, ["#123456", "#ff5331"]);
}

#[test]
fn both_readmes_show_the_light_and_dark_mark() {
    for readme in ["README.md", "README.fr.md"] {
        let text = read(readme);
        for needle in [
            "srcset=\"docs/assets/compagnon-logo-dark.svg\"",
            "src=\"docs/assets/compagnon-logo.svg\" alt=\"Compagnon by Mairistem\"",
        ] {
            assert!(text.contains(needle), "{readme} must contain `{needle}`");
        }
    }
}

/// Every inline use of the mark in the theme and on the site: either named
/// (`role="img"` with a label) or decorative (`aria-hidden="true"`), never
/// silent noise for a screen reader.
#[test]
fn inline_marks_are_named_or_decorative() {
    for file in [
        "docs/theme/index.hbs",
        "docs/site/index.html",
        "docs/site/fr.html",
    ] {
        let text = read(file);
        let uses: Vec<&str> = text
            .match_indices("<use href=\"#cp-")
            .filter(|(i, _)| {
                text[*i..].starts_with("<use href=\"#cp-mark\">")
                    || text[*i..].starts_with("<use href=\"#cp-logo\">")
            })
            .map(|(i, _)| {
                let start = text[..i].rfind("<svg").expect("inside an svg");
                &text[start..i]
            })
            .collect();
        assert!(
            !uses.is_empty(),
            "{file} shows neither the mark nor the logo"
        );
        for svg in uses {
            let named = svg.contains("role=\"img\"") && svg.contains("aria-label=\"Compagnon");
            let decorative = svg.contains("aria-hidden=\"true\"");
            assert!(
                named || decorative,
                "{file}: a mark is neither named nor decorative: {svg}"
            );
        }
    }
    let theme = read("docs/theme/index.hbs");
    assert!(
        theme.contains("Compagnon, by Mairistem — home"),
        "the English brand link names Compagnon"
    );
    assert!(
        theme.contains("Accueil du site Compagnon, par Mairistem"),
        "the French brand link names Compagnon"
    );
    let site = read("docs/site/index.html");
    assert!(
        site.contains("aria-label=\"Compagnon, by Mairistem — home\""),
        "the site's brand link names Compagnon"
    );
    let french = read("docs/site/fr.html");
    assert!(
        french.contains("aria-label=\"Accueil du site Compagnon, par Mairistem\""),
        "the French home page's brand link names Compagnon"
    );
}

#[test]
fn both_editions_list_the_migration_guide() {
    let guide = format!("guides/moving-from-{}.md", legacy::BINARY_STEM);
    for lang in ["en", "fr"] {
        let summary = read(&format!("docs/{lang}/src/SUMMARY.md"));
        assert!(
            summary.contains(&format!("]({guide})")),
            "docs/{lang} does not list {guide}"
        );
    }
}

#[test]
fn migration_guides_name_compagnon_migrate_dry_run() {
    for lang in ["en", "fr"] {
        let path = format!(
            "docs/{lang}/src/guides/moving-from-{}.md",
            legacy::BINARY_STEM
        );
        let guide = read(&path);
        for needle in [
            "compagnon migrate --dry-run",
            "compagnon sources update",
            "compagnon self-update --version 0.6.1",
            "migration_version_unsatisfied",
            "legacy_source",
        ] {
            assert!(guide.contains(needle), "{path} must mention `{needle}`");
        }
        let reference = read(&format!("docs/{lang}/src/reference/cli.md"));
        assert!(
            reference.contains("### compagnon migrate"),
            "docs/{lang} CLI reference lacks migrate"
        );
    }
}

#[test]
fn site_and_introductions_name_compagnon() {
    for file in ["docs/site/index.html", "docs/site/fr.html"] {
        let site = read(file);
        assert!(
            site.contains("<title>Compagnon — "),
            "{file}: the title names Compagnon"
        );
        assert!(
            site.contains("<meta name=\"description\" content=\"Compagnon "),
            "{file}: the description names Compagnon"
        );
    }
    for (lang, needle) in [("en", "compagnonnage"), ("fr", "compagnonnage")] {
        let intro = read(&format!("docs/{lang}/src/introduction.md"));
        assert!(
            intro.starts_with("# Compagnon "),
            "docs/{lang} introduction title"
        );
        assert!(
            intro.contains(needle),
            "docs/{lang} introduction explains the name"
        );
    }
}

/// The words of a README's prose: outside fenced code blocks (```` ``` ````
/// or `~~~`) and HTML lines (the mark's `<picture>`, the demo image).
/// Unlike the chapters' `prose_words`, which this integration test cannot
/// reach, every line starting with `<` is skipped: a README has no
/// `<details>` block, only HTML for its images.
fn readme_words(text: &str) -> usize {
    let mut fence = false;
    let mut words = 0;
    for line in text.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            fence = !fence;
            continue;
        }
        if fence || trimmed.starts_with('<') {
            continue;
        }
        words += line
            .split_whitespace()
            .filter(|w| w.chars().any(char::is_alphanumeric))
            .count();
    }
    words
}

#[test]
fn both_readmes_stay_short_keep_the_one_liners_and_link_the_profiles() {
    for (file, heading, cap) in [
        ("README.md", "## Getting started", 600),
        ("README.fr.md", "## Démarrer", 750),
    ] {
        let text = read(file);
        let words = readme_words(&text);
        assert!(words <= cap, "{file}: {words} words, cap {cap}");
        let start = text
            .find(heading)
            .unwrap_or_else(|| panic!("{file} has no `{heading}`"));
        let rest = &text[start + heading.len()..];
        let section = &rest[..rest.find("\n## ").unwrap_or(rest.len())];
        for needle in [
            "curl -sSL https://raw.githubusercontent.com/mairistem/compagnon/main/install.sh | sh",
            "iwr -useb https://raw.githubusercontent.com/mairistem/compagnon/main/install.ps1 | iex",
            "compagnon init",
            "/compagnon-propose",
        ] {
            assert!(
                section.contains(needle),
                "{file}: `{heading}` lacks `{needle}`"
            );
        }
        let lang = if file.ends_with(".fr.md") { "fr" } else { "en" };
        let problems = profile_problems(&text, &labels(lang), &path_pages(lang, ".html"));
        assert!(problems.is_empty(), "{file}: {problems:?}");
    }
}

/// The four profile labels of Choose your path, per edition — the same
/// words on every entry point (`docs_checks::profile_labels`, which this
/// integration test cannot reach).
fn labels(lang: &str) -> [&'static str; 4] {
    if lang == "fr" {
        [
            "Je relis et je valide des plans — je ne code pas",
            "Je ne code pas, mais je veux que Claude construise quelque chose pour moi",
            "Je suis développeur et je fais coder Claude",
            "J'installe Compagnon pour mon équipe",
        ]
    } else {
        [
            "I review and approve plans — I do not code",
            "I do not code, but I want Claude to build something for me",
            "I am a developer and I have Claude write code",
            "I set Compagnon up for my team",
        ]
    }
}

/// The path pages of an edition, as a link to them ends.
fn path_pages(lang: &str, extension: &str) -> [String; 4] {
    ["review", "build", "develop", "team"].map(|p| format!("{lang}/paths/{p}{extension}"))
}

/// Where the link labelled `label` points, in Markdown (`[label](…)`) or
/// HTML (`href="…">label</a>`), and where the label stands.
fn link_for(text: &str, label: &str) -> Option<(usize, String)> {
    if let Some(i) = text.find(&format!("[{label}](")) {
        let rest = &text[i + label.len() + 3..];
        return Some((i, rest[..rest.find(')')?].to_string()));
    }
    let i = text.find(&format!(">{label}</a>"))?;
    let start = text[..i].rfind("href=\"")? + 6;
    let end = start + text[start..].find('"')?;
    Some((i, text[start..end].to_string()))
}

/// What is wrong with the profile links of a page: each label, in order,
/// linking a target that ends with its path page.
/// It mirrors `docs_checks::profile_link_problems`, which this integration
/// test cannot reach, and also reads HTML links: change the two together.
fn profile_problems(text: &str, labels: &[&str; 4], pages: &[String; 4]) -> Vec<String> {
    let mut problems = Vec::new();
    let mut last = 0;
    for (label, page) in labels.iter().zip(pages) {
        match link_for(text, label) {
            None => problems.push(format!("no link \"{label}\"")),
            Some((at, target)) => {
                if !target.ends_with(page.as_str()) {
                    problems.push(format!("\"{label}\" links {target}, not {page}"));
                }
                if at < last {
                    problems.push(format!("\"{label}\" out of order"));
                }
                last = at;
            }
        }
    }
    problems
}

/// What is wrong with a home page: its profiles, the first one after the
/// install command, or no link to its edition's Origins for contributors.
fn home_page_problems(html: &str, lang: &str) -> Vec<String> {
    let labels = labels(lang);
    let mut problems = profile_problems(html, &labels, &path_pages(lang, ".html"));
    if let (Some((first, _)), Some(install)) = (link_for(html, labels[0]), html.find("curl -sSL"))
        && first > install
    {
        problems.push("the profiles come after the install command".into());
    }
    if !html.contains(&format!("href=\"{lang}/origins.html\"")) {
        problems.push(format!("no link to {lang}/origins.html"));
    }
    problems
}

#[test]
fn a_readme_missing_a_path_page_is_caught() {
    let labels = labels("en");
    let pages = path_pages("en", ".html");
    let mut text = String::new();
    for (label, page) in labels.iter().zip(&pages).take(3) {
        text.push_str(&format!("- [{label}](https://x/{page})\n"));
    }
    assert_eq!(
        profile_problems(&text, &labels, &pages),
        ["no link \"I set Compagnon up for my team\""]
    );
}

#[test]
fn a_home_page_that_asks_after_installing_is_caught() {
    let labels = labels("en");
    let mut html = String::from("<pre>curl -sSL https://x/install.sh | sh</pre>\n");
    for (label, page) in labels.iter().zip(path_pages("en", ".html")) {
        html.push_str(&format!("<a href=\"{page}\">{label}</a>\n"));
    }
    html.push_str("<a href=\"en/origins.html\">Origins</a>\n");
    assert_eq!(
        home_page_problems(&html, "en"),
        ["the profiles come after the install command"]
    );
}

#[test]
fn both_home_pages_lead_with_the_four_profiles() {
    for (file, lang) in [("docs/site/index.html", "en"), ("docs/site/fr.html", "fr")] {
        let html = read(file);
        let problems = home_page_problems(&html, lang);
        assert!(problems.is_empty(), "{file}: {problems:?}");
    }
    let english = read("docs/site/index.html");
    let french = read("docs/site/fr.html");
    assert!(
        french.contains("<html lang=\"fr\""),
        "fr.html is not declared French"
    );
    assert!(
        english.contains("href=\"fr.html\" hreflang=\"fr\" lang=\"fr\""),
        "index.html does not link fr.html in French"
    );
    assert!(
        french.contains("href=\"./\" hreflang=\"en\" lang=\"en\""),
        "fr.html does not link the English home page in English"
    );
    for page in [&english, &french] {
        for alternate in [
            "<link rel=\"alternate\" hreflang=\"en\" href=\"./\">",
            "<link rel=\"alternate\" hreflang=\"fr\" href=\"fr.html\">",
        ] {
            assert!(page.contains(alternate), "a home page lacks `{alternate}`");
        }
        assert_eq!(
            page.matches("rel=\"alternate\"").count(),
            2,
            "one alternate per language"
        );
    }
}

/// The header brand links of a page — the links whose opening tag holds
/// `marker` — that do not show the "Compagnon by Mairistem" logo, or still
/// show the Mairistem logo.
fn header_brand_problems(text: &str, marker: &str) -> Vec<String> {
    let mut problems = Vec::new();
    let mut found = 0;
    for (i, _) in text.match_indices(marker) {
        let start = text[..i].rfind("<a ").unwrap_or(i);
        let end = text[i..].find("</a>").map_or(text.len(), |e| i + e);
        let link = &text[start..end];
        found += 1;
        if !link.contains("<use href=\"#cp-logo\">") {
            problems.push(format!(
                "brand link {found} lacks the Compagnon by Mairistem logo"
            ));
        }
        if link.contains("#mx-logo") {
            problems.push(format!("brand link {found} still shows the Mairistem logo"));
        }
    }
    if found == 0 {
        problems.push("no brand link".into());
    }
    problems
}

#[test]
fn headers_show_the_compagnon_by_mairistem_logo() {
    for (file, marker, count) in [
        ("docs/theme/index.hbs", "class=\"mx-brand-logo\"", 2),
        ("docs/site/index.html", "class=\"brand\"", 1),
        ("docs/site/fr.html", "class=\"brand\"", 1),
    ] {
        let text = read(file);
        assert_eq!(text.matches(marker).count(), count, "{file}: brand links");
        let problems = header_brand_problems(&text, marker);
        assert!(problems.is_empty(), "{file}: {problems:?}");
    }
}

#[test]
fn a_header_still_showing_the_mairistem_logo_is_caught() {
    let page = "<a class=\"brand\" href=\"./\" aria-label=\"Compagnon, by Mairistem — home\">\
                <svg class=\"logo\"><use href=\"#mx-logo\"/></svg></a>";
    assert_eq!(
        header_brand_problems(page, "class=\"brand\""),
        [
            "brand link 1 lacks the Compagnon by Mairistem logo",
            "brand link 1 still shows the Mairistem logo"
        ]
    );
}

/// The home pages' copy buttons sit in each code block's header, beside the
/// command: the script must find the command from the block, not from the
/// button's parent, or a click does nothing and announces nothing.
#[test]
fn home_page_copy_buttons_find_their_command() {
    for file in ["docs/site/index.html", "docs/site/fr.html"] {
        let page = read(file);
        let buttons = page.matches("<button class=\"copy\"").count();
        assert_eq!(buttons, 2, "{file}: two copy buttons");
        for (i, _) in page.match_indices("<button class=\"copy\"") {
            let block_start = page[..i]
                .rfind("<div class=\"codeblock\">")
                .expect("in a code block");
            let block_end = block_start + page[block_start..].find("</div>").expect("closed");
            assert!(
                page[block_start..block_end].contains("<pre"),
                "{file}: a copy button's block has no command"
            );
        }
        assert!(
            page.contains("button.closest('.codeblock')"),
            "{file}: the script finds the command from the button's code block"
        );
        assert!(
            !page.contains("button.parentNode.querySelector('pre')"),
            "{file}: the script looks for the command beside the button, where it is not"
        );
    }
}
