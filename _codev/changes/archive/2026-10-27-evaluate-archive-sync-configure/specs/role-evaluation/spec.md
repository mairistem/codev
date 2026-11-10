## MODIFIED Requirements

### Requirement: The runner can play the review cases in the isolated mode

`codev-evals run --review-mode isolated` SHALL write `review: { mode:
isolated }` into each review case's `_codev/config.yaml` before the
skills of the codev under test are rendered, so that the skill runs the
evaluated role as its own sub-agent, as a project in that mode does; the
sub-agent tool is then granted like `codev`. `--fixture-config <file>`
SHALL append the YAML of `<file>` to each review case's
`_codev/config.yaml` at the same moment, for a setting a measure needs,
and to each implementation case's, as the `implementation-evaluation`
capability says. Planning, update, sync, archive and configure cases
ignore both options; implementation cases ignore `--review-mode`. `--review-mode` accepts
`inline` (the default) and `isolated`; an unknown mode, a
`--fixture-config` file that is missing or not a YAML mapping, or one
holding `review:` — the mode comes from `--review-mode` alone, so that
`run.json` records the mode the cases ran in —, SHALL exit with code 2
before any case runs. `run.json` SHALL record the review mode and the
appended configuration; a run resumed with other values SHALL exit with
code 2, as for another model; a `run.json` without them, of the earlier
format, reads as `inline` with nothing appended.

After an isolated case's run, the runner SHALL check that the role
actually ran alone: the review record's step says `Mode: isolated.`,
holds no `Fallback:` line, and the session started at least one
sub-agent. A case that fails the check ends in error, naming what was
missing, since the skill falls back to the inline mode by design
(decision 0007).

The run's cost and token totals SHALL be those Claude Code reports for
the whole session, its sub-agents included. The per-call measures SHALL
count the session's own calls; the tokens of the totals that the
session's calls do not account for SHALL be reported as the source
`sub-agents`, so that each token of an isolated run is traced to a source
(decision 0008). A run whose totals fall below the sum of its own calls
ends in error.

#### Scenario: A review case in the isolated mode

- **GIVEN** a review case of the `skeptic` at `plan.challenge`
- **WHEN** the user runs `codev-evals run --case <it> --review-mode
  isolated`
- **THEN** the case's configuration holds `review: { mode: isolated }`,
  and `run.json` records `isolated`

#### Scenario: A case that fell back to the inline mode

- **GIVEN** an isolated case whose review record holds `Mode: isolated.`
  and a `Fallback:` line, or whose session started no sub-agent
- **WHEN** its run is checked
- **THEN** it ends in error, naming what was missing

#### Scenario: The sub-agents' tokens

- **GIVEN** an isolated run whose session totals hold 120,000 input tokens
  and whose own calls hold 70,000
- **WHEN** its consumption is recorded
- **THEN** the source `sub-agents` holds the 50,000 others

#### Scenario: A configuration appended to every case

- **GIVEN** a file holding `roles: { skeptic: { done_when: ["The rollback
  plan is stated"] } }`
- **WHEN** the user runs `codev-evals run --step review --fixture-config
  <file>`
- **THEN** each review case's configuration ends with that text, and
  `run.json` records it

#### Scenario: Options that cannot be applied

- **GIVEN** `--review-mode sometimes`, or a `--fixture-config` file that
  does not exist, or one holding `review:`
- **WHEN** the user runs `codev-evals run`
- **THEN** it exits with code 2 before any case runs, naming the cause

#### Scenario: Resuming in another mode

- **GIVEN** an output directory recorded in the `inline` mode
- **WHEN** the user runs `codev-evals run --out <it> --review-mode
  isolated`
- **THEN** it exits with code 2, naming both modes

#### Scenario: Resuming with another appended configuration

- **GIVEN** an output directory recorded with one `--fixture-config`
- **WHEN** the user resumes it with another file's text
- **THEN** it exits with code 2

#### Scenario: A run recorded before these options

- **GIVEN** a `run.json` without a review mode nor an appended
  configuration
- **WHEN** it is read
- **THEN** it reads as `inline` with nothing appended, and resuming it
  without the options continues

#### Scenario: A sync, archive or configure case ignores both options

- **GIVEN** a `change.archive` case and a `configure.enrich` case
- **WHEN** the user runs `codev-evals run --review-mode isolated
  --fixture-config <file>` on them
- **THEN** neither case's `_codev/config.yaml` gains `review:` nor the
  file's text
