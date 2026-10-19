# Proposal: Fewer turns in the review steps

## Why

Each turn of an agent re-reads its whole context, about 35,000 to 50,000
tokens, so the number of turns drives what a review costs. In the
evaluation baseline (200 runs), a review takes a median of 8 turns, and 2
of them are spent working around codev: to write `Disabled by
configuration.`, the skill sends the agent to a second `codev roles`
call and a search for `when: never`, and to resume and check the record it
asks for `codev review --json`, which the agent then filters through
`python3` — 186 of the baseline's 266 permission denials are such
filters, each one a wasted turn. The measuring tool now exists
(`codev-evals compare`), so a gain can be proven.

## What Changes

- `codev roles --at <step>` lists, next to the active roles, the roles
  that serve the step but are disabled by configuration (`when: never`),
  with their id, name and where the setting comes from; the JSON gains a
  `disabled` array and the human output a line naming them.
- The review steps of `/codev-propose` and `/codev-apply` take the
  disabled roles from that same call, read `codev review --change <name>`
  as printed to resume and to check the record, and are told to read
  codev's output as printed rather than filter it through another program.
- The evaluation prompt stops prescribing `codev review --json`, so that
  the suite measures what the skill says.

## Capabilities

### New Capabilities

### Modified Capabilities

- roles
- skills

### Removed Capabilities

## Impact

- `crates/codev-core/src/roles.rs` (the roles disabled at a step),
  `crates/codev-engine/src/roles.rs`, `crates/codev-cli/src/commands.rs`,
  `contract.rs` (`RolesV1.disabled`, additive) and `render.rs`.
- `assets/workflows/_review-record.md` and `_isolated-review.md`, rendered
  into the `codev-propose` and `codev-apply` skills; codev's own installed
  skills refreshed with `codev update`.
- `evals/src/runner.rs` (the prompt).
- Documentation: `docs/{en,fr}/src/reference/json-output.md` and the
  `codev roles` reference in both languages.

## Domains

- [ ] security — the matches are in the evaluation baseline recorded again under `evals/baseline/`, data the suite stores about its fixtures' seeded defects; the change touches no such code
- [ ] personal-data — the matches are in the evaluation baseline recorded again under `evals/baseline/`, data the suite stores about its fixtures' seeded defects; the change touches no such code
- [ ] user-interface — the matches are in the evaluation baseline recorded again under `evals/baseline/`, data the suite stores about its fixtures' seeded defects; the change touches no such code
- [x] performance
- [ ] operations — the matches are in the evaluation baseline recorded again under `evals/baseline/`, data the suite stores about its fixtures' seeded defects; the change touches no such code
- [ ] migration — the JSON field is additive; skills and binary are updated together by `codev update`, and a skill that finds no `disabled` field reads no role as disabled

## Release note

- Changed: The review steps of `/codev-propose` and `/codev-apply` take fewer turns and ask for fewer permissions: `codev roles --at` now lists the roles disabled at the step, and the skills read codev's output as printed instead of filtering it through other programs. Measured on the evaluation suite with no change in what the roles find: permission denials down 46 %, context re-read down 16 %, turns down 6 %, time down 12 %, cost down 7 %.
