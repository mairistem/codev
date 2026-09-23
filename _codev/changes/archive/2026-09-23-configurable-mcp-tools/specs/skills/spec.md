## MODIFIED Requirements

### Requirement: Skill `propose` détecte les tickets Jira mentionnés et enrichit le proposal

Le workflow `propose` du catalogue SHALL, en amont de la résolution du
nom de change, scanner le prompt de l'utilisateur pour repérer un
identifiant de ticket qui matche le pattern régulier `[A-Z]{2,}-\d+`
(par exemple `JVS-1234`, `PROJ-42`).

Sur détection d'au moins un ticket, la skill MUST :

1. **Tenter d'appeler** l'outil MCP dont le nom est déclaré côté
   projet via `_codev/config.yaml.mcp.jira_tool`. Ce nom est injecté
   au moment de l'installation de la skill dans le frontmatter
   `allowed-tools` et dans le body — l'utilisateur/organisation
   choisit **son** MCP (`mcp__claude_ai_Atlassian__getJiraIssue`,
   `mcp__claude_ai_Atlassian_Rovo__getJiraIssue`, ou un autre) sans
   que ça touche au code source de codev.
2. **Un seul appel par invocation**, sur l'identifiant du ticket
   **le plus tôt mentionné dans le prompt** ; les autres sont juste
   nommés.
3. **En cas de succès** : injecter le contenu (titre, description,
   status, type) dans le contexte de rédaction, et faire figurer le
   ticket en tête du `proposal.md` sous forme d'une ligne citation
   `> Source : ticket **<ID>** — « <titre> » (<status>)`.
4. **En cas d'échec — outil MCP indisponible dans la session ou
   non configuré côté projet** : afficher un message informatif à
   l'utilisateur, puis continuer le flow habituel sans le contenu
   du ticket. Le proposal cite quand même le ticket en tête (« Source
   : ticket **<ID>** — contenu non récupéré »).
5. **En l'absence de pattern** : comportement bit-identique à
   aujourd'hui — aucun appel MCP, aucun message.

Le CATALOG des workflows MUST utiliser un placeholder textuel
`{{JIRA_MCP_TOOL}}` dans `allowed_tools` et dans le body du
workflow `propose`, en lieu et place d'un nom de MCP hardcodé. Le
placeholder est substitué au moment du rendering du frontmatter
d'installation par `ClaudeCode::render`.

Quand `_codev/config.yaml.mcp.jira_tool` est **absent** ou vide, le
rendering MUST :

- retirer proprement `{{JIRA_MCP_TOOL}}` de `allowed_tools` **et** la
  virgule qui le précède (pour ne pas laisser un `allowed-tools`
  malformé qui se terminerait par `", "`) ;
- remplacer chaque occurrence dans le body par la chaîne
  `(MCP Jira non configuré)` — la skill reste installée et
  fonctionnelle sur les autres aspects, mais n'appelle plus de MCP.

#### Scenario: Configuration `mcp.jira_tool` présente → skill fonctionnelle

- **GIVEN** un projet dont `_codev/config.yaml` déclare
  `mcp: { jira_tool: mcp__claude_ai_Atlassian_Rovo__getJiraIssue }`
- **WHEN** l'utilisateur lance `codev update`
- **THEN** le fichier `.claude/skills/codev-propose/SKILL.md` porte
  `allowed-tools: "Bash(codev:*), Read, Write, Edit, Glob, Grep, mcp__claude_ai_Atlassian_Rovo__getJiraIssue"`
- **AND** le body de la skill cite
  `mcp__claude_ai_Atlassian_Rovo__getJiraIssue` là où le CATALOG
  contient `{{JIRA_MCP_TOOL}}`
- **AND** aucun `{{…}}` ne subsiste dans le fichier installé

#### Scenario: Configuration `mcp.jira_tool` absente → fallback propre

- **GIVEN** un projet dont `_codev/config.yaml` n'a pas de bloc `mcp:`
- **WHEN** l'utilisateur lance `codev update`
- **THEN** le fichier `.claude/skills/codev-propose/SKILL.md` porte
  `allowed-tools: "Bash(codev:*), Read, Write, Edit, Glob, Grep"`
  (sans le placeholder et sans virgule finale)
- **AND** le body remplace `{{JIRA_MCP_TOOL}}` par la mention
  `(MCP Jira non configuré)`
- **AND** aucun `{{…}}` ne subsiste dans le fichier installé

#### Scenario: Ticket mentionné, MCP configuré et disponible → ticket cité en tête

- **GIVEN** un utilisateur tape `/codev-propose ajouter JWT pour JVS-1234`
- **AND** le projet a configuré `mcp.jira_tool:
  mcp__claude_ai_Atlassian_Rovo__getJiraIssue`
- **AND** ce MCP est disponible dans la session Claude Code
- **AND** le ticket `JVS-1234` existe
- **WHEN** la skill s'exécute
- **THEN** un appel `mcp__claude_ai_Atlassian_Rovo__getJiraIssue({issueIdOrKey: "JVS-1234"})`
  est émis
- **AND** le `proposal.md` du change créé contient en tête une ligne
  `> Source : ticket **JVS-1234** — « <titre> » (<status>)`

#### Scenario: Ticket mentionné, MCP absent → proposal quand même

- **GIVEN** un utilisateur tape `/codev-propose corriger JVS-1234`
- **AND** aucun MCP Jira n'est configuré (`mcp.jira_tool` absent ou
  outil déclaré indisponible)
- **WHEN** la skill s'exécute
- **THEN** la skill affiche un message informatif nommant
  `JVS-1234` et indiquant que le MCP Jira n'est pas actif
- **AND** le proposal est créé quand même, en tête duquel
  `JVS-1234` est mentionné avec « contenu non récupéré »

#### Scenario: Aucun ticket mentionné → comportement inchangé

- **GIVEN** un utilisateur tape `/codev-propose ajouter
  l'authentification`
- **WHEN** la skill s'exécute
- **THEN** aucun appel MCP n'est émis
- **AND** aucun message relatif à un ticket n'apparaît
- **AND** le proposal est rédigé exactement comme avant ce lot
