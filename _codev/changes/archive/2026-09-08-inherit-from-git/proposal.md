# Proposal : hériter d'un dépôt git distant

## Pourquoi

Depuis le début du projet, la promesse était : « les décisions et les
conventions se partagent entre projets d'une même organisation, sans qu'un
projet ait à cloner un dépôt à côté ». La moitié locale est déjà là — un
projet peut hériter d'un autre via `inherits: path:`. La moitié distante ne
l'est pas : `inherits: git:` émet un warning `inherit_git_unsupported` et
s'arrête là. Ce change livre le vrai transport, épinglé par SHA,
lecture-seule, aligné sur la décision
[0005](../../decisions/0005-sources-heritees-en-lecture-seule.md).

## Ce qui change

- **Sous-clé `git:` de `inherits`** — actuellement acceptée par la
  configuration mais bloquée par un warning ; devient fonctionnelle. Une
  entrée typique :

  ```yaml
  inherits:
    - git: git@github.com:mon-orga/codev-decisions.git
      ref: main
      subpath: shared/           # optionnel
  ```

- **Nouveau port `ProcessRunner`** dans `codev-engine`, avec une
  implémentation réelle qui exécute `git` via `std::process::Command` et
  une implémentation en mémoire pour les tests. C'est le quatrième port
  annoncé par l'architecture — le premier consommateur en est ce change.

- **Cache adressé par SHA** sous `~/.cache/codev/`
  (respect de `XDG_CACHE_HOME`) : un dépôt *bare* par URL dans
  `git/<hash-de-l-url>/`, le contenu extrait par SHA dans
  `content/<sha>/`. Lecture entièrement hors ligne dès que le SHA est en
  cache.

- **Fichier de verrouillage `_codev/codev.lock`** versionné avec le projet,
  au format TOML — comme la convention `Cargo.lock`. Chaque entrée `git:`
  y porte l'URL, le `ref` demandé, le `subpath`, le SHA résolu, et l'heure
  de résolution. Un `codev sources update` est **la seule commande qui
  déplace un pin**.

- **Résolution paresseuse à `config::resolve`** : quand un `inherits:
  git:` est déclaré, `resolve` cherche l'entrée correspondante dans le
  lock. Trouvée → le chemin résolu pointe vers le contenu du cache pour
  le SHA verrouillé. Absente → warning `git_source_unlocked` qui invite à
  lancer `codev sources update`. Rien n'est fetché à l'exécution ; c'est
  `sources update` qui touche au réseau, jamais autre chose.

- **Trois nouvelles commandes `codev sources`** :
  - `codev sources list [--json]` : liste toutes les sources déclarées
    avec leur état (locale résolue, git verrouillée, git non verrouillée) ;
  - `codev sources update [--json]` : résout chaque `git:` via
    `git ls-remote`, télécharge si nouveau SHA, affiche le diff des
    changements de SHA avant d'écrire le lock, écrit ;
  - `codev sources show <ref> [--json]` : détails d'une source (URL, ref,
    SHA verrouillé, chemin dans le cache, contenu hérité).

- **Garde-fous** — la décision 0005 les inscrit ; ils deviennent
  matériels :
  - le SHA est obligatoire, jamais lu depuis une branche flottante à
    l'exécution ;
  - seuls les fichiers `.md` et `.yaml` sont exposés aux consommateurs
    (décisions, main specs, config héritée) — aucun binaire, aucun hook,
    aucun script n'est jamais chargé depuis une source héritée.

## Capacités

### Nouvelles capacités

Aucune.

### Capacités modifiées

- `decisions` — les décisions héritées peuvent désormais venir d'un dépôt
  git en plus d'un dossier local. L'`origin` gagne une nouvelle forme
  `git:<url>`, exposée dans le contrat JSON. La logique d'index et
  d'injection dans `design` reste identique.

## Impact

- **Code** : nouveau port `ProcessRunner` (`codev-engine::ports`), nouveau
  module `codev-engine::sources` avec les fonctions de résolution et de
  cache, nouveau module `codev-engine::lockfile` pour la lecture/écriture
  du TOML, extension de `codev-engine::config` pour utiliser le lock,
  nouveau groupe `codev-cli::commands::sources` avec ses rendus humains et
  ses formes JSON versionnées.
- **Dépendances** : nouvelle dépendance `toml = "0.8"` pour le lock —
  cohérent avec la convention `<outil>.lock` (`Cargo.lock`, `poetry.lock`,
  `uv.lock`). Alternative écartée (garder YAML pour tout) dans le design.
- **Binaire externe** : `git` doit être présent sur le PATH pour
  `codev sources update`. Absent → `codev sources update` échoue avec un
  code stable `git_not_found` et un message explicite. Les autres
  commandes n'ont besoin ni de `git` ni du réseau.
- **Hors périmètre** :
  - **Support HTTP « tarball » codeload** (`https://codeload.github.com/…`) —
    utile pour un cas dégradé sans `git` installé, remonté si besoin.
  - **Extraction paresseuse par `git archive` sans clone complet** — l'implémentation MVP fait un
    `git fetch --depth 1 --filter=blob:none` puis un `git worktree add`
    dans le cache ; le passage à `git archive` peut arriver après retour
    d'expérience.
  - **Nettoyage automatique du cache** — le cache grossit à mesure que les
    SHAs changent ; une commande `codev sources gc` (ou équivalent) sera
    utile plus tard mais pas ici.
  - **Sources git en écriture** — décision 0005 : lecture seule, point.
  - **Authentification autre que celle de `git`** — pas de gestion de
    token PAT, pas de proxy HTTP à part. Ce que sait faire `git` sur ta
    machine, on le fait ; le reste, non.
  - **Fetch parallèle de plusieurs sources** — séquentiel dans ce change,
    parallélisable après si le besoin remonte.
