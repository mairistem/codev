## MODIFIED Requirements

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
  `change.yaml`, `review.md` and `corrections.md`, which hold the
  change's metadata, its reviews and the author's corrections, not its
  plan.
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
  spec delta, a `change.yaml`, a `review.md` and a `corrections.md`
- **WHEN** the user runs `codev roles --at plan.challenge --change <name>
  --json`
- **THEN** `review.material.files` lists the four planning files and
  neither `change.yaml`, `review.md` nor `corrections.md`

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
