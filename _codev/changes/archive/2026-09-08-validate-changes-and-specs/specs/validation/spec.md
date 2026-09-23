## Purpose

Fournir un verdict fiable et localisé sur un change ou une spec principale :
tout ce qu'`archive` et `sync` refuseraient d'écrire doit être signalé ici, en
amont, avec la ligne concernée et un code stable qu'un consommateur puisse
tester.

## ADDED Requirements

### Requirement: Règles structurelles au-delà du parseur

Le validateur MUST rejeter comme erreurs les défauts que le parseur laisse
passer parce qu'ils exigent d'inspecter le contenu d'une exigence ou d'une
spec : une exigence sans mot clé `SHALL` ou `MUST` dans sa description, une
exigence dépourvue de scénario, une spec principale sans aucune exigence
extractible.

#### Scenario: Exigence sans SHALL ni MUST

- **GIVEN** un delta `## ADDED Requirements` avec un `### Requirement: X`
  dont le texte descriptif est « The system does something »
- **WHEN** le validateur inspecte le fichier
- **THEN** un finding de code `requirement_no_shall` signale la ligne de
  l'en-tête et nomme l'exigence `X`

#### Scenario: Exigence sans aucun scénario

- **GIVEN** un delta `## ADDED Requirements` avec un `### Requirement: Y`
  suivi de son texte descriptif mais d'aucun `#### Scenario:`
- **WHEN** le validateur inspecte le fichier
- **THEN** un finding de code `requirement_no_scenario` signale la ligne
  de l'exigence

#### Scenario: Spec principale sans exigence

- **GIVEN** une spec principale `_codev/specs/x/spec.md` avec `## Purpose`
  mais dont `## Requirements` est vide
- **WHEN** le validateur inspecte la spec
- **THEN** un finding de code `spec_no_requirement` signale que la spec n'a
  aucune exigence extractible

### Requirement: Cohérence entre sections d'un même delta

Le validateur MUST détecter les incohérences entre les quatre opérations d'un
même delta : une même exigence ne peut pas figurer dans deux sections à la
fois, un `RENAMED.TO` ne peut pas coïncider avec un `ADDED` de même nom, et un
`MODIFIED` ne peut pas référencer l'ancien nom d'un `RENAMED`.

#### Scenario: Exigence présente dans ADDED et MODIFIED

- **GIVEN** un delta contenant à la fois `## ADDED Requirements` avec
  `### Requirement: Z` et `## MODIFIED Requirements` avec `### Requirement: Z`
- **WHEN** le validateur inspecte le delta
- **THEN** un finding de code `cross_section_conflict` nomme l'exigence `Z`,
  les deux sections en cause et leurs lignes respectives

#### Scenario: RENAMED.TO collide avec un ADDED de même nom

- **GIVEN** un delta contenant `## ADDED Requirements` avec
  `### Requirement: New Name` et `## RENAMED Requirements` avec
  `FROM: Old Name` / `TO: New Name`
- **WHEN** le validateur inspecte le delta
- **THEN** un finding de code `rename_target_collision` signale la collision
  sur `New Name`

#### Scenario: MODIFIED référence l'ancien nom d'un RENAMED

- **GIVEN** un delta contenant `## MODIFIED Requirements` avec
  `### Requirement: Old Name` et `## RENAMED Requirements` avec
  `FROM: Old Name` / `TO: New Name`
- **WHEN** le validateur inspecte le delta
- **THEN** un finding de code `modified_uses_old_name` demande d'utiliser
  `New Name` dans la section MODIFIED

### Requirement: Règle du zéro-delta explicite

Un change doit soit produire au moins un delta de spec, soit déclarer
explicitement qu'il n'en produira aucun ; le validateur MUST rejeter les cas
qui contredisent cette règle.

#### Scenario: Change sans aucun delta et sans skip_specs

- **GIVEN** un change dont le dossier `specs/` est vide et dont le
  `change.yaml` ne pose pas `skip_specs: true`
- **WHEN** le validateur inspecte le change
- **THEN** un finding de code `zero_delta_without_marker` demande soit
  d'ajouter un delta, soit de déclarer `skip_specs: true`

#### Scenario: skip_specs déclaré mais des specs existent

- **GIVEN** un change dont le `change.yaml` déclare `skip_specs: true` et
  dont le dossier `specs/` contient au moins un fichier `.md`
- **WHEN** le validateur inspecte le change
- **THEN** un finding de code `skip_specs_conflict` demande de retirer
  `skip_specs: true` ou de supprimer les fichiers du dossier `specs/`

### Requirement: Verdict d'ensemble et code de sortie

À la demande, le validateur MUST produire un verdict d'ensemble sur un item
seul, sur tous les changes, sur toutes les specs, ou sur les deux ; il MUST
distinguer une exécution sans aucune erreur d'une exécution qui en signale.

#### Scenario: Validation d'un item nommé

- **GIVEN** un projet initialisé avec un change `add-auth`
- **WHEN** l'utilisateur lance `codev validate add-auth`
- **THEN** le rapport ne concerne que ce change et son code de sortie reflète
  la présence ou l'absence d'erreur

#### Scenario: Validation en lot

- **GIVEN** un projet contenant deux changes et une spec principale
- **WHEN** l'utilisateur lance `codev validate --all`
- **THEN** le rapport couvre les trois éléments dans un même document

#### Scenario: Aucune erreur, code de sortie zéro

- **GIVEN** un change bien formé, sans finding d'erreur
- **WHEN** l'utilisateur lance `codev validate add-auth`
- **THEN** le processus se termine avec le code `0`

#### Scenario: Au moins une erreur, code de sortie non nul

- **GIVEN** un change contenant au moins un finding de sévérité `Error`
- **WHEN** l'utilisateur lance `codev validate add-auth`
- **THEN** le processus se termine avec le code `1`

### Requirement: Rapport JSON à contrat stable

Sur demande `--json`, le validateur MUST écrire sur stdout exactement un
document JSON dont la forme est figée par version : liste des items validés,
et pour chacun sa liste de findings avec `code`, `severity`, `path`, `line`,
`message`, plus un tableau `status` à la racine où atterrissent les erreurs
d'exécution (racine introuvable, item inconnu).

#### Scenario: Sortie JSON d'un run réussi

- **GIVEN** un change bien formé
- **WHEN** l'utilisateur lance `codev validate add-auth --json`
- **THEN** stdout porte un seul document JSON contenant la liste des items,
  chacun avec un tableau `findings` vide, et un `status` racine vide

#### Scenario: Sortie JSON quand la racine est introuvable

- **GIVEN** un dossier hors de toute racine `_codev/`
- **WHEN** l'utilisateur lance `codev validate --json`
- **THEN** stdout porte un seul document JSON de la forme du rapport, avec
  ses listes d'items vides, et un `status` racine portant une entrée d'erreur
  au code stable
