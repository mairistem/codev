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
