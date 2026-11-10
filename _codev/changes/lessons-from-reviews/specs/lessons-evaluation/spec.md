## Purpose

The lessons evaluation measures the lessons mode of `/codev-configure`:
given an archive of past changes, does the agent propose the lessons that
recur, for an acceptable destination, leave out what is covered, declined
or seen once, and write nothing before the author agrees.

## ADDED Requirements

### Requirement: A lessons case pins a past state and the lessons expected from its archive

A lessons case SHALL be a directory of `evals/cases/` holding a
`case.yaml` with `step: configure.lessons`, `kind` (`seeded`), `ref` (a
commit of this repository), `last` (the window the mode reads, a positive
integer), `summary` and `expected`; and an optional folder `overlay/`
whose files are laid on the tree at `ref`, relative to the project root,
as the `overlay/` of a sync or archive case is, to add what the case needs — a ledger, a proposed decision, an archived
change with a `corrections.md`. `expected` SHALL hold `lessons`, a list
of expected lessons, each with `origins` (at least one
`<archived change folder>/<id>`, any of which identifies it) and
`destinations` (the acceptable destinations, among `check`, `decision`,
`rule:specs`, `rule:design`, `rule:tasks`, `context`, `none` and
`covered`); and `absent`, origins no proposed lesson may name unless as
`covered` or `none`. A case SHALL expect at least one lesson or one
absent origin.

Loading a case SHALL fail, naming the case and the cause, when `ref` is
not a commit of the repository's history; when a path of `overlay/` is a
symbolic link, or would be laid outside the case's directory; when an
origin's folder is not
an archived change of the tree at `ref` with its overlay; when a
destination is unknown; when `last` is not a positive integer; or when an
unknown key is found.

#### Scenario: A case expecting a recurring lesson

- **GIVEN** a `case.yaml` with `step: configure.lessons`, `last: 10`, and
  one expected lesson whose `origins` name two findings of archived
  changes present at `ref`, with `destinations: [decision, rule:design]`
- **WHEN** the suite loads its cases
- **THEN** the case loads, as a `configure.lessons` case

#### Scenario: An overlay that leaves the tree

- **GIVEN** a case whose `overlay/` holds a symbolic link to
  `../../outside`
- **WHEN** the suite loads its cases
- **THEN** it fails, naming the case and the link

#### Scenario: An origin the archive does not hold

- **GIVEN** a case whose expected origin is
  `2026-01-01-never-archived/R-skeptic-1`
- **WHEN** the suite loads its cases
- **THEN** it fails, naming the case and that folder

### Requirement: The runner replays the lessons mode with the skill under test

For a lessons case, `codev-evals run` SHALL build the case's directory
from the tree at `ref`, lay `overlay/` on it, install the skills of the
codev under test as for a planning case, and commit. It SHALL then run
Claude Code with a prompt that names `/codev-configure lessons <last>`
and the evaluation mode: ask no question; present the lessons and the
diff as the skill says, then stop, since no one will agree. It runs under
the planning cases' budget and timeout. `--step configure.lessons`
SHALL select the lessons cases; no other step selects them.

The observation SHALL hold the text of every message the agent wrote,
the files modified in the directory, and the consumption of the run as
for every other step.

#### Scenario: The prompt of a lessons case

- **GIVEN** a lessons case with `last: 30`
- **WHEN** the runner builds its prompt
- **THEN** it names `/codev-configure lessons 30`, and says to ask no
  question and to stop once the lessons and the diff are shown

#### Scenario: A run that writes without agreement

- **GIVEN** a lessons run that writes `_codev/config.yaml`
- **WHEN** its observation is recorded
- **THEN** it lists `_codev/config.yaml` as modified

### Requirement: A lessons run is scored by deterministic checks

Scoring a lessons run SHALL read, in the agent's messages, the lesson
lines `L-<n> [<destination>] <lesson> — from <origins>`, a line
`[covered by <what>]` counting as the destination `covered`, and apply
these checks, without a model:

- each expected lesson is found: a lesson line names at least one of its
  origins, with a destination among its `destinations`;
- each `absent` origin is named by no lesson line whose destination is
  other than `covered` or `none`;
- no file of the directory is modified.

A run SHALL be given its checks passed and failed, as a run of the other
check-scored steps is, and it passes when every check passes. The lessons measures SHALL be, per repetition over
the lessons runs, the share of runs that pass, the share of expected
lessons found (recall) and the number of lesson lines naming no expected
origin (unexpected lessons), given as their median with their minimum and
maximum; and the runs that modify a file.

#### Scenario: A lesson found for another destination

- **GIVEN** an expected lesson with `destinations: [check]`, and a run
  whose line names one of its origins as `[rule:tasks]`
- **WHEN** the run is scored
- **THEN** that lesson's check fails and the run does not pass

#### Scenario: A declined lesson proposed again

- **GIVEN** a case whose `absent` names a finding the overlay's ledger
  declined, and a run that proposes it as `[decision]`
- **WHEN** the run is scored
- **THEN** the absent check fails

#### Scenario: A covered lesson shown as covered

- **GIVEN** an expected lesson with `destinations: [covered]`, and a run
  whose line names its origin with `[covered by 0013]`
- **WHEN** the run is scored
- **THEN** the check passes

### Requirement: The report and the comparison give the lessons measures

`report.md` and `report.json` SHALL give, for the step
`configure.lessons`, the cases, the median, minimum and maximum of the
share of runs that pass, of the recall and of the unexpected lessons, the
runs modifying a file, the cost per run, and per run its outcome and
each check. A suite without lessons case SHALL give no such section.
`codev-evals compare` SHALL fail the verdict when the candidate's share
of runs that pass or its recall falls under the baseline's minimum, or
when its runs modifying a file rise above the baseline's maximum.

#### Scenario: The report of a suite with lessons cases

- **GIVEN** observations of lessons cases
- **WHEN** the user runs `codev-evals score <dir>`
- **THEN** the report has a `configure.lessons` section with its
  measures and, per run, each check

#### Scenario: A suite without lessons cases

- **GIVEN** observations of planning cases only
- **WHEN** the user runs `codev-evals score <dir>`
- **THEN** the report has no `configure.lessons` section

#### Scenario: A lessons regression

- **GIVEN** a baseline whose lessons recall is 0.83 at least, and a
  candidate at 0.50
- **WHEN** the user runs `codev-evals compare <baseline> <candidate>`
- **THEN** the verdict fails, naming the lessons recall

### Requirement: The suite covers the lessons mode's main behaviors

The suite SHALL hold lessons cases drawn from this repository's archive
covering, at least: a recurring finding for a decision or a rule, a
recurrence a lint can enforce, a recurrence a decision already holds, a
lesson the ledger declined, a finding seen in one change only, and a
lesson from a single correction. The baseline SHALL hold five runs of
each, recorded with the Claude Code version the evaluation workflow pins.

#### Scenario: The lessons cases of the suite

- **GIVEN** the suite's cases
- **WHEN** the user runs `codev-evals list`
- **THEN** six `configure.lessons` cases are listed, covering the
  behaviors above

#### Scenario: A lessons case without its baseline runs

- **GIVEN** a lessons case with no observation in `evals/baseline/`
- **WHEN** the suite's tests run
- **THEN** they fail, naming the case
