# Inventaire des fonctionnalités

Périmètre de `codev`, dérivé d'une étude d'OpenSpec `v1.12.0` (190 fichiers
TypeScript, ~44 500 lignes) puis arbitré.

**Cible unique : Claude Code.** Le `trait AgentTarget` reste en place pour que
l'ajout d'un autre outil ne soit pas invasif, mais il n'a qu'une
implémentation.

Légende des lots : **1** = premier jet livrable · **2** = décisions et partage ·
**3** = transport git · **4** = confort · ✂️ = hors périmètre assumé.

## A. Fondations & configuration

| # | Fonctionnalité | Lot |
|---|---|---|
| A1 | Arborescence `_codev/` : `specs/`, `decisions/`, `changes/`, `changes/archive/`, `config.yaml`, `schemas/` | 1 |
| A2 | Découverte de la racine : remontée depuis le cwd jusqu'à `_codev/` | 1 |
| A3 | Config projet `config.yaml` : `schema`, `workflows`, `context`, `rules`, `inherits` | 1 |
| A4 | Métadonnées de change `change.yaml` : `schema`, `created`, `goal`, `skip_specs`, `retire_capabilities` | 1 |
| A5 | Chemins XDG multi-plateformes (cache, données) | 3 |
| A6 | Config globale (préférences machine) | 4 |

## B. Parsing & modèle de données

| # | Fonctionnalité | Lot |
|---|---|---|
| B1 | Parser de spec : `## Purpose`, `## Requirements`, `### Requirement:`, `#### Scenario:` | 1 |
| B2 | Parser de delta : `ADDED` / `MODIFIED` / `REMOVED` / `RENAMED`, `**Reason**` / `**Migration**`, `FROM:` / `TO:` | 1 |
| B3 | Parser de tasks : `- [ ] X.Y`, groupes `## N.`, progression | 1 |
| B4 | Parser de proposal : `## Why`, `## What Changes`, `## Capabilities`, `## Impact` | 1 |
| B5 | Masquage des code fences et commentaires HTML pendant le parsing | 1 |
| B6 | Conservation des spans d'origine — réécriture non destructive | 1 |

## C. Schémas & graphe d'artefacts

| # | Fonctionnalité | Lot |
|---|---|---|
| C1 | `schema.yaml` : `artifacts[{id, generates, description, template, instruction, requires}]` + bloc `apply` | 1 |
| C2 | Graphe DAG : tri topologique, détection de cycles, égalité tranchée par l'ordre de déclaration | 1 |
| C3 | État par existence de fichiers : `done` / `ready` / `blocked` / `skipped`. Aucun fichier d'état | 1 |
| C4 | Précédence de résolution : projet → hérité → embarqué dans le binaire | 1 |
| C5 | Schéma intégré `spec-driven` + ses templates | 1 |
| C6 | `codev schema init / fork / validate / which` | 4 |
| C7 | Composition des instructions : `context` + `rules` + `instruction` + `template` + `dependencies` + `resolvedOutputPath` | 1 |

## D. Surface CLI

| # | Commande | Lot |
|---|---|---|
| D1 | `init [chemin]` — scaffolding + génération des skills | 1 |
| D2 | `update [chemin]` — régénération, détection de drift | 1 |
| D3 | `list [--specs \| --changes] [--json]` | 1 |
| D4 | `show <item> [--json] [--deltas-only] [--diff] [-r N]` | 1 |
| D5 | `new change <nom>` — kebab-case strict, `--schema`, `--goal` | 1 |
| D6 | `status [--change] [--all] [--json]` | 1 |
| D7 | `instructions [artefact \| apply \| archive] --change --json` | 1 |
| D8 | `validate [item] [--all] [--strict] [--archived] [--json]` | 1 |
| D9 | `archive [change] [--yes] [--skip-specs]` | 1 |
| D10 | `sync [change]` — fusion des deltas sans archiver | 1 |
| D11 | `templates` / `schemas` (`--json`) | 4 |
| D12 | Contrat JSON : un seul document sur stdout, tableau `status[]`, forme nulle en erreur, exit 0/1 | 1 |
| D13 | Globaux : `--version`, `--no-color`, `--help`, `NO_COLOR` | 1 |
| D14 | `view` — tableau de bord interactif | 4 |
| D15 | Complétion shell (bash/zsh/fish) | 4 |
| D16 | `feedback` (issue GitHub) | ✂️ |

## E. Validation

| # | Fonctionnalité | Lot |
|---|---|---|
| E1 | ERREURS : requirement sans `SHALL`/`MUST`, sans scénario, scénario pas en `####`, sections manquantes | 1 |
| E2 | ERREURS de cohérence : doublons, présence croisée `ADDED`/`MODIFIED`/`REMOVED`, collisions `RENAMED` | 1 |
| E3 | Règle « zéro delta » : échec sauf `skip_specs: true` | 1 |
| E4 | AVERTISSEMENTS : `Purpose` trop court ou placeholder, requirement trop long, trop de deltas | 2 |
| E5 | Mode `--strict` | 2 |
| E6 | Préflight de conflit de fusion : `MODIFIED` confronté à la spec principale | 2 |
| E7 | `--archived` : échoue si un change archivé a des tâches non cochées | 2 |
| E8 | Validation parallèle bornée (`--concurrency`) | 4 |

## F. Archive & fusion de specs

| # | Fonctionnalité | Lot |
|---|---|---|
| F1 | Fusion sémantique : `ADDED` ajouté, `MODIFIED` remplacé en place, `REMOVED` supprimé, `RENAMED` retitré, le reste intact | 1 |
| F2 | Création d'une spec principale pour une nouvelle capacité (`## Purpose` du delta) | 1 |
| F3 | Déplacement vers `changes/archive/AAAA-MM-JJ-<nom>/` | 1 |
| F4 | Atomicité : plan validé entièrement avant la première écriture, rollback | 1 |
| F5 | Retraite de capacité, sous `retire_capabilities: true` | 2 |
| F6 | Fusion autonome (`sync`) sans archiver | 1 |
| F7 | Archive en lot, ordre chronologique, conflits inter-changes | 4 |

## G. Skills Claude Code

| # | Fonctionnalité | Lot |
|---|---|---|
| G1 | Catalogue de workflows : `propose`, `explore`, `apply`, `update`, `sync`, `archive` | 1 |
| G2 | Rendu `SKILL.md` : frontmatter `name`, `description`, `allowed-tools: Bash(codev:*)`, `metadata.version` | 1 |
| G3 | Cible Claude Code : `.claude/skills/codev-<workflow>/SKILL.md` | 1 |
| G4 | `trait AgentTarget` | 1 |
| G5 | Version stamp + détection de drift ; fichier édité à la main préservé sauf `--force` | 1 |
| G6 | Sélection des workflows installés via `config.yaml` | 1 |
| G7 | Workflows étendus : `new`, `continue`, `ff`, `verify`, `bulk-archive`, `onboard` | 4 |
| G8 | Cible générique `.agents/skills/` | 4 |
| G9 | Génération de fichiers de commandes séparés | ✂️ (redondant : une skill est déjà invocable en `/nom`) |
| G10 | 40+ adaptateurs d'outils | ✂️ |

## H. Décisions d'architecture

| # | Fonctionnalité | Lot |
|---|---|---|
| H1 | Format ADR : frontmatter (`id`, `title`, `status`, `date`, `supersedes`, `tags`) + sections | 2 |
| H2 | Index des décisions locales et héritées, résolution de `supersedes` / `superseded_by` | 2 |
| H3 | Immuabilité : détection de la modification d'une décision `accepted` | 2 |
| H4 | Injection des décisions en vigueur dans les instructions, `design` en priorité | 2 |
| H5 | `codev decision new / list / show / supersede` | 2 |
| H6 | Déviation `deviates-from` d'une décision héritée, remontée en validation | 2 |
| H7 | Promotion : les décisions durables d'un `design.md` deviennent des ADR | 2 |

## J. Sources héritées

| # | Fonctionnalité | Lot |
|---|---|---|
| J1 | Déclaration `inherits:` — source `path:` (poste) | 1 |
| J2 | Fusion avec précédence `git → poste → projet`, provenance exposée dans `instructions --json` | 1 |
| J3 | Index des specs héritées (id + résumé du `Purpose` + commande de lecture) | 2 |
| J4 | Lecture d'une spec héritée : `codev show <id> --from <source>` | 2 |
| J5 | Source `git:` — résolution `ref` → SHA, `codev.lock` versionné | 3 |
| J6 | Cache adressé par SHA, lecture hors ligne | 3 |
| J7 | `codev sources list / update / show` — diff avant déplacement d'un pin | 3 |
| J8 | Garde-fous : pin obligatoire, aucun contenu exécutable hérité | 3 |

## K. Divers

| # | Fonctionnalité | Lot |
|---|---|---|
| K1 | Prompts interactifs (sélection de change, confirmations) | 2 |
| K2 | `--language` : rédiger les artefacts dans une autre langue | 2 |
| K3 | Vérification de nouvelle version | 4 |
| K4 | Écran d'accueil, palette, spinners | 4 |
| K5 | Télémétrie | ✂️ |

## État

Livré et couvert par des tests : A1–A4, B (rien encore — le parsing n'est pas
requis par la première tranche), C1–C5 et C7, D1 `init`, D2 `update`, D3 `list`,
D5 `new change`, D6 `status`, D7 `instructions` (artefacts), D12, D13, G1–G6,
J1 et J2 (source `path:` uniquement).

Reste du lot 1 : D4 `show`, D8 `validate`, D9 `archive`, D10 `sync`, et donc
tout le bloc B, plus E1–E3 et F1–F4/F6.

## Séquencement

Le lot 1 est lui-même ordonné pour que la valeur arrive tôt. La première
tranche verticale utile ne demande **aucun parsing markdown** — l'agent rédige,
`codev` ne fait que dire quoi écrire et où :

```
codev init  →  codev new change X  →  codev status --json  →  codev instructions <id> --json
```

Cette chaîne suffit à rendre `/codev-propose` pleinement fonctionnel dans Claude
Code. Le parsing (B) n'est requis qu'à partir de `validate`, `show`, `sync` et
`archive`.
