## ADDED Requirements

### Requirement: Mode strict propage les warnings à l'exit code

`codev validate [--all|--changes|--specs|<item>] [--strict]` MUST
retourner un exit code non-nul dès qu'au moins un finding est émis,
**quelle que soit sa sévérité**, quand `--strict` est présent. Sans
`--strict`, le comportement reste inchangé : exit code 1 seulement en
présence d'au moins un `Error`.

Le mode strict **ne modifie pas** la sévérité des findings dans le
rapport ; il ne change **que** la règle de décision de l'exit code. Le
rendu humain (texte et JSON) reste identique.

#### Scenario: `--strict` sur un rapport propre → exit 0

- **GIVEN** un projet dont `codev validate --all` ne remonte aucun
  finding
- **WHEN** l'utilisateur lance `codev validate --all --strict`
- **THEN** l'exit code de la commande est 0

#### Scenario: `--strict` sur un warning → exit non-nul

- **GIVEN** un projet contenant un ADR local `accepted` non scellé,
  qui remonte un `decision_unsealed` (warning) au validate
- **WHEN** l'utilisateur lance `codev validate --strict`
- **THEN** l'exit code de la commande est non-nul (1)
- **AND** le rendu humain porte toujours la ligne `warning` (pas
  `erreur`) pour ce finding — la sévérité affichée est préservée

#### Scenario: Sans `--strict`, un warning ne fait pas basculer l'exit

- **GIVEN** le même contexte
- **WHEN** l'utilisateur lance `codev validate` (sans `--strict`)
- **THEN** l'exit code de la commande est 0
- **AND** le warning apparaît toujours dans la sortie

#### Scenario: Une erreur reste bloquante, même sans `--strict`

- **GIVEN** un projet dont un ADR local a été édité en place après
  scellement, qui remonte un `decision_seal_mismatch` (erreur)
- **WHEN** l'utilisateur lance `codev validate`
- **THEN** l'exit code de la commande est non-nul (1) — le mode strict
  n'est pas nécessaire pour les erreurs

### Requirement: `hasWarnings` exposé dans le contrat JSON

Le rapport JSON `ValidateReportV1` MUST porter un champ additif
`hasWarnings: bool` — `true` dès qu'au moins un finding de sévérité
`Warning` est présent, `false` sinon. Le champ est **toujours** présent
dans la sortie, indépendamment de la valeur ou du mode strict.

Ce champ est **informatif** : le consommateur qui veut trancher en
dehors du mode strict s'en sert. L'exit code reste le signal officiel.

#### Scenario: `hasWarnings: true` sur un rapport avec warning

- **GIVEN** un projet qui remonte un warning `decision_unsealed`
- **WHEN** l'utilisateur lance `codev validate --json`
- **THEN** le document JSON porte `"hasWarnings": true` en racine

#### Scenario: `hasWarnings: false` sur un rapport propre

- **GIVEN** un projet sans finding
- **WHEN** l'utilisateur lance `codev validate --all --json`
- **THEN** le document JSON porte `"hasWarnings": false`

#### Scenario: `hasWarnings` indépendant du mode strict

- **GIVEN** un projet qui remonte un warning et aucun erreur
- **WHEN** l'utilisateur lance `codev validate --strict --json`
- **THEN** le document JSON porte `"hasWarnings": true`
- **AND** l'exit code de la commande est non-nul — les deux signaux
  coexistent sans se contredire
