# Proposal : valider changes et specs sans faux positif

## Pourquoi

Le parseur de `codev-core` produit déjà des `Finding` structurels au niveau
d'un fichier isolé — Purpose manquant, exigence hors section, scénario en trois
dièses, doublon dans une section. Il manque à l'outil de vérifier un change ou
une spec dans sa totalité, à la demande, avec un rapport lisible par un humain
comme par un agent. Sans cela, `sync` et `archive` ne peuvent pas savoir avant
d'écrire s'ils ont affaire à un delta cohérent.

## Ce qui change

- **Nouvelle commande** `codev validate [item]`, avec `--all` / `--changes` /
  `--specs` pour les runs en lot, et `--json` pour un rapport machine.
- **Règles structurelles supplémentaires**, pures, jouées sur l'AST du
  parseur : exigence sans `SHALL`/`MUST`, exigence sans scénario, spec
  principale sans exigence.
- **Cohérence de delta** : exigence présente dans deux sections
  (`ADDED`/`MODIFIED`, `MODIFIED`/`REMOVED`, `ADDED`/`REMOVED`), collision
  `RENAMED.TO` avec un `ADDED` de même nom, `MODIFIED` référençant un ancien
  nom de `RENAMED`.
- **Règle « zéro delta »** : un change dont le dossier `specs/` ne contient
  aucun delta échoue, sauf si son `change.yaml` déclare `skip_specs: true` ;
  inversement, `skip_specs: true` avec des deltas présents est un conflit.
- **Contrat de sortie stable** : chaque `Finding` porte un `code`
  (`requirement_no_shall`, `requirement_no_scenario`, `cross_section_conflict`,
  `zero_delta_without_marker`, `skip_specs_conflict`, …), un `path` relatif au
  projet, une `line`, un `severity`. Codes stables — un consommateur peut s'y
  fier ; messages libres de reformulation.
- **Exit code** : `0` si aucune erreur, `1` sinon. Les avertissements du lot 2
  ne changeront pas ce code.

## Capacités

### Nouvelles capacités

- `validation`

### Capacités modifiées

- `spec-parsing` — non modifiée. Les règles supplémentaires vivent au-dessus
  du parseur ; le contrat des `Finding` déjà émis ne change pas.

## Impact

- **Code** : nouveau module `codev-core::validate` pour les règles pures
  (entrée : AST + métadonnées du change ; sortie : `Vec<Finding>`), nouveau
  module `codev-engine::validate` pour la coordination
  (lecture du disque, groupement par fichier, exit code), nouvelle sous-commande
  dans `codev-cli`, forme figée `contract::v1::ValidateReport`.
- **Dépendances** : aucune nouvelle — tout repose sur ce qui existe.
- **Hors périmètre** : `--strict` (E5, promotion des warnings en erreurs),
  préflight de fusion `MODIFIED` contre la spec principale (E6),
  `--archived` (E7), validation parallèle bornée (E8). Ces flags arriveront
  quand leurs consommateurs (archive, hook pre-commit) seront implémentés.
