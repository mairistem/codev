# Review record: evaluate-planning

<!-- codev-review: 1 -->

## plan.challenge

Mode: isolated.

### skeptic

#### R-skeptic-1 The suite cannot reach 20 historical cases with a primary requirement

```yaml
severity: medium
confidence: high
location: "specs/planning-evaluation/spec.md#Requirement: The suite covers planning with past changes and with decisions"
evidence: |
  The repository's suite SHALL hold at least 20 planning cases that replay
recommendation: "Set the threshold to the 17 the study's labels support, or name the archived changes that replace the three without a primary requirement."
history:
  - status: open
    by: skeptic
  - status: fixed
    by: qa
    ref: "spec: at least 17 historical cases; proposal, design and task 2.1: 17 historical, 5 on decisions, 3 on requirements, 25 in all"
  - status: fixed
    by: skeptic
```

#### R-skeptic-2 One of the four "decision" cases expects no decision

```yaml
severity: medium
confidence: high
location: "design.md#Context"
evidence: |
  collateral (modified in passing); plus 4 cases written on purpose against
recommendation: "Use another decision case of the study (E4 on 0006, E5 on 0003) and count E6 as a requirement case."
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    ref: "design: the decision cases are E1 to E5 (0007 to supersede, 0001, 0005, 0006, 0003); E6 to E8 are requirement cases"
  - status: fixed
    by: skeptic
```

#### R-skeptic-3 The regular CI's shallow checkout breaks `cargo test` once planning cases check their `ref`

```yaml
severity: medium
confidence: high
location: "tasks.md#5. Contribution rule and workflow"
evidence: |
  - [ ] 5.2 Check out the full history in `.github/workflows/evals.yml` (`fetch-depth: 0`); verified by the new suite test `the_workflow_checks_out_the_full_history`
recommendation: "Give the CI jobs that load cases the full history (`fetch-depth: 0`), or move the history checks out of the default load; add a task and a check."
history:
  - status: open
    by: skeptic
  - status: fixed
    by: lead
    ref: "task 5.2: `fetch-depth: 0` in the evaluation workflow and in the CI jobs that run `cargo test`, test `the_workflows_check_out_the_full_history`; design states both"
  - status: fixed
    by: skeptic
```

#### R-skeptic-4 The main role-evaluation requirements on cases and exit codes are left unmodified and contradict planning cases

```yaml
severity: medium
confidence: high
location: "specs/planning-evaluation/spec.md#Requirement: A planning case pins a past state of the repository, a request and what the real change did"
evidence: |
  A planning case SHALL be a directory of `evals/cases/` holding only a
recommendation: "Add to the delta, as MODIFIED, the requirements on the case format and on the report's exit code, so they cover `plan.write` and its failed runs."
history:
  - status: open
    by: skeptic
  - status: fixed
    by: qa
    ref: "role-evaluation delta: MODIFIED \"A case declares its fixture, its step, its role and what it expects\" (planning cases declared by planning-evaluation, scenario \"Review and planning cases side by side\") and \"The report is per role…\" (a `no-plan` run counts as in error, scenario \"A planning run that planned nothing\")"
  - status: fixed
    by: skeptic
```

#### R-skeptic-5 The extended rule requires every skill change to be cheaper, so a feature that adds work to a skill can never pass

```yaml
severity: medium
confidence: low
location: "proposal.md#What Changes"
evidence: |
    scores: a pull request that changes what a skill does — the workflows
recommendation: "Keep the full verdict for pull requests meant to reduce consumption; give other skill changes the `--quality-only` verdict, their consumption still reported."
history:
  - status: open
    by: skeptic
  - status: fixed
    by: product
    ref: "proposal and contribution requirement: the full verdict for pull requests meant to reduce consumption; `--quality-only` for other skill changes, their consumption still reported"
  - status: fixed
    by: skeptic
```

#### R-skeptic-6 The restored scenario loses a backquote

```yaml
severity: low
confidence: high
location: "specs/role-evaluation/spec.md#Requirement: A reliability chapter is generated from the committed baseline"
evidence: |
  - **GIVEN** a baseline directory whose `report.json records the date
recommendation: "Copy the archived text exactly: whose `report.json` records the date."
history:
  - status: open
    by: skeptic
  - status: fixed
    by: qa
    ref: "the restored scenario reads whose `report.json` records the date"
  - status: fixed
    by: skeptic
```

### ops

#### R-ops-1 The CI job's 360-minute limit no longer holds the three repetitions once planning cases run at 1,800 s each

```yaml
severity: medium
confidence: low
location: "design.md#Risks / Trade-offs"
evidence: |
    each] → Planning cases run under their own default budget and timeout;
recommendation: "Keep planning cases out of the workflow's default selection or bound `repeat` when they are selected, and update the sizing comment and the run step's check."
history:
  - status: open
    by: ops
  - status: fixed
    by: architect
    ref: "design: `--step` and a workflow `step` input, `review` by default; a planning run bounded to one repetition in CI (up to 190 minutes); task 5.2"
  - status: fixed
    by: ops
```

#### R-ops-2 The manual workflow cannot select or exclude the planning cases, so a skill change cannot be measured in CI as the new rule asks

```yaml
severity: low
confidence: low
location: "tasks.md#5. Contribution rule and workflow"
evidence: |
  - [ ] 5.2 Check out the full history in `.github/workflows/evals.yml` (`fetch-depth: 0`); verified by the new suite test `the_workflow_checks_out_the_full_history`
recommendation: "Add a selection by step to `run` and a workflow input for it."
history:
  - status: open
    by: ops
  - status: fixed
    by: lead
    ref: "task 3.0: `run --step`; task 5.2: the workflow's `step` input; spec scenario \"Only the planning cases\""
  - status: fixed
    by: ops
```

#### R-ops-3 Adding the planning runs to the existing baseline keeps its run.json, so the planning runs are labelled with versions they may not have run under

```yaml
severity: medium
confidence: low
location: "design.md#Decision: The baseline gains the planning runs by resuming it"
evidence: |
  only the 120 planning runs are made, under the same `run.json` (model,
recommendation: "Refuse to resume a directory recorded with another Claude Code version, naming both, or require the pinned version before recording."
raised_by: [ops, skeptic]
history:
  - status: open
    by: ops
  - status: fixed
    by: architect
    ref: "resuming refuses another Claude Code version (role-evaluation delta, scenario \"Resuming with another Claude Code\"; task 3.0); design and task 6.1 record the planning runs with the baseline's version"
  - status: fixed
    by: ops
```

### accessibility

No finding.

### privacy

#### R-privacy-1 The raw event stream keeps the operator's identifying paths with no retention or purge rule

```yaml
severity: low
confidence: high
location: "design.md#Decision: The runner reads Claude Code's event stream for every case"
evidence: |
  event as `json` does, plus every message: the runner keeps the raw stream
  (`claude.jsonl`, ignored by git like `claude.json`), reads the result as
  before, counts the tool calls, and finds the orientation — the calls
recommendation: "The stream keeps every message; its `system/init` event holds the operator's home directory with the username (plugin paths, `memory_paths`), a session id and the full transcript — more than counting calls and attributing cache writes needs (GDPR Art. 5(1)(c), (e)). It is git-ignored, but no retention rule exists: under `evals/baseline/` it stays on whoever records, and the CI artifact uploads it under the default retention. State in the design what the stream holds and how long it is kept; drop the `system/init` event, or the stream once `observed.json` is written; set an explicit `retention-days` on the evaluation workflow's artifact; write one sentence under the ticked personal-data domain naming this data and its handling."
history:
  - status: open
    by: privacy
  - status: fixed
    by: architect
    ref: "evals/src/runner.rs without_init_event, test the_stream_keeps_no_init_event; .github/workflows/evals.yml retention-days: 14; design.md and the personal-data line say what the stream holds and how long it is kept; task 3.4"
  - status: fixed
    by: privacy
```

## code.review

Mode: isolated.
Summarized: .github/workflows/evals.yml, CONTRIBUTING.md, CONTRIBUTING.fr.md, crates/codev-cli/src/docs.rs, docs/en/src/reliability.md, docs/fr/src/reliability.md — over the material limit, one summarizing sub-agent per file.
Summarized: evals/src/case.rs, checks.rs, compare.rs, main.rs, observed.rs, plan.rs, reliability.rs, report.rs, runner.rs, score.rs, suite.rs, evals/tests/cli.rs, fixture.rs, suite.rs — over the material limit, one summarizing sub-agent per file.
Summarized: evals/baseline/** — 479 files of generated observations, summarized as one set by one sub-agent rather than one per file; their checks are the suite tests (five runs per case, none in error, the report agreeing with the scoring).

### privacy

#### R-privacy-2 The baseline commits the recorder's per-user macOS temp path in every planning observation

```yaml
severity: low
confidence: low
location: "evals/src/runner.rs:1189-1191"
evidence: |
              match codev_json(tools, work_dir, &["validate", &change, "--json"]) {
                  Ok(value) => {
                      observed.validate.insert(change, value);
recommendation: "Remove the `root` field of the `codev validate --json` report before recording it — the scoring never reads it — and remove it from the 124 committed `evals/baseline/plan-*/*/observed.json`. A path like `/private/var/folders/_q/…/T/…` belongs to one account on one machine; recording it in a public repository is more than the measurement needs (GDPR Art. 5(1)(c), Art. 25). Confidence low: the path names nobody unless linked to a contributor."
history:
  - status: open
    by: privacy
  - status: fixed
    by: developer
    ref: "evals/src/runner.rs without_root, test a_validate_report_keeps_no_root; root removed from the 124 recorded evals/baseline/plan-*/*/observed.json, git grep /var/folders finds nothing"
  - status: fixed
    by: privacy
```

### accessibility

#### R-accessibility-1 Long source names are clipped at the left edge of the "Where the tokens go" chart

```yaml
severity: low
confidence: low
location: "evals/src/reliability.rs:1116-1123"
evidence: |
          marks.push_str(&text(
              "rel-label",
              INK,
              LABEL_END,
              cy + 4.0,
              "end",
              &s.source,
          ));
recommendation: "The sources chart right-aligns each label at LABEL_END (92 units, 12-unit font), sized for role names; \"codev instructions\" needs about 96 to 105 units and starts left of x=0, where an inline SVG clips it. Give this chart a wider label column (for example x0 = 140 and the labels ending at 132, with a narrower plot so that 100 % still fits), then check the page in both languages."
history:
  - status: open
    by: accessibility
  - status: fixed
    by: developer
    ref: "evals/src/reliability.rs: sources_chart's label column ends at 142 and the plot starts at 150; the_page_ranks_the_sources_by_cost checks it"
  - status: fixed
    by: accessibility
```

### ops

#### R-ops-4 Scoring planning runs looks up history in a repository guessed from the output directory, and silently scores against empty history when the guess is wrong

```yaml
severity: medium
confidence: high
location: "evals/src/suite.rs:293-331"
evidence: |
      let history = GitHistory {
          repo: &repo_of(dir),
      };
recommendation: "Pass the repository the cases were loaded and checked against (`--repo`, known in main.rs) into `score_dir_only` instead of guessing it from `dir`; if a lookup at a case's `ref` fails while scoring (no spec text, no decision files), fail with an error naming the case, the ref and the repository rather than handing an empty `AtRef` to `plan::score_plan`. Otherwise a `score` or `compare` of a moved directory skews the planning measures silently."
history:
  - status: open
    by: ops
  - status: fixed
    by: developer
    ref: "evals/src/suite.rs: score_dir and score_dir_only take the repository the cases come from; at_ref fails naming the case, the ref and the repository; test scoring_a_planning_case_without_its_history_fails"
  - status: fixed
    by: ops
```

### newcomer

#### R-newcomer-1 `read_calls` hides its cost-sharing rule in a 107-line, five-level loop

```yaml
severity: medium
confidence: high
location: "evals/src/runner.rs:445-551"
evidence: |
                      let total: usize = entered.iter().map(|(_, l)| *l).sum();
                      if calls.is_empty() || total == 0 {
                          sources.insert(
                              if calls.is_empty() { "prompt" } else { "output" }.to_string(),
                              writes,
                          );
recommendation: "Extract `fn share_writes(writes, entered, first) -> BTreeMap<String, u64>` with explicit branches (first → prompt, nothing entered → output, else proportional) and one comment on why an empty interval goes to output; extract the handling of an assistant event's content blocks and of a user event's tool results, so that the loop reads as: start a call if new, record usage, read blocks."
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "evals/src/runner.rs: share_writes with explicit branches and its reason, a CallReader with read_assistant, start_call, read_block, read_tool_results; test a_call_with_nothing_new_charges_its_writes_to_output"
  - status: fixed
    by: newcomer
```

#### R-newcomer-2 The planning branch of `run_case` nests five levels and repeats the review block

```yaml
severity: low
confidence: high
location: "evals/src/runner.rs:1132-1156"
evidence: |
                  Err(e) => observed.error = Some(format!("cannot run codev validate: {e}")),
recommendation: "Extract `fn codev_json(tools, dir, args) -> Result<serde_json::Value, String>`, holding once the comment on why exit code 1 is tolerated, and use it for `codev validate --json` and `codev review --json`."
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "evals/src/runner.rs: codev_json, used for codev validate and codev review"
  - status: fixed
    by: newcomer
```

#### R-newcomer-3 `plan_case` copies the whole `CaseScore` literal and builds its empty maps two ways

```yaml
severity: low
confidence: high
location: "evals/src/score.rs:174-209"
evidence: |
      let empty = BTreeMap::new();
      let (files, validate) = match &observed.error {
          Some(_) => (&empty, &BTreeMap::new()),
recommendation: "Extract a shared `fn unscored(case, observed) -> CaseScore` used by `case()` and `plan_case()`, and name both empty inputs the same way."
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "evals/src/score.rs: unscored shared by case and plan_case; no_files and no_validate"
  - status: fixed
    by: newcomer
```

#### R-newcomer-4 `sum_of` selects its accumulator with a boolean-to-index trick

```yaml
severity: low
confidence: high
location: "evals/src/score.rs:850-866"
evidence: |
                              let w = &mut weighted[usize::from(m == Measure::OutputPerCall)];
recommendation: "Replace the anonymous `[(f64, f64); 2]` indexed by a bool with two named accumulators, chosen with a plain `match`."
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "evals/src/score.rs: sum_of keeps two named accumulators, input and output, chosen by a match"
  - status: fixed
    by: newcomer
```

#### R-newcomer-5 The doc comment of `check_resume` still describes only the model check

```yaml
severity: low
confidence: high
location: "evals/src/runner.rs:1283-1310"
evidence: |
  /// Refuses to resume a run recorded with another model: its observations
  /// would be scored under the first model's name.
recommendation: "Extend the comment: or with another Claude Code version, whose runs would be credited to the wrong version."
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "evals/src/runner.rs: check_resume's comment names the Claude Code version"
  - status: fixed
    by: newcomer
```

#### R-newcomer-6 `word_runs` documents a fixed length that is really a parameter, and callers pass a bare 13

```yaml
severity: low
confidence: high
location: "evals/tests/suite.rs:390-414"
evidence: |
  /// The runs of more than twelve words of `text`, lowercased.
  fn word_runs(text: &str, n: usize) -> BTreeSet<String> {
recommendation: "Document it as every run of `n` consecutive words, lowercased, and name the threshold with its reason (`const COPIED_RUN: usize = 13`), used by `no_request_copies_its_proposal`."
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "evals/tests/suite.rs: word_runs documented as runs of n words; COPIED_RUN = 13 with its reason"
  - status: fixed
    by: newcomer
```

#### R-newcomer-7 An unexplained `#[allow(dead_code)]` on `PlanCaseFile::step`

```yaml
severity: low
confidence: high
location: "evals/src/case.rs:552-555"
evidence: |
      #[allow(dead_code)]
      step: String,
recommendation: "Add a one-line comment: read by `load` to dispatch here, declared so that `deny_unknown_fields` accepts it."
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "evals/src/case.rs: a comment on PlanCaseFile::step"
  - status: fixed
    by: newcomer
```

