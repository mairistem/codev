# Tâches

## 1. Diagramme — machine à états d'un change (§3)

- [x] 1.1 Éditer `docs/codev.md` : dans la **section 3 (Le cycle)**,
      juste avant l'ASCII art existant, insérer un bloc
      ` ```mermaid` de type `stateDiagram-v2` couvrant :
      - états : `proposed`, `applied`, `synced`, `archived` ;
      - transitions : `propose` (initial → proposed), `apply`
        (proposed → applied), `sync` (applied → synced), `archive`
        (applied → archived, synced → archived) ;
      - notes sur les gardes clés : `skip_specs`, `validate --strict`,
        « planification complète » avant apply.
- [x] 1.2 Ajouter une phrase de légende sous le diagramme :
      « `sync` est une variante d'`archive` qui ne déplace pas le
      dossier — utile pour rendre une capacité nouvelle disponible
      sans classer le change. »

## 2. Tutoriel — §3.5 « Ta première évolution, en cinq minutes »

- [x] 2.1 Ajouter en fin de section 3 (juste avant le séparateur
      `---` qui précède la section 4) une nouvelle sous-section :
      ```markdown
      ### 3.5. Ta première évolution, en cinq minutes
      ```
- [x] 2.2 Introduction (2-3 phrases) — poser le cas concret : « on
      va ajouter une option `--json` à `codev list` ». Préciser :
      cas volontairement simple, sert de fil rouge pour voir tout
      le cycle en pratique. Deux voies proposées à chaque étape :
      Claude Code (skill) ou CLI pur.
- [x] 2.3 Étape 1 — **créer le change**. Voie skill :
      `/codev-propose add-list-json`. Voie CLI :
      `codev new change add-list-json --goal "Ajouter --json à codev list"`.
      Sortie attendue (bloc `text`) :
      ```
      Change « add-list-json » créé
        Emplacement  <chemin>/_codev/changes/add-list-json
        Schéma       spec-driven
      ```
- [x] 2.4 Étape 2 — **rédiger les artefacts**. Rediriger vers §3.1
      pour le format complet. Montrer un mini-exemple de
      `proposal.md` (5 lignes) et le squelette du delta
      `specs/cli-list/spec.md`.
- [x] 2.5 Étape 3 — **statuts**. `codev status --change add-list-json`,
      sortie type :
      ```
        [x] proposal
        [x] specs
        [x] design
        [x] tasks
      Planification : 4/4 artefacts
      ```
- [x] 2.6 Étape 4 — **implémenter**. Voie skill : `/codev-apply
      add-list-json` (Claude Code lit `tasks.md`, coche à mesure).
      Voie manuelle : éditer le code, cocher les cases à la main.
- [x] 2.7 Étape 5 — **valider**. `codev validate add-list-json`,
      sortie type ✓ aucun défaut.
- [x] 2.8 Étape 6 — **archiver**. `/codev-archive add-list-json` ou
      `codev archive --change add-list-json`. Coller un extrait
      simplifié du `ArchiveReportV1` en sortie.
- [x] 2.9 Clôture : une phrase « ce que tu viens de faire peut se
      répéter à l'identique pour n'importe quelle évolution, du fix
      d'une ligne au refactor d'une capacité entière », plus un lien
      vers §5 (Concepts) et §6 (CLI) pour aller plus loin.

## 3. Diagramme — graphe de crates (§5)

- [x] 3.1 Ouvrir `docs/codev.md` §5 (Concepts). Vérifier qu'il y a
      bien un sous-titre « Architecture » (ou équivalent) ; sinon,
      en ajouter un (`### Architecture`).
- [x] 3.2 Insérer un bloc ` ```mermaid` `graph LR` couvrant :
      - noeuds : `codev-core` (marqué « pur, aucune I/O »),
        `codev-engine` (marqué « effets via ports »), `codev-cli`
        (marqué « coquille impérative »), `codev-agents` (marqué
        « cible Claude Code ») ;
      - arêtes : `codev-cli --> codev-engine`,
        `codev-engine --> codev-core`, `codev-cli --> codev-agents`,
        `codev-agents --> codev-core` ;
      - un style qui distingue `codev-core` (le cœur) des trois
        autres.
- [x] 3.3 Légende sous le diagramme (2-3 lignes) : « les flèches
      pointent vers les dépendances. Le graphe est acyclique et
      Cargo le vérifie à la compilation — c'est notre application
      concrète de la règle de dépendance (voir ADR 0002). »

## 4. Diagramme — cycle de vie d'un delta (§5)

- [x] 4.1 Trouver dans §5 la sous-section « Deltas » (existante) ou
      la sous-partie qui décrit `ADDED/MODIFIED/REMOVED/RENAMED`.
- [x] 4.2 Insérer un bloc ` ```mermaid` `sequenceDiagram` couvrant :
      - acteurs : `Agent` (ou `Toi`), `codev-cli`, `spec du change`,
        `spec principale` ;
      - séquence : Agent → cli (`propose`), cli → spec-du-change
        (crée `specs/<capa>/spec.md`) ; plus tard Agent → cli
        (`archive`), cli → spec-principale (fusion), cli → Agent
        (rapport `ArchiveReportV1`).
- [x] 4.3 Légende sous le diagramme (2-3 lignes) : « la spec
      principale n'est jamais éditée directement — elle reçoit ses
      modifications par fusion de deltas au moment de `sync` ou
      `archive`. »

## 5. Vérifications finales

- [x] 5.1 `codev validate --strict` reste vert.
- [x] 5.2 `cargo test --workspace` reste vert (rien de Rust n'a
      changé, mais on vérifie).
- [x] 5.3 Recompiler et lancer `codev docs --write /tmp/manuel.html`,
      grep :
      - `Ta première évolution` (le tutoriel est bien embarqué) ;
      - `stateDiagram-v2` (le source Mermaid apparaît dans le HTML) ;
      - `graph LR` (idem).
- [ ] 5.4 Push et ouvrir `docs/codev.md` sur github.com — vérifier
      visuellement que les trois diagrammes Mermaid rendent, et que
      le tutoriel est correctement mis en forme.

## 6. Livraison

- [ ] 6.1 Bump `Cargo.toml` à `0.2.2` (patch — contenu de doc). Tag
      `v0.2.2`, `git push origin main --tags`. Le workflow release
      publie automatiquement.
- [ ] 6.2 Ajouter l'entrée `[0.2.2] — 2026-09-24` en tête de
      `CHANGELOG.md`.
