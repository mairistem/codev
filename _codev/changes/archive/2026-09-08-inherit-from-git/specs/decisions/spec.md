## ADDED Requirements

### Requirement: Héritage depuis un dépôt git distant

Un projet SHALL pouvoir déclarer `inherits: git:` dans son
`_codev/config.yaml` pour hériter d'un dépôt git distant. La déclaration
supporte les champs `git` (URL, obligatoire), `ref` (branche ou tag,
obligatoire), et `subpath` (chemin dans le dépôt, optionnel).

#### Scenario: Décisions d'un dépôt git héritées et indexées

- **GIVEN** un projet dont `_codev/config.yaml` déclare `inherits: [{git:
  "git@github.com:acme/codev-shared.git", ref: main}]`
- **AND** un fichier `codev.lock` verrouillant un SHA `9f2c1ab7`
- **AND** un cache local sous `~/.cache/codev/content/9f2c1ab7/` contenant
  un ADR `0100 accepted`
- **WHEN** le validateur calcule l'index des décisions
- **THEN** l'ADR `0100` apparaît dans l'index
- **AND** son `origin` est `git:git@github.com:acme/codev-shared.git`
- **AND** son `qualifiedId` est
  `git:git@github.com:acme/codev-shared.git/0100`

#### Scenario: Décisions héritées injectées dans les instructions design

- **GIVEN** le même projet
- **WHEN** l'utilisateur lance `codev instructions design --change <nom>`
- **THEN** le tableau `decisions` de la réponse contient les décisions
  locales **et** les décisions héritées `accepted` du dépôt git verrouillé

### Requirement: Source git jamais lue depuis une branche flottante

Le validateur MUST refuser d'exposer le contenu d'une source `git:` tant
qu'un SHA n'a pas été verrouillé dans `_codev/codev.lock`. Aucune
commande courante (`list`, `show`, `status`, `instructions`, `validate`,
`sync`, `archive`) ne SHALL contacter le réseau — c'est `codev sources
update` seul qui déplace un pin.

#### Scenario: Source git déclarée mais non verrouillée

- **GIVEN** un projet déclarant `inherits: [{git: "…", ref: main}]`
- **AND** un `codev.lock` absent ou sans entrée pour cette source
- **WHEN** l'utilisateur lance `codev instructions design --change <nom>`
- **THEN** le tableau `decisions` ne contient que les décisions locales
- **AND** le champ `status[]` de la réponse porte un warning de code
  stable `git_source_unlocked` invitant à lancer `codev sources update`

#### Scenario: `codev status` ne contacte pas le réseau

- **GIVEN** un projet déclarant une source `git:` avec un SHA verrouillé
  qui n'est pas en cache
- **WHEN** l'utilisateur lance `codev status --change <nom>` alors que
  le réseau est indisponible
- **THEN** la commande n'échoue pas pour raison réseau
- **AND** aucun appel `git` n'est fait par l'exécution

### Requirement: `codev sources update` résout et verrouille

La commande `codev sources update` MUST, pour chaque source `git:`
déclarée, résoudre le `ref` demandé en SHA via `git ls-remote`, télécharger
le contenu si le SHA n'est pas en cache, et écrire un nouveau
`_codev/codev.lock` où le SHA de chaque source correspond à la résolution
courante. Elle MUST afficher un diff des changements de SHA avant
d'écrire.

#### Scenario: Premier update sur un projet sans lock

- **GIVEN** un projet déclarant une source `git:` mais sans `codev.lock`
- **WHEN** l'utilisateur lance `codev sources update`
- **THEN** `git ls-remote` est appelé pour résoudre le `ref`
- **AND** un cache est peuplé avec le contenu du SHA
- **AND** un nouveau `_codev/codev.lock` est écrit portant la ligne
  résolue

#### Scenario: Update sans changement

- **GIVEN** un projet dont le lock verrouille déjà le SHA résolu
  actuellement par `git ls-remote`
- **WHEN** l'utilisateur lance `codev sources update`
- **THEN** aucun téléchargement supplémentaire n'est effectué
- **AND** le fichier `codev.lock` n'est pas réécrit (comparaison contenu
  à contenu)

#### Scenario: Diff avant écriture d'un pin déplacé

- **GIVEN** un projet dont le lock porte `commit: aaaa1111` mais
  `git ls-remote` rend maintenant `bbbb2222`
- **WHEN** l'utilisateur lance `codev sources update`
- **THEN** la sortie humaine montre `aaaa1111 → bbbb2222` pour cette
  source avant l'écriture du lock

#### Scenario: git absent du PATH

- **GIVEN** un système dont le binaire `git` est introuvable
- **WHEN** l'utilisateur lance `codev sources update`
- **THEN** la commande échoue avec le code stable `git_not_found`
- **AND** le message rappelle que `codev sources update` est la seule
  commande qui a besoin de `git`

### Requirement: `codev sources list` et `codev sources show`

`codev sources list` MUST lister toutes les sources déclarées avec leur
état ; `codev sources show <ref>` MUST afficher les détails d'une source
précise, identifiée par son URL (pour une source `git:`) ou son chemin
(pour une `path:`).

#### Scenario: List montre l'état de chaque source

- **GIVEN** un projet avec une `path:` et une `git:` verrouillée
- **WHEN** l'utilisateur lance `codev sources list`
- **THEN** deux entrées apparaissent, chacune avec son type (`path` ou
  `git`), son adresse, et son état (`resolved`, `locked`, ou `unlocked`)

#### Scenario: Show pointe vers le cache résolu

- **GIVEN** un projet avec une source `git:` verrouillée sur `9f2c1ab7`
- **WHEN** l'utilisateur lance `codev sources show
  "git@github.com:acme/codev-shared.git"`
- **THEN** la sortie contient l'URL, le `ref` demandé, le SHA verrouillé,
  et le chemin résolu dans le cache
- **AND** liste les fichiers exposés (décisions, specs héritées)

### Requirement: Aucun contenu exécutable hérité

Le loader SHALL n'exposer aux consommateurs (index de décisions, index
de specs héritées, config héritée) que des fichiers avec les extensions
`.md` et `.yaml` — même si le dépôt source en contient d'autres. Un
fichier `.sh`, `.py`, `.rs`, un exécutable, un hook, ne SHALL JAMAIS être
chargé depuis une source héritée.

#### Scenario: Un script dans le dépôt hérité est ignoré

- **GIVEN** un dépôt git hérité qui contient `_codev/decisions/hook.sh`
- **WHEN** le validateur calcule l'index
- **THEN** aucun élément de l'index ne référence `hook.sh`
- **AND** aucune commande de codev ne lance ce fichier
