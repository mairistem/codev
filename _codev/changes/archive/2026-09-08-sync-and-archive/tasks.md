# Tâches

## 1. Extension de `Plan` avec `Move`

- [x] 1.1 Ajouter à `codev-core::plan` une struct `Move { from: PathBuf, to:
      PathBuf }` et un champ `pub moves: Vec<Move>` sur `Plan`, plus une
      méthode `Plan::move_dir(&mut self, from, to)`. Vérifié par
      `plan::move_est_planifiable_et_deduplicable` (deux mêmes moves n'entrent
      qu'une fois).
- [x] 1.2 Étendre `apply::execute` : après `dirs` et `writes`, appliquer
      chaque `Move` via un nouveau port `FileSystem::rename(from, to)` avec
      fallback copy+remove en cas d'erreur cross-device. Enrichir `Applied`
      d'un champ `moved: Vec<(PathBuf, PathBuf)>`. Vérifié par
      `apply::execute_deplace_apres_avoir_ecrit` (l'ordre est respecté) et
      `apply::execute_deplace_meme_cross_device` (fallback simulé côté
      `MemoryFileSystem`).
- [x] 1.3 Étendre `FileSystem` avec `fn rename(&self, from, to) ->
      io::Result<()>` en implémentation réelle (`std::fs::rename` +
      fallback) et en mémoire (`MemoryFileSystem` : déplace fichiers et
      dirs). Vérifié par `ports::memory_rename_deplace_un_arbre_entier`.

## 2. Fusion pure dans `codev-core::merge`

- [x] 2.1 Créer `codev-core::merge` (module `merge/mod.rs`,
      `merge/edits.rs`, `merge/render.rs`) et exposer :
      - `pub struct Edit { byte_range: Range<usize>, replacement: String }`
      - `pub struct MergePlan { edits: Vec<Edit>, /* ... */ }`
      - `pub fn merge_into_existing(spec_source, spec_ast, delta) -> Result<MergePlan, MergeError>`
      - `pub fn build_new_spec(capability_path, delta) -> Result<String, MergeError>`
      Vérifié par `cargo build -p codev-core` et un test compilant
      `merge::edit_construction_est_stable`.
- [x] 2.2 Rendu markdown d'un `Requirement` : `### Requirement: <name>` +
      ligne blanche + `description` + `\n\n` pour chaque `Scenario`, chaque
      scenario en `#### Scenario: <name>` + ligne blanche + `body`. Vérifié
      par `merge::render_requirement_est_stable` (snapshot du rendu d'un
      requirement complet).
- [x] 2.3 Édits pour `MODIFIED` : remplace `byte_range` de l'exigence cible
      par le nouveau rendu. Cible introuvable → `MergeError::ModifiedTargetMissing
      { name }` (code stable `modified_target_missing`). Vérifié par
      `merge::modified_remplace_le_bloc` et `merge::modified_sans_cible_echoue`.
- [x] 2.4 Édits pour `REMOVED` : supprime `byte_range` étendu à l'espacement
      qui suit. Si l'exigence est la dernière du spec → `MergeError::WouldLeaveSpecWithoutRequirement
      { name }` (code `would_leave_spec_without_requirement`). Vérifié par
      `merge::removed_supprime_le_bloc_et_son_espacement` et
      `merge::removed_de_derniere_exigence_echoue`.
- [x] 2.5 Édits pour `RENAMED` : remplace uniquement la ligne d'en-tête (la
      première ligne du `byte_range`). Cible introuvable → simple
      no-op signalé plus tard (le validateur E6 traitera ; ici, silence).
      Vérifié par `merge::renamed_ne_touche_qu_a_l_entete`.
- [x] 2.6 Édits pour `ADDED` : `byte_range` réduit au point d'insertion à
      la fin de la section `## Requirements` ; ajoute une ligne blanche avant
      chaque bloc si nécessaire pour préserver la densité. Vérifié par
      `merge::added_est_insere_apres_la_derniere_exigence`.
- [x] 2.7 `Spec` gagne un accesseur `requirements_section_end() ->
      Option<usize>` — offset en octets du point d'insertion des ADDED (juste
      avant le prochain `## ` de premier niveau, ou fin de fichier). Utilise
      les spans déjà présents. Vérifié par
      `spec::requirements_section_end_est_avant_section_libre_qui_suit`.
- [x] 2.8 Application des édits : la fonction `apply_edits(source: &str,
      edits: &[Edit]) -> String` trie par `byte_range.end` décroissant puis
      applique. Vérifié par `edits::apply_est_stable_meme_avec_ordre_melange`
      et `edits::apply_preserve_le_contenu_hors_ranges` (round-trip d'un
      fichier avec un edit vide).
- [x] 2.9 `build_new_spec` : rendu canonique
      `# <Titre> Specification\n\n## Purpose\n\n<purpose>\n\n## Requirements\n\n<blocs ADDED>`
      avec `<Titre>` dérivé du chemin (`identity/user-auth` → `User Auth`).
      Refus si delta sans Purpose → `MergeError::NewCapabilityWithoutPurpose`
      (code `new_capability_without_purpose`). Vérifié par
      `merge::build_new_spec_avec_purpose_et_added` et
      `merge::build_new_spec_sans_purpose_echoue`.

## 3. Orchestration `sync` côté engine

- [x] 3.1 Créer `codev-engine::sync` avec `pub fn plan_sync(fs, layout,
      change_id) -> Result<SyncPlan, EngineError>`. La `SyncPlan` porte le
      `Plan` (dirs + writes) et un rapport en construction (nouveaux
      fichiers vs mis à jour vs inchangés). Vérifié par
      `engine_sync::plan_sync_produit_un_write_par_capacite_touchee`.
- [x] 3.2 `SyncOutcome` sépare `updated: Vec<PathBuf>`, `created:
      Vec<PathBuf>`, `unchanged: Vec<PathBuf>`. `unchanged` alimenté par
      comparaison contenu-à-contenu avant l'écriture (idempotence). Vérifié
      par `engine_sync::deuxieme_sync_ne_change_rien` (invariant
      d'idempotence).
- [x] 3.3 `pub fn execute_sync(fs, layout, change_id) -> Result<SyncOutcome,
      EngineError>` : construit le plan, l'exécute, agrège l'outcome.
      Vérifié par `engine_sync::sync_dun_delta_multi_capacites_reussit`
      (deux capacités touchées, deux specs à jour, une créée).

## 4. Orchestration `archive` côté engine

- [x] 4.1 Créer `codev-engine::archive` avec `pub fn plan_archive(fs,
      layout, clock, change_id) -> Result<ArchivePlan, EngineError>`.
      Appelle `validate_change` en pré-flight ; renvoie `EngineError` de code
      `validation_failed` si `has_errors()`. Vérifié par
      `engine_archive::preflight_valide_avant_de_planifier` (avec un delta
      contenant un `duplicate_requirement`).
- [x] 4.2 Combine le `SyncPlan` et l'opération `Move` du dossier de change
      vers `archive/<date>-<nom>/`. `<date>` vient du port `Clock`. Vérifié
      par `engine_archive::plan_inclut_sync_puis_move_daté`.
- [x] 4.3 `pub fn execute_archive(fs, layout, clock, change_id) ->
      Result<ArchiveOutcome, EngineError>` : plan puis exécution. Vérifié par
      `engine_archive::archive_deplace_le_change_et_conserve_ses_fichiers`
      (utilise `FixedClock`).

## 5. Sous-commandes CLI

- [x] 5.1 Ajouter `Command::Sync { change: Option<String>, json: bool }`
      dans `codev-cli`, avec résolution du change identique à `status` (un
      seul actif → implicite, sinon ambigu ou inconnu). Exit code : `0` si
      succès, `1` sinon. Vérifié par
      `cli_sync::sync_un_seul_change_actif_est_implicite`.
- [x] 5.2 Ajouter `Command::Archive { change: Option<String>, json: bool }`.
      Vérifié par `cli_archive::archive_avec_validate_erreur_echoue_code_util`.
- [x] 5.3 Rendu humain : liste des chemins par catégorie (« Créé : … »,
      « Mis à jour : … », « Inchangé : … », pour archive « Déplacé vers
      … »). Vérifié par `render::sync_liste_par_categorie` et
      `render::archive_ajoute_moved_to`.

## 6. Contrat JSON v1

- [x] 6.1 `contract::v1::SyncReport` avec `changeName`, `root`, `updated`,
      `created`, `unchanged`, `status[]`, sérialisé en camelCase. Testé par
      `contract::sync_report_shape_stable`.
- [x] 6.2 `contract::v1::ArchiveReport` avec les champs de `SyncReport` plus
      `movedTo: String`, sérialisé en camelCase. Testé par
      `contract::archive_report_shape_stable`.
- [x] 6.3 Forme d'échec : quand la racine est introuvable ou le change
      inconnu, stdout porte exactement un document JSON de la forme
      correspondante, avec listes vides et `status` racine portant l'erreur.
      Vérifié par `cli_sync::echec_json_garde_la_forme` et
      `cli_archive::echec_json_garde_la_forme`.

## 7. Golden tests de fusion

- [x] 7.1 Créer `crates/codev-engine/tests/sync_golden.rs` avec des fixtures
      représentatives sous `crates/codev-engine/tests/fixtures/sync/` :
      spec principale existante + delta ADDED, spec + delta MODIFIED, spec +
      delta REMOVED, spec + delta RENAMED, delta multi-opérations. Vérifié
      par un test par fixture (`sync_golden::added_ajoute`,
      `sync_golden::modified_remplace`, `sync_golden::removed_supprime`,
      `sync_golden::renamed_retitle`, `sync_golden::multi_ops_est_deterministe`).
- [x] 7.2 Test d'invariant `sync_golden::une_section_libre_apres_requirements_survit` :
      fichier de spec avec `## Notes` après `## Requirements`, delta qui
      touche une exigence ; vérifier au caractère près que la section
      `## Notes` est intacte.
- [x] 7.3 Test d'invariant `sync_golden::commentaire_html_dans_exigence_non_touchee_survit` :
      un `<!-- ... -->` dans une exigence non touchée reste identique.
- [x] 7.4 Test d'idempotence `sync_golden::deux_syncs_successifs_donnent_le_meme_resultat` :
      appliquer le sync deux fois d'affilée doit produire le même contenu au
      caractère près.

## 8. Dogfooding et intégration workspace

- [x] 8.1 Après implémentation, faire tourner `codev archive
      parse-specs-and-deltas` puis `codev archive validate-changes-and-specs`
      sur ce dépôt. Vérifier que `_codev/specs/spec-parsing/spec.md` et
      `_codev/specs/validation/spec.md` sont créés à partir des Purpose des
      deltas, et que les deux dossiers de change sont sous
      `_codev/changes/archive/`.
- [x] 8.2 Après archivage des deux, lancer `codev validate --all` : doit
      exit 0 et lister les deux specs principales nouvellement créées.
- [x] 8.3 `cargo test --workspace` reste vert et compte au moins 30 tests
      supplémentaires (merge, engine sync/archive, CLI, contrat, rendu,
      golden).
- [x] 8.4 `cargo clippy --workspace --all-targets` reste sans avertissement.
