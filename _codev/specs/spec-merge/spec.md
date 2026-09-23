# Spec Merge Specification

## Purpose

Faire entrer un delta dans les specs principales sans dommage collatéral : ce
que le delta décrit change ; tout le reste — commentaires, ordre, espacement,
sections libres — reste au caractère près ce qu'il était. Boucler le cycle
d'un change par un déplacement chronologique vers l'archive.

## Requirements

### Requirement: Fusion sémantique par opération

Un sync SHALL appliquer chaque opération d'un delta selon sa sémantique
propre : `ADDED` insère à la fin de la section `## Requirements`, `MODIFIED`
remplace le bloc de l'exigence homonyme au caractère près, `REMOVED` supprime
le bloc entier, `RENAMED` retitre uniquement l'en-tête `### Requirement:` sans
toucher au corps.

#### Scenario: ADDED apparaît à la fin de Requirements

- **GIVEN** une spec principale `user-auth` contenant `## Requirements` avec
  une exigence `Login`
- **AND** un delta `ADDED` portant une exigence `Two-Factor Authentication`
  avec son scénario
- **WHEN** l'utilisateur lance `codev sync <change>`
- **THEN** la spec principale contient désormais `Login` en premier puis
  `Two-Factor Authentication`
- **AND** le bloc de `Login` — son en-tête, sa description, ses scénarios — est
  identique au caractère près à ce qu'il était

#### Scenario: MODIFIED remplace le bloc de l'exigence sans toucher aux autres

- **GIVEN** une spec principale contenant deux exigences `Login` et
  `Session Expiration`, dans cet ordre
- **AND** un delta `MODIFIED` portant `Session Expiration` avec un nouveau
  scénario
- **WHEN** l'utilisateur lance `codev sync <change>`
- **THEN** le bloc de `Session Expiration` a été remplacé par la version du
  delta
- **AND** le bloc de `Login` et l'espacement qui sépare les deux exigences
  restent identiques au caractère près

#### Scenario: REMOVED supprime le bloc entier de l'exigence

- **GIVEN** une spec principale contenant `Login` et `Remember Me`
- **AND** un delta `REMOVED` portant `Remember Me`
- **WHEN** l'utilisateur lance `codev sync <change>`
- **THEN** la spec principale ne contient plus aucune trace de `Remember Me`
- **AND** le bloc de `Login` reste identique au caractère près

#### Scenario: RENAMED retitre l'en-tête et rien d'autre

- **GIVEN** une spec principale contenant `Session Expiration` avec ses
  scénarios
- **AND** un delta `RENAMED` associant `Session Expiration` à `Session Timeout`
- **WHEN** l'utilisateur lance `codev sync <change>`
- **THEN** l'en-tête de l'exigence est devenu `### Requirement: Session Timeout`
- **AND** la description et les scénarios de cette exigence sont identiques
  au caractère près

### Requirement: Préservation du contenu non mentionné

Un sync MUST laisser strictement inchangé tout ce que le delta ne mentionne
pas : les autres exigences, les commentaires HTML, les blocs de code fencés,
les sections libres après `## Requirements`, et jusqu'aux espacements entre
les exigences.

#### Scenario: Une section libre après Requirements survit à un sync

- **GIVEN** une spec principale qui contient, après `## Requirements`, une
  section `## Notes` avec un paragraphe
- **AND** un delta qui ne mentionne pas cette section
- **WHEN** l'utilisateur lance `codev sync <change>`
- **THEN** la section `## Notes` est présente au caractère près dans le fichier
  après sync

#### Scenario: Un commentaire HTML dans une exigence non touchée survit

- **GIVEN** une exigence `Login` contenant un commentaire HTML dans sa
  description
- **AND** un delta qui ne touche pas à `Login`
- **WHEN** l'utilisateur lance `codev sync <change>`
- **THEN** le commentaire HTML de `Login` reste identique au caractère près

### Requirement: Création d'une spec principale pour une nouvelle capacité

Lorsque le delta cible une capacité qui n'a pas encore de spec principale
sous `_codev/specs/`, un sync SHALL créer le fichier `_codev/specs/<chemin>/spec.md`
à partir du `## Purpose` du delta et de ses exigences `ADDED`.

#### Scenario: Nouvelle capacité créée depuis Purpose et ADDED

- **GIVEN** aucune spec principale sous `_codev/specs/user-auth/`
- **AND** un delta portant `## Purpose` et une exigence `ADDED: Login`
- **WHEN** l'utilisateur lance `codev sync <change>`
- **THEN** le fichier `_codev/specs/user-auth/spec.md` existe désormais
- **AND** il commence par la section `## Purpose` du delta
- **AND** il contient sous `## Requirements` l'exigence `Login` du delta

#### Scenario: Nouvelle capacité sans Purpose est refusée

- **GIVEN** aucune spec principale sous `_codev/specs/x/`
- **AND** un delta portant uniquement des `ADDED` sans `## Purpose`
- **WHEN** l'utilisateur lance `codev sync <change>`
- **THEN** aucune écriture n'a lieu
- **AND** un message d'erreur nomme la capacité, indique que `## Purpose` est
  requis pour une nouvelle capacité, et rappelle le code stable
  `new_capability_without_purpose`

### Requirement: Atomicité du plan de fusion

Un sync ou un archive MUST valider son plan complet — chaque main spec à
réécrire, chaque main spec à créer, chaque déplacement — avant d'effectuer la
moindre écriture. Une seule opération irrésolue empêche toutes les autres.

#### Scenario: MODIFIED sur une exigence absente refuse tout le sync

- **GIVEN** un delta contenant deux `MODIFIED`, l'un sur `Login` qui existe,
  l'autre sur `Fantome` qui n'existe pas dans la spec principale
- **WHEN** l'utilisateur lance `codev sync <change>`
- **THEN** aucune écriture n'a lieu sur la spec principale
- **AND** un message d'erreur nomme `Fantome`, indique que la spec principale
  ne le contient pas, et rappelle le code stable `modified_target_missing`

#### Scenario: REMOVED sur la dernière exigence refuse l'opération

- **GIVEN** une spec principale ne contenant qu'une seule exigence
- **AND** un delta contenant un `REMOVED` sur cette exigence
- **WHEN** l'utilisateur lance `codev sync <change>`
- **THEN** aucune écriture n'a lieu
- **AND** un message d'erreur pointe le code stable
  `would_leave_spec_without_requirement` et explique que `retire_capabilities`
  n'est pas encore pris en charge

### Requirement: Sync laisse le change actif ; archive le déplace

Un sync SHALL laisser le dossier du change à sa place ; un archive MUST le
déplacer vers `_codev/changes/archive/<date>-<nom>/`, où `<date>` est la date
locale au format `AAAA-MM-JJ` et `<nom>` l'identifiant du change.

#### Scenario: Sync ne déplace pas le change

- **GIVEN** un change `add-auth` dont la fusion réussit
- **WHEN** l'utilisateur lance `codev sync add-auth`
- **THEN** le dossier `_codev/changes/add-auth/` existe toujours à cet endroit
- **AND** son contenu est identique à ce qu'il était avant le sync

#### Scenario: Archive déplace le change vers l'archive datée

- **GIVEN** un change `add-auth` dont la fusion réussit à la date `2026-09-08`
- **WHEN** l'utilisateur lance `codev archive add-auth`
- **THEN** `_codev/changes/add-auth/` n'existe plus
- **AND** `_codev/changes/archive/2026-09-08-add-auth/` existe, avec tous les
  fichiers du change préservés

### Requirement: Pré-flight de validation avant archive

`codev archive` MUST refuser d'agir si `codev validate <change>` remonte au
moins une erreur ; il MUST le faire sans procéder à la fusion, sans écrire, et
sans déplacer.

#### Scenario: Un change avec erreur de validation ne s'archive pas

- **GIVEN** un change dont un delta présente un `duplicate_requirement`
- **WHEN** l'utilisateur lance `codev archive <change>`
- **THEN** aucune spec principale n'est modifiée
- **AND** le change reste actif à son emplacement d'origine
- **AND** le message d'erreur nomme le code `validation_failed` et invite à
  lancer `codev validate <change>` pour voir le détail

#### Scenario: Un sync n'exige pas la validation complète

- **GIVEN** un change contenant un delta bien formé mais aussi une exigence
  sans `SHALL` (finding d'erreur signalé par validate)
- **WHEN** l'utilisateur lance `codev sync <change>`
- **THEN** la fusion a lieu ; le pré-flight de sync se limite aux invariants
  strictement nécessaires à la fusion (existence des cibles `MODIFIED`,
  présence d'un Purpose pour une nouvelle capacité)

### Requirement: Rapport avec contrat stable

Sur demande `--json`, sync et archive MUST écrire sur stdout exactement un
document JSON dont la forme est figée par version, listant les fichiers
écrits, créés, ou déplacés, avec un tableau `status` racine pour les erreurs
d'exécution.

#### Scenario: Rapport JSON d'un sync réussi

- **GIVEN** un change dont la fusion touche une seule spec principale
  existante
- **WHEN** l'utilisateur lance `codev sync <change> --json`
- **THEN** stdout porte un seul document JSON contenant le champ `updated`
  avec le chemin de la spec principale modifiée, le champ `created` vide, et
  un `status` racine vide

#### Scenario: Rapport JSON d'un archive réussi

- **GIVEN** un change dont l'archive réussit
- **WHEN** l'utilisateur lance `codev archive <change> --json`
- **THEN** le document JSON contient en plus le champ `movedTo` avec le
  chemin sous `changes/archive/<date>-<nom>/`

### Requirement: Suppression atomique d'une spec vidée quand `retire_capabilities`

Quand un change porte `retire_capabilities: true` dans son
`change.yaml`, et qu'un delta `## REMOVED Requirements` retire **toutes**
les exigences d'une spec principale, `codev sync` MUST écrire un plan
qui **supprime** le fichier `_codev/specs/<capa>/spec.md` en une seule
opération atomique, plutôt que de le laisser vide ou de le refuser.

Sans le marqueur, le comportement reste celui d'aujourd'hui : refus avec
le code stable `would_leave_spec_without_requirement`.

#### Scenario: Retirer une capacité avec le marqueur → fichier supprimé

- **GIVEN** un projet contenant une spec principale
  `_codev/specs/user-auth/spec.md` avec une seule exigence `Login`
- **AND** un change dont `change.yaml` porte `retire_capabilities: true`
- **AND** un delta `_codev/changes/<c>/specs/user-auth/spec.md` qui
  `## REMOVED Requirements` l'exigence `Login`
- **WHEN** l'utilisateur lance `codev sync <c>`
- **THEN** le fichier `_codev/specs/user-auth/spec.md` n'existe plus
  après l'exécution
- **AND** le rapport de sync liste le fichier dans `deleted[]`

#### Scenario: Retirer une capacité sans le marqueur → refus

- **GIVEN** le même contexte, mais **sans** `retire_capabilities: true`
- **WHEN** l'utilisateur lance `codev sync <c>`
- **THEN** aucune écriture ni suppression n'a lieu sur `_codev/specs/`
- **AND** le message d'erreur nomme le code stable
  `would_leave_spec_without_requirement`

#### Scenario: `retire_capabilities` sans effet quand une exigence reste

- **GIVEN** une spec principale avec les exigences `Login` et `Logout`
- **AND** un change `retire_capabilities: true` dont le delta retire
  uniquement `Login`
- **WHEN** l'utilisateur lance `codev sync <c>`
- **THEN** le fichier `_codev/specs/<capa>/spec.md` existe toujours,
  contient encore `Logout`, et n'apparaît pas dans `deleted[]`
- **AND** le marqueur n'a rien déclenché de plus qu'un merge normal

### Requirement: `deletions` est une opération de premier ordre du plan

Le type `Plan` du cœur MUST porter un champ `deletions: Vec<PathBuf>`
distinct de `writes` et de `moves`, et la coquille MUST appliquer les
deletions dans un ordre déterministe : **après** les writes et **avant**
les moves. Un `Plan` sans deletion garde le comportement historique bit-
identique.

#### Scenario: Le plan expose deletions séparément

- **GIVEN** un `plan_sync` sur un change qui retire une capacité
- **WHEN** l'inspecteur regarde le `Plan` produit
- **THEN** l'entrée du fichier à supprimer figure dans `plan.deletions`
- **AND** ne figure pas dans `plan.writes` (aucun write d'une chaîne vide)

#### Scenario: Ordre d'exécution — deletions après writes

- **GIVEN** un plan qui à la fois modifie une spec `A` (write) et
  supprime une spec `B` (deletion)
- **WHEN** la coquille exécute le plan
- **THEN** l'écriture sur `A` est appliquée avant la suppression de `B`
- **AND** la suppression de `B` est appliquée avant tout `move`

### Requirement: Contrat JSON `sync` et `archive` expose `deleted`

Le rapport JSON de `codev sync --json` (contrat `SyncReportV1`) et de
`codev archive --json` (contrat `ArchiveReportV1`) MUST porter un champ
additif `deleted: Vec<String>`, toujours présent, vide dans le cas
courant. Le champ contient les chemins absolus des specs principales
supprimées par le change, dans un ordre déterministe.

#### Scenario: `sync --json` avec une capacité retirée

- **GIVEN** un projet avec une spec `user-auth`, un change
  `retire_capabilities: true` qui retire l'unique exigence
- **WHEN** l'utilisateur lance `codev sync <c> --json`
- **THEN** le document JSON porte `"deleted": ["<abs>/…/user-auth/spec.md"]`
- **AND** `updated`, `created`, `unchanged` ne mentionnent PAS ce chemin

#### Scenario: `deleted` toujours présent, vide par défaut

- **GIVEN** un change sans suppression
- **WHEN** l'utilisateur lance `codev sync <c> --json`
- **THEN** le document JSON porte `"deleted": []`
