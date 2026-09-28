# Proposal : codev init interactif avec auto-détection

## Pourquoi

Aujourd'hui, `codev init` fait le strict minimum : il crée l'arborescence
`_codev/`, écrit un `config.yaml` template avec les workflows **commentés**,
et installe **trois** skills — `propose`, `explore`, `onboard`. Les quatre
autres (`apply`, `sync`, `archive`, `update`) sont opt-in : il faut
éditer `_codev/config.yaml` à la main, décommenter la liste, et relancer
`codev update`.

Ce défaut casse la découverte : un utilisateur qui installe codev tape
`/codev-apply` en toute logique après `/codev-propose`, ne trouve pas la
skill, en déduit que codev n'a pas d'implémenteur — ou qu'il a mal
installé.

Trois autres frictions qu'aucun `codev init` ne prend en charge :

- **Détection MCP** — le projet a probablement un ou plusieurs MCPs
  déclarés (Jira, Confluence, Figma). Aujourd'hui, l'utilisateur doit
  copier-coller le tool ID `mcp__…__getJiraIssue` dans `mcp.jira_tool`.
- **Détection stack** — `context:` reste vide, alors que le langage,
  l'édition, les crates du workspace, la licence et la présence d'un
  CI se lisent en 5 lignes.
- **Aucune prise en main** — la première invocation aboutit à un fichier
  YAML rempli de commentaires que l'utilisateur doit décoder.

Ce change transforme `codev init` en **prise en main réelle** :
auto-détection maximale, deux questions ciblées, config générée avec
commentaires de provenance.

## Ce qui change

### Détection (aucune question posée)

Au démarrage, `codev init` sonde silencieusement le dossier courant et
en tire :

| Champ | Source | Utilisation |
|---|---|---|
| **Stack + langage** | `Cargo.toml` / `package.json` / `pyproject.toml` / `go.mod` / `pom.xml` | Base du `context:` généré |
| **Édition, MSRV** | `Cargo.toml` `[package]` / `[workspace.package]` | Ajouté au `context:` |
| **Nom du projet** | manifeste + `git remote get-url origin` (fallback) | Log de bienvenue |
| **Framework de test** | dépendances déclarées | `context:` |
| **Licence** | fichier `LICENSE` racine (regex sur les 3-4 courants) | `context:` |
| **CI** | présence de `.github/workflows/` | `context:` |
| **MCPs Jira / Atlassian** | `<projet>/.mcp.json`, `~/.claude.json`, `.claude/settings.json`, `.claude/settings.local.json` — clé `mcpServers`, matcher `/jira\|atlassian/i` sur nom/commande/URL | Préremplit `mcp.jira_tool:` |
| **Git repo** | `.git/` présent | Log de bienvenue |

La règle de résolution nom-serveur → tool-id est la convention Claude
Code : espaces et points → `_`, préfixe `mcp__`, suffixe `__<tool>`.
Pour `"claude.ai Atlassian Rovo"` + `getJiraIssue` →
`mcp__claude_ai_Atlassian_Rovo__getJiraIssue`. Fonction pure dans
`codev-core::detect::mcp`.

### Prompts (deux questions, pas trois)

**Question 1 — Workflows** :

```
Quels workflows installer ?
    > Complet (7) — propose, explore, onboard, apply, sync, archive, update  [défaut]
      Minimal (3) — propose, explore, onboard
      Personnalisé — te laisse choisir un par un
```

**Question 2 — Contexte projet** :

```
Contexte pour les skills (ce qui n'est pas déductible du code) :
    stack détectée : Rust workspace (4 crates), édition 2024

    Ajoute tes conventions (Enter pour ouvrir $EDITOR, ou tape ta phrase) :
    ▓
```

Une phrase suffit. Ce que la détection a déjà rempli reste et est
préservé.

Le MCP détecté est **confirmé, pas demandé** :

```
✓ MCP Jira détecté : mcp__claude_ai_Atlassian_Rovo__getJiraIssue
   (source : .mcp.json → serveur « claude.ai Atlassian Rovo »)
```

Si zéro candidat : rien à confirmer, silence. Si plusieurs : liste
courte à choisir.

### `config.yaml` généré avec provenance

À la place du template commenté actuel, l'utilisateur voit un fichier
**pré-rempli** dont chaque champ non trivial porte son origine :

```yaml
schema: spec-driven

workflows:
  - propose
  - explore
  - onboard
  - apply
  - sync
  - archive
  - update

# détecté depuis .mcp.json → serveur « claude.ai Atlassian Rovo »
mcp:
  jira_tool: mcp__claude_ai_Atlassian_Rovo__getJiraIssue

# détecté depuis Cargo.toml — édite si besoin
context: |
  Projet Rust workspace (4 crates : codev-core, codev-engine,
  codev-agents, codev-cli), édition 2024, MSRV 1.89. Licence MIT.
  CI GitHub Actions.

  <ce que l'utilisateur a tapé à la question 2>

# rules: à définir au fil des cycles, par artefact — voir docs §7.
```

### Flags CLI

- `--yes` (`-y`) — applique tous les défauts, aucun prompt (utile en
  CI, script, ou pour l'utilisateur pressé).
- `--no-detect` — désactive la sonde (utile pour les tests
  déterministes).
- `--preset <complet|minimal|personnalise>` — préselectionne la
  réponse à la question 1.
- `--force` — inchangé, réécrit les skills même modifiées à la main.
- **Sans TTY** (stdin non-interactif — pipe, redirect) : implicit
  `--yes`, aucun prompt.

Toutes ces options composent : `codev init --preset complet --yes`
utilise le préset et ne pose aucune question.

### Défaut retourné

`DEFAULT_WORKFLOWS` passe de `["propose", "explore", "onboard"]` à la
liste **complète des 7 workflows**. Cela reste cohérent avec le
nouveau flux :

- **Interactif** : question 1 propose « Complet (7) » comme défaut ;
  l'utilisateur peut choisir minimal.
- **`--yes`** : applique le nouveau défaut → 7 workflows.
- **Absent de `_codev/config.yaml`** : `codev update` sur un projet
  existant applique le défaut → 7 workflows également.

La spec `skills` doit être mise à jour en conséquence.

## Capacités

### Nouvelles capacités

- **`init`** — nouvelle capacité qui décrit le contrat interactif de
  `codev init` : détection, prompts, génération avec provenance, flags
  non-interactifs. Aujourd'hui `codev init` a un comportement, mais
  aucune spec ne le fixe — ce change comble la lacune.

### Capacités modifiées

- **`skills`** — la Requirement « `onboard` fait partie du catalogue
  par défaut » est réécrite pour refléter le nouveau `DEFAULT_WORKFLOWS`
  (les 7 workflows). La note « les autres opt-in restent opt-in » est
  supprimée.

### Capacités retirées

Aucune.

## Impact

- **Code** :
  - Nouveau module `codev-core::detect` (pur — reçoit des `&[u8]` de
    manifestes, retourne un `Detected` typé).
  - Nouveau module `codev-core::config::generate` (pur — reçoit
    `Detected` + choix utilisateur, retourne du YAML avec commentaires).
  - Refonte de `codev-cli::commands::init` : orchestration
    (sniff → prompt → generate → scaffold → install).
  - Nouvelle dépendance workspace : **`dialoguer`** (lib de prompts,
    mature, utilisée par cargo/rustup). Alternative écartée :
    `inquire` — plus lourde, features non nécessaires ici.
- **Contrat JSON** : le contrat `SetupOutcome`/`InitReportV1` gagne un
  champ `detected` optionnel qui liste ce qui a été détecté. Non
  breaking — additif.
- **Backward compat** : sur un projet déjà initialisé, `codev init`
  détecte le `config.yaml` existant et **ne re-prompt pas** — même
  comportement idempotent qu'aujourd'hui. Seule différence : `codev
  update` respecte le nouveau `DEFAULT_WORKFLOWS`, donc un projet
  qui n'a jamais posé de `workflows:` explicite reçoit d'un coup les
  4 skills manquantes. Documenté dans le CHANGELOG.
- **Tests** :
  - `codev-core::detect` : tests unitaires par manifeste (fixtures de
    `Cargo.toml`, `package.json`, etc.) + tests MCP par variante de
    `.mcp.json`.
  - `codev-core::config::generate` : test golden — un `Detected`
    fixe + choix connus produit toujours le même YAML.
  - `codev-cli::commands::init` : test intégration `--yes` de bout
    en bout avec un `MemoryFileSystem`, plus test `--no-detect`,
    plus test « sans TTY implicit --yes ».
- **Fichier écrit** : ~4 fichiers de code neufs, ~2 modifiés, la spec
  `init` (~150 lignes), delta `skills` (MODIFIED), tests fixtures,
  entrée `docs/codev.md` §2.
- **Hors périmètre** :
  - **Détection Figma / design MCPs** — la même mécanique s'y appliquera
    quand on ajoutera la clé `mcp.design_tool:`, mais pas dans ce
    lot. La fonction de matching reste extensible (liste de patterns).
  - **Détection de règles** (`rules:` par artefact) — trop niché ; les
    utilisateurs les découvrent en éditant le YAML.
  - **`codev init --update`** — un flag qui rejouerait les prompts sur
    un projet existant pour compléter la config. Reporté à un cycle
    futur si demandé.
  - **Détection au-delà de la racine du projet** — on ne remonte pas
    l'arborescence.
