## ADDED Requirements

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
