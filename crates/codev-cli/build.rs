//! Embeds the documentation chapters into the binary, one set per language.
//!
//! The chapter list is read from `docs/<lang>/src/SUMMARY.md` — the same
//! table of contents mdBook uses for the website — so `codev docs` and the
//! published site can never disagree on which chapters exist or their order.
//! For each language this generates `$OUT_DIR/docs_<lang>.rs`, a
//! `&[(&str, &str)]` of `(chapter path, chapter source)` built with
//! `include_str!`.

use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

const LANGUAGES: &[&str] = &["en", "fr"];

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("set by cargo"));
    let docs_dir = manifest_dir.join("../../docs");
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("set by cargo"));

    for lang in LANGUAGES {
        let src = docs_dir.join(lang).join("src");
        let summary_path = src.join("SUMMARY.md");
        println!("cargo:rerun-if-changed={}", summary_path.display());

        let summary = fs::read_to_string(&summary_path).unwrap_or_else(|e| {
            panic!("cannot read {}: {e}", summary_path.display());
        });

        let mut generated = String::from("&[\n");
        for chapter in chapter_paths(&summary) {
            let file = src.join(&chapter);
            assert!(
                file.is_file(),
                "{} lists `{chapter}`, which does not exist",
                summary_path.display()
            );
            println!("cargo:rerun-if-changed={}", file.display());
            let _ = writeln!(
                generated,
                "    ({chapter:?}, include_str!({:?})),",
                canonical(&file)
            );
        }
        generated.push_str("]\n");

        fs::write(out_dir.join(format!("docs_{lang}.rs")), generated).expect("OUT_DIR is writable");
    }
}

/// The local chapter paths linked from a SUMMARY.md, in order.
///
/// Only `[Title](relative/path.md)` links count; external links and
/// anchors are ignored, as are draft chapters with an empty target.
fn chapter_paths(summary: &str) -> Vec<String> {
    let mut paths = Vec::new();
    let mut rest = summary;
    while let Some(start) = rest.find("](") {
        rest = &rest[start + 2..];
        let Some(end) = rest.find(')') else { break };
        let target = &rest[..end];
        rest = &rest[end..];
        let is_local_chapter = target.ends_with(".md") && !target.contains("://");
        if is_local_chapter {
            paths.push(target.trim_start_matches("./").to_string());
        }
    }
    paths
}

fn canonical(path: &Path) -> PathBuf {
    path.canonicalize()
        .unwrap_or_else(|e| panic!("cannot resolve {}: {e}", path.display()))
}
