# Review record: update-evaluation

<!-- codev-review: 1 -->

## plan.challenge

Mode: isolated.

### skeptic

#### R-skeptic-1 Recording three repetitions of only the update cases breaks the baseline tests and the documented re-record procedure

```yaml
severity: high
confidence: high
location: "design.md#Decision: The baseline gains the update runs, and #48 is judged on them"
evidence: |
  Following decision 0008, this change records the update runs of the
recommendation: "Record five repetitions with the pinned Claude Code and justify adding to the baseline without a full recording, or record it all again; regenerate and commit report.json and report.md."
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    note: "Five repetitions of the update cases alone, with the pinned Claude Code 2.1.289 and codev 0.6.1 (the baseline's), report.json and report.md regenerated and committed; CONTRIBUTING says a new step's cases may be recorded alone, changed cases or scoring still requiring a full recording."
    ref: "design.md#Decision: The baseline gains the update runs alone, five of each"
  - status: fixed
    by: skeptic
```

#### R-skeptic-2 Task 4.1 judges another change and cannot be completed within this one

```yaml
severity: medium
confidence: high
location: "tasks.md#4. Measure `capture-corrections` on the new step (decision 0008)"
evidence: |
  verified by the comparison pasted in #48, which can then be merged if it passes
recommendation: "Remove the task and the #48 half of the decision; keep a note that #48 will be measured on these cases."
history:
  - status: open
    by: skeptic
  - status: fixed
    by: lead
    note: "The task is removed; the proposal and the design note that #48 is measured on the update cases in its own pull request."
    ref: "tasks.md"
  - status: fixed
    by: skeptic
```

#### R-skeptic-3 The ripple check rests on a false claim about the skill's summary

```yaml
severity: medium
confidence: high
location: "design.md#Decision: The reply is the agent's last message"
evidence: |
  prompt asks the agent to end with what was revised and the ripple found,
recommendation: "Collect every message the agent wrote, or make the skill restate the ripple; fix the design's alternatives and risks."
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    note: "The check reads the text of every message the agent wrote, tool calls and outputs left out; the prompt no longer asks for the ripple in the reply."
    ref: "design.md#Decision: The ripple is looked for in every message the agent wrote"
  - status: fixed
    by: skeptic
```

#### R-skeptic-4 The validity check assumes the unrevised archived change validates with the codev under test, which nothing verifies

```yaml
severity: medium
confidence: low
location: "specs/update-evaluation/spec.md#Requirement: An update run is scored by deterministic checks"
evidence: |
  - `codev validate` reports no error on the change that it did not report
recommendation: "Validate the fixture before the run and count only the errors the run adds, with a scenario."
history:
  - status: open
    by: skeptic
  - status: fixed
    by: qa
    note: "The runner validates the fixture before the run; the check fails only on an error not there before; scenario \"An error already in the fixture\"."
    ref: "specs/update-evaluation/spec.md"
  - status: fixed
    by: skeptic
```

#### R-skeptic-5 The stray rule for `corrections.md` targets a file only the unmerged #48 would write, and has no scenario or test

```yaml
severity: low
confidence: high
location: "specs/update-evaluation/spec.md#Requirement: The runner replays the revision request with the skill under test"
evidence: |
  run. A file modified outside the change's folder SHALL be recorded as
recommendation: "Move the rule to #48, or keep it with a scenario and a test."
history:
  - status: open
    by: skeptic
  - status: fixed
    by: qa
    note: "The rule is dropped here; it belongs to the change that introduces the file."
    ref: "specs/update-evaluation/spec.md"
  - status: fixed
    by: skeptic
```
