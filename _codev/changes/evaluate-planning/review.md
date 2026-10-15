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
