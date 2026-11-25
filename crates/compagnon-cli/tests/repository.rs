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

/// Every `.rs` file under `dir`, as `/`-separated paths relative to the
/// repository.
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
        assert!(
            svg.contains("<title id=\"t\">Compagnon</title>"),
            "{file} has no title"
        );
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
            "src=\"docs/assets/compagnon-logo.svg\" alt=\"Compagnon\"",
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
    for file in ["docs/theme/index.hbs", "docs/site/index.html"] {
        let text = read(file);
        let uses: Vec<&str> = text
            .match_indices("<use href=\"#cp-mark\">")
            .map(|(i, _)| {
                let start = text[..i].rfind("<svg").expect("inside an svg");
                &text[start..i]
            })
            .collect();
        assert!(!uses.is_empty(), "{file} does not show the mark");
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
    let site = read("docs/site/index.html");
    assert!(
        site.contains("<title>Compagnon — "),
        "the site's title names Compagnon"
    );
    assert!(
        site.contains("<meta name=\"description\" content=\"Compagnon "),
        "the site's description names Compagnon"
    );
    for (lang, needle) in [("en", "compagnonnage"), ("fr", "compagnonnage")] {
        let intro = read(&format!("docs/{lang}/src/introduction.md"));
        assert!(
            intro.starts_with("# Compagnon\n"),
            "docs/{lang} introduction title"
        );
        assert!(
            intro.contains(needle),
            "docs/{lang} introduction explains the name"
        );
    }
}
