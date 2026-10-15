## Purpose

Measure how well `/codev-propose` plans a change and what it consumes doing
so — which existing requirements it modifies, which decisions it follows,
whether its plan validates — by replaying requests against this
repository's own history, so that a change to the skills is shown to use
fewer tokens without planning worse.

## ADDED Requirements

### Requirement: A planning case pins a past state of the repository, a request and what the real change did

A planning case SHALL be a directory of `evals/cases/` holding only a
`case.yaml` with `step: plan.write`, `kind` (`seeded`), `ref` (a commit
of this repository: the state the request is planned against), `request`
(the user's words, in any language), `summary`, and `expected`:
`modified` and `collateral`, each a list of
`<capability>::<requirement name>`, and `decisions`, a list of entries
with `id` and `expect` (`follow` or `supersede`). `modified` holds the
requirements the request is about; `collateral` the ones the real change
also modified in passing. A case SHALL expect at least one requirement
or one decision.

Loading a case SHALL fail, naming the case and the cause, when `ref` is
not a commit of the repository's history; when a requirement of
`modified` or `collateral` is not in `_codev/specs/<capability>/spec.md`
at `ref`; when a decision id is not one of `_codev/decisions/` at `ref`;
when the tree at `ref` holds `evals/cases/<case id>/`, which would hand
the agent the answer; or when an unknown key or an empty request is
found. `codev-evals list` SHALL show planning cases with their step, and
`codev-evals run` and `score` SHALL exit with code 2 on an invalid case,
as for the other cases.

#### Scenario: A case replaying an archived change

- **GIVEN** a `case.yaml` with `ref` the commit before the change
  `decision-accept-command` was merged, a request in French, and
  `modified: ["decisions::Creating a decision through the CLI"]`
- **WHEN** the suite loads its cases
- **THEN** the case loads, as a `plan.write` case

#### Scenario: A requirement that did not exist yet

- **GIVEN** a case whose `modified` names a requirement absent from the
  specs at its `ref`
- **WHEN** the suite loads its cases
- **THEN** it fails, naming the case and the requirement

#### Scenario: A commit outside the history

- **GIVEN** a case whose `ref` is not a commit of the repository, as in a
  shallow clone
- **WHEN** the suite loads its cases
- **THEN** it fails, naming the case and the `ref`, and saying the full
  history is needed

#### Scenario: A state that holds the answer

- **GIVEN** a case whose `ref` is a commit where `evals/cases/<its id>/`
  already exists
- **WHEN** the suite loads its cases
- **THEN** it fails, naming the case

### Requirement: The suite covers planning with past changes and with decisions

The repository's suite SHALL hold at least 17 planning cases that replay
archived changes, each expecting at least one requirement under
`modified`, and at least 4 planning cases that each expect a decision,
among them one to supersede. A request SHALL be written as a user would
ask, not copied from the archived proposal.

#### Scenario: The repository's planning suite

- **GIVEN** `evals/cases/`
- **WHEN** the test suite of `codev-evals` runs
- **THEN** it fails if there are fewer than 17 planning cases with a
  `modified` requirement, or fewer than 4 with a decision, or none with
  a decision to supersede

#### Scenario: A request copied from the proposal

- **GIVEN** a planning case whose `request` repeats a sentence of more
  than twelve words of the archived change's proposal
- **WHEN** the test suite of `codev-evals` runs
- **THEN** it fails, naming the case

### Requirement: The runner replays the request on the pinned state with the skills under test

For a planning case, `codev-evals run` SHALL build the case's directory
from the tree at `ref` (`git archive`), make it a new git repository with
one commit, install the skills of the codev under test into it (`codev
update` with that binary, forced over skills edited since), and commit
them. It SHALL then run Claude Code as for the other cases, with a
prompt that names the `codev-propose` skill and the request, and the
evaluation mode: ask no question — make the most reasonable assumption
and state it in the proposal —, write the proposal, the delta specs and
the design, then stop before the tasks, the traceability check and the
challenge, and modify no file outside `_codev/changes/`. Unless
`CODEV_EVAL_BUDGET_USD` and `CODEV_EVAL_TIMEOUT_SECS` say otherwise, a
planning case SHALL run under a budget of 6 USD and a timeout of 1,800
seconds. `codev-evals run --step <step>` (repeatable) SHALL select the
cases of the steps named, `review` naming the three review steps, so that
a change can be measured on the steps it touches; an unknown step SHALL
exit with code 2, naming the valid ones.

The observation SHALL hold, next to what every case records, the files
of every change the run created under `_codev/changes/`, and what `codev
validate` reports on each, as JSON. A file modified outside
`_codev/changes/` SHALL be recorded as modified outside the step.

#### Scenario: The prompt of a planning case

- **GIVEN** a planning case whose request is « Je voudrais qu'une
  décision naisse en brouillon »
- **WHEN** the runner builds its prompt
- **THEN** the prompt names `codev-propose`, quotes the request, and says
  to ask no question, to stop before the tasks, the traceability check
  and the challenge, and to modify nothing outside `_codev/changes/`

#### Scenario: A planning run that edits the code

- **GIVEN** a planning run that writes `crates/codev-core/src/lib.rs`
  besides its change
- **WHEN** its observation is recorded
- **THEN** it lists `crates/codev-core/src/lib.rs` as modified outside
  the step, and the report shows it for that run

#### Scenario: The budget of a planning case

- **GIVEN** no `CODEV_EVAL_BUDGET_USD` nor `CODEV_EVAL_TIMEOUT_SECS`
- **WHEN** the runner builds the Claude Code command of a planning case
- **THEN** it passes a budget of 6 and waits up to 1,800 seconds; a
  review case keeps 3 and 600

#### Scenario: Only the planning cases

- **GIVEN** a suite of review and planning cases
- **WHEN** the user runs `codev-evals run --step plan.write`
- **THEN** only the planning cases run; with `--step review`, only the
  review cases; with `--step plan.writ`, it exits with 2 naming the steps

#### Scenario: The skills of the codev under test

- **GIVEN** a planning case whose `ref` predates a change to the skills
- **WHEN** the runner builds its fixture
- **THEN** the fixture's specs and decisions are those at `ref`, and its
  skills are the ones the codev under test renders

### Requirement: A plan is scored on the requirements it modifies, its added duplicates, its decisions and its validity

Scoring a planning run SHALL read the deltas of the change it created
with codev's own parser, whatever the language of their keywords. An
expected requirement SHALL count as modified when a delta of its
capability holds it under `MODIFIED` or `REMOVED`, or as the `FROM` of a
`RENAMED`, its name compared ignoring case and runs of whitespace. Each
run SHALL be given:

- the primary recall: the share of `modified` the plan modifies;
- the collateral recall: the share of `collateral` it modifies, reported
  apart and never counted as a miss of the primary recall;
- the probable duplicates: the requirements the plan adds under `ADDED`
  in a capability where an expected requirement is left unmodified, or
  whose name is the name of a requirement the capability already had at
  `ref`;
- the decisions ignored: each expected decision the proposal and the
  design do not cite, by its id or its title; a decision to `supersede`
  also counts as ignored when no paragraph citing it says the plan
  supersedes or deviates from it;
- whether the plan validates: `codev validate` reports no error on it.

A run that created no change SHALL be a failed run (`no-plan`), with
every expected requirement missed; a run that created several changes is
scored on all of them together. The planning measures SHALL be computed
per repetition over the planning runs, then given as their median with
their minimum and maximum, as the role measures are; they have no
threshold of their own and are judged by `codev-evals compare` against
the baseline.

#### Scenario: A requirement modified, another added beside it

- **GIVEN** a case expecting `decisions::Creating a decision through the
  CLI` and a plan whose delta of `decisions` modifies that requirement
  and adds `Accepting a decision through the CLI`
- **WHEN** the run is scored
- **THEN** its primary recall is 1.00 and it has no probable duplicate

#### Scenario: An addition where a modification was due

- **GIVEN** the same case and a plan whose delta of `decisions` only
  adds `Creating a draft decision`
- **WHEN** the run is scored
- **THEN** its primary recall is 0.00 and it has one probable duplicate

#### Scenario: Keywords in French

- **GIVEN** a plan whose delta writes `## Exigences MODIFIÉES` and
  `### Exigence : Creating a decision through the CLI`
- **WHEN** the run is scored
- **THEN** the requirement counts as modified

#### Scenario: A decision to supersede, only cited

- **GIVEN** a case expecting decision `0007` to be superseded, and a
  design that cites `0007` without saying it supersedes or deviates from
  it
- **WHEN** the run is scored
- **THEN** one decision is ignored

#### Scenario: A run that plans nothing

- **GIVEN** a planning run that ended without creating a change
- **WHEN** the run is scored
- **THEN** it is a failed run, `no-plan`, and the suite exits with 1
  naming the case and the repetition

### Requirement: The report gives the planning measures by step

`report.md` and `report.json` SHALL give, for the step `plan.write`, the
cases and the median, minimum and maximum over the repetitions of the
primary recall, the collateral recall, the probable duplicates, the
decisions ignored and the plans that do not validate, next to the roles;
and per planning run, its outcome, the expected requirements found and
missed, the duplicates, the decisions ignored, and its consumption,
tool calls and orientation included. A suite without planning case SHALL
give no planning section.

#### Scenario: A suite with both kinds of cases

- **GIVEN** observations of review cases and of planning cases
- **WHEN** the user runs `codev-evals score <dir>`
- **THEN** the report has the per-role section and a `plan.write`
  section, and the roles' figures are those of the review cases alone

#### Scenario: Only review cases

- **GIVEN** observations of review cases only
- **WHEN** the report is written
- **THEN** it has no `plan.write` section
