## ADDED Requirements

### Requirement: A resumed run keeps the Compagnon version of the steps it adds

When `compagnon-evals run` resumes into an output directory whose
`run.json` records a Compagnon version other than the one under test, and
runs at least one case, it SHALL keep every field `run.json` records and
append to its list `added` one entry naming the steps of the cases it
actually runs, the Compagnon version under test and the date — unless the
last entry already names those steps and that version, which is then
kept. A `run.json` without `added` SHALL read as before. The reliability
chapter and the report SHALL give, beside the run's Compagnon version, the
version and the date of each entry of `added`, with its steps.

#### Scenario: Steps added with another Compagnon

- **GIVEN** an output directory whose `run.json` records
  "compagnon 0.6.1", and Compagnon 0.7.0 under test
- **WHEN** the user runs `compagnon-evals run --step idea.explore --out <dir>`
- **THEN** `run.json` still records "compagnon 0.6.1"
- **AND** its `added` list holds `idea.explore` with "compagnon 0.7.0" and
  the date of the run

#### Scenario: The chapter names the added steps' version

- **GIVEN** a baseline whose `run.json` records "compagnon 0.6.1" and an
  `added` entry for `idea.explore` and `project.onboard` with
  "compagnon 0.7.0" on 2026-11-26
- **WHEN** the user runs `compagnon-evals docs`
- **THEN** both pages give the version 0.6.1 for the run and 0.7.0, with
  its date, for `idea.explore` and `project.onboard`

#### Scenario: A resume with nothing left to run

- **GIVEN** an output directory holding every observation the run
  selects, and another Compagnon under test
- **WHEN** the user resumes the run into it
- **THEN** `run.json` gains no `added` entry

#### Scenario: A resume of a few cases

- **GIVEN** an output directory recorded with "compagnon 0.6.1", and
  Compagnon 0.7.0 under test
- **WHEN** the user runs `compagnon-evals run --case onboard-thin-config
  --out <dir>`
- **THEN** the `added` entry names `project.onboard` only

#### Scenario: A run.json without added

- **GIVEN** the committed baseline's `run.json` as it was before this
  requirement, without `added`
- **WHEN** it is read
- **THEN** it reads with no added step, and the chapter gives the run's
  Compagnon version alone

#### Scenario: Resuming with the same Compagnon

- **GIVEN** an output directory whose `run.json` records the Compagnon
  version under test
- **WHEN** the user resumes a run into it
- **THEN** `run.json` gains no `added` entry
