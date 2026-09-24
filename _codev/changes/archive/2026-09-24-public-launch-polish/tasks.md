# Tâches

## 1. Crédit OpenSpec

- [x] 1.1 Éditer `README.md` — ajouter une ligne de crédit après la
      description courte, avant les deux tableaux (« Les deux
      moitiés ») :
      ```
      > Inspiré par [OpenSpec](https://github.com/tobyhs/openspec) —
      > reconstruit en Rust avec ses propres choix. Voir la section
      > « Origines » de la documentation pour le détail.
      ```
- [x] 1.2 Éditer `docs/codev.md`, section 1 (« Pourquoi codev »),
      ajouter une nouvelle sous-section **« ### Origines »** en fin
      de section 1 :
      - Une phrase sur l'inspiration OpenSpec.
      - Ce qui a été **repris à l'idée** : cycle propose/apply/archive,
        deltas de spec (ADDED/MODIFIED/REMOVED/RENAMED), capacités,
        ADR de premier ordre.
      - Ce qui a été **écarté ou fait différemment** : Rust plutôt
        que TypeScript, `_codev/` visible plutôt que caché, cœur
        pur + coquille impérative plutôt qu'un binaire monolithique.
      - Ce qui est **propre à codev** : sceau K3, dérives K6,
        promotion K7, MCP configurable côté projet, `codev docs`,
        distribution précompilée multi-OS.

## 2. README enrichi

- [x] 2.1 Ajouter les 4 badges en tête de README, juste sous le
      titre `# codev` :
      ```markdown
      [![Release](https://github.com/mairistem/codev/actions/workflows/release.yml/badge.svg)](https://github.com/mairistem/codev/actions/workflows/release.yml)
      [![Latest release](https://img.shields.io/github/v/release/mairistem/codev)](https://github.com/mairistem/codev/releases/latest)
      [![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
      [![Platforms](https://img.shields.io/badge/platforms-macOS%20%7C%20Linux%20%7C%20Windows-lightgrey)](docs/codev.md)
      ```
- [x] 2.2 Ajouter une **table des matières** sous la description :
      Démarrage, Le modèle, Le cycle, Concepts, Documentation,
      Contribuer, Licence.
- [x] 2.3 Ajouter un exemple visuel **ASCII art du cycle** en
      section « Le cycle » (si absente, la créer) :
      ```
      ┌──────────┐   ┌──────────┐   ┌──────────┐   ┌──────────┐
      │ propose  │→→ │  apply   │→→ │   sync   │→→ │ archive  │
      │ planifie │   │ code     │   │ merge    │   │ classe   │
      └──────────┘   └──────────┘   └──────────┘   └──────────┘
      ```
- [x] 2.4 Ajouter en fin de README un lien clair vers
      `docs/codev.md` (« Manuel complet ») et le one-liner
      `codev docs`.
- [x] 2.5 Ajouter les sections **Contribuer** (« voir
      [CONTRIBUTING.md]») et **Licence** (« MIT — voir
      [LICENSE] ») en pied.

## 3. CONTRIBUTING.md

- [x] 3.1 Créer `CONTRIBUTING.md` à la racine, en français, avec
      les sections :
      - **Merci** — une phrase.
      - **Prérequis** — Rust stable + cargo, git, `gh` optionnel.
      - **Le cycle** — expliquer que **contribuer à codev, c'est
        utiliser codev**. Étapes :
        1. Fork + clone.
        2. `cargo install --path crates/codev-cli` pour construire.
        3. `codev init` sur le fork si pas déjà fait.
        4. `/codev-propose <mon-idée>` (ou `codev new change`).
        5. Implémenter, `cargo test --workspace`,
           `cargo clippy --workspace --all-targets`,
           `codev validate --strict`.
        6. `codev archive <mon-change>`.
        7. Push, PR.
      - **Style** — commits en français ou anglais, ton neutre,
        Co-Authored-By pour les IA si applicable.
      - **Release** — rappel : bump `Cargo.toml`, tag `vX.Y.Z`,
        `git push --tags`, éditer `CHANGELOG.md` avec l'entrée V.
      - **Signalement de faille** — renvoi vers `SECURITY.md`.
      - **Code de conduite** — renvoi vers `CODE_OF_CONDUCT.md`.

## 4. CHANGELOG.md

- [x] 4.1 Créer `CHANGELOG.md` à la racine, format
      **Keep-a-Changelog** :
      ```markdown
      # Changelog

      Toutes les évolutions notables de codev sont listées ici.
      Format : [Keep a Changelog](https://keepachangelog.com/fr/1.1.0/),
      versionnage SemVer.

      ## [0.2.0] — 2026-09-24

      ### Added
      - Cible Windows x86_64-pc-windows-msvc dans les binaires
        précompilés + script `install.ps1`.
      - Documentation embarquée : `codev docs` ouvre un manuel HTML
        autonome dans le navigateur.
      - Complétions shell : `codev completions <shell>` (bash, zsh,
        fish, powershell, elvish).
      - `codev docs --write PATH` pour diffusion ciblée.

      Notes complètes :
      https://github.com/mairistem/codev/releases/tag/v0.2.0

      ## [0.1.1] — 2026-09-23

      ### Fixed
      - CI Release : cross-compile `x86_64-apple-darwin` depuis
        `macos-14` (les runners `macos-13` gratuits ne sont plus
        fiables).

      Notes complètes :
      https://github.com/mairistem/codev/releases/tag/v0.1.1

      ## [0.1.0] — 2026-09-23

      ### Added
      - Import initial : cycle propose → apply → sync → archive,
        7 skills Claude Code, gouvernance décisions (K3/K6/K7),
        `codev validate --strict`, intégration MCP configurable.

      Notes complètes :
      https://github.com/mairistem/codev/releases/tag/v0.1.0
      ```

## 5. CODE_OF_CONDUCT.md

- [x] 5.1 Créer `CODE_OF_CONDUCT.md` à la racine — reprendre
      **Contributor Covenant 2.1** texte officiel, en **français**
      (traduction officielle disponible sur le site :
      contributor-covenant.org/version/2/1/code_of_conduct/).
- [x] 5.2 Ajouter le contact — soit un email si Ludovic en a un
      dédié, soit une note « ouvrir un GitHub Security Advisory
      privé sur le repo ». À trancher au moment de l'apply.

## 6. SECURITY.md

- [x] 6.1 Créer `SECURITY.md` à la racine :
      ```markdown
      # Politique de sécurité

      ## Versions supportées

      | Version | Support |
      |---------|---------|
      | 0.x     | Toutes les versions récentes tant que codev est en 0.x |

      ## Signaler une faille

      Merci de **ne pas ouvrir d'issue publique** pour signaler une
      faille de sécurité.

      Utilise le mécanisme privé de GitHub :

      1. Va sur https://github.com/mairistem/codev/security/advisories
      2. Clique « Report a vulnerability ».
      3. Décris la faille, un scénario reproductible, et l'impact
         attendu.

      Délai de première réponse cible : **72 heures**. Si tu ne
      reçois pas de réponse, relance via une issue publique
      demandant « Re: vulnerability report » sans donner le
      détail.
      ```

## 7. Templates GitHub

- [x] 7.1 Créer `.github/ISSUE_TEMPLATE/bug_report.md` :
      ```markdown
      ---
      name: Rapport de bug
      about: Signaler un comportement incorrect de codev
      title: 'bug: '
      labels: bug
      ---

      **Description**
      <!-- Une phrase claire du problème observé. -->

      **Reproduction**
      1. …
      2. …
      3. …

      **Comportement attendu**
      <!-- Ce à quoi tu t'attendais. -->

      **Comportement observé**
      <!-- Ce qui se passe. Colle les messages d'erreur exacts. -->

      **Environnement**
      - `codev --version` :
      - OS + version :
      - Shell :
      ```
- [x] 7.2 Créer `.github/ISSUE_TEMPLATE/feature_request.md` :
      ```markdown
      ---
      name: Demande de fonctionnalité
      about: Proposer une évolution
      title: 'feat: '
      labels: enhancement
      ---

      **Problème que ça résout**
      <!-- Décris le manque actuel, avec un cas concret. -->

      **Solution envisagée**
      <!-- Ta piste, sans engagement d'implémentation. -->

      **Alternatives considérées**
      <!-- Ce qui a été écarté et pourquoi. -->

      **Périmètre**
      <!-- Ce qui EST dans la demande, ce qui N'EST PAS. -->
      ```
- [x] 7.3 Créer `.github/ISSUE_TEMPLATE/config.yml` :
      ```yaml
      blank_issues_enabled: false
      contact_links:
        - name: Questions et discussions
          url: https://github.com/mairistem/codev/discussions
          about: Pour tout ce qui n'est pas un bug ou une demande de fonctionnalité.
        - name: Guide de contribution
          url: https://github.com/mairistem/codev/blob/main/CONTRIBUTING.md
          about: Comment proposer un change codev.
      ```
- [x] 7.4 Créer `.github/PULL_REQUEST_TEMPLATE.md` :
      ```markdown
      ## Change codev associé

      <!-- Nom du dossier sous `_codev/changes/` ou lien vers son
           archive. Une PR sans change codev est acceptable pour un
           fix trivial (typo, correction de doc). -->

      ## Résumé

      <!-- Ce que la PR apporte, en une ou deux phrases. -->

      ## Vérifications

      - [ ] `cargo test --workspace` vert.
      - [ ] `cargo clippy --workspace --all-targets` sans avertissement.
      - [ ] `codev validate --strict` vert.
      - [ ] Documentation à jour (`docs/codev.md`, README si besoin).

      ## Points d'attention pour le reviewer

      <!-- Zones sensibles, choix discutables, tests manquants
           connus. Vaut mieux les nommer ici que les laisser en
           surprise. -->
      ```

## 8. Vérifications finales

- [x] 8.0 Réparer la dette pré-existante `distribution` : créer le
      delta `MODIFIED` sur l'exigence « La documentation cite les
      trois voies d'installation » (héritée du change Windows) —
      recopier le bloc entier depuis la spec principale, terminer
      la phrase tronquée, ajouter deux scénarios `#### Scenario:`
      qui vérifient l'ordre des voies dans `docs/codev.md` et la
      présence des deux one-liners dans `README.md`. Le delta vit à
      `_codev/changes/public-launch-polish/specs/distribution/spec.md`.
- [x] 8.1 `codev validate --strict` — vert sur le change
      (`codev validate public-launch-polish --strict`) ; l'exigence
      principale devient également strict-verte après `codev archive`,
      quand le delta est fusionné.
- [x] 8.2 `cargo test --workspace` reste vert (rien de Rust n'a
      changé, mais on vérifie).
- [x] 8.3 `codev docs` s'ouvre et rend correctement la nouvelle
      sous-section « Origines ».
- [ ] 8.4 Ouvrir `README.md` sur GitHub et vérifier :
      - Les 4 badges apparaissent et pointent correctement.
      - La TOC est cliquable.
      - Le lien vers `docs/codev.md` marche.
      - L'ASCII art rend correctement (bloc de code).

## 9. Livraison

- [ ] 9.1 Après merge, bump `Cargo.toml` à `0.2.1` (patch — pas
      d'API changée), `git tag v0.2.1`, `git push origin main
      v0.2.1`. Le workflow tourne, la release apparaît.
- [ ] 9.2 Ajouter l'entrée `[0.2.1]` dans `CHANGELOG.md` (dans un
      commit après le tag ou en tête juste avant).
