## MODIFIED Requirements

### Requirement: `codev status` derives each artifact's state from the files present

`codev status` SHALL report, for each artifact of the change's schema, in
the schema's dependency order (ties broken by declaration order), a state
computed only from the files present in the change folder — codev keeps no
state file:

- `done` — its output exists; for a glob output such as `specs/**/*.md`, at
  least one file matches;
- `skipped` — the change sets `skip_specs: true` and the artifact writes
  under `specs/`; it counts as satisfied for the artifacts requiring it;
- `ready` — every artifact it requires is `done` or `skipped`;
- `blocked` — otherwise, with `missingDeps` naming the required artifacts
  not yet satisfied.

`applyRequires` SHALL be the schema's `apply.requires`, and
`isPlanningComplete` SHALL be `true` only when every artifact reachable from
`applyRequires` through the `requires` edges is `done` or `skipped` — an
artifact listed in `applyRequires` being `done` is not enough.

With `--json`, the document SHALL carry `changeName`, `schemaName`,
`planningHome` (the project root), `changeRoot` (the change folder),
`applyRequires`, `isPlanningComplete`, `artifacts` — each with `id`,
`outputPath` (the schema's `generates`, verbatim), `status`, `requires` and
`missingDeps` —, `planReview` when the change has a `review.md`
(whether the plan review lets the implementation start, as the review
record's requirements define it) and `status` (configuration warnings). The human output
SHALL print `Change:` and `Schema:`, one line per artifact marked `[x]`
done, `[ ]` ready, `[-]` blocked with `(waiting on: …)`, or `[~]` skipped
with `(disabled by skip_specs)`, then `Planning: <satisfied>/<total>
artifacts` and either `Next:` with the first ready artifact or
`Planning is complete.`, followed by the plan review's line when it is
not settled. The command exits with code 0, whether the plan review is
settled or not.

#### Scenario: A fresh change has only its first artifact ready

- **GIVEN** a `spec-driven` change with no artifact written
- **WHEN** the user runs `codev status --change add-auth --json`
- **THEN** `proposal` is `ready`, `specs` and `design` are `blocked` with
  `missingDeps: ["proposal"]`, and `tasks` is `blocked` with
  `missingDeps: ["specs", "design"]`
- **AND** `applyRequires` is `["tasks"]` and `isPlanningComplete` is
  `false`

#### Scenario: A written artifact unlocks the next ones

- **GIVEN** a `spec-driven` change whose `proposal.md` exists
- **WHEN** the user runs `codev status --change add-auth`
- **THEN** the output shows `[x] proposal`, `[ ] specs`, `[ ] design` and
  `[-] tasks (waiting on: specs, design)`
- **AND** it ends with `Planning: 1/4 artifacts` and `Next:     specs`

#### Scenario: A nested spec delta satisfies the glob

- **GIVEN** a change holding `proposal.md` and
  `specs/identity/user-auth/spec.md`
- **WHEN** the user runs `codev status --change add-auth --json`
- **THEN** `specs` is `done`

#### Scenario: `skip_specs` skips the spec artifact

- **GIVEN** a change with `skip_specs: true` whose `proposal.md` and
  `design.md` exist
- **WHEN** the user runs `codev status --change refactor`
- **THEN** `specs` is shown `[~]` and `(disabled by skip_specs)`
- **AND** `tasks` is `ready`

#### Scenario: Writing the tracked file first does not complete planning

- **GIVEN** a `spec-driven` change where only `tasks.md` exists
- **WHEN** the user runs `codev status --change add-auth --json`
- **THEN** `tasks` is `done`
- **AND** `isPlanningComplete` is `false`

#### Scenario: Every artifact written

- **GIVEN** a change where every artifact of its schema is written
- **WHEN** the user runs `codev status --change add-auth`
- **THEN** `isPlanningComplete` is `true` in the JSON
- **AND** the human output says `Planning is complete.` and has no `Next:`
  line

#### Scenario: Resolved paths for the skills

- **GIVEN** a project at `/p` with a change `add-auth`
- **WHEN** the user runs `codev status --json` from `/p/src`
- **THEN** `planningHome` is `/p` and `changeRoot` is
  `/p/_codev/changes/add-auth`

#### Scenario: The plan review is part of the status once reviewed

- **GIVEN** a change `add-auth` with every artifact written and a
  `review.md` whose `plan.challenge` section says `No finding.` for each
  role
- **WHEN** the user runs `codev status --change add-auth --json`
- **THEN** the document carries `planReview` with `settled` `true` and
  an empty `blockers`
- **AND** the exit code is 0

#### Scenario: No plan review field before the review

- **GIVEN** a change `add-auth` without `review.md`
- **WHEN** the user runs `codev status --change add-auth --json`
- **THEN** the document has no `planReview` key
