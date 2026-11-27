# Review record: baseline-records-each-version

<!-- compagnon-review: 1 -->

## plan.challenge

Mode: isolated.

### skeptic

#### R-skeptic-1 Hand-editing run.json without scoring again would make docs refuse the baseline

```yaml
severity: medium
confidence: high
location: 'design.md#Decision: The committed baseline gets its entry by hand, once'
evidence: |
  scored again (`cargo run -p compagnon-evals -- score evals/baseline`), so
recommendation: 'Score the baseline again after the edit, and list report.json and report.md in the impact.'
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    ref: "design.md; proposal.md Impact; tasks.md 2.2"
  - status: fixed
    by: skeptic
```

#### R-skeptic-2 report.md still showed one version for the whole run

```yaml
severity: low
confidence: high
location: 'proposal.md#What Changes'
evidence: |
  - The reliability chapter and the report (`report.md`) give, beside the
recommendation: 'Give report.rs the same line, with a test.'
history:
  - status: open
    by: skeptic
  - status: fixed
    by: product
    ref: "proposal.md; design.md; tasks.md 2.1"
  - status: fixed
    by: skeptic
```

#### R-skeptic-3 The steps the run selects were not the steps it adds

```yaml
severity: medium
confidence: high
location: 'design.md#Decision: An `added` list in `run.json`, appended by a resumed run'
evidence: |
  with the steps of the jobs `pending` actually runs (so `--case`,
recommendation: 'Record the steps of the jobs actually run, nothing when none, keep a repeated last entry; add scenarios.'
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    ref: "design.md record_added; specs scenarios A resume with nothing left to run and A resume of a few cases; tasks.md 1.2"
  - status: fixed
    by: skeptic
```

#### R-skeptic-4 Steps added under the same version string keep the run's date

```yaml
severity: low
confidence: high
location: 'design.md#Goals / Non-Goals'
evidence: |
  - Recording steps resumed under the same Compagnon version string: the
recommendation: 'Say it is out of scope, or record every addition.'
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    ref: "design.md Non-Goals"
  - status: fixed
    by: skeptic
```

## code.review

Mode: isolated.

### newcomer

#### R-newcomer-1 The comments said the entry names the steps that ran, before anything runs

```yaml
severity: medium
confidence: high
location: "evals/src/runner.rs:2563"
evidence: |
  pub fn record_added(
recommendation: 'Say the entry names the steps about to run, written before any case runs, and why an interrupted run still has them named.'
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "evals/src/runner.rs record_added doc; evals/src/main.rs comment"
  - status: fixed
    by: newcomer
```

#### R-newcomer-2 Two functions built the same line in two formats without saying why

```yaml
severity: low
confidence: high
location: "evals/src/report.rs:88"
evidence: |
  fn compagnon_versions(run: &RunInfo) -> String {
recommendation: 'Share one formatter, or say this is the raw-text variant of the chapter line and why.'
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "evals/src/report.rs doc comment naming reliability::compagnon_versions and the difference"
  - status: fixed
    by: newcomer
```

#### R-newcomer-3 A test helper's doc described another helper

```yaml
severity: low
confidence: high
location: "evals/src/runner.rs:3543"
evidence: |
  fn steps_added_with_another_compagnon() {
recommendation: 'Give each helper its own doc, say what read only means, and name the test after the behaviour.'
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "evals/src/runner.rs: recorded_with and read_only_cases docs; the test keeps the scenario name tasks.md 1.2 gives it"
  - status: fixed
    by: newcomer
```
