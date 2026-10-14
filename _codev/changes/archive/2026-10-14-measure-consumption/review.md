# Review record: measure-consumption

<!-- codev-review: 1 -->

## plan.challenge

Mode: isolated.

### skeptic

#### R-skeptic-1 The consumption scenario's token figures contradict the design's sum over `modelUsage`

```yaml
severity: medium
confidence: high
location: "specs/role-evaluation/spec.md#Requirement: The runner runs each case alone, headlessly, with Claude Code"
evidence: |
  - **GIVEN** a run for which Claude Code reports 9 turns, 14 uncached
    input tokens, 29 349 tokens written to the cache, 304 094 read from
    it and 2 713 output tokens
recommendation: Choose one rule and align the spec, the design and the test fixture — state the summed figures in the scenario, or say that what Claude Code reports means the sum over every model.
history:
  - status: open
    by: skeptic
  - status: fixed
    by: qa
    ref: "specs/role-evaluation/spec.md — the requirement says tokens are summed over every model and records the models used; the scenario gives both models' figures and the sums (1 187 input, 2 728 output); task 1.2's test uses the sums"
  - status: fixed
    by: skeptic
```

#### R-skeptic-2 The per-repetition layout leaves raw `claude.json` outside the baseline's gitignore

```yaml
severity: medium
confidence: high
location: "design.md#Decision: One directory per repetition, `<out>/<case>/<n>/`"
evidence: |
  Each repetition writes its `observed.json`, `claude.json`,
  `claude.stderr` and `review.md` under `<out>/<case>/<n>/`, `n` from 1, and
recommendation: Change the patterns to `baseline/**/claude.json` and `baseline/**/claude.stderr`, list `evals/.gitignore` in the Impact, and make the baseline test fail if a raw log is tracked.
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    ref: "task 6.0 (gitignore and the test `no_raw_log_is_tracked_in_the_baseline`), the baseline requirement and its scenario \"A raw log in the baseline\", the proposal's Impact and a risk in design.md"
  - status: fixed
    by: skeptic
```

#### R-skeptic-3 "Constant model, which the command enforces" compares only the model that was asked for

```yaml
severity: medium
confidence: high
location: "design.md#Decision: `compare` re-scores both directories and uses the baseline's range as the noise band"
evidence: |
  At a constant model the cost is the tokens weighted by their price, so it is the
recommendation: Record the models actually used, from `modelUsage`, in format 2, refuse a comparison when they differ, and report a different `claudeVersion`.
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    ref: "observations record the models used; compare refuses different models asked or used and names both Claude Code versions (spec scenarios \"The same default, another model used\" and \"Another Claude Code version\", tasks 1.1, 4.1, 4.2)"
  - status: fixed
    by: skeptic
```

#### R-skeptic-4 The cost verdict has no noise band, so an unchanged candidate passes about half the time

```yaml
severity: medium
confidence: high
location: "design.md#Decision: `compare` re-scores both directories and uses the baseline's range as the noise band"
evidence: |
  Consumption follows the same logic: the candidate's total cost (sum of
  per-case medians) must be lower than the cheapest single repetition of
recommendation: Require the candidate's total to fall below the lowest total the baseline's repetitions produced, and test that two runs of the same code do not pass on cost.
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    ref: "the candidate must beat the baseline's cheapest repetition (spec scenarios \"A gain within the baseline's spread\" and \"The same code compared with itself\", test `a_gain_within_the_baseline_spread_fails`, task 6.2); CONTRIBUTING recommends the baseline's `--jobs`"
  - status: fixed
    by: skeptic
  - status: fixed
    by: skeptic
    note: "Re-checked after a refinement during implementation: the bar is the baseline's floor, each case at its cheapest run, instead of the cheapest whole repetition. With costs skewed across repetitions, medians can sum below every repetition's total, so the unchanged code could have passed; the floor has no such gap."
    ref: "evals/src/compare.rs (floor), test `the_same_code_never_passes_on_cost_however_its_runs_spread`; spec and design updated"
```

#### R-skeptic-5 A baseline repetition in error makes every later comparison fail

```yaml
severity: medium
confidence: high
location: "specs/role-evaluation/spec.md#Requirement: Two runs are compared, with a verdict on quality and consumption"
evidence: |
  A repetition in error in either
  run SHALL fail the verdict, naming the run, the case and the repetition.
recommendation: Make the baseline test fail when any repetition is in error, and have resume re-run errored repetitions.
history:
  - status: open
    by: skeptic
  - status: fixed
    by: qa
    ref: "resume runs a repetition recorded in error again (scenario \"A repetition recorded in error is run again\"); the baseline holds no repetition in error (scenario \"A baseline repetition in error\"); tasks 2.2 and 6.1"
  - status: fixed
    by: skeptic
```

#### R-skeptic-6 The workflow's timeout is not raised for `repeat`

```yaml
severity: low
confidence: high
location: "tasks.md#5. Documentation and workflow"
evidence: |
  - [ ] 5.4 Add a `repeat` input (default 1) to `.github/workflows/evals.yml`, reaching the script through an `env:` entry (`EVAL_REPEAT`) used quoted as `--repeat "$EVAL_REPEAT"`, never interpolated into `run:`; set `timeout-minutes` to the hosted maximum, 360, and say in its comment that a full repeated baseline is recorded locally; verified by the new suite tests `the_workflow_passes_repeat_through_the_environment` and `no_workflow_input_is_interpolated_in_a_script`, and by `the_workflow_runs_only_on_dispatch` and `the_workflow_checks_the_secret_before_running` still passing
recommendation: Bound the run, set `timeout-minutes` to match, and test that the workflow passes `repeat` to `--repeat`.
history:
  - status: open
    by: skeptic
  - status: fixed
    by: lead
    ref: "task 5.4: timeout at the hosted maximum, full baseline recorded locally, test `the_workflow_passes_repeat_through_the_environment`; risk added in design.md"
  - status: fixed
    by: skeptic
```

#### R-skeptic-7 The reliability headline is not one of the medians the report gives

```yaml
severity: low
confidence: low
location: "specs/role-evaluation/spec.md#Requirement: A reliability chapter is generated from the committed baseline"
evidence: |
  The scores the page shows SHALL be the medians the report gives.
recommendation: State how each headline figure is computed over repetitions, have the report give it, and test it.
history:
  - status: open
    by: skeptic
  - status: fixed
    by: qa
    ref: "the report requirement gives the suite-level figures per repetition then their median and range (scenario \"The suite's recall over repetitions\"); tasks 3.1 and 5.1"
  - status: fixed
    by: skeptic
```

### security

#### R-security-1 Bypassing permissions hands the API key and the host's credentials to whatever the reviewed fixtures tell the agent to run

```yaml
severity: high
confidence: low
location: "design.md#Decision: Approve every shell command inside Claude Code's sandbox"
evidence: |
  **Alternatives considered**: `--permission-mode bypassPermissions` lets a
  prompt injection in a fixture or a role file run anything with the
recommendation: Do not rely on the prompt as the security boundary. Keep compound commands working but contain the agent — Claude Code's sandbox through `--settings` (Bash writes confined to the case directory, network limited to the Anthropic API) or a container without egress — start `claude` with a cleared environment plus an allow-list, deny network tools, and name fixtures and role files as untrusted input in Risks.
history:
  - status: open
    by: security
  - status: fixed
    by: architect
    note: "Option chosen by the user (Ludovic) among the three presented: Claude Code's sandbox rather than bypass, auto mode or an accepted risk."
    ref: "design.md Decision: Approve every shell command inside Claude Code's sandbox — acceptEdits, sandbox with Bash auto-allowed, writes in the case's directory, network to the API, key hidden, no unsandboxed retry, fail if unavailable, WebFetch and WebSearch disallowed; checked by a probe run (compound command ran; writes to $HOME, curl, printenv and a Write to $HOME denied); spec scenarios \"A write outside the case and a network call are denied\" and \"A machine without the sandbox\"; tasks 2.1, 5.4 and 6.2"
  - status: fixed
    by: security
```

#### R-security-2 The `strayed` check is presented as a containment, but it cannot see anything outside the case repository

```yaml
severity: medium
confidence: high
location: "design.md#Risks / Trade-offs"
evidence: |
  `strayed` check is no containment — it only sees the case's own git
recommendation: Either add a real filesystem boundary, or state plainly in the design and in CONTRIBUTING that a local run can modify any file the user can write, and recommend a disposable environment.
history:
  - status: open
    by: security
  - status: fixed
    by: architect
    ref: "design.md Risks states the check is no containment and refers the boundary to R-security-1; task 5.2 has CONTRIBUTING say a run can modify any file and reach the network, and to use a disposable environment"
  - status: fixed
    by: security
```

#### R-security-3 The new `repeat` workflow input does not say it must reach the script through the environment

```yaml
severity: low
confidence: low
location: "tasks.md#5. Documentation and workflow"
evidence: |
  reaching the script through an `env:` entry (`EVAL_REPEAT`) used quoted as `--repeat "$EVAL_REPEAT"`, never interpolated into `run:`
recommendation: Say in the task that `repeat` goes through an `env:` entry used quoted, never `${{ inputs.repeat }}` inside `run:`, and check that no `${{ inputs.` appears in a `run:` block.
history:
  - status: open
    by: security
  - status: fixed
    by: lead
    ref: "task 5.4 and the test `no_workflow_input_is_interpolated_in_a_script`"
  - status: fixed
    by: security
```

### ops

#### R-ops-4 The sandbox is not checked before any case runs, so a machine without it ends with every repetition in error

```yaml
severity: low
confidence: high
location: "specs/role-evaluation/spec.md#Requirement: The runner runs each case alone, headlessly, with Claude Code"
evidence: |
    the run SHALL refuse to start when the sandbox is unavailable on the
    machine, rather than run without it;
recommendation: "Add the sandbox to the preflight that checks `claude`, `git` and `codev`: before any case runs, check what the sandbox needs (bubblewrap and socat on Linux) and exit with code 2 naming it; align the scenario \"A machine without the sandbox\": one refusal for the run, not one error per case."
history:
  - status: open
    by: ops
  - status: fixed
    by: qa
    ref: "spec: the runner checks the sandbox before any case runs and exits with code 2 naming what is missing (scenario \"A machine without the sandbox\" rewritten); evals/src/runner.rs `sandbox_missing` in the preflight, test `a_machine_without_the_sandbox_is_refused_once`; design updated."
  - status: fixed
    by: ops
```

#### R-ops-5 The baseline is said to be recorded at the workflow's pinned Claude Code version, but it is recorded locally where nothing pins it

```yaml
severity: low
confidence: high
location: "design.md#Decision: The baseline is recorded again with five repetitions"
evidence: |
  The baseline becomes the run of the 40 cases with `--repeat 5` and open
  permissions, at the current Claude Code version pinned by the workflow,
recommendation: "Have CONTRIBUTING's baseline steps install the pinned version, and add a suite test that fails when the `claudeVersion` of `evals/baseline/run.json` differs from the version pinned in `.github/workflows/evals.yml`."
history:
  - status: open
    by: ops
  - status: fixed
    by: architect
    ref: "design: the baseline is recorded with the pinned version, installed as CONTRIBUTING (both languages) now says; suite test `the_baseline_is_recorded_with_the_pinned_claude_code` compares `run.json` with the workflow's pin."
  - status: fixed
    by: ops
```

## code.review

Mode: isolated.
Summarized: evals/baseline/** and docs/{en,fr}/src/reliability.md — about 480 generated files (observations, fixture review records, reports, generated pages), summarized as a whole by one sub-agent rather than one per file.

### security

#### R-security-4 The runner runs `git status` unsandboxed, with the API key in its environment, inside a tree the sandboxed agent can write

```yaml
severity: medium
confidence: low
location: "evals/src/runner.rs:688-694"
evidence: |
      if let Ok(status) = git(
          tools,
          &dir,
          &["status", "--porcelain", "-z", "--untracked-files=all"],
      ) {
          observed.strayed = strayed(&status, case);
      }
recommendation: "Treat the case directory as untrusted once the agent has run: run the post-run git with `-c core.fsmonitor=false`, `GIT_CONFIG_NOSYSTEM=1` and an empty global configuration, remove `ANTHROPIC_API_KEY` from the environment of every tool the runner starts, or fail the run when `.git/config` differs from what `setup` wrote; test the arguments."
history:
  - status: open
    by: security
  - status: fixed
    by: developer
    ref: "evals/src/runner.rs: every tool the runner starts (`tool_command`) runs without ANTHROPIC_API_KEY, with GIT_CONFIG_NOSYSTEM and an empty global configuration; git runs with `core.fsmonitor=false` next to `core.hooksPath`; a run whose agent changed `.git/config` is recorded in error (`config_tampered`). Tests `git_runs_no_hook_and_no_file_system_monitor` (fails without the fix), `the_runner_starts_its_tools_without_the_key_or_a_global_configuration`, `a_modified_git_config_is_an_error`; a real run checked that a normal run is not flagged."
  - status: fixed
    by: security
```

#### R-security-5 CONTRIBUTING says the agent can write only in the case's directory, but the sandbox and the file tools allow more

```yaml
severity: low
confidence: low
location: "CONTRIBUTING.md:99-103"
evidence: |
  The agent runs in Claude Code's sandbox, as a user working with it would:
  every shell command is approved without a prompt, but it can write only
  in the case's temporary directory, reach no host but the Anthropic API,
  and not read the API key; a write elsewhere or another network call is
  denied and counted in the report. On macOS the sandbox is built in. On
recommendation: "State the boundary as the design gives it: shell commands write in the working directory and the system temporary directory, which holds every case directory; the file tools run outside the sandbox; keep the advice to run the suite in a disposable environment; same in CONTRIBUTING.fr.md."
history:
  - status: open
    by: security
  - status: fixed
    by: developer
    ref: "CONTRIBUTING.md and CONTRIBUTING.fr.md: the boundary as the design gives it — writes in the case's directory and the system's temporary directory, which holds the other cases' too; file tools outside the sandbox; a `.git/config` change recorded in error; run the suite on a machine or in a container you can throw away."
  - status: fixed
    by: security
```

### newcomer

#### R-newcomer-1 `score::suite` grew into a ~195-line function mixing four computations

```yaml
severity: low
confidence: high
location: "evals/src/score.rs:827-829"
evidence: |
  pub fn suite(run: RunInfo, mut cases: Vec<CaseScore>, thresholds: Thresholds) -> SuiteScore {
      cases.sort_by(|a, b| a.id.cmp(&b.id).then(a.repetition.cmp(&b.repetition)));
      let repetitions: Vec<u32> = {
recommendation: "Split `suite` along its seams: per-case consumption medians, one role score, the suite figures; `suite` only assembles them."
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "evals/src/score.rs: `suite` assembles `case_consumption`, `role_score`, `threshold_failures` and `suite_figures`, over a named `Repetition`; the baseline's report is unchanged byte for byte (`the_baseline_agrees_with_the_scoring`)."
  - status: fixed
    by: newcomer
```

#### R-newcomer-2 `compare::report` is a ~190-line function with seven parameters and boolean flags

```yaml
severity: low
confidence: high
location: "evals/src/compare.rs:201-209"
evidence: |
  fn report(
      baseline: &SuiteScore,
      candidate: &SuiteScore,
      not_compared: &[String],
      quality_only: bool,
      base_floor: f64,
      consumption_failures: &[String],
      failures: &[String],
  ) -> String {
recommendation: "Split `report` per section, pass the verdict pieces as one struct, replace the `range(s, bool)` closure with named helpers, and make `compare`’s mode an enum instead of a bare `bool`."
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "evals/src/compare.rs: `compare` takes a `Mode` (`QualityAndCost`, `QualityOnly`); the report is one function per section over a `Verdict`; `ratio_range` and `count_range` replace the boolean closure."
  - status: fixed
    by: newcomer
```

#### R-newcomer-3 Consumption measures are selected by string names and array positions

```yaml
severity: low
confidence: high
location: "evals/src/compare.rs:304-308"
evidence: |
      let label = |name: &str| match name {
          "cost_usd" => "Cost (USD)",
          "duration_ms" => "Duration (s)",
          "duration_api_ms" => "API duration (s)",
          "turns" => "Turns",
recommendation: "Introduce a `Measure` enum carrying its label and format; have `Measures::fields()` return `(Measure, Option<f64>)` and let both reports and the median/sum code iterate over it."
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "evals/src/score.rs: a `Measure` enum carries each measure's label, column name and format; `Measures::get`, `median_of` and `sum_of` iterate over `Measure::ALL`; both reports use it."
  - status: fixed
    by: newcomer
```

#### R-newcomer-4 Hard-to-read "first run of this case" test in the per-case report

```yaml
severity: low
confidence: high
location: "evals/src/report.rs:191-202"
evidence: |
          let runs = score
              .case_consumption
              .iter()
              .find(|m| m.id == c.id)
              .map_or(1, |m| m.repetitions);
          if c.repetition
              == score
                  .cases
                  .iter()
                  .find(|r| r.id == c.id)
                  .map_or(1, |r| r.repetition)
          {
recommendation: "Group the runs by case (`chunk_by` on the sorted runs), write the heading once per group and loop over its runs."
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "evals/src/report.rs: the runs are grouped by case with `chunk_by`; heading and consumption written once per group."
  - status: fixed
    by: newcomer
```

#### R-newcomer-5 `bool` meaning "in error" in `BTreeMap<(String, u32), bool>` across modules

```yaml
severity: low
confidence: high
location: "evals/src/suite.rs:110-112"
evidence: |
  /// The repetitions recorded in `dir`, each mapped to whether it is in
  /// error. An unreadable observation counts as in error, so it is run again.
  pub fn recorded(dir: &Path) -> BTreeMap<(String, u32), bool> {
recommendation: "Name the value: an enum such as `RunState { Done, InError }` behind a `Recorded` alias used by `suite::recorded`, `runner::pending` and the checks."
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "evals/src/suite.rs: `RunState { Done, InError }` and `Recorded`, used by `suite::recorded`, `runner::pending` and the checks."
  - status: fixed
    by: newcomer
```

#### R-newcomer-6 `run_case` parameter renamed from `work` to `dir`, next to `case_dir`

```yaml
severity: low
confidence: high
location: "evals/src/runner.rs:643-649"
evidence: |
  /// Runs one repetition of a case in `dir` and returns its observation;
  /// never fails, a failure is the observation's `error`.
  pub fn run_case(
      case: &Case,
      case_dir: &Path,
      dir: &Path,
      log: &Path,
recommendation: "Rename it `work_dir` and drop the `let dir = dir.to_path_buf();` shadowing."
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "evals/src/runner.rs: `run_case` takes `work_dir`, without the shadowing."
  - status: fixed
    by: newcomer
```

#### R-newcomer-7 `--repeat` declared as a `String` and parsed by hand, without saying why

```yaml
severity: low
confidence: low
location: "evals/src/main.rs:102-105"
evidence: |
      rerun: bool,
      /// Run each case this many times, each repetition recorded apart
      #[arg(long, default_value = "1")]
      repeat: String,
recommendation: "Declare `repeat: u32` with `value_parser!(u32).range(1..)`, which clap rejects with exit code 2, or say why it is parsed by hand."
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "evals/src/main.rs: `repeat: u32` with `value_parser!(u32).range(1..)`; `parse_repeat` removed; `repeat_zero_exits_with_two` still passes."
  - status: fixed
    by: newcomer
```

### ops

#### R-ops-1 The workflow's timeout comment misstates how long a repetition takes

```yaml
severity: low
confidence: high
location: ".github/workflows/evals.yml:39-42"
evidence: |
      # The hosted maximum. 40 cases take about 40 minutes per repetition at
      # two jobs; a full baseline of five repetitions does not fit and is
      # recorded locally (CONTRIBUTING.md).
      timeout-minutes: 360
recommendation: "Base the comment on measured figures (about 21 minutes of agent time per repetition of the 40 cases), give the worst case (600 s x 40 cases / jobs) and the largest safe `repeat`, and put the same limit in the input's description."
history:
  - status: open
    by: ops
  - status: fixed
    by: developer
    ref: ".github/workflows/evals.yml: the comment gives the measured 21 minutes of agent time per repetition, about 6 at four jobs, the worst case of 100 minutes per repetition, and the bound of three repetitions; the input's description says the same."
  - status: fixed
    by: ops
```

#### R-ops-2 A run stopped by the 360-minute timeout uploads observations but no report

```yaml
severity: low
confidence: low
location: ".github/workflows/evals.yml:24-27"
evidence: |
        repeat:
          description: "Runs of each case (a full baseline of five is recorded locally)"
          required: false
          default: "1"
recommendation: "Reject a `repeat` above the safe bound in the run step, with a message saying to record larger runs locally, or document that a cut-off run is scored again with `codev-evals score <artifact>`."
history:
  - status: open
    by: ops
  - status: fixed
    by: developer
    ref: ".github/workflows/evals.yml: the run step refuses a `repeat` other than 1, 2 or 3 before the suite runs, saying to record more locally; test `the_workflow_passes_repeat_through_the_environment` checks the bound comes first."
  - status: fixed
    by: ops
```

#### R-ops-3 The workflow records runs with two jobs, CONTRIBUTING asks for the baseline's four, and no run records its job count

```yaml
severity: low
confidence: low
location: ".github/workflows/evals.yml:103-103"
evidence: |
            cargo run --locked -p codev-evals -- run --out evals-output --repeat "$EVAL_REPEAT" "${args[@]}"
recommendation: "Pass `--jobs 4` in the workflow; record `jobs` in `run.json` and have `compare` say when the two runs differ, as it does for the Claude Code version."
history:
  - status: open
    by: ops
  - status: fixed
    by: developer
    ref: "the workflow passes `--jobs 4`; `run.json` records `jobs` (the baseline's set to 4, as recorded); `compare` names both job counts and says when they differ (test `another_job_count_is_reported`); spec updated."
  - status: fixed
    by: ops
```
