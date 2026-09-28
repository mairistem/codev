//! Contrôle croisé : le YAML généré par `codev-core::config::render` doit
//! parser sans erreur via le lecteur `codev-engine::config::ProjectConfig`.
//!
//! Si un champ change de sérialisation dans le générateur sans que le
//! lecteur soit ajusté, ce test tombe rouge — c'est ce qu'on veut. Il
//! garantit qu'un `codev init --yes` produit un fichier immédiatement
//! utilisable par `codev status` sur le projet neuf.

use codev_core::config::{from_detected, render, UserChoices};
use codev_core::detect::{
    mcp::DetectedMcp,
    stack::Stack,
    Detected,
};
use codev_engine::config::ProjectConfig;

fn detected_full() -> Detected {
    Detected {
        stack: Some(Stack {
            language: "Rust".into(),
            edition_or_version: Some("2024".into()),
            workspace_crate_count: Some(4),
            dependencies_summary: vec!["serde".into()],
        }),
        project_name: Some("codev".into()),
        license: Some("MIT".into()),
        has_ci: true,
        is_git_repo: true,
        mcps: vec![DetectedMcp {
            name: "claude.ai Atlassian Rovo".into(),
            command: None,
            url: Some("https://mcp.atlassian.com/".into()),
            source: ".mcp.json".into(),
        }],
    }
}

#[test]
fn yaml_genere_full_est_relisible_par_project_config() {
    let choices = UserChoices {
        workflows: vec![
            "propose".into(),
            "explore".into(),
            "onboard".into(),
            "apply".into(),
            "sync".into(),
            "archive".into(),
            "update".into(),
        ],
        context_addition: Some("Test roundtrip.".into()),
        jira_tool_confirmed: Some("mcp__claude_ai_Atlassian_Rovo__getJiraIssue".to_string()),
    };
    let g = from_detected(&detected_full(), &choices);
    let yaml = render(&g);

    let parsed: ProjectConfig =
        serde_norway::from_str(&yaml).expect("YAML généré doit parser via ProjectConfig");

    let workflows = parsed.workflows.expect("workflows présent");
    assert_eq!(workflows.len(), 7);
    assert!(workflows.contains(&"apply".to_string()));

    assert_eq!(
        parsed.mcp.jira_tool.as_deref(),
        Some("mcp__claude_ai_Atlassian_Rovo__getJiraIssue")
    );

    let context = parsed.context.expect("context présent");
    assert!(context.contains("Rust workspace"));
    assert!(context.contains("Test roundtrip"));
}

#[test]
fn yaml_genere_minimal_est_relisible() {
    let choices = UserChoices {
        workflows: vec!["propose".into(), "explore".into(), "onboard".into()],
        context_addition: None,
        jira_tool_confirmed: None,
    };
    let g = from_detected(&Detected::empty(), &choices);
    let yaml = render(&g);

    let parsed: ProjectConfig = serde_norway::from_str(&yaml).unwrap();
    assert_eq!(parsed.workflows.unwrap().len(), 3);
    assert!(parsed.context.is_none());
    // Aucun bloc `mcp:` généré → jira_tool absent (McpConfig par défaut).
    assert!(parsed.mcp.jira_tool.is_none());
}
