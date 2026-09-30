# Tasks

## 1. Foundations in `codev-core`

- [x] 1.1 Create the `parser` module in `codev-core` (`parser/mod.rs`,
      `parser/ast.rs`, `parser/fence.rs`, `parser/spec.rs`, `parser/delta.rs`),
      expose the public items from `lib.rs`, verified by `cargo build -p
      codev-core` then `cargo doc -p codev-core --no-deps`.
- [x] 1.2 Define the AST types — `Spec`, `Delta`, `Requirement`,
      `Scenario`, the `DeltaOp` enumeration, the `Finding` and `Parsed<T>`
      structures — without logic, with `#[derive(Debug, Clone, PartialEq)]`,
      verified by a compiling test that builds each variant by hand.
- [x] 1.3 Define `Span { byte_range: Range<usize>, line_range: Range<u32> }`
      and attach it to each AST node as a field, verified by a test that
      builds a `Span` and reads both of its ranges.

## 2. Masking literal zones

- [x] 2.1 Write `build_fence_mask(source: &str) -> Vec<bool>` in
      `parser/fence.rs` — one boolean per line, true if the line belongs to
      a code block (opening fence, closing fence, content). Verified by the
      tests `fence::mask_reconnait_backticks_et_tildes`,
      `fence::mask_refuse_une_fermeture_de_marqueur_different` and
      `fence::mask_traite_deux_blocs_successifs`.
- [x] 2.2 Extend the mask to multi-line HTML comments `<!-- … -->` outside
      fences, verified by the test
      `fence::commentaire_multi_lignes_est_masque` (the comment encloses a
      fake `### Requirement:` that must not show up in the AST later).

## 3. Main spec parser

- [x] 3.1 Implement `parse_spec(source: &str) -> Parsed<Spec>` — extraction
      of `## Purpose` and `## Requirements`, iteration over the
      `### Requirement:` headings within `## Requirements` only, iteration
      over the `#### Scenario:` headings within each requirement. Verified by
      `spec::extrait_purpose_et_une_exigence_avec_scenario` (the spec's
      first scenario, `Well-formed Purpose and requirements`).
- [x] 3.2 Missing Purpose reported as a `Finding` with `Error` severity
      without preventing extraction of the requirements, verified by
      `spec::purpose_manquant_est_un_finding_localise` (scenario of the same
      name).
- [x] 3.3 A `### Requirement:` outside `## Requirements` produces a
      `Finding` that names the line and states that the expected section is
      `## Requirements`, verified by
      `spec::exigence_hors_section_est_signalee`.
- [x] 3.4 A delta header encountered in a main spec produces a `Finding`
      naming the line, verified by
      `spec::header_de_delta_dans_main_spec_est_signale` (scenario
      `Delta header in a main spec`).

## 4. Delta parser

- [x] 4.1 Implement `parse_delta(source: &str) -> Parsed<Delta>` with
      recognition of the four sections `## ADDED|MODIFIED|REMOVED|RENAMED
      Requirements`, verified by `delta::reconnait_les_quatre_sections`.
- [x] 4.2 Extract the complete requirement blocks under `ADDED` and
      `MODIFIED` — name from `### Requirement: <name>`, descriptive text up
      to the next heading, scenarios as `#### Scenario:`. Verified by
      `delta::bloc_added_porte_exigence_et_scenario` (scenario
      `ADDED block with requirement and scenario`).
- [x] 4.3 Extract under `REMOVED` the name, the `**Reason**:` line and the
      `**Migration**:` line, verified by
      `delta::bloc_removed_porte_raison_et_migration`.
- [x] 4.4 Extract under `RENAMED` the `FROM: <old>` / `TO: <new>` pairs,
      verified by `delta::bloc_renamed_associe_from_et_to`.
- [x] 4.5 Extract an optional `## Purpose` at the top of a delta (new
      capability), verified by `delta::purpose_de_nouvelle_capacite_est_extrait`
      (scenario `Delta for a new capability with Purpose`).
- [x] 4.6 Two requirements with the same name in the same section produce a
      `Finding` naming both lines and the section, verified by
      `delta::doublon_dans_added_est_signale` (scenario
      `Duplicate requirement within one section`).

## 5. Literal zones applied

- [x] 5.1 A requirement appearing in a code block of a main spec does not
      appear in the AST, verified by
      `spec::exigence_dans_fence_est_ignoree` (scenario
      `Example requirement inside a code block`).
- [x] 5.2 A delta header inside an HTML comment is not counted, verified by
      `delta::header_dans_commentaire_html_est_ignore` (scenario
      `Delta header inside a comment`).

## 6. Source position and invariant

- [x] 6.1 Each extracted node — Purpose, Requirement, Scenario, each delta
      operation block — carries a `Span` whose `line_range.start` equals the
      (1-indexed) line of its heading in the source, verified by
      `spans::scenario_expose_sa_ligne_de_debut` (scenario
      `Line position of a scenario`, anchored on line 42).
- [x] 6.2 A scenario written with three hashes produces a `Finding` with
      code `scenario_wrong_heading_level` and the requirement appears without
      that scenario, verified by `spec::scenario_trois_dieses_est_signale`
      (scenario `Scenario written with three hashes`).
- [x] 6.3 Invariant test `spans::round_trip_preserve_la_source_a_loctet`:
      for a sample file, take each node, extract
      `source[node.span.byte_range]`, replace each range with itself in a
      new string, check byte-for-byte equality with the original.

## 7. Golden tests

- [x] 7.1 Create `crates/codev-core/tests/parser_golden.rs` with at least
      one complete main spec example and one complete delta example (input
      files in `crates/codev-core/tests/fixtures/`), expectations serialized
      as `Debug`. Verified by
      `cargo test -p codev-core --test parser_golden`.
- [x] 7.2 Add to `parser_golden` the case of a targeted rewrite of a
      `MODIFIED` block: extract its `byte_range`, inject new text into it,
      check that the rest of the file — including whitespace — is
      identical. Verified by `parser_golden::reecriture_ciblee_ne_touche_pas`.

## 8. Workspace integration

- [x] 8.1 `cargo test --workspace` stays green and counts at least 15 more
      tests than before this change.
- [x] 8.2 `cargo clippy --workspace --all-targets` stays warning-free,
      apart from documented pre-existing ones.
