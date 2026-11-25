## Purpose

The configure evaluation measures the default mode of `/codev-configure`:
given a project, does the agent propose a `context:` and `rules:`
grounded in what the project does, its testing rules included, write
them only on the author's agreement, preserve every other field and
comment of `_codev/config.yaml`, and refuse when the file is missing.

## ADDED Requirements

### Requirement: A configure case pins a small project, the author's answer and the outcome expected

A configure case SHALL be a directory of `evals/cases/` holding a
`case.yaml` with `step: configure.enrich`, `kind` (`seeded`), `stacks`
(the fixture's languages), `summary`, `answer` (`yes` or `no`, the
author's answer to the skill's confirmation question) and `expected`;
and a folder `base/`, the project as it stands, its `_codev/config.yaml`
included unless the case is about a missing configuration. `expected`
SHALL hold `outcome` — `written`, `unchanged` or `refused` — and MAY
hold `written` and `unwritten`, each a mapping from a part of the
proposal (`context`, `specs`, `design` or `tasks`) to texts that part
must, or must not, hold once written; and `contains`, texts the agent's
messages must hold.

Loading a case SHALL fail, naming the case and the cause, when an
unknown key is found; when `answer` or `outcome` is not one of its
values; when `written` or `unwritten` is given for an outcome other than
`written`, or names another part; when the outcome is `written` with the
answer `no`; when the outcome is `refused` and `base/` holds
`_codev/config.yaml`, or another outcome and it does not; or when a file
of `base/` is not UTF-8 text or is larger than 64 KiB.

#### Scenario: A case expecting testing rules

- **GIVEN** a `case.yaml` with `step: configure.enrich`, `answer: yes`,
  `outcome: written`, `written: { tasks: [Jest, "__tests__/"] }` and a
  `base/` holding `_codev/config.yaml`
- **WHEN** the suite loads its cases
- **THEN** the case loads, as a `configure.enrich` case

#### Scenario: A written outcome the author refused

- **GIVEN** a case with `answer: no` and `outcome: written`
- **WHEN** the suite loads its cases
- **THEN** it fails, naming the case

#### Scenario: Expected texts for another outcome

- **GIVEN** a case with `outcome: unchanged` and
  `written: { tasks: [pytest] }`
- **WHEN** the suite loads its cases
- **THEN** it fails, naming the case and `written`

#### Scenario: A binary fixture file

- **GIVEN** a configure case whose `base/` holds a file that is not
  UTF-8 text
- **WHEN** the suite loads its cases
- **THEN** it fails, naming the case and the file

#### Scenario: A refusing case with a configuration

- **GIVEN** a case with `outcome: refused` whose `base/` holds
  `_codev/config.yaml`
- **WHEN** the suite loads its cases
- **THEN** it fails, naming the case and `_codev/config.yaml`

### Requirement: The runner replays the configure skill with the author's answer

For a configure case, `codev-evals run` SHALL make a new git repository
holding `base/`, install the skills of the codev under test into it
without changing the case's `_codev/config.yaml`, and commit. When
`base/` holds no `_codev/config.yaml`, the runner SHALL install the
skills, then remove `_codev/` before the commit, so that the project has
the skills and no codev root.

It SHALL then run Claude Code with a prompt that names the
`codev-configure` skill and the evaluation mode: ask no question but the
skill's confirmation; ask it as the skill says, then continue with the
author's answer, the case's `answer`; and, if the skill asks whether to
update codev, take the answer **Later**. The prompt says nothing of what
to propose or preserve. Unless `CODEV_EVAL_BUDGET_USD` and
`CODEV_EVAL_TIMEOUT_SECS` say otherwise, a configure case SHALL run
under a budget of 3 USD and a timeout of 600 seconds. `--step
configure.enrich` SHALL select the configure cases; no other step
selects them.

The observation SHALL hold the text of every message the agent wrote,
the content of `_codev/config.yaml` before and after the run (absent
when there is none), and whether `codev status --json` reads the project
after the run without error. A file modified, created or removed other
than `_codev/config.yaml` SHALL be recorded as modified outside the
step.

#### Scenario: The prompt of a configure case

- **GIVEN** a configure case with `answer: no`
- **WHEN** the runner builds its prompt
- **THEN** it names `codev-configure`, says to ask only the skill's
  confirmation and to continue with the answer `no`, and to answer
  **Later** to an update question

#### Scenario: A project without a configuration

- **GIVEN** a configure case whose `base/` holds no `_codev/`
- **WHEN** the runner prepares it
- **THEN** the case's directory holds the skill under
  `.claude/skills/codev-configure/` and no `_codev/` folder

#### Scenario: A run that edits the code

- **GIVEN** a configure run that writes `src/index.ts`
- **WHEN** its observation is recorded
- **THEN** it lists `src/index.ts` as modified outside the step

### Requirement: A configure run is scored by deterministic checks

Scoring a configure run SHALL apply these checks, without a model:

- for a `written` case: every top-level key of `_codev/config.yaml` but
  `context` and `rules` holds the value it held before the run; every
  comment line of the file before the run is still there, except one
  that stood directly above `context:`; the line
  `# written by /codev-configure` stands directly above `context:`;
  the value of `context` spans two to five non-empty lines of the file,
  counted in the text whatever its YAML style; `rules` holds `specs` and
  `design` with one or two rules each, and `tasks` with at least one;
  each `written` text is in its part and no `unwritten` text is; and
  `codev status --json` reads the project without error;
- for an `unchanged` case: `_codev/config.yaml` is byte-identical to the
  fixture's;
- for a `refused` case: no `_codev/` folder exists after the run;
- in every case: no file is modified outside the step, and each
  `contains` text is in the agent's messages.

Texts SHALL be compared ignoring case and runs of whitespace. A run
SHALL be given its checks passed and failed, and it passes when every
check passes. The configure measures SHALL be, per repetition over the
configure runs, the share of runs that pass and the share of checks that
pass, given as their median with their minimum and maximum; and the runs
that modify a file outside the step.

#### Scenario: A written configuration that drops the MCP

- **GIVEN** a `written` case whose configuration sets `mcp.jira_tool`,
  and a run whose written configuration no longer has `mcp`
- **WHEN** the run is scored
- **THEN** the check on the preserved keys fails and the run does not
  pass

#### Scenario: Testing rules that follow the existing tests

- **GIVEN** a case with `written: { tasks: [Jest, "__tests__/"] }` and
  `unwritten: { tasks: [Vitest] }`, and a run whose `tasks` rules name
  Jest and `__tests__/` and no other runner
- **WHEN** the run is scored
- **THEN** both checks pass

#### Scenario: The proposal's shape

- **GIVEN** a `written` case, and a run whose `context` is a folded
  block of three lines preceded by `# written by /codev-configure`, and
  whose `rules` hold two `specs` rules, one `design` rule and three
  `tasks` rules
- **WHEN** the run is scored
- **THEN** the checks on the marker and the shape pass
- **AND** the same run without its `design` rules, or without the
  marker line, fails the check concerned

#### Scenario: A context on a single line

- **GIVEN** a `written` case, and a run whose `context` is one quoted
  line
- **WHEN** the run is scored
- **THEN** the check on the shape fails

#### Scenario: A configuration codev no longer reads

- **GIVEN** a `written` case, and a run after which `codev status
  --json` fails on the configuration
- **WHEN** the run is scored
- **THEN** the check on the configuration's validity fails

#### Scenario: A comment removed

- **GIVEN** a `written` case whose configuration holds the comment
  `# Paths only this repository knows to be sensitive` above
  `triggers:`, and a run that drops it
- **WHEN** the run is scored
- **THEN** the check on the comments fails

#### Scenario: Nothing written on a no

- **GIVEN** an `unchanged` case with `contains: ["No changes made"]`,
  and a run that leaves the file as it was and says no changes were made
- **WHEN** the run is scored
- **THEN** every check passes and the run passes

#### Scenario: A configuration created where there was none

- **GIVEN** a `refused` case, and a run that writes
  `_codev/config.yaml`
- **WHEN** the run is scored
- **THEN** the check on the missing configuration fails

### Requirement: The report, the comparison and the reliability chapter give the configure measures

`report.md` and `report.json` SHALL give, for the step
`configure.enrich`, the cases, the median, minimum and maximum of the
share of runs that pass and of the share of checks that pass, and the
runs modifying a file outside the step; and per run, its outcome and
each check, passed or failed. A suite without a configure case SHALL
give no such section. `codev-evals compare` SHALL judge it as it judges
the update step: the verdict fails when the candidate's share of runs
that pass falls under the baseline's minimum, or its runs modifying a
file outside the step rise above the baseline's maximum; and, as for the
sync and archive steps, when a case's share of candidate runs that pass
falls under that case's share in the baseline, rescored with the case as
it now stands. The reliability
chapter SHALL give its measures, in both languages, when the baseline
holds configure cases.

#### Scenario: The report of a suite with configure cases

- **GIVEN** observations of configure cases
- **WHEN** the user runs `codev-evals score <dir>`
- **THEN** the report has a `configure.enrich` section with its measures
  and, per run, each check

#### Scenario: A configure regression

- **GIVEN** a baseline whose configure runs pass at 0.80 at least, and a
  candidate at 0.40
- **WHEN** the user runs `codev-evals compare <baseline> <candidate>`
- **THEN** the verdict fails, naming `configure.enrich`, 0.40 and 0.80

#### Scenario: One configure case regresses

- **GIVEN** a baseline where `configure-ts-jest` passes in its five
  runs, and a candidate where it passes in one of three, the step's share
  staying above the baseline's minimum
- **WHEN** the user runs `codev-evals compare <baseline> <candidate>`
- **THEN** the verdict fails, naming `configure-ts-jest`

#### Scenario: A suite without configure cases

- **GIVEN** observations of planning cases only
- **WHEN** the user runs `codev-evals score <dir>`
- **THEN** the report has no `configure.enrich` section

#### Scenario: The reliability chapter with configure cases

- **GIVEN** a baseline holding configure runs
- **WHEN** the user runs `codev-evals docs`
- **THEN** both pages give the `configure.enrich` measures

### Requirement: The suite covers the configure skill's default mode

The suite SHALL hold configure cases covering, at least: a TypeScript
project whose existing tests run with Jest in `__tests__/` folders; a Go
project without tests, whose testing rules follow the language's
conventions; a Python project tested with pytest whose configuration
sets `language: fr`, `mcp`, `codev_version` and comments, all to be
preserved; an author who answers `no`; and a project without
`_codev/config.yaml`. The baseline SHALL hold five runs of each,
recorded with the Claude Code version the evaluation workflow pins.

#### Scenario: The configure cases of the suite

- **GIVEN** the suite's cases
- **WHEN** the user runs `codev-evals list`
- **THEN** five `configure.enrich` cases are listed, covering the
  projects above

#### Scenario: A configure case without its baseline runs

- **GIVEN** a configure case with no observation in `evals/baseline/`
- **WHEN** the suite's tests run
- **THEN** they fail, naming the case
