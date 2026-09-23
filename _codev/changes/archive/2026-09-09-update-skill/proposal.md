# Proposal : livrer `/codev-update`

## Pourquoi

Sur ce dépôt même, dans les changes récents, j'ai édité un artefact de
planification à la main **au moins quatre fois** : une révision de proposal
après discussion, un ajustement de design, une correction de tasks une fois
`validate` réel. Chaque fois : ouvrir le fichier, éditer, relancer
`codev validate` de mémoire, croiser les doigts pour ne pas avoir cassé la
cohérence entre proposal, design et tasks. C'est le pattern qui revient le
plus après les cinq skills livrées, et le seul qui manque au cycle *fluide*
qu'annonçait le README.

## Ce qui change

- **Nouveau workflow `update`** dans le catalogue, invocable
  `/codev-update` une fois installé. Il révise un artefact de planification
  déjà écrit (proposal, specs, design, tasks) — un à la fois, guidé par la
  description de l'utilisateur, en préservant la cohérence avec les autres.
- **Nouveau fichier `assets/workflows/update.md`** — le corps de la skill,
  chargé à la compilation via `include_str!` comme les cinq autres.
- **Frontière stricte** : la skill modifie **uniquement** les fichiers sous
  `_codev/changes/<nom>/` et **jamais** de code du projet. Elle ne crée pas
  non plus d'artefact manquant — c'est le rôle de `/codev-propose`.
- **Ripple annoncé, pas caché** : quand la révision d'un artefact rend un
  autre incohérent (par exemple, retirer une capacité du proposal alors
  qu'un fichier `specs/<capa>/` a déjà été écrit), la skill le signale à
  l'utilisateur et propose la correction avant d'agir.
- **`codev validate` en garde-fou final** : après application des révisions,
  la skill relance `codev validate <change>` et affiche le résultat.
- **Sortie humaine** — la skill lit la sortie texte des commandes qu'elle
  invoque, sans parser de JSON. Choix cohérent avec `apply`, qui a la même
  nature (guide l'agent, ne consomme pas de contrat structuré).

## Capacités

### Nouvelles capacités

Aucune.

### Capacités modifiées

- `skills` — trois nouvelles exigences ADDED décrivant le contrat de
  `/codev-update` : révision d'un artefact, ripple annoncé, frontière
  planning-seulement.

## Impact

- **Code** : nouvelle entrée `Workflow { id: "update", … }` dans le
  `CATALOG` de `codev-agents::workflows`, un test dédié qui vérifie sa
  présence et son `allowed-tools`.
- **Config** : ajouter `- update` à la liste `workflows` de
  `_codev/config.yaml` de ce projet.
- **Catalogue par défaut** : NE PAS ajouter `update` à `DEFAULT_WORKFLOWS`
  — cohérent avec la ligne actuelle qui limite le catalogue par défaut à
  ce qui prépare le travail (`propose`, `explore`).
- **Hors périmètre** :
  - **Édition en batch** de plusieurs artefacts en un seul appel — chaque
    appel `/codev-update` cible un artefact et son ripple ; une révision
    plus large se fait en plusieurs invocations séquentielles.
  - **Modification du code** — la frontière est stricte. Un ajustement du
    code se fait par `/codev-apply` après révision.
  - **Création d'artefacts manquants** — c'est `/codev-propose` (ou
    `/codev-continue` du profil étendu) qui les crée.
  - **Édition d'un change déjà archivé** — techniquement possible en
    modifiant les fichiers sous `changes/archive/`, mais la skill refuse.
    Un change archivé est de l'histoire ; le corriger demande de le
    dé-archiver à la main.
