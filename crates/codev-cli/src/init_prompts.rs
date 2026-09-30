//! Interactive prompts of `codev init`, isolated in the shell.
//!
//! Three responsibilities:
//! - decide whether a prompt should be shown (`should_prompt`) — depends on
//!   the CLI flags and the TTY state;
//! - ask the user at most two questions (workflows + context);
//! - confirm a detected Jira MCP (one more question at most, if
//!   ambiguous).
//!
//! The output is a `UserChoices` ready to feed
//! `codev-core::config::from_detected`.

use std::io::IsTerminal;

use anyhow::Result;
use codev_core::config::UserChoices;
use codev_core::detect::{
    Detected,
    mcp::{self, DetectedMcp},
};
use dialoguer::{Confirm, Editor, Input, MultiSelect, Select, theme::ColorfulTheme};

/// Preset chosen via `--preset`, or inferred from a flag.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Preset {
    Full,
    Minimal,
    Custom,
}

/// Invocation options of `codev init`.
#[derive(Debug, Clone, Default)]
pub struct InitOptions {
    /// `--yes` — skips all prompts.
    pub yes: bool,
    /// `--no-detect` — disables the probe on the caller's side; the shell
    /// then passes a `Detected::empty()` to this module.
    pub no_detect: bool,
    /// `--preset` — preselects the answer to the workflows question.
    pub preset: Option<Preset>,
    /// `--language` — overrides the language detected from the locale.
    pub language: Option<String>,
}

/// True if a prompt should be shown to the user.
///
/// Extracted so it can be tested on its own — avoids having to simulate a TTY.
pub fn should_prompt(opts: &InitOptions, stdin_is_tty: bool) -> bool {
    !opts.yes && stdin_is_tty
}

const WORKFLOWS_FULL: &[&str] = &[
    "propose",
    "explore",
    "onboard",
    "apply",
    "sync",
    "archive",
    "update",
    "configure",
];
const WORKFLOWS_MINIMAL: &[&str] = &["propose", "explore", "onboard", "configure"];

/// Runs the prompts and returns the final choices.
///
/// In non-interactive mode (`--yes` or non-TTY stdin), no prompt is shown;
/// the defaults apply according to the preset (or "Full" if no preset was
/// requested).
pub fn run(detected: &Detected, opts: &InitOptions) -> Result<UserChoices> {
    let interactive = should_prompt(opts, std::io::stdin().is_terminal());

    // Question 1 — workflows
    let workflows = pick_workflows(opts, interactive)?;

    // Question 2 — free-form context
    let context_addition = if interactive {
        Some(prompt_context(detected)?)
    } else {
        None
    };

    // MCP — confirmation (interactive) or automatic (--yes / non-TTY)
    let jira_tool_confirmed = pick_mcp_jira(detected, interactive)?;

    Ok(UserChoices {
        workflows,
        context_addition,
        jira_tool_confirmed,
        language: opts.language.clone(),
    })
}

fn pick_workflows(opts: &InitOptions, interactive: bool) -> Result<Vec<String>> {
    let preset = match opts.preset {
        Some(p) => Some(p),
        None if !interactive => Some(Preset::Full),
        None => None,
    };

    if let Some(p) = preset {
        return Ok(match p {
            Preset::Full => WORKFLOWS_FULL.iter().map(|s| s.to_string()).collect(),
            Preset::Minimal => WORKFLOWS_MINIMAL.iter().map(|s| s.to_string()).collect(),
            Preset::Custom if !interactive => {
                // Without a TTY, `--preset custom` falls back to Full rather
                // than to an undefined state.
                WORKFLOWS_FULL.iter().map(|s| s.to_string()).collect()
            }
            Preset::Custom => pick_workflows_custom()?,
        });
    }

    let theme = ColorfulTheme::default();
    let choice = Select::with_theme(&theme)
        .with_prompt("Which workflows should be installed?")
        .items(&[
            "Full (8) — propose, explore, onboard, apply, sync, archive, update, configure",
            "Minimal (4) — propose, explore, onboard, configure",
            "Custom — pick them one by one",
        ])
        .default(0)
        .interact()?;
    match choice {
        0 => Ok(WORKFLOWS_FULL.iter().map(|s| s.to_string()).collect()),
        1 => Ok(WORKFLOWS_MINIMAL.iter().map(|s| s.to_string()).collect()),
        _ => pick_workflows_custom(),
    }
}

fn pick_workflows_custom() -> Result<Vec<String>> {
    let theme = ColorfulTheme::default();
    let indices = MultiSelect::with_theme(&theme)
        .with_prompt("Select the workflows (Space to toggle, Enter to confirm)")
        .items(WORKFLOWS_FULL)
        .defaults(&[true; 8])
        .interact()?;
    Ok(indices
        .into_iter()
        .map(|i| WORKFLOWS_FULL[i].to_string())
        .collect())
}

fn prompt_context(detected: &Detected) -> Result<String> {
    let theme = ColorfulTheme::default();
    if let Some(stack) = &detected.stack {
        eprintln!("\nContext for the skills (what cannot be inferred from the code):");
        eprintln!(
            "  detected stack: {} {}",
            stack.language,
            stack
                .edition_or_version
                .as_deref()
                .unwrap_or("(unknown version)")
        );
    } else {
        eprintln!("\nContext for the skills:");
    }
    let line: String = Input::with_theme(&theme)
        .with_prompt("  One line, or Enter to open $EDITOR on a prefilled skeleton")
        .allow_empty(true)
        .interact_text()?;
    if !line.trim().is_empty() {
        return Ok(line);
    }
    // Empty input → $EDITOR
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
        .map(|s| format!("# {} project.", s.language))
        .unwrap_or_default();
    format!(
        "{stack_hint}
# Describe here:
#   - API, naming and error-handling conventions.
#   - specific security or performance constraints.
#   - what you want the agent to know before writing.
"
    )
}

fn pick_mcp_jira(detected: &Detected, interactive: bool) -> Result<Option<String>> {
    let candidates: Vec<&DetectedMcp> = detected
        .mcps
        .iter()
        .filter(|m| mcp::matches_jira(m))
        .collect();
    match candidates.len() {
        0 => Ok(None),
        1 => {
            let only = candidates[0];
            let tool = mcp::tool_id(&only.name, "getJiraIssue");
            if !interactive {
                eprintln!(
                    "  ✓ Jira MCP detected: {tool}\n    (source: {} → server \"{}\")",
                    only.source, only.name
                );
                return Ok(Some(tool));
            }
            let theme = ColorfulTheme::default();
            let ok = Confirm::with_theme(&theme)
                .with_prompt(format!(
                    "Jira MCP detected: {tool}\n  (source: {} → server \"{}\")\n  Use it?",
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
                eprintln!("  ✓ Several Jira MCPs detected — using the first one: {tool}");
                return Ok(Some(tool));
            }
            let theme = ColorfulTheme::default();
            let labels: Vec<String> = candidates
                .iter()
                .map(|m| format!("{} ({})", m.name, m.source))
                .chain(std::iter::once("None".to_string()))
                .collect();
            let idx = Select::with_theme(&theme)
                .with_prompt("Several Jira MCPs detected — which one?")
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
    fn yes_skips_the_prompt() {
        let opts = InitOptions {
            yes: true,
            no_detect: false,
            preset: None,
            language: None,
        };
        assert!(!should_prompt(&opts, true));
    }

    #[test]
    fn non_tty_skips_the_prompt() {
        let opts = InitOptions::default();
        assert!(!should_prompt(&opts, false));
    }

    #[test]
    fn tty_without_yes_triggers_the_prompt() {
        let opts = InitOptions::default();
        assert!(should_prompt(&opts, true));
    }

    #[test]
    fn run_yes_without_detection_yields_the_full_default() {
        let opts = InitOptions {
            yes: true,
            no_detect: false,
            preset: None,
            language: None,
        };
        let choices = run(&Detected::empty(), &opts).unwrap();
        assert_eq!(choices.workflows.len(), 8);
        assert!(choices.workflows.contains(&"apply".to_string()));
        assert!(choices.workflows.contains(&"configure".to_string()));
        assert!(choices.context_addition.is_none());
        assert!(choices.jira_tool_confirmed.is_none());
    }

    #[test]
    fn run_yes_with_minimal_preset() {
        let opts = InitOptions {
            yes: true,
            no_detect: false,
            preset: Some(Preset::Minimal),
            language: None,
        };
        let choices = run(&Detected::empty(), &opts).unwrap();
        assert_eq!(
            choices.workflows,
            vec!["propose", "explore", "onboard", "configure"]
        );
    }

    #[test]
    fn run_yes_with_detected_mcp_accepts_it_automatically() {
        let opts = InitOptions {
            yes: true,
            no_detect: false,
            preset: None,
            language: None,
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
    fn run_yes_without_mcp_prefills_nothing() {
        let opts = InitOptions {
            yes: true,
            no_detect: false,
            preset: None,
            language: None,
        };
        let choices = run(&Detected::empty(), &opts).unwrap();
        assert!(choices.jira_tool_confirmed.is_none());
    }
}
