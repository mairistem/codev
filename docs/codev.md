# codev

**Développement piloté par les specs pour Claude Code.**

codev ajoute à un dépôt une fine couche de specs pour que toi et ton
agent soyez d'accord sur ce qui doit être construit avant qu'une ligne
de code ne soit écrite. Chaque évolution passe par un cycle documenté
dans `_codev/`, invocable depuis Claude Code — le CLI `codev` fait le
travail atomique en dessous.

Cette documentation couvre l'installation, le cycle, les concepts, la
CLI et la configuration. Elle complète l'aide contextuelle
(`codev --help` sur chaque sous-commande).

---

## 1. Pourquoi codev

Un dépôt sans specs est un dépôt où **les intentions vivent dans la
tête des gens**. On code d'un côté, on relit de l'autre, on redécouvre
« pourquoi ce truc est là » six mois plus tard en creusant dans un
commit. codev retourne ça — pour chaque évolution significative, on
écrit d'abord ce qu'on veut, on discute, on valide, puis on code.

Ce qui est spécifique à codev :

- **Cycle explicite** — propose → apply → sync → archive. Chaque
  étape a un artefact et une skill Claude Code dédiée.
- **Immutabilité des décisions** — un ADR accepté est scellé (K3).
  L'outil détecte automatiquement toute réécriture silencieuse.
- **Deltas fusionnés proprement** — chaque change livre un « delta »
  (`ADDED`, `MODIFIED`, `REMOVED`, `RENAMED`), fusionné dans les
  specs principales au moment de `sync`/`archive`, au caractère près.
- **Sources héritées** — une équipe peut partager ses ADR entre
  plusieurs projets, épinglés par SHA git ou par chemin local (K5).
- **Extension MCP** — les skills détectent les patterns externes
  (numéros de tickets Jira) et enrichissent le contexte via un MCP
  configuré côté projet.

---

## 2. Installation

Trois voies. La première (curl \| sh) est la plus courte et **ne
nécessite pas Rust**. Les deux suivantes servent des cas particuliers.

### Voie recommandée : `install.sh` en une commande

```bash
curl -sSL https://raw.githubusercontent.com/mairistem/codev/main/install.sh | sh
```

Le script détecte ton OS/arch (macOS arm64 ou x86_64, Linux x86_64),
télécharge le binaire précompilé correspondant depuis GitHub Releases,
vérifie son SHA-256, et l'installe dans `~/.local/bin/codev`. Il t'indique
en fin de course si `~/.local/bin` est bien dans ton `$PATH` — sinon
il te donne la ligne exacte à ajouter à ton `.zshrc` ou `.bashrc`.

Pour épingler une version précise :

```bash
curl -sSL https://raw.githubusercontent.com/mairistem/codev/main/install.sh | \
  CODEV_VERSION=0.2.0 sh
```

### Voie manuelle : téléchargement depuis GitHub Releases

Si tu préfères ne pas piper un `curl` sur `sh` (audit avant exécution,
politique interne), télécharge à la main depuis
`https://github.com/mairistem/codev/releases/latest` :

1. Récupère le fichier `codev-<version>-<target>.tar.gz` correspondant
   à ton OS/arch.
2. Récupère aussi `SHA256SUMS` et vérifie l'intégrité :

   ```bash
   shasum -a 256 -c SHA256SUMS  # macOS
   sha256sum -c SHA256SUMS      # Linux
   ```
3. Décompresse et copie `codev` dans un dossier sur ton `$PATH` :

   ```bash
   tar xzf codev-*.tar.gz
   mkdir -p ~/.local/bin
   cp codev-*/codev ~/.local/bin/
   chmod 755 ~/.local/bin/codev
   ```

### Voie contributeur : `cargo install`

Si tu as la toolchain Rust installée et que tu veux construire depuis
la source (contribution, expérimentation, plateforme non supportée par
les binaires précompilés) :

```bash
git clone git@github.com:mairistem/codev.git
cd codev
cargo install --path crates/codev-cli
```

Le binaire arrive dans `~/.cargo/bin/codev`.

### Vérifier l'installation et le `$PATH`

```bash
which codev       # doit pointer vers ~/.local/bin/codev ou ~/.cargo/bin/codev
codev --version   # affiche la version
```

Si `which codev` ne rend rien : `~/.local/bin` (ou `~/.cargo/bin`) n'est
pas dans ton `$PATH`. Ajouter à `~/.zshrc` ou `~/.bashrc` :

```bash
export PATH="$HOME/.local/bin:$PATH"
```

Puis ouvrir un nouveau shell (ou `source ~/.zshrc`).

### Complétions shell

```bash
# zsh
codev completions zsh > "${fpath[1]}/_codev"
compinit

# bash
codev completions bash > ~/.local/share/bash-completion/completions/codev

# fish
codev completions fish > ~/.config/fish/completions/codev.fish
```

Dans un nouveau shell, `codev de<TAB>` liste `decision`, `deviate`,
`docs`, etc. `codev --help` affiche l'aide de haut niveau.

### Initialiser codev dans un dépôt

Dans un dépôt existant :

```bash
cd mon-projet
codev init
```

crée `_codev/` (config, dossier de changes, dossier de décisions,
dossier de specs) et installe les skills par défaut dans
`.claude/skills/`. Le catalogue par défaut installe `propose`,
`explore` et `onboard` — les autres skills (`apply`, `sync`, `archive`,
`update`) sont opt-in via `_codev/config.yaml`.

---

## 3. Le cycle

Le cycle de codev s'articule autour de **quatre gestes** :

```
   ┌────────────┐  ┌────────────┐  ┌────────────┐  ┌────────────┐
   │  propose   │  │   apply    │  │    sync    │  │  archive   │
   │            │  │            │  │            │  │            │
   │ Rédiger    │  │ Implémenter│  │ Fusionner  │  │ Clore et   │
   │ le plan    │→ │ selon      │→ │ delta dans │→ │ déplacer   │
   │ (4 arte-   │  │ tasks.md   │  │ les specs  │  │ vers       │
   │ facts)     │  │            │  │ principales│  │ archive/   │
   └────────────┘  └────────────┘  └────────────┘  └────────────┘
```

### `propose`

Crée un dossier `_codev/changes/<nom>/` avec quatre artefacts :

- **proposal.md** — pourquoi ce change, ce qui change, quelles
  capacités sont touchées, hors périmètre.
- **specs/<capa>/spec.md** — le delta de spec, avec `## ADDED
  Requirements`, `## MODIFIED Requirements`, etc.
- **design.md** — les décisions techniques prises pour ce change,
  avec les alternatives écartées.
- **tasks.md** — la liste des tâches d'implémentation, à cocher.

Invoqué par la skill `/codev-propose` dans Claude Code. Le CLI équivalent :

```bash
codev new change <nom> --goal "..."
```

### `apply`

Lit `tasks.md`, traite chaque case non cochée dans l'ordre du fichier,
coche à mesure. C'est la seule skill qui a le `Bash` général — parce
qu'elle doit pouvoir lancer les tests du projet.

Skill `/codev-apply` dans Claude Code. Aucune commande CLI équivalente :
c'est l'agent qui fait le travail, guidé par le body de la skill.

### `sync`

Fusionne le delta d'un change dans les specs principales
(`_codev/specs/<capa>/spec.md`), sans déplacer le change. Utile pour
qu'une capacité nouvelle apparaisse dans les specs avant d'être
consommée par un autre change.

```bash
codev sync <nom>
```

### `archive`

Fait `sync` **et** déplace le dossier vers
`_codev/changes/archive/<date>-<nom>/`. C'est la sortie normale d'un
change terminé.

```bash
codev archive <nom>
```

Refus strict si `codev validate <nom>` remonte des erreurs.

---

## 4. Les 7 skills Claude Code

| Skill | Rôle | `allowed-tools` | Défaut |
|---|---|---|---|
| `codev-propose` | Rédiger les 4 artefacts de planif | Bash(codev:*), Read, Write, Edit, Glob, Grep, + MCP Jira si configuré | ✓ |
| `codev-explore` | Défricher une idée sans engager | Bash(codev:*), Read, Glob, Grep | ✓ |
| `codev-onboard` | Présenter codev à un utilisateur qui découvre | Bash(codev:*), Read, Glob | ✓ |
| `codev-apply` | Implémenter les tâches | Bash(codev:*), Read, Write, Edit, Glob, Grep, Bash | opt-in |
| `codev-sync` | Fusionner delta dans specs | Bash(codev:*), Read | opt-in |
| `codev-archive` | Clore et déplacer | Bash(codev:*), Read | opt-in |
| `codev-update` | Réviser un artefact de planif | Bash(codev:*), Read, Write, Edit, Glob, Grep | opt-in |

**Règle stricte** : seule `apply` a le `Bash` général — pour lancer
les tests projet. Les autres skills se cantonnent à `Bash(codev:*)`
(commandes du binaire codev). Un test d'invariant verrouille cette
règle dans `codev-agents::workflows`.

Chaque skill livre son body via `include_str!` sur un fichier
`assets/workflows/<id>.md` — la source des instructions envoyées à
Claude Code.

---

## 5. Concepts

### Capacité

Un **comportement** que le système offre. Chaque capacité vit dans
`_codev/specs/<chemin>/spec.md` avec :

- une section `## Purpose` (une ou deux phrases sur ce à quoi elle
  sert) ;
- une section `## Requirements` avec des blocs `### Requirement:
  <nom>` chacun suivi d'au moins un `#### Scenario:`.

Chaque scénario est une clause GIVEN/WHEN/THEN qui décrit un
comportement observable.

### Change

Un dossier `_codev/changes/<nom>/` qui porte une évolution du système.
Un change contient toujours au moins un `proposal.md` ; les autres
artefacts (`specs/…/spec.md`, `design.md`, `tasks.md`) sont conditionnels
au schéma en usage.

Un change est **actif** tant qu'il vit dans `_codev/changes/<nom>/`.
Il devient **archivé** quand `codev archive` le déplace vers
`_codev/changes/archive/<date>-<nom>/`.

### Décision (ADR)

Un fichier `_codev/decisions/NNNN-<slug>.md` qui fixe un choix
d'architecture. Frontmatter YAML (id, title, status, date, tags,
supersedes, deviates_from) suivi de sections libres (Contexte,
Décision, Conséquences, Alternatives écartées).

**Statuts reconnus** : `accepted`, `superseded`, `proposed`,
`deprecated`, `rejected`. Seuls `accepted` et `superseded` entrent
dans le calcul des décisions « en vigueur ».

**Immutabilité (K3)** : chaque décision `accepted` est scellée dans
`_codev/decisions/seal.yaml` avec un SHA-256 de son corps. Toute
réécriture silencieuse remonte comme erreur `decision_seal_mismatch`
à `codev validate`.

**Supersession (K5)** : `supersedes: ["0003"]` remplace `0003` — dont
le status bascule à `superseded` par le CLI, corps préservé au
caractère près.

**Dérive (K6)** : `deviates_from: ["path:~/partage/0100"]` déclare
qu'une décision héritée est écartée localement — elle disparaît des
instructions de `design` sans être modifiée côté source.

### Delta operation

Dans un delta de spec (dans `_codev/changes/<nom>/specs/<capa>/spec.md`) :

- **`## ADDED Requirements`** — comportement nouveau, inséré à la fin
  de la section `## Requirements` de la spec principale.
- **`## MODIFIED Requirements`** — comportement changé ; le bloc doit
  être recopié en entier depuis la spec principale puis édité.
- **`## REMOVED Requirements`** — supprimé, avec **Raison** et
  **Migration**. Si tout est retiré, il faut `retire_capabilities:
  true` dans `change.yaml`.
- **`## RENAMED Requirements`** — retitre l'en-tête `### Requirement:`
  sans toucher au corps ni aux scénarios.

Le merger (`codev-core::merge`) applique chaque opération au caractère
près — le reste de la spec (commentaires, sections libres, espacement)
reste identique.

---

## 6. La CLI

Groupée par famille. Chaque commande accepte `--help`.

### Projet

```bash
codev init [PATH] [--force]        # crée _codev/ et installe les skills
codev update [--force]             # ré-installe les skills à la version courante
codev docs [--print|--write PATH]  # ouvre cette documentation
codev completions <SHELL>          # génère un script de complétion
```

### Changes

```bash
codev list [--specs]               # liste les changes actifs (ou les specs)
codev new change <NOM> [--schema] [--goal]
codev status --change <NOM> [--json]
codev instructions <ARTEFACT> --change <NOM> [--json]
codev sync <NOM> [--json]
codev archive <NOM> [--json]
```

### Décisions

```bash
codev decision list [--json]
codev decision show <ID> [--json]
codev decision new <TITRE> [--status STATUS] [--json]
codev decision seal <ID> [--force] [--json]
codev decision supersede <OLD_ID> <NEW_TITRE> [--json]
codev decision deviate <QUALIFIED_ID> <TITRE> [--json]
codev decision promote <CHANGE> <TITRE> [--json]
```

### Sources héritées

```bash
codev sources list [--json]
codev sources show <TARGET> [--json]
codev sources update [--json]
```

### Validation

```bash
codev validate [ITEM] [--all|--changes|--specs] [--strict] [--json]
codev schemas [--json]
```

`--strict` transforme tout finding (Warning inclus) en motif d'exit
code non-nul. Utile pour la CI et les hooks. Sans `--strict`, seul
`Error` fait basculer.

---

## 7. Configuration

Le fichier `_codev/config.yaml` porte la configuration projet.
Voici la forme complète (tout est optionnel) :

```yaml
# Schéma de workflow utilisé par ce projet. Défaut : spec-driven.
schema: spec-driven

# Workflows à installer comme skills Claude Code. Absent, le
# catalogue par défaut s'applique : propose, explore, onboard.
workflows:
  - propose
  - explore
  - onboard
  - apply
  - sync
  - archive
  - update

# Contexte injecté dans les instructions de TOUS les artefacts.
context: |
  Pile technique : Rust workspace, quatre crates.
  Conventions : commentaires en français, erreurs typées.

# Règles par identifiant d'artefact.
rules:
  specs:
    - Décrire un comportement observable, jamais une implémentation.
  design:
    - Citer les décisions de _codev/decisions/ qui contraignent l'approche.

# Sources héritées, en lecture seule.
inherits:
  - path: ~/codev/partage
  - git: git@github.com:acme/codev-shared.git
    ref: main
    subpath: standards

# Configuration MCP. Le nom du tool dépend de la config Claude Code
# de l'utilisateur — remplacer par le nom exact du MCP disponible.
mcp:
  jira_tool: mcp__claude_ai_Atlassian_Rovo__getJiraIssue
```

### Sources héritées

Deux types :

- **`path:`** — chemin local sur la machine (chemin absolu ou avec
  `~`). Lecture directe.
- **`git:`** — dépôt distant, épinglé par SHA dans `_codev/codev.lock`.
  Aucune commande sauf `codev sources update` ne contacte le réseau.

### MCP

Le champ `mcp:` permet à chaque projet de déclarer le nom exact des
outils MCP disponibles dans sa session Claude Code. La skill
`codev-propose` utilise `mcp.jira_tool` pour détecter les tickets
mentionnés et enrichir le contexte du proposal. Sans cette clef, la
détection est silencieusement inactive — le comportement reste
identique à un codev sans intégration MCP.

---

## 8. Décisions d'architecture

Six ADR racines fixent l'architecture. Ils vivent dans
`_codev/decisions/` du dépôt codev lui-même — dogfooding.

- **0001 — Cœur fonctionnel, coquille impérative.** Le cœur retourne
  des `Plan` d'effets, la coquille (CLI + engine) les exécute. Rien
  dans `codev-core` ne touche au disque.
- **0002 — Le graphe de crates applique la règle de dépendance.**
  Quatre crates : `codev-core` (pur), `codev-engine` (effets via
  ports), `codev-agents` (cible Claude Code), `codev-cli` (coquille
  impérative). Dépendances acycliques, l'inverse est architecturalement
  refusé.
- **0003 — La racine de planification est `_codev/`, visible.** Le
  préfixe `_` la garde visible (les outils comme ripgrep et fd
  ignorent les dossiers cachés par défaut).
- **0004 — Une seule identité skill / slash command.** Pas de fichiers
  de commandes séparés — la skill Claude Code est le seul lieu où
  vit le workflow.
- **0005 — Sources héritées en lecture seule, épinglées par SHA.**
  Pas de stores partagés modifiables ; l'héritage est un pointeur
  figé, jamais un miroir dérivable.
- **0006 — `serde_norway` pour le YAML.** `serde_yaml` est déprécié
  en amont ; le fork maintenu est `serde_norway`.

Trois chapitres complètent le cycle de vie des décisions :

- **K3 — Immutabilité verrouillée par le CLI.** Sceau SHA-256 du
  corps dans `seal.yaml`, `codev validate` détecte toute altération.
- **K6 — Dérives locales des décisions héritées.** `codev decision
  deviate <origin>/<id> <titre>` — l'héritée reste visible dans
  `codev decision list`, mais disparaît des instructions de `design`.
- **K7 — Promotion depuis `design.md`.** `codev decision promote
  <change> <titre>` extrait un bloc `### Décision : <titre>` d'un
  `design.md`, en fait un ADR de premier ordre scellé, remplace le
  bloc par une référence.

Pour les détails, ouvre les fichiers dans `_codev/decisions/` — chaque
ADR est autoportant.

---

## 9. Extensions MCP

codev prévoit que ses skills détectent des patterns externes
(numéros de tickets, URLs, etc.) et appellent le MCP correspondant
pour enrichir le contexte. La stratégie est actée :

- **Les skills existantes détectent elles-mêmes** — pas de skills
  séparées `codev-propose-from-jira`, `codev-propose-from-figma`. Une
  seule skill par usage.
- **Le nom du MCP est configuré côté projet** — dans
  `_codev/config.yaml.mcp`, injecté dans le frontmatter `allowed-tools`
  à l'installation.
- **Fallback silencieux** — sans MCP configuré, le placeholder est
  retiré proprement et la skill se rabat sur un comportement
  informatif.

### Aujourd'hui : Jira / Atlassian

`codev-propose` détecte `[A-Z]{2,}-\d+` dans le prompt de
l'utilisateur. Sur détection :

- Appel `mcp.jira_tool` sur le premier ticket détecté.
- Contenu (titre, description, status) injecté dans le contexte.
- Citation en tête du proposal : `> Source : ticket **JVS-1234** —
  « <titre> » (<status>)`.

### Ajouter un autre MCP

Pour brancher (par exemple) un MCP Confluence sur une nouvelle skill :

1. Ajouter le champ `mcp.confluence_tool` à `McpConfig` dans
   `codev-engine::config`.
2. Étendre `RenderCtx` dans `codev-agents::claude` et
   `substitute_jira_mcp` (ou une famille de fonctions similaire).
3. Ajouter le placeholder `{{CONFLUENCE_MCP_TOOL}}` dans
   `allowed_tools` et dans le body de la skill concernée.
4. Faire propager par le CLI via `install_skills`.

Le pattern est stabilisé — ajouter un MCP est mécanique.

---

## 10. FAQ / troubleshooting

### `codev` ne trouve pas mon dépôt

Message typique : `no_codev_root`. Vérifier :

- Que `_codev/config.yaml` existe (sinon `codev init`).
- Que la commande est lancée dans le dépôt ou un sous-dossier.

### `codev update` refuse d'écraser ma skill

Message : « laissé(s) en place car modifié(s) à la main ». C'est
volontaire — `update` respecte les éditions manuelles. Deux options :

- **Garder l'édition** : ne pas relancer `codev update` sur cette
  skill.
- **Regénérer** : `codev update --force` écrase (l'édition est perdue,
  pense au `git diff`).

### `codev validate` remonte `decision_seal_mismatch`

Quelqu'un (ou un formatage automatique) a modifié le corps d'un ADR
`accepted`. Deux options :

- **Restaurer** le corps original (`git diff` peut aider), le sceau
  reste bon.
- **Ré-approuver** délibérément avec `codev decision seal <ID>
  --force` — la nouvelle version devient la référence.

### `/codev-propose` ignore mon ticket JVS-1234

Deux causes possibles :

1. Le champ `mcp.jira_tool` n'est pas dans `_codev/config.yaml` — la
   skill affiche « (MCP Jira non configuré) » dans les instructions.
   Solution : ajouter la config et relancer `codev update --force`.
2. Le MCP est configuré mais indisponible dans la session Claude Code
   courante (mauvais nom, MCP non branché, session expirée). La skill
   affiche un message informatif et rédige quand même le proposal
   avec ce qu'elle sait.

### Un `codev list` remonte 0 change actif mais je viens d'en créer un

Vérifier que tu utilises **notre** `codev`, pas un autre binaire du
même nom :

```bash
which codev  # doit être ~/.cargo/bin/codev
```

### Comment partager cette doc avec un collègue ?

```bash
codev docs --write ~/Downloads/codev-manuel.html
```

Le fichier HTML est autonome (CSS inline, pas de dépendance). Envoie
par email, drop dans Slack, publie sur Confluence.

---

*Documentation générée par `codev docs`. Le source markdown vit dans
`docs/codev.md` du dépôt codev.*
