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
`missingDeps` — `tasks`, and `status` (configuration warnings). `tasks`
SHALL be `null` while the file named by the schema's `apply.tracks` is
absent from the change folder, and otherwise `{ "checked", "total",
"groups" }`, each group being `{ "id", "title", "checked", "total" }`,
counted as the `task-progress` capability defines; a tracked file without
any box gives `{ "checked": 0, "total": 0, "groups": [] }`, and one that
cannot be read gives `null` with a `tasks_unreadable` warning in `status`.
The key is always present. The human output
SHALL print `Change:` and `Schema:`, one line per artifact marked `[x]`
done, `[ ]` ready, `[-]` blocked with `(waiting on: …)`, or `[~]` skipped
with `(disabled by skip_specs)`, then `Planning: <satisfied>/<total>
artifacts` and either `Next:` with the first ready artifact or
`Planning is complete.`, then, only when the tracked file exists, a last
line `Tasks:    <checked>/<total> checked`, or `Tasks:    no box in
<tracked file>` when it holds none, or `Tasks:    <tracked file>
unreadable` when it cannot be read. The command exits with code 0, whatever
the boxes.

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


#### Scenario: No task progress before the tracked file exists

- **GIVEN** a `spec-driven` change whose `proposal.md` exists and whose
  `tasks.md` does not
- **WHEN** the user runs `codev status --change add-auth --json`
- **THEN** `tasks` is `null`
- **AND** the human output of `codev status --change add-auth` has no line
  starting with `Tasks:`

#### Scenario: Task progress once the tracked file exists

- **GIVEN** a change where every artifact is written and `tasks.md` holds
  three boxes, two of them checked
- **WHEN** the user runs `codev status --change add-auth`
- **THEN** the output ends with `Planning is complete.` followed by
  `Tasks:    2/3 checked`
- **AND** the JSON's `tasks.checked` is 2 and `tasks.total` is 3

#### Scenario: A tracked file without any box

- **GIVEN** a change whose `tasks.md` holds only a heading and a paragraph
- **WHEN** the user runs `codev status --change add-auth --json`
- **THEN** `tasks` is `{ "checked": 0, "total": 0, "groups": [] }`
- **AND** the human output ends with `Tasks:    no box in tasks.md`
