## ADDED Requirements

### Requirement: The runner can play the review cases in the isolated mode

`codev-evals run --review-mode isolated` SHALL write `review: { mode:
isolated }` into each review case's `_codev/config.yaml` before the
skills of the codev under test are rendered, so that the skill runs the
evaluated role as its own sub-agent, as a project in that mode does; the
sub-agent tool is then granted like `codev`. `--fixture-config <file>`
SHALL append the YAML of `<file>` to each review case's
`_codev/config.yaml` at the same moment, for a setting a measure needs.
Planning and update cases ignore both options. `--review-mode` accepts
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

## MODIFIED Requirements

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
validity is under the baseline's minimum; and, for the planning step
`plan.write`, when the candidate's median recall of the primary
requirements is under the baseline's minimum, or its median of probable
duplicates, of decisions ignored or of plans that do not validate is
above the baseline's maximum. A measure without a value in
either run decides nothing. The report SHALL give the planning measures
in a section of their own, next to the roles. A run's total cost is the sum, over the
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

The report SHALL also compare the input per call and the output per call
of both runs, and where the tokens go, source by source, with each
relative change. The output never decides: it carries the model's
reasoning, which the quality measures guard. The report SHALL name every
quality measure on which the candidate's median does better than the
best repetition of the baseline — a recall or an evidence validity above
its maximum, false positives, duplicates, decisions ignored or invalid
plans under its minimum — as quality improved.

The command SHALL exit with 0 when the verdict passes, 1 when it fails,
naming each failing role and measure with both values, and each cause on
consumption; and 2, writing no report, when a directory cannot be read,
when the two runs were recorded with different models, naming both, unless
`--across-models` is given, or, when both hold review cases, in
different review modes, naming both, or when they have no case in
common. The report SHALL name both runs' appended configurations and say
when they differ. Two runs are recorded with different
models when the models they asked for differ, or when the models their
observations record as used differ; observations that record none, of
the earlier format, compare by the model asked alone. `--across-models`
SHALL compare such runs anyway, for a change that is itself a change of
model, and the report SHALL then name both runs' models at its top. The report SHALL
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

#### Scenario: Comparing across models on purpose

- **GIVEN** a baseline whose observations record `claude-opus-5-5` and a
  candidate whose observations record `claude-opus-5-5` and
  `claude-sonnet-5-5`
- **WHEN** the user runs `codev-evals compare <baseline> <candidate>
  --across-models`
- **THEN** it compares them, and the report names both runs' models at
  its top

#### Scenario: Different appended configurations

- **GIVEN** a baseline and a candidate recorded with different
  `--fixture-config` texts
- **WHEN** the user runs `codev-evals compare <baseline> <candidate>`
- **THEN** the report names both texts and says they differ

#### Scenario: Two review modes

- **GIVEN** a baseline recorded in the `inline` review mode and a
  candidate in the `isolated` mode
- **WHEN** the user runs `codev-evals compare <baseline> <candidate>`
- **THEN** it exits with 2, names both modes, and prints no report

#### Scenario: Another Claude Code version

- **GIVEN** a baseline recorded with Claude Code 2.1.289 and a candidate
  with 2.1.301, on the same model
- **WHEN** the user runs `codev-evals compare <baseline> <candidate>`
- **THEN** the report names both versions and says they differ

#### Scenario: No case in common

- **GIVEN** a baseline and a candidate that share no case
- **WHEN** the user runs `codev-evals compare <baseline> <candidate>`
- **THEN** it exits with 2 and says no case is in common

#### Scenario: Quality improved

- **GIVEN** a baseline whose `skeptic` has from 3 to 6 false positives,
  and a candidate whose median is 2
- **WHEN** the user runs `codev-evals compare <baseline> <candidate>`
- **THEN** the report says the quality of `skeptic` improved on false
  positives, from 3 at best to 2

#### Scenario: Lighter calls

- **GIVEN** a candidate whose input per call falls from 38,000 to 31,000
  tokens and whose `skill` source falls by half
- **WHEN** the user runs `codev-evals compare <baseline> <candidate>`
- **THEN** the report shows both changes, the input per call at −18 %
  and the `skill` source at −50 %

#### Scenario: A plan that misses more requirements

- **GIVEN** a baseline whose planning cases have a primary recall from
  0.60 to 0.75, and a cheaper candidate whose median is 0.50
- **WHEN** the user runs `codev-evals compare <baseline> <candidate>`
- **THEN** it exits with 1, naming `plan.write`, the recall 0.50 and the
  baseline's minimum 0.60
