# Proposal : configurer les noms de MCP côté projet

## Pourquoi

Le change précédent (`propose-detects-jira-tickets`) a hardcodé
`mcp__claude_ai_Atlassian__getJiraIssue` dans le CATALOG. Le premier
test grandeur nature a immédiatement montré la fragilité : la même
journée, la session de Claude Code de Ludovic voyait le MCP renommé
en `mcp__claude_ai_Atlassian_Rovo__*`. Sur cet environnement, notre
skill déclarait un outil qui **n'existait plus** dans les tools
disponibles — la détection de tickets était donc silencieusement
morte.

Le problème est structurel : les noms de MCP sont **spécifiques à
l'environnement Claude Code de chaque utilisateur / organisation**.
Une skill portable ne peut pas les hardcoder — chaque projet doit
pouvoir déclarer localement quel MCP il utilise, sans que ça
demande de modifier le code source de codev.

## Ce qui change

- **Nouveau champ `mcp:` dans `_codev/config.yaml`** — un bloc
  optionnel qui déclare les MCP que le projet utilise, avec une
  entrée par usage :
  ```yaml
  mcp:
    jira_tool: mcp__claude_ai_Atlassian_Rovo__getJiraIssue
  ```
  Pour la V1, un seul champ : `jira_tool: Option<String>`. La
  structure est extensible pour de futurs MCP (Design, Confluence…).
- **`_codev/config.yaml.mcp.jira_tool`** est lu par `codev-engine::config`
  et propagé à `ResolvedConfig`.
- **CATALOG des workflows** — le `allowed_tools` de `propose` retire
  le nom hardcodé. Il contient un **placeholder** `{{JIRA_MCP_TOOL}}`
  qui sera substitué au moment du rendering du frontmatter :
  ```rust
  allowed_tools: "Bash(codev:*), Read, Write, Edit, Glob, Grep, {{JIRA_MCP_TOOL}}",
  ```
- **Le body de `propose.md`** cite lui aussi `{{JIRA_MCP_TOOL}}` là
  où il nommait le tool avant. À l'install, la substitution rend le
  body concret pour l'agent.
- **`ClaudeCode::render`** gagne un contexte `RenderCtx` qui porte
  le nom du MCP Jira (Option) et fait la substitution :
  - **`jira_tool` défini** → le placeholder est remplacé par le nom
    partout où il apparaît. Dans `allowed_tools`, la valeur est
    injectée précédée d'une virgule.
  - **`jira_tool` absent** → le placeholder est retiré proprement
    (avec la virgule qui le précède dans `allowed_tools`), et
    remplacé dans le body par la mention `(MCP Jira non configuré)`.
    La skill continue de tourner, mais la détection de tickets ne
    fera pas d'appel MCP.
- **`codev update`** lit `_codev/config.yaml` et passe le contexte
  au rendering.
- **`DEFAULT_CONFIG` du scaffold** — le nouveau `_codev/config.yaml`
  généré par `codev init` porte un bloc commenté `# mcp:` en
  exemple.
- **Test existant `propose_cite_la_detection_de_ticket_dans_son_body`**
  — adapté : vérifie la présence de `{{JIRA_MCP_TOOL}}` (placeholder)
  au lieu du nom hardcodé.
- **Test `propose_declare_le_mcp_atlassian_get_jira_issue`** —
  supprimé (l'`allowed_tools` du CATALOG ne cite plus de nom
  Atlassian) et remplacé par
  `propose_utilise_un_placeholder_pour_le_mcp_jira` qui vérifie que
  `{{JIRA_MCP_TOOL}}` est bien présent dans `allowed_tools`.
- **Test `propose_ne_declare_pas_dautre_mcp_atlassian`** — devient
  redondant (l'`allowed_tools` du CATALOG ne cite aucun MCP tout
  court) et est retiré. Remplacé par un test au niveau `render` qui
  vérifie qu'après substitution, `allowed_tools` ne contient qu'un
  seul nom de MCP au maximum.

## Capacités

### Nouvelles capacités

Aucune.

### Capacités modifiées

- `skills` — l'exigence « Skill `propose` détecte les tickets Jira
  mentionnés et enrichit le proposal » est modifiée pour refléter le
  passage du nom hardcodé à un nom configuré côté projet.

### Capacités retirées

Aucune.

## Impact

- **Code** :
  - `codev-engine::config::ProjectConfig` gagne `mcp: Option<McpConfig>`
    avec `McpConfig { jira_tool: Option<String> }`.
  - `codev-engine::config::ResolvedConfig` gagne un
    `mcp: McpConfig` (résolu, avec valeurs des sources héritées si
    présentes).
  - `codev-agents::claude::RenderCtx { jira_mcp_tool:
    Option<String> }` — nouveau type de contexte de rendu, passé à
    `ClaudeCode::render`.
  - `codev-agents::claude::render` substitue `{{JIRA_MCP_TOOL}}` dans
    `allowed_tools` et dans `body` de chaque workflow.
  - `codev-cli::commands::update` construit le `RenderCtx` depuis
    la config résolue.
- **Contrat JSON** : rien. La configuration MCP est un détail
  d'installation ; elle n'apparaît dans aucune sortie structurée.
- **Fichier écrit** : le contenu du frontmatter des SKILL.md
  installés change (nom du MCP substitué). `_codev/config.yaml`
  gagne un bloc `mcp:` si l'utilisateur le déclare.
- **Migration** :
  - **Pour ce dépôt** : ajouter `mcp: { jira_tool:
    mcp__claude_ai_Atlassian_Rovo__getJiraIssue }` à
    `_codev/config.yaml` puis relancer `codev update --force`.
  - **Pour un projet existant sans MCP branché** : le comportement
    devient bit-identique à celui d'avant `propose-detects-jira-tickets`
    (skill sans détection MCP fonctionnelle).
- **Hors périmètre** :
  - **Configuration d'autres MCP** (Design, Confluence, GitHub…) —
    la structure `mcp:` est prête, mais on n'ajoute des champs que
    quand un cas d'usage arrive. Pas de spéculation.
  - **Héritage inter-projets du champ `mcp:`** — les sources
    héritées (`inherits:`) ne propagent pas encore leur `mcp:` ;
    chaque projet déclare le sien. Reportable si le besoin apparaît.
  - **Un flag CLI pour tester un rendering** (`codev render-skill
    propose`) — utile pour debug mais reportable.
