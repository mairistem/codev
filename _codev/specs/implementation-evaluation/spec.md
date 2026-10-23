# Implementation Evaluation Specification

## Purpose

The implementation evaluation measures the `/codev-apply` skill: given a
past change's plan laid on the state before it, does the agent deliver
the behavior the plan specifies, with every task done, the project's
checks passing, the code reviewed, and nothing touched it must leave.

## Requirements

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
must touch the documentation of both languages) and `acceptance` (the
names of the acceptance assertions). A case that does not refuse SHALL
hold a `tasks.md` in `change/` whose tasks are all unchecked, no
`review.md`, and an `acceptance.sh` next to `case.yaml`, with at least
one assertion named in `acceptance`. A refusing case SHALL hold no
`tasks.md`, no `acceptance.sh` and no assertion, and expect no
documentation.

Loading a case SHALL fail, naming the case and the cause, when `ref` or
`merged` is not a commit of the repository's history; when the tree at
`ref` already holds `_codev/changes/<change>/` or `evals/cases/<case
id>/`; when `change/` is missing; when a non-refusing case's `tasks.md`
has a checked task or none, or its `change/` holds a `review.md`; when
`acceptance.sh` is missing, or present in a refusing case; when two
assertions have the same name; or when an unknown key is found.

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

### Requirement: An acceptance script drives the built binary and is calibrated on the real change

An acceptance script SHALL be run by `bash`, in a new empty directory,
with the environment variable `CODEV` naming the `codev` binary to test,
under a timeout of 300 seconds; it SHALL print one line per assertion,
`ok <name>` or `not ok <name>`. An assertion of `acceptance` the script
does not print `ok` for SHALL count as failed, a timeout included.

`codev-evals calibrate <case>` SHALL build the tree at `merged` and the
tree at `ref`, each in a new directory, and run the script against the
binary each builds. It SHALL pass when, on `merged`, every assertion of
`acceptance` is `ok`, no other name is printed, and the project's checks
pass — format, lints and tests as the runner runs them —; and when, on
`ref`, at least one assertion is not `ok`. It SHALL print each assertion
on both commits and the checks on `merged`, and exit with 0 when it
passes, 1 when it does not, naming each cause, and 2 when the case
cannot be loaded or a tree cannot be built.

#### Scenario: A script that cannot tell the change apart

- **GIVEN** a case whose every assertion is `ok` on `ref` as on `merged`
- **WHEN** the user runs `codev-evals calibrate <case>`
- **THEN** it exits with 1, saying no assertion fails before the change

#### Scenario: A calibrated script

- **GIVEN** a case whose every assertion is `ok` on `merged`, one is
  `not ok` on `ref`, and whose checks pass on `merged`
- **WHEN** the user runs `codev-evals calibrate <case>`
- **THEN** it exits with 0 and prints each assertion on both commits

#### Scenario: A merged commit the checks reject

- **GIVEN** a case whose merged commit no longer passes the lints of the
  installed toolchain
- **WHEN** the user runs `codev-evals calibrate <case>`
- **THEN** it exits with 1, naming the lints

### Requirement: The runner replays the change's implementation with the skill under test, offline

For an implementation case, `codev-evals run` SHALL build the case's
directory from the tree at `ref`, write `change/` into
`_codev/changes/<change>/`, install the skills of the codev under test
as for a planning case, and commit. It SHALL then, before the agent
starts, fetch the project's dependencies into a Cargo home inside the
case's directory, which git ignores, and build the workspace and its
tests, so that the agent's builds need no network; a failure there SHALL
record the run as an error. Every command the runner starts itself on
the case's directory — this preparation, the checks below, the build of
the binary, the acceptance script, and its calls to `git` and `codev` —
SHALL run with an environment
cleared of everything but what the toolchain needs to run — the search
path, the home directory, the Rust toolchain's own variables, the
temporary directory and the locale — so that code the agent wrote never
sees the API key or any other secret of the operator's environment; and
under a timeout of 1,200 seconds each, the script's own aside, stopping
every process it started. A check that times out fails, its output
saying so; a preparation that times out records the run as an error.
Claude Code SHALL run with that Cargo home and Cargo set offline, in the
sandbox every case runs in, granted the tools the installed
`codev-apply` skill grants in its `allowed-tools` — as a user who
invokes the skill, since the agent reads the skill file instead —, with
a prompt
that names the `codev-apply` skill and the change, and the evaluation
mode: ask no question — make the most reasonable assumption —, carry the
skill through its review, and do not archive or sync. Unless
`CODEV_EVAL_BUDGET_USD` and `CODEV_EVAL_TIMEOUT_SECS` say otherwise, an
implementation case SHALL run under a budget of 30 USD and a timeout of
5,400 seconds. `--step code.implement` SHALL select the implementation
cases; `--step review` does not.

Before the run, the runner SHALL record what `codev validate <change>
--json` reports on the fixture. After a run that did not stop on an
error, the runner SHALL itself run, in the case's directory, the
project's checks — `cargo fmt --all --check`, `cargo clippy --workspace
--all-targets -- -D warnings` and `cargo test --workspace`, offline —,
build the `codev` binary, and run the acceptance script against it. The
observation SHALL hold, next to what every case records, each check's
outcome with the end of its output, each assertion's outcome, the files
of the change's folder after the run, the paths the run changed, created
or removed, what `codev validate <change> --json` and `codev review
--change <change> --json` report after the run, and the text of every
message the agent wrote, and the paths of the files the agent read or
searched outside the case's directory. A file changed under `_codev/` outside the
change's folder, and a `corrections.md` written in it, SHALL be recorded
as modified outside the step. A run whose agent changed what git reads
under `.git/` — its configuration, `commondir`, or a file of `info/`
such as the attributes — SHALL be recorded as an error naming the files;
what git writes as it works, such as the objects, the refs, the index or
`AUTO_MERGE`, is not a change of what it reads, before the runner calls `git` on it: git would
run outside the sandbox what such a file sets.

#### Scenario: A run that changes what git reads

- **GIVEN** an implementation run whose agent writes `.git/commondir`
- **WHEN** the run ends
- **THEN** it is recorded as an error naming `commondir`, and `git status`
  is not run on its directory

#### Scenario: The prompt of an implementation case

- **GIVEN** an implementation case for the change `decision-accept-command`
- **WHEN** the runner builds its prompt
- **THEN** it names `codev-apply` and the change, and says to ask no
  question, to carry the skill through its review, and not to archive or
  sync

#### Scenario: The tools of an implementation run

- **GIVEN** a fixture whose `codev-apply` skill grants `Bash(codev:*),
  Read, Write, Edit, Glob, Grep, Bash, Agent`
- **WHEN** the runner builds the Claude Code command of the run
- **THEN** it grants those tools, so that `cargo test` runs in the
  sandbox without a prompt

#### Scenario: The budget of an implementation case

- **GIVEN** no `CODEV_EVAL_BUDGET_USD` nor `CODEV_EVAL_TIMEOUT_SECS`
- **WHEN** the runner builds the Claude Code command of an
  implementation case
- **THEN** it passes a budget of 30 and waits up to 5,400 seconds

#### Scenario: Only the implementation cases

- **GIVEN** a suite of review, planning and implementation cases
- **WHEN** the user runs `codev-evals run --step code.implement`
- **THEN** only the implementation cases run

#### Scenario: A test that never ends

- **GIVEN** an implementation run whose agent wrote a test that loops
- **WHEN** the runner runs `cargo test --workspace` after the run
- **THEN** the test check fails after 1,200 seconds, saying it timed out,
  and the run is still scored

#### Scenario: The checks never see the API key

- **GIVEN** a runner started with `ANTHROPIC_API_KEY` in its environment
- **WHEN** it runs the checks and the acceptance script of a run
- **THEN** none of them has `ANTHROPIC_API_KEY` in its environment

#### Scenario: A run that syncs its change

- **GIVEN** an implementation run that writes `_codev/specs/decisions/spec.md`
- **WHEN** its observation is recorded
- **THEN** it lists that file as modified outside the step

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
  changed, created or removed, and the agent's messages name
  `/codev-propose`.

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

### Requirement: The report and the comparison give the implementation measures

`report.md` and `report.json` SHALL give, for the step `code.implement`,
the cases, the median, minimum and maximum of the share of runs that
pass, of the share of checks that pass and of the share of acceptance
assertions that pass, and the runs modifying a file outside the step;
and per implementation run, its outcome and each check, passed or
failed. A suite without implementation case SHALL give no such section.
`codev-evals compare` SHALL judge the implementation quality as it
judges the planning quality: the candidate's median share of runs that
pass, its median share of acceptance assertions that pass and its median
share of refusing runs that pass must not fall under the baseline's
minimum, and its runs modifying a file outside the step must not rise
above the baseline's maximum.

#### Scenario: One more failed implementation in most repetitions

- **GIVEN** a baseline of three implementation cases whose every
  repetition passes all three, and a candidate that fails one of them in
  three repetitions out of five
- **WHEN** the user runs `codev-evals compare <baseline> <candidate>`
- **THEN** the verdict fails, naming the pass share, 0.67 under 1.00

#### Scenario: The report of a suite with implementation cases

- **GIVEN** observations of implementation cases
- **WHEN** the user runs `codev-evals score <dir>`
- **THEN** the report has a `code.implement` section with its measures
  and, per run, each check

#### Scenario: An implementation regression

- **GIVEN** a baseline whose acceptance share is 0.90 at least, and a
  candidate at 0.70
- **WHEN** the user runs `codev-evals compare <baseline> <candidate>`
- **THEN** the verdict fails, naming the acceptance share

### Requirement: The reliability chapter gives the implementation figures

When the committed baseline holds implementation runs, the reliability
chapter that `codev-evals docs` generates SHALL count the implementation
cases with the others in its description of the run, and give a section
on implementing a plan, in English and in French: what an
implementation case replays and how a run is checked, then the median
and range of the share of runs that pass, of checks that pass, of
acceptance assertions that hold, of refusals that pass, and of the runs
that modify a file outside the step. A baseline without implementation
runs SHALL give no such section.

#### Scenario: A baseline with implementation runs

- **GIVEN** a baseline with four implementation cases among its cases
- **WHEN** the user runs `codev-evals docs`
- **THEN** both pages count `4 implementation` (`4 d'implémentation`)
  with the other cases, and give the section with its five measures

#### Scenario: A baseline without implementation runs

- **GIVEN** a baseline with no implementation case
- **WHEN** the user runs `codev-evals docs`
- **THEN** neither page has a section on implementing a plan

### Requirement: The suite covers the implementation of past changes and a refusal

The suite SHALL hold implementation cases drawn from at least three
archived changes of this repository whose acceptance scripts calibrate,
among them one fixing a defect and one adding a command, and a refusal
for a change whose plan has no tasks. The baseline SHALL hold five runs
of each, recorded with the Claude Code version the evaluation workflow
pins.

#### Scenario: The implementation cases of the suite

- **GIVEN** the suite's cases
- **WHEN** the user runs `codev-evals list`
- **THEN** at least four `code.implement` cases are listed, one of them
  refusing

#### Scenario: An implementation case without its baseline runs

- **GIVEN** an implementation case with no observation in `evals/baseline/`
- **WHEN** the suite's tests run
- **THEN** they fail, naming the case
