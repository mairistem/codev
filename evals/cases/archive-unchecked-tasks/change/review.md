# Review record: fewer-review-turns

<!-- codev-review: 1 -->

## plan.challenge

Mode: isolated.

### skeptic

#### R-skeptic-1 The procedure switches to the human output of `codev review` but keeps the JSON's field names

```yaml
severity: medium
confidence: high
location: "tasks.md#2. The skills"
evidence: |
  and by the existing test `propose_and_apply_describe_the_record_and_the_loop`, updated to expect `codev review --change "<name>"` without `--json` (scenario "The skills describe the record and the loop")
recommendation: "Have task 2.1 reword steps a, g and h of the review-record procedure in the terms of the human output (its waiting list, its issues, its low-confidence mark) instead of `findings[]`, `awaiting`, `issues[]` and `reported`, and update the test's expected keywords; otherwise the agent looks for `issues[]`, falls back to `--json` and a filter, and spends the turn the change saves."
history:
  - status: open
    by: skeptic
  - status: fixed
    by: lead
    ref: "task 2.1 rewords steps a, g and h in the human output's terms and updates the test's keywords"
  - status: fixed
    by: skeptic
```

#### R-skeptic-2 `disabled` is undefined when the project sets `never` and the change sets `always`

```yaml
severity: low
confidence: high
location: "specs/roles/spec.md#Requirement: `codev roles` serves the roles of a step"
evidence: |
  With `--at`, `disabled` SHALL list, in workflow order, the roles that
  serve the step but that the project or the change sets to `never`, each
recommendation: "State that `disabled` holds the roles whose resolved activation is `never`, add a scenario (project `never`, change `always`: in `roles`, not in `disabled`) and cover it in task 1.1's tests."
history:
  - status: open
    by: skeptic
  - status: fixed
    by: qa
    ref: "roles spec: `disabled` holds the roles whose resolved activation is `never`, `whenOrigin` the file whose setting won; scenario \"A change turns a role back on\"; task 1.1 test `a_role_turned_back_on_by_the_change_is_not_disabled`"
  - status: fixed
    by: skeptic
```

#### R-skeptic-3 Nothing says what happens when the measurement does not beat the floor

```yaml
severity: low
confidence: low
location: "tasks.md#5. Measure"
evidence: |
  verified by the comparison's verdict: quality within the baseline's range for every role (required), cost under the baseline's floor (the goal), the turns and denials reported
recommendation: "State the outcome: when quality holds but cost does not beat the floor, the human decides whether to keep the change, and if it is kept the baseline is recorded again anyway."
history:
  - status: open
    by: skeptic
  - status: fixed
    by: lead
    ref: "task 5.1: quality failing stops the change; cost not under the floor goes to the human; task 5.2 records the baseline again whenever the change is kept"
  - status: fixed
    by: skeptic
```

### performance

#### R-performance-1 The claimed gain in turns and denials is reported but never checked

```yaml
severity: medium
confidence: high
location: "tasks.md#5. Measure"
evidence: |
  verified by the comparison's verdict: quality within the baseline's range for every role (required), cost under the baseline's floor (the goal), the turns and denials reported
recommendation: "Add an explicit check to 5.1: the candidate's turns and permission denials against the baseline's, and no `codev review --json | python3` or `codev roles … | python3` left among the denials; word the release note from what the comparison shows."
history:
  - status: open
    by: performance
  - status: fixed
    by: lead
    ref: "task 5.1 checks the turns and denials against the baseline and that no codev-output filter is left among the denials; the release note is worded from the comparison"
  - status: fixed
    by: performance
```

## code.review

Mode: isolated.
Summarized: crates/codev-agents/src/workflows.rs — over the material budget; its diff fit in two hundred lines and was returned verbatim.
Summarized: crates/codev-cli/src/commands.rs — over the material budget; its diff fit in two hundred lines and was returned verbatim.
Summarized: crates/codev-cli/src/contract.rs — over the material budget; its diff fit in two hundred lines and was returned verbatim.
Summarized: crates/codev-cli/src/main.rs — over the material budget; its diff fit in two hundred lines and was returned verbatim.
Summarized: crates/codev-cli/src/render.rs — over the material budget; its diff fit in two hundred lines and was returned verbatim.
Summarized: crates/codev-core/src/roles.rs — over the material budget; its diff fit in two hundred lines and was returned verbatim.
Summarized: docs/fr/src/reference/cli.md — over the material budget; its diff fit in two hundred lines and was returned verbatim.
Summarized: evals/src/runner.rs — over the material budget; its diff fit in two hundred lines and was returned verbatim.

### performance

#### R-performance-2 The "call of its own" rule added in 2.1.b raises turns above the baseline, and the turns check in 5.1 is ticked though it failed

```yaml
severity: high
confidence: low
location: "assets/workflows/_review-record.md:5-11"
evidence: |
  Read codev's output as it prints it: each command gives what the step
  needs, so never filter it through another program (`python3`, `jq`) — a
  command your session may not approve, which costs a turn. Run each
  `codev` command as a call of its own, never chained after `cd`, `&&` or
  `;`: the skill's grant covers a command that starts with `codev`, not a
  chain. Independent calls — `codev` ones, file reads — go together in one
  turn.
recommendation: "Drop the 'call of its own' half and keep the first run's balance (or reword it and measure again); state the turn count and the right denial ratio in the release note; do not tick 5.1 while its own check fails."
history:
  - status: open
    by: performance
  - status: fixed
    by: developer
    ref: "The rule is removed from `_review-record.md`, the skills delta and the test; the design records the trial and its numbers; the release note gives the first run's measures (denials −46 %, re-read −16 %, turns −6 %, time −12 %, cost −7 %); task 5.1 states the outcome on the run of the shipped text, with the cost criterion not met."
  - status: fixed
    by: performance
```

#### R-performance-3 Task 5.2 would complete the pre-2.1.b run into the new baseline

```yaml
severity: low
confidence: low
location: "_codev/changes/fewer-review-turns/tasks.md:25-25"
evidence: |
  - [ ] 5.2 If the change is kept, complete the candidate to five repetitions (`run --repeat 5 --out target/evals/fewer-turns`), make it the new `evals/baseline/`, and regenerate the reliability chapter (`codev-evals docs`); verified by `cargo test -p codev-evals` (five repetitions, none in error, pinned Claude Code, pages follow the baseline)
recommendation: "Point 5.2 at the run that measured the shipped text, and check 'none in error' once its repetitions are complete."
history:
  - status: open
    by: performance
  - status: fixed
    by: developer
    ref: "Task 5.2 completes `target/evals/fewer-turns`, the run of the shipped text, to five repetitions; the completion of the other run was stopped; the baseline test checks none is in error."
  - status: fixed
    by: performance
```

### newcomer

#### R-newcomer-1 The cli reference says the disabled roles are on a "last line", but render prints them in the header

```yaml
severity: medium
confidence: high
location: "docs/en/src/reference/cli.md:755-757"
evidence: |
  With `--at`, a last line names the roles of the step that the
  configuration turned off, each with the file whose setting did it
  (`project` or `change`). The skills call it with `--json`; see
recommendation: "Make the doc and the output agree: move the line after the active roles, as the spec says, or reword both docs."
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-cli/src/render.rs: `disabled_line` writes the line last, after the active roles (or after the line saying none is active), as the roles spec and both cli references say; `roles_list_the_roles_disabled_at_the_step` asserts it is the last line."
  - status: fixed
    by: newcomer
```

#### R-newcomer-2 `disabled_at` calls `active_at` and discards the result, only to validate the step

```yaml
severity: low
confidence: high
location: "crates/codev-engine/src/roles.rs:40-45"
evidence: |
      /// The roles serving `step` that the configuration turned off;
      /// `unknown_step` when `step` is not one of the workflow steps.
      pub fn disabled_at(&self, step: &str) -> Result<Vec<&ResolvedRole>> {
          self.active_at(step)?;
          Ok(core_roles::disabled_at(&self.roles, step))
      }
recommendation: "Move the step check into a private helper both methods call, so the line says what it does."
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-engine/src/roles.rs: `check_step`, called by `active_at` and `disabled_at`."
  - status: fixed
    by: newcomer
```
