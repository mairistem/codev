# Proposal : livrer `/codev-sync` et `/codev-archive`

## Pourquoi

Le cycle est bouclé côté CLI (`codev sync`, `codev archive`) et côté planification
(`/codev-propose`, `/codev-apply`). Le dernier maillon manquant : rendre le
bouclage invocable **dans le chat**, sans que l'utilisateur ait à quitter
Claude Code pour taper une commande bash. Deux skills — c'est peu — mais elles
transforment le rythme d'usage : « planifier, implémenter, archiver » devient
trois slash commands consécutives.

## Ce qui change

- **Nouveau workflow `sync`** dans le catalogue, invocable `/codev-sync` :
  résout un change actif, lance `codev sync <nom>`, résume les specs
  principales créées/modifiées/inchangées. Ne déplace jamais le change.
- **Nouveau workflow `archive`** dans le catalogue, invocable `/codev-archive` :
  vérifie que la planification est complète, lance `codev archive <nom>`,
  résume la destination archive et les specs principales touchées. Si
  `codev archive` refuse (pré-flight validate en échec), la skill renvoie
  explicitement vers `/codev-validate` (à venir) ou `codev validate <nom>`
  pour voir le détail — sans tenter de trancher elle-même.
- **Deux fichiers d'assets** : `assets/workflows/sync.md` et
  `assets/workflows/archive.md`, chargés à la compilation via `include_str!`
  comme les trois autres.
- **Ces deux workflows n'ont pas besoin de `Bash` général** — leur seule action
  effective est le passage par `codev` lui-même. `allowed-tools` reste donc
  `Bash(codev:*), Read` pour les deux (le `Read` sert à relire `tasks.md` si
  l'utilisateur pose une question de contexte).
- **Rendu structuré à partir du JSON du CLI** : les deux skills invoquent
  `codev sync <nom> --json` et `codev archive <nom> --json`, lisent le
  `SyncReportV1` / `ArchiveReportV1` (contrat public déjà versionné et testé
  par snapshot), et rendent à l'utilisateur un résumé net — nombres par
  catégorie, chemin d'archive, code stable d'erreur en cas de refus.
- **La skill `sync` termine par une invitation à archiver quand la fusion a
  produit du changement** : « Le change est prêt à être archivé si tu veux
  clore le cycle. » Une seule ligne, non-injonctive.

## Capacités

### Nouvelles capacités

- `skills`

**Note d'ordonnancement.** La capacité `skills` est également déclarée par le
change `skill-apply-change` actuellement actif. Deux changes qui déclarent la
même capacité « nouvelle » est le cas normal : celui qui est archivé en
premier crée `_codev/specs/skills/spec.md` avec son `Purpose` et ses `ADDED`
propres ; celui qui est archivé en second voit `merge_into_existing` ajouter
ses propres `ADDED` à la spec existante — le `## Purpose` du second delta est
silencieusement ignoré, comme le contrat le veut. Aucun conflit à prévoir,
quel que soit l'ordre.

### Capacités modifiées

Aucune — les workflows existants (`propose`, `explore`, `apply`) ne bougent
pas.

## Impact

- **Code** : deux entrées `Workflow { … }` de plus dans le `CATALOG` de
  `codev-agents::workflows`, un test dédié
  (`workflows::cycle_completion_skills_present_et_restreintes`) qui vérifie
  que `sync` et `archive` sont là, avec le bon `allowed-tools` (pas de `Bash`
  général) et une description claire.
- **Config** : ajouter `- sync` et `- archive` à la liste `workflows` de
  `_codev/config.yaml` de ce dépôt, pour que la skill existe après un
  `codev update`.
- **Catalogue par défaut** : NE PAS ajouter `sync` ni `archive` à
  `DEFAULT_WORKFLOWS` — même décision que pour `apply`, pour rester cohérent :
  le catalogue par défaut se limite à ce qui prépare le travail (propose,
  explore), le reste est opt-in projet par projet.
- **Hors périmètre** :
  - **`update`** en tant que skill — sémantique différente (révise des
    artefacts déjà écrits, ne boucle rien), à traiter séparément si le
    besoin apparaît.
  - **Auto-installation par défaut** — cf. paragraphe précédent.
  - **Parsing d'autres commandes que sync/archive en JSON** — la skill ne
    lit le JSON que de ces deux commandes précises. Un changement de la
    forme de `codev status --json` ou d'un autre contrat ne l'affecte pas.
