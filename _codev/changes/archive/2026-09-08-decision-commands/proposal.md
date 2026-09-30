# Proposal: CLI commands for decisions

## Why

The repository's six ADRs were written by hand, in files created with
`touch`. It worked because the format is simple — but it does not scale:
a user who wants to create a seventh decision has to remember the
four-digit prefix, the mandatory frontmatter, the file slug. The previous
change made decisions **usable** by the tool; this change makes them
**manageable** by it.

## What Changes

- **`codev decision new <title>`** — creates a new ADR under
  `_codev/decisions/`. Automatically determines the `id` (the largest
  integer found + 1, formatted on four digits), derives the file name from
  the title (`0007-title-slug.md`), writes a skeleton with the standard
  sections (Context / Decision / Consequences / Rejected alternatives).
  Default status `accepted` — the repository's documented practice.
- **`codev decision list`** — lists all local and inherited decisions,
  each with its `id`, its title, its status, its effect state
  (`in_effect` or `superseded_by:<id>`), and its origin.
- **`codev decision show <id>`** — displays one specific decision, human
  or JSON output. Resolves by local `id` if unambiguous, requires an
  `<origin>/<id>` qualifier in case of a cross-source collision.
- **`codev decision supersede <old-id> <new-title>`** — creates a new
  decision that references the old one in its `supersedes`, and rewrites
  the old one's frontmatter to switch its `status` to `superseded`. Both
  writes are in the same `Plan`: atomic.
- **Extended JSON contract** — new `DecisionV1` types (list, show,
  new, supersede) in `contract::v1`, in camelCase.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `decisions` — the change adds four new requirements describing the
  CLI commands. The existing capability (parser, index, injection into
  `design`) remains unchanged.

## Impact

- **Code**: new module `codev-engine::decisions::actions` with pure
  functions `plan_new(index, title, options) -> Result<Plan, …>` and
  `plan_supersede(index, old_id, new_title) -> Result<Plan, …>`; new
  `codev decision …` subcommand group in `codev-cli`; new contracts
  `DecisionV1`, `DecisionCreatedV1`, `DecisionSupersededV1`.
- **Dependencies**: none new.
- **ADR skeleton**: embedded in the binary via `include_str!`, under
  `assets/templates/decision.md`. A project that wants a different
  skeleton will have to write it by hand for now — templating this file is
  a future change if the need arises.
- **Out of scope**:
  - **K3 (immutability)** — detecting the modification of an `accepted`
    decision requires a stored reference hash. That work is a follow-up
    change, now less blocked since mutations will go through the CLI.
  - **K6 (`deviates-from` deviation)** — a project inheriting a decision
    may want to depart from it locally. A dedicated frontmatter field and
    a `codev decision deviate <origin/id>` command would be clean
    additions, to be handled separately.
  - **K7 (promotion from `design.md`)** — recognizing an architecture
    decision in an archived `design.md` and promoting it to an ADR, next
    batch.
  - **Interactive editing** in an `$EDITOR` after creation — the tool
    just writes the file; the user opens it themselves. Doing it properly
    requires a `ProcessRunner` port for the editor, which does not exist
    yet.
