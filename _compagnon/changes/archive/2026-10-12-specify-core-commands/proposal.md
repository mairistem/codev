# Proposal: Specify the core commands that predate codev's own specs

## Why

The commands built before codev developed itself with codev — `new change`,
`list`, `status`, `instructions`, `schemas`, `decision show` and `update` —
have no dedicated requirement, so nothing says what someone changing them
must preserve. The specs must cover the whole tool before the real-world
workflows start relying on it.

## What Changes

- A new `changes` capability specifies `codev new change` (naming rules,
  refusal of an existing name, the `change.yaml` it writes, `--goal`, JSON),
  `codev list` (active changes, `--specs`), `codev status` (artifact states
  derived from the schema graph and the files present, `applyRequires`,
  `isPlanningComplete`, `planningHome`/`changeRoot`, human and JSON output)
  and how the commands taking `--change` pick the change.
- A new `instructions` capability specifies `codev instructions`: the
  artifact chosen, the fields returned (instruction, template, language,
  resolved output path, context and rules blocks with their origin and
  order, dependencies, unlocks, skipped), the human output and the errors.
- A new `schemas` capability specifies `codev schemas`, the resolution of a
  schema (project before built-in), what makes a schema valid, and how
  `--schema`, the `schema:` key of the configuration and `change.yaml`
  decide the schema of a change.
- The `decisions` capability's `decision show` requirement is completed with
  its human output, an inherited decision and its JSON document.
- The `skills` capability gains a requirement for what `codev update` does
  with the skills of the configured workflows; what it does with managed
  sections stays in `init`, and the preservation of a hand-edited skill
  stays in the skill frontmatter requirement.
- Behavior is described as it is today. No code changes; the tests that are
  missing to cover every scenario are added. Behaviors that look like bugs
  are left out of the specs and reported for their own change.
- Documentation (English and French) is corrected where it misses what the
  specs state: error codes of `new change`, `status` and `instructions`,
  the `schema_unusable` warning of `codev schemas`, the output of
  `codev decision show`.

## Capabilities

### New Capabilities

- `changes` — `codev new change`, `codev list`, `codev status`, and the
  choice of the change for the commands taking `--change`.
- `instructions` — `codev instructions`.
- `schemas` — `codev schemas`, schema resolution and validity, the schema of
  a change.

### Modified Capabilities

- `decisions` — the output of `codev decision show`.
- `skills` — `codev update` and the skills of the configured workflows.

### Removed Capabilities

None.

## Impact

- `_codev/specs/`: three new specs (`changes`, `instructions`, `schemas`),
  `decisions` and `skills` updated.
- Tests only, in `codev-core`, `codev-engine`, `codev-agents` and
  `codev-cli`: no production code changes.
- Documentation in `docs/en` and `docs/fr`: reference/cli.md,
  reference/json-output.md, guides/custom-schemas.md.
- The status requirements already specified elsewhere — the `codev doctor`
  hint (`doctor`) and the update notice (`self-update`) — are unchanged.

## Release note

None
