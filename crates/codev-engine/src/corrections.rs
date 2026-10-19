//! The corrections of a change: reads `corrections.md` and lets
//! `codev-core::corrections` parse and check it (decision 0001).

use std::path::PathBuf;

use codev_core::corrections::{self as core_corrections, Corrections};
use codev_core::parser::ast::Finding;
use codev_core::{ChangeId, Layout};

use crate::error::{EngineError, Result};
use crate::ports::FileSystem;

/// What codev can say about a change's corrections.
#[derive(Debug, Clone, Default)]
pub struct CorrectionsCheck {
    /// `corrections.md`, relative to the project root.
    pub path: PathBuf,
    /// The well-formed entries; empty without the file, without its format
    /// marker, or in a newer format.
    pub corrections: Corrections,
    /// The file's defects, at their line in `corrections.md`.
    pub findings: Vec<Finding>,
}

/// Reads and checks the corrections of `change`.
pub fn check(fs: &dyn FileSystem, layout: &Layout, change: &ChangeId) -> Result<CorrectionsCheck> {
    let path = layout.change_dir(change).join(core_corrections::FILE);
    let relative = path
        .strip_prefix(layout.project_root())
        .unwrap_or(&path)
        .to_path_buf();
    if !fs.exists(&path) {
        return Ok(CorrectionsCheck {
            path: relative,
            corrections: Corrections::default(),
            findings: Vec::new(),
        });
    }
    let source = fs
        .read_to_string(&path)
        .map_err(|e| EngineError::Unreadable {
            path: path.clone(),
            reason: e.to_string(),
        })?;
    let parsed = core_corrections::parse(&source);
    Ok(CorrectionsCheck {
        path: relative,
        corrections: parsed.value,
        findings: parsed.findings,
    })
}
