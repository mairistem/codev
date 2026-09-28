# Design : skill `configure` + nudge

## Contexte

Voir `proposal.md`. Nouveau workflow (le 8ème), fonction pure
`is_config_thin` dans `codev-core`, deux nudges dans `codev-cli`,
mise à jour du body `onboard`.

## Décisions

### Décision : seuil « thin » = `context < 200 chars ET rules vides`

Deux critères conjonctifs. Un projet dont un utilisateur a rempli l'un
ou l'autre est considéré comme « déjà configuré » — la skill
`configure` n'a plus rien à offrir de fondamental.

Le seuil de 200 caractères est arbitraire mais éclairé par la sortie
minimale de `codev init` sur un projet Rust nu : `Projet Rust, 2024.`
fait 22 caractères. Le seuil de 200 laisse largement passer un
`context:` détaillé (stack + une phrase de contexte), et coupe court
aux stubs.

**Alternative écartée A** : seuil sur `context` seul. Rejeté — un
utilisateur qui a écrit des `rules:` sans `context:` a explicitement
choisi sa configuration.

**Alternative écartée B** : seuil configurable via `_codev/config.yaml`.
Rejeté — sur-ingénierie ; le seuil est un défaut, pas une politique.

### Décision : `configure` a `Bash(codev:*), Read, Glob, Grep, Write, Edit` — mais pas Bash général

La skill doit :

- Lire des fichiers (`Read`, `Glob`, `Grep`).
- Écrire un `_codev/config.yaml` (`Write`).
- Éditer avec précision — préserver les commentaires — (`Edit`).
- Appeler `codev status`, `codev list --specs` pour se situer
  (`Bash(codev:*)`).

Elle **n'a pas** le `Bash` général : elle n'exécute pas de tests, pas
de `git`, pas d'outil externe. La règle d'invariant « seule `apply` a
le Bash général » reste préservée.

**Alternative écartée** : donner le Bash général pour laisser Claude
lire l'historique via `git log`. Rejeté — le fichier `.git/index` +
`git log` suffit rarement pour comprendre un projet, et `Read` sur
`README.md` + un échantillon de code fait 90% du travail.

### Décision : la skill affiche le diff, écrit sur confirmation — jamais d'écriture silencieuse

Le body de `configure.md` MUST guider Claude sur une séquence stricte :

1. Rassembler ce qu'on veut proposer (patch YAML).
2. **Afficher** le diff à l'utilisateur — ligne par ligne, avec le
   commentaire d'origine « voici ce que je propose ».
3. Demander « Applique ? [oui/non] ».
4. Écrire seulement si oui.

La séquence est explicite dans le body — pas une convention implicite.
Une skill qui écrit sans confirmation trahirait la confiance.

**Alternative écartée** : mode `--auto` qui écrit sans confirmation
pour scripter. Rejeté — c'est justement une skill, pas un CLI. Un
utilisateur qui veut scripter édite `_codev/config.yaml` à la main
via un template.

### Décision : la fonction `is_config_thin` vit dans `codev-core::config`

C'est une fonction pure sur `ProjectConfig` — pas de I/O, pas
d'environnement. Elle appartient au cœur.

Le fait qu'elle prenne un type de `codev-engine` (`ProjectConfig`)
casserait le sens de dépendance. **Solution** : la fonction prend en
paramètres directement `context: Option<&str>` et `rules_empty: bool`,
pas le `ProjectConfig` entier. Chaque appelant lit ces deux champs et
passe. Simple, testable seule.

**Alternative écartée** : dupliquer `ProjectConfig` dans `codev-core`
juste pour cette fonction. Rejeté — la duplication de type est un
coût élevé pour un gain minuscule.

### Décision : la nudge dans `codev status` n'apparaît que quand aucun change n'est actif

`codev status` a plusieurs modes de sortie — avec change actif, sans,
avec plusieurs. Le nudge n'a de sens que dans le cas « projet
initialisé, aucun change actif » — c'est-à-dire le cas typique après
un `codev init`. L'insérer dans le cas « change actif » polluerait
l'écran d'un utilisateur en cours de travail.

Techniquement : le nudge s'affiche uniquement dans la branche
d'erreur `no_active_change` de la sortie humaine, jamais dans le
rapport JSON.

### Décision : lecture du projet par la skill — un budget explicite

Le body de `configure.md` MUST inscrire des limites explicites pour
que Claude ne lise pas tout le projet :

- **README** — lecture complète.
- **CONTRIBUTING.md** — lecture complète si présent.
- **docs/** — glob des `*.md`, lecture des 3-5 fichiers les plus
  courts.
- **Fichiers source** — jusqu'à 8 fichiers, priorisés par récence de
  commit (`git log --since='6 months ago' --pretty=format: --name-only`)
  filtrés par extension usuelle du langage détecté.
- **Structure** — `ls _codev/`, `ls src/` ou équivalent — un seul
  niveau.

Ce budget contient le coût token et la variabilité de sortie. Sans
budget, Claude lirait au hasard et proposerait des contextes
incohérents.

**Alternative écartée** : « laisse Claude lire ce qu'il veut ». Rejeté
— l'expérience prouve que sans borne, la sortie devient imprévisible.

### Décision : mise à jour du body `onboard` — nouvelle branche AVANT les autres

Dans la table de décision de `onboard.md`, la branche « config thin »
doit être placée **avant** les branches sur les changes, car un
projet fraîchement initialisé n'a par définition aucun change actif —
sinon on serait déjà dans une autre branche.

Ordre final :

1. `_codev/` absent → `codev init`
2. Config thin, aucun change → `/codev-configure` puis `/codev-propose`
3. Aucun change → lire README puis `/codev-propose`
4. Un change actif, planification incomplète → `/codev-propose <nom>`
5. Un change actif, planification complète → `/codev-apply <nom>`
6. Plusieurs changes → lister

## Risques et compromis

- **Le seuil « thin » vieillit** — un projet peut avoir un context de
  180 caractères qui est parfait, et la nudge sera fausse. →
  **Atténuation** : la nudge est douce (« Prochaine étape
  recommandée » / « Astuce »), jamais bloquante. L'utilisateur peut
  ignorer.
- **La skill peut proposer un mauvais contexte** — Claude peut lire
  un projet et se tromper de conventions. → **Atténuation** :
  confirmation obligatoire avec diff. Ce que l'utilisateur voit et
  accepte est ce qu'il obtient.
- **Le body `configure.md` est notoirement fragile** — comme toutes
  les skills, une consigne trop lâche produit une sortie erratique.
  → **Atténuation** : le budget de lecture explicite + la séquence
  strictement décrite + le préservé/interdit typé.
- **`Bash(codev:*), Write, Edit` est presque `apply`** — la skill est
  puissante. → **Atténuation** : elle ne touche qu'un fichier
  (`_codev/config.yaml`) et le body l'énonce. Un test d'invariant
  vérifie que le body cite explicitement cette contrainte.

## Plan de migration

Aucune migration nécessaire. Un projet existant :

- Voit la 8ème skill apparaître au prochain `codev update`.
- Reçoit la nudge à son prochain `codev init` ou `codev status`
  **seulement** si son `_codev/config.yaml` est thin.

Un projet dont l'utilisateur a rempli sa config manuellement ne
verra jamais la nudge — l'expérience est parfaitement identique à
aujourd'hui.
