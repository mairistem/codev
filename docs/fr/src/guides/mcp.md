# Jira et autres serveurs MCP

Les skills de codev peuvent puiser du contexte dans des systèmes externes via
les serveurs MCP connectés à votre session Claude Code. La CLI elle-même ne
communique jamais avec eux : ce sont les skills qui détectent dans votre
demande ce dont elles ont besoin et qui appellent directement l'outil MCP.

Une intégration est disponible aujourd'hui : `/codev-propose` lit un ticket
Jira.

## Jira

Lorsque votre demande à `/codev-propose` contient un identifiant de ticket —
toute chaîne correspondant à `[A-Z]{2,}-\d+`, comme `PROJ-123` — et qu'un
outil MCP Jira est configuré, la skill :

1. appelle une seule fois l'outil configuré, pour le premier identifiant
   trouvé ;
2. utilise le titre, la description, le statut et le type du ticket comme
   contexte pour le plan ;
3. cite le ticket en tête de la proposal :

   ```markdown
   # Proposal: Ajouter un mode sombre

   > Source: ticket **PROJ-123** — "Mode sombre pour le tableau de bord" (In Progress)

   ## Why
   ```

L'intégration est strictement en lecture seule : un appel à l'unique outil
configuré, jamais de recherche, de commentaire ni de transition. Si la demande
mentionne plusieurs tickets, seul le premier est récupéré ; les autres sont
listés sous la citation, pour la traçabilité.

Si un ticket est mentionné mais que le serveur MCP n'est pas disponible dans la
session, la skill le signale, rédige la proposal à partir de votre seule
demande et cite le ticket avec la mention « contenu non récupéré ».

## Configurer l'intégration

Le nom de l'outil Jira dépend de la façon dont le serveur MCP est enregistré
dans votre installation de Claude Code ; il se configure donc projet par
projet :

```yaml
# _codev/config.yaml
mcp:
  jira_tool: mcp__atlassian__getJiraIssue
```

`codev init` le renseigne pour vous lorsqu'il trouve, dans `.mcp.json`,
`.claude/settings.json` ou `~/.claude.json`, un serveur dont le nom, la
commande ou l'URL mentionne Jira ou Atlassian :

```text
  ✓ Jira MCP detected: mcp__atlassian__getJiraIssue
    (source: .mcp.json → server "atlassian")
```

Le nom de l'outil suit la convention de Claude Code,
`mcp__<server name>__getJiraIssue`, où les espaces et les points du nom du
serveur sont remplacés par des tirets bas — par exemple, un connecteur nommé
`claude.ai Atlassian Rovo` donne
`mcp__claude_ai_Atlassian_Rovo__getJiraIssue`. Si le vôtre diffère, cherchez
le nom exact dans la liste des outils d'une session Claude Code et
renseignez-le à la main.

À l'installation, codev ajoute l'outil aux `allowed-tools` de
`codev-propose` : la skill peut ainsi l'appeler, et rien d'autre sur ce
serveur. Après avoir modifié `mcp:`, régénérez les skills :

```bash
codev update --force
```

Sans `mcp.jira_tool`, la détection de tickets reste inactive : la skill se
comporte exactement comme dans un projet sans intégration MCP.

> **Remarque**
> `mcp:` n'est jamais repris d'une [source héritée](inherited-sources.md) :
> les noms d'outils dépendent de l'installation de Claude Code de chacun, et
> non d'un dépôt partagé.

## Ajouter une autre intégration MCP

Les intégrations MCP font partie de codev lui-même : chacune consiste en une
clé de configuration et un emplacement réservé dans un workflow. En ajouter une
— par exemple pour une page Confluence ou un outil de design — relève donc de
la contribution à codev, et non du paramétrage. Le principe est le même que
pour Jira :

1. Ajoutez une clé sous `mcp:` dans la configuration du projet
   (`crates/codev-engine/src/config.rs`).
2. Transmettez-la au contexte de rendu des skills et substituez un emplacement
   réservé comme `{{JIRA_MCP_TOOL}}` à la fois dans les `allowed-tools` et dans
   le corps du workflow qui l'utilise (`crates/codev-agents/src/claude.rs`).
3. Décrivez la détection et l'appel dans le fichier du workflow sous
   `assets/workflows/`, avec un repli propre lorsque la clé est absente.

On enrichit la skill existante plutôt que de la dupliquer : il n'existe qu'une
seule `/codev-propose`, quelle que soit l'origine de la demande. Consultez
[CONTRIBUTING.fr.md](https://github.com/mairistem/codev/blob/main/CONTRIBUTING.fr.md)
pour en proposer une.
