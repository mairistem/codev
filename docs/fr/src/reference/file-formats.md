# Formats de fichiers

Les fichiers de codev sont du Markdown et du YAML destinés à être rédigés par
des personnes et des agents, puis analysés par codev. Ce chapitre en donne les
règles exactes. Les titres et les mots-clés sont toujours en anglais, quelle
que soit la [langue des artefacts](../guides/artifact-language.md).

Tout ce qui figure dans des blocs de code délimités et dans des commentaires
HTML est ignoré par les analyseurs : les templates peuvent donc contenir des
consignes dans des commentaires `<!-- … -->`.

## Spec principale

Chemin : `_codev/specs/<capability>/spec.md`, où `<capability>` est formé
d'un ou plusieurs segments en kebab-case (`ui/theme`).

```markdown
# Theme Specification

## Purpose

Permettre aux utilisateurs de choisir le thème de couleurs de l'interface.

## Requirements

### Requirement: Le thème suit par défaut la préférence du système

L'application SHALL s'afficher avec le jeu de couleurs du système d'exploitation tant que l'utilisateur n'a pas choisi de thème.

#### Scenario: Aucun thème encore choisi

- **GIVEN** un utilisateur qui n'a jamais modifié le réglage du thème
- **WHEN** son système d'exploitation est en mode sombre
- **THEN** l'application s'affiche avec le thème sombre
```

- `## Purpose` est obligatoire.
- Les exigences sont des titres `### Requirement: <name>` placés dans
  `## Requirements`. Le nom identifie l'exigence dans les deltas : veillez à
  ce qu'il soit unique au sein de la spec.
- La description d'une exigence contient `SHALL` ou `MUST`.
- Chaque exigence comporte au moins un `#### Scenario: <name>`, avec
  exactement quatre caractères `#`. Les étapes d'un scénario sont des lignes à
  puces utilisant `**GIVEN**`, `**WHEN**`, `**THEN**` et `**AND**`.
- Les autres sections `##` sont autorisées et préservées lors des fusions.
- Les titres de delta (`## ADDED Requirements`…) sont interdits dans une spec
  principale.

Lorsque l'archivage crée la spec principale d'une nouvelle capacité, il la
titre d'après le dernier segment du chemin de la capacité
(`# Theme Specification`) et recopie la section `## Purpose` du delta.

## Delta de spec

Chemin : `_codev/changes/<name>/specs/<capability>/spec.md`, avec le même
chemin de capacité que la spec principale qu'il modifie.

```markdown
## ADDED Requirements

### Requirement: Le thème choisi est mémorisé

L'application SHALL restaurer le dernier thème choisi par l'utilisateur, sur chacun des appareils depuis lesquels il se connecte.

#### Scenario: Utilisateur de retour

- **GIVEN** un utilisateur qui a choisi le thème sombre sur son ordinateur portable
- **WHEN** il se connecte depuis son téléphone
- **THEN** l'application s'affiche avec le thème sombre

## MODIFIED Requirements

### Requirement: Le thème suit par défaut la préférence du système

<l'exigence complète, scénarios compris, telle qu'elle doit se lire après le change>

## REMOVED Requirements

### Requirement: Ancien sélecteur de couleurs

**Reason**: Remplacé par le réglage du thème.
**Migration**: Les couleurs personnalisées des utilisateurs sont abandonnées ; le thème par défaut s'applique.

## RENAMED Requirements

- FROM: Le thème suit par défaut la préférence du système
- TO: Le thème suit la préférence du système jusqu'au choix de l'utilisateur
```

| Section | Contenu | Fusion |
|---|---|---|
| `## ADDED Requirements` | Nouvelles exigences complètes | Ajoutées à la fin de `## Requirements` |
| `## MODIFIED Requirements` | Exigences complètes, portant un nom existant | Chacune remplace l'exigence de même nom |
| `## REMOVED Requirements` | Un titre `### Requirement:` suivi des lignes `**Reason**:` et `**Migration**:` | L'exigence est supprimée |
| `## RENAMED Requirements` | Paires de lignes `FROM:` / `TO:`, précédées ou non de `- ` | Le titre est renommé ; le contenu et les scénarios restent intacts |

Dans `RENAMED`, écrivez après `FROM:` et `TO:` soit le nom seul de
l'exigence, comme ci-dessus, soit le titre complet entre accents graves, comme
le fait le template : `` - FROM: `### Requirement: <old name>` ``.

Règles vérifiées par la validation :

- Une exigence figure dans une seule section au plus parmi `ADDED`,
  `MODIFIED` et `REMOVED` (`cross_section_conflict`), et au plus une fois par
  section (`duplicate_requirement`).
- Dans une section de delta, chaque titre `###` est un `### Requirement:`
  (`delta_unexpected_heading`) ; une section sans aucune entrée fait l'objet
  d'un avertissement (`delta_section_empty`).
- La source d'un `RENAMED` doit exister dans la spec principale
  (`rename_source_missing`, sauf si le renommage a déjà été appliqué par une
  synchronisation antérieure) ; la cible d'un `RENAMED` ne peut pas figurer
  aussi dans `ADDED` (`rename_target_collision`) ; et `MODIFIED` emploie le nouveau nom d'une
  exigence renommée, et non l'ancien (`modified_uses_old_name`).
- Les noms cités dans `MODIFIED` et `REMOVED` doivent exister dans la spec
  principale ; cette vérification a lieu lors de la fusion du delta
  (`modified_target_missing`).
- **Nouvelle capacité :** commencez le delta par `## Purpose`. Pour une
  capacité existante, omettez cette section : le Purpose de la spec principale
  est conservé.
- Un delta `REMOVED` qui laisserait une spec sans aucune exigence est refusé
  (`would_leave_spec_without_requirement`), sauf si le change déclare
  `retire_capabilities: true` ; la spec principale est alors supprimée.

## proposal.md

Le template du schéma `spec-driven` :

```markdown
# Proposal: <change title>

## Why

## What Changes

## Capabilities

### New Capabilities

### Modified Capabilities

### Removed Capabilities

## Impact
```

Chaque capacité listée sous *New Capabilities* ou *Modified Capabilities*
reçoit son propre fichier de delta dans `specs/<capability>/spec.md`. Les
changements incompatibles sont signalés par **BREAKING** dans *What Changes*.
Une capacité listée sous *Removed Capabilities* exige
`retire_capabilities: true`.

## design.md

```markdown
# Design: <change title>

## Context

## Goals / Non-Goals

## Decisions

### Decision: <the choice>

**Alternatives considered**:

## Risks / Trade-offs

## Migration Plan

## Open Questions
```

Chaque bloc `### Decision: <title>` peut être promu en ADR avec
`codev decision promote <change> "<title>"`. *Risks / Trade-offs* suit la
forme `[Risk] → Mitigation`. *Migration Plan* et *Open Questions* sont
supprimées lorsqu'elles ne s'appliquent pas.

## tasks.md

```markdown
# Tasks

## 1. Jetons de thème

- [ ] 1.1 Déplacer chaque couleur dans une propriété CSS personnalisée, vérifié par un diff visuel de la page des réglages
- [ ] 1.2 Ajouter le jeu de jetons sombres, vérifié par `npm test -- theme`
```

Les tâches sont regroupées sous des titres `##` numérotés et s'écrivent
`- [ ] X.Y <task>` ; `/codev-apply` les coche sous la forme `- [x]`. Chaque
tâche indique comment la vérifier.

## change.yaml

Chemin : `_codev/changes/<name>/change.yaml`. Écrit par `codev new change`.

```yaml
schema: spec-driven
created: 2026-09-30
goal: Let users switch the UI to a dark theme
```

| Clé | Type | Signification |
|---|---|---|
| `schema` | chaîne, obligatoire | Schéma du change |
| `created` | date | Date de création, `YYYY-MM-DD` |
| `goal` | chaîne | L'objectif fourni avec `--goal` |
| `skip_specs` | booléen | Le change n'a volontairement aucun delta de spec : les artefacts qui écrivent sous `specs/` sont ignorés, et la validation accepte l'absence de deltas |
| `retire_capabilities` | booléen | Autorise la synchronisation et l'archivage à supprimer une spec principale dont le change retire la dernière exigence |

Les clés inconnues sont rejetées. Un dossier de change dépourvu de
`change.yaml` utilise le schéma du projet.

## Décision (ADR)

Chemin : `_codev/decisions/NNNN-<slug>.md`. Chaque fichier `.md` de ce
dossier est analysé comme une décision.

```markdown
---
id: "0003"
title: "Use SQLite for persistence"
status: accepted
date: 2026-09-30
tags: [storage]
supersedes: ["0001"]
---

## Context

## Decision

## Consequences

## Alternatives considered
```

| Champ | Obligatoire | Signification |
|---|---|---|
| `id` | oui | Identifiant court, par convention sur quatre chiffres ; mettez-le entre guillemets pour que YAML le conserve comme chaîne |
| `title` | oui | Titre |
| `status` | oui | `accepted`, `superseded`, `proposed`, `deprecated` ou `rejected` |
| `date` | oui | `YYYY-MM-DD` |
| `tags` | non | Liste d'étiquettes |
| `supersedes` | non | Identifiant, ou liste d'identifiants, des décisions que celle-ci remplace |
| `deviates_from` | non | Liste des identifiants qualifiés des décisions héritées dont celle-ci s'écarte |

Le frontmatter est délimité par des lignes `---` placées tout au début du
fichier. Le contenu est découpé en sections `##` ; les quatre sections du
template relèvent de la convention, et non de l'obligation. Seules les
décisions `accepted` qui ne sont ni remplacées ni l'objet d'un écart sont en
vigueur, et `supersedes` comme `deviates_from` ne prennent effet que sur une
décision `accepted` : un remplacement ou un écart `proposed` ne change rien
avant `codev decision accept`.

## seal.yaml

Chemin : `_codev/decisions/seal.yaml`. Maintenu par codev ; ne le modifiez
pas à la main.

```yaml
version: 1
seals:
- id: '0001'
  bodySha256: sha256:fad1f272bf13eb44f97aba1bc22f36f294503d1a2471bf12ab4bfc6ee7b4d7f6
  sealedAt: 2026-09-30
```

Chaque entrée enregistre l'empreinte SHA-256 du contenu d'une décision — tout
ce qui suit le `---` fermant du frontmatter, octet pour octet — et la date de
son scellement. Commitez ce fichier avec vos décisions.

## codev.lock

Chemin : `_codev/codev.lock`. Écrit uniquement par `codev sources update`, au
format TOML.

```toml
version = 1

[[source]]
git = "git@github.com:acme/codev-standards.git"
ref = "main"
commit = "12958274cbed0eb66985f57d3272c6d3134b672d"
resolved_at = "2026-09-30"
```

Une entrée `[[source]]` par source `git:`, avec la `ref` qu'elle suit, le
commit sur lequel elle est épinglée et, le cas échéant, son `subpath`.
Commitez ce fichier.

## SKILL.md

Chemin : `.claude/skills/codev-<workflow>/SKILL.md`. Généré par `codev init`
et `codev update` :

```markdown
---
name: codev-explore
description: "Explore an idea, investigate a problem or clarify a need before creating a codev change. …"
allowed-tools: "Bash(codev:*), Read, Glob, Grep"
license: MIT
metadata:
  generator: codev
  version: "0.4.0"
---

<workflow instructions>
```

C'est grâce à `metadata.version` que `codev update` distingue une skill issue
d'une version antérieure (régénérée) d'une skill modifiée à la main
(préservée, sauf avec `--force`).

## Codes de validation

Chaque constat signalé par `codev validate` porte un code stable. Les messages
peuvent être reformulés d'une version à l'autre ; les codes, non.

| Code | Gravité | Signification |
|---|---|---|
| `spec_purpose_missing` | erreur | Une spec principale n'a pas de `## Purpose` |
| `spec_no_requirement` | erreur | Une spec principale ne contient aucune exigence |
| `requirement_outside_section` | erreur | Un `### Requirement:` en dehors de `## Requirements` |
| `delta_header_in_main_spec` | erreur | Un titre de delta dans une spec principale |
| `requirement_no_shall` | erreur | Une exigence sans `SHALL` ni `MUST` |
| `requirement_no_scenario` | erreur | Une exigence sans scénario |
| `scenario_wrong_heading_level` | erreur | Un scénario écrit avec trois `#` au lieu de quatre |
| `duplicate_requirement` | erreur | La même exigence deux fois dans une même section de delta |
| `delta_unexpected_heading` | erreur | Un titre `###` d'une section de delta qui n'est pas un `### Requirement:`, par exemple un mot-clé traduit |
| `cross_section_conflict` | erreur | La même exigence dans deux sections parmi `ADDED`, `MODIFIED`, `REMOVED` |
| `rename_source_missing` | erreur | La source d'un `RENAMED` ne désigne aucune exigence de la spec principale |
| `rename_target_collision` | erreur | Une cible de `RENAMED` figure aussi dans `ADDED` |
| `modified_uses_old_name` | erreur | `MODIFIED` emploie l'ancien nom d'une exigence renommée |
| `zero_delta_without_marker` | erreur | Un change n'a aucun delta et ne déclare pas `skip_specs: true` |
| `skip_specs_conflict` | erreur | Un change déclare `skip_specs: true` mais comporte des deltas |
| `decision_missing_frontmatter` | erreur | Un fichier de `_codev/decisions/` n'a pas de frontmatter |
| `decision_missing_field` | erreur | Il manque à une décision un champ obligatoire, ou elle en comporte un inconnu |
| `decision_unknown_status` | erreur | Une décision a un statut inconnu |
| `decision_field_type_mismatch` | erreur | Un champ du frontmatter n'a pas la forme attendue |
| `decision_seal_mismatch` | erreur | Le contenu d'une décision scellée a été modifié |
| `decision_conflicting_deviations` | erreur | Deux décisions locales acceptées s'écartent de la même décision héritée |
| `delta_section_empty` | avertissement | Une section de delta ne comporte aucune entrée : rien n'en est fusionné |
| `decision_unsealed` | avertissement | Une décision locale acceptée ou remplacée n'a pas de sceau |
| `decision_orphan_seal` | avertissement | Un sceau ne correspond à aucune décision |
| `decision_supersedes_unknown` | avertissement | `supersedes` désigne une décision inconnue |
| `decision_id_collision` | avertissement | Une décision locale et une décision héritée partagent un identifiant |
| `decision_dangling_deviation` | avertissement | `deviates_from` désigne une décision non indexée |
| `decision_supersession_cycle` | avertissement | Les remplacements forment un cycle ; aucune des décisions concernées n'est en vigueur |
