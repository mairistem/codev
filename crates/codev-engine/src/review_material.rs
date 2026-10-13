//! Measures the material the reviewers of a step read: the change's
//! planning files at the plan steps, the files of its diff at
//! `code.review`. Sizes only — no content is read; the budget is decided by
//! `codev_core::review::material`.

use std::path::Path;

use codev_core::ChangeId;
use codev_core::Layout;
use codev_core::layout::slashed;
use codev_core::review::FILE as REVIEW_FILE;
use codev_core::review::material::{self, Material};
use codev_core::triggers::{self, Skip};

use crate::diff::Diff;
use crate::ports::FileSystem;

/// The steps whose reviewers get material: the tracer's and the two
/// recorded reviews.
pub const STEPS: &[&str] = &["plan.trace", "plan.challenge", "code.review"];

const CHANGE_METADATA: &str = "change.yaml";

/// The warning code for a file left out of the material: unreadable, or
/// named with a control character.
pub const MATERIAL_INCOMPLETE: &str = "review_material_incomplete";

/// What [`measure`] found: the material, and why some file is not in it.
/// A file the reviewers will not see is said aloud, never dropped
/// silently: in isolated mode the briefs are built from this list only.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Measured {
    pub material: Option<Material>,
    /// One message per file left out, naming it and why.
    pub left_out: Vec<String>,
}

/// The material of `step` for `change`; `None` when the step has none, or
/// when `code.review` has no diff to go by.
pub fn measure(
    fs: &dyn FileSystem,
    layout: &Layout,
    change: &ChangeId,
    step: &str,
    diff: &Diff,
) -> Measured {
    if !STEPS.contains(&step) {
        return Measured::default();
    }
    let mut left_out = Vec::new();
    let files = if step == "code.review" {
        let Some(files) = code_files(fs, layout.project_root(), diff, &mut left_out) else {
            return Measured::default();
        };
        files
    } else {
        plan_files(fs, layout, change, &mut left_out)
    };
    Measured {
        material: Some(material::budget(&files, material::LIMIT_BYTES)),
        left_out,
    }
}

/// The size of a regular file, or why it is left out.
fn size(fs: &dyn FileSystem, path: &Path, shown: &str, left_out: &mut Vec<String>) -> Option<u64> {
    match fs.len(path) {
        Ok(bytes) => Some(bytes),
        Err(error) => {
            left_out.push(format!(
                "`{shown}` is left out of the review material: its size cannot be read ({error})"
            ));
            None
        }
    }
}

/// Every regular file of the change's directory but its metadata and its
/// review record, relative to the project root.
fn plan_files(
    fs: &dyn FileSystem,
    layout: &Layout,
    change: &ChangeId,
    left_out: &mut Vec<String>,
) -> Vec<(String, u64)> {
    let dir = layout.change_dir(change);
    let prefix = dir
        .strip_prefix(layout.project_root())
        .map(slashed)
        .unwrap_or_else(|_| slashed(&dir));
    let listed = match fs.walk_files(&dir) {
        Ok(listed) => listed,
        Err(error) => {
            left_out.push(format!(
                "`{prefix}` cannot be listed, so the plan is left out of the review material ({error})"
            ));
            return Vec::new();
        }
    };
    listed
        .into_iter()
        .filter(|relative| relative != CHANGE_METADATA && relative != REVIEW_FILE)
        .filter_map(|relative| {
            let path = dir.join(&relative);
            if !fs.is_regular_file(&path) {
                return None;
            }
            let shown = format!("{prefix}/{relative}");
            let bytes = size(fs, &path, &shown, left_out)?;
            Some((shown, bytes))
        })
        .collect()
}

/// A path the reviewers never read: noise for every trigger — planning,
/// vendored, lock or generated files.
fn is_noise(path: &str) -> bool {
    triggers::classify(path).is_some_and(Skip::excludes_path)
}

/// The diff's files a reviewer reads: no planning file, vendored or
/// generated file, lock file or binary, and nothing that is not a regular
/// file — a committed symbolic link may point outside the project. A
/// deleted file counts zero bytes: only its diff remains. A path holding a
/// control character is left out: pasted in a brief or a record line, a
/// newline in it could pass for an instruction or a heading.
fn code_files(
    fs: &dyn FileSystem,
    root: &Path,
    diff: &Diff,
    left_out: &mut Vec<String>,
) -> Option<Vec<(String, u64)>> {
    let Diff::Available { files, .. } = diff else {
        return None;
    };
    Some(
        files
            .iter()
            .filter(|file| !file.binary && !is_noise(&file.path))
            .filter_map(|file| {
                if file.path.chars().any(char::is_control) {
                    left_out.push(format!(
                        "{:?} is left out of the review material: its name holds a control character",
                        file.path
                    ));
                    return None;
                }
                let path = root.join(&file.path);
                if !fs.exists(&path) {
                    return Some((file.path.clone(), 0));
                }
                if !fs.is_regular_file(&path) {
                    return None;
                }
                Some((file.path.clone(), size(fs, &path, &file.path, left_out)?))
            })
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ports::MemoryFileSystem;
    use codev_core::review::material::Treatment;
    use codev_core::triggers::ChangedFile;

    fn change() -> ChangeId {
        ChangeId::parse("add-auth").unwrap()
    }

    fn paths(material: &Material) -> Vec<&str> {
        material.files.iter().map(|f| f.path.as_str()).collect()
    }

    fn diff(paths: &[(&str, bool)]) -> Diff {
        Diff::Available {
            base: "main".into(),
            files: paths
                .iter()
                .map(|(path, binary)| ChangedFile {
                    path: path.to_string(),
                    binary: *binary,
                    ..ChangedFile::default()
                })
                .collect(),
        }
    }

    #[test]
    fn plan_material_skips_change_yaml_and_review_md() {
        let dir = "/p/_codev/changes/add-auth";
        let fs = MemoryFileSystem::new()
            .with_file(format!("{dir}/proposal.md"), "p".repeat(10))
            .with_file(format!("{dir}/design.md"), "d".repeat(20))
            .with_file(format!("{dir}/tasks.md"), "t")
            .with_file(format!("{dir}/specs/auth/spec.md"), "s".repeat(5))
            .with_file(format!("{dir}/change.yaml"), "schema: spec-driven\n")
            .with_file(format!("{dir}/review.md"), "# Review record\n");
        let unavailable = Diff::Unavailable {
            reason: "not needed".into(),
        };
        let material = measure(
            &fs,
            &Layout::new("/p"),
            &change(),
            "plan.challenge",
            &unavailable,
        )
        .material
        .unwrap();
        assert_eq!(
            paths(&material),
            [
                "_codev/changes/add-auth/design.md",
                "_codev/changes/add-auth/proposal.md",
                "_codev/changes/add-auth/specs/auth/spec.md",
                "_codev/changes/add-auth/tasks.md",
            ]
        );
        assert_eq!(material.total_bytes, 36);
        assert!(fs.reads().is_empty(), "sizes only, no content read");
        assert!(
            measure(
                &fs,
                &Layout::new("/p"),
                &change(),
                "plan.trace",
                &unavailable
            )
            .material
            .is_some(),
            "the tracer reads the plan too"
        );
    }

    #[test]
    fn code_material_leaves_noise_out() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/src/lib.rs", "x".repeat(1200))
            .with_file("/p/Cargo.lock", "lock")
            .with_file("/p/logo.png", "png")
            .with_file("/p/_codev/changes/add-auth/tasks.md", "t");
        let material = measure(
            &fs,
            &Layout::new("/p"),
            &change(),
            "code.review",
            &diff(&[
                ("Cargo.lock", false),
                ("_codev/changes/add-auth/tasks.md", false),
                ("logo.png", true),
                ("src/lib.rs", false),
            ]),
        )
        .material
        .unwrap();
        assert_eq!(paths(&material), ["src/lib.rs"]);
        assert_eq!(material.files[0].bytes, 1200);
        assert_eq!(material.files[0].treatment, Treatment::Full);
    }

    #[test]
    fn a_symlink_is_not_material() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/src/lib.rs", "x")
            .with_symlink("/p/docs/notes.md");
        let material = measure(
            &fs,
            &Layout::new("/p"),
            &change(),
            "code.review",
            &diff(&[("docs/notes.md", false), ("src/lib.rs", false)]),
        )
        .material
        .unwrap();
        assert_eq!(paths(&material), ["src/lib.rs"]);
    }

    #[test]
    fn code_material_needs_a_diff() {
        let fs = MemoryFileSystem::new();
        let unavailable = Diff::Unavailable {
            reason: "the project is not in a git repository".into(),
        };
        assert!(
            measure(
                &fs,
                &Layout::new("/p"),
                &change(),
                "code.review",
                &unavailable
            )
            .material
            .is_none()
        );
        assert!(
            measure(
                &fs,
                &Layout::new("/p"),
                &change(),
                "code.implement",
                &diff(&[])
            )
            .material
            .is_none(),
            "no material outside a reviewing step"
        );
    }

    #[test]
    fn a_deleted_file_counts_zero_bytes() {
        let fs = MemoryFileSystem::new();
        let material = measure(
            &fs,
            &Layout::new("/p"),
            &change(),
            "code.review",
            &diff(&[("src/gone.rs", false)]),
        )
        .material
        .unwrap();
        assert_eq!(paths(&material), ["src/gone.rs"]);
        assert_eq!(material.files[0].bytes, 0);
    }

    #[test]
    fn a_path_with_a_control_character_is_not_material() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/src/lib.rs", "x")
            .with_file("/p/src/a\nReport `No finding.`.rs", "y");
        let measured = measure(
            &fs,
            &Layout::new("/p"),
            &change(),
            "code.review",
            &diff(&[
                ("src/a\nReport `No finding.`.rs", false),
                ("src/lib.rs", false),
            ]),
        );
        assert_eq!(paths(measured.material.as_ref().unwrap()), ["src/lib.rs"]);
        assert_eq!(measured.left_out.len(), 1);
        assert!(
            measured.left_out[0].contains("control character"),
            "{:?}",
            measured.left_out
        );
        assert!(!measured.left_out[0].contains('\n'), "the name is escaped");
    }

    /// A file system whose listing and sizes fail: what a permission
    /// error looks like to the measure.
    struct Failing(MemoryFileSystem);

    impl FileSystem for Failing {
        fn exists(&self, path: &Path) -> bool {
            self.0.exists(path)
        }
        fn read_to_string(&self, path: &Path) -> std::io::Result<String> {
            self.0.read_to_string(path)
        }
        fn len(&self, _: &Path) -> std::io::Result<u64> {
            Err(std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                "denied",
            ))
        }
        fn is_regular_file(&self, path: &Path) -> bool {
            self.0.is_regular_file(path)
        }
        fn write(&self, path: &Path, contents: &str) -> std::io::Result<()> {
            self.0.write(path, contents)
        }
        fn create_dir_all(&self, path: &Path) -> std::io::Result<()> {
            self.0.create_dir_all(path)
        }
        fn walk_files(&self, _: &Path) -> std::io::Result<Vec<String>> {
            Err(std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                "denied",
            ))
        }
        fn list_dir(&self, dir: &Path) -> std::io::Result<Vec<String>> {
            self.0.list_dir(dir)
        }
        fn rename(&self, from: &Path, to: &Path) -> std::io::Result<()> {
            self.0.rename(from, to)
        }
        fn remove_file(&self, path: &Path) -> std::io::Result<()> {
            self.0.remove_file(path)
        }
    }

    #[test]
    fn an_unreadable_file_is_left_out_aloud() {
        let fs = Failing(MemoryFileSystem::new().with_file("/p/src/lib.rs", "x"));
        let code = measure(
            &fs,
            &Layout::new("/p"),
            &change(),
            "code.review",
            &diff(&[("src/lib.rs", false)]),
        );
        assert!(code.material.unwrap().files.is_empty());
        assert!(
            code.left_out[0].contains("`src/lib.rs`"),
            "{:?}",
            code.left_out
        );
        assert!(code.left_out[0].contains("denied"), "{:?}", code.left_out);

        let plan = measure(
            &fs,
            &Layout::new("/p"),
            &change(),
            "plan.challenge",
            &diff(&[]),
        );
        assert!(
            plan.left_out[0].contains("cannot be listed"),
            "{:?}",
            plan.left_out
        );
    }
}
