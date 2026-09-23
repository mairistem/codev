# Proposal : livrer la skill `/codev-apply`

## Pourquoi

Le cycle du dépôt fonctionne aujourd'hui **jusqu'à la planification**
(`/codev-explore` + `/codev-propose`) et **au-delà de la validation**
(`codev sync`, `codev archive` en CLI direct). Le maillon central manque : la
skill qui guide l'agent, dans le chat de Claude Code, à travers les tâches de
`tasks.md`. Sans elle, l'utilisateur doit demander l'implémentation par une
phrase libre à chaque fois — sans garde-fou, sans invariant, sans suivi.

## Ce qui change

- **Nouveau workflow `apply`** dans le catalogue `codev-agents::workflows` —
  troisième entrée à côté de `propose` et `explore`, invocable
  `/codev-apply` une fois installée par `codev init`/`codev update`.
- **Nouveau fichier d'assets `assets/workflows/apply.md`** — le corps de la
  skill, chargé à la compilation via `include_str!`, comme les deux autres.
- **Comportement attendu** :
  - lit `tasks.md` du change nommé (déduit s'il n'y en a qu'un seul actif) ;
  - implémente les tâches non cochées, dans l'ordre du fichier ;
  - coche `- [ ]` → `- [x]` à mesure ;
  - respecte la frontière d'un change : ne modifie pas d'autres changes,
    et refuse d'archiver ou de sync — c'est un pas suivant explicite ;
  - `allowed-tools` autorise `Bash(codev:*), Read, Write, Edit, Glob, Grep,
    Bash` (le dernier pour les commandes de build/test des tâches).

## Capacités

### Nouvelles capacités

- `skills`

Ce change **crée** la capacité, avec pour Purpose de décrire le contrat des
skills livrées, et n'y met qu'un `ADDED` pour `apply` — les workflows
`propose` et `explore` existent déjà en tant que fichiers mais n'ont pas
encore leur exigence documentée. Un futur change pourra les ajouter par un
`ADDED` supplémentaire, sans que ce change s'en occupe.

### Capacités modifiées

Aucune.

## Impact

- **Code** : nouveau `Workflow { id: "apply", … }` dans le `CATALOG` de
  `codev-agents::workflows`, plus une entrée dans les tests d'invariant du
  catalogue (frontmatter YAML valide, description assez longue, `Bash(codev:*)`
  présent).
- **Config par défaut** : le commentaire de `codev init` mentionnait déjà
  `apply` dans son bloc d'exemples ; le catalogue par défaut
  (`DEFAULT_WORKFLOWS`) reste `[propose, explore]` — ajouter `apply` par
  défaut n'est pas obligatoire pour que la skill existe, elle apparaît dès
  qu'un projet la déclare dans `_codev/config.yaml`.
- **Hors périmètre** :
  - **`update`, `sync`, `archive`** en tant que skills — ces workflows
    existent déjà comme commandes CLI (`codev sync`, `codev archive`)
    directement invocables via `Bash`. Un change ultérieur (nommé
    `skill-cycle-completion` ou similaire) livrera leurs skills.
  - **Auto-ajout de `apply` à `DEFAULT_WORKFLOWS`** — décision à prendre
    séparément, après retour d'expérience sur l'usage réel de la skill.
  - **Skills paramétrées** (`/codev-apply --dry-run`, etc.) — Claude Code
    ne supporte pas les arguments de skills nativement ; les options
    passent par le CLI derrière.
