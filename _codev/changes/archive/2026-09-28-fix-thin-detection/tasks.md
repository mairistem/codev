# Tâches

## 1. Simplification de `is_config_thin`

- [x] 1.1 Dans `crates/codev-core/src/config/mod.rs`, changer la signature de `is_config_thin` : passer de `is_config_thin(context: Option<&str>, rules_empty: bool) -> bool` à `is_config_thin(rules_empty: bool) -> bool` — retourner directement `rules_empty`. Retirer la doc du seuil 200.
- [x] 1.2 Réécrire les tests unitaires : supprimer les 3 tests basés sur le seuil, les remplacer par deux tests binaires (`is_config_thin_vrai_quand_rules_vides`, `is_config_thin_faux_quand_rules_presentes`). Vérifié par `cargo test -p codev-core is_config_thin` vert.

## 2. Adaptation des call sites

- [x] 2.1 Dans `crates/codev-cli/src/commands.rs`, `install_skills` : remplacer le calcul avec `context_total` par un simple `let config_thin = codev_core::config::is_config_thin(config.rules.is_empty());`. Retirer les lignes qui construisent `context_total`.
- [x] 2.2 Dans `crates/codev-cli/src/main.rs`, `config_is_thin` : simplifier pareil — plus besoin de reconstruire `context_total`, un seul appel `is_config_thin(cfg.rules.is_empty())`. Retirer les lignes intermédiaires.
- [x] 2.3 Vérifier que les tests de `render::setup` (test unitaire sur `config_thin: true/false`) restent verts sans modification, puisque le champ `config_thin: bool` sur `SetupOutcome` n'a pas changé de type.

## 3. Mise à jour du body `onboard`

- [x] 3.1 Dans `assets/workflows/onboard.md`, section « Recommander la prochaine action » : réécrire la ligne du tableau et la sous-section explicative pour ne parler que de « `rules:` vides ou absentes ». Retirer toute mention de « 200 caractères » ou de la longueur du contexte.
- [x] 3.2 Ajuster le test `onboard_est_dans_le_catalogue_et_a_les_bons_outils` de `codev-agents::workflows` : l'assertion `body.contains("thin") || body.contains("200")` doit rester vraie (le body cite toujours « thin »). Ajouter une assertion complémentaire que le body ne cite plus « 200 caractères ».

## 4. Documentation

- [x] 4.1 `docs/codev.md` §2 (sous-section « L'expérience `codev init` ») : la phrase « Si la config générée est thin (contexte court, aucune `rules:`) » devient « Si la config générée n'a pas encore de `rules:` ». Ajuster.
- [x] 4.2 `CHANGELOG.md` : nouvelle entrée `[Unreleased]` en tête, section `### Fixed` : « Détection `is_config_thin` : ne regarde plus la longueur du `context:`, base la décision uniquement sur `rules.is_empty()`. Un projet à stack riche (TypeScript, Java Maven, etc.) recevait un contexte auto-détecté long qui inhibait la nudge, alors qu'aucune règle n'avait été rédigée. »

## 5. Vérifications finales

- [x] 5.1 `cargo test --workspace` vert (les tests adaptés + les tests existants).
- [x] 5.2 `cargo clippy --workspace --all-targets` sans avertissement.
- [x] 5.3 `codev validate --strict` vert.
- [x] 5.4 Test manuel : recompiler codev, relancer sur un projet TypeScript ou avec Cargo à multiples dépendances. La nudge `/codev-configure` DOIT s'afficher dans la sortie de `codev init` et de `codev status`, alors qu'auparavant elle était absente. *(Vérifié sur `~/Sources/mira` — les deux nudges s'affichent correctement, alors qu'ils étaient absents en v0.3.1.)*

## 6. Livraison

- [ ] 6.1 Bump `Cargo.toml` à `0.3.2` (patch — fix pur). Tag `v0.3.2`, `git push origin main --tags`.
- [ ] 6.2 Dater l'entrée `[0.3.2] — <date>` dans `CHANGELOG.md`.
