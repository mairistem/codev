//! Detection of MCP servers configured at the project or user level.
//!
//! Pure building blocks here:
//! - `parse_mcp_config` reads a JSON file containing `mcpServers` and returns
//!   the list of declared servers, with their source.
//! - `matches_jira` filters the candidates likely to be a Jira MCP.
//! - `tool_id` computes the tool identifier Claude Code will expose, from
//!   the server name and the tool name. This is the deterministic
//!   convention used to wire up `mcp.jira_tool:`.

use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DetectedMcp {
    /// The MCP server name as written in `mcpServers`.
    pub name: String,
    /// The server's stdio command, if present.
    pub command: Option<String>,
    /// The server's HTTP/SSE URL, if present.
    pub url: Option<String>,
    /// The file the entry comes from (a user-readable path).
    pub source: String,
}

#[derive(Deserialize)]
struct McpFile {
    #[serde(rename = "mcpServers")]
    mcp_servers: Option<serde_json::Map<String, serde_json::Value>>,
}

/// Parses an MCP config file in the Claude Code format. The file may
/// carry other fields — only `mcpServers` is extracted.
///
/// Returns an empty list if the file is unreadable or has no
/// `mcpServers` key — the probe carries on.
pub fn parse_mcp_config(bytes: &[u8], source: &str) -> Vec<DetectedMcp> {
    let Ok(parsed) = serde_json::from_slice::<McpFile>(bytes) else {
        return Vec::new();
    };
    let Some(servers) = parsed.mcp_servers else {
        return Vec::new();
    };
    servers
        .into_iter()
        .map(|(name, spec)| {
            let command = spec
                .get("command")
                .and_then(|v| v.as_str())
                .map(String::from);
            let url = spec.get("url").and_then(|v| v.as_str()).map(String::from);
            DetectedMcp {
                name,
                command,
                url,
                source: source.to_string(),
            }
        })
        .collect()
}

/// True if the server looks like a Jira / Atlassian MCP: case-insensitive
/// matching on name, command or URL.
pub fn matches_jira(mcp: &DetectedMcp) -> bool {
    matches_regex_jira(&mcp.name)
        || mcp.command.as_deref().is_some_and(matches_regex_jira)
        || mcp.url.as_deref().is_some_and(matches_regex_jira)
}

fn matches_regex_jira(s: &str) -> bool {
    let lower = s.to_ascii_lowercase();
    lower.contains("jira") || lower.contains("atlassian")
}

/// Claude Code convention: `mcp__<name_normalized>__<tool_suffix>`.
///
/// Normalization replaces spaces and dots with `_`; all other
/// characters are preserved (Claude Code accepts uppercase letters).
///
/// Example: `("claude.ai Atlassian Rovo", "getJiraIssue")` →
/// `"mcp__claude_ai_Atlassian_Rovo__getJiraIssue"`.
pub fn tool_id(server_name: &str, tool_suffix: &str) -> String {
    let normalized: String = server_name
        .chars()
        .map(|c| if c == ' ' || c == '.' { '_' } else { c })
        .collect();
    format!("mcp__{normalized}__{tool_suffix}")
}

/// Merges several lists, preserving order and deduplicating by
/// name — the first occurrence wins. This is what enforces "project
/// wins over global" when the orchestrator passes the lists in the right
/// order.
pub fn merge_first_wins(sources: Vec<Vec<DetectedMcp>>) -> Vec<DetectedMcp> {
    let mut seen = std::collections::BTreeSet::new();
    let mut out = Vec::new();
    for list in sources {
        for mcp in list {
            if seen.insert(mcp.name.clone()) {
                out.push(mcp);
            }
        }
    }
    out
}

// ─────────────────────────────── tests ───────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    const ATLASSIAN_ROVO: &str = r#"{
        "mcpServers": {
            "claude.ai Atlassian Rovo": {
                "url": "https://mcp.atlassian.com/xyz"
            }
        }
    }"#;

    const ATLASSIAN_CLASSIC: &str = r#"{
        "mcpServers": {
            "claude.ai Atlassian": {
                "url": "https://mcp.atlassian.com/xyz"
            }
        }
    }"#;

    const NON_JIRA: &str = r#"{
        "mcpServers": {
            "GitHub": { "command": "gh-mcp", "args": [] },
            "Figma": { "url": "https://mcp.figma.com/" }
        }
    }"#;

    const MULTI: &str = r#"{
        "mcpServers": {
            "GitHub": { "command": "gh-mcp" },
            "claude.ai Atlassian Rovo": { "url": "https://mcp.atlassian.com/" }
        }
    }"#;

    #[test]
    fn atlassian_rovo_matches() {
        let list = parse_mcp_config(ATLASSIAN_ROVO.as_bytes(), ".mcp.json");
        assert_eq!(list.len(), 1);
        assert!(matches_jira(&list[0]));
        assert_eq!(list[0].source, ".mcp.json");
    }

    #[test]
    fn atlassian_classic_matches() {
        let list = parse_mcp_config(ATLASSIAN_CLASSIC.as_bytes(), "~/.claude.json");
        assert!(matches_jira(&list[0]));
    }

    #[test]
    fn non_jira_does_not_match() {
        let list = parse_mcp_config(NON_JIRA.as_bytes(), ".mcp.json");
        assert_eq!(list.len(), 2);
        assert!(list.iter().all(|m| !matches_jira(m)));
    }

    #[test]
    fn multi_servers_matches_only_jira() {
        let list = parse_mcp_config(MULTI.as_bytes(), ".mcp.json");
        let jira_only: Vec<_> = list.iter().filter(|m| matches_jira(m)).collect();
        assert_eq!(jira_only.len(), 1);
        assert_eq!(jira_only[0].name, "claude.ai Atlassian Rovo");
    }

    #[test]
    fn unreadable_json_returns_empty() {
        assert!(parse_mcp_config(b"not json", ".mcp.json").is_empty());
    }

    #[test]
    fn file_without_mcp_servers_returns_empty() {
        assert!(parse_mcp_config(br#"{"other": "data"}"#, ".mcp.json").is_empty());
    }

    #[test]
    fn tool_id_normalizes_spaces_and_dots() {
        assert_eq!(
            tool_id("claude.ai Atlassian Rovo", "getJiraIssue"),
            "mcp__claude_ai_Atlassian_Rovo__getJiraIssue"
        );
        assert_eq!(
            tool_id("claude.ai Atlassian", "getJiraIssue"),
            "mcp__claude_ai_Atlassian__getJiraIssue"
        );
        assert_eq!(tool_id("GitHub", "search"), "mcp__GitHub__search");
    }

    #[test]
    fn merge_first_wins_on_duplicate_name() {
        let project = vec![DetectedMcp {
            name: "Atlassian".into(),
            command: None,
            url: Some("https://project".into()),
            source: ".mcp.json".into(),
        }];
        let global = vec![DetectedMcp {
            name: "Atlassian".into(),
            command: None,
            url: Some("https://global".into()),
            source: "~/.claude.json".into(),
        }];
        let merged = merge_first_wins(vec![project, global]);
        assert_eq!(merged.len(), 1);
        assert_eq!(merged[0].url.as_deref(), Some("https://project"));
        assert_eq!(merged[0].source, ".mcp.json");
    }
}
