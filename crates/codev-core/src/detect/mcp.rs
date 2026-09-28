//! Détection des MCPs configurés côté projet ou utilisateur.
//!
//! Deux briques pures ici :
//! - `parse_mcp_config` lit un fichier JSON contenant `mcpServers` et retourne
//!   la liste des serveurs déclarés, avec leur source.
//! - `matches_jira` filtre les candidats susceptibles d'être un MCP Jira.
//! - `tool_id` calcule l'identifiant d'outil que Claude Code exposera, à
//!   partir du nom du serveur et du nom du tool. C'est la convention
//!   déterministe utilisée pour brancher `mcp.jira_tool:`.

use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DetectedMcp {
    /// Le nom du serveur MCP tel qu'écrit dans `mcpServers`.
    pub name: String,
    /// La commande stdio du serveur, si présente.
    pub command: Option<String>,
    /// L'URL HTTP/SSE du serveur, si présente.
    pub url: Option<String>,
    /// Le fichier d'où provient l'entrée (chemin lisible pour l'utilisateur).
    pub source: String,
}

#[derive(Deserialize)]
struct McpFile {
    #[serde(rename = "mcpServers")]
    mcp_servers: Option<serde_json::Map<String, serde_json::Value>>,
}

/// Parse un fichier de config MCP au format Claude Code. Le fichier peut
/// porter d'autres champs — on n'extrait que `mcpServers`.
///
/// Retourne une liste vide si le fichier est illisible ou n'a pas de clé
/// `mcpServers` — la sonde continue.
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

/// Vrai si le serveur ressemble à un MCP Jira / Atlassian : matching
/// insensible à la casse sur nom, commande ou URL.
pub fn matches_jira(mcp: &DetectedMcp) -> bool {
    matches_regex_jira(&mcp.name)
        || mcp.command.as_deref().is_some_and(matches_regex_jira)
        || mcp.url.as_deref().is_some_and(matches_regex_jira)
}

fn matches_regex_jira(s: &str) -> bool {
    let lower = s.to_ascii_lowercase();
    lower.contains("jira") || lower.contains("atlassian")
}

/// Convention Claude Code : `mcp__<name_normalized>__<tool_suffix>`.
///
/// La normalisation remplace espaces et points par `_` ; les autres
/// caractères sont préservés (Claude Code accepte les majuscules).
///
/// Exemple : `("claude.ai Atlassian Rovo", "getJiraIssue")` →
/// `"mcp__claude_ai_Atlassian_Rovo__getJiraIssue"`.
pub fn tool_id(server_name: &str, tool_suffix: &str) -> String {
    let normalized: String = server_name
        .chars()
        .map(|c| if c == ' ' || c == '.' { '_' } else { c })
        .collect();
    format!("mcp__{normalized}__{tool_suffix}")
}

/// Fusion de plusieurs listes en gardant l'ordre et en dédoublonnant par
/// nom — la première occurrence gagne. C'est ce qui applique « projet
/// gagne sur global » quand l'orchestrateur passe les listes dans le bon
/// ordre.
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
    fn atlassian_rovo_matche() {
        let list = parse_mcp_config(ATLASSIAN_ROVO.as_bytes(), ".mcp.json");
        assert_eq!(list.len(), 1);
        assert!(matches_jira(&list[0]));
        assert_eq!(list[0].source, ".mcp.json");
    }

    #[test]
    fn atlassian_classique_matche() {
        let list = parse_mcp_config(ATLASSIAN_CLASSIC.as_bytes(), "~/.claude.json");
        assert!(matches_jira(&list[0]));
    }

    #[test]
    fn non_jira_ne_matche_pas() {
        let list = parse_mcp_config(NON_JIRA.as_bytes(), ".mcp.json");
        assert_eq!(list.len(), 2);
        assert!(list.iter().all(|m| !matches_jira(m)));
    }

    #[test]
    fn multi_serveurs_matche_seulement_jira() {
        let list = parse_mcp_config(MULTI.as_bytes(), ".mcp.json");
        let jira_only: Vec<_> = list.iter().filter(|m| matches_jira(m)).collect();
        assert_eq!(jira_only.len(), 1);
        assert_eq!(jira_only[0].name, "claude.ai Atlassian Rovo");
    }

    #[test]
    fn json_illisible_retourne_vide() {
        assert!(parse_mcp_config(b"pas du json", ".mcp.json").is_empty());
    }

    #[test]
    fn fichier_sans_mcp_servers_retourne_vide() {
        assert!(parse_mcp_config(br#"{"other": "data"}"#, ".mcp.json").is_empty());
    }

    #[test]
    fn tool_id_normalise_espaces_et_points() {
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
    fn merge_premier_gagne_sur_doublon_de_nom() {
        let projet = vec![DetectedMcp {
            name: "Atlassian".into(),
            command: None,
            url: Some("https://projet".into()),
            source: ".mcp.json".into(),
        }];
        let global = vec![DetectedMcp {
            name: "Atlassian".into(),
            command: None,
            url: Some("https://global".into()),
            source: "~/.claude.json".into(),
        }];
        let merged = merge_first_wins(vec![projet, global]);
        assert_eq!(merged.len(), 1);
        assert_eq!(merged[0].url.as_deref(), Some("https://projet"));
        assert_eq!(merged[0].source, ".mcp.json");
    }
}
