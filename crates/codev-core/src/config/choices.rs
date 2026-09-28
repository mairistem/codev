//! Choix collectés au prompt (ou fournis par les flags CLI) pour produire
//! le YAML final.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserChoices {
    /// Les workflows à installer, dans l'ordre voulu dans le YAML.
    pub workflows: Vec<String>,
    /// Le texte libre ajouté par l'utilisateur au bloc `context:`.
    /// `None` ou chaîne vide → aucun ajout, seule la détection remplit
    /// le contexte.
    pub context_addition: Option<String>,
    /// Le tool ID MCP Jira retenu, s'il y en a un.
    pub jira_tool_confirmed: Option<String>,
}

impl UserChoices {
    /// Un ensemble de choix « défaut complet » — 7 workflows, aucune
    /// personnalisation.
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

    /// Un ensemble « minimal » — 3 workflows, aucune personnalisation.
    pub fn defaults_minimal() -> Self {
        Self {
            workflows: vec!["propose".into(), "explore".into(), "onboard".into()],
            context_addition: None,
            jira_tool_confirmed: None,
        }
    }
}
