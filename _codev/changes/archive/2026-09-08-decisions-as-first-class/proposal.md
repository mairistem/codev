# Proposal : les décisions d'architecture, objet de première classe

## Pourquoi

Le dépôt contient déjà 6 ADR sous `_codev/decisions/` — écrits à la main, lus
par les humains, cités à la main dans le `design.md` des changes. C'est le
socle qu'annonce le README et la raison pour laquelle codev existe plutôt
qu'OpenSpec. Aujourd'hui pourtant, aucun code de codev ne les *voit* : elles
ne sont pas indexées, elles ne sont pas injectées dans les instructions de
`design`, et un agent qui rédige un design peut tout à fait re-débattre en
silence un choix déjà tranché. Ce change livre la brique qui rend les
décisions **exploitables par l'outil**.

## Ce qui change

- **Format ADR reconnu** — frontmatter YAML (`id`, `title`, `status`, `date`,
  `tags`, `supersedes` optionnel) suivi de sections libres. Le format déjà
  utilisé par les 6 ADR du dépôt.
- **Parseur pur** dans `codev-core::decisions` — prend le texte, rend un
  `Decision` typé, sans I/O.
- **Index des décisions** dans `codev-engine::decisions` — lit
  `_codev/decisions/` du projet et de chaque source héritée `path:`,
  résout les liens `supersedes` → calcule pour chaque décision son état
  d'effet (`in_effect` / `superseded_by(<id>)` / `superseded_by(<source>/<id>)`).
- **Statuts reconnus** : `accepted`, `superseded`, `proposed`, `deprecated`,
  `rejected` — seuls `accepted` et `superseded` interviennent dans le calcul
  d'effet. Les autres sont exposés tels quels dans l'index.
- **Injection dans les instructions de `design`** — l'appel `codev
  instructions design --change <nom>` gagne un nouveau champ
  `decisions[]` porteur des décisions **en vigueur** (celles qui ne sont
  supersedées par aucune autre). Chaque entrée porte `id`, `title`, `status`,
  `tags`, `path` relatif, et `origin` (`projet` ou `path:<chemin>`). Le
  contenu complet reste dans le fichier — l'agent le lit via `path`, comme
  les dépendances.
- **Rendu humain de `codev instructions design`** enrichi d'une section
  « Décisions en vigueur » qui liste les entrées, une ligne chacune.

## Capacités

### Nouvelles capacités

- `decisions`

### Capacités modifiées

- `skills` — le workflow `propose` (et éventuellement `apply` plus tard)
  continue de lire `codev instructions`. La forme du contrat s'enrichit d'un
  champ, sans casser l'existant. Aucune modification d'exigence, donc pas
  de delta MODIFIED sur cette capacité.

## Impact

- **Code** : nouveau module `codev-core::decisions` (parseur + AST), nouveau
  module `codev-engine::decisions` (index + supersession), extension de
  `codev-engine::instructions::Instructions` avec un champ `decisions:
  Vec<DecisionRef>`, nouveau `DecisionRefV1` dans `contract::v1`, rendu humain
  étendu dans `codev-cli::render`.
- **Dépendances** : aucune nouvelle — le frontmatter YAML est déjà géré par
  `serde_norway`, le parsing de sections utilise la mécanique du module
  `parser`.
- **Sources héritées** : quand un `inherits: path: X` est déclaré, les ADR
  de `X/_codev/decisions/` sont fusionnés dans l'index, avec provenance
  visible. Ordre de précédence : projet en dernier (donc « gagne » pour
  un même `id` — cas rare, à signaler comme conflit).
- **Hors périmètre** :
  - **K3 — immuabilité** d'une décision `accepted` : détecter une
    modification demande un hash de référence, dont la re-génération
    demande à son tour une commande CLI (`codev decision seal`) qui n'existe
    pas encore. Reporté avec K5 pour rester cohérent.
  - **K5 — commandes CLI** `codev decision new/list/show/supersede` :
    l'utilisateur continue de créer et éditer ses ADR à la main pour ce
    change. Ergonomiquement suffisant en dogfooding, à améliorer ensuite.
  - **K6 — déviation** `deviates-from` d'une décision héritée : demande K5
    en amont pour créer proprement une décision de déviation.
  - **K7 — promotion** depuis `design.md` : demande un chemin d'écriture
    guidé qui rejoint K5.
  - **Injection dans les instructions d'autres artefacts** que `design` :
    l'index existe côté engine, un futur change peut l'exposer ailleurs
    (par exemple dans `proposal` pour rappeler les décisions qui bornent la
    scope). Rien de bloquant, juste hors du strict K4.
