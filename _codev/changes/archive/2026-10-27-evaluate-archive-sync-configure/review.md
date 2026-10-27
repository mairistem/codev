# Review record: evaluate-archive-sync-configure

<!-- codev-review: 1 -->

## plan.challenge

Mode: isolated.

### skeptic

#### R-skeptic-1 The review-refusal case puts `review.md` in `overlay/`, which the spec's loader rejects

```yaml
severity: medium
confidence: high
location: "design.md#Decision: Sync and archive cases replay history, like update cases"
evidence: |
  optional `overlay/` lays other files on the tree — a configuration with
recommendation: Edit the reopened `high` finding into `change/review.md`, as the other refusal cases edit `change/`, and drop `review.md` from the overlay example; say so in task 5.1.
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    ref: "design.md (the overlay holds only the configuration; the finding is reopened in change/review.md, at a location that exists at ref); tasks.md 5.1"
  - status: fixed
    by: skeptic
```

#### R-skeptic-2 Changing an expectation later does not give the per-case verdict the design promises the waiting changes

```yaml
severity: medium
confidence: high
location: "design.md#Decision: Sync and archive cases replay history, like update cases"
evidence: |
  These record today's behavior. The waiting changes then change the
recommendation: Specify a per-case verdict for the check-scored steps, and state how the baseline stays consistent when a case's expectation changes, in the design and in the CONTRIBUTING text of task 6.1.
history:
  - status: open
    by: skeptic
  - status: fixed
    by: qa
    ref: "specs/sync-archive-evaluation and specs/configure-evaluation: per-case verdict against the baseline rescored, the report naming every case not passing in all runs, scenarios 'One case regresses inside the step's range', 'A case whose expectation changed', 'One configure case regresses'; design.md (rescoring without a model); tasks.md 2.5, 3.4, 6.1"
  - status: fixed
    by: skeptic
```

#### R-skeptic-3 A `refused` case does not check which refusal it covers

```yaml
severity: medium
confidence: high
location: "specs/sync-archive-evaluation/spec.md#Requirement: A sync or archive run is scored by deterministic checks"
evidence: |
  - for a `refused` case, every file under `_codev/` after the run is the
    fixture's;
recommendation: Add an optional `expected.code` matched against the reference's `status[0].code`, a mismatch ending the case in error as stale, with a scenario and a test.
history:
  - status: open
    by: skeptic
  - status: fixed
    by: qa
    ref: "specs/sync-archive-evaluation: `code` in the case, its load failure on a done case, the error on another code, scenario 'A refusal for another reason than the case's'; design.md table; tasks.md 2.1, 2.4"
  - status: fixed
    by: skeptic
```

#### R-skeptic-4 The context line count cannot be checked when the configuration is read as YAML

```yaml
severity: medium
confidence: high
location: "specs/configure-evaluation/spec.md#Requirement: A configure run is scored by deterministic checks"
evidence: |
  - for a `written` case: every top-level key of `_codev/config.yaml` but
recommendation: Say what is counted — the non-empty lines of the `context` value in the file's text — and add a scenario for a context on a single line.
history:
  - status: open
    by: skeptic
  - status: fixed
    by: qa
    ref: "specs/configure-evaluation: lines counted in the text whatever the YAML style, scenario 'A context on a single line'; design.md Risks; tasks.md 3.3"
  - status: fixed
    by: skeptic
```

#### R-skeptic-5 `sync-planning-incomplete` expects `/codev-propose`, but the skill points to `/codev-continue` for this state

```yaml
severity: medium
confidence: low
location: "design.md#Decision: Sync and archive cases replay history, like update cases"
evidence: |
  | `sync-planning-incomplete` | sync | as `archive-planning-incomplete` | `refused`, reply does not hold `Synced` |
recommendation: Check the stop by the file comparison and an `absent` text, and note that the skill's mention of `/codev-continue`, which codev does not install, is a defect of the skill left to a change of its own.
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    ref: "design.md: the case checks the stop with `absent: Synced`, and the paragraph on `/codev-continue`"
  - status: fixed
    by: skeptic
```

#### R-skeptic-6 The proposal lists a second change's work under its own What Changes

```yaml
severity: low
confidence: high
location: "proposal.md#Impact"
evidence: |
  - Follow-up, not built here: `evaluate-review-meeting`, described in the
recommendation: Keep What Changes to what this change builds; name the follow-up elsewhere.
history:
  - status: open
    by: skeptic
  - status: fixed
    by: product
    ref: "proposal.md: the bullet moved from What Changes to Impact as a follow-up not built here"
  - status: fixed
    by: skeptic
```

## code.review

Mode: isolated.

### newcomer

#### R-newcomer-1 `is_checked` and the "checked" family leave out `plan.update`, which is check-scored too

```yaml
severity: medium
confidence: high
location: "evals/src/case.rs:92-98"
evidence: |
      /// The steps scored by named checks besides `plan.update`: each run
      /// passes when every check does.
      pub fn is_checked(self) -> bool {
recommendation: "Name the predicate and its companions after what they actually select (sync, archive, configure), so that a reader does not take them for every check-scored step."
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "evals/src/case.rs, evals/src/score.rs, evals/src/compare.rs, evals/src/runner.rs (renames)"
    note: "Renamed `Step::is_merge_or_configure`, `SuiteScore::merge_and_configure_steps`, and in compare.rs `merge_and_configure_pairs`, `_quality`, `_sections`; their docs say `plan.update` keeps its own section."
  - status: fixed
    by: newcomer
```

#### R-newcomer-2 The sync, archive and configure figures are typed `UpdateFigures`

```yaml
severity: medium
confidence: high
location: "evals/src/reliability.rs:216-222"
evidence: |
      /// The sync step, when the baseline holds sync runs.
      pub change_sync: Option<UpdateFigures>,
      /// The archive step, when the baseline holds archive runs.
recommendation: "Rename `UpdateFigures` to a step-neutral `CheckStepFigures`, keeping `UpdateFigures` as an alias."
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "evals/src/reliability.rs (CheckStepFigures)"
    note: "`CheckStepFigures`, documented as the figures of any check-scored step; `UpdateFigures` kept as an alias."
  - status: fixed
    by: newcomer
```

#### R-newcomer-3 `checked_section` hides the configure case behind `_` and builds the same `skill` twice

```yaml
severity: low
confidence: high
location: "evals/src/reliability.rs:1257-1300"
evidence: |
      fn checked_section(out: &mut String, step: Step, f: &UpdateFigures, lang: Lang) {
          let (title_en, title_fr) = match step {
              Step::ChangeSync => ("Syncing a change", "La synchronisation d'un change"),
recommendation: "Match `Step::ConfigureEnrich` explicitly here and in `report.rs::checked_noun`, and compute `skill` once."
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "evals/src/reliability.rs (checked_section), evals/src/report.rs (checked_noun)"
    note: "Both matches name `Step::ConfigureEnrich` and reach `unreachable!` for another step; `skill` is computed once before the texts."
  - status: fixed
    by: newcomer
```

#### R-newcomer-4 `without_archive_date` checks a date with bare offsets

```yaml
severity: low
confidence: high
location: "evals/src/merge.rs:23-31"
evidence: |
      let dated = rest.len() > 11
          && rest.as_bytes()[10] == b'-'
recommendation: "Name the shape: a `DATE_LEN` constant and an `is_iso_date` helper."
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "evals/src/merge.rs (DATE_LEN, is_iso_date)"
    note: "`DATE_LEN = \"YYYY-MM-DD\".len()` and `is_iso_date`; the body reads `rest.get(..DATE_LEN).is_some_and(is_iso_date) && rest[DATE_LEN..].starts_with('-')`."
  - status: fixed
    by: newcomer
```

#### R-newcomer-5 `score_written` runs eight checks in one 60-line body, with a duplicated loop

```yaml
severity: low
confidence: low
location: "evals/src/configure.rs:106-166"
evidence: |
  fn score_written(expect: &ConfigureCase, seen: Option<&ConfigureObserved>, checks: &mut Checks) {
      let before = seen.and_then(|s| s.before.as_deref());
recommendation: "Pull out `keys_preserved` and `comments_kept` as named predicates, fold the `written` and `unwritten` loops, rename `part`."
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "evals/src/configure.rs (keys_preserved, comments_kept, proposal_text)"
    note: "`keys_preserved` and `comments_kept` extracted; `part` renamed `proposal_text`. The two loops stay apart: each writes a check named differently (`holds` / `does not hold`), and a helper taking the polarity would bring back a boolean parameter."
  - status: fixed
    by: newcomer
```

#### R-newcomer-6 Tests named for one behavior assert several unrelated ones, and boolean helpers hide what each run is

```yaml
severity: low
confidence: high
location: "evals/src/case.rs:2875-2900"
evidence: |
      fn an_overlay_link_fails() {
          let sync = ARCHIVE_YAML.replace("change.archive", "change.sync");
recommendation: "Split tests into one per behavior, and replace the boolean helpers `done_run(.., through_codev: bool, ..)` and `unchanged_run(passes: bool)` with named constructors."
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "evals/src/merge.rs, evals/src/configure.rs, evals/src/report.rs, evals/src/compare.rs (named run constructors)"
    note: "The boolean helpers are replaced by `run_through_codev` / `run_merged_by_hand` and `run_leaving_the_file` / `run_writing_the_file`. The tests keep one test per spec scenario, named after it as tasks.md asks; the extra assertions are the scenario's loading rules (the spec lists the overlay's three refusals together)."
  - status: fixed
    by: newcomer
```
