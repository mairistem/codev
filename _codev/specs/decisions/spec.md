# Decisions Specification

## Purpose

Rendre les décisions d'architecture exploitables par codev : les lire, les
indexer, résoudre la chaîne des supersessions, et les injecter dans les
instructions de l'artefact `design` pour qu'un agent qui rédige voie
d'emblée les choix déjà tranchés.

## Requirements

### Requirement: Format ADR reconnu

Le parseur SHALL reconnaître un ADR écrit en frontmatter YAML suivi de
sections markdown libres. Le frontmatter porte au moins les champs `id`,
`title`, `status`, `date`, et éventuellement `tags` (liste), `supersedes`
(identifiant ou tableau d'identifiants). Un fichier sans frontmatter ou dont
le frontmatter manque un champ obligatoire est signalé, pas parsé
silencieusement.

#### Scenario: ADR bien formé

- **GIVEN** un fichier commençant par `---`, contenant un frontmatter YAML
  avec `id: 0007`, `title: "…"`, `status: accepted`, `date: 2026-09-08`,
  `tags: [architecture]`, puis un séparateur `---` puis du markdown libre
- **WHEN** le parseur lit le fichier
- **THEN** le résultat expose l'identifiant `0007`, le titre, le statut
  `accepted`, la date et le tag `architecture`

#### Scenario: ADR sans frontmatter

- **GIVEN** un fichier markdown sans en-tête `---`
- **WHEN** le parseur lit le fichier
- **THEN** un finding de code stable `decision_missing_frontmatter` signale
  que le fichier n'a pas le format attendu

#### Scenario: Champ obligatoire manquant

- **GIVEN** un fichier dont le frontmatter n'a pas de `title`
- **WHEN** le parseur lit le fichier
- **THEN** un finding de code `decision_missing_field` nomme le champ
  manquant

### Requirement: Statuts reconnus et effet

Le validateur MUST reconnaître les statuts `accepted`, `superseded`,
`proposed`, `deprecated`, `rejected` ; seuls `accepted` et `superseded`
sont pris en compte pour le calcul de l'effet — les trois autres sont
exposés tels quels dans l'index et jamais considérés comme « en vigueur ».

#### Scenario: Statut inconnu signalé

- **GIVEN** un ADR dont le `status` vaut `pending`
- **WHEN** le parseur lit le fichier
- **THEN** un finding de code `decision_unknown_status` signale la valeur
  et rappelle la liste des statuts reconnus

#### Scenario: Statut proposed n'entre pas en vigueur

- **GIVEN** un ADR de statut `proposed`, sans lien de supersession
- **WHEN** l'index est calculé
- **THEN** cette décision n'apparaît pas dans les « décisions en vigueur »

### Requirement: Supersession résolue en chaîne

L'index MUST résoudre le champ `supersedes` : chaque décision qu'un ADR
supersede est marquée `superseded_by(<id>)` dans l'index, et n'est pas en
vigueur. Une chaîne `A ← B ← C` laisse `A` et `B` supersedées, seule `C`
reste en vigueur.

#### Scenario: Supersession directe

- **GIVEN** un ADR `0003 accepted` et un ADR `0007 accepted supersedes: [0003]`
- **WHEN** l'index est calculé
- **THEN** `0003` est marqué `superseded_by(0007)` et n'apparaît pas dans
  les décisions en vigueur
- **AND** `0007` apparaît dans les décisions en vigueur

#### Scenario: Chaîne à trois maillons

- **GIVEN** trois ADR `accepted` où `B` supersede `A` et `C` supersede `B`
- **WHEN** l'index est calculé
- **THEN** seul `C` est en vigueur

#### Scenario: Cible de supersession absente

- **GIVEN** un ADR `0007 supersedes: [9999]` alors que `9999` n'existe pas
- **WHEN** l'index est calculé
- **THEN** un finding de code `decision_supersedes_unknown` signale
  l'identifiant fantôme
- **AND** `0007` reste en vigueur (le lien perdu ne le disqualifie pas)

### Requirement: Décisions héritées prises en compte

Quand un projet déclare `inherits: path: <chemin>` dans son
`_codev/config.yaml`, le validateur MUST parser aussi les ADR de
`<chemin>/_codev/decisions/` et les fusionner dans l'index avec leur
`origin` visible.

#### Scenario: ADR d'une source héritée apparaît dans l'index

- **GIVEN** un projet qui hérite d'une source `path: ~/partage` contenant
  un ADR `0100 accepted`
- **WHEN** l'index est calculé
- **THEN** l'entrée porte l'`origin` `path:~/partage`
- **AND** son identifiant qualifié pour éviter les collisions est
  `path:~/partage/0100`

#### Scenario: Collision d'id entre projet et source

- **GIVEN** un ADR `0007 accepted` local **et** un ADR `0007 accepted` dans
  une source héritée
- **WHEN** l'index est calculé
- **THEN** un finding de code `decision_id_collision` signale le doublon
- **AND** la version du projet gagne (elle est plus proche de l'auteur)

### Requirement: Injection dans les instructions de design

L'appel `codev instructions design --change <nom>` MUST enrichir sa réponse
d'un champ `decisions[]` porteur des décisions en vigueur. Chaque entrée
expose `id`, `title`, `status`, `tags`, `path` (relatif au projet) et
`origin`. Le contenu complet reste dans le fichier — pas de duplication
dans la réponse.

#### Scenario: Instructions design portent les décisions en vigueur

- **GIVEN** un projet avec 6 ADR `accepted` et aucun supersession
- **WHEN** l'utilisateur lance `codev instructions design --change <nom>
  --json`
- **THEN** la réponse JSON contient un tableau `decisions` avec exactement
  6 entrées, chacune portant `id`, `title`, `status`, `path` relatif au
  projet et `origin: "projet"`

#### Scenario: Décisions supersedées absentes des instructions

- **GIVEN** un projet où `0003` est supersedée par `0007`
- **WHEN** l'utilisateur lance `codev instructions design --change <nom>
  --json`
- **THEN** `0003` n'apparaît pas dans le tableau `decisions`
- **AND** `0007` y apparaît

#### Scenario: Rendu humain liste les décisions

- **GIVEN** le même contexte
- **WHEN** l'utilisateur lance `codev instructions design --change <nom>`
  (sans `--json`)
- **THEN** le rendu contient une section « Décisions en vigueur » listant
  chaque décision sur sa propre ligne avec son `id` et son `title`

#### Scenario: Aucune décision, aucune section

- **GIVEN** un projet neuf sans aucun ADR
- **WHEN** l'utilisateur lance `codev instructions design --change <nom>`
- **THEN** aucune section « Décisions en vigueur » n'est ajoutée au rendu
- **AND** le champ `decisions[]` dans la réponse JSON est présent et vide

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

### Requirement: Sceau du corps enregistré par le CLI

Le CLI SHALL maintenir un fichier `_codev/decisions/seal.yaml` — versionné
avec le projet — qui recense les décisions locales scellées, avec pour
chaque entrée l'`id` de la décision, le hash SHA-256 du **corps** de l'ADR
(tout ce qui suit le séparateur `---` fermant le frontmatter), et la date
à laquelle le sceau a été apposé. Seules les décisions locales sont
scellées — les décisions héritées relèvent du projet source, pas du
consommateur.

Le fichier ne contient que la version du schéma et la liste des sceaux —
un sceau, c'est un `id`, un `bodySha256` préfixé `sha256:`, et un `sealedAt`
au format `AAAA-MM-JJ`.

#### Scenario: Sceau écrit à la création d'un projet

- **GIVEN** un projet dont `_codev/decisions/seal.yaml` n'existe pas
- **WHEN** l'utilisateur lance `codev decision new "Un premier choix"`
- **THEN** le fichier `_codev/decisions/seal.yaml` est créé
- **AND** il contient une entrée dont l'`id` est `"0001"` et dont le
  `bodySha256` correspond au SHA-256 hexadécimal du corps du fichier
  `0001-un-premier-choix.md` (préfixé `sha256:`)

#### Scenario: Format du fichier de sceau

- **GIVEN** le fichier `_codev/decisions/seal.yaml` créé par le CLI
- **WHEN** un humain l'ouvre
- **THEN** il contient une clé `version: 1` en tête
- **AND** une clé `seals` portant une liste dont chaque entrée a
  exactement les champs `id`, `bodySha256`, `sealedAt`
- **AND** aucun autre champ inconnu

### Requirement: `decision new` écrit l'ADR et le sceau dans le même plan

L'opération `codev decision new <titre>` MUST produire un plan d'effets
qui écrit à la fois l'ADR **et** l'entrée correspondante dans
`_codev/decisions/seal.yaml`. Si l'un des deux échoue, aucun n'est écrit
— la commande ne laisse jamais un ADR sans sceau ni un sceau sans ADR.

#### Scenario: Échec d'écriture du sceau annule la création

- **GIVEN** un projet dans lequel `_codev/decisions/seal.yaml` est
  verrouillé en écriture par le système
- **WHEN** l'utilisateur lance `codev decision new "Titre"`
- **THEN** aucun fichier ADR n'est écrit dans `_codev/decisions/`
- **AND** le fichier `seal.yaml` reste inchangé

### Requirement: `decision supersede` scelle le nouvel ADR sans toucher au sceau de l'ancien

L'opération `codev decision supersede <id> <titre>` MUST ajouter une
entrée de sceau pour le nouvel ADR créé, et MUST NOT modifier l'entrée de
sceau de l'ancien — puisque son corps reste identique au caractère près
(exigence déjà en vigueur), son sceau reste valide.

#### Scenario: Supersession scelle uniquement le nouveau

- **GIVEN** un projet contenant un ADR `0003 accepted` scellé et référencé
  dans `seal.yaml` sous `id: "0003"`
- **WHEN** l'utilisateur lance `codev decision supersede 0003 "Nouveau choix"`
- **THEN** le fichier `seal.yaml` contient maintenant deux entrées : celle
  de `0003` (inchangée) et une nouvelle pour l'ADR créé (par exemple
  `0007`)
- **AND** l'entrée `0003` a exactement le même `bodySha256` qu'avant la
  supersession

### Requirement: `validate` détecte les altérations du corps

`codev validate` MUST émettre trois nouveaux findings de code stable
pour signaler les écarts entre les ADR et leur sceau :

- `decision_unsealed` — **warning** émis pour tout ADR local de statut
  `accepted` ou `superseded` qui n'a pas d'entrée dans `seal.yaml`. Le
  message rappelle que la migration se fait par `codev decision seal`.
- `decision_seal_mismatch` — **erreur** émise pour tout ADR dont le
  `bodySha256` calculé ne correspond plus à celui enregistré dans
  `seal.yaml`. Le message nomme l'ADR concerné et rappelle qu'une
  modification délibérée passe par `codev decision seal --force`.
- `decision_orphan_seal` — **warning** émis pour toute entrée de
  `seal.yaml` dont l'ADR référencé n'existe plus dans
  `_codev/decisions/`.

#### Scenario: Détection d'un corps modifié en place

- **GIVEN** un projet dont l'ADR `0001` est scellé
- **AND** un humain a édité le corps du fichier `0001-*.md` sans mettre à
  jour `seal.yaml`
- **WHEN** l'utilisateur lance `codev validate`
- **THEN** la sortie contient un finding de code `decision_seal_mismatch`
  nommant `0001`
- **AND** le code d'erreur de la commande est non nul

#### Scenario: Détection d'un ADR non scellé

- **GIVEN** un projet contenant six ADR locaux `accepted`, tous sans
  entrée dans `seal.yaml` (état de migration initial)
- **WHEN** l'utilisateur lance `codev validate`
- **THEN** la sortie contient six findings de code `decision_unsealed`,
  un par ADR
- **AND** le code d'erreur de la commande est nul (warnings, pas erreurs)

#### Scenario: Détection d'un sceau orphelin

- **GIVEN** un projet dont `seal.yaml` contient une entrée pour
  l'ADR `0004`
- **AND** le fichier `0004-*.md` a été supprimé
- **WHEN** l'utilisateur lance `codev validate`
- **THEN** la sortie contient un finding `decision_orphan_seal` nommant
  `0004`
- **AND** le code d'erreur de la commande est nul (warning, pas erreur)

### Requirement: `decision seal` ajoute ou renouvelle une entrée de sceau

`codev decision seal <id>` MUST créer une entrée de sceau pour un ADR
local qui n'en a pas — c'est le geste de migration. Pour un ADR déjà
scellé dont le corps a changé, la commande MUST refuser sans `--force` et
rappeler que l'immutabilité est intentionnelle. Avec `--force`, elle
réécrit le `bodySha256` et met à jour `sealedAt`. Un ADR héritée ne peut
pas être scellé par le projet consommateur (le scellement appartient au
projet source).

#### Scenario: Migration d'un ADR non scellé

- **GIVEN** un projet contenant un ADR `0001 accepted` sans entrée dans
  `seal.yaml`
- **WHEN** l'utilisateur lance `codev decision seal 0001`
- **THEN** `seal.yaml` porte maintenant une entrée pour `0001` dont le
  `bodySha256` correspond au corps courant du fichier

#### Scenario: Refus de re-sceller sans `--force`

- **GIVEN** un projet dont l'ADR `0001` est scellé, et dont le corps a
  ensuite été édité en place
- **WHEN** l'utilisateur lance `codev decision seal 0001`
- **THEN** `seal.yaml` reste inchangé
- **AND** le message d'erreur nomme le code stable `seal_conflict` et
  rappelle que `--force` réécrit délibérément le sceau

#### Scenario: Re-sceau avec `--force`

- **GIVEN** le même contexte
- **WHEN** l'utilisateur lance `codev decision seal 0001 --force`
- **THEN** l'entrée `0001` dans `seal.yaml` porte maintenant le nouveau
  `bodySha256` et une date `sealedAt` correspondant à la date du jour

#### Scenario: Refus de sceller une décision héritée

- **GIVEN** un projet héritant d'une source contenant `path:~/partage/0100`
- **WHEN** l'utilisateur lance `codev decision seal path:~/partage/0100`
- **THEN** aucune écriture n'a lieu
- **AND** le message d'erreur nomme le code stable
  `cannot_seal_inherited` et rappelle que le scellement appartient au
  projet source

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

### Requirement: Commande `codev decision promote` extrait un bloc du design en ADR

`codev decision promote <change> <titre>` MUST créer un nouvel ADR local
sous `_codev/decisions/NNNN-<slug>.md`, avec `status: accepted`, scellé
par K3, dont le corps reproduit **verbatim** le contenu du bloc
`### Décision : <titre>` trouvé sous la section `## Décisions` du
`design.md` du change. Le nouvel ADR contient une section `## Décision`
qui porte ce corps, et les sections `## Contexte`, `## Conséquences` et
`## Alternatives écartées` sont émises avec un `<!-- placeholder -->`
inviant l'auteur à ventiler.

Le corps « verbatim » du bloc s'entend comme : tout le texte qui suit la
ligne `### Décision : <titre>` jusqu'à la prochaine ligne commençant par
`### ` ou `## ` (exclu), sans normalisation.

Refus explicites — codes stables :

- `unknown_change` : le change n'est pas dans `codev list`.
- `cannot_promote_from_archived` : la cible pointe vers un dossier sous
  `changes/archive/` — un design archivé est de l'histoire.
- `design_missing` : le change n'a pas de `design.md`.
- `decision_heading_not_found` : aucun bloc `### Décision : <titre>` ne
  correspond dans le design.
- `ambiguous_decision_heading` : plusieurs blocs portent le même titre —
  l'utilisateur précise en éditant.

#### Scenario: Promotion réussie

- **GIVEN** un change `add-auth` dont `design.md` contient sous
  `## Décisions` un bloc `### Décision : Utiliser JWT` avec deux
  paragraphes de rationale
- **WHEN** l'utilisateur lance `codev decision promote add-auth
  "Utiliser JWT"`
- **THEN** un nouvel ADR est créé sous
  `_codev/decisions/NNNN-utiliser-jwt.md` avec `status: accepted`, `date`
  du jour, et un frontmatter valide
- **AND** son corps contient une section `## Décision` avec les deux
  paragraphes de rationale, byte pour byte
- **AND** une entrée est ajoutée à `_codev/decisions/seal.yaml` pour cet
  ADR — cohérence avec K3

#### Scenario: Refus d'un change archivé

- **GIVEN** un change qui vit sous `_codev/changes/archive/…-<nom>/`
- **WHEN** l'utilisateur lance `codev decision promote <nom> "..."`
- **THEN** aucun fichier n'est écrit
- **AND** le message d'erreur nomme le code stable
  `cannot_promote_from_archived`

#### Scenario: Titre introuvable

- **GIVEN** un change `add-auth` dont le `design.md` ne mentionne pas de
  bloc « Utiliser Kerberos »
- **WHEN** l'utilisateur lance `codev decision promote add-auth
  "Utiliser Kerberos"`
- **THEN** aucun fichier n'est écrit
- **AND** le message d'erreur nomme le code stable
  `decision_heading_not_found`

#### Scenario: Titre ambigu

- **GIVEN** un `design.md` contenant **deux** blocs
  `### Décision : Choix de la librairie` (par exemple une version
  révisée du premier bloc pendant la discussion)
- **WHEN** l'utilisateur lance `codev decision promote <c> "Choix de la
  librairie"`
- **THEN** aucun fichier n'est écrit
- **AND** le message d'erreur nomme le code stable
  `ambiguous_decision_heading` et invite à éditer un des deux titres

### Requirement: Le design est mis à jour avec une référence traçable

Après une promotion réussie, le bloc `### Décision : <titre>` du
`design.md` MUST être remplacé par le même titre suivi d'**une seule
ligne** de citation textuelle qui référence le nouvel ADR :

```
### Décision : <titre>

> Promue en ADR **NNNN** — voir `_codev/decisions/NNNN-<slug>.md`.
```

La référence est en texte simple (`` ` `` pour le chemin), pas un lien
markdown — un lien `[..](../..)` casserait au moment de l'archive du
change (où la profondeur des `..` change). Le titre du bloc est
préservé pour permettre à un lecteur du design de comprendre le sujet
qui a été promu.

#### Scenario: Contenu du bloc remplacé par la référence

- **GIVEN** le même contexte que le scénario « Promotion réussie »
- **WHEN** l'utilisateur lance la commande
- **THEN** le `design.md` du change contient désormais, à
  l'emplacement du bloc :
  ```
  ### Décision : Utiliser JWT

  > Promue en ADR **NNNN** — voir `_codev/decisions/NNNN-utiliser-jwt.md`.

  ```
- **AND** le reste du fichier (autres sections, autres blocs `###`,
  espacement) est identique au caractère près
- **AND** l'espacement autour du bloc reste préservé — pas de ligne
  blanche ajoutée ni retirée

#### Scenario: Deux promotions successives sur le même design

- **GIVEN** un design contenant deux blocs distincts :
  `### Décision : A` et `### Décision : B`
- **AND** l'utilisateur a déjà promu `A` en ADR
- **WHEN** l'utilisateur lance `codev decision promote <c> "B"`
- **THEN** le bloc `B` est promu à son tour
- **AND** la ligne de référence de `A` n'est PAS altérée par cette
  seconde promotion — chaque promotion n'agit que sur son propre bloc
