# Proposal : commandes CLI pour les décisions

## Pourquoi

Les six ADR du dépôt ont été écrits à la main, dans un fichier créé avec
`touch`. Ça a marché parce que le format est simple — mais ça ne monte pas :
un utilisateur qui veut créer une septième décision doit se souvenir du
préfixe à quatre chiffres, du frontmatter obligatoire, du slug de fichier.
Le change précédent a rendu les décisions **exploitables** par l'outil ; ce
change les rend **manipulables** par lui.

## Ce qui change

- **`codev decision new <titre>`** — crée un nouvel ADR sous
  `_codev/decisions/`. Détermine automatiquement l'`id` (le plus grand
  entier trouvé + 1, formaté à quatre chiffres), dérive le nom de fichier
  du titre (`0007-slug-du-titre.md`), écrit un squelette avec les sections
  standard (Contexte / Décision / Conséquences / Alternatives écartées).
  Statut par défaut `accepted` — l'usage documenté du dépôt.
- **`codev decision list`** — liste toutes les décisions locales et
  héritées, avec pour chacune son `id`, son titre, son statut, son état
  d'effet (`in_effect` ou `superseded_by:<id>`), et son origine.
- **`codev decision show <id>`** — affiche une décision précise, sortie
  humaine ou JSON. Résout par `id` local si non ambigu, exige un
  qualifieur `<origin>/<id>` en cas de collision inter-sources.
- **`codev decision supersede <ancien-id> <nouveau-titre>`** — crée une
  nouvelle décision qui référence l'ancienne dans son `supersedes`, et
  réécrit le frontmatter de l'ancienne pour passer son `status` à
  `superseded`. Les deux écritures sont dans un même `Plan` : atomique.
- **Contrat JSON étendu** — nouveaux types `DecisionV1` (list, show,
  new, supersede) dans `contract::v1`, en camelCase.

## Capacités

### Nouvelles capacités

Aucune.

### Capacités modifiées

- `decisions` — le change ajoute quatre nouvelles exigences décrivant les
  commandes CLI. La capacité existante (parseur, index, injection dans
  `design`) reste inchangée.

## Impact

- **Code** : nouveau module `codev-engine::decisions::actions` avec des
  fonctions pures `plan_new(index, title, options) -> Result<Plan, …>` et
  `plan_supersede(index, old_id, new_title) -> Result<Plan, …>` ; nouveau
  groupe de sous-commandes `codev decision …` dans `codev-cli` ; nouveaux
  contrats `DecisionV1`, `DecisionCreatedV1`, `DecisionSupersededV1`.
- **Dépendances** : aucune nouvelle.
- **Squelette d'ADR** : embarqué dans le binaire via `include_str!`, sous
  `assets/templates/decision.md`. Un projet qui voudrait un squelette
  différent devra l'écrire à la main pour l'instant — templater ce fichier
  est un futur change si le besoin remonte.
- **Hors périmètre** :
  - **K3 (immuabilité)** — détecter la modification d'une décision
    `accepted` demande un hash de référence stocké. Ce chantier est un
    change suivant, désormais moins bloqué puisque les mutations passeront
    par le CLI.
  - **K6 (déviation `deviates-from`)** — un projet qui hérite d'une
    décision peut vouloir s'en écarter localement. Un champ dédié dans le
    frontmatter et une commande `codev decision deviate <origin/id>`
    seraient des ajouts propres, à traiter séparément.
  - **K7 (promotion depuis `design.md`)** — reconnaître une décision
    d'architecture dans un `design.md` archivé et la promouvoir en ADR,
    lot suivant.
  - **Édition interactive** dans un `$EDITOR` après création — l'outil se
    contente d'écrire le fichier ; l'utilisateur l'ouvre lui-même. Le
    faire proprement demande un port `ProcessRunner` pour l'éditeur, qui
    n'existe pas encore.
