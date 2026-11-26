## ADDED Requirements

### Requirement: An explore or onboard case pins a small project, a request and what the reply must say

An explore case and an onboard case SHALL each be a directory of
`evals/cases/` holding a `case.yaml` with `step: idea.explore` or
`step: project.onboard`, `kind` (`seeded`), `stacks` (the fixture's
languages), `summary` and `expected`; and a folder `base/`, the project
as it stands. An explore case SHALL also hold `request`, the topic said to
`/compagnon-explore`, and `expected.outcome`, one of `crystallizes`,
`missing` and `not-worth`. `expected` SHALL hold `contains`, at least one
text the agent's messages must hold, and MAY hold `absent`, texts they
must not hold.

Loading a case SHALL fail, naming the case and the cause, when an
unknown key is found; when an explore case has no `request` or an
`outcome` that is not one of its values; when an onboard case holds
`request` or `outcome`; when `contains` is empty; or when a file of
`base/` is not UTF-8 text or is larger than 64 KiB.

#### Scenario: An explore case that crystallizes

- **GIVEN** a `case.yaml` with `step: idea.explore`, a `request`,
  `outcome: crystallizes` and `contains: ["/compagnon-propose"]`
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

### Requirement: The runner replays explore and onboard on the case's project

For an explore or onboard case, `compagnon-evals run` SHALL make a new git
repository holding `base/`, install the skills of the Compagnon under
test into it without changing the case's `_compagnon/config.yaml`, and
commit; when `base/` holds no `_compagnon/`, it SHALL remove the
`_compagnon/` the installation wrote before the commit. It SHALL then run
Claude Code with a prompt naming the skill — `compagnon-explore` with the
case's `request`, or `compagnon-onboard` — and the evaluation mode: ask no
question, take the most reasonable assumption and say it, and answer
**Later** if the skill asks whether to update Compagnon. The prompt says
nothing of the outcome expected. Unless `COMPAGNON_EVAL_BUDGET_USD` and
`COMPAGNON_EVAL_TIMEOUT_SECS` say otherwise, a case SHALL run under a
budget of 3 USD and a timeout of 600 seconds. `--step idea.explore` and
`--step project.onboard` SHALL select their cases; no other step selects
them.

The observation SHALL hold the text of every message the agent wrote and
every file modified, created or removed in the project, `.git` excepted.

#### Scenario: The prompt of an explore case

- **GIVEN** an explore case whose `request` is "rate limiting for the API"
- **WHEN** the runner builds its prompt
- **THEN** it names `compagnon-explore` and the request, says to ask no
  question and to answer **Later** to an update question, and names no
  outcome

#### Scenario: An onboard case without Compagnon

- **GIVEN** an onboard case whose `base/` holds no `_compagnon/`
- **WHEN** the runner prepares it
- **THEN** the case's directory holds the skill under
  `.claude/skills/compagnon-onboard/` and no `_compagnon/` folder

### Requirement: An explore or onboard run is scored by deterministic checks

Scoring an explore or onboard run SHALL apply these checks, without a
model: no file of the project is modified, created or removed; no folder
appears under `_compagnon/changes/`; each `contains` text is in the
agent's messages; and no `absent` text is. Texts SHALL be compared
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

- **GIVEN** an onboard case with a thin configuration and
  `contains: ["/compagnon-configure"]`, and a run whose messages
  recommend only `/compagnon-propose`
- **WHEN** the run is scored
- **THEN** the check on `contains` fails

#### Scenario: An explore run that offers to implement

- **GIVEN** an explore case with `absent: ["I'll implement"]`, and a run
  whose messages say "I'll implement it now"
- **WHEN** the run is scored
- **THEN** the check on `absent` fails

### Requirement: The report, the comparison and the reliability chapter give the explore and onboard measures

The markdown and JSON reports of `compagnon-evals score` SHALL give, for
each of the two steps, its measures and each run's failed checks;
`compagnon-evals compare` SHALL compare them as it compares the configure
measures, and give a quality verdict for each step present in both runs;
and `compagnon-evals docs` SHALL give them in the reliability chapter of
both books, each step under its own heading.

#### Scenario: Comparing two runs of explore

- **GIVEN** a baseline and a candidate that both hold `idea.explore`
  runs, the candidate's share of runs passing under the baseline's
  minimum
- **WHEN** the user runs `compagnon-evals compare`
- **THEN** the verdict on `idea.explore` is a regression

### Requirement: The suite covers explore's outcomes and onboard's recommendations

The suite SHALL hold at least one `idea.explore` case per outcome
(`crystallizes`, `missing`, `not-worth`) and one `project.onboard` case per
state of the skill's recommendation table: no `_compagnon/`; a thin
configuration with no active change; a configuration with rules and no
active change; one active change whose planning is incomplete; one active
change whose planning is complete. The committed baseline SHALL hold five
repetitions of each.

#### Scenario: A missing state

- **GIVEN** the cases of the suite, none of them an onboard case with an
  active change whose planning is complete
- **WHEN** the test suite of `compagnon-evals` runs
- **THEN** the coverage test fails, naming that state
