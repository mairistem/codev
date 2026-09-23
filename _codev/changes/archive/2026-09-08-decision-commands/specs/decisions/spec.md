## ADDED Requirements

### Requirement: Création d'une décision par CLI

`codev decision new <titre>` SHALL créer un nouvel ADR dans le projet, avec
un `id` numérique généré automatiquement (le plus grand `id` numérique
trouvé dans le projet + 1, à quatre chiffres), un fichier nommé
`NNNN-<slug>.md` où `<slug>` est dérivé du titre en kebab-case, et un
frontmatter valide comportant `status: accepted` et la date du jour.

#### Scenario: Création dans un projet sans ADR

- **GIVEN** un projet dont le dossier `_codev/decisions/` est vide
- **WHEN** l'utilisateur lance `codev decision new "Un premier choix"`
- **THEN** le fichier `_codev/decisions/0001-un-premier-choix.md` est créé
- **AND** son frontmatter porte `id: "0001"`, `title: "Un premier choix"`,
  `status: accepted`, et une date au format `AAAA-MM-JJ`

#### Scenario: Numérotation à la suite

- **GIVEN** un projet dont le dossier `_codev/decisions/` contient déjà six
  ADR numérotés `0001` à `0006`
- **WHEN** l'utilisateur lance `codev decision new "Un septième choix"`
- **THEN** le fichier `_codev/decisions/0007-un-septieme-choix.md` est créé
- **AND** son `id` vaut `"0007"`

#### Scenario: Statut personnalisable

- **GIVEN** un projet
- **WHEN** l'utilisateur lance `codev decision new "Piste à explorer"
  --status proposed`
- **THEN** l'ADR créé porte `status: proposed`

#### Scenario: Titre vide refusé

- **GIVEN** un projet
- **WHEN** l'utilisateur lance `codev decision new ""`
- **THEN** aucun fichier n'est écrit
- **AND** un message d'erreur nomme le code stable `empty_title` et
  demande un titre non vide

### Requirement: Listing des décisions par CLI

`codev decision list` MUST afficher toutes les décisions locales et
héritées, avec pour chacune son identifiant, son titre, son statut, son
état d'effet, et son origine. Le mode `--json` MUST rendre exactement un
document JSON dont la forme est figée par version.

#### Scenario: Liste avec décisions locales et une supersession

- **GIVEN** un projet contenant trois ADR `accepted` où `0007` supersede
  `0003`
- **WHEN** l'utilisateur lance `codev decision list --json`
- **THEN** le tableau `decisions` contient les trois entrées
- **AND** l'entrée `0003` porte `inEffect: false` et `supersededBy:
  "projet/0007"`
- **AND** l'entrée `0007` porte `inEffect: true`

#### Scenario: Rendu humain concis

- **GIVEN** le même contexte
- **WHEN** l'utilisateur lance `codev decision list`
- **THEN** chaque décision apparaît sur sa propre ligne avec, dans
  l'ordre, un marqueur d'effet (`•` pour en vigueur, `–` sinon), son `id`,
  son titre, et son statut

### Requirement: Affichage d'une décision par CLI

`codev decision show <id>` MUST afficher le contenu complet de la décision
demandée. La résolution accepte un `id` court quand il est non ambigu, ou
un identifiant qualifié `<origin>/<id>` en cas de collision.

#### Scenario: Show d'un id sans collision

- **GIVEN** un projet contenant un ADR `0001` unique
- **WHEN** l'utilisateur lance `codev decision show 0001`
- **THEN** le rendu contient l'en-tête du frontmatter (`id`, `title`,
  `status`, `date`) suivi du corps markdown de la décision

#### Scenario: Show d'un id ambigu

- **GIVEN** un projet contenant un ADR `0007` local **et** un ADR `0007`
  dans une source héritée `path:~/partage`
- **WHEN** l'utilisateur lance `codev decision show 0007`
- **THEN** aucun contenu n'est affiché
- **AND** le message d'erreur `ambiguous_decision_id` liste les deux
  identifiants qualifiés (`projet/0007`, `path:~/partage/0007`) et demande
  de préciser

#### Scenario: Show avec un identifiant qualifié

- **GIVEN** le même contexte
- **WHEN** l'utilisateur lance `codev decision show path:~/partage/0007`
- **THEN** la décision héritée est affichée

### Requirement: Supersession d'une décision par CLI

`codev decision supersede <ancien-id> <nouveau-titre>` MUST créer une
nouvelle décision qui référence l'ancienne dans son `supersedes`, et
réécrire le frontmatter de l'ancienne pour que son `status` passe à
`superseded`. Les deux écritures se font dans un même plan — soit les
deux réussissent, soit aucune n'est appliquée.

#### Scenario: Supersession locale

- **GIVEN** un projet contenant un ADR `0003 accepted` intitulé « Vieux
  choix »
- **WHEN** l'utilisateur lance `codev decision supersede 0003 "Nouveau
  choix"`
- **THEN** un nouvel ADR `0007-nouveau-choix.md` (ou le prochain `id`
  disponible) est créé avec `supersedes: ["0003"]` et `status: accepted`
- **AND** le frontmatter du fichier `0003` a maintenant `status:
  superseded`
- **AND** le corps du fichier `0003` — Contexte, Décision, tout ce qui
  suit le frontmatter — est resté identique au caractère près

#### Scenario: Supersession d'un id introuvable refuse tout

- **GIVEN** un projet dans lequel `9999` n'existe pas
- **WHEN** l'utilisateur lance `codev decision supersede 9999 "X"`
- **THEN** aucun fichier n'est écrit
- **AND** le message d'erreur `unknown_decision_id` nomme `9999`

#### Scenario: Supersession d'une décision héritée refusée

- **GIVEN** un projet héritant d'une source contenant `path:~/partage/0100`
- **WHEN** l'utilisateur lance `codev decision supersede path:~/partage/0100
  "Notre alternative"`
- **THEN** aucun fichier n'est écrit
- **AND** le message d'erreur `cannot_supersede_inherited` explique qu'une
  décision héritée est en lecture seule et suggère de la « dévier » — nom
  du geste que K6 livrera

### Requirement: Contrat JSON stable pour toutes les commandes

Sur demande `--json`, chaque commande MUST écrire sur stdout exactement un
document JSON dont la forme est figée par version, avec un tableau
`status` racine pour les erreurs d'exécution — mêmes règles que le reste
du contrat de codev.

#### Scenario: Forme d'un `decision new --json`

- **GIVEN** un projet
- **WHEN** l'utilisateur lance `codev decision new "X" --json`
- **THEN** stdout porte un document JSON contenant `changeName: null`
  (cette commande n'agit pas sur un change), `decision: { id: "0001",
  qualifiedId: "projet/0001", path: "…", title: "X", status: "accepted"
  }`, et un `status: []`

#### Scenario: Forme d'un échec `decision show` sur id inconnu

- **GIVEN** un projet
- **WHEN** l'utilisateur lance `codev decision show 9999 --json`
- **THEN** stdout porte un unique document JSON contenant
  `decision: null` et `status: [{ code: "unknown_decision_id", … }]`
