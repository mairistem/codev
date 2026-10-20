# Review record: isolated-review-evaluation

<!-- codev-review: 1 -->

## plan.challenge

Mode: isolated.

### skeptic

#### R-skeptic-1 The guard against unnoticed inline runs checks the wrong thing

```yaml
severity: medium
confidence: high
location: "design.md#Risks / Trade-offs"
evidence: |
  - [A skill rendered without the sub-agent tool runs the role inline,
recommendation: "Check after the run that the case really ran isolated (record and Agent calls)."
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    note: "The runner checks Mode: isolated., no Fallback: line and a sub-agent started after the run; the rendered-skill check is dropped."
    ref: "design.md#Decision: The mode and extra settings go into the fixture's configuration"
  - status: fixed
    by: skeptic
```

#### R-skeptic-2 Per-call and per-source token measures are left wrong in the isolated mode

```yaml
severity: medium
confidence: high
location: "specs/role-evaluation/spec.md#Requirement: The runner can play the review cases in the isolated mode"
evidence: |
  The run's cost and token totals SHALL be those Claude Code reports for
recommendation: "Specify how sub-agent calls are read, or that the measures are not valid in the isolated mode."
history:
  - status: open
    by: skeptic
  - status: fixed
    by: qa
    note: "The tokens beyond the session's own calls are the source sub-agents; a run whose totals fall below its calls fails; task 1.2 tests it on canned streams."
    ref: "specs/role-evaluation/spec.md"
  - status: fixed
    by: skeptic
```

#### R-skeptic-3 Two runs with different appended configurations are compared silently

```yaml
severity: medium
confidence: high
location: "design.md#Goals / Non-Goals"
evidence: |
  **Goals**: play the existing review cases in the `isolated` mode with no
recommendation: "Name both runs' appended configurations in the report and say when they differ."
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    note: "The compare requirement names both texts and says when they differ; scenario and test added."
    ref: "specs/role-evaluation/spec.md"
  - status: fixed
    by: skeptic
```

#### R-skeptic-4 Task 1.2's pass condition cannot be computed from what the transcript holds

```yaml
severity: medium
confidence: low
location: "tasks.md#1. The runner"
evidence: |
  - [ ] 1.3 Run one isolated review case with Claude Code and check what only a real session shows
recommendation: "Express the check in tokens, and make the runner check or report it."
history:
  - status: open
    by: skeptic
  - status: fixed
    by: lead
    note: "The runner reports the sub-agents source and fails a run whose totals fall below its calls; the real run only checks a sub-agent started and the source is above zero."
    ref: "tasks.md#1. The runner"
  - status: fixed
    by: skeptic
```

#### R-skeptic-5 The new options have no error or edge cases

```yaml
severity: low
confidence: high
location: "specs/role-evaluation/spec.md#Requirement: The runner can play the review cases in the isolated mode"
evidence: |
  SHALL append the YAML of `<file>` to each review case's
recommendation: "Exit 2 before any case runs on an unknown mode, a bad file or a duplicate review key; decide what planning-only runs record."
history:
  - status: open
    by: skeptic
  - status: fixed
    by: qa
    note: "Scenario \"Options that cannot be applied\" and its test; the mode guard in compare applies only when both runs hold review cases."
    ref: "specs/role-evaluation/spec.md"
  - status: fixed
    by: skeptic
```

#### R-skeptic-6 The scenario's example setting is rejected by today's codev

```yaml
severity: low
confidence: high
location: "specs/role-evaluation/spec.md#Scenario: A configuration appended to every case"
evidence: |
  - **GIVEN** a file holding `roles: { skeptic: { done_when: ["The rollback
recommendation: "Use a setting that exists today."
history:
  - status: open
    by: skeptic
  - status: fixed
    by: qa
    note: "The example uses done_when."
    ref: "specs/role-evaluation/spec.md"
  - status: fixed
    by: skeptic
```

#### R-skeptic-7 The design cites none of the decisions that constrain it

```yaml
severity: low
confidence: high
location: "design.md#Decision: The mode and extra settings go into the fixture's configuration"
evidence: |
  case is then a project in that mode, as a user's would be; nothing in the
recommendation: "Cite decisions 0007 and 0008 and explain how the plan honours them."
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    note: "The design cites 0007 (fallback, permission) and 0008 (tokens traced to their source)."
    ref: "design.md"
  - status: fixed
    by: skeptic
```

## code.review

Mode: isolated.

The reviewer read each file's diff and the functions it touches directly, without summaries.

### newcomer

#### R-newcomer-1 The per-call measures recover the session's own tokens by undoing the subtraction trace_sub_agents just did

```yaml
severity: low
confidence: high
location: "evals/src/score.rs:833-843"
evidence: |
  let own = tokens.map(|t| {
recommendation: "Compute own from the session's calls, or say in the comment that it equals their sum."
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    note: "The comment says the difference is the sum of the session's calls, set by record_sub_agent_tokens."
    ref: "evals/src/score.rs"
  - status: fixed
    by: newcomer
```

#### R-newcomer-2 trace_sub_agents computes and stores a value, while its name and docstring suggest tracing

```yaml
severity: low
confidence: high
location: "evals/src/runner.rs:365-368"
evidence: |
  pub fn trace_sub_agents(consumption: &mut Consumption) -> Result<(), String> {
recommendation: "Rename it for what it does and reword the docstring."
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    note: "Renamed record_sub_agent_tokens, docstring reworded."
    ref: "evals/src/runner.rs"
  - status: fixed
    by: newcomer
```

#### R-newcomer-6 configured() is a name that does not say what the function returns

```yaml
severity: low
confidence: high
location: "evals/src/runner.rs:289-292"
evidence: |
  pub fn configured(existing: &str, review: &ReviewSettings) -> Option<String> {
recommendation: "Name it for the content it builds."
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    note: "Renamed config_with_review_settings."
    ref: "evals/src/runner.rs"
  - status: fixed
    by: newcomer
```

#### R-newcomer-3 compare() takes a bare boolean across_models, so its call sites end in an unexplained `false`

```yaml
severity: low
confidence: high
location: "evals/src/compare.rs:327-333"
evidence: |
  across_models: bool,
recommendation: "Use a two-variant enum or an options struct."
history:
  - status: open
    by: newcomer
  - status: deferred
    by: developer
    to: "the next change to codev-evals"
    note: "Readability only; left out to keep this diff small."
```

#### R-newcomer-4 The test name covers one of the four behaviours it checks

```yaml
severity: low
confidence: high
location: "evals/src/runner.rs:2744-2771"
evidence: |
  fn an_isolated_review_case_says_so_in_its_configuration() {
recommendation: "Split it into tests named for each behaviour."
history:
  - status: open
    by: newcomer
  - status: deferred
    by: developer
    to: "the next change to codev-evals"
    note: "Readability only; left out to keep this diff small."
```

#### R-newcomer-5 fixture_configs builds its Markdown from fragments, so the output layout cannot be read from the code

```yaml
severity: low
confidence: high
location: "evals/src/compare.rs:469-480"
evidence: |
  let block = |text: &Option<String>| {
recommendation: "Write each line explicitly with writeln!."
history:
  - status: open
    by: newcomer
  - status: deferred
    by: developer
    to: "the next change to codev-evals"
    note: "Readability only; left out to keep this diff small."
```
