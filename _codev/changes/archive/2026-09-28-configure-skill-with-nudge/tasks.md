# Tâches

## 1. Fonction `is_config_thin`

- [x] 1.1 Créer `codev-core::config::is_config_thin(context: Option<&str>, rules_empty: bool) -> bool` : retourne vrai si `context` est absent ou fait < 200 caractères ET `rules_empty` est vrai. Fonction pure, dans `crates/codev-core/src/config/mod.rs`. Test unitaire : trois cas — contexte thin + rules vides → true, contexte long → false, rules non vides → false.

## 2. Skill body — `configure`

- [x] 2.1 Créer `assets/workflows/configure.md` — body de la skill. Sections attendues :
      - Intro : « Enrichir `_codev/config.yaml` en analysant le projet, sans jamais toucher aux workflows/MCPs/schéma. »
      - **Entrée** — aucune ; refuse si `_codev/config.yaml` absent (renvoie vers `codev init`).
      - **Étapes** :
        1. Lire le `_codev/config.yaml` existant.
        2. Explorer en respectant le **budget** : README (complet), CONTRIBUTING (si présent), `docs/**/*.md` (3-5 fichiers), 8 fichiers source max priorisés par récence git, `ls _codev/` + `ls <src-root>/` — un niveau seul.
        3. Rédiger un patch : nouveau `context:` (2-5 lignes) + `rules:` par artefact (1-2 chacune).
        4. **Afficher le diff** ligne par ligne, sans écrire.
        5. Demander « Applique ? [oui/non] » — écrire seulement si oui.
      - **Garde-fous** : préserve `schema`, `workflows`, `mcp`, `inherits`, commentaires ; ne touche que `context` et `rules` ; refuse si config absente.

## 3. Catalogue — ajouter `configure`

- [x] 3.1 Ajouter l'entrée `configure` dans `CATALOG` de `crates/codev-agents/src/workflows.rs`. Description courte, `allowed_tools = "Bash(codev:*), Read, Write, Edit, Glob, Grep"` (pas de `Bash` général), `body = include_str!("../../../assets/workflows/configure.md")`.
- [x] 3.2 `DEFAULT_WORKFLOWS` passe de 7 à 8 en ajoutant `"configure"` en fin de liste. Retirer aussi l'ancien commentaire qui listait les 7 workflows.
- [x] 3.3 Réécrire le test `sans_demande_installe_le_catalogue_par_defaut` avec les 8 workflows. Ajouter le test « restriction opt-out » reste vert.
- [x] 3.4 Ajouter un test d'invariant qui vérifie que `configure` a `Write` et `Edit` mais **pas** `Bash` général — pour verrouiller la règle « seule `apply` a le Bash général ».

## 4. Mettre à jour le body `onboard`

- [x] 4.1 Modifier `assets/workflows/onboard.md` — ajouter une nouvelle branche dans la table de décision, placée **avant** la branche « aucun change actif » : si `is_config_thin` est vrai (le body doit expliquer comment le calculer via `Read` sur le YAML), recommander `/codev-configure` en premier avec la phrase « Claude enrichira ta config à partir du projet », puis `/codev-propose <idée>` en second.
- [x] 4.2 Mettre à jour le test `onboard_est_dans_le_catalogue_et_a_les_bons_outils` de `codev-agents::workflows` si le contenu du body est testé (pour couvrir la nouvelle mention `configure`).

## 5. Nudge dans `codev init`

- [x] 5.1 Dans `crates/codev-cli/src/render.rs`, sortie de `setup` (render de `SetupOutcome`) : après l'appel qui écrit « Redémarre Claude Code… », lire le `_codev/config.yaml` du projet, calculer `is_config_thin` via les champs `context` et `rules` de `ProjectConfig`. Si thin, remplacer la ligne finale par la call-to-action « → Prochaine étape recommandée : /codev-configure ». Sinon garder la ligne actuelle. *(Champ `config_thin` ajouté à `SetupOutcome`, calculé dans `install_skills` sur la config résolue.)*
- [x] 5.2 Tester la branche via un test unitaire dans `codev-cli` : injecter un `SetupOutcome` + un config thin → sortie contient `/codev-configure`. Un config non-thin → sortie ne le contient pas.
- [x] 5.3 Confirmer que la sortie JSON (`--json`) ne change pas — pas de champ ajouté au contrat. *(Le champ `config_thin` est côté `SetupOutcome`, pas propagé à `SetupV1` dans `main.rs::setup_v1`.)*

## 6. Nudge dans `codev status`

- [x] 6.1 Dans `crates/codev-cli/src/render.rs`, sortie « aucun change actif » : ajouter une troisième ligne conditionnelle. Si `is_config_thin` sur la config résolue → « Astuce : config peu remplie — /codev-configure peut l'enrichir. ». Sinon rien. *(Implémenté dans `main.rs` après le `fail()` du statut, uniquement en sortie humaine.)*
- [x] 6.2 Vérifier via un test que le JSON de `status` ne change pas. *(La nudge sort via `eprintln!` uniquement dans la branche `!json` du dispatch de `Command::Status` ; le contrat `StatusV1` et `status_shape()` restent identiques.)*

## 7. Documentation

- [x] 7.1 `docs/codev.md` §4 (Les skills Claude Code) — ajouter une ligne dans le tableau pour `codev-configure` (rôle : « Enrichir _codev/config.yaml en analysant le projet », `allowed-tools` : `Bash(codev:*), Read, Write, Edit, Glob, Grep`, défaut : ✓).
- [x] 7.2 `docs/codev.md` §2 (Installation, sous-section « L'expérience `codev init` ») — ajouter une phrase à la fin qui décrit la nudge : « Si la config générée est thin, la sortie recommande explicitement `/codev-configure` comme prochaine étape — l'utilisateur peut suivre ou ignorer. »
- [x] 7.3 `CHANGELOG.md` — nouvelle entrée `[Unreleased]` en tête, section `### Added` : « Skill `/codev-configure` qui enrichit `_codev/config.yaml` en analysant le projet (README, CONTRIBUTING, docs, échantillon de code), avec diff + confirmation obligatoire avant écriture. Nudge automatique dans `codev init` et `codev status` quand la config est thin. Le catalogue par défaut passe à 8 workflows. »

## 8. Vérifications finales

- [x] 8.1 `cargo test --workspace` vert.
- [x] 8.2 `cargo clippy --workspace --all-targets` sans avertissement.
- [x] 8.3 `codev validate --strict` vert.
- [x] 8.4 Test manuel : dans un scratchpad, `cargo init --lib` puis `codev init --yes` — la sortie humaine doit contenir `/codev-configure` (config thin par construction). *(Vérifié — la call-to-action « → Prochaine étape recommandée : /codev-configure » s'affiche bien.)*
- [x] 8.5 Test manuel : `codev status` sur le même projet vide doit afficher la ligne « Astuce : config peu remplie — /codev-configure peut l'enrichir. » *(Vérifié — la ligne s'affiche bien sur stderr, sortie JSON inchangée.)*

## 9. Livraison

- [ ] 9.1 Bump `Cargo.toml` à `0.3.1` (patch — ajout de skill, comportement défaut inchangé sur un projet dont la config est non-thin). Tag `v0.3.1`, `git push origin main --tags`.
- [ ] 9.2 Dater l'entrée `[0.3.1] — <date>` dans `CHANGELOG.md`.
