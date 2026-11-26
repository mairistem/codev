//! The rename of a replayed tree when rustfmt cannot run.

use std::fs;
use std::path::PathBuf;

use compagnon_core::legacy;
use compagnon_evals::runner;

#[test]
fn without_rustfmt_the_preparation_fails() {
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("without-rustfmt");
    let _ = fs::remove_dir_all(&dir);
    let write = |path: &str, text: &str| {
        let path = dir.join(path);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, text).unwrap();
    };
    write(
        &format!("{}/specs/a/spec.md", legacy::PLANNING_DIR),
        "# A\n",
    );
    write(
        "Cargo.toml",
        "[package]\nname = \"x\"\nedition = \"2024\"\n",
    );
    write("src/lib.rs", &format!("// {}\n", legacy::BINARY_STEM));
    let missing = dir.join("no-bin/rustfmt");
    let error = runner::bring_through_rename_with(&dir, &missing).unwrap_err();
    assert!(error.contains("rustfmt"), "{error}");
    assert!(error.contains("src/lib.rs"), "{error}");
}
