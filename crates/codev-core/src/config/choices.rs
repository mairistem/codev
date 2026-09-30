//! Choices collected at the prompt (or supplied through CLI flags) to produce
//! the final YAML.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserChoices {
    /// The workflows to install, in the order they should appear in the YAML.
    pub workflows: Vec<String>,
    /// Free-form text the user adds to the `context:` block.
    /// `None` or an empty string → nothing added; detection alone fills
    /// the context.
    pub context_addition: Option<String>,
    /// The selected Jira MCP tool ID, if any.
    pub jira_tool_confirmed: Option<String>,
}

impl UserChoices {
    /// A "full default" set of choices — 7 workflows, no
    /// customization.
    pub fn defaults_full() -> Self {
        Self {
            workflows: vec![
                "propose".into(),
                "explore".into(),
                "onboard".into(),
                "apply".into(),
                "sync".into(),
                "archive".into(),
                "update".into(),
            ],
            context_addition: None,
            jira_tool_confirmed: None,
        }
    }

    /// A "minimal" set — 3 workflows, no customization.
    pub fn defaults_minimal() -> Self {
        Self {
            workflows: vec!["propose".into(), "explore".into(), "onboard".into()],
            context_addition: None,
            jira_tool_confirmed: None,
        }
    }
}
