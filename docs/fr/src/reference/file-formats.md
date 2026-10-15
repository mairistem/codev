# Formats de fichiers

Les fichiers de codev sont du Markdown et du YAML destinés à être rédigés par
des personnes et des agents, puis analysés par codev. Ce chapitre en donne les
règles exactes, avec les mots-clés anglais. codev lit aussi leurs équivalents
français (`### Exigence :`, `## Exigences AJOUTÉES`, `DOIT`…), présentés dans
[Langue des artefacts](../guides/artifact-language.md).

Tout ce qui figure dans des blocs de code délimités et dans des commentaires
HTML est ignoré par les analyseurs : les templates peuvent donc contenir des
consignes dans des commentaires `<!-- … -->`.

## Spec principale

Chemin : `_codev/specs/<capability>/spec.md`, où `<capability>` est formé
d'un ou plusieurs segments en kebab-case (`ui/theme`). Seul un fichier nommé
exactement `spec.md` est une spec principale : les autres fichiers de
`_codev/specs/`, comme un `README.md`, sont ignorés par `codev list --specs`
et `codev validate`.

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
- La description d'une exigence contient un mot-clé normatif en majuscules :
  `SHALL` ou `MUST`, ou `DOIT` ou `DOIVENT`.
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

## Domains

## Release note
```

Chaque capacité listée sous *New Capabilities* ou *Modified Capabilities*
reçoit son propre fichier de delta dans `specs/<capability>/spec.md`. Les
changements incompatibles sont signalés par **BREAKING** dans *What Changes*.
Une capacité listée sous *Removed Capabilities* exige
`retire_capabilities: true`.

### Release note

Ce que les utilisateurs liront dans le changelog, construit par
[`codev changelog`](cli.md#codev-changelog). Soit un ou plusieurs éléments :

```markdown
## Release note

- Added: `--format json` pour `codev export`.
- Changed: **BREAKING** `codev export` écrit sur stdout par défaut ; passez
  `--output <PATH>` pour écrire un fichier.
```

soit, pour un change que les utilisateurs ne remarqueront pas, la seule
ligne :

```markdown
## Release note

None
```

Chaque élément s'écrit `- <Category>: <texte>`, la catégorie étant l'une de
`Added`, `Changed`, `Deprecated`, `Removed`, `Fixed` et `Security` — les
catégories de [Keep a Changelog](https://keepachangelog.com/fr/1.1.0/). Un
élément peut s'étendre sur plusieurs lignes. Un élément incompatible commence
son texte par `**BREAKING**`. Le texte suit la langue des artefacts du
projet. `codev validate` signale une section absente par un avertissement, et
une catégorie inconnue, un élément sans texte ou une section vide par une
erreur, sur les changes actifs uniquement.

### Domains

Les domaines que touche le change, qui activent leurs
[experts](../roles.md#experts-et-domaines). Le modèle liste les six
domaines non cochés ; cochez ceux que touche le change :

```markdown
## Domains

- [x] security
- [ ] personal-data
- [ ] user-interface
- [ ] performance
- [x] operations — un nouveau worker en arrière-plan
- [ ] migration
```

ou, lorsqu'il n'en touche aucun, écrivez la seule ligne `None`.

`- [x] <domaine>` ou `- <domaine>` coche un domaine, `- [ ] <domaine>` le
laisse non coché, et le texte qui suit le nom du domaine est un commentaire
libre. Les domaines sont `security`, `personal-data`, `user-interface`,
`performance`, `operations` et `migration`. `codev validate` signale, sur
les changes actifs uniquement, un domaine inconnu (`unknown_domain`) et une
ligne qui n'est ni un élément ni `None`, ou `None` à côté d'un domaine coché
(`domains_invalid`), par une erreur, et une section absente
(`domains_missing`) par un avertissement.

Ce commentaire est aussi la façon de répondre à un
[domaine suggéré](../roles.md#les-déclencheurs-et-les-domaines-suggérés)
que le change ne touche pas : un domaine non coché suivi d'une raison est
`justified` — `- [ ] security — only renames a variable`. Les tirets,
deux-points et espaces de tête ne font pas partie de la raison ; `None` ne
justifie rien. `codev validate` signale chaque domaine suggéré ni coché ni
justifié par `domain_suggested`, un avertissement à la ligne du domaine.

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
`<name>` n'est jamais `archive`, le dossier des changes archivés.

```yaml
schema: spec-driven
created: 2026-09-30
goal: Let users switch the UI to a dark theme
```

| Clé | Type | Signification |
|---|---|---|
| `schema` | chaîne, obligatoire | Schéma du change : `spec-driven`, ou le nom du dossier d'un schéma du projet sous `_codev/schemas/` |
| `created` | date | Date de création, `YYYY-MM-DD` |
| `goal` | chaîne | L'objectif fourni avec `--goal` |
| `skip_specs` | booléen | Le change n'a volontairement aucun delta de spec : les artefacts qui écrivent sous `specs/` sont ignorés, et la validation accepte l'absence de deltas |
| `retire_capabilities` | booléen | Autorise la synchronisation et l'archivage à supprimer une spec principale dont le change retire la dernière exigence |
| `roles` | table | Activations de rôles pour ce change seulement, par-dessus le [`roles:`](configuration.md#roles) du projet : `newcomer: always`. Activations uniquement — les critères relèvent du projet |
| `review` | table | `mode: inline` ou `mode: isolated` pour ce change seulement, par-dessus le [`review:`](configuration.md#review) du projet. Une autre valeur échoue avec `invalid_review_mode` |

Les clés inconnues sont rejetées — dans `review` aussi. Un codev antérieur à
`review` refuse un `change.yaml` qui la définit : relevez le
[`codev_version`](configuration.md#codev_version) du projet jusqu'à la
version qui l'a introduite avant de l'utiliser. Un dossier de change dépourvu de
`change.yaml` utilise le schéma du projet. Une entrée invalide de `roles` est
signalée par `codev validate` comme une erreur du change (`unknown_role`,
`invalid_role_activation`, `role_not_disableable`).

Les artefacts d'un `schema.yaml` peuvent déclarer leur [rôle](../roles.md)
comme une donnée, sous `role:` — voir
[Schémas personnalisés](../guides/custom-schemas.md#déclarer-les-rôles-des-artefacts).

## review.md

Chemin : `_codev/changes/<name>/review.md`. Le
[compte rendu de revue](../guides/review-record.md) du change, écrit par
`/codev-propose` (`plan.challenge`) et `/codev-apply` (`code.review`), ou à
la main. Ce n'est pas un artefact de planification : `codev status`
l'ignore, `codev sync` le laisse en place, `codev archive` le déplace avec
le change.

````markdown
# Review record: add-sessions

<!-- codev-review: 1 -->

## plan.challenge

### skeptic

No finding.

### security

#### R-security-1 Session tokens are stored in clear

```yaml
severity: high
confidence: high
location: "design.md#Decision: Sessions live in the database"
evidence: |
  The session token is stored in the `sessions` table.
recommendation: Store a SHA-256 of the token and compare digests.
history:
  - status: open
    by: security
  - status: fixed
    by: architect
    ref: design.md, the decision now stores a digest
  - status: fixed
    by: security
    note: verified
```

## code.review

### newcomer

Disabled by configuration.
````

Les titres et les lignes `No finding.` et `Disabled by configuration.`
restent en anglais : codev les lit.

| Élément | Règle |
|---|---|
| `<!-- codev-review: <n> -->` | Marqueur de format facultatif ; sans lui, format 1. Un format supérieur n'est pas vérifié et bloque l'archivage |
| `## <step>` | `plan.challenge` ou `code.review`, une fois chacun |
| `### <role-id>` | Un par rôle sous une étape, une fois chacun. Contient des constats, ou la ligne `No finding.`, ou la ligne `Disabled by configuration.` |
| `#### <id> <title>` | Un constat : `R-<role-id>-<n>`, unique dans le compte rendu, jamais réutilisé, suivi d'un unique bloc `yaml` |
| `Mode: <mode>.` | Facultatif, sous une étape : `inline` ou `isolated`, la manière dont l'étape a été relue. Une fois par étape |
| `Summarized: <texte>.` | Facultatif, sous une étape, en nombre quelconque : un fichier de la matière que les relecteurs ont reçu sous forme de résumé |
| `Fallback: <raison>.` | Facultatif, sous un rôle, une fois : pourquoi le rôle a relu en ligne lors d'une revue isolée |

| Champ | Type | Signification |
|---|---|---|
| `severity` | `high`, `medium` ou `low` | Voir [l'échelle](../roles.md#gravité-confiance-et-compte-rendu-de-revue) |
| `confidence` | `high` ou `low` | Un constat `low` n'est pas présenté à la personne |
| `location` | chaîne | `<path>:<line>` ou `<path>:<start>-<end>` depuis la racine du projet, ou `<file>#<heading>` depuis le dossier du change. Relatif, sans `..`, sans lecteur ; `\` se lit `/`. Mettez-le entre guillemets s'il contient `: ` |
| `evidence` | chaîne | Le texte cité, mot pour mot : des lignes entières ou un fragment de chacune |
| `recommendation` | chaîne | Ce qu'il faut faire |
| `history` | liste | Les réponses, de la plus ancienne à la plus récente ; la dernière donne le statut |
| `raised_by` | liste, facultative | Chaque rôle qui a relevé le constat, son propre rôle en premier, chacun une fois : un constat fusionné à partir de plusieurs relecteurs lors d'une [revue isolée](../guides/review-record.md#les-revues-isolées) |

| Clé de `history[]` | Signification |
|---|---|
| `status` | `open`, `fixed`, `disputed`, `accepted-risk`, `deferred` ou `rejected` |
| `by` | Un identifiant de rôle, `human` ou `human:<name>` |
| `ref` | Le commit ou la modification de fichier — exigé par `fixed` après `open` ou `disputed` |
| `note` | L'argument (`disputed`), la justification (`accepted-risk`, `by` une personne), ou pourquoi une correction ne tient pas (`open` après `fixed`) |
| `to` | Où va un constat `deferred` : un change ou un ticket. Un constat `high` est reporté par une personne |

Un champ que ce codev ne connaît pas est un avertissement,
`review_unknown_field`, et il est ignoré : un codev ultérieur peut ajouter
des champs facultatifs sans changer de format. Un nouveau statut, une
nouvelle étape ou un nouveau champ obligatoire relève au contraire le
marqueur de format, qu'un codev plus ancien refuse de vérifier. Les transitions et les contrôles
sont dans [Compte rendu de revue](../guides/review-record.md#ce-que-codev-vérifie).

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

Les fichiers enregistrés avec des fins de ligne Windows (CRLF) sont lus comme
les autres. Lorsque codev réécrit le statut d'une décision — `codev decision
accept`, ou un prédécesseur passé en `superseded` —, il conserve les fins de
ligne du fichier et laisse le corps identique à l'octet près : le sceau reste
valide.

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
  version: "0.6.1"
  checksum: "8d676b09…"
---

<workflow instructions>
```

`metadata.checksum` est l'empreinte SHA-256 de toute la skill, hors de cette
ligne. Avec `metadata.version`, c'est ce qui permet à `codev update` de
distinguer les rendus de codev de vos modifications. `codev update` réécrit
une skill marquée d'une autre version ; parmi les skills marquées de sa propre
version, il réécrit celle dont le contenu correspond encore à son `checksum` —
un rendu de codev, d'une compilation antérieure ou pour l'autre [mode de
revue](configuration.md#review) — et préserve celle qui n'y correspond plus,
modifiée à la main, sauf si vous passez `--force`. Une skill sans checksum,
rendue avant leur existence, est préservée quand elle porte la marque de cette
version et diffère du rendu par plus que le mode de revue : lancez une fois
`codev update --force` pour la doter d'un checksum — il écrase aussi une skill
que vous avez réellement modifiée : vérifiez d'abord `git diff`, puis
réappliquez vos modifications ; une modification faite ensuite est préservée,
puisqu'elle ne correspond plus au checksum.

## Codes de validation

Chaque constat signalé par `codev validate` porte un code stable. Les messages
peuvent être reformulés d'une version à l'autre ; les codes, non.

| Code | Gravité | Signification |
|---|---|---|
| `spec_purpose_missing` | erreur | Une spec principale n'a pas de `## Purpose` |
| `spec_no_requirement` | erreur | Une spec principale ne contient aucune exigence |
| `requirement_outside_section` | erreur | Un `### Requirement:` en dehors de `## Requirements` |
| `delta_header_in_main_spec` | erreur | Un titre de delta dans une spec principale |
| `requirement_no_shall` | erreur | Une exigence sans `SHALL`, `MUST`, `DOIT` ni `DOIVENT` |
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
| `release_note_invalid` | erreur | La section `## Release note` de la proposal a une catégorie inconnue, un élément sans texte, ou est vide |
| `release_note_missing` | avertissement | La proposal d'un change actif n'a pas de section `## Release note` |
| `unknown_domain` | erreur | La section `## Domains` de la proposal nomme un domaine hors des six |
| `domains_invalid` | erreur | La section `## Domains` de la proposal a une ligne qui n'est ni un élément ni `None`, ou `None` à côté d'un domaine coché |
| `domains_missing` | avertissement | La proposal d'un change actif n'a pas de section `## Domains` |
| `domain_suggested` | avertissement | Le diff du change suggère un domaine que la proposal ne coche pas et ne garde pas non coché avec une raison |
| `invalid_trigger` | erreur | Un groupe de `triggers:` dans `_codev/config.yaml` n'a pas de domaine connu, pas de déclencheur, ou un glob ou une expression qui ne compile pas (un avertissement pour une clé que codev ne connaît pas) |
| `review_invalid` | erreur | La structure du compte rendu de revue, un champ, un identifiant ou un emplacement de constat, un statut ou une transition est incorrect |
| `review_decision_incomplete` | erreur | Une entrée d'historique n'a pas son `ref`, sa `note` ou son `to`, ou un risque est accepté, ou un constat haut reporté, par quelqu'un d'autre qu'une personne |
| `review_location_missing` | erreur | Un constat cite un fichier, une ligne ou un titre qui n'existe pas |
| `review_evidence_not_found` | erreur | La preuve d'un constat ne figure nulle part dans le fichier ou la section cités |
| `review_role_missing` | erreur | Un rôle actif à une étape consignée n'a pas de section, ou en a une qui le dit désactivé |
| `review_evidence_moved` | avertissement | La preuve d'un constat est dans le fichier, à plus de 5 lignes des lignes citées |
| `review_evidence_unchecked` | avertissement | Le fichier cité n'est pas du texte UTF-8, contient des octets NUL, ou dépasse 4 Mio : la preuve n'est pas comparée |
| `review_unknown_section` | avertissement | Une étape ou un rôle que le compte rendu ne devrait pas contenir ; il est ignoré |
| `review_unknown_field` | avertissement | Un champ de constat que ce codev ne connaît pas ; il est ignoré |
| `review_format_unsupported` | avertissement | Le compte rendu vient d'un codev plus récent ; rien n'y est vérifié |
| `delta_section_empty` | avertissement | Une section de delta ne comporte aucune entrée : rien n'en est fusionné |
| `decision_unsealed` | avertissement | Une décision locale acceptée ou remplacée n'a pas de sceau |
| `unknown_config_key` | avertissement | `_codev/config.yaml` contient une clé de premier niveau ou de `mcp` que ce codev ne connaît pas — voir [Clés inconnues](configuration.md#clés-inconnues) |
| `decision_orphan_seal` | avertissement | Un sceau ne correspond à aucune décision |
| `decision_supersedes_unknown` | avertissement | `supersedes` désigne une décision inconnue |
| `decision_id_collision` | avertissement | Une décision locale et une décision héritée partagent un identifiant |
| `decision_dangling_deviation` | avertissement | `deviates_from` désigne une décision non indexée |
| `decision_supersession_cycle` | avertissement | Les remplacements forment un cycle ; aucune des décisions concernées n'est en vigueur |
