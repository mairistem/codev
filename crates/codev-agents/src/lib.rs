//! Skill generation: the only crate that knows anything about an agent
//! tool.
//!
//! It is the project's reason for being. The rest of codev manages markdown
//! files; here the workflow is made reachable from the Claude Code chat, by
//! writing the skills it discovers at startup.
//!
//! A single target exists, deliberately — but it goes through
//! [`AgentTarget`], so that adding another one remains just one more file.

pub mod claude;
pub mod target;
pub mod workflows;

pub use claude::ClaudeCode;
pub use target::{AgentTarget, SkillsPlan};
pub use workflows::{CATALOG, DEFAULT_WORKFLOWS, Workflow};
