//! Prompts interactifs de `codev init`, isolés dans la coquille.
//!
//! Trois responsabilités :
//! - décider si un prompt doit s'afficher (`should_prompt`) — dépend des
//!   flags CLI et de l'état du TTY ;
//! - poser au plus deux questions à l'utilisateur (workflows + contexte) ;
//! - confirmer un MCP Jira détecté (une seule question de plus si
//!   ambigu).
//!
//! La sortie est un `UserChoices` prêt à alimenter
//! `codev-core::config::from_detected`.

use std::io::IsTerminal;

use anyhow::Result;
use codev_core::config::UserChoices;
use codev_core::detect::{
    mcp::{self, DetectedMcp},
    Detected,
};
use dialoguer::{theme::ColorfulTheme, Confirm, Editor, Input, MultiSelect, Select};

/// Préset choisi via `--preset`, ou déduit d'un flag.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Preset {
    Complet,
    Minimal,
    Personnalise,
}

/// Options d'invocation de `codev init`.
#[derive(Debug, Clone, Default)]
pub struct InitOptions {
    /// `--yes` — court-circuite tous les prompts.
    pub yes: bool,
    /// `--no-detect` — désactive la sonde côté appelant, la coquille passe
    /// alors un `Detected::empty()` à ce module.
    pub no_detect: bool,
    /// `--preset` — préselectionne la réponse à la question workflows.
    pub preset: Option<Preset>,
}

/// Vrai si un prompt doit être posé à l'utilisateur.
///
/// Extraite pour être testée seule — évite d'avoir à simuler un TTY.
pub fn should_prompt(opts: &InitOptions, stdin_is_tty: bool) -> bool {
    !opts.yes && stdin_is_tty
}

const WORKFLOWS_COMPLET: &[&str] = &[
    "propose", "explore", "onboard", "apply", "sync", "archive", "update",
];
const WORKFLOWS_MINIMAL: &[&str] = &["propose", "explore", "onboard"];

/// Exécute les prompts et retourne les choix finaux.
///
/// En mode non-interactif (`--yes` ou stdin non-TTY), aucun prompt n'est
/// affiché ; les défauts s'appliquent selon le préset (ou « Complet » si
/// aucun préset n'est demandé).
pub fn run(detected: &Detected, opts: &InitOptions) -> Result<UserChoices> {
    let interactive = should_prompt(opts, std::io::stdin().is_terminal());

    // Question 1 — workflows
    let workflows = pick_workflows(opts, interactive)?;

    // Question 2 — contexte libre
    let context_addition = if interactive {
        Some(prompt_context(detected)?)
    } else {
        None
    };

    // MCP — confirmation (interactif) ou automatique (--yes / non-TTY)
    let jira_tool_confirmed = pick_mcp_jira(detected, interactive)?;

    Ok(UserChoices {
        workflows,
        context_addition,
        jira_tool_confirmed,
    })
}

fn pick_workflows(opts: &InitOptions, interactive: bool) -> Result<Vec<String>> {
    let preset = match opts.preset {
        Some(p) => Some(p),
        None if !interactive => Some(Preset::Complet),
        None => None,
    };

    if let Some(p) = preset {
        return Ok(match p {
            Preset::Complet => WORKFLOWS_COMPLET.iter().map(|s| s.to_string()).collect(),
            Preset::Minimal => WORKFLOWS_MINIMAL.iter().map(|s| s.to_string()).collect(),
            Preset::Personnalise if !interactive => {
                // Sans TTY, `--preset personnalise` retombe sur Complet plutôt
                // que sur un état indéfini.
                WORKFLOWS_COMPLET.iter().map(|s| s.to_string()).collect()
            }
            Preset::Personnalise => pick_workflows_custom()?,
        });
    }

    let theme = ColorfulTheme::default();
    let choice = Select::with_theme(&theme)
        .with_prompt("Quels workflows installer ?")
        .items(&[
            "Complet (7) — propose, explore, onboard, apply, sync, archive, update",
            "Minimal (3) — propose, explore, onboard",
            "Personnalisé — te laisse choisir un par un",
        ])
        .default(0)
        .interact()?;
    match choice {
        0 => Ok(WORKFLOWS_COMPLET.iter().map(|s| s.to_string()).collect()),
        1 => Ok(WORKFLOWS_MINIMAL.iter().map(|s| s.to_string()).collect()),
        _ => pick_workflows_custom(),
    }
}

fn pick_workflows_custom() -> Result<Vec<String>> {
    let theme = ColorfulTheme::default();
    let indices = MultiSelect::with_theme(&theme)
        .with_prompt("Sélectionne les workflows (espace pour cocher, Entrée pour valider)")
        .items(WORKFLOWS_COMPLET)
        .defaults(&[true; 7])
        .interact()?;
    Ok(indices
        .into_iter()
        .map(|i| WORKFLOWS_COMPLET[i].to_string())
        .collect())
}

fn prompt_context(detected: &Detected) -> Result<String> {
    let theme = ColorfulTheme::default();
    if let Some(stack) = &detected.stack {
        eprintln!("\nContexte pour les skills (ce qui n'est pas déductible du code) :");
        eprintln!(
            "  stack détectée : {} {}",
            stack.language,
            stack
                .edition_or_version
                .as_deref()
                .unwrap_or("(version inconnue)")
        );
    } else {
        eprintln!("\nContexte pour les skills :");
    }
    let line: String = Input::with_theme(&theme)
        .with_prompt(
            "  Une ligne, ou Entrée pour ouvrir $EDITOR sur un squelette prérempli",
        )
        .allow_empty(true)
        .interact_text()?;
    if !line.trim().is_empty() {
        return Ok(line);
    }
    // Entrée vide → $EDITOR
    let skeleton = context_skeleton(detected);
    match Editor::new().extension(".md").edit(&skeleton)? {
        Some(text) => Ok(text),
        None => Ok(String::new()),
    }
}

fn context_skeleton(detected: &Detected) -> String {
    let stack_hint = detected
        .stack
        .as_ref()
        .map(|s| format!("# Projet {}.", s.language))
        .unwrap_or_default();
    format!(
        "{stack_hint}
# Décris ici :
#   - conventions d'API, de nommage, d'erreur.
#   - contraintes de sécurité ou de perf spécifiques.
#   - ce que tu veux que l'agent sache avant d'écrire.
"
    )
}

fn pick_mcp_jira(detected: &Detected, interactive: bool) -> Result<Option<String>> {
    let candidates: Vec<&DetectedMcp> = detected.mcps.iter().filter(|m| mcp::matches_jira(m)).collect();
    match candidates.len() {
        0 => Ok(None),
        1 => {
            let only = candidates[0];
            let tool = mcp::tool_id(&only.name, "getJiraIssue");
            if !interactive {
                eprintln!(
                    "  ✓ MCP Jira détecté : {tool}\n    (source : {} → serveur « {} »)",
                    only.source, only.name
                );
                return Ok(Some(tool));
            }
            let theme = ColorfulTheme::default();
            let ok = Confirm::with_theme(&theme)
                .with_prompt(format!(
                    "MCP Jira détecté : {tool}\n  (source : {} → serveur « {} »)\n  Retenir ?",
                    only.source, only.name
                ))
                .default(true)
                .interact()?;
            Ok(ok.then_some(tool))
        }
        _ => {
            if !interactive {
                let first = candidates[0];
                let tool = mcp::tool_id(&first.name, "getJiraIssue");
                eprintln!("  ✓ Plusieurs MCPs Jira détectés — premier retenu : {tool}");
                return Ok(Some(tool));
            }
            let theme = ColorfulTheme::default();
            let labels: Vec<String> = candidates
                .iter()
                .map(|m| format!("{} ({})", m.name, m.source))
                .chain(std::iter::once("Aucun".to_string()))
                .collect();
            let idx = Select::with_theme(&theme)
                .with_prompt("Plusieurs MCPs Jira détectés — lequel ?")
                .items(&labels)
                .default(0)
                .interact()?;
            if idx == candidates.len() {
                Ok(None)
            } else {
                Ok(Some(mcp::tool_id(&candidates[idx].name, "getJiraIssue")))
            }
        }
    }
}

// ─────────────────────────────── tests ───────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yes_court_circuite_prompt() {
        let opts = InitOptions {
            yes: true,
            no_detect: false,
            preset: None,
        };
        assert!(!should_prompt(&opts, true));
    }

    #[test]
    fn non_tty_court_circuite_prompt() {
        let opts = InitOptions::default();
        assert!(!should_prompt(&opts, false));
    }

    #[test]
    fn tty_sans_yes_declenche_prompt() {
        let opts = InitOptions::default();
        assert!(should_prompt(&opts, true));
    }

    #[test]
    fn run_yes_sans_detection_produit_defaut_complet() {
        let opts = InitOptions {
            yes: true,
            no_detect: false,
            preset: None,
        };
        let choices = run(&Detected::empty(), &opts).unwrap();
        assert_eq!(choices.workflows.len(), 7);
        assert!(choices.workflows.contains(&"apply".to_string()));
        assert!(choices.context_addition.is_none());
        assert!(choices.jira_tool_confirmed.is_none());
    }

    #[test]
    fn run_yes_avec_preset_minimal() {
        let opts = InitOptions {
            yes: true,
            no_detect: false,
            preset: Some(Preset::Minimal),
        };
        let choices = run(&Detected::empty(), &opts).unwrap();
        assert_eq!(choices.workflows, vec!["propose", "explore", "onboard"]);
    }

    #[test]
    fn run_yes_avec_mcp_detecte_retient_automatiquement() {
        let opts = InitOptions {
            yes: true,
            no_detect: false,
            preset: None,
        };
        let detected = Detected {
            mcps: vec![DetectedMcp {
                name: "claude.ai Atlassian Rovo".into(),
                command: None,
                url: Some("https://mcp.atlassian.com/".into()),
                source: ".mcp.json".into(),
            }],
            ..Detected::empty()
        };
        let choices = run(&detected, &opts).unwrap();
        assert_eq!(
            choices.jira_tool_confirmed.as_deref(),
            Some("mcp__claude_ai_Atlassian_Rovo__getJiraIssue")
        );
    }

    #[test]
    fn run_yes_sans_mcp_ne_prerempli_rien() {
        let opts = InitOptions {
            yes: true,
            no_detect: false,
            preset: None,
        };
        let choices = run(&Detected::empty(), &opts).unwrap();
        assert!(choices.jira_tool_confirmed.is_none());
    }
}
