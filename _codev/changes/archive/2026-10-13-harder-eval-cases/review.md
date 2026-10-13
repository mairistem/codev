# Review record: harder-eval-cases

<!-- codev-review: 1 -->

## plan.challenge

### skeptic

#### R-skeptic-1 Tuning the vocabulary after the new run would fit the measure to the output

```yaml
severity: medium
confidence: high
location: "design.md#Decision: The category is recognised by terms, from a vocabulary kept with the suite"
evidence: |
  The vocabulary is tuned once, on the first baseline's 26 recorded
recommendation: >-
  Nothing stops the vocabulary from being edited again once the new run
  is in: adding terms until every miss becomes a true positive would
  inflate recall by construction. Freeze the vocabulary before the new
  run, and require any later edit to be listed in the design with the
  findings it re-classifies, each read by a person and naming its seeded
  defect.
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    ref: design.md vocabulary decision freezes the vocabulary for the new run; a later edit is listed under "Baseline before and after" with the findings it re-classifies, each read by a person
  - status: fixed
    by: skeptic
    note: verified in the design
```

#### R-skeptic-2 The first baseline cannot be re-scored with its old cases under the new loader

```yaml
severity: medium
confidence: high
location: "tasks.md#5. Cases"
evidence: |
  - [ ] 5.1 Re-score the first baseline (old cases, old observations, copied outside the repository) under the new matching and tune `evals/categories.yaml` until every finding that named its seeded defect stays a true positive and the icon button's findings pair the right way round, verified by `cargo run -p codev-evals -- score <copy>` and the per-case lines of its report
recommendation: >-
  `score` loads the cases through `case::load`, which after task 2.2
  refuses the old cases the lint rejects (`skeptic-control-small-flag`
  cites files its base lacks). Re-score the old observations against the
  completed cases instead, after 5.2, and keep the lines that existing
  expectations point to unchanged while completing the fixtures, so that
  the old observations still describe the code they cite.
history:
  - status: open
    by: skeptic
  - status: fixed
    by: lead
    ref: tasks.md 5.1 completes the cases first, keeping the lines expectations point to; 5.2 re-scores the first baseline against the completed cases, then freezes the vocabulary; design.md baseline decision says the same
  - status: fixed
    by: skeptic
    note: verified; the copied observations carry their own evidence issues, so the re-scoring needs no new run
```

#### R-skeptic-3 The spec checks bare names inside commands, the design skips them

```yaml
severity: low
confidence: high
location: "specs/role-evaluation/spec.md#Requirement: A case declares its fixture, its step, its role and what it expects"
evidence: |
  heading (`#…`) is ignored; a path inside a backquoted command is
  checked the same way;
recommendation: >-
  The design says a bare name inside a command (`ledger import
  data.csv`) is an argument and is skipped; the spec says a path inside
  a command is checked "the same way", which includes bare names. Say in
  the spec that only a path holding a `/` is checked inside a command.
history:
  - status: open
    by: skeptic
  - status: fixed
    by: qa
    ref: spec requirement on cases says only words holding a `/` are checked inside a command, a bare name there is an argument
  - status: fixed
    by: skeptic
    note: verified; spec and design agree
```

## code.review

### newcomer

#### R-newcomer-1 Two unnamed options in the classification's result

```yaml
severity: low
confidence: high
location: "evals/src/score.rs:349-354"
evidence: |
  ) -> (Class, Option<String>, Option<String>) {
recommendation: >-
  `classify_unmatched` returns the class, the matched expected finding
  and the expected finding nearby as a tuple of two `Option<String>`; a
  reader of the call site cannot tell which is which. Return a small
  struct with named fields.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: evals/src/score.rs, `Unpaired { class, expected, near }` returned by `classify_unmatched` and destructured at the call site
  - status: fixed
    by: newcomer
    note: verified
```

#### R-newcomer-2 A required field nobody reads, silenced without a reason

```yaml
severity: low
confidence: high
location: "evals/src/case.rs:243"
evidence: |
  #[allow(dead_code)]
recommendation: >-
  `CategoryFile::description` is deserialized and never used, with the
  warning silenced. Say why it is kept: it is required so that every
  category of `evals/categories.yaml` is explained to case authors.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: evals/src/case.rs, a doc comment on `description` says it is required for case authors and that the scoring reads only the terms
  - status: fixed
    by: newcomer
    note: verified
```
