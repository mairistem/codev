# Review record: explore-onboard-evaluation

<!-- compagnon-review: 1 -->

## plan.challenge

Mode: isolated.

### skeptic

#### R-skeptic-1 The reply checks could not tell which outcome the agent concluded on

```yaml
severity: medium
confidence: high
location: "design.md#Decision: The final message is checked, for its own outcome and against the others"
evidence: |
  `contains` and `absent` are the only checks on the reply, beside the
recommendation: 'Check the final message only, require absent marks of the other outcomes or states, add a scenario for a wrong conclusion, and check it on the trial run.'
history:
  - status: open
    by: skeptic
  - status: fixed
    by: qa
    ref: "specs: final message, absent required, scenarios A reply concluding on another outcome and Narration is not the conclusion; design decision; tasks 1.3, 3.1"
  - status: fixed
    by: skeptic
```

#### R-skeptic-2 The evaluation prompt pushed explore away from its missing outcome

```yaml
severity: medium
confidence: high
location: "design.md#Decision: The evaluation mode lets the skill end as it would"
evidence: |
  A headless run has no user to answer. The prompt says so and asks the
recommendation: 'Say the user cannot answer and to end as the skill says where it would ask; drop the update clause.'
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    ref: "specs runner requirement and its prompt scenario; design decision on the evaluation mode"
  - status: fixed
    by: skeptic
```

#### R-skeptic-3 The several-changes row of onboard's table had no case

```yaml
severity: medium
confidence: high
location: "specs/read-only-skills-evaluation/spec.md#Requirement: The suite covers explore's outcomes and onboard's recommendations"
evidence: |
  state (`no-compagnon`, `thin-config`, `configured`,
recommendation: 'Add the sixth state, an expected.state field, and fix the case count and cost.'
history:
  - status: open
    by: skeptic
  - status: fixed
    by: qa
    ref: "specs: expected.state with six values, coverage; proposal; design cost; tasks 2.1, 3.1"
  - status: fixed
    by: skeptic
```

#### R-skeptic-4 The new capability had no Purpose

```yaml
severity: low
confidence: high
location: "specs/read-only-skills-evaluation/spec.md#Purpose"
evidence: |
  Measure the two read-only skills a newcomer meets first,
recommendation: 'Add a Purpose section before the requirements.'
history:
  - status: open
    by: skeptic
  - status: fixed
    by: qa
    ref: "specs: ## Purpose"
  - status: fixed
    by: skeptic
```

## code.review

Mode: isolated.

### newcomer

#### R-newcomer-1 A test name described an input, not the behaviour it checks

```yaml
severity: medium
confidence: high
location: "evals/src/case.rs:3436"
evidence: |
  fn read_only_gaps_names_each_outcome_and_state_without_a_case() {
recommendation: 'Name each test after the behaviour it checks; a_missing_state reads as a case without its state key.'
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "evals/src/case.rs: a_missing_state renamed; the other tests keep the scenario names tasks.md gives them (1.1 to 1.4), as in the configure tests"
  - status: fixed
    by: newcomer
```

#### R-newcomer-2 The compare comments still named only sync, archive and configure

```yaml
severity: medium
confidence: high
location: "evals/src/compare.rs:758"
evidence: |
  fn checked_pairs<'a>(
recommendation: 'Say the check-scored steps include explore and onboard.'
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "evals/src/compare.rs: doc comments of checked_pairs and checked_quality"
  - status: fixed
    by: newcomer
```

#### R-newcomer-3 load_read_only repeats one rule in two mirrored branches

```yaml
severity: low
confidence: high
location: "evals/src/case.rs:1777"
evidence: |
  fn load_read_only(id: &str, step: Step, yaml: &str, fixture: &Fixture) -> Result<Case, CaseError> {
recommendation: 'Extract the outcome and state parsing into one helper.'
history:
  - status: open
    by: newcomer
  - status: deferred
    by: developer
    to: "the change converting explore and onboard to plain words"
    note: 'A refactor of working loading code, tested by the four loading scenarios; it goes with the next change that touches these steps.'
```

#### R-newcomer-4 The runner's test helpers paired outcomes with the wrong marks

```yaml
severity: low
confidence: high
location: "evals/src/runner.rs:4468"
evidence: |
  fn read_only_case(id: &str, situation: crate::case::Situation) -> Case {
recommendation: 'Give the helpers neutral or consistent marks, and say they do not matter to the tests.'
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "evals/src/runner.rs: neutral marks and a comment"
  - status: fixed
    by: newcomer
```

#### R-newcomer-5 A case promised a check it does not make

```yaml
severity: low
confidence: high
location: "evals/cases/onboard-planning-incomplete/case.yaml:12"
evidence: |
  absent: ["/compagnon-configure", "compagnon init"]
recommendation: 'Say why /compagnon-apply is not absent, and stop promising it in the summary.'
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "evals/cases/onboard-planning-incomplete/case.yaml: summary and comment"
  - status: fixed
    by: newcomer
```
