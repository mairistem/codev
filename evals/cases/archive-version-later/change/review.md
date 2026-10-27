# Review record: role-models

<!-- codev-review: 1 -->

## plan.challenge

Mode: isolated.

### skeptic

#### R-skeptic-1 The change's own measure skips the isolated path, the only path it changes, and measures quality only although the change is meant to cut cost

```yaml
severity: high
confidence: high
location: "tasks.md#4. Measure (decision 0008)"
evidence: |
  - [ ] 4.1 Paste in the pull request the per-role comparison on Sonnet of the design, then run `codev-evals run --step review --repeat 3 --jobs 4` with this branch's binary and compare with `codev-evals compare evals/baseline <out> --quality-only`; verified by both pasted in the pull request
recommendation: "Cover the isolated review in the suite first, by its own change, then measure this change on that path with cost deciding; or propose a deviation from 0008 and drop the cost claim. (performance) The release note promises about half the cost for the isolated mode, but the only figure comes from single-session runs: word it as measured, or measure the isolated path with per-call token tracing."
raised_by: [skeptic, performance]
history:
  - status: open
    by: skeptic
  - status: fixed
    by: lead
    note: "A prior change, isolated-review-evaluation, covers the isolated mode in the suite (task 4.0); task 4.1 compares an isolated reference with every role on the session's model and an isolated candidate with the defaults, cost deciding; the Why and the release note no longer claim half the cost."
    ref: "tasks.md#4. Measure (decision 0008)"
  - status: fixed
    by: skeptic
```

#### R-skeptic-2 The design assumes the single-session Sonnet measure carries over to a sub-agent, but the suite runs a different setup

```yaml
severity: medium
confidence: high
location: "design.md#Decision: Measured by the per-role results above, and a run of the change"
evidence: |
  measures the role on a model through `CODEV_EVAL_MODEL`, not through the
recommendation: "Measure the isolated path, or state the assumption as unverified and restate the saving as a single-session one."
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    note: "The decision is rewritten: the single-session figures are named as such, and the change is measured on the isolated path."
    ref: "design.md#Decision: Measured on the isolated path, after the suite covers it"
  - status: fixed
    by: skeptic
```

#### R-skeptic-3 A model set on a role that never runs as a sub-agent is accepted silently

```yaml
severity: low
confidence: high
location: "specs/roles/spec.md#Requirement: A role may carry the model its sub-agent runs on"
evidence: |
  A role's mapping under `roles:` in `_codev/config.yaml` MAY set `model:`
recommendation: "Report a model on a role that serves no review step, or say it is ignored, with a scenario and a test."
history:
  - status: open
    by: skeptic
  - status: fixed
    by: qa
    note: "The warning role_model_unused, scenario \"A model on a role that is never a sub-agent\", tested in task 1.1."
    ref: "specs/roles/spec.md"
  - status: fixed
    by: skeptic
```

### performance

No finding.

### migration

#### R-migration-1 A shared source that sets `model:` silently drops its context and rules in consumer projects still on an older codev

```yaml
severity: high
confidence: high
location: "design.md#Risks / Trade-offs"
evidence: |
  - [An older codev reads a configuration with `model:`] → The `roles:`
recommendation: "Say in the release note and the configuration reference that a shared source must not set model: until its consumers run this release, and stop parsing an inherited source's roles: strictly, with an engine test."
history:
  - status: open
    by: migration
  - status: fixed
    by: architect
    note: "The release note and task 3.1 say so; an inherited source's roles: is skipped, not parsed; scenario \"A source's role setting from a newer codev\" and the engine test a_source_s_unknown_role_key_keeps_its_rules."
    ref: "design.md#Decision: A source's `roles:` is not parsed, and a model off the review steps is flagged"
  - status: fixed
    by: migration
```

#### R-migration-2 The default model change does not tell existing isolated projects how to keep their reviewers on the session's model

```yaml
severity: low
confidence: high
location: "proposal.md#Release note"
evidence: |
  ## Release note
recommendation: "Say in the Changed item and the roles chapter how to keep a role on the session's model."
history:
  - status: open
    by: migration
  - status: fixed
    by: product
    note: "The release note says to set model: inherit; the roles chapter will say the same (task 3.1)."
    ref: "proposal.md#Release note"
  - status: fixed
    by: migration
```

## code.review

Mode: isolated.

The reviewers read each file's diff and the functions it touches directly, without summaries.

### performance

No finding.

### migration

No finding.

### newcomer

#### R-newcomer-1 `parse_project_model` returns a double `Option` whose two "nones" mean opposite things

```yaml
severity: medium
confidence: high
location: "crates/codev-core/src/roles.rs:383-389"
evidence: |
  fn parse_project_model(text: &str) -> Option<Option<Model>> {
recommendation: "Return a named type such as ProjectModel { Inherit, Use(Model) }."
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    note: "A ProjectModel enum with parse and model(); the callers read it by name."
    ref: "crates/codev-core/src/roles.rs"
  - status: fixed
    by: newcomer
```

#### R-newcomer-2 The list of accepted models is written twice: once in a constant, once inline

```yaml
severity: low
confidence: high
location: "crates/codev-core/src/roles.rs:574-580"
evidence: |
  "{file}: role `{id}` has the model `{model}`; use `opus`, `sonnet`, `haiku` or `inherit`"
recommendation: "Build the message from constants."
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    note: "ProjectModel::ALLOWED feeds the message, next to Model::ALLOWED, with a comment that only a project may write inherit."
    ref: "crates/codev-core/src/roles.rs"
  - status: fixed
    by: newcomer
```

#### R-newcomer-3 The test name and comment rely on a measurement only the plan describes

```yaml
severity: low
confidence: high
location: "crates/codev-core/src/roles.rs:1276-1286"
evidence: |
  fn the_default_models_follow_the_measure() {
recommendation: "Rename the test and point the comment to the docs."
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    note: "Renamed no_built_in_role_carries_a_model; the comment points to the roles chapter."
    ref: "crates/codev-core/src/roles.rs"
  - status: fixed
    by: newcomer
```

#### R-newcomer-4 Unexplained needle `"73"` in the documentation check

```yaml
severity: low
confidence: high
location: "crates/codev-cli/src/docs.rs:536-546"
evidence: |
  "73",
recommendation: "Use a more specific needle, with a comment."
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    note: "A comment says it checks the measured cost of the isolated mode in both editions; the needle stays bare because the French edition writes a non-breaking space before %."
    ref: "crates/codev-cli/src/docs.rs"
  - status: fixed
    by: newcomer
```
