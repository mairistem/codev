## Purpose

Lets a project choose whether its reviewing roles run in the
conversation that wrote the work (`inline`) or each on its own, given
only its role and the material (`isolated`), and tells the skills which
mode is in effect and how much material the reviewers would read.

## ADDED Requirements

### Requirement: The project and a change set the review mode

`_codev/config.yaml` SHALL accept a `review:` mapping whose `mode` is
`inline` or `isolated`. A change's `change.yaml` SHALL accept the same
`review:` mapping. The effective mode of a change SHALL be the change's
`review.mode` when set, otherwise the project's, otherwise `inline`; its
origin is `change`, `project` or `default` accordingly. Without a change,
the effective mode is the project's, or `inline`. An inherited source's
`review:` MUST NOT be read: the mode belongs to the team writing in the
repository.

A `mode` other than `inline` or `isolated`, in either file, SHALL make
every command that reads that file fail with the stable code
`invalid_review_mode`, naming the file, the value and the allowed values.
A `review:` that is not a mapping, or a `mode` that is not text, fails
the same way. An unknown key under `review:` in `change.yaml` fails as
any unknown key of that file does.

#### Scenario: The default mode is inline

- **GIVEN** a project whose configuration has no `review:` key and a
  change `add-auth` whose `change.yaml` has none either
- **WHEN** the user runs `codev roles --at code.review --change add-auth
  --json`
- **THEN** `review.mode` is `inline` and `review.origin` is `default`

#### Scenario: The project sets isolated

- **GIVEN** a configuration with `review: { mode: isolated }`
- **WHEN** the user runs `codev roles --at plan.challenge --json`
- **THEN** `review.mode` is `isolated` and `review.origin` is `project`

#### Scenario: A change overrides the project

- **GIVEN** a configuration with `review: { mode: isolated }` and a change
  `fix-typo` whose `change.yaml` sets `review: { mode: inline }`
- **WHEN** the user runs `codev roles --at code.review --change fix-typo
  --json`
- **THEN** `review.mode` is `inline` and `review.origin` is `change`

#### Scenario: An unknown mode in the configuration

- **GIVEN** a configuration with `review: { mode: parallel }`
- **WHEN** the user runs `codev status --json`
- **THEN** the exit code is 1 and `status[0].code` is
  `invalid_review_mode`
- **AND** the message names `parallel`, `inline` and `isolated`

#### Scenario: An unknown mode in a change

- **GIVEN** a change whose `change.yaml` sets `review: { mode: alone }`
- **WHEN** the user runs `codev roles --at code.review --change <name>
  --json`
- **THEN** the exit code is 1 and `status[0].code` is
  `invalid_review_mode`, naming `change.yaml`

#### Scenario: An inherited source's mode is not read

- **GIVEN** a project inheriting a source whose configuration sets
  `review: { mode: isolated }`, and no `review:` key of its own
- **WHEN** the user runs `codev roles --at code.review --json`
- **THEN** `review.mode` is `inline` and `review.origin` is `default`

### Requirement: `codev roles` measures the material of a reviewing step

`codev roles --json` SHALL carry a `review` object with `mode`, `origin`
and `material`; in a failure's document, `review` is `null`. `material` SHALL be `null` unless both `--change` and an
`--at` among `plan.trace`, `plan.challenge` and `code.review` are given.
Then it SHALL list what the reviewers of that step read, each file once,
as `files`, each with its `path` relative to the project root,
`/`-separated, its size in bytes, and its `treatment`, `full` or
`summarize`; and carry `limitBytes`, `totalBytes` (the sum of the sizes)
and `summarized` (whether any file is to be summarized).

- At `plan.trace` and `plan.challenge`, the material is the change's
  planning files: every file of the change's directory except
  `change.yaml` and `review.md`.
- At `code.review`, the material is the files of the change's diff, as
  the `domain-suggestions` capability reads it, other than planning
  files, vendored and generated files, lock files, binary files and
  anything that is not a regular file, such as a symbolic link; a deleted
  file counts zero bytes. When the diff is unavailable, `material` is
  `null`.

The treatment SHALL be decided without judgement: files are taken from
the smallest to the largest, ties in path order, and each is `full` as
long as the running sum stays within the limit; every other file is
`summarize`. The limit is 163840 bytes (160 KiB). Measuring reads file
sizes, never their content, and writes nothing.

#### Scenario: Small material is read in full

- **GIVEN** a change `add-auth` whose diff modifies two source files of
  1200 and 800 bytes
- **WHEN** the user runs `codev roles --at code.review --change add-auth
  --json`
- **THEN** `review.material.files` lists both with `treatment: "full"`,
  `totalBytes` is 2000 and `summarized` is `false`

#### Scenario: Material over the limit is partly summarized

- **GIVEN** a change whose diff modifies files of 100000, 60000 and 10000
  bytes
- **WHEN** the user runs `codev roles --at code.review --change <name>
  --json`
- **THEN** the 10000- and 60000-byte files are `full`, the 100000-byte
  file is `summarize`, and `summarized` is `true`

#### Scenario: The plan is the material of the challenge

- **GIVEN** a change with `proposal.md`, `design.md`, `tasks.md`, one
  spec delta, a `change.yaml` and a `review.md`
- **WHEN** the user runs `codev roles --at plan.challenge --change <name>
  --json`
- **THEN** `review.material.files` lists the four planning files and
  neither `change.yaml` nor `review.md`

#### Scenario: No material without a reviewing step

- **WHEN** the user runs `codev roles --at code.implement --change <name>
  --json`, or `codev roles --json`
- **THEN** `review.material` is `null` and `review.mode` is given

#### Scenario: No diff, no material

- **GIVEN** a project outside a git repository
- **WHEN** the user runs `codev roles --at code.review --change <name>
  --json`
- **THEN** `review.material` is `null`

#### Scenario: Noise is not material

- **GIVEN** a change whose diff modifies `Cargo.lock`, a binary image, a
  symbolic link `docs/notes.md` and `src/lib.rs`
- **WHEN** the user runs `codev roles --at code.review --change <name>
  --json`
- **THEN** `review.material.files` lists only `src/lib.rs`

#### Scenario: A failure carries no review

- **WHEN** the user runs `codev roles --at plan.review --json`
- **THEN** the exit code is 1 and `review` is `null`
