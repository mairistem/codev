## ADDED Requirements

### Requirement: A reliability chapter is generated from the committed baseline

`codev-evals docs` SHALL write the chapter `reliability.md` of the English
and the French books from the committed baseline's `report.json` and
`run.json`, the same scores in both languages. Each page SHALL hold:

- what the suite measures and what it does not;
- the run: its date, the model, the Claude Code and codev versions, the
  number of cases (seeded and controls) and the cost;
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
- the limits of the measure, how to run the suite, and a link to
  `evals/baseline` in the repository.

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

- **GIVEN** a baseline directory whose `report.json` carries format 2
- **WHEN** the user runs `codev-evals docs`
- **THEN** it exits with 2, names `report.json` and the format, and
  writes no page

#### Scenario: A report of another run

- **GIVEN** a baseline directory whose `report.json` records the date
  2026-10-13 and whose `run.json` records 2026-11-02
- **WHEN** the user runs `codev-evals docs`
- **THEN** it exits with 2, names `report.json` and `run.json`, and
  writes no page

## MODIFIED Requirements

### Requirement: A role change comes with its scores

`CONTRIBUTING.md` and `CONTRIBUTING.fr.md` SHALL say that a pull request
modifying a file of `assets/roles/` runs the evaluations and pastes the
scores before and after the change, and that recording the baseline
again is followed by `codev-evals docs`, which regenerates the
reliability chapter. The Roles chapter of each language SHALL say how
the built-in roles are measured and where the baseline is.

#### Scenario: Both languages say it

- **GIVEN** the two contributing guides and the two `roles.md` chapters
- **WHEN** the test suite of `codev-evals` runs
- **THEN** it fails if one of them does not name `evals/baseline`, if
  a contributing guide does not name `assets/roles/`, or if a
  contributing guide does not name `codev-evals -- docs`

#### Scenario: A guide that forgets the rule

- **GIVEN** a French contributing guide that does not name
  `assets/roles/`
- **WHEN** the documentation check runs
- **THEN** it fails and names `CONTRIBUTING.fr.md`
