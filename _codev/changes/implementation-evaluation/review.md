# Review record: implementation-evaluation

<!-- codev-review: 1 -->

## plan.challenge

Mode: isolated.

### skeptic

#### R-skeptic-1 Replayed merged plans hold tasks that cannot be verified in the offline sandbox

```yaml
severity: medium
confidence: high
location: "design.md#Decision: An implementation case replays an archived change, through the update cases' fixture"
evidence: |
  The plan replayed is the plan as merged. It may hold tasks added during
  the review, so it is the plan the change actually delivered. A run is
  asked to implement that plan, not to rediscover it.
recommendation: Reword in the case's `change/` every task verification the sandbox cannot run (such as `mdbook` for forward-compatible-config's books), say so in the case's summary, and name it in tasks 5.1-5.3.
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    ref: "design.md, the spec's case requirement (MAY differ only by reworded verifications), tasks 5.1 and 2.6 (no task left unchecked for a blocker)"
  - status: fixed
    by: skeptic
```

#### R-skeptic-2 The acceptance script and the merged solution are hidden from the fixture's tree, not from the agent

```yaml
severity: medium
confidence: low
location: "design.md#Decision: Quality is judged by a hidden black-box script against the built binary"
evidence: |
  the script: it lives in `evals/cases/`, which the fixture does not hold.
  That directory is absent at `ref`, and the loader refuses a case whose
recommendation: State what the loader guarantees (only `evals/cases/<case id>/` is absent), and either deny reads of the operator's repository or detect them; have the smoke run check the transcript reads nothing outside the case's directory.
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    ref: "design.md (the sentence corrected; reads outside the case's directory recorded and checked), spec (a scoring check and the scenario 'A run that reads the merged solution'), tasks 2.4, 2.6 and 4.1"
  - status: fixed
    by: skeptic
```

#### R-skeptic-3 Four cases at five repetitions may not give the resolution the stated goal needs

```yaml
severity: medium
confidence: low
location: "design.md#Goals / Non-Goals"
evidence: |
  - A measure of `/codev-apply` precise enough to compare two models, or
    two versions of the skill, with `compare`.
recommendation: State the smallest regression compare detects, keep the refusal out of the pass and acceptance shares, and add a test naming the detectable regression.
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    ref: "design.md#Risks / Trade-offs (the resolution stated), spec (refusals kept out of the shares, judged on their own; scenario 'One more failed implementation in most repetitions'), tasks 4.1 and 4.3"
  - status: fixed
    by: skeptic
```

### security

#### R-security-1 Post-run checks and the acceptance script run agent-written code outside the sandbox without dropping the API key

```yaml
severity: medium
confidence: low
location: "design.md#Decision: The runner runs the checks itself, after the run"
evidence: |
  The agent's own report of a passing test is not a measure. After the
recommendation: Start every runner command on the fixture without the API key, assert it in a test, and warn in CONTRIBUTING about the other secrets of the operator's shell.
history:
  - status: open
    by: security
  - status: fixed
    by: architect
    ref: "design.md#Decision: The runner's own commands run without secrets and under a timeout (a cleared environment, stricter than tool_command), spec (scenario 'The checks never see the API key'), tasks 2.2 and 6.1"
  - status: fixed
    by: security
```

### ops

#### R-ops-1 The runner's post-run checks on agent-written code have no timeout

```yaml
severity: medium
confidence: high
location: "design.md#Decision: The runner runs the checks itself, after the run"
evidence: |
  same Cargo home. It records each outcome with the last 40 lines of its
  output, then runs `acceptance.sh` against `target/debug/codev`. A check
recommendation: Put a timeout on every runner command (preparation, checks, calibrate), in its own process group; a check that times out fails and the run is still scored; add a test.
history:
  - status: open
    by: ops
  - status: fixed
    by: architect
    ref: "design.md#Decision: The runner's own commands run without secrets and under a timeout, spec (1,200 seconds each, scenario 'A test that never ends'), task 2.2 (a_command_that_times_out_fails_its_check)"
  - status: fixed
    by: ops
```

#### R-ops-2 Build outputs are removed only after an observation is written

```yaml
severity: low
confidence: low
location: "design.md#Decision: An implementation run's build outputs are removed once recorded"
evidence: |
  `target/` and `.cargo-home/` take about a gigabyte per run. Once the
recommendation: Remove `target/` and `.cargo-home/` also when the run ends in an error.
history:
  - status: open
    by: ops
  - status: fixed
    by: architect
    ref: "design.md (removed whether or not the run produced an observation), task 2.5"
  - status: fixed
    by: ops
```
