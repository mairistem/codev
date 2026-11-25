## MODIFIED Requirements

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
- run it as a user who works with Claude Code's sandbox would: the
  shell commands Claude Code approves in its sandbox, and `codev`, which
  the skills grant, run without a prompt, in a sandbox that lets them
  write only in the case's directory and the temporary directory and
  reach no host but the Anthropic API; file edits inside the case's
  directory are accepted; any other call — a write elsewhere, a network
  tool, a program neither approves — is denied and counted;
  the runner SHALL check, before any case runs, that the sandbox can
  run on the machine — on Linux, `bubblewrap` and `socat` — and exit with
  code 2 naming what is missing, rather than run without it;
- record what it observed: the review record as `codev review --json`
  reports it, the files the case checks, the files the run modified
  outside what the step allows, what the run consumed as Claude Code
  reports it — the cost, the duration and the API duration, the number
  of turns, the input tokens read without the cache, written to the
  cache and read from the cache, and the output tokens, each summed over
  every model the run used, the models it used, and the number of
  permission denials — or the error that stopped it. A measure Claude
  Code does not report is recorded as absent, never as zero.

With `--repeat N` (default 1, at least 1), the runner SHALL run each
selected case N times, each repetition in a directory of its own, and
record each repetition's observation separately. A value of `--repeat`
that is not a whole number of at least 1 SHALL exit with code 2 before
any case runs, naming the flag.

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

#### Scenario: A compound command is not denied

- **GIVEN** a case whose agent runs `cd src && cat users.ts | head -20`
- **WHEN** the runner runs the case
- **THEN** the command runs, and the observation records 0 permission
  denials

#### Scenario: A write outside the case and a network call are denied

- **GIVEN** a case whose agent runs `echo x > $HOME/probe.txt`, then
  `curl https://example.com`, then writes `$HOME/probe2.txt` with its
  file tool
- **WHEN** the runner runs the case
- **THEN** none of the three takes effect, and the observation records
  3 permission denials

#### Scenario: A machine without the sandbox

- **GIVEN** a Linux machine where `socat` is not installed
- **WHEN** the user runs `codev-evals run`
- **THEN** it exits with code 2 before setting up any case, and the
  message names the sandbox and `socat`

#### Scenario: What a run consumed is recorded

- **GIVEN** a run for which Claude Code reports 9 turns, and per model:
  `claude-opus-5-5` with 14 uncached input tokens, 29 349 written to the
  cache, 304 094 read from it and 2 713 output tokens, and
  `claude-haiku-4-5-20251001` with 1 173 uncached input tokens and 15
  output tokens
- **WHEN** its observation is recorded
- **THEN** it holds 9 turns, 1 187 uncached input tokens, 29 349 written
  to the cache, 304 094 read from it and 2 728 output tokens, and both
  models, next to the cost and the durations

#### Scenario: A run whose output reports no usage

- **GIVEN** a run that times out before Claude Code prints its result
- **WHEN** its observation is recorded
- **THEN** its tokens and turns are absent, not zero, and the case is in
  error

#### Scenario: Each case repeated

- **GIVEN** two selected cases
- **WHEN** the user runs `codev-evals run --repeat 3`
- **THEN** six runs are made, and each case holds three observations

#### Scenario: An invalid repetition count

- **WHEN** the user runs `codev-evals run --repeat 0`
- **THEN** it exits with code 2 before setting up any case, and the
  message names `--repeat`

### Requirement: The report is per role, in markdown and JSON, and the exit code follows thresholds

`codev-evals run` and `codev-evals score` SHALL write `report.md` and
`report.json` in their output directory, with, per role: the cases, the
expected findings, true positives, misses, false positives, unreported,
duplicates, cross-role findings, recall (true positives over expected),
evidence validity and severity agreement; and per case: its outcome,
each finding's classification, the findings whose status is no longer
`open` and the files modified outside the step. The model, date and
versions of the run are those recorded when the run started, and the
report SHALL say how many repetitions it holds. The order is fixed — roles and cases by
id, repetitions by number — so that the same observations give the same bytes.

Each role's measures SHALL be computed for each repetition over that
repetition's cases, and the report SHALL give, for each measure, its
median over the repetitions and, when there are several, its minimum and
maximum. The suite-level figures — recall over every seeded defect,
evidence validity over every checked finding, false positives in all
and per case — SHALL likewise be computed for each repetition over all
its cases, then given as their median with its minimum and maximum.
Each case's outcome and findings SHALL be given per repetition.

The report SHALL also give what the runs consumed: per case, the median
over its repetitions of the cost, the duration, the API duration, the
turns, each kind of token and the permission denials; per role and for
the whole suite, the sum of those medians. A measure absent from a
repetition is left out of its median; a measure absent from every
repetition is shown as `—`.

A role SHALL fail when the median of its recall is under `--min-recall`
(default 0.5), the median of its false positives exceeds
`--max-false-positives` (default 2), or the median of its evidence
validity is under `--min-evidence` (default 0.8); each threshold
can also be set by `CODEV_EVAL_MIN_RECALL`,
`CODEV_EVAL_MAX_FALSE_POSITIVES` and `CODEV_EVAL_MIN_EVIDENCE`, the flag
winning. The command SHALL exit with 0 when every role passes and no
repetition of a case is in error, 1 otherwise, naming each failing role
and measure and each case in error with its repetition, and 2 on
an invalid threshold, an invalid case or a missing tool. With a single
repetition, the measures and the verdict SHALL be those of that run.

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

#### Scenario: A role measured over three repetitions

- **GIVEN** three repetitions where `ops` has a recall of 1.00, 0.50 and
  1.00
- **WHEN** the user runs `codev-evals score <dir>`
- **THEN** the report gives `ops` a recall of 1.00, from 0.50 to 1.00,
  and `ops` passes a recall threshold of 0.5

#### Scenario: One repetition in error

- **GIVEN** three repetitions of `ops-code-no-timeout`, the second of
  which timed out
- **WHEN** the user runs `codev-evals score <dir>`
- **THEN** it exits with 1, naming the case and its second repetition,
  and that repetition's cost is left out of the case's median

#### Scenario: The suite's recall over repetitions

- **GIVEN** three repetitions finding 36, 38 and 39 of 39 seeded defects
- **WHEN** the report is written
- **THEN** it gives the suite's recall as 0.974, from 0.923 to 1.00

#### Scenario: Consumption per case and for the suite

- **GIVEN** a case whose three repetitions read 300 000, 320 000 and
  900 000 tokens from the cache
- **WHEN** the report is written
- **THEN** the case shows 320 000 tokens read from the cache, and the
  suite's total counts 320 000 for it

### Requirement: Recorded runs are scored again without a model, and resumed

Each repetition's observation SHALL be written as
`<out>/<case>/<n>/observed.json`, `<n>` the repetition's number from 1,
carrying a format marker. An observation written directly as
`<out>/<case>/observed.json`, as earlier runs did, SHALL be read as the
case's only repetition. `codev-evals score <dir>` SHALL score the
observations of a directory again, calling no model, and give the same
report as the run did. `codev-evals run --out <dir>` SHALL skip a
repetition whose observation already exists in `<dir>` and is not in
error, unless `--rerun` is given — a repetition recorded in error is run
again — so that an interrupted run is resumed and a run asked for more
repetitions than recorded runs only the missing ones; it SHALL refuse,
with code 2 and naming both models, to resume a directory recorded with
another model than the one asked. An observation whose format marker
is above the one the runner knows SHALL be refused with a message naming
the case. An observation of an earlier format SHALL be read, the measures
it does not hold being absent.

#### Scenario: Scoring twice gives the same report

- **GIVEN** a directory of recorded observations
- **WHEN** the user runs `codev-evals score <dir>` twice
- **THEN** both runs write identical `report.json` and `report.md`

#### Scenario: A resumed run

- **GIVEN** an output directory holding the observations of three cases
  out of five
- **WHEN** the user runs `codev-evals run --out <dir>`
- **THEN** only the two other cases are run

#### Scenario: More repetitions than recorded

- **GIVEN** an output directory where each case has its repetitions 1
  and 2
- **WHEN** the user runs `codev-evals run --out <dir> --repeat 5`
- **THEN** only the repetitions 3, 4 and 5 of each case are run

#### Scenario: A repetition recorded in error is run again

- **GIVEN** an output directory where the second repetition of
  `ops-code-no-timeout` timed out and every other repetition succeeded
- **WHEN** the user runs `codev-evals run --out <dir>` with the same
  `--repeat`
- **THEN** only that repetition is run, and its new observation replaces
  the one in error

#### Scenario: An observation in the earlier layout and format

- **GIVEN** a directory holding `<case>/observed.json` with format 1 and
  no token counts
- **WHEN** the user runs `codev-evals score <dir>`
- **THEN** the case is scored as one repetition, its quality measures
  as before and its tokens and turns shown as `—`

#### Scenario: Resuming with another model

- **GIVEN** an output directory recorded with Claude Code's default model
- **WHEN** the user runs `codev-evals run --out <dir>` with
  `CODEV_EVAL_MODEL=claude-sonnet-4-5`
- **THEN** it exits with code 2 and names both models

#### Scenario: A newer observation

- **GIVEN** an `observed.json` with format 3
- **WHEN** the user runs `codev-evals score <dir>`
- **THEN** it exits with 2, naming the case and the format

### Requirement: A committed baseline is the reference for role changes

The repository SHALL hold a baseline under `evals/baseline/`: the
observations of a run of the whole suite against the current roles, with
at least five repetitions of every case and no repetition in error, and
the report scored from them, with the model and the date of the run.
Scoring the baseline's observations again SHALL give its committed
report. Claude Code's raw output of each run SHALL never be committed
with it.

#### Scenario: The baseline agrees with the scoring

- **GIVEN** `evals/baseline/`
- **WHEN** the test suite of `codev-evals` runs
- **THEN** scoring its observations gives its committed `report.json`

#### Scenario: A case missing from the baseline

- **GIVEN** a case added to the suite after the baseline was recorded
- **WHEN** the test suite of `codev-evals` runs
- **THEN** it fails and names the case, so that the baseline is recorded
  again

#### Scenario: A case with too few repetitions

- **GIVEN** a baseline where one case has three repetitions
- **WHEN** the test suite of `codev-evals` runs
- **THEN** it fails and names the case and its number of repetitions

#### Scenario: A baseline repetition in error

- **GIVEN** a baseline where the fourth repetition of one case timed out
- **WHEN** the test suite of `codev-evals` runs
- **THEN** it fails and names the case and the repetition

#### Scenario: A raw log in the baseline

- **GIVEN** a baseline holding `<case>/2/claude.json`
- **WHEN** the files to commit are listed
- **THEN** that file is ignored by version control

### Requirement: A role change comes with its scores

`CONTRIBUTING.md` and `CONTRIBUTING.fr.md` SHALL say that a pull request
modifying a file of `assets/roles/` runs the evaluations and pastes the
output of `codev-evals compare --quality-only` against the baseline;
that a pull request meant to reduce what the skills consume pastes the
output of `codev-evals compare` against the baseline; that the candidate
of a comparison runs with at least three repetitions (`--repeat 3`); that
a run confines the agent in Claude Code's sandbox, and what the sandbox
needs on Linux; and that recording the baseline
again is followed by `codev-evals docs`, which regenerates the
reliability chapter. The Roles chapter of each language SHALL say how
the built-in roles are measured, where the baseline is, and that a role
change is judged with `codev-evals compare`.

#### Scenario: Both languages say it

- **GIVEN** the two contributing guides and the two `roles.md` chapters
- **WHEN** the test suite of `codev-evals` runs
- **THEN** it fails if one of them does not name `evals/baseline` or
  `codev-evals compare`, if a contributing guide does not name
  `assets/roles/`, `codev-evals -- docs` or `--repeat 3`

#### Scenario: A guide that forgets the rule

- **GIVEN** a French contributing guide that does not name
  `assets/roles/`
- **WHEN** the documentation check runs
- **THEN** it fails and names `CONTRIBUTING.fr.md`

### Requirement: A reliability chapter is generated from the committed baseline

`codev-evals docs` SHALL write the chapter `reliability.md` of the English
and the French books from the committed baseline's `report.json` and
`run.json`, the same scores in both languages. Each page SHALL hold:

- what the suite measures and what it does not;
- the run: its date, the model, the Claude Code and codev versions, the
  number of cases (seeded and controls), the number of repetitions of
  each case, and the cost;
- headline figures: recall over every seeded defect, evidence validity
  over every checked finding, the false positives and their number per
  case, and how many roles meet every threshold;
- a chart of false positives per role, the threshold drawn as a labeled
  line, and each role over it marked by an icon and a text label;
- a chart of recall and severity agreement per role, the recall
  threshold drawn, and each role under it marked by an icon and a text
  label;
- a table of every role: cases (seeded/control), defects found over
  expected, misses, false positives, evidence validity, severity
  agreement, and whether it meets the thresholds, in words and with an
  icon;
- the thresholds, and, when a role fails one, a statement naming how
  many roles fail and on which measure;
- the limits of the measure — among them that each score is the median
  of the repetitions, whose number it names — how to run the suite, and
  a link to `evals/baseline` in the repository.

The scores the page shows SHALL be the medians the report gives.

The charts SHALL be drawn without a script, readable in the light and
the dark themes of the website and of `codev docs`, each with a title
and a text description for assistive technologies, and each mark naming
its role and value on hover. A measure that has no value — recall for a
role without a seeded case — SHALL show as `—` and draw no mark.

The command SHALL exit with 0 after writing both pages, and with 2,
naming the file, when `report.json` or `run.json` is missing, is not
valid, or carries a format newer than the command reads, and naming both
files when the run recorded in `report.json` is not the one `run.json`
describes. The committed
pages SHALL be exactly what the command produces from the committed
baseline.

#### Scenario: The committed pages follow the baseline

- **GIVEN** the committed `evals/baseline/` and both `reliability.md`
- **WHEN** the test suite of `codev-evals` runs
- **THEN** it fails, naming the page, if a committed page differs from
  what `codev-evals docs` would write

#### Scenario: A role over the false-positive threshold

- **GIVEN** a report where `ops` has 11 false positives and the
  threshold is 2
- **WHEN** the page is generated
- **THEN** the false-positive chart marks `ops` with an icon and the
  label saying it is over the threshold, and the table says `ops` fails
  on false positives

#### Scenario: A role under the recall threshold

- **GIVEN** a report where `ops` finds one of four seeded defects and
  the recall threshold is 0.50
- **WHEN** the page is generated
- **THEN** its recall mark is a quarter of the scale, it is marked under
  the threshold with an icon and a label, and the headline recall counts
  its misses

#### Scenario: A role without a seeded case

- **GIVEN** a report where `tracer` has no expected finding
- **WHEN** the page is generated
- **THEN** its recall reads `—` in the chart and the table, and no
  recall mark is drawn for it

#### Scenario: Every role within the thresholds

- **GIVEN** a report where every role meets every threshold
- **WHEN** the page is generated
- **THEN** no role is marked, and the page says every role meets the
  thresholds

#### Scenario: A report the command cannot read

- **GIVEN** a baseline directory whose `report.json` carries format 3
- **WHEN** the user runs `codev-evals docs`
- **THEN** it exits with 2, names `report.json` and the format, and
  writes no page

#### Scenario: A report of another run

- **GIVEN** a baseline directory whose `report.json` records the date
  2026-10-13 and whose `run.json` records 2026-11-02
- **WHEN** the user runs `codev-evals docs`
- **THEN** it exits with 2, names `report.json` and `run.json`, and
  writes no page

## ADDED Requirements

### Requirement: Two runs are compared, with a verdict on quality and consumption

`codev-evals compare <baseline> <candidate>` SHALL read two output
directories, calling no model, and compare them on the cases both hold,
each run's role measures computed on those cases alone. It SHALL print a
markdown report on its standard output with, per role, each quality
measure of both runs (the baseline's median, minimum and maximum, the
candidate's median) and, for the whole of the common cases, the sum of
the per-case medians of the cost, the API duration, the turns, each kind
of token and the permission denials, in both runs and as a relative
change. It SHALL list the cases only one run holds, which are not
compared.

The verdict SHALL fail on quality when, for a role, the candidate's
median recall is under the baseline's minimum recall, its median false
positives are above the baseline's maximum, or its median evidence
validity is under the baseline's minimum; a measure without a value in
either run decides nothing. A run's total cost is the sum, over the
compared cases, of each case's median cost. It SHALL fail on consumption
when the candidate's total cost is not lower than the baseline's floor —
the sum, over the compared cases, of each case's lowest cost over its
repetitions not in error — so that a candidate must be cheaper than
anything the baseline's own spread produced, and the same code compared
with itself never passes; or when a compared case has no cost in either
run; `--quality-only` SHALL leave
consumption out of the verdict while still reporting it. The duration
SHALL be reported and SHALL never decide. A repetition in error in either
run SHALL fail the verdict, naming the run, the case and the repetition.

The command SHALL exit with 0 when the verdict passes, 1 when it fails,
naming each failing role and measure with both values, and each cause on
consumption; and 2, writing no report, when a directory cannot be read,
when the two runs were recorded with different models, naming both, or
when they have no case in common. Two runs are recorded with different
models when the models they asked for differ, or when the models their
observations record as used differ; observations that record none, of
the earlier format, compare by the model asked alone. The report SHALL
name both runs' Claude Code versions and how many runs each made at the
same time, and say so when either differs.

#### Scenario: A cheaper run of the same quality

- **GIVEN** a baseline where `security` has a recall from 0.83 to 1.00,
  a total cost of 2.04 USD and a floor of 1.95 USD, and a candidate on
  the same cases with a median recall of 1.00 and a total cost of
  1.70 USD
- **WHEN** the user runs `codev-evals compare <baseline> <candidate>`
- **THEN** it exits with 0, and the report shows the cost down by about
  17 %

#### Scenario: A gain within the baseline's spread

- **GIVEN** a baseline with a total cost of 2.04 USD and a floor of
  1.95 USD, and a candidate of the same quality at 1.99 USD
- **WHEN** the user runs `codev-evals compare <baseline> <candidate>`
- **THEN** it exits with 1, naming 1.99 USD and the baseline's floor,
  1.95 USD

#### Scenario: The same code compared with itself

- **GIVEN** a baseline
- **WHEN** the user runs `codev-evals compare <baseline> <baseline>`
- **THEN** it fails on consumption and passes on quality

#### Scenario: A cheaper run that misses more

- **GIVEN** a baseline where `ops` has a recall from 0.83 to 1.00, and a
  cheaper candidate where its median recall is 0.67
- **WHEN** the user runs `codev-evals compare <baseline> <candidate>`
- **THEN** it exits with 1, naming `ops`, the recall 0.67 and the
  baseline's minimum 0.83

#### Scenario: A run of the same quality that costs as much

- **GIVEN** a candidate whose role measures stay within the baseline's
  ranges and whose total cost is above the baseline's
- **WHEN** the user runs `codev-evals compare <baseline> <candidate>`
- **THEN** it exits with 1 and names the cost of both runs; with
  `--quality-only`, it exits with 0 and still reports both costs

#### Scenario: Only the duration grew

- **GIVEN** a candidate cheaper than the baseline, of the same quality,
  whose API duration is twice the baseline's
- **WHEN** the user runs `codev-evals compare <baseline> <candidate>`
- **THEN** it exits with 0 and reports the duration's increase

#### Scenario: A candidate on a subset of the cases

- **GIVEN** a baseline of 40 cases and a candidate that ran 10 of them
- **WHEN** the user runs `codev-evals compare <baseline> <candidate>`
- **THEN** both runs are measured on those 10 cases, and the report
  lists the 30 others as not compared

#### Scenario: A baseline in the earlier format

- **GIVEN** a baseline of format 1, which records each case's cost but
  no tokens, and a cheaper candidate of format 2 of the same quality
- **WHEN** the user runs `codev-evals compare <baseline> <candidate>`
- **THEN** it exits with 0, the costs compared and the baseline's tokens
  and turns shown as `—`

#### Scenario: A baseline without costs

- **GIVEN** a baseline whose observations hold no cost
- **WHEN** the user runs `codev-evals compare <baseline> <candidate>`
- **THEN** it exits with 1 and says the consumption cannot be compared;
  with `--quality-only`, the verdict rests on quality alone

#### Scenario: Two models

- **GIVEN** a baseline recorded with Claude Code's default model and a
  candidate recorded with `claude-sonnet-4-5`
- **WHEN** the user runs `codev-evals compare <baseline> <candidate>`
- **THEN** it exits with 2, names both models, and prints no report

#### Scenario: The same default, another model used

- **GIVEN** a baseline and a candidate both recorded with Claude Code's
  default model, whose observations record `claude-opus-5-5` and
  `claude-sonnet-5-5` as used
- **WHEN** the user runs `codev-evals compare <baseline> <candidate>`
- **THEN** it exits with 2, names both models, and prints no report

#### Scenario: Another Claude Code version

- **GIVEN** a baseline recorded with Claude Code 2.1.289 and a candidate
  with 2.1.301, on the same model
- **WHEN** the user runs `codev-evals compare <baseline> <candidate>`
- **THEN** the report names both versions and says they differ

#### Scenario: No case in common

- **GIVEN** a baseline and a candidate that share no case
- **WHEN** the user runs `codev-evals compare <baseline> <candidate>`
- **THEN** it exits with 2 and says no case is in common
