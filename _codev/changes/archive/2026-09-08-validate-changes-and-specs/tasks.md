# Tasks

## 1. Foundations in `codev-core`

- [x] 1.1 Create `codev-core::validate` (module `validate/mod.rs`,
      `validate/rules.rs`), expose `pub trait Rule` with `code()`,
      `check_spec()`, `check_delta()` (empty defaults), plus
      `pub static RULES: &[&dyn Rule]`. Verified by
      `cargo build -p codev-core` and a test compiling
      `validate::registry_liste_au_moins_une_regle`.
- [x] 1.2 Invariant test `validate::codes_de_findings_sont_uniques`:
      concatenates all the codes emitted by the parser (constants in the
      parser modules) and by the rules, checks that they are pairwise
      distinct. Serves as a guardrail against an accidental duplicate.

## 2. Additional structural rules (pure core)

- [x] 2.1 Rule `RequirementNoShall`: walks the requirements of a `Spec`
      and of each `Added`/`Modified` section of a `Delta`, emits
      `requirement_no_shall` if the description contains neither `SHALL`
      nor `MUST` (exact uppercase). Verified by
      `validate::requirement_sans_shall_est_signalee` (scenario
      `Requirement without SHALL or MUST`).
- [x] 2.2 Rule `RequirementNoScenario`: emits `requirement_no_scenario` if
      `scenarios` is empty. Verified by
      `validate::requirement_sans_scenario_est_signalee` (scenario
      `Requirement without any scenario`).
- [x] 2.3 Rule `SpecNoRequirement`: on a `Spec`, emits
      `spec_no_requirement` if `requirements` is empty. Verified by
      `validate::spec_sans_requirement_est_signalee` (scenario
      `Main spec without a requirement`).

## 3. Consistency rules between sections (pure core)

- [x] 3.1 Rule `CrossSectionConflict`: indexes requirement names by
      section (Added/Modified/Removed), emits `cross_section_conflict` for
      each name present in two sections, with the list of sections
      involved and their lines. Verified by
      `validate::exigence_dans_added_et_modified_est_signalee` (scenario
      `Requirement present in ADDED and MODIFIED`).
- [x] 3.2 Rule `RenameTargetCollision`: if a `RENAMED.TO` already exists
      as an `ADDED`, emits `rename_target_collision`. Verified by
      `validate::rename_to_qui_collide_avec_added_est_signale` (scenario
      `RENAMED.TO collides with a same-named ADDED`).
- [x] 3.3 Rule `ModifiedUsesOldName`: if a `MODIFIED` references a
      `RENAMED.FROM`, emits `modified_uses_old_name`. Verified by
      `validate::modified_reference_ancien_nom_renamed_est_signale`
      (scenario `MODIFIED references the old name of a RENAMED`).

## 4. Zero-delta rule and skip_specs conflict (engine side)

- [x] 4.1 Extend `codev-engine::validate` with `check_change_metadata`,
      which compares the presence of deltas with the `skip_specs` marker.
      Emits `zero_delta_without_marker` if no delta and marker absent,
      emits `skip_specs_conflict` if marker present and deltas exist.
      Verified by `engine_validate::zero_delta_sans_marqueur_echoue` and
      `engine_validate::skip_specs_avec_delta_est_un_conflit` (same-named
      scenarios of the spec).

## 5. Orchestration on the engine side

- [x] 5.1 Type `LocatedFinding { path: PathBuf, kind: ItemKind, ..Finding }`
      in `codev-engine::validate`, with `From<(&Finding, path, kind)>`.
      Verified by `engine_validate::located_conserve_code_line_severite`.
- [x] 5.2 `validate_change(fs, layout, change) -> ItemReport`: opens each
      delta of the change, calls `parse_delta`, applies the `check_delta`
      rules, produces the `LocatedFinding`s grouped by file. Also runs
      `check_change_metadata` (task 4.1). Verified by
      `engine_validate::rapport_change_couvre_tous_les_fichiers_de_delta`.
- [x] 5.3 `validate_spec(fs, layout, capability) -> ItemReport`: opens the
      main spec, calls `parse_spec`, applies the `check_spec` rules.
      Verified by `engine_validate::rapport_spec_expose_les_findings_du_parseur`.
- [x] 5.4 `validate_all(fs, layout) -> ValidateReport`: combines the two
      previous ones for all active changes and all main specs. Verified by
      `engine_validate::validate_all_couvre_changes_et_specs`.

## 6. CLI subcommand

- [x] 6.1 Add the `validate [item]` subcommand to `codev-cli` with
      the flags `--all`, `--changes`, `--specs`, `--json`; exit code `0` if
      no `Finding` of severity `Error`, `1` otherwise. Verified by
      `cli_validate::exit_zero_sur_projet_propre` and
      `cli_validate::exit_un_sur_erreur_structurelle` (existing in-memory
      `Harnais` harness, cf. the CLI commands).
- [x] 6.2 Resolution of `item`: if a single change/spec matches the name,
      it is taken; ambiguous → error `ambiguous_item` with the list; absent
      → `unknown_item`. Verified by
      `cli_validate::item_ambigu_liste_les_candidats`.

## 7. JSON contract v1 and human rendering

- [x] 7.1 `contract::v1::ValidateReport` with `items[]` (each with `kind`,
      `name`, `path`, `findings[]`) and `status[]` at the root, serialized
      in camelCase, snapshot-tested in `contract::tests`:
      `validate_report_shape_stable`.
- [x] 7.2 Human rendering: one line per finding, format `path:line: code —
      message`, preceded by the item's title. Verified by
      `render::validate_ecrit_ligne_par_finding_avec_path_et_ligne`.
- [x] 7.3 Failure shape: when the root cannot be found, stdout carries
      exactly one JSON document with the report's shape, empty lists, and
      a root `status` carrying the error entry. Verified by
      `cli_validate::echec_json_garde_la_forme` (scenario
      `JSON output when the root cannot be found`).

## 8. Dogfooding and workspace integration

- [x] 8.1 Run `codev validate --all` on this repository and fix any
      finding reported; check that the command passes with exit 0. The
      repository itself is the first consumer, as for `codev init`.
- [x] 8.2 `cargo test --workspace` stays green and counts at least 20
      additional tests (rules + engine + CLI + contract + rendering).
- [x] 8.3 `cargo clippy --workspace --all-targets` stays warning-free.
