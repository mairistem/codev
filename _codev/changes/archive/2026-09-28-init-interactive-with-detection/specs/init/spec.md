## Purpose

Décrit l'expérience de première initialisation d'un projet par
`codev init` : détection silencieuse de l'environnement, prompts
interactifs ciblés, génération d'un `_codev/config.yaml` prérempli
avec commentaires de provenance. La capacité contractualise ce
qu'aujourd'hui aucune spec ne fixait — le comportement de la
commande était laissé à l'implémentation.

## ADDED Requirements

### Requirement: `codev init` interactif détecte l'environnement avant de poser des questions

Avant tout prompt, `codev init` SHALL sonder silencieusement le
dossier courant et en tirer un rapport `Detected` couvrant :

- **Stack** — langage et framework, lus depuis `Cargo.toml`,
  `package.json`, `pyproject.toml`, `go.mod` ou `pom.xml`. Le premier
  manifeste trouvé fixe la stack primaire ; les autres sont ignorés.
- **Nom du projet** — issu du manifeste (`[package].name`, `name`
  JSON, etc.), ou à défaut `git remote get-url origin` (last-path
  segment sans `.git`).
- **Licence** — depuis un fichier `LICENSE` ou `LICENSE.md` racine,
  reconnaissance par regex sur les noms courants (MIT, Apache-2.0,
  BSD-3-Clause, GPL-3.0).
- **CI** — présence d'un dossier `.github/workflows/` non vide.
- **MCPs configurés** — parse les fichiers suivants s'ils existent, dans
  cet ordre, en fusionnant les entrées (le premier gagne en cas de
  conflit) : `<projet>/.mcp.json`, `~/.claude.json`,
  `<projet>/.claude/settings.json`, `<projet>/.claude/settings.local.json`.
  Extrait la clé `mcpServers` (map de `{nom: {command? | url?, args?, env?}}`).

La sonde MUST être **best-effort** : un manifeste illisible ou une
sortie git en erreur produit un warning silencieux ; la détection
continue avec les champs récupérables.

La sonde MUST rester en **lecture seule** — aucun fichier créé, aucun
processus lancé, aucun appel réseau.

Un flag `--no-detect` MUST court-circuiter la sonde intégralement.

#### Scenario: Détection d'un workspace Rust

- **GIVEN** un dossier avec `Cargo.toml` de workspace contenant 4
  membres et `[workspace.package].edition = "2024"`
- **WHEN** l'utilisateur lance `codev init --yes`
- **THEN** le `context:` du `_codev/config.yaml` généré cite la stack
  Rust workspace, l'édition 2024, le nombre de crates
- **AND** aucun prompt n'est affiché (mode `--yes`)

#### Scenario: Détection en absence de manifeste connu

- **GIVEN** un dossier sans manifeste reconnu, sans `.git/`, sans
  `LICENSE`, sans `.github/workflows/`
- **WHEN** l'utilisateur lance `codev init --yes`
- **THEN** la commande réussit
- **AND** le `context:` généré est vide ou minimal (une ligne
  générique)
- **AND** aucune erreur n'est remontée pour cause de détection à vide

### Requirement: `codev init` détecte les MCPs Jira/Atlassian et prérempli la config

Sur détection d'au moins un serveur MCP dont le nom, la commande ou
l'URL matche la regex insensible à la casse `/jira|atlassian/`,
`codev init` SHALL calculer un `tool_id` candidat via la convention
Claude Code :

- Le nom du serveur est normalisé : espaces et points remplacés par
  `_`.
- Le préfixe est `mcp__`, le suffixe est `__getJiraIssue`.
- Exemple : `"claude.ai Atlassian Rovo"` →
  `mcp__claude_ai_Atlassian_Rovo__getJiraIssue`.

La fonction de calcul MUST être pure (`codev-core::detect::mcp`), sans
appel réseau ni introspection du serveur MCP lui-même.

Si un seul candidat non ambigu émerge, `codev init` interactif MUST
l'afficher pour confirmation avec sa source (`.mcp.json`,
`~/.claude.json`, etc.), et le `_codev/config.yaml` généré porte le
tool_id sous la clé `mcp.jira_tool` avec un **commentaire de
provenance** au-dessus.

Si plusieurs candidats émergent, l'utilisateur choisit dans une
liste.

Si zéro candidat, aucune question n'est posée, aucune clé `mcp:`
n'est écrite. La clé restera commentée dans le YAML template pour
édition future.

En mode `--yes` sans candidat non ambigu, aucune clé `mcp:` n'est
écrite ; en mode `--yes` avec un candidat non ambigu, il est retenu
sans confirmation.

#### Scenario: MCP Jira détecté depuis .mcp.json

- **GIVEN** un `.mcp.json` racine contenant un serveur nommé
  `"claude.ai Atlassian Rovo"`
- **WHEN** l'utilisateur lance `codev init --yes`
- **THEN** le `_codev/config.yaml` généré contient
  `mcp.jira_tool: mcp__claude_ai_Atlassian_Rovo__getJiraIssue`
- **AND** un commentaire au-dessus cite `.mcp.json` comme source

#### Scenario: Aucun MCP détecté

- **GIVEN** un dossier sans aucun fichier de config MCP
- **WHEN** l'utilisateur lance `codev init --yes`
- **THEN** le `_codev/config.yaml` généré ne contient pas de clé
  `mcp:` active (elle reste commentée dans le template)
- **AND** aucun warning n'est remonté

### Requirement: `codev init` interactif pose au plus deux questions

Sans flag non-interactif, `codev init` MUST poser exactement les deux
questions suivantes, dans cet ordre :

1. **Workflows à installer** — choix parmi trois presets, défaut
   « Complet (7) » :
   - `Complet (7)` — `propose`, `explore`, `onboard`, `apply`, `sync`,
     `archive`, `update`.
   - `Minimal (3)` — `propose`, `explore`, `onboard`.
   - `Personnalisé` — l'utilisateur coche chaque workflow un à un.
2. **Contexte projet** — texte libre ajouté au `context:` généré. La
   stack détectée est affichée au-dessus du prompt à titre de
   contexte. L'utilisateur peut : taper une phrase, appuyer sur Entrée
   pour ouvrir `$EDITOR` sur un squelette prérempli, ou taper Entrée
   sur une saisie vide pour sauter (le contexte détecté reste seul).

Aucune autre question n'est posée en mode interactif — ni sur le
schéma (un seul disponible), ni sur les MCPs (confirmés par la
détection, pas demandés).

#### Scenario: Deux questions posées en mode interactif

- **GIVEN** un TTY interactif et un projet neuf
- **WHEN** l'utilisateur lance `codev init`
- **THEN** exactement deux prompts s'affichent — workflows puis
  contexte
- **AND** le prompt workflows a « Complet (7) » comme option par
  défaut

### Requirement: Les flags non-interactifs composent proprement

`codev init` MUST accepter les flags suivants, composables :

- `--yes` (`-y`) — applique les défauts pour toutes les questions,
  aucun prompt affiché.
- `--no-detect` — désactive la sonde.
- `--preset <complet|minimal|personnalise>` — préselectionne la
  réponse à la question 1 ; en mode interactif, `personnalise`
  déclenche encore le sous-prompt, `complet` et `minimal` la sautent.
- `--force` — inchangé, réécrit les skills même modifiées à la main.

En **absence de TTY sur stdin** (pipe, redirect, CI), `codev init`
MUST se comporter comme si `--yes` était passé — aucun prompt,
défauts appliqués. Cela garantit la scriptabilité et le comportement
en pipeline.

#### Scenario: --yes court-circuite les prompts

- **GIVEN** un TTY interactif
- **WHEN** l'utilisateur lance `codev init --yes`
- **THEN** aucun prompt n'est affiché
- **AND** les 7 workflows sont installés
- **AND** le `context:` généré ne porte que ce que la détection a
  produit

#### Scenario: Stdin non-TTY implique --yes

- **GIVEN** stdin redirigé depuis `/dev/null`
- **WHEN** l'utilisateur lance `codev init` (sans `--yes` explicite)
- **THEN** aucun prompt n'est affiché
- **AND** l'installation aboutit comme sous `--yes`

### Requirement: Le `config.yaml` généré porte des commentaires de provenance

`codev init` MUST écrire un `_codev/config.yaml` **prérempli** (et
non un template commenté) dont chaque clé non triviale porte, sur la
ligne au-dessus, un commentaire indiquant sa source :

- `# détecté depuis Cargo.toml` (pour `context:`).
- `# détecté depuis .mcp.json → serveur « claude.ai Atlassian Rovo »`
  (pour `mcp.jira_tool:`).
- Absence de commentaire pour les clés triviales (`schema:`) ou
  choisies par l'utilisateur (`workflows:`).

Le format YAML généré MUST rester **relisible par le lecteur codev
existant** (`codev-engine::config::resolve`) — aucune régression sur
la relecture.

#### Scenario: Provenance présente pour context et mcp

- **GIVEN** un workspace Rust + un `.mcp.json` Atlassian
- **WHEN** l'utilisateur lance `codev init --yes`
- **THEN** le `_codev/config.yaml` généré porte
  `# détecté depuis Cargo.toml` au-dessus de la clé `context:`
- **AND** porte
  `# détecté depuis .mcp.json → serveur « claude.ai Atlassian Rovo »`
  au-dessus de `mcp.jira_tool:`
- **AND** `codev status` sur ce projet neuf réussit (relecture OK)
