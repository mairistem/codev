## MODIFIED Requirements

### Requirement: The suite covers the implementation of past changes and a refusal

The suite SHALL hold implementation cases drawn from at least three
archived changes of this repository whose acceptance scripts calibrate,
among them one fixing a defect and one adding a command, a refusal
for a change whose plan has no tasks, and a refusal for a change whose
plan is complete but whose review record holds a `high` finding still
`open` at `plan.challenge`. The baseline SHALL hold five runs
of each, recorded with the Claude Code version the evaluation workflow
pins.

#### Scenario: The implementation cases of the suite

- **GIVEN** the suite's cases
- **WHEN** the user runs `codev-evals list`
- **THEN** at least five `code.implement` cases are listed, two of them
  refusing

#### Scenario: An implementation case without its baseline runs

- **GIVEN** an implementation case with no observation in `evals/baseline/`
- **WHEN** the suite's tests run
- **THEN** they fail, naming the case

#### Scenario: The refusal of an unsettled plan review passes

- **GIVEN** the refusal case whose plan review holds an open `high`
  finding, run with the `apply` skill of this codev
- **WHEN** the user runs `codev-evals run --case <that case>`
- **THEN** the run passes: the agent modified no file outside the step
  and checked no task

### Requirement: An implementation case pins a past state, a change's plan and a hidden acceptance script

An implementation case SHALL be a directory of `evals/cases/` holding a
`case.yaml` with `step: code.implement`, `kind` (`seeded`), `ref` (a
commit of this repository, the state before the change), `merged` (the
commit that brought the change to `main`), `change` (a change name),
`summary`, and `expected`; and a folder `change/` holding the planning
files of `_codev/changes/<change>/`, as archived at `merged`. They MAY
differ from the archive only where a task's verification needs a tool
the run's sandbox does not hold, such as `mdbook`, or the codev the change
builds — `codev` on the search path is the codev under test, whose
validation and doctor judge an older change by today's rules —: that
verification is reworded into one the sandbox can run, such as `cargo run
-p codev-cli -- validate --strict`, and `summary` says so.
`expected` SHALL hold `refuse`
(whether the skill must refuse to implement), `docs` (whether the run
must touch the documentation of both languages), `acceptance` (the
names of the acceptance assertions) and, for a refusing case only,
`points_to` (the text the refusal must name, `/codev-propose` when
absent). A case that does not refuse SHALL
hold a `tasks.md` in `change/` whose tasks are all unchecked, no
`review.md`, and an `acceptance.sh` next to `case.yaml`, with at least
one assertion named in `acceptance`. A refusing case SHALL hold no
`acceptance.sh` and no assertion, and expect no documentation; its
`change/` holds either no `tasks.md` — an incomplete plan — or a
`tasks.md` whose tasks are all unchecked together with the `review.md`
that makes the skill refuse.

Loading a case SHALL fail, naming the case and the cause, when `ref` or
`merged` is not a commit of the repository's history; when the tree at
`ref` already holds `_codev/changes/<change>/` or `evals/cases/<case
id>/`; when `change/` is missing; when a non-refusing case's `tasks.md`
has a checked task or none, or its `change/` holds a `review.md`; when a
refusing case's `tasks.md` has a checked task or none, or comes without
a `review.md`; when `points_to` is set on a case that does not refuse,
or is empty; when `acceptance.sh` is missing, or present in a refusing
case; when two assertions have the same name; or when an unknown key is
found.

#### Scenario: A case replaying a change

- **GIVEN** a `case.yaml` with `step: code.implement`, `ref`, `merged`,
  `acceptance: [accept-moves-the-status]`, an `acceptance.sh`, and a
  `change/` holding a proposal, a delta spec, a design and a `tasks.md`
  whose three tasks are unchecked
- **WHEN** the suite loads its cases
- **THEN** the case loads, as a `code.implement` case

#### Scenario: A plan already implemented

- **GIVEN** a non-refusing case whose `tasks.md` has a checked task
- **WHEN** the suite loads its cases
- **THEN** it fails, naming the case and `tasks.md`

#### Scenario: A refusing case with an acceptance script

- **GIVEN** a case with `refuse: true` and an `acceptance.sh`
- **WHEN** the suite loads its cases
- **THEN** it fails, naming the case

#### Scenario: A task that validates with the change's own codev

- **GIVEN** an archived change whose last task runs `codev validate
  --strict`, which today's codev fails on its proposal written before the
  `## Domains` section existed
- **WHEN** the suite's tests run
- **THEN** they fail unless that verification is `cargo run -q -p
  codev-cli -- validate --strict` and the case's `summary` says so

#### Scenario: A refusing case with a complete plan and its record

- **GIVEN** a case with `refuse: true`, `points_to: plan_review_unsettled`,
  and a `change/` holding a `tasks.md` whose tasks are all unchecked and a
  `review.md`
- **WHEN** the suite loads its cases
- **THEN** the case loads, as a refusing `code.implement` case

#### Scenario: A refusing case with tasks but no record

- **GIVEN** a case with `refuse: true` whose `change/` holds a `tasks.md`
  and no `review.md`
- **WHEN** the suite loads its cases
- **THEN** it fails, naming the case and `review.md`

### Requirement: An implementation run is scored by deterministic checks

Scoring an implementation run SHALL apply these checks, without a model:

- every task of the case's `tasks.md` is checked in the run's, and none
  is removed;
- the format, the lints and the tests pass, and the binary builds;
- each assertion of `acceptance` is `ok`;
- the review record holds a `code.review` step;
- `codev validate` reports no error on the change that it did not report
  on the fixture before the run;
- nothing is modified outside the step, and the change's folder is still
  in `_codev/changes/`;
- the agent read and searched no file of the repository the cases come
  from, outside the case's directory;
- when `docs` is expected, a file under `docs/en/` and a file under
  `docs/fr/` changed;
- for a refusing case, instead: no path of the case's directory is
  changed, created or removed, and the agent's messages name the case's
  `points_to`, `/codev-propose` when the case sets none.

A run SHALL be given its checks passed and failed, and it passes when
every check passes. The implementation measures SHALL be, per repetition
over the implementation runs of the cases that do not refuse, the share
of runs that pass, the share of checks that pass and the share of
acceptance assertions that pass, given as their median with their
minimum and maximum; the share of refusing runs that pass, given the same
way; and the runs that modify a file outside the step. The refusing
cases are kept out of the first three shares so that a refusal, easy to
pass, does not hide a failed implementation.

#### Scenario: A run that leaves a task unchecked

- **GIVEN** a run whose code passes every assertion but whose `tasks.md`
  leaves task 2.3 unchecked
- **WHEN** the run is scored
- **THEN** the task check fails, naming 2.3, and the run does not pass

#### Scenario: A run whose tests fail

- **GIVEN** a run whose `cargo test --workspace` fails
- **WHEN** the run is scored
- **THEN** the test check fails and the run does not pass, and its
  acceptance assertions are still counted

#### Scenario: A run that reads the merged solution

- **GIVEN** a run whose agent ran `git show <merged>` in the repository
  the cases come from
- **WHEN** the run is scored
- **THEN** the check on reads outside the case's directory fails and the
  run does not pass

#### Scenario: A refusal well made

- **GIVEN** a refusing case and a run that changes no file and whose
  messages point to `/codev-propose`
- **WHEN** the run is scored
- **THEN** every check passes and the run passes

#### Scenario: A refusal that names the wrong way out

- **GIVEN** a refusing case with `points_to: plan_review_unsettled` and a
  run that changes no file and whose messages name `/codev-propose` but
  not `plan_review_unsettled`
- **WHEN** the run is scored
- **THEN** the check on what the refusal names fails, naming
  `plan_review_unsettled`, and the run does not pass

