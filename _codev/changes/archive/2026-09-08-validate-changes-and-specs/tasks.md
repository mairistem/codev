# Tâches

## 1. Fondations dans `codev-core`

- [x] 1.1 Créer `codev-core::validate` (module `validate/mod.rs`,
      `validate/rules.rs`), exposer `pub trait Rule` avec `code()`,
      `check_spec()`, `check_delta()` (défauts vides), plus
      `pub static RULES: &[&dyn Rule]`. Vérifié par
      `cargo build -p codev-core` et un test compilant
      `validate::registry_liste_au_moins_une_regle`.
- [x] 1.2 Test d'invariant `validate::codes_de_findings_sont_uniques` :
      concatène tous les codes émis par le parseur (constants dans les
      modules parser) et par les règles, vérifie qu'ils sont deux à deux
      distincts. Sert de garde-fou contre un doublon involontaire.

## 2. Règles structurelles supplémentaires (cœur pur)

- [x] 2.1 Règle `RequirementNoShall` : parcourt les exigences d'une `Spec`
      et de chaque section `Added`/`Modified` d'un `Delta`, émet
      `requirement_no_shall` si la description ne contient ni `SHALL` ni
      `MUST` (majuscules exactes). Vérifié par
      `validate::requirement_sans_shall_est_signalee` (scénario
      `Exigence sans SHALL ni MUST`).
- [x] 2.2 Règle `RequirementNoScenario` : émet `requirement_no_scenario` si
      `scenarios` est vide. Vérifié par
      `validate::requirement_sans_scenario_est_signalee` (scénario
      `Exigence sans aucun scénario`).
- [x] 2.3 Règle `SpecNoRequirement` : sur une `Spec`, émet
      `spec_no_requirement` si `requirements` est vide. Vérifié par
      `validate::spec_sans_requirement_est_signalee` (scénario
      `Spec principale sans exigence`).

## 3. Règles de cohérence entre sections (cœur pur)

- [x] 3.1 Règle `CrossSectionConflict` : indexe les noms d'exigences par
      section (Added/Modified/Removed), émet `cross_section_conflict` pour
      chaque nom présent dans deux sections, avec la liste des sections en
      cause et leurs lignes. Vérifié par
      `validate::exigence_dans_added_et_modified_est_signalee` (scénario
      `Exigence présente dans ADDED et MODIFIED`).
- [x] 3.2 Règle `RenameTargetCollision` : si un `RENAMED.TO` existe déjà
      comme `ADDED`, émet `rename_target_collision`. Vérifié par
      `validate::rename_to_qui_collide_avec_added_est_signale` (scénario
      `RENAMED.TO collide avec un ADDED de même nom`).
- [x] 3.3 Règle `ModifiedUsesOldName` : si un `MODIFIED` référence un
      `RENAMED.FROM`, émet `modified_uses_old_name`. Vérifié par
      `validate::modified_reference_ancien_nom_renamed_est_signale`
      (scénario `MODIFIED référence l'ancien nom d'un RENAMED`).

## 4. Règle zéro-delta et conflit skip_specs (côté engine)

- [x] 4.1 Étendre `codev-engine::validate` avec `check_change_metadata` qui
      compare la présence de deltas au marqueur `skip_specs`. Émet
      `zero_delta_without_marker` si aucun delta et marqueur absent, émet
      `skip_specs_conflict` si marqueur présent et deltas existent. Vérifié
      par `engine_validate::zero_delta_sans_marqueur_echoue` et
      `engine_validate::skip_specs_avec_delta_est_un_conflit` (scénarios
      homonymes de la spec).

## 5. Orchestration côté engine

- [x] 5.1 Type `LocatedFinding { path: PathBuf, kind: ItemKind, ..Finding }`
      dans `codev-engine::validate`, avec `From<(&Finding, path, kind)>`.
      Vérifié par `engine_validate::located_conserve_code_line_severite`.
- [x] 5.2 `validate_change(fs, layout, change) -> ItemReport` : ouvre chaque
      delta du change, appelle `parse_delta`, applique les règles
      `check_delta`, produit les `LocatedFinding` groupés par fichier. Fait
      aussi tourner `check_change_metadata` (tâche 4.1). Vérifié par
      `engine_validate::rapport_change_couvre_tous_les_fichiers_de_delta`.
- [x] 5.3 `validate_spec(fs, layout, capability) -> ItemReport` : ouvre la
      spec principale, appelle `parse_spec`, applique les règles `check_spec`.
      Vérifié par `engine_validate::rapport_spec_expose_les_findings_du_parseur`.
- [x] 5.4 `validate_all(fs, layout) -> ValidateReport` : combine les deux
      précédents pour tous les changes actifs et toutes les specs
      principales. Vérifié par
      `engine_validate::validate_all_couvre_changes_et_specs`.

## 6. Sous-commande CLI

- [x] 6.1 Ajouter la sous-commande `validate [item]` dans `codev-cli` avec
      les flags `--all`, `--changes`, `--specs`, `--json` ; exit code `0` si
      aucun `Finding` de sévérité `Error`, `1` sinon. Vérifié par
      `cli_validate::exit_zero_sur_projet_propre` et
      `cli_validate::exit_un_sur_erreur_structurelle` (harnais existant
      `Harnais` en mémoire, cf. commandes du CLI).
- [x] 6.2 Résolution d'`item` : si un seul change/spec correspond au nom,
      on le prend ; ambigu → erreur `ambiguous_item` avec la liste ; absent
      → `unknown_item`. Vérifié par
      `cli_validate::item_ambigu_liste_les_candidats`.

## 7. Contrat JSON v1 et rendu humain

- [x] 7.1 `contract::v1::ValidateReport` avec `items[]` (chacun `kind`,
      `name`, `path`, `findings[]`) et `status[]` à la racine, sérialisé en
      camelCase, testé par snapshot dans `contract::tests` :
      `validate_report_shape_stable`.
- [x] 7.2 Rendu humain : une ligne par finding, format `path:line: code —
      message`, précédé du titre de l'item. Vérifié par
      `render::validate_ecrit_ligne_par_finding_avec_path_et_ligne`.
- [x] 7.3 Forme d'échec : quand la racine est introuvable, stdout porte
      exactement un document JSON de la forme du rapport, listes vides,
      `status` racine porte l'entrée d'erreur. Vérifié par
      `cli_validate::echec_json_garde_la_forme` (scénario
      `Sortie JSON quand la racine est introuvable`).

## 8. Dogfooding et intégration workspace

- [x] 8.1 Faire tourner `codev validate --all` sur ce dépôt et corriger tout
      finding remonté ; vérifier que la commande passe avec exit 0. Le
      dépôt lui-même est le premier consommateur, comme pour `codev init`.
- [x] 8.2 `cargo test --workspace` reste vert et compte au moins 20 tests
      supplémentaires (règles + engine + CLI + contrat + rendu).
- [x] 8.3 `cargo clippy --workspace --all-targets` reste sans avertissement.
