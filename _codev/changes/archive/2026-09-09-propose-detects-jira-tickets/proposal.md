# Proposal : `codev-propose` détecte et enrichit un ticket Jira mentionné

## Pourquoi

C'est la première vraie intégration MCP côté codev. Aujourd'hui,
quand un utilisateur mentionne `JVS-1234` dans son prompt de
`/codev-propose`, la skill traite la chaîne comme un mot opaque au
même titre que « ajouter l'authentification » — le proposal
sort **sans** le contexte du ticket, alors qu'un MCP Atlassian
branché sur la même session peut le récupérer en un appel.

C'est la stratégie que Ludovic a actée : **les futures skills codev
détectent elles-mêmes les patterns MCP-triggerables et appellent le
MCP directement, sans skills séparées** ([[project_codev_mcp-integration]]).
Ce change livre le premier cas — Jira / Atlassian — dont les
apprentissages guideront les intégrations suivantes (Design, Notion,
GitHub Issues…).

Décidé avec Ludovic après une exploration `/codev-explore`
(2026-09-09) — les 6 questions ouvertes ont été tranchées :

1. **Périmètre** — Jira seul, un vrai deuxième cas viendra plus tard.
2. **MCP** — `mcp__claude_ai_Atlassian__*` (MCP officiel Claude
   Atlassian, disponible sur Claude Code).
3. **Pattern** — générique `[A-Z]{2,}-\d+`, pas de configuration
   projet.
4. **Contenu récupéré** — injecté dans le contexte de la conversation
   **et** cité en tête du proposal pour la traçabilité.
5. **Sans MCP branché** — message informatif, le proposal se rédige
   quand même sans le contenu du ticket.
6. **Où vit la logique** — dans le body markdown de la skill (le CLI
   codev n'a pas à connaître Jira).

## Ce qui change

- **Le body de `assets/workflows/propose.md`** gagne une étape 0 :
  détection d'un pattern `[A-Z]{2,}-\d+` dans le prompt de
  l'utilisateur, avant même la résolution du nom de change.
- **Sur détection** :
  - Si `mcp__claude_ai_Atlassian__getJiraIssue` est disponible, la
    skill l'appelle avec l'identifiant du ticket.
  - Le contenu du ticket (titre, description, status, type)
    devient une **source de contexte** que l'agent lit avant de
    rédiger le proposal.
  - Le proposal cite le ticket en tête, section « Contexte externe »
    ou similaire, avec une ligne du type
    `> Source : ticket **JVS-1234** — « <titre> » (<status>)`.
- **Sans MCP branché** : la skill affiche
  « Un ticket JVS-1234 est mentionné mais aucun MCP Atlassian n'est
  disponible dans cette session — le proposal sera rédigé sans son
  contenu. » Puis continue.
- **`allowed-tools` de `propose`** gagne
  `mcp__claude_ai_Atlassian__getJiraIssue`. Aucun autre outil MCP
  n'est ajouté — la skill n'a pas besoin de rechercher ni de
  modifier des tickets.
- **Aucun changement de frontière** — la skill reste incapable
  d'écrire hors de `_codev/changes/<nom>/`.

## Capacités

### Nouvelles capacités

Aucune.

### Capacités modifiées

- `skills` — nouvelle exigence ADDED décrivant le comportement de
  détection et d'enrichissement de `codev-propose`. Les autres
  exigences de propose (implicites aujourd'hui) restent inchangées.

### Capacités retirées

Aucune.

## Impact

- **Code** : édition du body `assets/workflows/propose.md` (ajout
  d'une étape 0). Extension de l'`allowed-tools` de l'entrée
  `Workflow { id: "propose", … }` dans le `CATALOG` de
  `codev-agents::workflows`. Un test dédié
  `propose_declare_le_mcp_atlassian` verrouille la présence du MCP
  dans `allowed-tools`.
- **Contrat JSON** : rien. La détection et l'appel MCP vivent
  entièrement dans le body — le CLI codev n'en sait rien.
- **Fichiers écrits** : aucun changement sur le disque hors le
  proposal.md du change en question. La skill continue de créer le
  change via `codev new change` comme avant.
- **Migration** : aucune. Sans MCP Atlassian branché, comportement
  bit-identique à aujourd'hui (message informatif au premier ticket
  mentionné, sinon silencieux).
- **Hors périmètre** :
  - **Autres MCP** (Design, Notion, GitHub) — attendront un vrai
    deuxième cas concret.
  - **Recherche de tickets** (JQL, listing) — la skill n'appelle
    que `getJiraIssue` pour un ID précis mentionné.
  - **Écriture vers Jira** (transitions, commentaires) — pas dans
    ce lot ; le proposal peut le suggérer comme travail futur si
    utile.
  - **Multi-tickets dans une même invocation** — la skill traite le
    premier ticket détecté ; les autres sont juste mentionnés.
    Reportable si le pattern devient récurrent.
  - **Configuration projet du pattern** — pas de champ
    `ticket_pattern:` dans `_codev/config.yaml` pour la V1. Le
    pattern générique `[A-Z]{2,}-\d+` suffit pour toutes les orgs
    Atlassian standard.
