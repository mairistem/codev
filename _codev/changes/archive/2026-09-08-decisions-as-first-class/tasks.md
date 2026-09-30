# Tasks

## 1. Pure parser in `codev-core::decisions`

- [x] 1.1 Create `codev-core::decisions` (`decisions/mod.rs`,
      `decisions/ast.rs`, `decisions/parser.rs`), expose `Decision`,
      `DecisionStatus`, `parse_decision`. Verified by `cargo build -p
      codev-core`.
- [x] 1.2 AST types: `Decision { id: String, title: String, status:
      DecisionStatus, date: String, tags: Vec<String>, supersedes:
      Vec<String>, sections: Vec<Section>, span: Span }`, enumeration
      `DecisionStatus { Accepted, Superseded, Proposed, Deprecated,
      Rejected, Unknown(String) }`. Verified by
      `decisions::les_types_dast_se_construisent_a_la_main`.
- [x] 1.3 Stable finding codes in `parser/codes.rs` (centralized
      module): `decision_missing_frontmatter`,
      `decision_missing_field`, `decision_unknown_status`,
      `decision_supersedes_unknown`, `decision_id_collision`,
      `decision_supersession_cycle`. Added to the `ALL` constant so that
      `validate`'s uniqueness invariant covers them. Verified by
      `validate::codes_de_findings_sont_uniques`.
- [x] 1.4 `parse_decision`: split on `---\n`, extract the frontmatter
      via `serde_norway::from_str` with `deny_unknown_fields`, parse the
      body into `##` sections via `parser::shared`. Verified by
      `decisions::parse_adr_bien_forme` (scenario of the same name).
- [x] 1.5 `parse_decision` without frontmatter → finding
      `decision_missing_frontmatter`, empty result. Verified by
      `decisions::parse_sans_frontmatter_signale`.
- [x] 1.6 `parse_decision` with a missing mandatory field → finding
      `decision_missing_field` naming the field. Verified by
      `decisions::parse_sans_titre_signale`.
- [x] 1.7 `parse_decision` with an unknown `status` → variant
      `Unknown(...)` + finding `decision_unknown_status` listing the
      recognized statuses. Verified by
      `decisions::status_inconnu_est_signale`.

## 2. Index and supersession in `codev-engine::decisions`

- [x] 2.1 Create `codev-engine::decisions` (`decisions/mod.rs`). Public
      types: `IndexEntry { qualified_id, decision, origin, path }`,
      `Origin { Project, Path(String) }`, `DecisionIndex { entries,
      in_effect, findings }`, `pub fn index(fs, layout, config) ->
      DecisionIndex`. Verified by
      `engine_decisions::index_vide_sur_projet_sans_adr`.
- [x] 2.2 `index` walks the project's `_codev/decisions/` and, for each
      `inherits: path:` source, `<path>/_codev/decisions/`. Respective
      origin marked. Verified by
      `engine_decisions::adr_du_projet_et_dune_source_apparaissent_avec_leur_origin`.
- [x] 2.3 Computing `in_effect`: an `accepted` entry not superseded by
      another `accepted` entry is in effect; other statuses never are.
      Verified by
      `engine_decisions::supersession_directe_masque_la_source` (scenario
      "Direct supersession") and
      `engine_decisions::chaine_a_trois_maillons_laisse_le_dernier`
      (scenario of the same name).
- [x] 2.4 `supersedes` pointing to a missing id → finding
      `decision_supersedes_unknown`, the decision stays in effect.
      Verified by `engine_decisions::supersedes_vers_absent_est_signale`
      (scenario "Missing supersession target").
- [x] 2.5 Id collision between project and source → finding
      `decision_id_collision`, project version kept. Verified by
      `engine_decisions::collision_projet_source_projet_gagne` (scenario
      of the same name).
- [x] 2.6 Supersession cycle → finding `decision_supersession_cycle`,
      none of the decisions in the cycle enters `in_effect`. Verified by
      `engine_decisions::cycle_de_supersession_est_signale`.

## 3. Injection into the instructions

- [x] 3.1 Extend `Instructions` (`codev-engine::instructions`) with a
      `pub decisions: Vec<DecisionRef>` field, with `DecisionRef { id,
      qualified_id, title, status, tags, path: PathBuf, origin }`.
      Verified by
      `instructions::instructions_portent_un_champ_decisions_meme_vide`.
- [x] 3.2 `for_artifact`: for the `design` artifact, fills `decisions`
      with the index's `in_effect` entries. For any other artifact, leaves
      it empty. Verified by
      `instructions::design_recoit_les_decisions_en_vigueur` and
      `instructions::proposal_ne_recoit_pas_les_decisions` (at this
      stage — a future change may extend it).

## 4. JSON contract v1

- [x] 4.1 New `DecisionRefV1` in `contract::v1` in camelCase:
      `id`, `qualifiedId`, `title`, `status`, `tags`, `path`, `origin`.
      Added to `InstructionsV1` as a `decisions: Vec<DecisionRefV1>`
      field. Verified by
      `contract::instructions_v1_expose_decisions_en_camel_case`.
- [x] 4.2 Backward-compatibility test:
      `contract::instructions_v1_sans_decisions_reste_valide` — the other
      artifacts do have `decisions: []` but the field exists.

## 5. Human rendering

- [x] 5.1 `render::instructions` adds a "Decisions in effect" section
      when `decisions` is not empty, one line per entry in the format
      `- <id> <title>`. Verified by
      `render::instructions_design_liste_les_decisions_en_vigueur`.
- [x] 5.2 No section when `decisions` is empty. Verified by
      `render::instructions_sans_decisions_nest_pas_seche_de_titre_vide`.

## 6. Dogfooding and workspace integration

- [x] 6.1 Create a small test change `poc-design-with-decisions` (a
      fictitious capability), run `codev instructions design --change …
      --json`, check that the `decisions` array contains the 6 ADRs
      already present in the repository (`0001` to `0006`). Then delete
      this test change — it does not remain in the repository.
- [x] 6.2 `cargo test --workspace` stays green and counts at least 15
      additional tests (parser + index + injection + contract + rendering).
- [x] 6.3 `cargo clippy --workspace --all-targets` stays warning-free.
- [x] 6.4 `codev validate --all` stays green.
