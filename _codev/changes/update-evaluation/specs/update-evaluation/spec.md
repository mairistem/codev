## Purpose

The update evaluation measures the `/codev-update` skill: given an active
change and the author's request to revise one of its artifacts, does the
agent revise what it should, leave the rest, report the ripple instead of
applying it, and refuse what the skill must refuse.

## ADDED Requirements

### Requirement: An update case pins a past state, an active change and a revision request

An update case SHALL be a directory of `evals/cases/` holding a
`case.yaml` with `step: plan.update`, `kind` (`seeded`), `ref` (a commit
of this repository), `change` (a change name), `artifact` (`proposal`,
`specs`, `design` or `tasks`), `request` (the author's words, in any
language), `summary`, and `expected`; and a folder `change/` holding the
files of `_codev/changes/<change>/`, laid on the state at `ref`.
`expected` SHALL hold `revised` (the paths, relative to the change's
folder, the revision must change), `kept` (the paths it must leave
byte-identical), `ripple` (the paths whose consequence the agent must
name in its reply without editing them), `contains` (text the revised
files must hold once revised) and `refuse` (whether the skill must
refuse the request). A refusing case expects no `revised` path and no
`contains`; any other case expects at least one `revised` path.

Loading a case SHALL fail, naming the case and the cause, when `ref` is
not a commit of the repository's history; when the tree at `ref` already
holds `_codev/changes/<change>/` or `evals/cases/<case id>/`; when
`change/` is missing; when a path of `revised`, `kept` or `ripple` is not
a file of `change/`; when the request is empty; or when an unknown key is
found.

#### Scenario: A case revising a design

- **GIVEN** a `case.yaml` with `step: plan.update`, `artifact: design`,
  `revised: [design.md]`, `kept: [proposal.md, tasks.md]`, and a
  `change/` holding those three files
- **WHEN** the suite loads its cases
- **THEN** the case loads, as a `plan.update` case

#### Scenario: A path the change does not hold

- **GIVEN** a case whose `kept` names `notes.md`, absent from its
  `change/`
- **WHEN** the suite loads its cases
- **THEN** it fails, naming the case and `notes.md`

#### Scenario: A refusing case that expects a revision

- **GIVEN** a case with `refuse: true` and `revised: [design.md]`
- **WHEN** the suite loads its cases
- **THEN** it fails, naming the case

### Requirement: The runner replays the revision request with the skill under test

For an update case, `codev-evals run` SHALL build the case's directory
from the tree at `ref`, write `change/` into `_codev/changes/<change>/`,
install the skills of the codev under test as for a planning case, and
commit. It SHALL then run Claude Code with a prompt that names the
`codev-update` skill, the change, the artifact and the request, and the
evaluation mode: ask no question — make the most reasonable assumption —;
report a ripple as the skill says, without applying it, since no one
will confirm; modify nothing outside the change's folder; and end with a
reply that states what was revised and the ripple found. It runs under
the planning cases' budget and timeout. `--step plan.update` SHALL
select the update cases; `--step review` does not.

The observation SHALL hold the files of the change's folder after the
run, the agent's final reply, and what `codev validate <change> --json`
reports. A file modified outside the change's folder, and a
`corrections.md` written in it, SHALL be recorded as modified outside the
step: no author corrects the agent in an evaluation run.

#### Scenario: The prompt of an update case

- **GIVEN** an update case for the change `add-auth`, the artifact
  `design` and the request « remplace la décision X par Y »
- **WHEN** the runner builds its prompt
- **THEN** it names `codev-update`, the change, the artifact and the
  request, and says to ask no question, to report a ripple without
  applying it, and to modify nothing outside the change's folder

#### Scenario: A run that edits the code

- **GIVEN** an update run that writes `src/lib.rs`
- **WHEN** its observation is recorded
- **THEN** it lists `src/lib.rs` as modified outside the step

### Requirement: An update run is scored by deterministic checks

Scoring an update run SHALL apply these checks, without a model:

- each `revised` path differs from the case's `change/`;
- each `kept` path is byte-identical to it, and so is each `ripple` path;
- each `ripple` path is named in the agent's final reply, by its path
  relative to the change's folder;
- each `contains` text is in one of the `revised` files once revised,
  compared ignoring case and runs of whitespace;
- `codev validate` reports no error on the change;
- for a refusing case: no file of the change's folder is created, changed
  or removed, and the reply names `/codev-propose`.

A run SHALL be given its checks passed and failed, and it passes when
every check passes. The update measures SHALL be, per repetition over the
update runs, the share of runs that pass and the share of checks that
pass, given as their median with their minimum and maximum; and the runs
that modify a file outside the step.

#### Scenario: A revision that also edits the ripple

- **GIVEN** a case with `revised: [proposal.md]` and
  `ripple: [specs/doctor/spec.md]`, and a run that changes both files
- **WHEN** the run is scored
- **THEN** the check on `specs/doctor/spec.md` fails and the run does not
  pass

#### Scenario: A refusal well made

- **GIVEN** a refusing case for a `design` the change does not have, and
  a run that creates nothing and whose reply points to `/codev-propose`
- **WHEN** the run is scored
- **THEN** every check passes and the run passes

#### Scenario: Expected text written differently

- **GIVEN** a case whose `contains` is `Sessions live in Redis`, and a
  revised design holding `sessions live in   Redis`
- **WHEN** the run is scored
- **THEN** the check passes

### Requirement: The report and the comparison give the update measures

`report.md` and `report.json` SHALL give, for the step `plan.update`, the
cases, the median, minimum and maximum of the share of runs that pass
and of the share of checks that pass, and the runs modifying a file
outside the step; and per update run, its outcome and each check, passed
or failed. A suite without update case SHALL give no such section.
`codev-evals compare` SHALL judge the update quality as it judges the
planning quality: the candidate's share of runs that pass must not fall
under the baseline's minimum, and its runs modifying a file outside the
step must not rise above the baseline's maximum.

#### Scenario: The report of a suite with update cases

- **GIVEN** observations of update cases
- **WHEN** the user runs `codev-evals score <dir>`
- **THEN** the report has a `plan.update` section with its measures and,
  per run, each check

#### Scenario: An update regression

- **GIVEN** a baseline whose update runs pass at 0.83 at least, and a
  candidate at 0.50
- **WHEN** the user runs `codev-evals compare <baseline> <candidate>`
- **THEN** the verdict fails, naming the update pass rate

### Requirement: The suite covers the update skill's main behaviors

The suite SHALL hold update cases drawn from archived changes covering,
at least: a proposal revision whose ripple reaches a delta spec, a delta
spec revision, a design revision, a tasks revision, a revision whose
ripple reaches the tasks, and a refusal for an artifact the change does
not have. The baseline SHALL hold their runs.

#### Scenario: The update cases of the suite

- **GIVEN** the suite's cases
- **WHEN** the user runs `codev-evals list`
- **THEN** six `plan.update` cases are listed, covering the behaviors
  above

#### Scenario: An update case without its baseline runs

- **GIVEN** an update case with no observation in `evals/baseline/`
- **WHEN** the suite's tests run
- **THEN** they fail, naming the case
