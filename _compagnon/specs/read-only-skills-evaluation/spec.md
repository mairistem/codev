# Read Only Skills Evaluation Specification

## Purpose

Measure the two read-only skills a newcomer meets first,
`/compagnon-explore` and `/compagnon-onboard`, on small projects and without
a model in the scoring, so that a change to their text can be judged
against a baseline, as decision 0008 asks.

## Requirements

### Requirement: An explore or onboard case pins a small project, its situation and what the reply must conclude

An explore case and an onboard case SHALL each be a directory of
`evals/cases/` holding a `case.yaml` with `step: idea.explore` or
`step: project.onboard`, `kind` (`seeded`), `stacks` (the fixture's
languages), `summary` and `expected`; and a folder `base/`, the project
as it stands. An explore case SHALL also hold `request`, the topic said to
`/compagnon-explore`, and `expected.outcome`, one of `crystallizes`,
`missing` and `not-worth`. An onboard case SHALL hold `expected.state`,
one of `no-compagnon`, `thin-config`, `configured`, `planning-incomplete`,
`planning-complete` and `several-changes`, the row of the skill's
recommendation table its project is in. `expected` SHALL hold
`contains`, at least one text the agent's final message must hold, and
`absent`, at least one text it must not hold — the marks of the other
outcomes or states.

Loading a case SHALL fail, naming the case and the cause, when an
unknown key is found; when an explore case has no `request`, or an
`outcome` that is not one of its values, or a `state`; when an onboard
case holds `request` or `outcome`, or a `state` that is not one of its
values; when `contains` or `absent` is empty; or when a file of `base/`
is not UTF-8 text or is larger than 64 KiB.

#### Scenario: An explore case that crystallizes

- **GIVEN** a `case.yaml` with `step: idea.explore`, a `request`,
  `outcome: crystallizes`, `contains: ["/compagnon-propose"]` and
  `absent: ["is missing"]`
- **WHEN** the suite loads its cases
- **THEN** the case loads, as an `idea.explore` case

#### Scenario: An onboard case with a request

- **GIVEN** a case with `step: project.onboard` and a `request`
- **WHEN** the suite loads its cases
- **THEN** it fails, naming the case and `request`

#### Scenario: An explore case with an unknown outcome

- **GIVEN** a case with `step: idea.explore` and `outcome: maybe`
- **WHEN** the suite loads its cases
- **THEN** it fails, naming the case and `outcome`

#### Scenario: An onboard case without its state

- **GIVEN** a case with `step: project.onboard` and no `expected.state`
- **WHEN** the suite loads its cases
- **THEN** it fails, naming the case and `state`

### Requirement: The runner replays explore and onboard on the case's project

For an explore or onboard case, `compagnon-evals run` SHALL make a new git
repository holding `base/`, install the skills of the Compagnon under
test into it without changing the case's `_compagnon/config.yaml`, and
commit; when `base/` holds no `_compagnon/`, it SHALL remove the
`_compagnon/` the installation wrote before the commit. It SHALL then run
Claude Code with a prompt naming the skill — `compagnon-explore` with the
case's `request`, or `compagnon-onboard` — and the evaluation mode: the
user cannot answer, so ask nothing and, where the skill would ask, end as
the skill says. The prompt says nothing of the outcome or the state.
Unless `COMPAGNON_EVAL_BUDGET_USD` and `COMPAGNON_EVAL_TIMEOUT_SECS` say
otherwise, a case SHALL run under a budget of 3 USD and a timeout of 600
seconds. `--step idea.explore` and `--step project.onboard` SHALL select
their cases; no other step selects them.

The observation SHALL hold the agent's final message, the text of every
message it wrote, and every file modified, created or removed in the
project, `.git` excepted.

#### Scenario: The prompt of an explore case

- **GIVEN** an explore case whose `request` is "rate limiting for the API"
- **WHEN** the runner builds its prompt
- **THEN** it names `compagnon-explore` and the request, says the user
  cannot answer and to end as the skill says where it would ask, and
  names no outcome

#### Scenario: An onboard case without Compagnon

- **GIVEN** an onboard case whose `base/` holds no `_compagnon/`
- **WHEN** the runner prepares it
- **THEN** the case's directory holds the skill under
  `.claude/skills/compagnon-onboard/` and no `_compagnon/` folder

#### Scenario: Selecting the explore cases

- **GIVEN** the suite's cases
- **WHEN** the user runs `compagnon-evals run --step idea.explore`
- **THEN** only the `idea.explore` cases are selected

#### Scenario: A run that creates a file

- **GIVEN** an explore run that creates `notes.md`
- **WHEN** its observation is recorded
- **THEN** it lists `notes.md` as created

### Requirement: An explore or onboard run is scored by deterministic checks

Scoring an explore or onboard run SHALL apply these checks, without a
model: no file of the project is modified, created or removed; no folder
appears under `_compagnon/changes/`; each `contains` text is in the
agent's final message; and no `absent` text is. Texts SHALL be compared
ignoring case and runs of whitespace. A run SHALL be given its checks
passed and failed, and it passes when every check passes. The measures of
each step SHALL be, per repetition over its runs, the share of runs that
pass and the share of checks that pass, given as their median with their
minimum and maximum; and the runs that modify a file.

#### Scenario: An explore run that writes a file

- **GIVEN** an explore run that creates `notes.md`
- **WHEN** the run is scored
- **THEN** the check on the files fails and the run does not pass

#### Scenario: An onboard run that recommends the wrong action

- **GIVEN** an onboard case in the state `thin-config`, with
  `contains: ["/compagnon-configure"]`, and a run whose final message
  recommends only `/compagnon-propose`
- **WHEN** the run is scored
- **THEN** the check on `contains` fails

#### Scenario: A reply concluding on another outcome

- **GIVEN** a `not-worth` explore case with `contains: ["src/limiter.rs"]`
  and `absent: ["/compagnon-propose"]`, and a run that reads
  `src/limiter.rs` along the way, then ends suggesting
  `/compagnon-propose`
- **WHEN** the run is scored
- **THEN** the check on `absent` fails and the run does not pass

#### Scenario: Narration is not the conclusion

- **GIVEN** an onboard case with `absent: ["/compagnon-apply"]`, and a run
  that relays `/compagnon-apply` while reading the project, then ends with
  a final message without it
- **WHEN** the run is scored
- **THEN** the check on `absent` passes

### Requirement: The report, the comparison and the reliability chapter give the explore and onboard measures

The markdown and JSON reports of `compagnon-evals score` SHALL give, for
each of the two steps present, its measures and each run's failed
checks, under a heading of its own; `compagnon-evals compare` SHALL compare
them as it compares the configure measures, its verdict failing, naming
the step and the measure, when a share falls under the baseline's
minimum; and `compagnon-evals docs` SHALL give them in the reliability
chapter of both books, each step under its own heading. A run without
these steps SHALL show no section for them.

#### Scenario: The report of an explore run

- **GIVEN** the observations of `idea.explore` runs, one of them failing
  its `absent` check
- **WHEN** the user runs `compagnon-evals score`
- **THEN** the markdown report has an `idea.explore` section giving the
  measures and naming that run's failed check

#### Scenario: Comparing two runs of explore

- **GIVEN** a baseline and a candidate that both hold `idea.explore`
  runs, the candidate's share of runs passing under the baseline's
  minimum
- **WHEN** the user runs `compagnon-evals compare`
- **THEN** the verdict fails, naming `idea.explore` and the share of runs
  passing

#### Scenario: The reliability chapter

- **GIVEN** a baseline holding `project.onboard` runs
- **WHEN** the user runs `compagnon-evals docs`
- **THEN** both books' reliability chapter gives the onboard measures
  under their own heading

### Requirement: The suite covers explore's outcomes and onboard's recommendations

The suite SHALL hold at least one `idea.explore` case per outcome
(`crystallizes`, `missing`, `not-worth`) and one `project.onboard` case per
state (`no-compagnon`, `thin-config`, `configured`,
`planning-incomplete`, `planning-complete`, `several-changes`). The
committed baseline SHALL hold five repetitions of each.

#### Scenario: A missing state

- **GIVEN** the cases of the suite, none of them an onboard case in the
  state `planning-complete`
- **WHEN** the test suite of `compagnon-evals` runs
- **THEN** the coverage test fails, naming `planning-complete`
