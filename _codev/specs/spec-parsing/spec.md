# Spec Parsing Specification

## Purpose

Fournir à codev une lecture structurée et fiable des specs principales et des
deltas écrits en markdown, sur laquelle validation, sync et archive puissent
s'appuyer sans risquer une réécriture destructive.

## Requirements

### Requirement: Structure d'une spec principale extraite

Le parseur SHALL extraire d'un fichier markdown de spec principale sa section
`## Purpose`, sa section `## Requirements`, et pour chaque exigence son nom, son
texte descriptif et ses scénarios.

#### Scenario: Purpose et exigences bien formées

- **GIVEN** un fichier contenant `## Purpose`, une phrase, puis `## Requirements`,
  puis un `### Requirement: Session Expiration` suivi d'un `#### Scenario: Idle`
  avec des lignes **WHEN** / **THEN**
- **WHEN** le parseur lit le fichier
- **THEN** le résultat expose le texte du Purpose
- **AND** expose une exigence nommée `Session Expiration` porteuse de son
  scénario nommé `Idle`

#### Scenario: Purpose manquant sur une spec principale

- **GIVEN** un fichier de spec principale sans section `## Purpose`
- **WHEN** le parseur lit le fichier
- **THEN** le résultat signale l'absence de Purpose comme un défaut structurel
  nommant la nature du manque
- **AND** le reste des exigences reste extractible

### Requirement: Opérations d'un delta reconnues

Le parseur MUST reconnaître les quatre opérations d'un delta et associer
chacune à sa charge utile : le bloc d'exigence complet pour `ADDED` et
`MODIFIED`, le nom accompagné de `**Raison**` et `**Migration**` pour
`REMOVED`, et le couple `FROM:` / `TO:` pour `RENAMED`.

#### Scenario: Bloc ADDED avec exigence et scénario

- **GIVEN** un delta contenant `## ADDED Requirements` puis
  `### Requirement: Two-Factor Authentication` avec un `#### Scenario: Enrolment`
- **WHEN** le parseur lit le delta
- **THEN** l'opération `ADDED` porte l'exigence `Two-Factor Authentication`
- **AND** cette exigence porte son scénario `Enrolment`

#### Scenario: Bloc REMOVED avec raison et migration

- **GIVEN** un delta contenant `## REMOVED Requirements` puis
  `### Requirement: Remember Me` suivi de `**Reason**: <texte>` et
  `**Migration**: <texte>`
- **WHEN** le parseur lit le delta
- **THEN** l'opération `REMOVED` porte le nom `Remember Me`, sa raison et sa
  migration

#### Scenario: Bloc RENAMED avec FROM et TO

- **GIVEN** un delta contenant `## RENAMED Requirements` puis les lignes
  `FROM: Old Name` et `TO: New Name`
- **WHEN** le parseur lit le delta
- **THEN** l'opération `RENAMED` associe l'ancien nom `Old Name` au nouveau
  nom `New Name`

#### Scenario: Delta d'une capacité nouvelle avec Purpose

- **GIVEN** un delta qui débute par `## Purpose` suivi d'une phrase, puis
  contient un bloc `## ADDED Requirements`
- **WHEN** le parseur lit le delta
- **THEN** le résultat porte le texte du Purpose, à recopier tel quel lors de
  la création d'une nouvelle spec principale

### Requirement: Zones littérales ignorées

Le parseur MUST ignorer toute structure — titres, exigences, scénarios,
en-têtes de delta — qui apparaît à l'intérieur d'un bloc de code délimité par
` ``` ` ou `~~~`, ou à l'intérieur d'un commentaire HTML `<!-- … -->`.

#### Scenario: Exemple d'exigence à l'intérieur d'un bloc de code

- **GIVEN** un fichier dont la section Purpose contient un bloc ` ``` ` où figure
  la ligne `### Requirement: Exemple`
- **WHEN** le parseur lit le fichier
- **THEN** aucune exigence nommée `Exemple` n'apparaît dans le résultat

#### Scenario: En-tête de delta à l'intérieur d'un commentaire

- **GIVEN** un delta contenant un commentaire `<!-- ## ADDED Requirements … -->`
  suivi, plus bas, d'un vrai `## ADDED Requirements` avec une exigence
- **WHEN** le parseur lit le delta
- **THEN** l'opération `ADDED` n'est comptée qu'une fois, avec l'exigence
  du vrai bloc

### Requirement: Position d'origine préservée

Chaque élément extrait — Purpose, exigence, scénario, bloc de delta — SHALL
porter l'intervalle exact `[début, fin)` qu'il occupe dans le texte source, en
octets et en lignes.

#### Scenario: Réécriture d'un bloc sans toucher au reste

- **GIVEN** un fichier de spec principale contenant deux exigences successives
- **WHEN** un consommateur remplace le texte source occupé par la première
  exigence par un nouveau bloc de longueur différente
- **THEN** la seconde exigence, ses scénarios et l'espacement qui les entoure
  restent identiques au caractère près

#### Scenario: Position en ligne d'un scénario

- **GIVEN** un fichier où le scénario `Idle` d'une exigence commence à la
  ligne 42
- **WHEN** le parseur lit le fichier
- **THEN** l'élément représentant ce scénario expose la ligne 42 comme début

### Requirement: Défauts structurels localisés

Face à un fichier mal formé, le parseur MUST produire un rapport nommant la
ligne concernée, le type de défaut, et un message lisible ; il ne SHALL PAS
échouer en bloc sur un fichier partiellement récupérable.

#### Scenario: Scénario écrit avec trois dièses

- **GIVEN** une exigence dont le scénario est écrit `### Scenario:` au lieu de
  `#### Scenario:`
- **WHEN** le parseur lit le fichier
- **THEN** un défaut structurel signale la ligne, indique que le scénario doit
  porter quatre dièses
- **AND** l'exigence continue d'apparaître dans le résultat, sans ce scénario

#### Scenario: Exigence dupliquée dans une même section

- **GIVEN** un delta `## ADDED Requirements` contenant deux exigences portant
  exactement le même nom
- **WHEN** le parseur lit le delta
- **THEN** un défaut signale les deux lignes des exigences en double
- **AND** nomme la section `ADDED` comme lieu du conflit

#### Scenario: En-tête de delta dans une spec principale

- **GIVEN** un fichier de spec principale qui contient par erreur un
  `## ADDED Requirements`
- **WHEN** le parseur lit le fichier
- **THEN** un défaut signale la ligne, précise que les en-têtes de delta
  n'appartiennent qu'aux fichiers de change
