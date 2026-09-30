//! Project environment detection — pure parts.
//!
//! This module receives `&[u8]` (manifest contents read by the imperative shell)
//! and returns a typed report. It opens no files and spawns no
//! processes. The orchestration that queries the file system lives
//! in `codev-engine::detect`.

pub mod license;
pub mod mcp;
pub mod stack;

/// What the probe extracted from a project.
///
/// Every field is optional: detection is best-effort — a missing or
/// unreadable manifest yields `None`, never a made-up value. A project
/// with no known manifest yields a mostly empty `Detected`; that is
/// not an error.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Detected {
    pub stack: Option<stack::Stack>,
    pub project_name: Option<String>,
    pub license: Option<String>,
    pub has_ci: bool,
    pub is_git_repo: bool,
    pub mcps: Vec<mcp::DetectedMcp>,
}

impl Detected {
    /// A completely empty report — for tests, and as the starting point
    /// for the orchestrator, which fills it in field by field.
    pub fn empty() -> Self {
        Self::default()
    }
}
