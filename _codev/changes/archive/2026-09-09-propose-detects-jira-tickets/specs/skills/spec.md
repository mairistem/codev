## ADDED Requirements

### Requirement: Skill `propose` détecte les tickets Jira mentionnés et enrichit le proposal

Le workflow `propose` du catalogue SHALL, en amont de la résolution du
nom de change, scanner le prompt de l'utilisateur pour repérer un
identifiant de ticket qui matche le pattern régulier `[A-Z]{2,}-\d+`
(par exemple `JVS-1234`, `PROJ-42`).

Sur détection d'au moins un ticket, la skill MUST :

1. **Tenter d'appeler** l'outil MCP `mcp__claude_ai_Atlassian__getJiraIssue`
   avec l'identifiant du ticket **le plus tôt mentionné dans le prompt**
   (un seul ticket par invocation ; les autres sont juste nommés dans
   le proposal, pas récupérés).
2. **En cas de succès** : injecter le contenu (titre, description,
   status, type) dans le contexte de rédaction, et faire figurer le
   ticket en tête du `proposal.md` du change, sous forme d'une ligne
   citation `> Source : ticket **<ID>** — « <titre> » (<status>)`.
3. **En cas d'échec — outil MCP indisponible dans la session** :
   afficher un message informatif à l'utilisateur (« Un ticket <ID>
   est mentionné mais aucun MCP Atlassian n'est disponible dans cette
   session — le proposal sera rédigé sans son contenu »), puis
   continuer le flow habituel sans le contenu du ticket. Le proposal
   final mentionne quand même le ticket en tête, avec ce que la skill
   sait (l'ID seul).
4. **En l'absence de pattern** : comportement bit-identique à
   aujourd'hui — aucun appel MCP, aucun message.

L'`allowed-tools` du workflow `propose` MUST déclarer
`mcp__claude_ai_Atlassian__getJiraIssue`. Aucun autre outil MCP
Atlassian n'est nécessaire pour cette capacité — la skill n'écrit
jamais dans Jira et ne fait pas de recherche JQL.

#### Scenario: Ticket mentionné, MCP disponible → ticket cité en tête

- **GIVEN** un utilisateur tape `/codev-propose ajouter JWT pour JVS-1234`
- **AND** le MCP `mcp__claude_ai_Atlassian__getJiraIssue` est
  disponible dans la session
- **AND** le ticket `JVS-1234` existe et a pour titre « Authentifier
  les utilisateurs par JWT » et pour status « In Progress »
- **WHEN** la skill s'exécute
- **THEN** un appel `mcp__claude_ai_Atlassian__getJiraIssue({issueIdOrKey: "JVS-1234"})`
  est émis
- **AND** le `proposal.md` du change créé contient en tête une ligne
  `> Source : ticket **JVS-1234** — « Authentifier les utilisateurs
  par JWT » (In Progress)`
- **AND** le corps du proposal (Pourquoi, Ce qui change…) exploite le
  contenu du ticket pour être plus précis qu'avec le seul prompt

#### Scenario: Ticket mentionné, MCP indisponible → proposal quand même

- **GIVEN** un utilisateur tape `/codev-propose ajouter JWT pour JVS-1234`
- **AND** aucun MCP Atlassian n'est disponible dans la session
- **WHEN** la skill s'exécute
- **THEN** la skill affiche un message informatif nommant
  `JVS-1234` et disant qu'aucun MCP Atlassian n'est disponible
- **AND** le proposal est créé quand même, en tête duquel
  `JVS-1234` est mentionné sous forme d'ID seul (« Source : ticket
  **JVS-1234** — contenu non récupéré »)
- **AND** le corps du proposal est rédigé à partir du seul prompt
  utilisateur

#### Scenario: Aucun ticket mentionné → comportement inchangé

- **GIVEN** un utilisateur tape `/codev-propose ajouter
  l'authentification`
- **WHEN** la skill s'exécute
- **THEN** aucun appel MCP n'est émis
- **AND** aucun message relatif à un ticket n'apparaît
- **AND** le proposal est rédigé exactement comme avant ce change

#### Scenario: Plusieurs tickets mentionnés → seul le premier est récupéré

- **GIVEN** un utilisateur tape `/codev-propose corriger JVS-1234
  et JVS-5678`
- **AND** le MCP est disponible
- **WHEN** la skill s'exécute
- **THEN** un appel MCP est émis pour `JVS-1234` uniquement
- **AND** le proposal cite `JVS-1234` en tête avec son contenu
  récupéré
- **AND** le proposal cite `JVS-5678` comme second ticket lié, sans
  son contenu récupéré (« autre(s) ticket(s) mentionné(s) :
  JVS-5678 »)

#### Scenario: `allowed-tools` déclare bien le MCP Atlassian

- **GIVEN** l'entrée `Workflow { id: "propose", … }` du CATALOG
- **WHEN** on inspecte son `allowed_tools`
- **THEN** la chaîne contient
  `mcp__claude_ai_Atlassian__getJiraIssue`
- **AND** ne contient PAS d'autre outil `mcp__claude_ai_Atlassian__*`
  (règle stricte : la skill n'a besoin que de lire un ticket)
