# Proposal: Documentation and tests in the definition of done

## Why

A plan is not complete if it leaves the documentation stale or the behavior
untested, whatever the project. Today the default workflow only asks each
task to name "its verification": nothing requires a documentation update or
an automated test, and nothing says tests must follow the conventions of the
project's language.

## What Changes

- The tech lead's "Done when" criteria of the `spec-driven` `tasks`
  instruction gain three items: the affected documentation is updated in a
  task (or the plan states none is affected); every spec scenario is covered
  by an automated test written in a task, a manual check only when
  automation is impossible and with the reason; tests follow the project's
  existing test suite, the conventions of its language, and its `rules`.
- The `proposal` instruction asks Impact to name the affected documentation.
- `/codev-propose`'s traceability check also verifies that a documentation
  task exists when Impact touches documented behavior, and that each
  scenario's covering task names an automated test.
- `/codev-apply` writes tests following the existing suite first, then the
  language's conventions and the project's `rules`; it finds the test command
  from the project instead of guessing, checks a task off only once its
  verification passes, and runs the full suite once before reporting the
  change as done.
- `/codev-configure` proposes testing rules for `tasks`, grounded in the
  project's existing tests, with a compact reference of mainstream
  conventions (Rust, TypeScript/JavaScript, Python, Go, Java) to fall back on
  when the project has no tests yet.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `skills` — the tech lead criteria of the default schema, the `propose`
  traceability check, and the `apply` verification and testing behavior.
- `configure` — testing rules among the proposed `rules:`.

### Removed Capabilities

None.

## Impact

- `assets/schemas/spec-driven/schema.yaml` (`proposal` and `tasks`
  instructions), `assets/workflows/propose.md`, `apply.md` and
  `configure.md`; the regenerated skills under `.claude/skills/`.
- Tests asserting on instruction and skill text in `codev-engine` and
  `codev-agents`.
- Documentation, in `docs/en` and `docs/fr`: `skills.md` (propose, apply,
  configure), `workflow.md` (apply step), `reference/configuration.md`
  (testing rules example); CHANGELOG.
- No change to the CLI, the JSON contract, the file formats or the init
  probe. Plans written by custom schemas are unaffected.
