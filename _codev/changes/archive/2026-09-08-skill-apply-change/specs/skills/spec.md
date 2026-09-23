## Purpose

Décrit le contrat des skills que codev installe dans Claude Code : leur nom,
ce qu'elles doivent faire, ce qu'elles n'ont pas le droit de faire, et
comment leur frontmatter garantit ces promesses. Les entrées sont ajoutées
au fil des changes qui introduisent chaque workflow — un ADDED par workflow.

## ADDED Requirements

### Requirement: Skill `apply` guide l'implémentation d'un change

Le catalogue de codev SHALL exposer un workflow `apply` — installé sous
`.claude/skills/codev-apply/SKILL.md`, invocable `/codev-apply` — dont le
rôle est de traiter les tâches non cochées du `tasks.md` d'un change, dans
l'ordre du fichier, en cochant chaque case à mesure.

#### Scenario: Implémentation d'un change avec un seul actif

- **GIVEN** un projet avec un seul change actif dont `tasks.md` porte deux
  tâches non cochées
- **WHEN** l'utilisateur tape `/codev-apply`
- **THEN** la skill résout implicitement le change actif
- **AND** implémente la première tâche puis la coche
- **AND** implémente la seconde tâche puis la coche

#### Scenario: Reprise après interruption

- **GIVEN** un `tasks.md` où la première tâche est déjà cochée `- [x]` et la
  seconde ne l'est pas
- **WHEN** l'utilisateur tape `/codev-apply`
- **THEN** la skill ignore la tâche déjà cochée
- **AND** commence par la première tâche non cochée

#### Scenario: Ambiguïté demande un choix explicite

- **GIVEN** deux changes actifs
- **WHEN** l'utilisateur tape `/codev-apply` sans nom
- **THEN** la skill demande lequel appliquer, en listant les deux noms

### Requirement: Skill `apply` respecte les frontières du change

Le workflow `apply` MUST se cantonner à ce qui est nécessaire pour cocher
les tâches du change nommé : il MUST NOT modifier d'autres changes, MUST NOT
archiver ni sync tout seul, et MUST s'arrêter dès qu'une tâche est
ambiguë ou bloquée plutôt que de deviner.

#### Scenario: Refus d'archiver depuis apply

- **GIVEN** un change dont toutes les tâches sont cochées
- **WHEN** l'utilisateur tape `/codev-apply`
- **THEN** la skill signale que le change est prêt à être archivé
- **AND** invite explicitement à lancer `/codev-archive` ou `codev archive`
  comme prochaine étape séparée

#### Scenario: Tâche ambiguë interrompt le flux

- **GIVEN** un `tasks.md` contenant une tâche dont la formulation admet
  plusieurs interprétations qui changeraient matériellement le résultat
- **WHEN** la skill arrive à cette tâche
- **THEN** la skill demande une clarification à l'utilisateur avant
  d'implémenter
- **AND** ne coche pas la tâche tant que la clarification n'est pas obtenue

### Requirement: Contrat du frontmatter d'une skill codev

Toute skill livrée par codev MUST porter un frontmatter YAML valide dont le
`name` correspond au nom du dossier `.claude/skills/<name>/`, dont le champ
`allowed-tools` inclut au moins `Bash(codev:*)`, et dont `metadata.version`
correspond à la version du binaire qui l'a générée.

#### Scenario: Frontmatter parseur par un lecteur YAML tiers

- **GIVEN** une skill livrée par la version courante du binaire
- **WHEN** son frontmatter est extrait et passé à un parseur YAML standard
- **THEN** le parseur rend `name`, `allowed-tools` et `metadata.version`
  sans erreur
- **AND** `metadata.version` égale la version que le binaire annonce

#### Scenario: Édition à la main détectée à l'update

- **GIVEN** une skill dont un utilisateur a édité le corps à la main, sans
  changer sa version
- **WHEN** l'utilisateur relance `codev update` sans `--force`
- **THEN** la skill n'est pas écrasée
- **AND** le rapport de l'update la signale comme préservée
