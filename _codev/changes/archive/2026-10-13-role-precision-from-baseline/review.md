# Review record: role-precision-from-baseline

<!-- codev-review: 1 -->

## plan.challenge

### skeptic

#### R-skeptic-1 Re-scoring cannot separate the fixture fixes of the controls from the roles' effect

```yaml
severity: medium
confidence: high
location: "design.md#Decision: Fix only what the cases misstate"
evidence: |
  committed baseline is re-scored against them separately (column
  "re-scored" below) so that their effect and the roles' stay apart.
recommendation: >-
  Re-scoring only reflects the two added alternatives. The three
  controls whose fixtures change (skeptic, ops, accessibility) are seen
  anew by the model in the new run, and the old observations still
  describe the old code: the six (d) findings they held disappear in
  the re-scored column only if the scorer drops them, which it does not.
  As written, the drop in false positives for accessibility, ops and
  skeptic would mix the fixture fixes with the role changes. Add an
  "adjusted" reading — the re-scored count minus the (d) findings the
  fixture fixes remove — and compare the new run with that.
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    ref: design.md "Fix only what the cases misstate" and "Baseline before and after" add the adjusted reading (re-scored minus the six (d) findings of the three changed controls), against which the new run is compared
  - status: fixed
    by: skeptic
    note: verified; the adjusted column isolates the roles' effect on the changed controls
```

#### R-skeptic-2 The proposal counts four controls where three change

```yaml
severity: low
confidence: high
location: "proposal.md#What Changes"
evidence: |
  state are fixed: two controls that were not clean, a third and a
  fourth whose components missed criteria their role checks, and two
recommendation: >-
  The design's table fixes three controls (`skeptic-control-small-flag`,
  `ops-control-graceful-shutdown`, `accessibility-control-custom-listbox`)
  and two seeded cases. Say "three controls that were not clean — one
  whose checks passed without the flag, two whose components missed
  criteria their role or pattern requires — and two seeded cases".
history:
  - status: open
    by: skeptic
  - status: fixed
    by: product
    ref: proposal.md What Changes counts three controls and two seeded cases
  - status: fixed
    by: skeptic
    note: verified against the design's table
```

## code.review

### newcomer

#### R-newcomer-1 The Reporting check passes on any "name" or "state" without saying why

```yaml
severity: low
confidence: high
location: "crates/codev-core/src/roles.rs:1267"
evidence: |
  reporting.contains("name") || reporting.contains("state"),
recommendation: >-
  A reader cannot tell what this assertion protects: "name" also matches
  "names" anywhere, "state" any "stated". Check the phrase each expert
  actually uses for what a finding must show, listed per role, so that
  dropping that sentence from a role fails the test with its name.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: crates/codev-core/src/roles.rs, `every_expert_says_what_it_does_not_report` lists per expert the Reporting sentence it checks, with a comment saying why
  - status: fixed
    by: newcomer
    note: verified
```
