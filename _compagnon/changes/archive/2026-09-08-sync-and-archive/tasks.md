# Tasks

## 1. Extending `Plan` with `Move`

- [x] 1.1 Add to `codev-core::plan` a struct `Move { from: PathBuf, to:
      PathBuf }` and a field `pub moves: Vec<Move>` on `Plan`, plus a
      method `Plan::move_dir(&mut self, from, to)`. Verified by
      `plan::move_est_planifiable_et_deduplicable` (two identical moves are
      only added once).
- [x] 1.2 Extend `apply::execute`: after `dirs` and `writes`, apply each
      `Move` via a new port `FileSystem::rename(from, to)` with a
      copy+remove fallback on a cross-device error. Enrich `Applied` with a
      field `moved: Vec<(PathBuf, PathBuf)>`. Verified by
      `apply::execute_deplace_apres_avoir_ecrit` (the order is respected) and
      `apply::execute_deplace_meme_cross_device` (fallback simulated on the
      `MemoryFileSystem` side).
- [x] 1.3 Extend `FileSystem` with `fn rename(&self, from, to) ->
      io::Result<()>` in the real implementation (`std::fs::rename` +
      fallback) and in memory (`MemoryFileSystem`: moves files and
      dirs). Verified by `ports::memory_rename_deplace_un_arbre_entier`.

## 2. Pure merge in `codev-core::merge`

- [x] 2.1 Create `codev-core::merge` (module `merge/mod.rs`,
      `merge/edits.rs`, `merge/render.rs`) and expose:
      - `pub struct Edit { byte_range: Range<usize>, replacement: String }`
      - `pub struct MergePlan { edits: Vec<Edit>, /* ... */ }`
      - `pub fn merge_into_existing(spec_source, spec_ast, delta) -> Result<MergePlan, MergeError>`
      - `pub fn build_new_spec(capability_path, delta) -> Result<String, MergeError>`
      Verified by `cargo build -p codev-core` and a test compiling
      `merge::edit_construction_est_stable`.
- [x] 2.2 Markdown rendering of a `Requirement`: `### Requirement: <name>` +
      blank line + `description` + `\n\n` for each `Scenario`, each
      scenario as `#### Scenario: <name>` + blank line + `body`. Verified
      by `merge::render_requirement_est_stable` (snapshot of the rendering
      of a complete requirement).
- [x] 2.3 Edits for `MODIFIED`: replaces the target requirement's
      `byte_range` with the new rendering. Target not found →
      `MergeError::ModifiedTargetMissing { name }` (stable code
      `modified_target_missing`). Verified by
      `merge::modified_remplace_le_bloc` and `merge::modified_sans_cible_echoue`.
- [x] 2.4 Edits for `REMOVED`: deletes the `byte_range` extended to the
      following spacing. If the requirement is the last one in the spec →
      `MergeError::WouldLeaveSpecWithoutRequirement { name }` (code
      `would_leave_spec_without_requirement`). Verified by
      `merge::removed_supprime_le_bloc_et_son_espacement` and
      `merge::removed_de_derniere_exigence_echoue`.
- [x] 2.5 Edits for `RENAMED`: replaces only the heading line (the first
      line of the `byte_range`). Target not found → plain no-op reported
      later (validator E6 will handle it; here, silence). Verified by
      `merge::renamed_ne_touche_qu_a_l_entete`.
- [x] 2.6 Edits for `ADDED`: `byte_range` reduced to the insertion point at
      the end of the `## Requirements` section; adds a blank line before
      each block if needed to preserve density. Verified by
      `merge::added_est_insere_apres_la_derniere_exigence`.
- [x] 2.7 `Spec` gains an accessor `requirements_section_end() ->
      Option<usize>` — byte offset of the insertion point for ADDED (just
      before the next top-level `## `, or end of file). Uses the spans
      already present. Verified by
      `spec::requirements_section_end_est_avant_section_libre_qui_suit`.
- [x] 2.8 Applying the edits: the function `apply_edits(source: &str,
      edits: &[Edit]) -> String` sorts by `byte_range.end` descending then
      applies. Verified by `edits::apply_est_stable_meme_avec_ordre_melange`
      and `edits::apply_preserve_le_contenu_hors_ranges` (round-trip of a
      file with an empty edit).
- [x] 2.9 `build_new_spec`: canonical rendering
      `# <Title> Specification\n\n## Purpose\n\n<purpose>\n\n## Requirements\n\n<ADDED blocks>`
      with `<Title>` derived from the path (`identity/user-auth` → `User Auth`).
      Refused if the delta has no Purpose → `MergeError::NewCapabilityWithoutPurpose`
      (code `new_capability_without_purpose`). Verified by
      `merge::build_new_spec_avec_purpose_et_added` and
      `merge::build_new_spec_sans_purpose_echoue`.

## 3. `sync` orchestration on the engine side

- [x] 3.1 Create `codev-engine::sync` with `pub fn plan_sync(fs, layout,
      change_id) -> Result<SyncPlan, EngineError>`. The `SyncPlan` carries
      the `Plan` (dirs + writes) and a report under construction (new files
      vs updated vs unchanged). Verified by
      `engine_sync::plan_sync_produit_un_write_par_capacite_touchee`.
- [x] 3.2 `SyncOutcome` separates `updated: Vec<PathBuf>`, `created:
      Vec<PathBuf>`, `unchanged: Vec<PathBuf>`. `unchanged` is fed by a
      content-to-content comparison before writing (idempotence). Verified
      by `engine_sync::deuxieme_sync_ne_change_rien` (idempotence
      invariant).
- [x] 3.3 `pub fn execute_sync(fs, layout, change_id) -> Result<SyncOutcome,
      EngineError>`: builds the plan, executes it, aggregates the outcome.
      Verified by `engine_sync::sync_dun_delta_multi_capacites_reussit`
      (two capabilities touched, two specs updated, one created).

## 4. `archive` orchestration on the engine side

- [x] 4.1 Create `codev-engine::archive` with `pub fn plan_archive(fs,
      layout, clock, change_id) -> Result<ArchivePlan, EngineError>`.
      Calls `validate_change` as pre-flight; returns an `EngineError` with
      code `validation_failed` if `has_errors()`. Verified by
      `engine_archive::preflight_valide_avant_de_planifier` (with a delta
      containing a `duplicate_requirement`).
- [x] 4.2 Combines the `SyncPlan` and the `Move` operation of the change
      folder to `archive/<date>-<name>/`. `<date>` comes from the `Clock`
      port. Verified by `engine_archive::plan_inclut_sync_puis_move_date`.
- [x] 4.3 `pub fn execute_archive(fs, layout, clock, change_id) ->
      Result<ArchiveOutcome, EngineError>`: plan then execution. Verified by
      `engine_archive::archive_deplace_le_change_et_conserve_ses_fichiers`
      (uses `FixedClock`).

## 5. CLI subcommands

- [x] 5.1 Add `Command::Sync { change: Option<String>, json: bool }`
      in `codev-cli`, with change resolution identical to `status` (only
      one active → implicit, otherwise ambiguous or unknown). Exit code: `0`
      on success, `1` otherwise. Verified by
      `cli_sync::sync_un_seul_change_actif_est_implicite`.
- [x] 5.2 Add `Command::Archive { change: Option<String>, json: bool }`.
      Verified by `cli_archive::archive_avec_validate_erreur_echoue_code_util`.
- [x] 5.3 Human rendering: list of paths per category ("Created: …",
      "Updated: …", "Unchanged: …", for archive "Moved to …"). Verified by
      `render::sync_liste_par_categorie` and
      `render::archive_ajoute_moved_to`.

## 6. JSON contract v1

- [x] 6.1 `contract::v1::SyncReport` with `changeName`, `root`, `updated`,
      `created`, `unchanged`, `status[]`, serialized in camelCase. Tested by
      `contract::sync_report_shape_stable`.
- [x] 6.2 `contract::v1::ArchiveReport` with the fields of `SyncReport` plus
      `movedTo: String`, serialized in camelCase. Tested by
      `contract::archive_report_shape_stable`.
- [x] 6.3 Failure shape: when the root cannot be found or the change is
      unknown, stdout carries exactly one JSON document of the corresponding
      shape, with empty lists and a root `status` carrying the error.
      Verified by `cli_sync::echec_json_garde_la_forme` and
      `cli_archive::echec_json_garde_la_forme`.

## 7. Merge golden tests

- [x] 7.1 Create `crates/codev-engine/tests/sync_golden.rs` with
      representative fixtures under `crates/codev-engine/tests/fixtures/sync/`:
      existing main spec + ADDED delta, spec + MODIFIED delta, spec +
      REMOVED delta, spec + RENAMED delta, multi-operation delta. Verified
      by one test per fixture (`sync_golden::added_ajoute`,
      `sync_golden::modified_remplace`, `sync_golden::removed_supprime`,
      `sync_golden::renamed_retitle`, `sync_golden::multi_ops_est_deterministe`).
- [x] 7.2 Invariant test `sync_golden::une_section_libre_apres_requirements_survit`:
      spec file with `## Notes` after `## Requirements`, delta touching a
      requirement; check character for character that the `## Notes`
      section is intact.
- [x] 7.3 Invariant test `sync_golden::commentaire_html_dans_exigence_non_touchee_survit`:
      a `<!-- ... -->` in an untouched requirement stays identical.
- [x] 7.4 Idempotence test `sync_golden::deux_syncs_successifs_donnent_le_meme_resultat`:
      applying the sync twice in a row must produce the same content
      character for character.

## 8. Dogfooding and workspace integration

- [x] 8.1 After implementation, run `codev archive
      parse-specs-and-deltas` then `codev archive validate-changes-and-specs`
      on this repository. Check that `_codev/specs/spec-parsing/spec.md` and
      `_codev/specs/validation/spec.md` are created from the deltas'
      Purpose, and that both change folders are under
      `_codev/changes/archive/`.
- [x] 8.2 After archiving both, run `codev validate --all`: must exit 0
      and list the two newly created main specs.
- [x] 8.3 `cargo test --workspace` stays green and counts at least 30
      additional tests (merge, engine sync/archive, CLI, contract,
      rendering, golden).
- [x] 8.4 `cargo clippy --workspace --all-targets` stays warning-free.
