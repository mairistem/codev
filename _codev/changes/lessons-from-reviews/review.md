# Review record: lessons-from-reviews

<!-- codev-review: 1 -->

## plan.challenge

Mode: isolated.

### skeptic

#### R-skeptic-1 The waiting threshold and the default window count different changes, so some changes are never read

```yaml
severity: medium
confidence: high
location: "design.md#Decision: `/codev-archive` adds one line, driven by an additive field"
evidence: |
  command, no new tool. Ten is the mode's default window: a pass then reads
  exactly what accumulated.
recommendation: Make the two rules agree. Count every archived change no pass has read, give the window a pass needs in the archive report, have the invitation line pass it to the mode, and add a scenario and a test for a gap holding changes without findings.
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    ref: "passes record the folders they read (`read` list); `lessons.changes` counts every unread change and `lessons.window` gives the window; the invitation passes it (design.md, specs/lessons/spec.md, specs/skills/spec.md, tasks.md 1.3, 4.1, 5.2)"
  - status: fixed
    by: skeptic
```

#### R-skeptic-2 Ordering by folder name puts same-day archives out of sequence, so the waiting signal can skip a change

```yaml
severity: medium
confidence: high
location: "specs/lessons/spec.md#Requirement: `codev archive` tells whether lessons may be waiting"
evidence: |
  Archived changes are ordered by folder name.
recommendation: Record what a pass read independently of sort order, for example the set of folders read, and add a scenario for two changes archived on the same day.
history:
  - status: open
    by: skeptic
  - status: fixed
    by: qa
    ref: "a pass lists the folders it read; unread means listed by no pass; scenario `Changes archived the same day, in any name order` and tests in tasks 1.3 and 4.1"
  - status: fixed
    by: skeptic
```

#### R-skeptic-3 The design departs from decision 0008 without saying so: the archive skill and its JSON get no evaluation

```yaml
severity: medium
confidence: high
location: "tasks.md#5. The skills"
evidence: |
  `archive_allowed_tools_are_unchanged`. No evaluation step replays
recommendation: Name the archive skill, and the creation of `configure.lessons` in the same change as the mode, as departures from decision 0008 for the author to accept; otherwise plan the covering step first, or propose superseding 0008 for additive one-line changes.
history:
  - status: open
    by: skeptic
  - status: fixed
    by: lead
    ref: "the author planned the covering steps first in `evaluate-archive-sync-configure` (`change.archive`, `configure.enrich`, with a baseline), now a declared prerequisite: proposal.md What Changes and Impact; design.md Context, the archive and `configure.lessons` decisions (the invitation line measured on `change.archive` with edited cases and `archive-lessons-not-waiting`, the default mode against the `configure.enrich` baseline, `configure.lessons` on its shared scoring and `overlay/`), the split decision, Risks, Open Questions without the three departures; specs/lessons-evaluation/spec.md; tasks.md 0.1, 5.2, 5.3, 6.1, 6.3, 6.6"
  - status: fixed
    by: skeptic
```

#### R-skeptic-4 The Migration Plan relies on an update check that the configure skill does not have

```yaml
severity: medium
confidence: high
location: "design.md#Migration Plan"
evidence: |
  ignores the ledger, and its `codev review` rejects `--archived` with a
recommendation: Have the lessons mode run the version-check fragment and recognise the usage error of an older binary, saying in one line that a newer codev is needed and pointing to `codev self-update`, then stop without reading the archive by other means; correct the Migration Plan; add a catalog test such as `configure_lessons_stops_on_an_older_binary`.
raised_by: [skeptic, migration]
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    ref: "design.md Migration Plan and lessons-mode procedure step 1; specs/configure/spec.md requirement and scenario `An older binary`; tasks.md 5.1 with `configure_lessons_stops_on_an_older_binary`"
  - status: fixed
    by: skeptic
```

#### R-skeptic-5 The Domains reason for leaving security unticked covers only the shell, but the plan also builds paths from project content

```yaml
severity: medium
confidence: low
location: "proposal.md#Domains"
evidence: |
  - [ ] security — the served history is the repository's own planning files, already read by the skills; no input reaches the shell, nothing executable is added, and the folders the ledger names are single path segments looked up only under `_codev/changes/archive/`, refused otherwise
recommendation: Tick security, or extend the reason to the path resolutions, stating that origin folders are single path segments; bound the evaluation overlay the same way.
history:
  - status: open
    by: skeptic
  - status: fixed
    by: product
    ref: "proposal.md Domains reason extended; specs/lessons/spec.md single-segment folders and scenario `An origin that leaves the archive`; specs/lessons-evaluation/spec.md overlay without symbolic link, scenario `An overlay that leaves the tree`"
  - status: fixed
    by: skeptic
```

#### R-skeptic-6 Personal-data is unticked although the catch-up copies the user's Claude Code memory into versioned files

```yaml
severity: low
confidence: low
location: "proposal.md#Domains"
evidence: |
  - [ ] personal-data — the ledger quotes findings and corrections, which already name no person; the one-time reading of this project's Claude Code memory happens only with the author's agreement, proposes only project instructions (never a personal preference, a person or a secret), and each write is confirmed
recommendation: Tick personal-data, or refer to the memory folder without the user's path and state what is never copied.
history:
  - status: open
    by: skeptic
  - status: fixed
    by: product
    ref: "proposal.md Domains reason states what is never copied; tasks.md 8.2 names the memory folder without the user's path"
  - status: fixed
    by: skeptic
```

#### R-skeptic-7 The design misreads decision 0004 as "no new skill"

```yaml
severity: low
confidence: high
location: "design.md#Decision: An on-demand mode of `/codev-configure`, with a fixed order of destinations"
evidence: |
  The mode is `/codev-configure lessons [<n>]`: a mode of the existing
recommendation: Base the choice on its real reason, that two skills would write `rules:`, and stop citing 0004 as forbidding a new skill.
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    ref: "design.md Context, the mode's decision and its alternatives no longer cite 0004 as forbidding a skill"
  - status: fixed
    by: skeptic
```

#### R-skeptic-8 The design's count of archived changes is wrong

```yaml
severity: low
confidence: high
location: "design.md#Context"
evidence: |
  - The archive holds 63 changes on 2026-10-26; 21 have a `review.md`
recommendation: Correct the figure to 63.
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    ref: "design.md Context: 63 changes"
  - status: fixed
    by: skeptic
```

### performance

#### R-performance-1 The archive's waiting signal re-reads every archived record since the last pass, with no upper limit

```yaml
severity: low
confidence: high
location: "specs/lessons/spec.md#Requirement: `codev archive` tells whether lessons may be waiting"
evidence: |
  Only the corrections files of the changes counted are read; no review
  record is read for `lessons`.
recommendation: Bound the scan or document its cost, and time `codev archive --json` on this repository's archive.
history:
  - status: open
    by: performance
  - status: fixed
    by: qa
    ref: "the signal reads only unread changes' corrections.md, never a review record (specs/lessons/spec.md); cost stated in design.md; task 4.1 times it"
  - status: fixed
    by: performance
```

### migration

No finding.
