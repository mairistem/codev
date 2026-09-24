# Proposal : tutoriel end-to-end + diagrammes Mermaid

## Pourquoi

`docs/codev.md` fait 693 lignes en 10 sections — c'est déjà solide, mais
un visiteur qui découvre codev y trouve un **manuel de référence**, pas
un **chemin guidé** de sa première évolution. Il lit *ce que* fait chaque
commande, sans jamais suivre *un cas concret* du début à la fin.

Par ailleurs, plusieurs relations structurantes de codev sont
aujourd'hui décrites en prose alors qu'un schéma les rendrait
immédiatement lisibles :

- la **machine à états d'un change** (propose → apply → sync/archive)
  est décrite par un ASCII art en section 3, mais on ne voit pas les
  transitions conditionnelles (skip_specs, sync sans archive) ;
- le **graphe de crates** — l'application concrète de l'ADR 0002 (règle
  de dépendance) — n'a aucun schéma alors qu'il matérialise la
  frontière cœur pur / coquille impérative ;
- le **cycle de vie d'un delta** (proposal → apply → sync → merge dans
  la spec principale) traverse plusieurs sections sans jamais être
  visualisé.

GitHub rend Mermaid nativement dans les fichiers `.md`. `pulldown-cmark`
(la lib qu'utilise `codev docs`) ne rend pas Mermaid — mais un bloc
Mermaid dégrade proprement en bloc de code, ce qui reste lisible dans
le HTML embarqué.

Ce change ne touche **aucun code Rust** ni comportement observable de
la CLI : `skip_specs: true`.

## Ce qui change

**Fichier modifié** : `docs/codev.md`.

**Nouvelle section 3.5 — « Ta première évolution, en cinq minutes »** —
tutoriel pas à pas, inséré entre la section 3 (Le cycle) et la
section 4 (Les 7 skills). Il simule un cas concret : « ajouter une
option `--json` à `codev list` ». Chaque étape porte :

- la commande exacte (copiable) ;
- la sortie attendue (bloc `text`) ;
- une phrase d'explication.

Étapes du tutoriel :

1. `codev init` sur un projet vierge (facultatif si déjà fait).
2. `/codev-propose add-list-json` dans Claude Code — ou l'équivalent
   `codev new change add-list-json`.
3. Édition manuelle des artefacts si sans Claude Code, ou pilotée par
   la skill.
4. `codev status --change add-list-json` pour confirmer que la
   planification est complète.
5. `/codev-apply add-list-json` — implémentation guidée.
6. `codev validate add-list-json` — dernière vérification.
7. `/codev-archive add-list-json` — clôture, avec la sortie
   `ArchiveReportV1`.

Le tutoriel MUST pointer explicitement vers les sections de
référence pour les détails (« pour le format complet des artefacts,
voir §3.1 »), pour éviter la duplication.

**Trois diagrammes Mermaid** — insérés dans les sections existantes,
sans nouvelle section :

- Dans **§3 (Le cycle)**, juste avant l'ASCII art : un `stateDiagram-v2`
  qui montre la machine à états d'un change (états : *proposed*,
  *applied*, *synced*, *archived*), les transitions nommées, et les
  gardes (`skip_specs`, `validate --strict`).
- Dans **§5 (Concepts)**, sous-section « Architecture » (à créer si
  absente) : un `graph LR` du graphe de crates (`codev-core` →
  `codev-engine` → `codev-cli`, avec `codev-agents` en aparté),
  légendé « la règle de dépendance ADR 0002 rendue visible ».
- Dans **§5 (Concepts)**, sous la sous-section « Deltas » : un
  `sequenceDiagram` du cycle de vie d'un delta de la naissance
  (édition de `specs/<capa>/spec.md` dans le change) à la fusion
  (`codev archive` → contenu écrit dans `_codev/specs/<capa>/spec.md`).

## Capacités

### Nouvelles capacités

Aucune.

### Capacités modifiées

Aucune.

### Capacités retirées

Aucune.

*(Change marqué `skip_specs: true` — pure évolution du contenu de
`docs/codev.md`, aucun comportement observable du binaire ne change.
La spec `docs` porte sur ce que fait la commande `codev docs`, pas sur
ce que contient le markdown ; les deux sont indépendants.)*

## Impact

- **Code** : rien dans les crates Rust ni dans les scripts. Uniquement
  du markdown dans `docs/codev.md`.
- **Rendu `codev docs`** : le HTML embarqué grandit d'environ
  200-300 lignes ; les blocs Mermaid apparaîtront comme des blocs de
  code (dégradation propre), sans erreur de rendu.
- **README** : inchangé — l'ASCII du cycle du README reste, il joue le
  rôle de « teaser » et le tutoriel complet vit dans la doc.
- **Tests** : pas de test automatique — la doc n'a pas de suite. Une
  vérification manuelle du rendu Mermaid sur GitHub + une lecture du
  tutoriel suffisent.
- **Hors périmètre** :
  - **Recettes/cookbook** (« comment scinder une capacité », etc.) —
    reporté au cycle suivant (`docs-cookbook`).
  - **Référence du contrat JSON v1** — reporté à un cycle dédié.
  - **Traduction EN** — reportée.
  - **Diagrammes SVG statiques** — Mermaid suffit tant que GitHub le
    rend.
