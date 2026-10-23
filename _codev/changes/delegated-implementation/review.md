# Review record: delegated-implementation

<!-- codev-review: 1 -->

## plan.challenge

Mode: isolated.

### skeptic

#### R-skeptic-1 The plan contradicts decision 0011 but says 0011 stays as written

```yaml
severity: medium
confidence: high
location: "design.md#Decision: The decision is recorded as an ADR, without changing 0011"
evidence: |
  recommends `developer: sonnet` to projects. It does not ship that
  default, so decision 0011 stays as written.
recommendation: Have the new ADR supersede 0011's scope sentence (the model applies only in the isolated review mode) and say so in the design; fix the proposal, which labels the new decision "(decision 0011)".
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    ref: "design.md#Decision: A new decision supersedes 0011; proposal.md#What Changes; task 5.2 supersedes 0011"
  - status: fixed
    by: skeptic
```

#### R-skeptic-2 The decision cites a saving measured on a different setup, and is accepted before this setup is measured

```yaml
severity: medium
confidence: high
location: "tasks.md#5. Measure and record"
evidence: |
  - [ ] 5.1 The decision: `codev decision new` then `accept`, "Implementation may run on another model than planning, chosen per project through the developer role", with the measure of the implementation evaluation (Opus 13.02 USD, Sonnet session 5.68 USD, every measure at 1.00) and the recommendation `developer: sonnet`, no built-in default (0011 unchanged); verified by `codev validate --strict`
recommendation: "Measure the delegated setup first, record its cost in the ADR, and state the cost result that justifies recommending `developer: sonnet`."
history:
  - status: open
    by: skeptic
  - status: fixed
    by: lead
    ref: "tasks.md 5.1 (measure first, the bar is the full compare verdict: quality within range and cost under the floor) then 5.2 (the decision records that measure)"
  - status: fixed
    by: skeptic
```

#### R-skeptic-3 Nothing says who implements the remaining tasks after the sub-agent stops

```yaml
severity: medium
confidence: high
location: "specs/skills/spec.md#Requirement: The `apply` skill guides the implementation of a change"
evidence: |
  blocker. The session then relays a stop to the user as the skill says,
recommendation: Specify the resume path (a new sub-agent on the developer's model with the answer), pass a run without a user's mode into the brief, and add a scenario with its test.
history:
  - status: open
    by: skeptic
  - status: fixed
    by: qa
    ref: "specs/skills: a stop is answered, then a new sub-agent on the developer model takes the remaining tasks; the brief carries the session instructions; scenario The sub-agent stops on an ambiguity; task 3.1 test"
  - status: fixed
    by: skeptic
```

#### R-skeptic-4 `role_model_unused` would stay silent for code.implement roles whose model never applies

```yaml
severity: medium
confidence: high
location: "design.md#Decision: `role_model_unused` spares the roles served at `code.implement`"
evidence: |
  **Alternatives considered**: exempting the `developer` by name. A
  project role declared at `code.implement` would then be warned wrongly.
recommendation: Exempt only the role the skill dispatches, the developer, and add a scenario for a project role at `code.implement` that carries a model.
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    ref: "design.md#Decision: role_model_unused spares the developer only; specs/roles scenario A model on another role at code.implement; task 1.1"
  - status: fixed
    by: skeptic
```

#### R-skeptic-5 The retry and fallback path has no scenario and does not handle partly written work

```yaml
severity: low
confidence: high
location: "specs/skills/spec.md#Requirement: The `apply` skill guides the implementation of a change"
evidence: |
  platform cannot start, or that fails, is retried once; after a second
recommendation: Define "fails" apart from a reported stop, have the next attempt inspect the working tree for the first unchecked task, and add a "The sub-agent cannot be started" scenario checked by the 3.1 test.
history:
  - status: open
    by: skeptic
  - status: fixed
    by: qa
    ref: "specs/skills: a failure is defined apart from a stop, the retry inspects the working tree; scenario The sub-agent cannot be started; task 3.1 test"
  - status: fixed
    by: skeptic
```
