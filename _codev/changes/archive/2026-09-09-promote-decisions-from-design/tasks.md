# Tasks

## 1. Core — parser for `### Decision: ...` blocks

- [x] 1.1 New module `codev-engine::design` (pure function — same
      pattern as the other parsers). Signature:
      ```rust
      pub struct DecisionBlock {
          pub title: String,
          pub body: String,
          pub byte_range: Range<usize>,
          pub line: u32,
      }
      pub fn extract_decision_blocks(source: &str) -> Vec<DecisionBlock>;
      ```
      Locates `## Decisions`, then loops over the lines `### Decision:
      <title>` (exact H3, colon, space), up to the next `### ` or
      `## `. `title` = what follows `Decision: ` **trimmed**. `body`
      = bytes between the end of the title line and the start of the
      next block, **verbatim**.
- [x] 1.2 Tests: single block, two blocks, no block, block with
      markdown formatting (table, code fence) — the body is
      byte-for-byte.

## 2. Core — lookup by title + ambiguity refusal

- [x] 2.1 Function `find_decision_block(blocks: &[DecisionBlock], title:
      &str) -> Result<usize, PromoteLookupError>`. Typed errors:
      `NotFound(String)`, `Ambiguous(String, Vec<u32>)` with the line
      numbers.
- [x] 2.2 Tests: found, not found, ambiguous (two positions).

## 3. Shell — `plan_promote` in `decisions_actions`

- [x] 3.1 New struct `PromotePlan`: `plan: Plan`, `new_id`,
      `new_path`, `body_sha256`, `source_change: String`.
- [x] 3.2 New `ActionError` variants: `CannotPromoteFromArchived
      { change: String }`, `DesignMissing { change: String }`,
      `DecisionHeadingNotFound { title: String }`,
      `AmbiguousDecisionHeading { title: String, lines: Vec<u32> }`.
      Corresponding stable codes.
- [x] 3.3 Function `plan_promote(index, existing_seal, change_id,
      heading, design_source, today, layout, change_dir)`. It:
      - refuses if the title is empty (`EmptyTitle`);
      - parses `design_source` via `design::extract_decision_blocks`;
      - looks up the block by title; returns the appropriate error;
      - builds the ADR body: sections `## Context` (empty,
        placeholder comment), `## Decision` (verbatim body),
        `## Consequences` (placeholder), `## Rejected Alternatives`
        (placeholder);
      - computes `next_local_id` and the slug;
      - computes `body_sha256` and adds the seal entry;
      - builds the `new_design_source` with the block substituted
        (preserves spacing);
      - returns a plan with 3 writes: ADR (CreateOnly), seal.yaml
        (Overwrite), design.md (Overwrite).
- [x] 3.4 Tests: basic promotion, verbatim body, refusal of an absent
      title, refusal of an ambiguous title, refusal of an empty title.

## 4. Shell — `execute_promote` on the engine side

- [x] 4.1 Function `execute_promote(fs, env, layout, config, clock,
      change_id, heading)` that composes: `change::load` (gets the
      `ChangeContext`, checks that the change exists), reading the
      design, calling `plan_promote`, execution.
- [x] 4.2 Explicit refusal `cannot_promote_from_archived`: if the
      computed `change_dir` is under `changes/archive/`. Detected via a
      simple path check (the layout only knows active changes under
      `changes/<name>/`, an archived change is not reachable by
      `change::load` — the `UnknownChange` error is surfaced today).
      **New**: `codev list` alone shows the active ones; we add a
      manual pass that checks for the existence of a folder under
      `_codev/changes/archive/*-<name>/` to distinguish the two cases
      and return the right stable code.
- [x] 4.3 `design_missing`: if `<change_dir>/design.md` does not exist
      → dedicated code.
- [x] 4.4 Integration tests: end-to-end promotion on an in-memory
      project, checks the created ADR, the seal.yaml, the substitution
      in the design.

## 5. CLI — `codev decision promote` subcommand

- [x] 5.1 `DecisionCommand::Promote { change, title, json }`.
      clap documentation: "Promote a `### Decision: <title>` block
      from an active change's design.md to a first-class ADR, sealed
      by K3, referenced from the design."
- [x] 5.2 Routes to `commands::decision_promote(ctx, change, title)`.
      Returns a `DecisionPromotedOutcome` (root, decision summary,
      path, body_sha256, source_change).
- [x] 5.3 Human rendering:
      ```
      ✓ Promoted decision "<title>" to ADR 0007: <title>
        File:      _codev/decisions/0007-<slug>.md
        Hash:      sha256:...
        Source:    _codev/changes/<change>/design.md

      Split the body into Context / Decision / Consequences /
      Rejected Alternatives before archiving the change.
      ```
- [x] 5.4 CLI integration tests: success, refusal of an archived
      change, refusal of an absent title, refusal of an ambiguous
      title.

## 6. JSON contract

- [x] 6.1 New `DecisionPromotedV1`:
      ```rust
      { root, decision: Option<DecisionV1>, path, bodySha256,
        sourceChange, status: [] }
      ```
      Fully additive.
- [x] 6.2 New `decision_promoted_shape()` in `main.rs`.
- [x] 6.3 Contract test: the success JSON carries all fields, the
      failure JSON carries the shape with the stable codes.

## 7. Workspace integration

- [x] 7.1 `cargo test --workspace` stays green, +12 new tests
      (parser + lookup + plan + execute + CLI).
- [x] 7.2 `cargo clippy --workspace --all-targets` without warnings.
- [x] 7.3 `codev validate --strict --all` stays green.
- [x] 7.4 Manual test: on the test change, create a design.md with a
      decision block, run the promotion, check that the ADR is
      created, sealed, referenced, and that `codev decision list`
      does list it.
