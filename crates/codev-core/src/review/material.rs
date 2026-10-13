//! What the reviewers of a step read, and which part of it must be
//! summarized to stay within a size limit.
//!
//! In isolated mode every reviewer reads the whole material: its size is
//! multiplied by the number of roles. The budget is decided here, without
//! judgement, so that the record can say exactly what was summarized; the
//! summary itself is the skill's work.

use std::collections::BTreeSet;

/// The material a step's reviewers may read whole: about 40,000 tokens,
/// far from a context window once a role's criteria and knowledge are
/// added.
pub const LIMIT_BYTES: u64 = 160 * 1024;

/// How a file reaches the reviewers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Treatment {
    /// Read whole.
    Full,
    /// Over the budget: the skill summarizes its change.
    Summarize,
}

impl Treatment {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Full => "full",
            Self::Summarize => "summarize",
        }
    }
}

/// A file of the material, as measured.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaterialFile {
    /// Relative to the project root, `/`-separated.
    pub path: String,
    pub bytes: u64,
    pub treatment: Treatment,
}

/// The material of a step, its files in path order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Material {
    pub limit_bytes: u64,
    pub total_bytes: u64,
    pub files: Vec<MaterialFile>,
}

impl Material {
    /// At least one file is to be summarized.
    pub fn summarized(&self) -> bool {
        self.files
            .iter()
            .any(|f| f.treatment == Treatment::Summarize)
    }
}

/// Decides each file's treatment: from the smallest to the largest, ties
/// in path order, a file is read whole as long as the running sum stays
/// within `limit`. Smallest first keeps the most files whole. A path given
/// twice counts once.
pub fn budget(files: &[(String, u64)], limit: u64) -> Material {
    let mut unique: Vec<(String, u64)> = files.to_vec();
    unique.sort_by(|(a, _), (b, _)| a.cmp(b));
    unique.dedup_by(|(a, _), (b, _)| a == b);

    // Walk the files from the smallest, ties in path order, keeping each
    // whole while the running sum fits.
    let mut by_size: Vec<&(String, u64)> = unique.iter().collect();
    by_size.sort_by(|(path_a, bytes_a), (path_b, bytes_b)| {
        bytes_a.cmp(bytes_b).then_with(|| path_a.cmp(path_b))
    });
    let mut sum: u64 = 0;
    let mut kept_whole: BTreeSet<&str> = BTreeSet::new();
    for (path, bytes) in by_size {
        if sum.saturating_add(*bytes) > limit {
            break;
        }
        sum += bytes;
        kept_whole.insert(path);
    }

    let files = unique
        .iter()
        .map(|(path, bytes)| MaterialFile {
            path: path.clone(),
            bytes: *bytes,
            treatment: if kept_whole.contains(path.as_str()) {
                Treatment::Full
            } else {
                Treatment::Summarize
            },
        })
        .collect();
    Material {
        limit_bytes: limit,
        total_bytes: unique.iter().map(|(_, bytes)| *bytes).sum(),
        files,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn files(list: &[(&str, u64)]) -> Vec<(String, u64)> {
        list.iter().map(|(p, b)| (p.to_string(), *b)).collect()
    }

    fn treatment(material: &Material, path: &str) -> Treatment {
        material
            .files
            .iter()
            .find(|f| f.path == path)
            .unwrap()
            .treatment
    }

    #[test]
    fn small_material_is_read_in_full() {
        let material = budget(
            &files(&[("src/b.rs", 800), ("src/a.rs", 1200)]),
            LIMIT_BYTES,
        );
        assert_eq!(material.total_bytes, 2000);
        assert!(!material.summarized());
        assert_eq!(
            material
                .files
                .iter()
                .map(|f| f.path.as_str())
                .collect::<Vec<_>>(),
            ["src/a.rs", "src/b.rs"],
            "listed in path order"
        );
        assert!(
            material
                .files
                .iter()
                .all(|f| f.treatment == Treatment::Full)
        );
    }

    #[test]
    fn material_over_the_limit_summarizes_the_largest() {
        let material = budget(
            &files(&[("a.rs", 100_000), ("b.rs", 60_000), ("c.rs", 10_000)]),
            LIMIT_BYTES,
        );
        assert_eq!(treatment(&material, "c.rs"), Treatment::Full);
        assert_eq!(treatment(&material, "b.rs"), Treatment::Full);
        assert_eq!(treatment(&material, "a.rs"), Treatment::Summarize);
        assert!(material.summarized());
        assert_eq!(material.total_bytes, 170_000);
        assert_eq!(material.limit_bytes, 163_840);
    }

    #[test]
    fn ties_are_broken_by_path() {
        let material = budget(&files(&[("b.rs", 60), ("a.rs", 60)]), 100);
        assert_eq!(treatment(&material, "a.rs"), Treatment::Full);
        assert_eq!(treatment(&material, "b.rs"), Treatment::Summarize);
    }

    #[test]
    fn an_empty_material_is_not_summarized() {
        let material = budget(&[], LIMIT_BYTES);
        assert!(material.files.is_empty());
        assert_eq!(material.total_bytes, 0);
        assert!(!material.summarized());
    }

    #[test]
    fn a_file_alone_over_the_limit_is_summarized() {
        let material = budget(
            &files(&[("huge.rs", LIMIT_BYTES + 1), ("x.rs", 0)]),
            LIMIT_BYTES,
        );
        assert_eq!(treatment(&material, "huge.rs"), Treatment::Summarize);
        assert_eq!(treatment(&material, "x.rs"), Treatment::Full);
    }
}
