# Formats de fichiers

Les fichiers de Compagnon sont du Markdown et du YAML destinés à être rédigés par
des personnes et des agents, puis analysés par Compagnon. Ce chapitre en donne les
règles exactes, avec les mots-clés anglais. Compagnon lit aussi leurs équivalents
français (`### Exigence :`, `## Exigences AJOUTÉES`, `DOIT`…), présentés dans
[Langue des artefacts](../guides/artifact-language.md).

Tout ce qui figure dans des blocs de code délimités et dans des commentaires
HTML est ignoré par les analyseurs : les modèles peuvent donc contenir des
consignes dans des commentaires `<!-- … -->`.

## Spécification principale

Chemin : `_compagnon/specs/<capability>/spec.md`, où `<capability>` est formé
d'un ou plusieurs segments en kebab-case (`ui/theme`). Seul un fichier nommé
exactement `spec.md` est une spécification principale : les autres fichiers de
`_compagnon/specs/`, comme un `README.md`, sont ignorés par `compagnon list --specs`
et `compagnon validate`.

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
  ce qu'il soit unique au sein de la spécification.
- La description d'une exigence contient un mot-clé normatif en majuscules :
  `SHALL` ou `MUST`, ou `DOIT` ou `DOIVENT`.
- Chaque exigence comporte au moins un `#### Scenario: <name>`, avec
  exactement quatre caractères `#`. Les étapes d'un scénario sont des lignes à
  puces utilisant `**GIVEN**`, `**WHEN**`, `**THEN**` et `**AND**`.
- Les autres sections `##` sont autorisées et préservées lors des fusions.
- Les titres de delta (`## ADDED Requirements`…) sont interdits dans une spécification
  principale.

Lorsque l'archivage crée la spécification principale d'une nouvelle capacité, il la
titre d'après le dernier segment du chemin de la capacité
(`# Theme Specification`) et recopie la section `## Purpose` du delta.

## Delta de spécification

Chemin : `_compagnon/changes/<name>/specs/<capability>/spec.md`, avec le même
chemin de capacité que la spécification principale qu'il modifie.

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
le fait le modèle : `` - FROM: `### Requirement: <old name>` ``.

Règles vérifiées par la validation :

- Une exigence figure dans une seule section au plus parmi `ADDED`,
  `MODIFIED` et `REMOVED` (`cross_section_conflict`), et au plus une fois par
  section (`duplicate_requirement`).
- Dans une section de delta, chaque titre `###` est un `### Requirement:`
  (`delta_unexpected_heading`) ; une section sans aucune entrée fait l'objet
  d'un avertissement (`delta_section_empty`).
- La source d'un `RENAMED` doit exister dans la spécification principale
  (`rename_source_missing`, sauf si le renommage a déjà été appliqué par une
  synchronisation antérieure) ; la cible d'un `RENAMED` ne peut pas figurer
  aussi dans `ADDED` (`rename_target_collision`) ; et `MODIFIED` emploie le nouveau nom d'une
  exigence renommée, et non l'ancien (`modified_uses_old_name`).
- Les noms cités dans `MODIFIED` et `REMOVED` doivent exister dans la spécification
  principale ; cette vérification a lieu lors de la fusion du delta
  (`modified_target_missing`).
- **Nouvelle capacité :** commencez le delta par `## Purpose`. Pour une
  capacité existante, omettez cette section : le Purpose de la spécification principale
  est conservé.
- Un delta `REMOVED` qui laisserait une spécification sans aucune exigence est refusé
  (`would_leave_spec_without_requirement`), sauf si la modification déclare
  `retire_capabilities: true` ; la spécification principale est alors supprimée.

## proposal.md

Le schéma `spec-driven` fournit ses modèles en anglais et en français, et
sert celui de la langue des mots-clés du projet
([Langue des artefacts](../guides/artifact-language.md)). Le modèle
anglais :

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
[`compagnon changelog`](cli.md#compagnon-changelog). Soit un ou plusieurs éléments :

```markdown
## Release note

- Added: `--format json` pour `compagnon export`.
- Changed: **BREAKING** `compagnon export` écrit sur stdout par défaut ; passez
  `--output <PATH>` pour écrire un fichier.
```

soit, pour une modification que les utilisateurs ne remarqueront pas, la seule
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
projet. `compagnon validate` signale une section absente par un avertissement, et
une catégorie inconnue, un élément sans texte ou une section vide par une
erreur, sur les modifications actives uniquement.

### Domains

Les domaines que touche la modification, qui activent leurs
[experts](../roles.md#experts-et-domaines). Le modèle liste les six
domaines non cochés ; cochez ceux que touche la modification :

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
`performance`, `operations` et `migration`. `compagnon validate` signale, sur
les modifications actives uniquement, un domaine inconnu (`unknown_domain`) et une
ligne qui n'est ni un élément ni `None`, ou `None` à côté d'un domaine coché
(`domains_invalid`), par une erreur, et une section absente
(`domains_missing`) par un avertissement.

Ce commentaire est aussi la façon de répondre à un
[domaine suggéré](../roles.md#les-déclencheurs-et-les-domaines-suggérés)
que la modification ne touche pas : un domaine non coché suivi d'une raison est
`justified` — `- [ ] security — only renames a variable`. Les tirets,
deux-points et espaces de tête ne font pas partie de la raison ; `None` ne
justifie rien. `compagnon validate` signale chaque domaine suggéré ni coché ni
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
`compagnon decision promote <change> "<title>"`. *Risks / Trade-offs* suit la
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
`- [ ] X.Y <task>` ; `/compagnon-apply` les coche sous la forme `- [x]`. Chaque
tâche indique comment la vérifier.

## change.yaml

Chemin : `_compagnon/changes/<name>/change.yaml`. Écrit par `compagnon new change`.
`<name>` n'est jamais `archive`, le dossier des modifications archivées.

```yaml
schema: spec-driven
created: 2026-09-30
goal: Let users switch the UI to a dark theme
```

| Clé | Type | Signification |
|---|---|---|
| `schema` | chaîne, obligatoire | Schéma de la modification : `spec-driven`, ou le nom du dossier d'un schéma du projet sous `_compagnon/schemas/` |
| `created` | date | Date de création, `YYYY-MM-DD` |
| `goal` | chaîne | L'objectif fourni avec `--goal` |
| `skip_specs` | booléen | La modification n'a volontairement aucun delta de spécification : les artefacts qui écrivent sous `specs/` sont ignorés, et la validation accepte l'absence de deltas |
| `retire_capabilities` | booléen | Autorise la synchronisation et l'archivage à supprimer une spécification principale dont la modification retire la dernière exigence |
| `roles` | table | Activations de rôles pour cette modification seulement, par-dessus le [`roles:`](configuration.md#roles) du projet : `newcomer: always`. Activations uniquement — les critères relèvent du projet |
| `review` | table | `mode: inline` ou `mode: isolated` pour cette modification seulement, par-dessus le [`review:`](configuration.md#review) du projet. Une autre valeur échoue avec `invalid_review_mode` |

Une clé que Compagnon ne connaît pas — au premier niveau, ou sous `review`
(`review.<clé>`) — est ignorée avec un avertissement `change_unknown_key` qui
la nomme, cite le `compagnon_version` du projet et renvoie à
`compagnon self-update` ; `compagnon validate --strict` échoue dessus, à sa ligne. La
modification a peut-être été écrite avec une version plus récente de Compagnon, ou la
clé est mal orthographiée. Une valeur que Compagnon ne connaît pas — un
`review.mode`, une activation de rôle — reste une erreur. Compagnon 0.6.1 et les
versions antérieures rejettent toute clé inconnue de ce fichier : un Compagnon
antérieur à `review` refuse donc un `change.yaml` qui la définit. Relevez le
[`compagnon_version`](configuration.md#compagnon_version) du projet jusqu'à la
version qui l'a introduite avant de l'utiliser. Un dossier de modification dépourvu de
`change.yaml` utilise le schéma du projet. Une entrée invalide de `roles` est
signalée par `compagnon validate` comme une erreur de la modification (`unknown_role`,
`invalid_role_activation`, `role_not_disableable`).

Les artefacts d'un `schema.yaml` peuvent déclarer leur [rôle](../roles.md)
comme une donnée, sous `role:` — voir
[Schémas personnalisés](../guides/custom-schemas.md#déclarer-les-rôles-des-artefacts).
Une clé d'un schéma de projet que Compagnon ne connaît pas, à tout niveau, est
ignorée avec un avertissement `schema_unknown_key` qui nomme la clé et sa
ligne.

## review.md

Chemin : `_compagnon/changes/<name>/review.md`. Le
[compte rendu de relecture](../guides/review-record.md) de la modification, écrit par
`/compagnon-propose` (`plan.challenge`) et `/compagnon-apply` (`code.review`), ou à
la main. Ce n'est pas un artefact de planification : `compagnon status`
l'ignore, `compagnon sync` le laisse en place, `compagnon archive` le déplace avec
la modification.

````markdown
# Review record: add-sessions

<!-- compagnon-review: 1 -->

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
restent en anglais : Compagnon les lit.

| Élément | Règle |
|---|---|
| `<!-- compagnon-review: <n> -->` | Marqueur de format facultatif ; sans lui, format 1. Un format supérieur n'est pas vérifié et bloque l'archivage |
| `## <step>` | `plan.challenge` ou `code.review`, une fois chacun |
| `### <role-id>` | Un par rôle sous une étape, une fois chacun. Contient des constats, ou la ligne `No finding.`, ou la ligne `Disabled by configuration.` |
| `#### <id> <title>` | Un constat : `R-<role-id>-<n>`, unique dans le compte rendu, jamais réutilisé, suivi d'un unique bloc `yaml` |
| `Mode: <mode>.` | Facultatif, sous une étape : `inline` ou `isolated`, la manière dont l'étape a été relue. Une fois par étape |
| `Summarized: <texte>.` | Facultatif, sous une étape, en nombre quelconque : un fichier de la matière que les relecteurs ont reçu sous forme de résumé |
| `Fallback: <raison>.` | Facultatif, sous un rôle, une fois : pourquoi le rôle a relu en ligne lors d'une relecture isolée |

| Champ | Type | Signification |
|---|---|---|
| `severity` | `high`, `medium` ou `low` | Voir [l'échelle](../roles.md#gravité-confiance-et-compte-rendu-de-relecture) |
| `confidence` | `high` ou `low` | Un constat `low` n'est pas présenté à la personne |
| `location` | chaîne | `<path>:<line>` ou `<path>:<start>-<end>` depuis la racine du projet, ou `<file>#<heading>` depuis le dossier de la modification. Relatif, sans `..`, sans lecteur ; `\` se lit `/`. Mettez-le entre guillemets s'il contient `: ` |
| `evidence` | chaîne | Le texte cité, mot pour mot : des lignes entières ou un fragment de chacune |
| `recommendation` | chaîne | Ce qu'il faut faire |
| `history` | liste | Les réponses, de la plus ancienne à la plus récente ; la dernière donne le statut |
| `raised_by` | liste, facultative | Chaque rôle qui a relevé le constat, son propre rôle en premier, chacun une fois : un constat fusionné à partir de plusieurs relecteurs lors d'une [relecture isolée](../guides/review-record.md#les-relectures-isolées) |

| Clé de `history[]` | Signification |
|---|---|
| `status` | `open`, `fixed`, `disputed`, `accepted-risk`, `deferred` ou `rejected` |
| `by` | Un identifiant de rôle, `human` ou `human:<name>` |
| `ref` | L'enregistrement ou la modification de fichier — exigé par `fixed` après `open` ou `disputed` |
| `note` | L'argument (`disputed`), la justification (`accepted-risk`, `by` une personne), ou pourquoi une correction ne tient pas (`open` après `fixed`) |
| `to` | Où va un constat `deferred` : une modification ou un ticket. Un constat `high` est reporté par une personne |

Un champ que ce Compagnon ne connaît pas est un avertissement,
`review_unknown_field`, et il est ignoré : un Compagnon ultérieur peut ajouter
des champs facultatifs sans changer de format. Un nouveau statut, une
nouvelle étape ou un nouveau champ obligatoire relève au contraire le
marqueur de format, qu'un Compagnon plus ancien refuse de vérifier. Les transitions et les contrôles
sont dans [Compte rendu de relecture](../guides/review-record.md#ce-que-compagnon-vérifie).

## corrections.md

Chemin : `_compagnon/changes/<name>/corrections.md`. Les consignes que
l'auteur a données à l'agent sur la manière de faire du projet, notées par
`/compagnon-propose`, `/compagnon-apply` et `/compagnon-update` — voir
[les corrections de l'auteur](../skills.md#les-corrections-de-lauteur).
Comme `review.md`, ce n'est pas un artefact de planification : `compagnon
status` et `compagnon changelog` l'ignorent, `Compagnon sync` le laisse en place,
`compagnon archive` le déplace avec la modification, et les relecteurs ne le
reçoivent pas.

````markdown
# Corrections: add-sessions

<!-- compagnon-corrections: 1 -->

### C-1 Erreurs typées dans les bibliothèques

```yaml
step: apply
where: crates/sessions/src/store.rs
did: A renvoyé une erreur anyhow depuis le store.
asked: Utiliser un type thiserror ; anyhow reste dans la CLI.
```
````

Le marqueur de format est obligatoire : un fichier qui ne l'a pas est un
autre document du même nom, que Compagnon ne vérifie ni ne liste. Chaque
entrée est un titre `### C-<n> <titre>`, `<n>` unique à partir de 1, suivi
d'un bloc `yaml` :

| Champ | Signification |
|---|---|
| `step` | `propose`, `apply` ou `update` : la compétence pendant laquelle l'auteur a corrigé l'agent |
| `where` | Le fichier ou l'artefact concerné |
| `did` | Ce que l'agent avait fait, en une ligne |
| `asked` | Ce que l'auteur a demandé à la place, en une ligne |

Un champ que ce Compagnon ne connaît pas est un avertissement,
`corrections_unknown_field`, et il est ignoré. Un `did` ou un `asked` de
plus de 300 caractères est un avertissement, `corrections_too_long` : une
entrée énonce une consigne, jamais la conversation. Seules les modifications
actives sont vérifiées.

Une entrée ne nomme personne et ne cite ni donnée personnelle ni secret.
Elle est conservée aussi longtemps que l'archive. Pour en retirer une qui
serait passée, modifiez-la ou supprimez-la dans sa modification, archivée ou
non — Compagnon ne revérifie jamais une modification archivée —, et réécrivez
l'historique si l'enregistrement a déjà été poussé.

## Décision (ADR)

Chemin : `_compagnon/decisions/NNNN-<slug>.md`. Chaque fichier `.md` de ce
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
modèle relèvent de la convention, et non de l'obligation. Seules les
décisions `accepted` qui ne sont ni remplacées ni l'objet d'un écart sont en
vigueur, et `supersedes` comme `deviates_from` ne prennent effet que sur une
décision `accepted` : un remplacement ou un écart `proposed` ne change rien
avant `compagnon decision accept`.

Un champ du frontmatter que Compagnon ne connaît pas ne fait pas rejeter la
décision : elle est lue à partir des champs que Compagnon connaît, et
`compagnon validate` signale le champ par un avertissement
`decision_unknown_field` à sa ligne — `--strict` échoue dessus. Les champs
inconnus d'une décision héritée ne sont pas signalés. `compagnon decision accept`
conserve un tel champ, tel qu'il est écrit, lorsqu'il réécrit le frontmatter.
Compagnon 0.6.1 et les versions antérieures rejettent la décision à la place.

Les fichiers enregistrés avec des fins de ligne Windows (CRLF) sont lus comme
les autres. Lorsque Compagnon réécrit le statut d'une décision — `compagnon decision
accept`, ou un prédécesseur passé en `superseded` —, il conserve les fins de
ligne du fichier et laisse le corps identique à l'octet près : le sceau reste
valide.

## seal.yaml

Chemin : `_compagnon/decisions/seal.yaml`. Maintenu par Compagnon ; ne le modifiez
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
son scellement. Enregistrez ce fichier avec vos décisions.

`version` est le format du fichier : un Compagnon qui lui ajoute un champ ou une
valeur relève la version. Chaque commande qui lit le fichier —
`compagnon validate`, `compagnon decision new`, `compagnon decision accept`,
`compagnon decision seal` — refuse une version qu'elle ne lit pas avec
`seal_version_unsupported`, sans rien écrire ; le message nomme les deux
versions et renvoie à `compagnon self-update`. Un champ inconnu dans un fichier
d'une version connue est une corruption, signalée par `seal_invalid`.

## compagnon.lock

Chemin : `_compagnon/compagnon.lock`. Écrit uniquement par `compagnon sources update`, au
format TOML.

```toml
version = 1

[[source]]
git = "git@github.com:acme/compagnon-standards.git"
ref = "main"
commit = "12958274cbed0eb66985f57d3272c6d3134b672d"
resolved_at = "2026-09-30"
```

Une entrée `[[source]]` par source `git:`, avec la `ref` qu'elle suit,
l'enregistrement sur lequel elle est épinglée et, le cas échéant, son `subpath`.
Enregistrez ce fichier.

## SKILL.md

Chemin : `.claude/skills/compagnon-<workflow>/SKILL.md`. Généré par `compagnon init`
et `compagnon update` :

```markdown
---
name: compagnon-explore
description: "Explore an idea, investigate a problem or clarify a need before creating a compagnon change. …"
allowed-tools: "Bash(compagnon:*), Read, Glob, Grep"
license: MIT
metadata:
  generator: compagnon
  version: "0.6.1"
  checksum: "8d676b09…"
---

<workflow instructions>
```

`metadata.checksum` est l'empreinte SHA-256 de toute la compétence, hors de cette
ligne. Avec `metadata.version`, c'est ce qui permet à `compagnon update` de
distinguer les rendus de Compagnon de vos modifications. `compagnon update` réécrit
une compétence marquée d'une autre version ; parmi les compétences marquées de sa propre
version, il réécrit celle dont le contenu correspond encore à son `checksum` —
un rendu de Compagnon, d'une compilation antérieure ou pour l'autre [mode de
relecture](configuration.md#review) — et préserve celle qui n'y correspond plus,
modifiée à la main, sauf si vous passez `--force`. Une compétence sans checksum,
rendue avant leur existence, est préservée quand elle porte la marque de cette
version et diffère du rendu par plus que le mode de relecture : lancez une fois
`compagnon update --force` pour la doter d'un checksum — il écrase aussi une compétence
que vous avez réellement modifiée : vérifiez d'abord `git diff`, puis
réappliquez vos modifications ; une modification faite ensuite est préservée,
puisqu'elle ne correspond plus au checksum.

## Codes de validation

Chaque constat signalé par `compagnon validate` porte un code stable. Les messages
peuvent être reformulés d'une version à l'autre ; les codes, non.

| Code | Gravité | Signification |
|---|---|---|
| `spec_purpose_missing` | erreur | Une spécification principale n'a pas de `## Purpose` |
| `spec_no_requirement` | erreur | Une spécification principale ne contient aucune exigence |
| `requirement_outside_section` | erreur | Un `### Requirement:` en dehors de `## Requirements` |
| `delta_header_in_main_spec` | erreur | Un titre de delta dans une spécification principale |
| `requirement_no_shall` | erreur | Une exigence sans `SHALL`, `MUST`, `DOIT` ni `DOIVENT` |
| `requirement_no_scenario` | erreur | Une exigence sans scénario |
| `scenario_wrong_heading_level` | erreur | Un scénario écrit avec trois `#` au lieu de quatre |
| `duplicate_requirement` | erreur | La même exigence deux fois dans une même section de delta |
| `delta_unexpected_heading` | erreur | Un titre `###` d'une section de delta qui n'est pas un `### Requirement:`, par exemple un mot-clé traduit |
| `cross_section_conflict` | erreur | La même exigence dans deux sections parmi `ADDED`, `MODIFIED`, `REMOVED` |
| `rename_source_missing` | erreur | La source d'un `RENAMED` ne désigne aucune exigence de la spécification principale |
| `rename_target_collision` | erreur | Une cible de `RENAMED` figure aussi dans `ADDED` |
| `modified_uses_old_name` | erreur | `MODIFIED` emploie l'ancien nom d'une exigence renommée |
| `zero_delta_without_marker` | erreur | Une modification n'a aucun delta et ne déclare pas `skip_specs: true` |
| `skip_specs_conflict` | erreur | Une modification déclare `skip_specs: true` mais comporte des deltas |
| `decision_missing_frontmatter` | erreur | Un fichier de `_compagnon/decisions/` n'a pas de frontmatter |
| `decision_missing_field` | erreur | Il manque à une décision un champ obligatoire, ou elle en comporte un inconnu |
| `decision_unknown_status` | erreur | Une décision a un statut inconnu |
| `decision_field_type_mismatch` | erreur | Un champ du frontmatter n'a pas la forme attendue |
| `decision_seal_mismatch` | erreur | Le contenu d'une décision scellée a été modifié |
| `decision_conflicting_deviations` | erreur | Deux décisions locales acceptées s'écartent de la même décision héritée |
| `release_note_invalid` | erreur | La section `## Release note` de la proposition a une catégorie inconnue, un élément sans texte, ou est vide |
| `release_note_missing` | avertissement | La proposition d'une modification active n'a pas de section `## Release note` |
| `unknown_domain` | erreur | La section `## Domains` de la proposition nomme un domaine hors des six |
| `domains_invalid` | erreur | La section `## Domains` de la proposition a une ligne qui n'est ni un élément ni `None`, ou `None` à côté d'un domaine coché |
| `domains_missing` | avertissement | La proposition d'une modification active n'a pas de section `## Domains` |
| `domain_suggested` | avertissement | Le diff de la modification suggère un domaine que la proposition ne coche pas et ne garde pas non coché avec une raison |
| `invalid_trigger` | erreur | Un groupe de `triggers:` dans `_compagnon/config.yaml` n'a pas de domaine connu, pas de déclencheur, ou un glob ou une expression qui ne compile pas (un avertissement pour une clé que Compagnon ne connaît pas) |
| `review_invalid` | erreur | La structure du compte rendu de relecture, un champ, un identifiant ou un emplacement de constat, un statut ou une transition est incorrect |
| `review_decision_incomplete` | erreur | Une entrée d'historique n'a pas son `ref`, sa `note` ou son `to`, ou un risque est accepté, ou un constat haut reporté, par quelqu'un d'autre qu'une personne |
| `review_location_missing` | erreur | Un constat cite un fichier, une ligne ou un titre qui n'existe pas |
| `review_evidence_not_found` | erreur | La preuve d'un constat ne figure nulle part dans le fichier ou la section cités |
| `review_role_missing` | erreur | Un rôle actif à une étape consignée n'a pas de section, ou en a une qui le dit désactivé |
| `review_evidence_moved` | avertissement | La preuve d'un constat est dans le fichier, à plus de 5 lignes des lignes citées |
| `review_evidence_unchecked` | avertissement | Le fichier cité n'est pas du texte UTF-8, contient des octets NUL, ou dépasse 4 Mio : la preuve n'est pas comparée |
| `review_unknown_section` | avertissement | Une étape ou un rôle que le compte rendu ne devrait pas contenir ; il est ignoré |
| `review_unknown_field` | avertissement | Un champ de constat que ce Compagnon ne connaît pas ; il est ignoré |
| `review_format_unsupported` | avertissement | Le compte rendu vient d'un Compagnon plus récent ; rien n'y est vérifié |
| `corrections_invalid` | erreur | Une entrée de `corrections.md` a un titre ou un identifiant faux, pas de bloc `yaml`, ou un champ absent, vide ou invalide |
| `corrections_unknown_field` | avertissement | Un champ de correction que ce Compagnon ne connaît pas ; il est ignoré |
| `corrections_too_long` | avertissement | Le `did` ou le `asked` d'une correction dépasse 300 caractères |
| `corrections_format_unsupported` | avertissement | `corrections.md` vient d'un Compagnon plus récent ; rien n'y est vérifié |
| `delta_section_empty` | avertissement | Une section de delta ne comporte aucune entrée : rien n'en est fusionné |
| `decision_unsealed` | avertissement | Une décision locale acceptée ou remplacée n'a pas de sceau |
| `unknown_config_key` | avertissement | `_compagnon/config.yaml` contient une clé de premier niveau ou de `mcp` que ce Compagnon ne connaît pas — voir [Clés inconnues](configuration.md#clés-inconnues) |
| `decision_orphan_seal` | avertissement | Un sceau ne correspond à aucune décision |
| `decision_supersedes_unknown` | avertissement | `supersedes` désigne une décision inconnue |
| `decision_id_collision` | avertissement | Une décision locale et une décision héritée partagent un identifiant |
| `decision_dangling_deviation` | avertissement | `deviates_from` désigne une décision non indexée |
| `decision_supersession_cycle` | avertissement | Les remplacements forment un cycle ; aucune des décisions concernées n'est en vigueur |
