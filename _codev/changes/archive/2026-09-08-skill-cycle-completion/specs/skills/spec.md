## Purpose

Décrit le contrat des skills que codev installe dans Claude Code : leur nom,
ce qu'elles doivent faire, ce qu'elles n'ont pas le droit de faire, et
comment leur frontmatter garantit ces promesses. Les entrées sont ajoutées
au fil des changes qui introduisent chaque workflow — un ADDED par workflow.

## ADDED Requirements

### Requirement: Skill `sync` merge le delta d'un change sans le déplacer

Le catalogue de codev SHALL exposer un workflow `sync` — installé sous
`.claude/skills/codev-sync/SKILL.md`, invocable `/codev-sync` — dont le rôle
est de faire entrer les deltas d'un change dans les specs principales, en
laissant le change actif à son emplacement.

#### Scenario: Sync d'un change actif unique

- **GIVEN** un projet avec un seul change actif dont la planification est
  complète et qui porte un delta ADDED sur une capacité nouvelle
- **WHEN** l'utilisateur tape `/codev-sync`
- **THEN** la skill résout implicitement le change actif
- **AND** lance `codev sync <nom>`
- **AND** résume à l'utilisateur les main specs créées ou mises à jour

#### Scenario: Deuxième sync silencieux

- **GIVEN** un change déjà synchronisé, dont aucune main spec n'a changé
  depuis
- **WHEN** l'utilisateur tape `/codev-sync` une seconde fois
- **THEN** la skill rend compte qu'il n'y a rien à faire
- **AND** ne relance pas d'écriture

#### Scenario: Sync ne déplace jamais

- **GIVEN** un change dont la fusion réussit
- **WHEN** l'utilisateur tape `/codev-sync`
- **THEN** le dossier `_codev/changes/<nom>/` existe toujours à son
  emplacement d'origine

#### Scenario: Sync invite à archiver après un changement

- **GIVEN** un change dont la fusion a modifié au moins une spec principale
  (créée ou mise à jour)
- **WHEN** l'utilisateur tape `/codev-sync`
- **THEN** le rendu final contient une ligne invitant à `/codev-archive`
  pour clore le cycle, formulée sans injonction

#### Scenario: Sync sans changement n'invite pas

- **GIVEN** un change dont la fusion est un no-op (toutes les specs
  principales sont déjà à jour)
- **WHEN** l'utilisateur tape `/codev-sync`
- **THEN** le rendu final rend compte de l'absence de changement
- **AND** ne suggère PAS d'archiver — il n'y a rien de nouveau à propager

### Requirement: Skill `archive` clôt un change avec pré-flight strict

Le catalogue SHALL exposer un workflow `archive` — installé sous
`.claude/skills/codev-archive/SKILL.md`, invocable `/codev-archive` — dont le
rôle est de fusionner le delta puis de déplacer le change vers
`_codev/changes/archive/<date>-<nom>/`. La skill MUST refuser d'agir si
`codev archive` rapporte un pré-flight de validation en échec.

#### Scenario: Archive d'un change validé

- **GIVEN** un change dont la planification est complète et qui passe
  `codev validate`
- **WHEN** l'utilisateur tape `/codev-archive`
- **THEN** la skill lance `codev archive <nom>`
- **AND** résume à l'utilisateur les main specs touchées
- **AND** nomme la destination d'archive datée

#### Scenario: Archive refusé pour erreur de validation

- **GIVEN** un change dont un delta contient une erreur remontée par
  `codev validate` (par exemple, une exigence dupliquée)
- **WHEN** l'utilisateur tape `/codev-archive`
- **THEN** la skill n'insiste pas
- **AND** invite explicitement l'utilisateur à lancer `codev validate <nom>`
  pour voir le détail
- **AND** ne tente pas de deviner ou de corriger l'erreur

### Requirement: Skills `sync` et `archive` s'appuient sur le contrat JSON

Les workflows `sync` et `archive` MUST invoquer le CLI avec `--json` et lire
la forme structurée (`SyncReportV1`, `ArchiveReportV1`) plutôt que la sortie
humaine — c'est le contrat public que codev garantit stable dans sa version
courante, et c'est ce qui rend le rendu de la skill fiable.

#### Scenario: Rendu structuré des créations et mises à jour

- **GIVEN** un change dont la fusion crée une spec principale et en met une
  autre à jour
- **WHEN** l'utilisateur tape `/codev-sync`
- **THEN** le rendu nomme distinctement les deux — le fichier créé et le
  fichier mis à jour — chacun sur sa ligne

#### Scenario: Refus d'archive détecté par code stable

- **GIVEN** un change dont `codev archive --json` refuse avec le code
  `validation_failed` dans son tableau `status`
- **WHEN** l'utilisateur tape `/codev-archive`
- **THEN** la skill détecte le code stable dans le JSON
- **AND** dit exactement : « Le change a des erreurs. Lance `codev validate
  <nom>` pour voir le détail. »
- **AND** ne parse pas le message humain (qui peut être reformulé sans
  préavis)

### Requirement: Skills `sync` et `archive` ne demandent pas le Bash général

Les workflows `sync` et `archive` MUST se limiter à `Bash(codev:*)` et à des
outils de lecture dans leur frontmatter `allowed-tools` — ils n'exécutent
aucune commande de vérification autre que celles du binaire codev, à
l'inverse de `apply` qui doit pouvoir lancer des tests projets.

#### Scenario: Le Bash général n'apparaît pas

- **GIVEN** la skill `sync` livrée par la version courante
- **WHEN** son frontmatter est inspecté
- **THEN** la chaîne `allowed-tools` ne contient pas `Bash` seul en fin de
  liste, seulement le préfixe `Bash(codev:*)`
- **AND** la même règle vaut pour `archive`
