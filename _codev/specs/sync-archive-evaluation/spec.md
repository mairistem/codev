# Sync Archive Evaluation Specification

## Purpose

The sync and archive evaluation measures the `/codev-sync` and
`/codev-archive` skills: given an active change, does the agent merge or
archive it through codev and report what codev did, and stop where the
skill must stop, without writing the specs or moving the change itself.

## Requirements

### Requirement: A sync or archive case pins a past state, an active change and the outcome expected

A sync or archive case SHALL be a directory of `evals/cases/` holding a
`case.yaml` with `step` (`change.sync` or `change.archive`), `kind`
(`seeded`), `ref` (a commit of this repository), `change` (a change
name), `summary` and `expected`; a folder `change/` holding the files of
`_codev/changes/<change>/`, laid on the state at `ref`; and an optional
folder `overlay/`, whose files are laid on the tree at `ref`, relative to
the project root, replacing those already there, for what the case
needs, such as a configuration that sets `codev_version`. `expected`
SHALL hold `outcome` — `done` when the skill must leave the project as
codev's own command leaves it, `refused` when it must leave the project
as it found it — and MAY hold `code`, for a `refused` case that codev
itself refuses, the code codev's own command reports (`status[0].code`);
`contains`, texts the agent's messages must hold; and `absent`, texts
they must not hold.

Loading a case SHALL fail, naming the case and the cause, when `ref` is
not a commit of the repository's history; when the tree at `ref` already
holds `_codev/changes/<change>/` or `evals/cases/<case id>/`; when
`change/` is missing; when a path of `overlay/` is a symbolic link, lies
under `_codev/changes/<change>/`, or would be laid outside the case's
directory; when `outcome` is neither `done` nor `refused`; when `code`
is given for a `done` case; when a text
of `contains` or `absent` is empty; or when an unknown key is found.

#### Scenario: An archive case

- **GIVEN** a `case.yaml` with `step: change.archive`, `outcome: done`,
  `contains: ["Moved to"]`, and a `change/` holding a proposal, a delta
  spec, a design and tasks
- **WHEN** the suite loads its cases
- **THEN** the case loads, as a `change.archive` case

#### Scenario: An overlay that leaves the tree

- **GIVEN** a sync case whose `overlay/` holds a symbolic link to
  `../../outside`
- **WHEN** the suite loads its cases
- **THEN** it fails, naming the case and the link

#### Scenario: An outcome the suite does not know

- **GIVEN** an archive case with `outcome: skipped`
- **WHEN** the suite loads its cases
- **THEN** it fails, naming the case and `skipped`

### Requirement: The runner replays the skill and records what codev's own command does

For a sync or archive case, `codev-evals run` SHALL build the case's
directory from the tree at `ref`, write `change/` into
`_codev/changes/<change>/`, lay `overlay/` on it, install the skills of
the codev under test as for a planning case, and commit.

Before the agent runs, the runner SHALL make the reference: in a copy of
that directory, outside the case's directory, it runs `codev sync
--change <change> --json` or `codev archive --change <change> --json`
with the codev under test, and records its exit code, its JSON report,
and the path and a digest of the content of every file under `_codev/`
it leaves. The reference runs with the environment the runner already
gives codev, and calls no model.

It SHALL then run Claude Code with a prompt that names the `codev-sync`
or `codev-archive` skill and the change, and the evaluation mode: ask no
question — make the most reasonable assumption —, and, if the skill asks
whether to update codev, take the author's answer **Later**. The prompt
says nothing of what the skill must write or refuse: the checks measure
what the skill makes the agent do. Unless `CODEV_EVAL_BUDGET_USD` and
`CODEV_EVAL_TIMEOUT_SECS` say otherwise, a sync or archive case SHALL run
under a budget of 3 USD and a timeout of 600 seconds. `--step
change.sync` and `--step change.archive` SHALL select those cases;
`--step review` selects neither.

The observation SHALL hold the reference, the text of every message the
agent wrote, and the path and a digest of every file under `_codev/`
after the run. A file modified, created or removed outside
`_codev/specs/` by a sync run, or outside `_codev/specs/` and
`_codev/changes/` by an archive run, SHALL be recorded as modified
outside the step.

#### Scenario: The prompt of an archive case

- **GIVEN** an archive case for the change `add-auth`
- **WHEN** the runner builds its prompt
- **THEN** it names `codev-archive` and `add-auth`, says to ask no
  question and to answer **Later** to an update question, and says
  nothing of what to write or refuse

#### Scenario: The reference of a sync case

- **GIVEN** a sync case whose change modifies one requirement of
  `_codev/specs/doctor/spec.md`
- **WHEN** the runner prepares the case
- **THEN** the observation holds the exit code and the report of `codev
  sync --change <change> --json` run on a copy, and the digest of the
  copy's `_codev/specs/doctor/spec.md` after it
- **AND** the case's own directory is as the fixture's commit left it
  when the agent starts

#### Scenario: A sync run that edits the code

- **GIVEN** a sync run that writes `src/lib.rs`
- **WHEN** its observation is recorded
- **THEN** it lists `src/lib.rs` as modified outside the step

#### Scenario: An archive run that moves the change

- **GIVEN** an archive run that updates `_codev/specs/doctor/spec.md`
  and moves the change under `_codev/changes/archive/`, and a sync run
  that edits `_codev/changes/<change>/tasks.md`
- **WHEN** their observations are recorded
- **THEN** the archive run lists nothing as modified outside the step,
  and the sync run lists `_codev/changes/<change>/tasks.md`

#### Scenario: Only the sync cases

- **GIVEN** a suite of review, update, sync and archive cases
- **WHEN** the user runs `codev-evals run --step change.sync`
- **THEN** only the sync cases run; with `--step change.sinc`, it exits
  with 2 naming the steps

### Requirement: A sync or archive run is scored by deterministic checks

Scoring a sync or archive run SHALL apply these checks, without a
model:

- for a `done` case, every file under `_codev/` after the run is the
  reference's — the same paths with the same content, the date that
  starts the name of an archived change's folder aside;
- for a `refused` case, every file under `_codev/` after the run is the
  fixture's;
- no file is modified outside the step;
- each `contains` text is in the agent's messages, and no `absent` text
  is, compared ignoring case and runs of whitespace.

A run SHALL be given its checks passed and failed, and it passes when
every check passes. A `done` case whose reference run did not exit with
0 SHALL end in error, naming the case and the code of the reference's
report, rather than be scored: the case no longer describes what codev
does. So SHALL a `refused` case with a `code` whose reference reports
another code, or exits with 0, naming the case and both codes. The measures of each step SHALL be, per repetition over its runs,
the share of runs that pass and the share of checks that pass, given as
their median with their minimum and maximum; and the runs that modify a
file outside the step.

#### Scenario: A merge written by hand

- **GIVEN** a `done` sync case, and a run that edits
  `_codev/specs/doctor/spec.md` itself, leaving a requirement worded
  differently from the reference's
- **WHEN** the run is scored
- **THEN** the check on the files under `_codev/` fails and the run does
  not pass

#### Scenario: A refusal well made

- **GIVEN** a `refused` archive case whose change has a validation
  error, `contains: ["codev validate"]`, and a run that changes nothing
  and replies with the skill's sentence naming `codev validate`
- **WHEN** the run is scored
- **THEN** every check passes and the run passes

#### Scenario: An archive made on another day

- **GIVEN** a `done` archive case whose reference moved the change to
  `_codev/changes/archive/2026-10-27-add-auth/`, and a run that moved it,
  with the same files, to `_codev/changes/archive/2026-10-28-add-auth/`
- **WHEN** the run is scored
- **THEN** the check on the files under `_codev/` passes

#### Scenario: A text the reply must not hold

- **GIVEN** a case with `absent: ["ready to be archived"]`, and a run
  whose reply suggests the change is ready to be archived
- **WHEN** the run is scored
- **THEN** the check on that text fails

#### Scenario: A case codev no longer completes

- **GIVEN** a `done` archive case whose reference exits with 1 and the
  code `validation_failed`
- **WHEN** the run is scored
- **THEN** it ends in error, naming the case and `validation_failed`

#### Scenario: A refusal for another reason than the case's

- **GIVEN** a `refused` archive case with `code: review_blocked`, whose
  reference exits with 1 and the code `validation_failed`
- **WHEN** the run is scored
- **THEN** it ends in error, naming the case, `review_blocked` and
  `validation_failed`

### Requirement: The report, the comparison and the reliability chapter give the sync and archive measures

`report.md` and `report.json` SHALL give, for each of the steps
`change.sync` and `change.archive`, the cases, the median, minimum and
maximum of the share of runs that pass and of the share of checks that
pass, and the runs modifying a file outside the step; and per run, its
outcome and each check, passed or failed. A suite without a case of a
step SHALL give no section for it. `codev-evals compare` SHALL judge each
step as it judges the update step: the verdict fails when the
candidate's share of runs that pass falls under the baseline's minimum,
or its runs modifying a file outside the step rise above the baseline's
maximum, naming the step and both values; and when a case's share of
candidate runs that pass falls under that case's share in the baseline,
rescored with the case as it now stands, naming the case and both
values. The report SHALL name every case of these steps whose candidate
runs do not all pass. Scoring reads only the observations, so a case
whose expectation changes is judged by scoring the baseline again,
without a model. The reliability chapter SHALL
give each step's measures, in both languages, when the baseline holds
its cases.

#### Scenario: The report of a suite with archive cases

- **GIVEN** observations of archive cases
- **WHEN** the user runs `codev-evals score <dir>`
- **THEN** the report has a `change.archive` section with its measures
  and, per run, each check

#### Scenario: A suite without sync cases

- **GIVEN** observations of review and archive cases only
- **WHEN** the user runs `codev-evals score <dir>`
- **THEN** the report has no `change.sync` section

#### Scenario: An archive regression

- **GIVEN** a baseline whose archive runs pass at 0.83 at least, and a
  candidate at 0.50
- **WHEN** the user runs `codev-evals compare <baseline> <candidate>`
- **THEN** the verdict fails, naming `change.archive`, 0.50 and 0.83

#### Scenario: One case regresses inside the step's range

- **GIVEN** a baseline where every archive case passes in its five runs,
  and a candidate where `archive-review-blocked` passes in one of three
  runs and every other case in all of its runs
- **WHEN** the user runs `codev-evals compare <baseline> <candidate>`
- **THEN** the verdict fails, naming `archive-review-blocked`, 0.33 and
  1.00

#### Scenario: A case whose expectation changed

- **GIVEN** an archive case whose `contains` gained a text after the
  baseline was recorded, the baseline scored again, and a candidate
  whose runs of that case all hold the text
- **WHEN** the user runs `codev-evals compare <baseline> <candidate>`
- **THEN** that case decides nothing against the verdict, and the
  report names no case of the step as not passing

#### Scenario: The reliability chapter with archive cases

- **GIVEN** a baseline holding sync and archive runs
- **WHEN** the user runs `codev-evals docs`
- **THEN** both pages give the `change.sync` and `change.archive`
  measures

### Requirement: The suite covers the sync and archive skills' main paths

The suite SHALL hold archive cases covering, at least: an archive that
merges and moves a change; an archive of a change whose tasks are not
all checked; a refusal on a validation error; a refusal by the review
record; a stop on incomplete planning; and an archive under a
`codev_version` the running codev does not satisfy, the update check
answered **Later**. It SHALL hold sync cases covering, at least: a sync
that merges; a refusal on a validation error; a stop on incomplete
planning; and a sync under a `codev_version` the running codev does not
satisfy, answered **Later**. Each change is drawn from this repository's
archive. The baseline SHALL hold five runs of each, recorded with the
Claude Code version the evaluation workflow pins.

#### Scenario: The sync and archive cases of the suite

- **GIVEN** the suite's cases
- **WHEN** the user runs `codev-evals list`
- **THEN** six `change.archive` and four `change.sync` cases are listed,
  covering the paths above

#### Scenario: An archive case without its baseline runs

- **GIVEN** an archive case with no observation in `evals/baseline/`
- **WHEN** the suite's tests run
- **THEN** they fail, naming the case
