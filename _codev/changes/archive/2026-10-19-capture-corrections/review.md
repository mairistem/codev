# Review record: capture-corrections

<!-- codev-review: 1 -->

## plan.challenge

Mode: isolated.

### skeptic

#### R-skeptic-2 `corrections.md` becomes plan.challenge material for every reviewer, which the plan neither handles nor measures

```yaml
severity: medium
confidence: high
location: "design.md#Goals / Non-Goals"
evidence: |
  written once, in the change, in a format a later codev step can read. The
  capture adds no reading, and only a short text to three skills. Quality
recommendation: "Decide whether corrections are review material; if not, filter `corrections.md` out next to `review.md`, with a scenario and a test such as `plan_material_skips_corrections_md`."
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    note: "New decision \"Corrections are not review material\"; review-mode delta and task 2.2 leave corrections.md out of the plan material."
    ref: "specs/review-mode/spec.md"
  - status: fixed
    by: skeptic
```

#### R-skeptic-3 Making unknown fields warnings does not protect older codevs under `--strict`, which is the reason the design gives for it

```yaml
severity: medium
confidence: high
location: "design.md#Decision: The review record's format, with four fields and tolerant unknown fields"
evidence: |
  must read entries without guessing their shape. Strict unknown fields —
recommendation: "Make the tolerant cases info-level or exempt from --strict, or keep them as warnings and correct the rationale, the Risks and the Migration text to state the real strict-mode outcome."
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    note: "Kept as warnings, like the review record and the configuration; the design now states that an older codev never errors nor misreads, that --strict fails until it updates, which codev_version and the update notice tell, and why exempting them from --strict is rejected."
    ref: "design.md#Decision: The review record's format, with four fields and tolerant unknown fields"
  - status: fixed
    by: skeptic
```

#### R-skeptic-1 The measurement leaves `codev-update` and the capture during `/codev-apply` unevaluated, which decision 0008 does not allow

```yaml
severity: medium
confidence: high
location: "design.md#Decision: Measured against the current planning and reviews"
evidence: |
  Following decision 0008, the change adds text to three skills read on
  every call, so it must show no quality loss and a bounded cost. The
recommendation: "Cover codev-update (and apply beyond code.review) first, in a separate change, as 0008 requires; or state explicitly that those steps are not measured and get that exception accepted. Add a cost bound for the apply skill at code.review. (performance) Only plan.write checks cost; code.review reads codev-apply and has no cost check, and no step reads codev-update: report the skill source at code.review, and bound update by the fragment's measured token size or record it as unmeasured."
raised_by: [skeptic, performance]
history:
  - status: open
    by: skeptic
  - status: disputed
    by: architect
    note: "The fragment is the same text in the three skills, so its cost per call is now measured exactly for all three (task 6.1, at most 400 tokens per rendered skill), and the skill source of code.review is reported (6.3). What stays unmeasured is the quality of codev-update and of apply outside code.review, a gap that predates this change; the design states it and leaves the choice to the author."
    ref: "design.md#Decision: Measured against the current planning and reviews"
  - status: disputed
    by: skeptic
    note: "Decision 0008 says a step the suite does not cover is covered first, by its own change; shipping before that is an exception only a person can accept."
  - status: fixed
    by: human:Ludovic
    note: "The author chose to cover /codev-update first: #49 adds its cases and baseline; this change passes them (18/18 runs, cost -2.4 %)."
    ref: "#49"
```

### privacy

#### R-privacy-1 The one-line notice does not show the author what was written, but the plan relies on it to keep personal data out of git

```yaml
severity: medium
confidence: high
location: "design.md#Risks / Trade-offs"
evidence: |
  - [An entry carries personal data or a secret from the conversation into
recommendation: "Require the notice to quote the entry as written (where, did, asked) so the author can refuse or edit it before commit; extend the scenario and the test of task 3.1, and align the Risks line."
history:
  - status: open
    by: privacy
  - status: fixed
    by: qa
    note: "The skills requirement, its scenario, the design and task 3.1 now have the agent quote the entry it wrote to the author."
    ref: "specs/skills/spec.md"
  - status: fixed
    by: privacy
```

#### R-privacy-2 No retention or removal path for a corrections entry once committed and archived

```yaml
severity: medium
confidence: low
location: "specs/corrections/spec.md#Requirement: The corrections file is not a planning artifact"
evidence: |
  changelog` SHALL NOT read it. `codev archive` moves it with the change.
recommendation: "State that corrections.md is kept as long as the archive, and how an author removes an entry with personal data or a secret; document it in the reference page of task 5.1."
history:
  - status: open
    by: privacy
  - status: fixed
    by: lead
    note: "The design's Risks and task 5.1's reference page state the retention and the removal procedure."
    ref: "tasks.md#5. Documentation"
  - status: fixed
    by: privacy
```

#### R-privacy-3 The personal-data domain is ticked but no purpose or lawful basis is stated

```yaml
severity: low
confidence: low
location: "proposal.md#Domains"
evidence: |
  - [x] personal-data
recommendation: "Name the data, its purpose and the basis on the Domains line."
history:
  - status: open
    by: privacy
  - status: fixed
    by: product
    note: "The Domains line names the data (the instruction, tied to its committer by git) and its purpose (project knowledge within the team)."
    ref: "proposal.md#Domains"
  - status: fixed
    by: privacy
```

### performance

#### R-performance-1 The 3 % total-cost gate is smaller than the run-to-run noise it is measured against

```yaml
severity: medium
confidence: high
location: "tasks.md#6. Measure, then improve (decision 0008)"
evidence: |
  and the total cost must be at most 3 % above the latest main runs, with the `skill` source reported;
recommendation: "Gate on the skill tokens per call, which barely move between runs, with a ceiling in tokens; report the total cost as a median per case against the baseline's spread, and apply the same rule to the re-measurement."
history:
  - status: open
    by: performance
  - status: fixed
    by: lead
    note: "Task 6.1 measures each rendered skill's token size before and after (at most 400 tokens); 6.2 and 6.3 report cost and the skill source per case as medians against the baseline's spread; 6.4 credits a cut only outside that spread."
    ref: "tasks.md#6. Measure, then improve (decision 0008)"
  - status: fixed
    by: performance
```

### migration

#### R-migration-1 Corrections defects are mixed into the record's issues, which installed skills fix in review.md

```yaml
severity: medium
confidence: high
location: "specs/review-record/spec.md#Requirement: `codev review` shows the record and what is left to decide"
evidence: |
  location and what the author asked, under a section `Corrections`
recommendation: "List corrections defects in their own section and in a separate JSON field so that issues keeps its v1 meaning, or update the review-record fragment to fix each issue in its own file."
history:
  - status: open
    by: migration
  - status: fixed
    by: qa
    note: "Option (a): defects under the Corrections section and in correctionsIssues, never in issues nor in the exit code; scenario and task 2.1 updated."
    ref: "specs/review-record/spec.md"
  - status: fixed
    by: migration
```

#### R-migration-2 An existing user file named corrections.md becomes validated without a migration path

```yaml
severity: medium
confidence: low
location: "design.md#Migration Plan"
evidence: |
  Nothing to migrate: the file is new and optional
recommendation: "State the case in the Migration Plan with a release-note line, or require the format marker before any entry is checked."
history:
  - status: open
    by: migration
  - status: fixed
    by: qa
    note: "The format marker is required; a file without it is neither checked nor listed (new scenario \"A file without the marker is not read\", tested in 1.1 and 2.1)."
    ref: "specs/corrections/spec.md"
  - status: fixed
    by: migration
```

## code.review

Mode: isolated.
Summarized: crates/codev-agents/src/claude.rs — over the material limit with the whole change (1,112,760 bytes against 163,840).
Summarized: crates/codev-agents/src/workflows.rs — over the material limit with the whole change (1,112,760 bytes against 163,840).
Summarized: crates/codev-cli/src/commands.rs — over the material limit with the whole change (1,112,760 bytes against 163,840).
Summarized: crates/codev-cli/src/contract.rs — over the material limit with the whole change (1,112,760 bytes against 163,840).
Summarized: crates/codev-cli/src/docs.rs — over the material limit with the whole change (1,112,760 bytes against 163,840).
Summarized: crates/codev-cli/src/main.rs — over the material limit with the whole change (1,112,760 bytes against 163,840).
Summarized: crates/codev-cli/src/render.rs — over the material limit with the whole change (1,112,760 bytes against 163,840).
Summarized: crates/codev-engine/src/validate/mod.rs — over the material limit with the whole change (1,112,760 bytes against 163,840).
Summarized: docs/en/src/reference/cli.md — over the material limit with the whole change (1,112,760 bytes against 163,840).
Summarized: docs/en/src/reference/file-formats.md — over the material limit with the whole change (1,112,760 bytes against 163,840).
Summarized: docs/en/src/reference/json-output.md — over the material limit with the whole change (1,112,760 bytes against 163,840).
Summarized: docs/fr/src/reference/cli.md — over the material limit with the whole change (1,112,760 bytes against 163,840).
Summarized: docs/fr/src/reference/file-formats.md — over the material limit with the whole change (1,112,760 bytes against 163,840).
Summarized: docs/fr/src/reference/json-output.md — over the material limit with the whole change (1,112,760 bytes against 163,840).
Summarized: evals/src/runner.rs — over the material limit with the whole change (1,112,760 bytes against 163,840).

### privacy

No finding.

### performance

No finding.

### migration

#### R-migration-3 The capture fragment has no rule for a corrections.md that exists without the format marker

```yaml
severity: medium
confidence: low
location: "assets/workflows/fragments/memory-capture.md:5-7"
evidence: |
  an artifact — note it once in the change's `corrections.md`: the line
  `<!-- codev-corrections: 1 -->` first when the file is new, then
recommendation: "Add one sentence: when corrections.md exists without the marker, it is not codev's file; do not write to it nor add the marker, and tell the author the correction could not be noted; assert it in a codev-agents test."
history:
  - status: open
    by: migration
  - status: deferred
    by: developer
    to: "the second change of the lessons loop, which reworks the capture"
    note: "Low confidence, and the plan.write measurement runs with this exact fragment; changing it now would ship an unmeasured skill text."
```

### newcomer

#### R-newcomer-1 `Reader::block` hides a positional coupling between FIELDS and the destructured names

```yaml
severity: medium
confidence: high
location: "crates/codev-core/src/corrections.rs:264-278"
evidence: |
  let [step, r#where, did, asked] = match <[String; 4]>::try_from(values) {
recommendation: "Read each field by name through a helper, combine with let-else, and move the unknown-field and length checks into their own functions."
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    note: "required_text, warn_unknown_fields and warn_long_text; the fields are bound by name, with a comment on the early return."
    ref: "crates/codev-core/src/corrections.rs"
  - status: fixed
    by: newcomer
```

#### R-newcomer-2 The unknown-step test builds and parses the same source twice

```yaml
severity: low
confidence: high
location: "crates/codev-core/src/corrections.rs:367-377"
evidence: |
  fn an_unknown_step_names_the_accepted_ones() {
recommendation: "Bind the source once, as the neighbouring tests do."
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-core/src/corrections.rs"
  - status: fixed
    by: newcomer
```

#### R-newcomer-3 The invalid-corrections archive test builds a broken file by deleting text from a literal

```yaml
severity: low
confidence: high
location: "crates/codev-engine/src/archive.rs:241-244"
evidence: |
  let fs = well_formed_project().with_file(
recommendation: "Declare one CORRECTIONS constant in the test module and name the broken variant at the call site."
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    note: "A CORRECTIONS constant in the archive tests, and a named missing_asked variant."
    ref: "crates/codev-engine/src/archive.rs"
  - status: fixed
    by: newcomer
```

#### R-newcomer-4 The changelog test's fixture looks like a mistake without a comment

```yaml
severity: low
confidence: high
location: "crates/codev-engine/src/changelog.rs:314-317"
evidence: |
  let with_corrections = project().with_file(
recommendation: "Add a one-line comment saying the content is shaped like a release note on purpose."
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-engine/src/changelog.rs"
  - status: fixed
    by: newcomer
```
