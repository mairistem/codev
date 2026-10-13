## Purpose

Measures what each reviewing role finds, misses and invents, on small
projects with seeded defects and clean controls, so that a change to a
role is judged on scores rather than on a reading of its text.

## ADDED Requirements

### Requirement: A case declares its fixture, its step, its role and what it expects

The repository SHALL hold an evaluation suite under `evals/cases/`, one
directory per case, named by the case's id. A case holds:

- `case.yaml` — `role` (the role evaluated), `step` (`plan.challenge`,
  `code.review` or `plan.trace`), `change` (the change's name), `stacks`
  (the fixture's languages), `kind` (`seeded` or `control`), a one-line
  `summary`, and either `expected` (for `plan.challenge` and
  `code.review`) or `checks` (for `plan.trace`);
- `base/` — the fixture project as it is on its default branch;
- `change/` — the files the change adds or modifies, its planning
  artifacts under `_codev/changes/<change>/` included.

An expected finding states its `id`, `severity`, `category` and one or
more `locations`, in the review record's location syntax: `path:line` or
`path:start-end` relative to the project for code, `<file>#<heading>` or
`<file>` relative to the change's directory for the plan. A `plan.trace`
case states `checks`: `fixed` entries (a file of the fixture and a text
it must contain after the run) and `unchanged` paths (files the run must
leave as they were).

A case SHALL be refused, naming the case and the reason, when: a key is
unknown; the role does not serve the step; a `seeded` case expects
nothing or a `control` expects something; an expected location names a
file absent from the fixture, a line beyond its end or a heading it does
not have; a fixture file is not UTF-8 text or is larger than 64 KiB.

#### Scenario: The repository's suite is well formed

- **GIVEN** the cases under `evals/cases/`
- **WHEN** the test suite of `codev-evals` runs
- **THEN** every case loads, and every expected location exists in its
  fixture

#### Scenario: An unknown key

- **GIVEN** a `case.yaml` with the key `expect` instead of `expected`
- **WHEN** the case is loaded
- **THEN** it is refused, and the message names `expect`

#### Scenario: A location beyond the end of the file

- **GIVEN** a seeded case expecting `src/users.ts:120` and a fixture
  whose `src/users.ts` has 40 lines
- **WHEN** the case is loaded
- **THEN** it is refused, and the message names the file and the line

#### Scenario: A binary fixture file

- **GIVEN** a case whose `change/` holds a file that is not UTF-8 text
- **WHEN** the case is loaded
- **THEN** it is refused, and the message names the file

### Requirement: The suite covers every reviewing role

The suite SHALL hold, for each expert (`security`, `privacy`,
`accessibility`, `performance`, `ops`, `migration`), at least two seeded
cases and one control; for the `newcomer` and the `skeptic`, at least two
seeded cases and one control; for the `tracer`, at least one seeded case
and one control. Among the cases, the stacks SHALL include Rust,
TypeScript and Python.

#### Scenario: The repository's suite meets the minimum

- **GIVEN** the cases under `evals/cases/`
- **WHEN** the test suite of `codev-evals` runs
- **THEN** it passes

#### Scenario: A role without a control

- **GIVEN** a suite where the `ops` role has two seeded cases and no
  control
- **WHEN** its coverage is checked
- **THEN** the check fails and names `ops` and the missing control

### Requirement: The runner runs each case alone, headlessly, with Claude Code

`codev-evals run` SHALL, for each selected case, in a new directory
outside the repository:

- create a git repository whose branch `main` holds `base/` with codev
  initialized (configuration and skills, non-interactive), and a branch
  holding `change/` on top of it, both committed;
- write the change's `change.yaml` so that every reviewing role serving
  the case's step other than the evaluated one is set to `never` — the
  evaluated role reviews alone;
- run Claude Code non-interactively in that directory with a prompt that
  names the skill step to run and the evaluation mode: at
  `plan.challenge` and `code.review`, write the findings to the review
  record and leave every finding `open`, without the author's answer and
  without modifying any other file; at `plan.trace`, fix the gaps in the
  plan as the step says;
- record what it observed: the review record as `codev review --json`
  reports it, the files the case checks, the files the run modified
  outside what the step allows, the cost and duration the run reports,
  or the error that stopped it.

The model SHALL be the one named by the environment variable
`CODEV_EVAL_MODEL`, else Claude Code's default. Each case SHALL run under
a budget (`CODEV_EVAL_BUDGET_USD`, default 3) and a timeout
(`CODEV_EVAL_TIMEOUT_SECS`, default 600). Before any case runs, the
runner SHALL check that `claude`, `git` and the `codev` binary can be
executed, and exit with code 2 naming the missing one otherwise. A case
that fails or times out is recorded as an error; the others still run.

#### Scenario: Only the evaluated role reviews

- **GIVEN** a `code.review` case evaluating `security`, at a step where
  `newcomer` and `security` serve
- **WHEN** the runner prepares the change
- **THEN** its `change.yaml` sets `newcomer` to `never` and leaves
  `security` as configured

#### Scenario: The prompt names the step and the evaluation mode

- **GIVEN** a `plan.challenge` case on the change `add-sessions`
- **WHEN** the runner builds its prompt
- **THEN** the prompt names the `codev-propose` skill, its
  `plan.challenge` step, the change `add-sessions`, and says to leave
  every finding `open` and to modify no file but the review record

#### Scenario: The model and the budget come from the environment

- **GIVEN** `CODEV_EVAL_MODEL=claude-sonnet-4-5` and
  `CODEV_EVAL_BUDGET_USD=1.5`
- **WHEN** the runner builds the Claude Code command
- **THEN** it passes that model and that budget; without the variables,
  it passes no model and a budget of 3

#### Scenario: A run that strays outside the step

- **GIVEN** a `code.review` case whose run modified `src/users.ts` and
  wrote `review.md`
- **WHEN** its observation is recorded
- **THEN** it lists `src/users.ts` as modified outside the step, not
  `review.md`, and the report shows it for that case

#### Scenario: Claude Code is not installed

- **GIVEN** a `claude` command that cannot be executed
- **WHEN** the user runs `codev-evals run`
- **THEN** it exits with code 2 before setting up any case, and the
  message names `claude`

### Requirement: A finding matches an expected finding by role and location

A finding of the review record SHALL match an expected finding when it
was written by the expected finding's role (its `### <role-id>` section)
and its location matches one of the expected locations:

- code: the same path — `\` read as `/`, a leading `./` ignored — and
  line ranges that overlap once the expected range is widened by two
  lines on each side;
- plan: the same file and, when the expected location names a heading,
  the same heading, compared ignoring case and runs of whitespace; an
  expected location without a heading matches any location in that file.

Each expected finding is matched at most once, by the first matching
finding in record order, expected findings taken in case order. A
matched expected finding is a **true positive**; one left unmatched is a
**miss**. The severity of a true positive is compared with the expected
one and counted as **severity agreement**; it does not decide the match.

#### Scenario: Overlapping lines

- **GIVEN** an expected `security` finding at `src/users.ts:12-14` and a
  `security` finding at `src/users.ts:15`
- **WHEN** the case is scored
- **THEN** the expected finding is a true positive

#### Scenario: Lines too far away

- **GIVEN** an expected `security` finding at `src/users.ts:12-14` and a
  `security` finding at `src/users.ts:30`
- **WHEN** the case is scored
- **THEN** the expected finding is a miss, and the finding is a false
  positive

#### Scenario: A plan heading written in another case and spacing

- **GIVEN** an expected finding at
  `design.md#Decision: Tokens are stored as given` and a finding at
  `design.md#decision:  tokens are stored as given`
- **WHEN** the case is scored
- **THEN** the expected finding is a true positive

#### Scenario: A Windows path

- **GIVEN** an expected finding at `src/db/orders.rs:20-24` and a finding
  at `src\db\orders.rs:22`
- **WHEN** the case is scored
- **THEN** the expected finding is a true positive

### Requirement: What a role invents is counted, and only that

A finding that matches no expected finding SHALL be:

- a **false positive** of its role when its confidence is `high`;
- **unreported** when its confidence is `low` — the human never sees it —
  counted apart, not as a false positive;
- a **duplicate**, not a false positive, when it matches an expected
  finding of its role already matched;
- **cross-role**, not a false positive, when it matches an expected
  finding of another role.

In a `control` case, every finding with a high confidence is a false
positive.

#### Scenario: A clean control

- **GIVEN** a `control` case whose record holds one `privacy` finding of
  high confidence and one of low confidence
- **WHEN** the case is scored
- **THEN** `privacy` has one false positive and one unreported finding

#### Scenario: The same defect raised twice

- **GIVEN** an expected finding and two findings of its role that both
  match it
- **WHEN** the case is scored
- **THEN** there is one true positive, one duplicate, and no false
  positive

### Requirement: Evidence validity comes from codev's own check

For each finding, the evidence SHALL be counted as `found` when
`codev review` reports no evidence or location issue for it, `moved` when
it reports `review_evidence_moved`, `invalid` when it reports
`review_evidence_not_found` or `review_location_missing`, and `unchecked`
when it reports `review_evidence_unchecked`. A role's evidence validity
is (`found` + `moved`) divided by every finding but the `unchecked` ones;
a role with no checked finding has no validity, which no threshold
fails.

#### Scenario: A quote that is not in the code

- **GIVEN** a finding for which `codev review` reports
  `review_evidence_not_found`
- **WHEN** the case is scored
- **THEN** its evidence is `invalid`, and its role's validity counts it

#### Scenario: A quote a few lines away

- **GIVEN** a finding for which `codev review` reports
  `review_evidence_moved`
- **WHEN** the case is scored
- **THEN** its evidence is `moved` and counts as valid

### Requirement: A case whose role did not review is a failed case, not a clean one

A case SHALL be marked with its reason, and every expected finding
counted as a miss, when the run stopped with an error, when the record is
missing, when the record has an error that keeps codev from reading it,
or when the evaluated role's section is missing or reads `Disabled by
configuration.` Such a case is never counted as a clean control.

#### Scenario: The evaluated role was disabled

- **GIVEN** a seeded `security` case whose record's `security` section
  reads `Disabled by configuration.`
- **WHEN** the case is scored
- **THEN** the case is marked `role-not-run`, and its expected finding
  is a miss

#### Scenario: The run timed out

- **GIVEN** a case whose observation records a timeout error
- **WHEN** the case is scored
- **THEN** the case is marked `error` with the message, and the suite
  fails

### Requirement: The tracer is scored on the plan it leaves

At `plan.trace` the tracer fixes the gaps it finds instead of recording
them. A `fixed` check SHALL be a true positive when the file contains its
text after the run, ignoring case, and a miss otherwise; an `unchanged`
path SHALL be a false positive of the tracer when its content changed,
ignoring line endings.

#### Scenario: The missing task was added

- **GIVEN** a seeded `tracer` case whose check expects `tasks.md` to
  contain `expired link`, and a `tasks.md` that names "the expired link
  scenario" after the run
- **WHEN** the case is scored
- **THEN** the check is a true positive

#### Scenario: A complete plan rewritten

- **GIVEN** a `control` `tracer` case whose `tasks.md` must stay
  unchanged, and a run that edited it
- **WHEN** the case is scored
- **THEN** the tracer has one false positive

### Requirement: The report is per role, in markdown and JSON, and the exit code follows thresholds

`codev-evals run` and `codev-evals score` SHALL write `report.md` and
`report.json` in their output directory, with, per role: the cases, the
expected findings, true positives, misses, false positives, unreported,
duplicates, cross-role findings, recall (true positives over expected),
evidence validity and severity agreement; and per case: its outcome,
each finding's classification, the findings whose status is no longer
`open` and the files modified outside the step. The model, date and
versions of the run are those recorded when the run started. The order is fixed — roles and cases by
id — so that the same observations give the same bytes.

A role SHALL fail when its recall is under `--min-recall` (default 0.5),
its false positives exceed `--max-false-positives` (default 2), or its
evidence validity is under `--min-evidence` (default 0.8); each threshold
can also be set by `CODEV_EVAL_MIN_RECALL`,
`CODEV_EVAL_MAX_FALSE_POSITIVES` and `CODEV_EVAL_MIN_EVIDENCE`, the flag
winning. The command SHALL exit with 0 when every role passes and no case
is in error, 1 otherwise, naming each failing role and measure, and 2 on
an invalid threshold, an invalid case or a missing tool.

#### Scenario: Every role passes

- **GIVEN** observations where each role's recall, false positives and
  evidence validity meet the thresholds
- **WHEN** the user runs `codev-evals score <dir>`
- **THEN** it exits with 0 and writes both reports

#### Scenario: A role under the recall threshold

- **GIVEN** observations where `ops` finds one of four expected findings
- **WHEN** the user runs `codev-evals score <dir> --min-recall 0.5`
- **THEN** it exits with 1, and the message names `ops`, its recall
  0.25 and the threshold

#### Scenario: An invalid threshold

- **GIVEN** `CODEV_EVAL_MIN_RECALL=lots`
- **WHEN** the user runs `codev-evals score <dir>`
- **THEN** it exits with 2 and names the variable

### Requirement: Recorded runs are scored again without a model, and resumed

Each case's observation SHALL be written as `<out>/<case>/observed.json`,
carrying a format marker. `codev-evals score <dir>` SHALL score the
observations of a directory again, calling no model, and give the same
report as the run did. `codev-evals run --out <dir>` SHALL skip a case
whose observation already exists in `<dir>`, unless `--rerun` is given,
so that an interrupted run is resumed; it SHALL refuse, with code 2 and
naming both models, to resume a directory recorded with another model than
the one asked. An observation whose format marker
is above the one the runner knows SHALL be refused with a message naming
the case.

#### Scenario: Scoring twice gives the same report

- **GIVEN** a directory of recorded observations
- **WHEN** the user runs `codev-evals score <dir>` twice
- **THEN** both runs write identical `report.json` and `report.md`

#### Scenario: A resumed run

- **GIVEN** an output directory holding the observations of three cases
  out of five
- **WHEN** the user runs `codev-evals run --out <dir>`
- **THEN** only the two other cases are run

#### Scenario: Resuming with another model

- **GIVEN** an output directory recorded with Claude Code's default model
- **WHEN** the user runs `codev-evals run --out <dir>` with
  `CODEV_EVAL_MODEL=claude-sonnet-4-5`
- **THEN** it exits with code 2 and names both models

#### Scenario: A newer observation

- **GIVEN** an `observed.json` with format 2
- **WHEN** the user runs `codev-evals score <dir>`
- **THEN** it exits with 2, naming the case and the format

### Requirement: A committed baseline is the reference for role changes

The repository SHALL hold a baseline under `evals/baseline/`: the
observations of one run of the whole suite against the current roles,
and the report scored from them, with the model and the date of the run.
Scoring the baseline's observations again SHALL give its committed
report.

#### Scenario: The baseline agrees with the scoring

- **GIVEN** `evals/baseline/`
- **WHEN** the test suite of `codev-evals` runs
- **THEN** scoring its observations gives its committed `report.json`

#### Scenario: A case missing from the baseline

- **GIVEN** a case added to the suite after the baseline was recorded
- **WHEN** the test suite of `codev-evals` runs
- **THEN** it fails and names the case, so that the baseline is recorded
  again

### Requirement: The suite runs in CI only on demand, and only with a key

The repository SHALL hold a workflow `.github/workflows/evals.yml`
started by hand only (`workflow_dispatch`), which runs the suite with the
`ANTHROPIC_API_KEY` secret and keeps its output as a workflow artifact.
When the secret is absent, the workflow SHALL end successfully after
printing that the evaluations were skipped and why.

#### Scenario: Never on a pull request

- **GIVEN** `.github/workflows/evals.yml`
- **WHEN** the test suite of `codev-evals` runs
- **THEN** it fails if the workflow has a trigger other than
  `workflow_dispatch`

#### Scenario: No secret

- **GIVEN** the workflow and a repository without `ANTHROPIC_API_KEY`
- **WHEN** the test suite of `codev-evals` runs
- **THEN** it fails unless the workflow tests the secret before running
  the suite and prints a skip message

### Requirement: A role change comes with its scores

`CONTRIBUTING.md` and `CONTRIBUTING.fr.md` SHALL say that a pull request
modifying a file of `assets/roles/` runs the evaluations and pastes the
scores before and after the change. The Roles chapter of each language
SHALL say how the built-in roles are measured and where the baseline is.

#### Scenario: Both languages say it

- **GIVEN** the two contributing guides and the two `roles.md` chapters
- **WHEN** the test suite of `codev-evals` runs
- **THEN** it fails if one of them does not name `evals/baseline`, or if
  a contributing guide does not name `assets/roles/`

#### Scenario: A guide that forgets the rule

- **GIVEN** a French contributing guide that does not name
  `assets/roles/`
- **WHEN** the documentation check runs
- **THEN** it fails and names `CONTRIBUTING.fr.md`
