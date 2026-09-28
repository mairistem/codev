//! Génération du `_codev/config.yaml` par `codev init`.
//!
//! Deux briques pures :
//!
//! - [`GeneratedConfig`] et son rendu [`render`] — écrivent le YAML **à la
//!   main**, caractère par caractère, avec les commentaires de provenance
//!   au-dessus des clés qui en portent. Aucune lib de sérialisation ne
//!   préserve les commentaires ; le rendu manuel est simple et testé par
//!   golden.
//! - [`from_detected`] — assemble un `GeneratedConfig` à partir du rapport
//!   de la sonde + des choix utilisateur.
//!
//! Le contrôle croisé « ce qu'on écrit doit être relisible par
//! `codev-engine::config` » vit dans les tests d'intégration de
//! `codev-engine` — sans cycle de dépendance.

use crate::detect::Detected;

pub mod choices;

pub use choices::UserChoices;

/// Une valeur générée pour le YAML, avec sa source (optionnelle).
///
/// La provenance devient un commentaire `# ...` sur la ligne juste
/// au-dessus de la clé. Aucun formattage : juste l'origine, écrite
/// telle quelle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedValue<T> {
    pub value: T,
    pub provenance: Option<String>,
}

impl<T> GeneratedValue<T> {
    pub fn without_source(value: T) -> Self {
        Self {
            value,
            provenance: None,
        }
    }

    pub fn with_source(value: T, provenance: impl Into<String>) -> Self {
        Self {
            value,
            provenance: Some(provenance.into()),
        }
    }
}

/// La config prête à être rendue en YAML.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedConfig {
    pub schema: String,
    /// Workflows retenus, dans l'ordre où on veut les voir dans le YAML.
    pub workflows: Vec<String>,
    /// Tool ID MCP Jira, si détecté et confirmé.
    pub mcp_jira: Option<GeneratedValue<String>>,
    /// Bloc `context:` libre. `None` = clé absente du YAML.
    pub context: Option<GeneratedValue<String>>,
}

impl Default for GeneratedConfig {
    fn default() -> Self {
        Self {
            schema: "spec-driven".to_string(),
            workflows: Vec::new(),
            mcp_jira: None,
            context: None,
        }
    }
}

/// Un `_codev/config.yaml` est **thin** quand il ne porte pas encore
/// d'indication utile pour piloter les skills : le `context:` est absent
/// ou fait moins de 200 caractères, ET la clé `rules:` est vide.
///
/// Le seuil de 200 caractères est arbitraire mais éclairé : la sortie
/// minimale de `codev init` sur un projet Rust nu produit `Projet Rust,
/// 2024.` (22 caractères). 200 caractères laissent largement passer un
/// contexte détaillé (stack + une phrase de projet) et coupent court aux
/// stubs.
///
/// Cette fonction est le seul juge — les trois lieux de nudge
/// (`codev init`, `codev status`, skill `onboard`) l'appellent avec les
/// mêmes primitives, garantissant un comportement cohérent.
pub fn is_config_thin(context: Option<&str>, rules_empty: bool) -> bool {
    let context_len = context.map(str::len).unwrap_or(0);
    context_len < 200 && rules_empty
}

/// Assemble un `GeneratedConfig` à partir de la détection et des choix.
///
/// Règles :
/// - Le contexte final est la concaténation du contexte auto-détecté (issu
///   de la stack) et de l'ajout utilisateur, séparés par une ligne vide.
///   Si aucun des deux n'existe, la clé `context:` reste absente.
/// - Le tool MCP Jira n'est retenu que si un candidat a été confirmé
///   (déterminé par la coquille — ici on prend ce que `choices` porte).
pub fn from_detected(detected: &Detected, choices: &UserChoices) -> GeneratedConfig {
    let context_detected = build_context_from_detection(detected);
    let addition = choices
        .context_addition
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty());
    let context = match (context_detected, addition) {
        (None, None) => None,
        (Some(auto), None) => Some(GeneratedValue::with_source(
            auto,
            provenance_for_stack(detected),
        )),
        (None, Some(addition)) => Some(GeneratedValue::without_source(addition.to_string())),
        (Some(auto), Some(addition)) => {
            let merged = format!("{auto}\n\n{addition}");
            Some(GeneratedValue::with_source(
                merged,
                provenance_for_stack(detected),
            ))
        }
    };

    let mcp_jira = choices.jira_tool_confirmed.as_ref().map(|tool_id| {
        let source = detected
            .mcps
            .iter()
            .find(|m| crate::detect::mcp::matches_jira(m))
            .map(|m| format!("détecté depuis {} → serveur « {} »", m.source, m.name))
            .unwrap_or_else(|| "fourni à la main".to_string());
        GeneratedValue::with_source(tool_id.clone(), source)
    });

    GeneratedConfig {
        schema: "spec-driven".to_string(),
        workflows: choices.workflows.clone(),
        mcp_jira,
        context,
    }
}

fn build_context_from_detection(d: &Detected) -> Option<String> {
    let mut parts: Vec<String> = Vec::new();
    if let Some(stack) = &d.stack {
        let head = match (stack.workspace_crate_count, &stack.edition_or_version) {
            (Some(n), Some(v)) => format!("Projet {} workspace ({} crates), {}.", stack.language, n, v),
            (Some(n), None) => format!("Projet {} workspace ({} crates).", stack.language, n),
            (None, Some(v)) => format!("Projet {}, {}.", stack.language, v),
            (None, None) => format!("Projet {}.", stack.language),
        };
        parts.push(head);
        if !stack.dependencies_summary.is_empty() {
            let deps = stack.dependencies_summary.join(", ");
            parts.push(format!("Dépendances principales : {deps}."));
        }
    }
    if let Some(lic) = &d.license {
        parts.push(format!("Licence : {lic}."));
    }
    if d.has_ci {
        parts.push("CI GitHub Actions active.".to_string());
    }
    if parts.is_empty() {
        None
    } else {
        Some(parts.join(" "))
    }
}

fn provenance_for_stack(d: &Detected) -> String {
    match d.stack.as_ref().map(|s| s.language.as_str()) {
        Some("Rust") => "détecté depuis Cargo.toml".to_string(),
        Some("TypeScript" | "JavaScript") => "détecté depuis package.json".to_string(),
        Some("Python") => "détecté depuis pyproject.toml".to_string(),
        Some("Go") => "détecté depuis go.mod".to_string(),
        Some("Java") => "détecté depuis pom.xml".to_string(),
        _ => "détecté depuis l'environnement".to_string(),
    }
}

/// Rend un `GeneratedConfig` en YAML, avec commentaires de provenance.
///
/// L'ordre des clés est fixe : `schema`, `workflows`, `mcp`, `context`.
/// Chaque clé non triviale porte, sur la ligne au-dessus, un commentaire
/// `# <provenance>` si l'entrée en a une.
///
/// Le rendu est déterministe : mêmes entrées, mêmes bytes.
pub fn render(config: &GeneratedConfig) -> String {
    let mut out = String::new();

    out.push_str("# Configuration codev — généré par `codev init`.\n");
    out.push_str("# Éditable à la main : chaque champ non trivial porte sa source en commentaire.\n\n");

    // schema
    out.push_str(&format!("schema: {}\n", config.schema));

    // workflows
    if !config.workflows.is_empty() {
        out.push_str("\nworkflows:\n");
        for w in &config.workflows {
            out.push_str(&format!("  - {w}\n"));
        }
    }

    // mcp
    if let Some(jira) = &config.mcp_jira {
        out.push('\n');
        if let Some(source) = &jira.provenance {
            out.push_str(&format!("# {source}\n"));
        }
        out.push_str("mcp:\n");
        out.push_str(&format!("  jira_tool: {}\n", jira.value));
    }

    // context — bloc littéral YAML `|`
    if let Some(ctx) = &config.context {
        out.push('\n');
        if let Some(source) = &ctx.provenance {
            out.push_str(&format!("# {source} — édite si besoin\n"));
        }
        out.push_str("context: |\n");
        for line in ctx.value.lines() {
            if line.is_empty() {
                out.push('\n');
            } else {
                out.push_str(&format!("  {line}\n"));
            }
        }
    }

    out
}

// ─────────────────────────────── tests ───────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::detect::{mcp::DetectedMcp, stack::Stack};

    fn workspace_rust_detected() -> Detected {
        Detected {
            stack: Some(Stack {
                language: "Rust".into(),
                edition_or_version: Some("2024".into()),
                workspace_crate_count: Some(4),
                dependencies_summary: vec!["serde".into(), "anyhow".into()],
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
    fn from_detected_full_produit_contexte_et_mcp() {
        let detected = workspace_rust_detected();
        let choices = UserChoices {
            workflows: vec!["propose".into(), "apply".into()],
            context_addition: Some("Conventions maison : erreurs typées.".into()),
            jira_tool_confirmed: Some(
                "mcp__claude_ai_Atlassian_Rovo__getJiraIssue".to_string(),
            ),
        };
        let g = from_detected(&detected, &choices);
        assert_eq!(g.schema, "spec-driven");
        assert_eq!(g.workflows, vec!["propose", "apply"]);
        assert_eq!(
            g.mcp_jira.as_ref().unwrap().value,
            "mcp__claude_ai_Atlassian_Rovo__getJiraIssue"
        );
        assert!(g
            .mcp_jira
            .as_ref()
            .unwrap()
            .provenance
            .as_deref()
            .unwrap()
            .contains(".mcp.json"));
        let ctx = g.context.unwrap();
        assert!(ctx.value.contains("Rust workspace"));
        assert!(ctx.value.contains("Conventions maison"));
        assert_eq!(ctx.provenance.as_deref(), Some("détecté depuis Cargo.toml"));
    }

    #[test]
    fn from_detected_vide_produit_config_minimale() {
        let detected = Detected::empty();
        let choices = UserChoices {
            workflows: vec!["propose".into(), "explore".into(), "onboard".into()],
            context_addition: None,
            jira_tool_confirmed: None,
        };
        let g = from_detected(&detected, &choices);
        assert_eq!(g.workflows.len(), 3);
        assert!(g.mcp_jira.is_none());
        assert!(g.context.is_none());
    }

    #[test]
    fn from_detected_mcp_seul_sans_contexte_libre() {
        let detected = workspace_rust_detected();
        let choices = UserChoices {
            workflows: vec!["propose".into()],
            context_addition: None,
            jira_tool_confirmed: Some(
                "mcp__claude_ai_Atlassian_Rovo__getJiraIssue".to_string(),
            ),
        };
        let g = from_detected(&detected, &choices);
        // Le contexte auto vient tout seul, sans l'ajout utilisateur.
        assert!(g.context.as_ref().unwrap().value.contains("Rust workspace"));
        assert!(!g.context.as_ref().unwrap().value.contains("Conventions"));
    }

    const GOLDEN_FULL: &str = "# Configuration codev — généré par `codev init`.
# Éditable à la main : chaque champ non trivial porte sa source en commentaire.

schema: spec-driven

workflows:
  - propose
  - explore
  - onboard
  - apply
  - sync
  - archive
  - update

# détecté depuis .mcp.json → serveur « claude.ai Atlassian Rovo »
mcp:
  jira_tool: mcp__claude_ai_Atlassian_Rovo__getJiraIssue

# détecté depuis Cargo.toml — édite si besoin
context: |
  Projet Rust workspace (4 crates), 2024. Dépendances principales : serde, anyhow. Licence : MIT. CI GitHub Actions active.

  Conventions maison : erreurs typées.
";

    #[test]
    fn render_full_matche_golden() {
        let detected = workspace_rust_detected();
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
            context_addition: Some("Conventions maison : erreurs typées.".into()),
            jira_tool_confirmed: Some(
                "mcp__claude_ai_Atlassian_Rovo__getJiraIssue".to_string(),
            ),
        };
        let g = from_detected(&detected, &choices);
        let rendered = render(&g);
        assert_eq!(rendered, GOLDEN_FULL);
    }

    const GOLDEN_MINIMAL: &str = "# Configuration codev — généré par `codev init`.
# Éditable à la main : chaque champ non trivial porte sa source en commentaire.

schema: spec-driven

workflows:
  - propose
  - explore
  - onboard
";

    #[test]
    fn is_config_thin_vrai_quand_contexte_court_et_rules_vides() {
        assert!(is_config_thin(Some("Projet Rust, 2024."), true));
        assert!(is_config_thin(None, true));
        assert!(is_config_thin(Some(""), true));
    }

    #[test]
    fn is_config_thin_faux_quand_contexte_long() {
        let long = "x".repeat(300);
        assert!(!is_config_thin(Some(&long), true));
    }

    #[test]
    fn is_config_thin_faux_quand_rules_presentes() {
        assert!(!is_config_thin(Some("court"), false));
        assert!(!is_config_thin(None, false));
    }

    #[test]
    fn render_minimal_matche_golden() {
        let g = GeneratedConfig {
            schema: "spec-driven".to_string(),
            workflows: vec!["propose".into(), "explore".into(), "onboard".into()],
            mcp_jira: None,
            context: None,
        };
        assert_eq!(render(&g), GOLDEN_MINIMAL);
    }
}
