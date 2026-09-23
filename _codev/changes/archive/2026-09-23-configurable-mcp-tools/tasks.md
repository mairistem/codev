# Tâches

## 1. Config projet — nouveau champ `mcp:`

- [x] 1.1 Ajouter la struct `McpConfig` dans
      `codev-engine::config` :
      ```rust
      #[derive(Debug, Clone, Default, Deserialize)]
      #[serde(deny_unknown_fields)]
      pub struct McpConfig {
          #[serde(default)]
          pub jira_tool: Option<String>,
      }
      ```
- [x] 1.2 Ajouter le champ `pub mcp: McpConfig` (avec
      `#[serde(default)]`) à `ProjectConfig`.
- [x] 1.3 Ajouter le champ `pub mcp: McpConfig` à `ResolvedConfig`,
      alimenté par le projet uniquement (pas de fusion héritée).
- [x] 1.4 Tests dans `config` : parse un YAML avec `mcp:` → valeur
      correcte ; parse un YAML sans `mcp:` → défaut vide ; parse un
      YAML avec un champ inconnu dans `mcp:` → erreur `serde`.

## 2. Rendering des skills — placeholder + substitution

- [x] 2.1 Ajouter `pub struct RenderCtx { pub jira_mcp_tool:
      Option<String> }` dans `codev-agents::claude` (ou module dédié
      si plus propre).
- [x] 2.2 Étendre `ClaudeCode::render` pour accepter un
      `&RenderCtx` en argument. Signature avant :
      `render(workflow: &Workflow, version: &str)`. Signature après :
      `render(workflow: &Workflow, version: &str, ctx: &RenderCtx)`.
- [x] 2.3 Nouvelle fonction interne `substitute_jira_mcp(source:
      &str, jira_tool: Option<&str>) -> String` qui applique la
      règle du design (retire `, {{JIRA_MCP_TOOL}}` avant fallback).
      Appelée sur `allowed_tools` **et** `body` avant assemblage.
- [x] 2.4 Tests dédiés sur `substitute_jira_mcp` :
      - Some(tool) → `{{JIRA_MCP_TOOL}}` remplacé par le nom.
      - None + placeholder précédé de `, ` dans `allowed_tools` →
        placeholder ET virgule retirés.
      - None + placeholder seul dans le body → remplacé par
        `(MCP Jira non configuré)`.
      - Aucun placeholder dans la source → source inchangée.

## 3. CATALOG — utilise le placeholder

- [x] 3.1 Dans `codev-agents::workflows::CATALOG`, entrée `propose` :
      `allowed_tools: "Bash(codev:*), Read, Write, Edit, Glob, Grep, {{JIRA_MCP_TOOL}}"`.
      Retirer la mention hardcodée de
      `mcp__claude_ai_Atlassian__getJiraIssue`.
- [x] 3.2 Éditer `assets/workflows/propose.md` : partout où le nom
      `mcp__claude_ai_Atlassian__getJiraIssue` est cité, le
      remplacer par `{{JIRA_MCP_TOOL}}`.
- [x] 3.3 Actualiser le doc-comment de l'entrée `propose` — ne plus
      dire « MCP Atlassian » spécifiquement, mais « le MCP Jira
      configuré côté projet ».

## 4. Tests d'invariants — adapter et remplacer

- [x] 4.1 Retirer les tests obsolètes :
      `propose_declare_le_mcp_atlassian_get_jira_issue`,
      `propose_ne_declare_pas_dautre_mcp_atlassian`. Justification :
      l'`allowed_tools` du CATALOG ne cite plus aucun nom MCP.
- [x] 4.2 Nouveau test
      `propose_utilise_un_placeholder_pour_le_mcp_jira` :
      `find("propose").unwrap().allowed_tools.contains("{{JIRA_MCP_TOOL}}")`.
- [x] 4.3 Adapter `propose_cite_la_detection_de_ticket_dans_son_body`
      pour vérifier la présence de `{{JIRA_MCP_TOOL}}` dans le body
      (au lieu du nom hardcodé).
- [x] 4.4 Nouveau test dans `codev-agents::claude::tests` :
      `render_avec_jira_tool_configure_substitue_partout` — vérifie
      qu'après `render` avec `RenderCtx { jira_mcp_tool:
      Some("mcp__x__y".into()) }`, ni `{{JIRA_MCP_TOOL}}` ni
      `(MCP Jira non configuré)` ne subsistent, et que le nom
      apparaît bien.
- [x] 4.5 Nouveau test
      `render_sans_jira_tool_retire_placeholder_et_virgule` : après
      `render` avec `jira_mcp_tool: None`, l'`allowed_tools` du
      rendu se termine par `Grep"` (sans virgule ni MCP), et le body
      contient `(MCP Jira non configuré)`.

## 5. Wiring — le CLI passe le contexte

- [x] 5.1 `codev-cli::commands::update` construit un `RenderCtx`
      depuis `ResolvedConfig.mcp.jira_tool` et le passe à
      `ClaudeCode::render`.
- [x] 5.2 `codev-cli::commands::init` fait la même chose (dans le
      chemin de scaffolding qui installe les skills).
- [x] 5.3 Test d'intégration `init_installe_skill_avec_mcp_configure` :
      dans un `Harnais::neuf` qui écrit un `config.yaml` avec `mcp:`,
      lancer `init`, vérifier que le SKILL.md installé porte le bon
      nom.
- [x] 5.4 Test d'intégration
      `init_installe_skill_sans_mcp_configure` : sans bloc `mcp:`,
      le SKILL.md installé porte `allowed-tools` sans le suffixe MCP
      et un body avec la mention « (MCP Jira non configuré) ».

## 6. Scaffold — exemple commenté

- [x] 6.1 `DEFAULT_CONFIG` de `codev-engine::scaffold` gagne un bloc
      commenté d'exemple :
      ```yaml
      # Nom de l'outil MCP Jira à utiliser quand un ticket est
      # mentionné dans le prompt d'une skill. Décommenter et
      # remplacer par le nom exact du MCP disponible dans ta
      # session Claude Code.
      # mcp:
      #   jira_tool: mcp__claude_ai_Atlassian_Rovo__getJiraIssue
      ```
      Placé après le bloc `inherits:` commenté.

## 7. Migration du dépôt

- [x] 7.1 Éditer `_codev/config.yaml` du dépôt — ajouter :
      ```yaml
      mcp:
        jira_tool: mcp__claude_ai_Atlassian_Rovo__getJiraIssue
      ```
- [x] 7.2 `cargo install --path crates/codev-cli`
- [x] 7.3 `codev update --force` — le SKILL.md de propose porte
      maintenant le nom Rovo.
- [x] 7.4 Vérification à la main : `head -5
      .claude/skills/codev-propose/SKILL.md` — l'`allowed-tools`
      contient bien `mcp__claude_ai_Atlassian_Rovo__getJiraIssue`,
      et aucun `{{…}}` ne subsiste.

## 8. Intégration workspace

- [x] 8.1 `cargo test --workspace` reste vert, gagne au moins 8
      tests nouveaux (config + substitute + render + intégration
      init).
- [x] 8.2 `cargo clippy --workspace --all-targets` reste sans
      avertissement.
- [x] 8.3 `codev validate --strict --all` reste vert.
