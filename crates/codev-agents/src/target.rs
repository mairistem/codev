use std::path::{Path, PathBuf};

use codev_core::Plan;
use codev_engine::FileSystem;

use crate::workflows::Workflow;

/// An agent tool able to host codev's workflows.
///
/// A single implementation exists — Claude Code — deliberately: this trait
/// is not there for hypothetical generality, but so that adding Cursor or an
/// `.agents/` target remains one more file, without touching the rest. It is
/// the only place in the project that knows anything about a tool.
///
/// `dyn`-compatible: the ports arrive as dynamic references so that the CLI
/// can hold a list of heterogeneous targets.
pub trait AgentTarget {
    fn id(&self) -> &'static str;

    fn label(&self) -> &'static str;

    /// True if this tool is visibly used in this project.
    ///
    /// Used to prefill the `codev init` selection, never to decide alone: a
    /// project may want skills for a tool it has not configured yet.
    fn detect(&self, fs: &dyn FileSystem, project_root: &Path) -> bool;

    /// Plans the writing of the skills, without writing anything.
    fn plan_skills(
        &self,
        fs: &dyn FileSystem,
        project_root: &Path,
        workflows: &[&Workflow],
        version: &str,
        force: bool,
    ) -> SkillsPlan;
}

/// The result of planning skills.
#[derive(Debug, Default)]
pub struct SkillsPlan {
    pub plan: Plan,
    /// The files left in place because they were edited by hand.
    ///
    /// Distinguishing this case from a plain "nothing to do" is what makes it
    /// possible to tell the user why their skill did not change, and how to
    /// force it.
    pub preserved: Vec<PathBuf>,
}
