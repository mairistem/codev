# Proposal : livrer `/codev-onboard`

## Pourquoi

Un utilisateur qui découvre codev — soit qu'il vient d'installer le
binaire, soit qu'il ouvre Claude Code sur un projet où quelqu'un
d'autre a déjà lancé `codev init` — ne sait pas par quoi commencer. Les
six skills existantes (`propose`, `explore`, `apply`, `sync`, `archive`,
`update`) suffisent pour **travailler**, mais aucune ne se présente. Le
README explique le concept, mais Claude Code ne le lit pas.

Résultat aujourd'hui : les nouveaux utilisateurs tapent `/help`, voient
une liste de slash-commands sans hiérarchie, et n'ont aucun repère pour
distinguer « par où entrer » de « quand utiliser ». `/codev-onboard`
livre ce repère — une seule invocation, une carte du terrain, une
recommandation d'action.

Utile aussi aux nouveaux **workflows MCP** : quand le premier vrai
projet JVS branchera un MCP Jira, l'utilisateur qui l'ouvrira pour la
première fois voudra un point d'entrée qui explique « voici comment
cette combinaison codev + Jira marche ici ». La skill onboard, du
projet-parent ou héritée, tiendra ce rôle.

## Ce qui change

- **Nouveau workflow `onboard`** dans le catalogue, invocable
  `/codev-onboard`. Rôle : montrer l'état actuel du projet
  (initialisé ou non, specs présentes, changes actifs, décisions
  indexées) et recommander la prochaine action.
- **Nouveau fichier `assets/workflows/onboard.md`** — le corps de la
  skill, chargé via `include_str!` comme les six autres.
- **`onboard` est ajouté à `DEFAULT_WORKFLOWS`** — passage de
  `["propose", "explore"]` à `["propose", "explore", "onboard"]`. Un
  utilisateur qui lance `codev init` dans un projet neuf voit donc
  `/codev-onboard` disponible immédiatement, sans opt-in à ajouter
  dans `config.yaml`.
- **Frontière stricte lecture seule** — pas d'écriture, pas de
  création de change, pas d'appel à `codev init`. La skill **guide**,
  elle **n'agit pas** à la place de l'utilisateur.
- **`allowed-tools` restreint** — `Bash(codev:*), Read, Glob`. Pas de
  `Bash` général ; pas d'`Edit`, pas de `Write`.

## Capacités

### Nouvelles capacités

Aucune.

### Capacités modifiées

- `skills` — deux nouvelles exigences ADDED : présence de `onboard`
  dans le catalogue, et son inclusion dans `DEFAULT_WORKFLOWS`.

### Capacités retirées

Aucune.

## Impact

- **Code** : nouvelle entrée `Workflow { id: "onboard", … }` dans le
  `CATALOG` de `codev-agents::workflows`, `DEFAULT_WORKFLOWS` étendu à
  trois entrées, et deux tests dédiés (présence, `allowed-tools`).
- **Test existant `sans_demande_installe_le_catalogue_par_defaut`** —
  ajustement mineur : la liste attendue passe de deux à trois entrées,
  et la vérification « opt-in » passe de 4 à 3 workflows
  (`apply`, `sync`, `archive`, `update` restent opt-in ; `onboard`
  devient default).
- **Test `chaque_workflow_a_un_corps_…`** — couvre automatiquement
  `onboard`.
- **Config du dépôt** : `_codev/config.yaml` gagne `- onboard` à sa
  liste `workflows` — pour dogfood, puisque le dépôt utilise ce qu'il
  fabrique.
- **Contrat JSON** : rien. La skill lit des JSON existants (`codev list
  --specs --json` n'existe pas encore — la skill lira les sorties
  humaines qui existent, cohérent avec `apply`/`update`).
- **Migration** : aucune. Les projets qui ont déjà un `config.yaml`
  avec une liste explicite `workflows:` **ne** gagnent **pas**
  `onboard` automatiquement (le default ne s'applique que quand la
  clef est absente) — ils l'ajoutent quand ils veulent.
- **Hors périmètre** :
  - **Un tutoriel interactif étape par étape** (« maintenant tape
    ceci, puis ça… ») — trop injonctif ; la skill informe et propose,
    l'utilisateur agit.
  - **Un mode `--refresh` qui reset l'état** — pas un problème de
    présentation, pas la responsabilité de cette skill.
  - **Détection automatique de patterns MCP** (numéros de tickets,
    URL Figma) — c'est le rôle des skills d'action (`propose`,
    `apply`), pas de `onboard`.
