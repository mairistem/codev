## MODIFIED Requirements

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
`_codev/changes/` SHALL be recorded as modified outside the step, and so
SHALL a `corrections.md` written under `_codev/changes/`: no author
corrects the agent in an evaluation run, so a correction noted there is
one the agent invented.

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

#### Scenario: A planning run that invents a correction

- **GIVEN** a planning run that writes its change and a
  `corrections.md` in it
- **WHEN** its observation is recorded
- **THEN** it lists that `corrections.md` as modified outside the step
