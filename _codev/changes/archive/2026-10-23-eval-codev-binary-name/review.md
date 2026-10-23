# Review record: eval-codev-binary-name

<!-- codev-review: 1 -->

## plan.challenge

Mode: isolated.

### skeptic

#### R-skeptic-1 The Why invalidates the Sonnet runs too, but the plan re-records only Opus

```yaml
severity: medium
confidence: high
location: "tasks.md#3. The implementation baseline again"
evidence: |
  - [x] 3.1 Record the `code.implement` baseline again on Opus with the frozen binary named `codev`; verified by `every_case_has_a_baseline_observation` and by no run's transcript holding `unrecognized subcommand`
recommendation: Add a task that re-records the Sonnet runs with the binary named `codev`, or say in the proposal that they are out of scope and change the Why to match.
history:
  - status: open
    by: skeptic
  - status: fixed
    by: lead
    ref: "proposal.md#Why: the Sonnet runs are kept outside the repository and measured again outside this change"
  - status: fixed
    by: skeptic
```

#### R-skeptic-2 No task updates docs/en and docs/fr, yet re-recording changes both reliability pages

```yaml
severity: medium
confidence: high
location: "tasks.md#2. Documentation"
evidence: |
  - [x] 2.1 `CONTRIBUTING.md` and `CONTRIBUTING.fr.md`: the binary frozen for a comparison keeps the name `codev`, in a folder of its own; verified by the suite's test `contributing_chapters_say_the_frozen_binary_keeps_its_name`
recommendation: Add a task that regenerates both reliability pages from the new baseline, verified by `the_reliability_pages_follow_the_baseline`, and list both pages under Impact.
history:
  - status: open
    by: skeptic
  - status: fixed
    by: lead
    ref: "tasks.md 3.3 and proposal.md#Impact: both reliability pages regenerated"
  - status: fixed
    by: skeptic
```

#### R-skeptic-3 The new implementation-evaluation scenario is an authoring guideline that no test can check

```yaml
severity: medium
confidence: high
location: "specs/implementation-evaluation/spec.md#Scenario: A task that validates with the change's own codev"
evidence: |
  - **WHEN** the case is written
recommendation: Add a suite test that fails when a non-refusing case's `change/tasks.md` calls bare `codev validate` or `codev doctor`, and checks its `summary` mentions the rewording; it also catches implement-fix-merge-and-skill-refresh's task 4.1.
history:
  - status: open
    by: skeptic
  - status: fixed
    by: qa
    ref: "evals/tests/suite.rs implementation_cases_check_the_repository_with_the_codev_the_change_builds; the scenario now triggers on the suite tests; fix-merge-and-skill-refresh reworded too (task 3.2)"
  - status: fixed
    by: skeptic
```

## code.review

Mode: isolated.

### newcomer

#### R-newcomer-1 `codev_name_problem` returns None ("name is right") also when the path has no file name

```yaml
severity: low
confidence: high
location: "evals/src/runner.rs:1079"
evidence: |
  pub fn codev_name_problem(codev: &Path) -> Option<String> {
recommendation: Handle the missing file name explicitly and report it as a problem, so the code and its doc comment agree.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "evals/src/runner.rs codev_name_problem: a path naming no file is a problem, tested"
  - status: fixed
    by: newcomer
```
