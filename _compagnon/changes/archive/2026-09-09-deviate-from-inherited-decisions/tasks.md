# Tasks

## 1. Core — extended ADR parser

- [x] 1.1 Add the `deviates_from: Vec<String>` field to
      `codev_core::decisions::Decision` (next to `supersedes:
      Vec<String>`, same conventions).
- [x] 1.2 Extend `parse_decision` to read `deviates_from` from the YAML
      frontmatter: optional list, each entry must be a string. Field
      absent → empty list.
- [x] 1.3 New stable finding `decision_field_type_mismatch` (warning on
      the parser side, code exposed in `codev_core::parser::codes` if
      that module is the right place; otherwise in the decisions
      module) — emitted when `deviates_from` is not a list or carries a
      non-string entry.
- [x] 1.4 Tests: ADR with `deviates_from` with one entry, with several
      entries, absent, malformed (string instead of a list), non-string
      entry in the list.

## 2. Core — `plan_deviate` action

- [x] 2.1 New pure function `plan_deviate(index, existing_seal,
      qualified_target, title, today, layout) -> Result<DeviatePlan,
      ActionError>` in `codev-engine::decisions_actions`. Produces a
      plan that writes the local ADR (`deviates_from: ["<qualified>"]`,
      `status: accepted`) **and** the seal entry, in a single atomic
      plan.
- [x] 2.2 Explicit refusals — stable codes:
      - `cannot_deviate_from_local` if `qualified_target` starts with
        `project/` — message pointing to `codev decision supersede`;
      - `unknown_decision_id` if the target is not indexed;
      - `ambiguous_decision_id` if two sources expose the same
        qualified id;
      - `empty_title` if the title is empty.
- [x] 2.3 Extend `render_frontmatter` (or a dedicated function) to emit
      the `deviates_from: ["…"]` field when the list is non-empty —
      same style as `supersedes`.
- [x] 2.4 Tests: the plan produces two writes (ADR + seal); the ADR
      does contain the line `deviates_from: ["path:~/shared/0100"]`;
      refusals surface the right code.

## 3. Core — index and hiding

- [x] 3.1 Extend `IndexEntry` with a derived field `deviated_by:
      Option<QualifiedId>`. Computed, not persisted.
- [x] 3.2 New pass `resolve_deviations(&mut entries, &mut findings)` in
      `codev-engine::decisions`, called after supersession resolution.
      Two effects:
      - For each local `accepted` ADR with `deviates_from`, mark each
        inherited target with `deviated_by = <qualified-local>`;
      - Remove the targets so marked from the `in_effect` computation.
- [x] 3.3 `decision_dangling_deviation` (warning) emitted during this
      pass for each target not found in the index.
- [x] 3.4 `decision_conflicting_deviations` (error) emitted during this
      pass for any target referenced by at least two local `accepted`
      ADRs. The target stays marked `deviated_by` with the first one
      found (stable order by `qualified_id`), but the finding makes the
      conflict visible.
- [x] 3.5 Tests: deviation from path:, deviation from git:, two
      deviations on the same target (conflict), orphan deviation
      (dangling), proposed ADR with deviates_from (no effect).

## 4. Core — injection into `design` instructions

- [x] 4.1 Rendering of the `decisions[]` array in the instructions must
      filter out entries where `deviated_by.is_some()` in addition to
      the `in_effect` filter already in place. Test: the design
      instructions of a change where an inherited decision was deviated
      only mention the local one.
- [x] 4.2 Same for the human "Decisions in effect" section of the text
      rendering.
- [x] 4.3 Golden test: the `deviates_from: [...]` line in a local ADR's
      frontmatter **makes the referenced inherited decision invisible**,
      for all active changes — not only the current change.

## 5. Shell — `decision deviate` CLI

- [x] 5.1 New subcommand `codev decision deviate
      <qualified-id> <title> [--json]`. Routes to the engine action.
- [x] 5.2 Human success rendering: `✓ Created local deviation from
      "<qualified>": <local-id> <title>` + `  File: <path>`.
- [x] 5.3 JSON contract: new struct `DecisionDeviatedV1` (close to
      `DecisionCreatedV1`) carrying the created decision and
      `bodySha256`. New shape `decision_deviated_shape()` in
      `main.rs`.
- [x] 5.4 CLI integration tests: successful deviation, deviation from a
      local decision (refusal `cannot_deviate_from_local`), deviation
      from an unknown id (refusal `unknown_decision_id`).

## 6. JSON contract — additions

- [x] 6.1 `DecisionV1` gains `deviatesFrom: Vec<String>` (additive —
      ADRs without the field output `[]`).
- [x] 6.2 The entries of `codev decision list --json` (from
      `build_summaries`) gain `deviatedBy: Option<String>`, serialized
      only when not `None`
      (`skip_serializing_if = "Option::is_none"`).
- [x] 6.3 Contract test: the JSON list of a project with a deviation
      does expose `deviatedBy` on the target and `deviatesFrom` on the
      source.

## 7. Validate — integrated findings

- [x] 7.1 The `validate_decisions` report surfaces the
      `decision_dangling_deviation` and `decision_conflicting_deviations`
      findings computed by `resolve_deviations` together with the other
      sealing findings.
- [x] 7.2 Tests in `validate::tests`: conflict → `has_errors()` is
      `true`; dangling alone → warning only, `has_errors()` is
      `false`.

## 8. Dogfooding

- [x] 8.1 `cargo test --workspace` stays green, gains at least 15 new
      tests (parser + plan_deviate + index + CLI).
- [x] 8.2 `cargo clippy --workspace --all-targets` stays free of
      warnings.
- [x] 8.3 `codev validate --all` stays green on this repository (no
      local deviation, no inherited source declared).
