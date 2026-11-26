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
