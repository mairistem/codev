//! The product's former names, from before it was renamed Compagnon.
//!
//! **This module is the only place in the code where the former name may
//! appear.** Everything else uses the current names; a test or a feature
//! that needs a former name — `compagnon migrate`, the refusal of a former
//! planning root, the release assets published under the former name, the
//! notice printed when the binary still runs under its former name —
//! builds it from these constants, never from a literal. The repository's
//! audit (`rg -i codev`) relies on it.
//!
//! See `_compagnon/decisions/0014-the-product-is-named-compagnon.md`.

/// The former planning root, refused by every command but `migrate`.
pub const PLANNING_DIR: &str = "_codev";

/// The former lock file, inside the planning root.
pub const LOCK_FILE: &str = "codev.lock";

/// The former configuration key holding the version requirement.
pub const VERSION_KEY: &str = "codev_version";

/// The former skill prefix: `propose` was installed as `codev-propose`.
pub const SKILL_PREFIX: &str = "codev";

/// Prefix of the former opening line of a managed section.
pub const BEGIN_PREFIX: &str = "<!-- codev:begin";

/// The former closing line of a managed section.
pub const END_LINE: &str = "<!-- codev:end -->";

/// The former first-line marker of a file the product owned entirely.
pub const MANAGED_YAML_MARKER: &str = "# codev:managed";

/// The former format marker of a review record, without its number:
/// `<!-- codev-review: 1 -->`.
pub const REVIEW_MARKER: &str = "codev-review:";

/// The former format marker of a corrections file, without its number.
pub const CORRECTIONS_MARKER: &str = "codev-corrections:";

/// The former generated CI workflow.
pub const WORKFLOW: &str = ".github/workflows/codev.yml";

/// The former binary's file stem (`codev`, `codev.exe`).
pub const BINARY_STEM: &str = "codev";

/// The former prefix of the release archives and of the folder they
/// extract to: `codev-0.6.1-x86_64-unknown-linux-musl.tar.gz`.
pub const ASSET_PREFIX: &str = "codev";

/// The former installers' version variable.
pub const VERSION_ENV: &str = "CODEV_VERSION";

/// The first release published under the current name. Releases below it
/// were published under the former names only.
pub const FIRST_RENAMED_RELEASE: (u64, u64, u64) = (0, 7, 0);

/// Whether `version` (`X.Y.Z`, with or without a leading `v`) was
/// published under the former names only. `None` when it does not parse.
pub fn published_under_former_name(version: &str) -> Option<bool> {
    let v = semver::Version::parse(version.trim().trim_start_matches('v')).ok()?;
    let (major, minor, patch) = FIRST_RENAMED_RELEASE;
    Some(v < semver::Version::new(major, minor, patch))
}

/// The notice printed on stderr when the binary runs under its former
/// name, or `None` when it runs under any other. `exe_stem` is the file
/// name of the running executable without its extension.
pub fn rename_notice(exe_stem: &str) -> Option<String> {
    (exe_stem == BINARY_STEM).then(|| {
        format!(
            "note: {BINARY_STEM} is now Compagnon. Rename this executable to `compagnon` \
             (or reinstall it with the installer), then run `compagnon migrate` in each project."
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legacy_names_are_the_codev_ones() {
        let stem = BINARY_STEM;
        assert_eq!(stem.len(), 5);
        assert_eq!(PLANNING_DIR, format!("_{stem}"));
        assert_eq!(LOCK_FILE, format!("{stem}.lock"));
        assert_eq!(VERSION_KEY, format!("{stem}_version"));
        assert_eq!(SKILL_PREFIX, stem);
        assert_eq!(ASSET_PREFIX, stem);
        assert_eq!(BEGIN_PREFIX, format!("<!-- {stem}:begin"));
        assert_eq!(END_LINE, format!("<!-- {stem}:end -->"));
        assert_eq!(MANAGED_YAML_MARKER, format!("# {stem}:managed"));
        assert_eq!(REVIEW_MARKER, format!("{stem}-review:"));
        assert_eq!(CORRECTIONS_MARKER, format!("{stem}-corrections:"));
        assert_eq!(WORKFLOW, format!(".github/workflows/{stem}.yml"));
        assert_eq!(VERSION_ENV, stem.to_uppercase() + "_VERSION");
    }

    #[test]
    fn releases_below_0_7_were_published_under_the_former_name() {
        assert_eq!(published_under_former_name("0.6.1"), Some(true));
        assert_eq!(published_under_former_name("v0.6.1"), Some(true));
        assert_eq!(published_under_former_name("0.7.0"), Some(false));
        assert_eq!(published_under_former_name("1.0.0"), Some(false));
        assert_eq!(published_under_former_name("nope"), None);
    }

    #[test]
    fn a_codev_stem_gets_the_rename_notice() {
        let notice = rename_notice(BINARY_STEM).expect("a notice");
        assert!(notice.contains("compagnon"), "{notice}");
        assert!(notice.contains("compagnon migrate"), "{notice}");
        assert!(!notice.contains('\n'), "one line: {notice}");
    }

    #[test]
    fn a_compagnon_stem_gets_no_notice() {
        assert_eq!(rename_notice("compagnon"), None);
        assert_eq!(rename_notice("anything-else"), None);
    }
}
