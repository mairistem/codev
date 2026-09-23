## Purpose

Rendre les décisions d'architecture exploitables par codev : les lire, les
indexer, résoudre la chaîne des supersessions, et les injecter dans les
instructions de l'artefact `design` pour qu'un agent qui rédige voie
d'emblée les choix déjà tranchés.

## ADDED Requirements

### Requirement: Format ADR reconnu

Le parseur SHALL reconnaître un ADR écrit en frontmatter YAML suivi de
sections markdown libres. Le frontmatter porte au moins les champs `id`,
`title`, `status`, `date`, et éventuellement `tags` (liste), `supersedes`
(identifiant ou tableau d'identifiants). Un fichier sans frontmatter ou dont
le frontmatter manque un champ obligatoire est signalé, pas parsé
silencieusement.

#### Scenario: ADR bien formé

- **GIVEN** un fichier commençant par `---`, contenant un frontmatter YAML
  avec `id: 0007`, `title: "…"`, `status: accepted`, `date: 2026-09-08`,
  `tags: [architecture]`, puis un séparateur `---` puis du markdown libre
- **WHEN** le parseur lit le fichier
- **THEN** le résultat expose l'identifiant `0007`, le titre, le statut
  `accepted`, la date et le tag `architecture`

#### Scenario: ADR sans frontmatter

- **GIVEN** un fichier markdown sans en-tête `---`
- **WHEN** le parseur lit le fichier
- **THEN** un finding de code stable `decision_missing_frontmatter` signale
  que le fichier n'a pas le format attendu

#### Scenario: Champ obligatoire manquant

- **GIVEN** un fichier dont le frontmatter n'a pas de `title`
- **WHEN** le parseur lit le fichier
- **THEN** un finding de code `decision_missing_field` nomme le champ
  manquant

### Requirement: Statuts reconnus et effet

Le validateur MUST reconnaître les statuts `accepted`, `superseded`,
`proposed`, `deprecated`, `rejected` ; seuls `accepted` et `superseded`
sont pris en compte pour le calcul de l'effet — les trois autres sont
exposés tels quels dans l'index et jamais considérés comme « en vigueur ».

#### Scenario: Statut inconnu signalé

- **GIVEN** un ADR dont le `status` vaut `pending`
- **WHEN** le parseur lit le fichier
- **THEN** un finding de code `decision_unknown_status` signale la valeur
  et rappelle la liste des statuts reconnus

#### Scenario: Statut proposed n'entre pas en vigueur

- **GIVEN** un ADR de statut `proposed`, sans lien de supersession
- **WHEN** l'index est calculé
- **THEN** cette décision n'apparaît pas dans les « décisions en vigueur »

### Requirement: Supersession résolue en chaîne

L'index MUST résoudre le champ `supersedes` : chaque décision qu'un ADR
supersede est marquée `superseded_by(<id>)` dans l'index, et n'est pas en
vigueur. Une chaîne `A ← B ← C` laisse `A` et `B` supersedées, seule `C`
reste en vigueur.

#### Scenario: Supersession directe

- **GIVEN** un ADR `0003 accepted` et un ADR `0007 accepted supersedes: [0003]`
- **WHEN** l'index est calculé
- **THEN** `0003` est marqué `superseded_by(0007)` et n'apparaît pas dans
  les décisions en vigueur
- **AND** `0007` apparaît dans les décisions en vigueur

#### Scenario: Chaîne à trois maillons

- **GIVEN** trois ADR `accepted` où `B` supersede `A` et `C` supersede `B`
- **WHEN** l'index est calculé
- **THEN** seul `C` est en vigueur

#### Scenario: Cible de supersession absente

- **GIVEN** un ADR `0007 supersedes: [9999]` alors que `9999` n'existe pas
- **WHEN** l'index est calculé
- **THEN** un finding de code `decision_supersedes_unknown` signale
  l'identifiant fantôme
- **AND** `0007` reste en vigueur (le lien perdu ne le disqualifie pas)

### Requirement: Décisions héritées prises en compte

Quand un projet déclare `inherits: path: <chemin>` dans son
`_codev/config.yaml`, le validateur MUST parser aussi les ADR de
`<chemin>/_codev/decisions/` et les fusionner dans l'index avec leur
`origin` visible.

#### Scenario: ADR d'une source héritée apparaît dans l'index

- **GIVEN** un projet qui hérite d'une source `path: ~/partage` contenant
  un ADR `0100 accepted`
- **WHEN** l'index est calculé
- **THEN** l'entrée porte l'`origin` `path:~/partage`
- **AND** son identifiant qualifié pour éviter les collisions est
  `path:~/partage/0100`

#### Scenario: Collision d'id entre projet et source

- **GIVEN** un ADR `0007 accepted` local **et** un ADR `0007 accepted` dans
  une source héritée
- **WHEN** l'index est calculé
- **THEN** un finding de code `decision_id_collision` signale le doublon
- **AND** la version du projet gagne (elle est plus proche de l'auteur)

### Requirement: Injection dans les instructions de design

L'appel `codev instructions design --change <nom>` MUST enrichir sa réponse
d'un champ `decisions[]` porteur des décisions en vigueur. Chaque entrée
expose `id`, `title`, `status`, `tags`, `path` (relatif au projet) et
`origin`. Le contenu complet reste dans le fichier — pas de duplication
dans la réponse.

#### Scenario: Instructions design portent les décisions en vigueur

- **GIVEN** un projet avec 6 ADR `accepted` et aucun supersession
- **WHEN** l'utilisateur lance `codev instructions design --change <nom>
  --json`
- **THEN** la réponse JSON contient un tableau `decisions` avec exactement
  6 entrées, chacune portant `id`, `title`, `status`, `path` relatif au
  projet et `origin: "projet"`

#### Scenario: Décisions supersedées absentes des instructions

- **GIVEN** un projet où `0003` est supersedée par `0007`
- **WHEN** l'utilisateur lance `codev instructions design --change <nom>
  --json`
- **THEN** `0003` n'apparaît pas dans le tableau `decisions`
- **AND** `0007` y apparaît

#### Scenario: Rendu humain liste les décisions

- **GIVEN** le même contexte
- **WHEN** l'utilisateur lance `codev instructions design --change <nom>`
  (sans `--json`)
- **THEN** le rendu contient une section « Décisions en vigueur » listant
  chaque décision sur sa propre ligne avec son `id` et son `title`

#### Scenario: Aucune décision, aucune section

- **GIVEN** un projet neuf sans aucun ADR
- **WHEN** l'utilisateur lance `codev instructions design --change <nom>`
- **THEN** aucune section « Décisions en vigueur » n'est ajoutée au rendu
- **AND** le champ `decisions[]` dans la réponse JSON est présent et vide
