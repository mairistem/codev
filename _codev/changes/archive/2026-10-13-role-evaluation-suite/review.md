# Review record: role-evaluation-suite

<!-- codev-review: 1 -->

## plan.challenge

### skeptic

#### R-skeptic-1 The prompt alone keeps the agent inside one step

```yaml
severity: medium
confidence: high
location: "design.md#Decision: The runner isolates the evaluated role and runs one skill step"
evidence: |
  every finding `open`, modify no file but `review.md`, ask no question".
recommendation: >-
  Do not trust the prompt silently: after the run, record the files the
  agent modified outside what the step allows (from `git status`) and the
  findings it moved past `open`, and show them per case in the report, so
  a score obtained by an agent that rewrote the code under review is
  visible.
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    ref: design.md runner step 4 records `strayed` files from `git status --porcelain`; spec requirement "The runner runs each case alone" records them, new scenario "A run that strays outside the step"; the report shows strayed files and answered findings per case
  - status: fixed
    by: skeptic
    note: verified in the design, the spec and task 5.1
```

#### R-skeptic-2 Re-scoring the baseline cannot give its report if the report is dated now

```yaml
severity: medium
confidence: high
location: "design.md#Decision: The baseline is a recorded run, checked by the tests"
evidence: |
  `report.md`, `report.json` (with `model`, `date`, `codevVersion`), and
recommendation: >-
  The run's metadata (model asked, date, Claude Code and codev versions)
  belongs to the run, not to the scoring: write it once in the output
  directory when the run starts, and have `score` read it, so that scoring
  the same directory twice gives the same bytes.
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    ref: 'design.md baseline decision: `run.json` written once when a run starts, read by `score`; spec report requirement takes the metadata recorded at the start'
  - status: fixed
    by: skeptic
    note: verified; the baseline test can compare bytes
```

#### R-skeptic-3 `codev review` exits with 1 on the very records the suite must score

```yaml
severity: low
confidence: high
location: "design.md#Decision: The runner isolates the evaluated role and runs one skill step"
evidence: |
  4. run `codev review --change <change> --json` (review steps) and read the
recommendation: >-
  Say that the runner keeps the JSON whatever the exit code: exit 1 means
  the record has an error, which the scoring turns into `record-invalid`
  or an invalid evidence; only an unparsable output is a run error.
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    ref: design.md runner step 4 keeps the JSON whatever the exit code; only non-JSON output is a run error
  - status: fixed
    by: skeptic
    note: verified
```

#### R-skeptic-4 The baseline fails four roles on false positives

```yaml
severity: medium
confidence: high
location: "design.md#Goals / Non-Goals"
evidence: |
  - Tuning any role. A role that scores badly is recorded in the baseline
recommendation: >-
  The first real run (evals/baseline/report.md) gives recall 1.00 to every
  role but false positives over the default threshold of 2 for skeptic (8),
  privacy (6), ops (5) and accessibility (3). Read finding by finding,
  about ten are low-severity remarks reported with high confidence on clean
  code (ops: hard-coded constants and stringified errors in a clean retry
  loop; performance: a measurement behind a release note; newcomer: a
  module comment). Give the roles' Reporting sections a rule on
  low-severity remarks (confidence, or not reported on a control-like
  change) in a change of its own, measured against this baseline.
history:
  - status: open
    by: skeptic
  - status: deferred
    by: architect
    to: a new change `role-precision-from-baseline` (the roles' Reporting sections on low-severity remarks, measured against evals/baseline)
```

#### R-skeptic-5 Seeded defects are too easy and fixtures cite files they lack

```yaml
severity: medium
confidence: high
location: "design.md#Risks / Trade-offs"
evidence: |
  - [Seeded defects are too easy, so recall says little] → each seeded
recommendation: >-
  The risk materialized: every seeded defect was found, so recall cannot
  detect an improvement. And about nine false positives are the fixtures'
  fault — plans cite files or tests the fixture does not have
  (`web/src/settings/`, `src/cli/progress.rs`, a test named in a task) —
  while about ten point at real issues the cases did not seed (the
  accessibility control's hidden error referenced by `aria-describedby`, a
  merge job deleting personal data with no domain ticked, CSV escaping).
  Add harder cases (subtle defects, defects outside the obvious line) and
  complete the fixtures, then record the baseline again.
history:
  - status: open
    by: skeptic
  - status: deferred
    by: architect
    to: a new change `harder-eval-cases` (subtle seeded defects, complete fixtures, baseline recorded again)
```

#### R-skeptic-6 Location-only matching can pair a finding with the wrong defect

```yaml
severity: low
confidence: high
location: "design.md#Decision: Scoring rules, all in `score`"
evidence: |
  Two lines of tolerance: a reviewer citing the statement after the one the
recommendation: >-
  In the baseline, `accessibility-code-icon-button`'s button finding
  matched the image's expectation (two lines apart) and the image finding
  became a duplicate. The counts hold, but a pairing is not proof the
  right issue was named. Consider a category keyword on expected findings,
  or a one-to-one assignment that maximizes matches.
history:
  - status: open
    by: skeptic
  - status: deferred
    by: architect
    to: the change `harder-eval-cases` (category keywords or one-to-one assignment in the matching)
```

### security

#### R-security-1 `Bash(git:*)` lets the agent run any program

```yaml
severity: medium
confidence: high
location: "design.md#Decision: The runner isolates the evaluated role and runs one skill step"
evidence: |
  Bash(git:*) Read Write Edit Glob Grep Skill" --max-budget-usd <b>
recommendation: >-
  git runs programs through its configuration (`git -c core.pager=…`,
  `git config alias.x '!…'`), so the closed tool list is not closed
  (CWE-78). Allow only the read-only subcommands the review needs:
  `Bash(git diff:*)`, `Bash(git log:*)`, `Bash(git show:*)`,
  `Bash(git status:*)`.
history:
  - status: open
    by: security
  - status: fixed
    by: architect
    ref: design.md runner step 3 allows `git diff`, `git log`, `git show`, `git status` only, and says why
  - status: fixed
    by: security
    note: verified; the remaining Bash access is `codev`, which the fixture configures
```

#### R-security-2 Workflow inputs interpolated into a shell script

```yaml
severity: low
confidence: high
location: "design.md#Decision: The workflow is dispatch-only and checks its secret first"
evidence: |
  `evals.yml`: `on: workflow_dispatch` with optional inputs (model,
recommendation: >-
  Pass the inputs to the run step through `env:` and quote them in the
  script, never `${{ inputs.model }}` inside `run:`: a dispatcher could
  otherwise inject commands in a job that holds the API key. Only writers
  can dispatch, hence low.
history:
  - status: open
    by: security
  - status: fixed
    by: architect
    ref: design.md workflow decision passes the inputs through `env:`, quoted, never interpolated into `run:`
  - status: fixed
    by: security
    note: verified
```

### ops

#### R-ops-1 A timed-out run leaves Claude Code's children running

```yaml
severity: medium
confidence: high
location: "design.md#Decision: The runner isolates the evaluated role and runs one skill step"
evidence: |
  `PATH`, killed after the timeout;
recommendation: >-
  Start Claude Code in its own process group and terminate the group on
  timeout (on Unix), so the shells and tools it spawned stop spending;
  say what happens on Windows.
history:
  - status: open
    by: ops
  - status: fixed
    by: architect
    ref: design.md runner step 3 starts Claude Code in its own process group and terminates the group (TERM, then KILL after five seconds); Windows kills the process only and is unsupported
  - status: fixed
    by: ops
    note: verified
```

#### R-ops-2 The workflow's timeout is shorter than a full run, and loses its output

```yaml
severity: medium
confidence: high
location: "design.md#Decision: The workflow is dispatch-only and checks its secret first"
evidence: |
  runs the suite with a job timeout of 120 minutes, and uploads the output
recommendation: >-
  Twenty-six cases at up to fifteen minutes each, two at a time, exceed
  two hours: upload the output with `if: always()` so a stopped job keeps
  what it measured, size the job timeout to the per-case timeout, and
  lower the per-case default.
history:
  - status: open
    by: ops
  - status: fixed
    by: architect
    ref: 'design.md workflow decision: job timeout 240 minutes against a 130-minute worst case, upload with `if: always()`; spec lowers the per-case timeout to 600 s'
  - status: fixed
    by: ops
    note: verified
```

## code.review

### newcomer

#### R-newcomer-1 `missed` builds the expectations before any match

```yaml
severity: low
confidence: high
location: "evals/src/score.rs:142"
evidence: |
  fn missed(case: &Case) -> Vec<ExpectedScore> {
recommendation: >-
  The function returns every expectation unmatched, the starting point of
  the scoring, but its name reads as "the misses". Name it after what it
  returns, such as `unmatched_expectations`.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: evals/src/score.rs, `unmatched_expectations`
  - status: fixed
    by: newcomer
    note: verified
```

#### R-newcomer-2 A failed Claude Code run with output is not an error, silently

```yaml
severity: low
confidence: high
location: "evals/src/runner.rs:502"
evidence: |
  if !status.success() && out.trim().is_empty() {
recommendation: >-
  Say why a non-zero exit with output is not an error here: Claude Code
  exits non-zero on a budget or turn limit and still prints its JSON,
  whose `is_error` the caller reads.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: evals/src/runner.rs, a comment says Claude Code prints its JSON on a budget or turn limit and the caller reads `is_error`
  - status: fixed
    by: newcomer
    note: verified
```

### security

#### R-security-3 The workflow installs whatever Claude Code version npm serves

```yaml
severity: medium
confidence: high
location: ".github/workflows/evals.yml:58"
evidence: |
  run: npm install -g @anthropic-ai/claude-code
recommendation: >-
  The job holds the API key and runs the package it installs: pin the
  version (the baseline's, 2.1.289), so that a compromised or broken
  release is not pulled silently and runs are comparable with the
  baseline; raise it deliberately.
history:
  - status: open
    by: security
  - status: fixed
    by: developer
    ref: .github/workflows/evals.yml installs `@anthropic-ai/claude-code@2.1.289` (checked on the npm registry), with a comment
  - status: fixed
    by: security
    note: verified
```

#### R-security-4 Killing a process group by a pid that may have been reused

```yaml
severity: low
confidence: low
location: "evals/src/runner.rs:521"
evidence: |
  let group = format!("-{}", child.id());
recommendation: >-
  If the child exited and its pid was reused, `kill -- -<pid>` would signal
  another process group. Check the child is still running right before.
history:
  - status: open
    by: security
  - status: disputed
    by: developer
    note: >-
      `terminate` runs only after `try_wait` returned `None`, and the child
      is reaped only by the `wait` that follows the kills: until then its pid
      is a zombie's at worst, never reused, and the group it leads keeps its
      id. No other process can be signalled.
  - status: rejected
    by: security
    note: agreed, an unreaped child keeps its pid
```

### ops

#### R-ops-3 A resumed run can mix models without saying so

```yaml
severity: medium
confidence: high
location: "evals/src/runner.rs:673"
evidence: |
  if path.exists() {
recommendation: >-
  `run.json` of a resumed directory is kept as is: resuming with another
  `CODEV_EVAL_MODEL` scores observations of two models under the first
  one's name. Refuse to resume when the model asked differs from the one
  recorded, naming both, and say how to start a new directory.
history:
  - status: open
    by: ops
  - status: fixed
    by: developer
    ref: evals/src/runner.rs `check_resume`, called by `run` before resuming (exit 2 naming both models); spec scenario "Resuming with another model", test `a_resumed_run_with_another_model_is_refused`
  - status: fixed
    by: ops
    note: verified
```
