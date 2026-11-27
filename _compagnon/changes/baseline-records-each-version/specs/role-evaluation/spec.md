## ADDED Requirements

### Requirement: A resumed run keeps the Compagnon version of the steps it adds

When `compagnon-evals run` resumes into an output directory whose
`run.json` records a Compagnon version other than the one under test, it
SHALL keep every field `run.json` records and append to its list `added`
one entry naming the steps it runs, the Compagnon version under test and
the date. A `run.json` without `added` SHALL read as before. The
reliability chapter SHALL give, beside the run's Compagnon version, the
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

#### Scenario: Resuming with the same Compagnon

- **GIVEN** an output directory whose `run.json` records the Compagnon
  version under test
- **WHEN** the user resumes a run into it
- **THEN** `run.json` gains no `added` entry
