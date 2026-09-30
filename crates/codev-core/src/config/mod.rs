//! Generation of `_codev/config.yaml` by `codev init`.
//!
//! Two pure building blocks:
//!
//! - [`GeneratedConfig`] and its renderer [`render`] — write the YAML **by
//!   hand**, character by character, with provenance comments above the
//!   keys that carry one. No serialization library preserves comments;
//!   manual rendering is simple and covered by golden tests.
//! - [`from_detected`] — assembles a `GeneratedConfig` from the probe
//!   report plus the user's choices.
//!
//! The cross-check "what we write must be readable by
//! `codev-engine::config`" lives in the `codev-engine` integration
//! tests — avoiding a dependency cycle.

use crate::detect::Detected;

pub mod choices;

pub use choices::UserChoices;

/// A generated YAML value, with its (optional) source.
///
/// The provenance becomes a `# ...` comment on the line directly
/// above the key. No formatting: just the origin, written
/// verbatim.
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

/// The config, ready to be rendered as YAML.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedConfig {
    pub schema: String,
    /// Language of the prose skills write in artifacts (ISO 639 code).
    pub language: GeneratedValue<String>,
    /// Selected workflows, in the order they should appear in the YAML.
    pub workflows: Vec<String>,
    /// Jira MCP tool ID, if detected and confirmed.
    pub mcp_jira: Option<GeneratedValue<String>>,
    /// Free-form `context:` block. `None` = key omitted from the YAML.
    pub context: Option<GeneratedValue<String>>,
}

impl Default for GeneratedConfig {
    fn default() -> Self {
        Self {
            schema: "spec-driven".to_string(),
            language: GeneratedValue::with_source(DEFAULT_LANGUAGE.to_string(), "default"),
            workflows: Vec::new(),
            mcp_jira: None,
            context: None,
        }
    }
}

/// The artifact language when neither `--language` nor the locale gives one.
pub const DEFAULT_LANGUAGE: &str = "en";

/// A `_codev/config.yaml` is **thin** when it does not yet carry any
/// guidance useful for steering the skills. The only criterion used
/// is **the absence of `rules:`** — it is the only reliable signal of
/// user intent, because per-artifact rules are never
/// auto-detected.
///
/// The previous version combined "short `context:`" and "empty
/// `rules:`", but the context is filled automatically by the
/// `codev init` probe (stack, dependencies, license, CI), so its size
/// says nothing about what the user wrote — a TypeScript project with
/// ten dependencies easily exceeded the threshold without any user
/// intent having been expressed. See change
/// `fix-thin-detection`.
///
/// This function is the sole judge — the three hint sites
/// (`codev init`, `codev status`, the `onboard` skill) call the same
/// primitive, guaranteeing consistent behavior.
pub fn is_config_thin(rules_empty: bool) -> bool {
    rules_empty
}

/// Assembles a `GeneratedConfig` from detection results and choices.
///
/// Rules:
/// - The final context is the concatenation of the auto-detected context
///   (derived from the stack) and the user's addition, separated by a blank line.
///   If neither exists, the `context:` key is omitted.
/// - The Jira MCP tool is only kept if a candidate was confirmed
///   (decided by the imperative shell — here we take whatever `choices` carries).
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
            .map(|m| format!("detected from {} → server \"{}\"", m.source, m.name))
            .unwrap_or_else(|| "provided manually".to_string());
        GeneratedValue::with_source(tool_id.clone(), source)
    });

    let language = match (&choices.language, &detected.locale) {
        (Some(code), _) => {
            GeneratedValue::with_source(code.clone(), "set with `codev init --language`")
        }
        (None, Some(locale)) => GeneratedValue::with_source(
            locale.language.clone(),
            format!("detected from {}={}", locale.var, locale.value),
        ),
        (None, None) => GeneratedValue::with_source(
            DEFAULT_LANGUAGE.to_string(),
            "default — no locale detected",
        ),
    };

    GeneratedConfig {
        schema: "spec-driven".to_string(),
        language,
        workflows: choices.workflows.clone(),
        mcp_jira,
        context,
    }
}

fn build_context_from_detection(d: &Detected) -> Option<String> {
    let mut parts: Vec<String> = Vec::new();
    if let Some(stack) = &d.stack {
        let head = match (stack.workspace_crate_count, &stack.edition_or_version) {
            (Some(n), Some(v)) => format!(
                "{} workspace project ({} crates), {}.",
                stack.language, n, v
            ),
            (Some(n), None) => format!("{} workspace project ({} crates).", stack.language, n),
            (None, Some(v)) => format!("{} project, {}.", stack.language, v),
            (None, None) => format!("{} project.", stack.language),
        };
        parts.push(head);
        if !stack.dependencies_summary.is_empty() {
            let deps = stack.dependencies_summary.join(", ");
            parts.push(format!("Main dependencies: {deps}."));
        }
    }
    if let Some(lic) = &d.license {
        parts.push(format!("License: {lic}."));
    }
    if d.has_ci {
        parts.push("GitHub Actions CI enabled.".to_string());
    }
    if parts.is_empty() {
        None
    } else {
        Some(parts.join(" "))
    }
}

fn provenance_for_stack(d: &Detected) -> String {
    match d.stack.as_ref().map(|s| s.language.as_str()) {
        Some("Rust") => "detected from Cargo.toml".to_string(),
        Some("TypeScript" | "JavaScript") => "detected from package.json".to_string(),
        Some("Python") => "detected from pyproject.toml".to_string(),
        Some("Go") => "detected from go.mod".to_string(),
        Some("Java") => "detected from pom.xml".to_string(),
        _ => "detected from the environment".to_string(),
    }
}

/// Renders a `GeneratedConfig` as YAML, with provenance comments.
///
/// Key order is fixed: `schema`, `language`, `workflows`, `mcp`, `context`.
/// Each non-trivial key carries, on the line above it, a
/// `# <provenance>` comment if the entry has one.
///
/// Rendering is deterministic: same inputs, same bytes.
pub fn render(config: &GeneratedConfig) -> String {
    let mut out = String::new();

    out.push_str("# codev configuration — generated by `codev init`.\n");
    out.push_str("# Hand-editable: each non-trivial field carries its source as a comment.\n\n");

    // schema
    out.push_str(&format!("schema: {}\n", config.schema));

    // language
    out.push_str("\n# Language of the prose skills write in artifacts (proposal, design,\n");
    out.push_str("# tasks, specs). Structural keywords such as `## Why` or\n");
    out.push_str("# `### Requirement:` always stay in English.\n");
    if let Some(source) = &config.language.provenance {
        out.push_str(&format!("# {source}\n"));
    }
    out.push_str(&format!("language: {}\n", config.language.value));

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

    // context — YAML literal block `|`
    if let Some(ctx) = &config.context {
        out.push('\n');
        if let Some(source) = &ctx.provenance {
            out.push_str(&format!("# {source} — edit as needed\n"));
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
    use crate::detect::{locale::DetectedLocale, mcp::DetectedMcp, stack::Stack};

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
            locale: Some(DetectedLocale {
                var: "LANG".into(),
                value: "fr_FR.UTF-8".into(),
                language: "fr".into(),
            }),
        }
    }

    #[test]
    fn from_detected_full_produces_context_and_mcp() {
        let detected = workspace_rust_detected();
        let choices = UserChoices {
            workflows: vec!["propose".into(), "apply".into()],
            context_addition: Some("In-house conventions: typed errors.".into()),
            jira_tool_confirmed: Some("mcp__claude_ai_Atlassian_Rovo__getJiraIssue".to_string()),
            language: None,
        };
        let g = from_detected(&detected, &choices);
        assert_eq!(g.schema, "spec-driven");
        assert_eq!(g.workflows, vec!["propose", "apply"]);
        assert_eq!(
            g.mcp_jira.as_ref().unwrap().value,
            "mcp__claude_ai_Atlassian_Rovo__getJiraIssue"
        );
        assert!(
            g.mcp_jira
                .as_ref()
                .unwrap()
                .provenance
                .as_deref()
                .unwrap()
                .contains(".mcp.json")
        );
        let ctx = g.context.unwrap();
        assert!(ctx.value.contains("Rust workspace"));
        assert!(ctx.value.contains("In-house conventions"));
        assert_eq!(ctx.provenance.as_deref(), Some("detected from Cargo.toml"));
    }

    #[test]
    fn from_detected_empty_produces_minimal_config() {
        let detected = Detected::empty();
        let choices = UserChoices {
            workflows: vec!["propose".into(), "explore".into(), "onboard".into()],
            context_addition: None,
            jira_tool_confirmed: None,
            language: None,
        };
        let g = from_detected(&detected, &choices);
        assert_eq!(g.workflows.len(), 3);
        assert!(g.mcp_jira.is_none());
        assert!(g.context.is_none());
        assert_eq!(g.language.value, "en");
        assert_eq!(
            g.language.provenance.as_deref(),
            Some("default — no locale detected")
        );
    }

    #[test]
    fn language_comes_from_the_locale_when_not_chosen() {
        let g = from_detected(&workspace_rust_detected(), &UserChoices::defaults_full());
        assert_eq!(g.language.value, "fr");
        assert_eq!(
            g.language.provenance.as_deref(),
            Some("detected from LANG=fr_FR.UTF-8")
        );
    }

    #[test]
    fn explicit_language_wins_over_the_locale() {
        let choices = UserChoices {
            language: Some("de".into()),
            ..UserChoices::defaults_full()
        };
        let g = from_detected(&workspace_rust_detected(), &choices);
        assert_eq!(g.language.value, "de");
        assert_eq!(
            g.language.provenance.as_deref(),
            Some("set with `codev init --language`")
        );
    }

    #[test]
    fn full_defaults_install_every_workflow() {
        let full = UserChoices::defaults_full();
        assert_eq!(full.workflows.len(), 8);
        assert!(full.workflows.iter().any(|w| w == "configure"));
    }

    #[test]
    fn from_detected_mcp_only_without_free_context() {
        let detected = workspace_rust_detected();
        let choices = UserChoices {
            workflows: vec!["propose".into()],
            context_addition: None,
            jira_tool_confirmed: Some("mcp__claude_ai_Atlassian_Rovo__getJiraIssue".to_string()),
            language: None,
        };
        let g = from_detected(&detected, &choices);
        // The auto context comes on its own, without the user's addition.
        assert!(g.context.as_ref().unwrap().value.contains("Rust workspace"));
        assert!(!g.context.as_ref().unwrap().value.contains("Conventions"));
    }

    const GOLDEN_FULL: &str = "# codev configuration — generated by `codev init`.
# Hand-editable: each non-trivial field carries its source as a comment.

schema: spec-driven

# Language of the prose skills write in artifacts (proposal, design,
# tasks, specs). Structural keywords such as `## Why` or
# `### Requirement:` always stay in English.
# detected from LANG=fr_FR.UTF-8
language: fr

workflows:
  - propose
  - explore
  - onboard
  - apply
  - sync
  - archive
  - update

# detected from .mcp.json → server \"claude.ai Atlassian Rovo\"
mcp:
  jira_tool: mcp__claude_ai_Atlassian_Rovo__getJiraIssue

# detected from Cargo.toml — edit as needed
context: |
  Rust workspace project (4 crates), 2024. Main dependencies: serde, anyhow. License: MIT. GitHub Actions CI enabled.

  In-house conventions: typed errors.
";

    #[test]
    fn render_full_matches_golden() {
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
            context_addition: Some("In-house conventions: typed errors.".into()),
            jira_tool_confirmed: Some("mcp__claude_ai_Atlassian_Rovo__getJiraIssue".to_string()),
            language: None,
        };
        let g = from_detected(&detected, &choices);
        let rendered = render(&g);
        assert_eq!(rendered, GOLDEN_FULL);
    }

    const GOLDEN_MINIMAL: &str = "# codev configuration — generated by `codev init`.
# Hand-editable: each non-trivial field carries its source as a comment.

schema: spec-driven

# Language of the prose skills write in artifacts (proposal, design,
# tasks, specs). Structural keywords such as `## Why` or
# `### Requirement:` always stay in English.
# default
language: en

workflows:
  - propose
  - explore
  - onboard
";

    #[test]
    fn is_config_thin_true_when_rules_empty() {
        assert!(is_config_thin(true));
    }

    #[test]
    fn is_config_thin_false_when_rules_present() {
        assert!(!is_config_thin(false));
    }

    #[test]
    fn render_minimal_matches_golden() {
        let g = GeneratedConfig {
            schema: "spec-driven".to_string(),
            language: GeneratedValue::with_source(DEFAULT_LANGUAGE.to_string(), "default"),
            workflows: vec!["propose".into(), "explore".into(), "onboard".into()],
            mcp_jira: None,
            context: None,
        };
        assert_eq!(render(&g), GOLDEN_MINIMAL);
    }
}
