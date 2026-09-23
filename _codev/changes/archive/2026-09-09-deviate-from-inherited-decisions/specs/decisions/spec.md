## ADDED Requirements

### Requirement: Champ `deviates_from` reconnu dans le frontmatter d'un ADR

Le parseur d'ADR SHALL reconnaître un champ optionnel `deviates_from`
dans le frontmatter YAML — une liste d'identifiants qualifiés
(`<origin>/<id>`) qui pointe vers les décisions dont ce nouvel ADR se
détache localement. Le champ est additif : son absence conserve la
sémantique actuelle des ADR. Une valeur qui n'est pas une liste, ou dont
les entrées ne sont pas des chaînes, est signalée.

#### Scenario: ADR avec `deviates_from`

- **GIVEN** un ADR local dont le frontmatter porte
  `deviates_from: ["path:~/partage/0100"]`
- **WHEN** le parseur lit le fichier
- **THEN** le résultat expose la liste `["path:~/partage/0100"]` sur le
  champ `deviates_from`
- **AND** aucun finding n'est émis pour ce champ

#### Scenario: `deviates_from` mal formé

- **GIVEN** un ADR dont le frontmatter porte `deviates_from: "pas-une-liste"`
- **WHEN** le parseur lit le fichier
- **THEN** un finding de code `decision_field_type_mismatch` signale le
  champ `deviates_from`

### Requirement: Commande `codev decision deviate` crée un ADR de dérive

`codev decision deviate <qualified-id> <titre>` MUST créer un ADR local
`accepted` avec `deviates_from: ["<qualified-id>"]` et un frontmatter
valide (`id`, `title`, `status: accepted`, `date`), scellé par le même
plan d'effets — cohérence avec K3.

La commande MUST refuser :

- si `<qualified-id>` désigne une décision locale (`projet/…`) : code
  stable `cannot_deviate_from_local`, avec un message qui renvoie vers
  `codev decision supersede`.
- si `<qualified-id>` ne correspond à aucune décision indexée : code
  stable `unknown_decision_id` (déjà existant).
- si `<titre>` est vide : code stable `empty_title` (déjà existant).

#### Scenario: Dérive d'une décision héritée `path:`

- **GIVEN** un projet héritant d'une source `path: ~/partage` contenant
  un ADR `path:~/partage/0100`
- **WHEN** l'utilisateur lance
  `codev decision deviate path:~/partage/0100 "Notre alternative locale"`
- **THEN** un nouvel ADR local est créé sous
  `_codev/decisions/NNNN-notre-alternative-locale.md` avec
  `deviates_from: ["path:~/partage/0100"]` et `status: accepted`
- **AND** une entrée est ajoutée à `_codev/decisions/seal.yaml` pour ce
  nouvel ADR

#### Scenario: Refus de dériver d'une décision locale

- **GIVEN** un projet contenant un ADR local `0003 accepted`
- **WHEN** l'utilisateur lance `codev decision deviate projet/0003 "…"`
- **THEN** aucun fichier n'est écrit
- **AND** le message d'erreur nomme le code stable
  `cannot_deviate_from_local` et suggère `codev decision supersede`

#### Scenario: Refus d'une cible inconnue

- **GIVEN** un projet sans source héritée
- **WHEN** l'utilisateur lance
  `codev decision deviate path:~/inconnue/0100 "…"`
- **THEN** aucun fichier n'est écrit
- **AND** le message d'erreur nomme le code stable `unknown_decision_id`

### Requirement: L'index cache les décisions héritées déviées et expose `deviated_by`

Quand l'index des décisions calcule les entrées en vigueur, chaque
décision héritée référencée par un `deviates_from` d'un ADR local
`accepted` MUST être marquée `deviated_by: <qualified-id-local>` dans
l'index, retirée du tableau `in_effect`, et n'apparaître ni dans le
tableau `decisions[]` des instructions de `design`, ni dans la section
humaine « Décisions en vigueur » de son rendu.

L'entrée héritée reste visible dans `codev decision list` — la
transparence prime sur l'invisibilisation.

#### Scenario: Instructions design ne portent pas la décision déviée

- **GIVEN** un projet qui hérite d'une source contenant
  `path:~/partage/0100 accepted`, et un ADR local `0007 accepted` dont
  le frontmatter porte `deviates_from: ["path:~/partage/0100"]`
- **WHEN** l'utilisateur lance
  `codev instructions design --change <nom> --json`
- **THEN** le tableau `decisions` de la réponse contient `projet/0007`
- **AND** le tableau `decisions` ne contient PAS `path:~/partage/0100`

#### Scenario: `decision list` expose la dérive

- **GIVEN** le même contexte
- **WHEN** l'utilisateur lance `codev decision list --json`
- **THEN** l'entrée `path:~/partage/0100` porte `deviatedBy:
  "projet/0007"` et `inEffect: false`
- **AND** l'entrée `projet/0007` porte `deviatesFrom:
  ["path:~/partage/0100"]` et `inEffect: true`

#### Scenario: Un ADR local `proposed` ne fait pas dévier

- **GIVEN** un ADR local `0007 proposed` avec `deviates_from:
  ["path:~/partage/0100"]`
- **WHEN** l'index est calculé
- **THEN** `path:~/partage/0100` reste en vigueur (le proposed n'est pas
  encore engagé, il ne peut pas dévier)
- **AND** l'entrée `path:~/partage/0100` ne porte pas de `deviatedBy`

### Requirement: `validate` détecte les dérives dégénérées

`codev validate` MUST émettre deux nouveaux findings de code stable :

- `decision_dangling_deviation` — **warning** — quand un ADR local a un
  `deviates_from: ["<qualified-id>"]` dont la cible n'existe pas ou
  n'existe plus dans l'index (source retirée, SHA déplacé, id changé).
- `decision_conflicting_deviations` — **erreur** — quand deux ADR
  locaux `accepted` référencent la même cible dans leur
  `deviates_from`. La règle est : « une cible, une dérive ».

#### Scenario: Dérive orpheline

- **GIVEN** un projet dont un ADR local `0007 accepted` porte
  `deviates_from: ["path:~/inconnue/9999"]`, sans qu'aucune source
  n'expose ce `qualified-id`
- **WHEN** l'utilisateur lance `codev validate`
- **THEN** la sortie contient un finding de code
  `decision_dangling_deviation` nommant `0007` et sa cible manquante
- **AND** le code d'erreur de la commande est nul (warning)

#### Scenario: Deux dérives sur la même cible

- **GIVEN** un projet avec deux ADR locaux `accepted`, `0007` et
  `0008`, tous deux avec `deviates_from: ["path:~/partage/0100"]`
- **WHEN** l'utilisateur lance `codev validate`
- **THEN** la sortie contient un finding
  `decision_conflicting_deviations` qui nomme les deux ADR et la cible
  en conflit
- **AND** le code d'erreur de la commande est non nul (erreur)
