## ADDED Requirements

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
