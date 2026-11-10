# Review record: validate-detects-merge-collisions

<!-- codev-review: 1 -->

## plan.challenge

Mode: isolated.

### skeptic

#### R-skeptic-1 A REMOVED already applied passes validate but archive still refuses it

```yaml
severity: high
confidence: high
location: "design.md#Decision: `ADDED` collisions compared on parsed content, language-blind"
evidence: |
  of the same delta produces it. `REMOVED` of an absent requirement is not
  reported: the outcome it asks for is already there, which is also the
  state after its own sync.
recommendation: >-
  The merge's `DeltaSection::Removed` arm (crates/codev-core/src/merge/mod.rs)
  returns `ModifiedTargetMissing` for an absent removal, with no idempotent
  skip, so validate stays silent while archive fails. Either make the merge
  skip an absent REMOVED or report it in validate, with a spec-merge scenario
  and an engine test.
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    ref: "specs/spec-merge/spec.md (Semantic merge per operation: REMOVED of an absent requirement skipped, new scenario); design.md; tasks.md 2.5"
  - status: fixed
    by: skeptic
```

#### R-skeptic-2 The pre-flight does not check the main specs its deltas target

```yaml
severity: medium
confidence: high
location: "design.md#Decision: One duplicate rule for delta sections and main specs"
evidence: |
  The merge itself does not change: `validate` runs before every sync and
  archive (pre-flight), and a main spec holding a duplicate fails
  `codev validate` as a whole.
recommendation: >-
  `read_main_spec` drops the parser's findings, so a change modifying `Login`
  in a spec holding it twice passes the pre-flight and the merge rewrites one
  copy only. Report the targeted specs' duplicate and marker findings in the
  change, with a spec-merge scenario and an engine test.
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    ref: "specs/validation/spec.md (targeted spec defects reported for the change, new scenario); specs/spec-merge/spec.md (new scenario); design.md; tasks.md 2.4"
  - status: fixed
    by: skeptic
```

#### R-skeptic-3 CI validates the merge result only if it reruns when the base moves

```yaml
severity: medium
confidence: high
location: "design.md#Context"
evidence: |
  GitHub workflows), and on a GitHub `pull_request` event the checkout is
  the merge of the branch into its base: whatever `validate` detects on a
  tree, CI detects on the merge result, before the merge is made.
recommendation: >-
  Two green pull requests merged back to back are not revalidated against
  each other unless the forge requires up-to-date branches or a merge queue.
  Say so in the guide requirement and task 5.1, or state that such a
  collision is caught after the merge.
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    ref: "specs/docs/spec.md (guide states the up-to-date or merge-queue condition); design.md (archive rule decision); tasks.md 5.1"
  - status: fixed
    by: skeptic
```

#### R-skeptic-4 A conflict marker in change.yaml is never reported as merge_conflict_marker

```yaml
severity: low
confidence: high
location: "tasks.md#3. Conflict markers"
evidence: |
  `corrections.md`, `change.yaml`) and on each local decision, as errors
recommendation: >-
  `change::load` parses change.yaml before any finding is collected. Give it
  the read-time handling of config.yaml and seal.yaml, or drop it from the
  scan.
history:
  - status: open
    by: skeptic
  - status: fixed
    by: lead
    ref: "specs/validation/spec.md (change.yaml read-time failure, new scenario); tasks.md 3.2, 3.3"
  - status: fixed
    by: skeptic
```

#### R-skeptic-5 Two seal entries for one id make the second decision report a seal mismatch

```yaml
severity: low
confidence: high
location: "specs/decisions/spec.md#Requirement: Two local decisions never share an id, nor two seals"
evidence: |
  remain the warning `decision_id_collision`. Each of the decisions sharing an
  id is still checked against its seal.
recommendation: >-
  `seal.find` returns the first entry; say which entry counts when an id is
  sealed twice, for example any matching entry, with a scenario.
history:
  - status: open
    by: skeptic
  - status: fixed
    by: qa
    ref: "specs/decisions/spec.md (any matching entry; new scenario); tasks.md 4.2"
  - status: fixed
    by: skeptic
```

#### R-skeptic-6 The fence exemption leaves conflicts inside fenced examples undetected

```yaml
severity: low
confidence: low
location: "design.md#Goals / Non-Goals"
evidence: |
  - Every collision that a text merge of two branches leaves in `_codev/`,
    and that the merged tree alone can reveal, is an error of
    `codev validate`, at its file and line, with what to do.
recommendation: >-
  Git writes markers at column 0 even inside a fence. Narrow the goal and
  state the limit, or report a complete opener/closer pair inside a fence.
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    ref: "specs/validation/spec.md (limit stated); design.md (alternative and risk); tasks.md 5.1 (guide)"
  - status: fixed
    by: skeptic
```

### migration

#### R-migration-1 Regenerating a seal.yaml with a duplicated id does not remove the duplicate

```yaml
severity: medium
confidence: high
location: "design.md#Decision: One documented rule — archive inside the pull request"
evidence: |
  with `codev decision seal`, the `CHANGELOG.md` managed section with
recommendation: >-
  `plan_seal_new` refuses an id already sealed and `plan_seal_force` rewrites
  every entry of the id, keeping the duplicate. Make `seal --force` collapse
  duplicates, with an engine test showing `decision_seal_duplicate` cleared.
history:
  - status: open
    by: migration
  - status: fixed
    by: architect
    ref: "specs/decisions/spec.md (MODIFIED `decision seal` adds or renews a seal entry); design.md; tasks.md 4.3"
  - status: fixed
    by: migration
```

#### R-migration-2 The failure code for an unparseable config, seal or change file changes for every command

```yaml
severity: low
confidence: high
location: "specs/validation/spec.md#Requirement: Git conflict markers left in planning files are reported"
evidence: |
  and holds such a marker, `codev validate` — like every command that reads
  that file — MUST fail with the code `merge_conflict_marker`, naming the
recommendation: >-
  Consumers testing `invalid`, `unreadable` or `seal_invalid` now get
  `merge_conflict_marker`. Say so in the BREAKING release note item, add the
  code to the error table of `reference/json-output.md` in both languages,
  and test it.
history:
  - status: open
    by: migration
  - status: fixed
    by: product
    ref: "proposal.md (What Changes, Release note); specs/docs/spec.md; tasks.md 5.2"
  - status: fixed
    by: migration
```
