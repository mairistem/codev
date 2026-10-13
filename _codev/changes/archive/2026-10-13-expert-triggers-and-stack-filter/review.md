# Review record: expert-triggers-and-stack-filter

<!-- codev-review: 1 -->

## plan.challenge

### skeptic

#### R-skeptic-1 The product owner's reaction to suggestions is mostly empty at planning time

```yaml
severity: medium
confidence: high
location: "design.md#Every case, planned"
evidence: |
  at `/codev-propose` time the diff usually holds only the planning files, which are `planning` noise
recommendation: >-
  The proposal presents the product owner's reaction as the guard, while
  the design shows the diff is empty before the code exists. Say plainly,
  in the proposal and the design's goals, that the review step and
  `codev validate` after the code is written are where a missed domain is
  caught, so nobody relies on the planning-time check.
history:
  - status: open
    by: skeptic
  - status: fixed
    by: product
    ref: proposal.md What Changes, last bullet, says the review is where a missed domain is caught; design.md Goals gains "Where the check bites"
  - status: fixed
    by: skeptic
    note: verified in both artifacts
```

#### R-skeptic-2 An expert forced by `change.yaml` still gets its domain suggested

```yaml
severity: low
confidence: high
location: "specs/domain-suggestions/spec.md#Requirement: Each suggested domain has a state"
evidence: |
  - `unticked` otherwise — including when the proposal does not exist yet,
recommendation: >-
  A change with `roles: { security: always }` already brings the expert;
  count its domain as answered instead of warning.
history:
  - status: open
    by: skeptic
  - status: disputed
    by: qa
    note: >-
      `## Domains` is the change's record of what it touches, read by
      reviewers and by the release; `always` in `change.yaml` activates the
      expert without documenting the domain. Ticking costs one line and
      keeps one rule: a suggestion is answered in the proposal.
  - status: rejected
    by: skeptic
    note: agreed, the proposal stays the single place a suggestion is answered
```

### security

#### R-security-1 Running git in an unpacked repository runs its configured programs

```yaml
severity: low
confidence: high
location: "design.md#Decision: The diff in `codev-engine::diff`, through `ProcessRunner`"
evidence: |
  All commands run in the project root, with
recommendation: >-
  Every `codev roles --change` and `codev validate` now runs `git diff`. A
  repository unpacked from an archive brings its own `.git/config`, whose
  `core.fsmonitor` or a `textconv` driver is a program git runs. Pass
  `-c core.fsmonitor=false` and `--no-textconv` (with `--no-ext-diff`)
  so codev never runs one on the user's behalf (CWE-427, uncontrolled
  search path element).
history:
  - status: open
    by: security
  - status: fixed
    by: architect
    ref: design.md, the diff decision passes `-c core.fsmonitor=false`, `--no-textconv` and `--no-ext-diff`, and says why
  - status: fixed
    by: security
    note: verified; hooks do not run on `diff`, `ls-files` or `rev-parse`
```

### performance

#### R-performance-1 Every added line is matched once per pattern

```yaml
severity: low
confidence: high
location: "design.md#Decision: The core classifies, caps and matches"
evidence: |
  `codev-core::triggers` holds the parsing of groups, the noise list
recommendation: >-
  With six experts and project groups, a line meets some thirty patterns;
  match all of them at once first (one multi-pattern expression) and name
  the triggers only for the rare line that matches.
history:
  - status: open
    by: performance
  - status: fixed
    by: architect
    ref: design.md, the core decision matches each line against one multi-pattern expression first
  - status: fixed
    by: performance
    note: verified
```

### migration

#### R-migration-1 The new key breaks an older codev's strict validation without a word

```yaml
severity: medium
confidence: high
location: "proposal.md#Release note"
evidence: |
  - Added: A `triggers:` key in `_codev/config.yaml` adds the project's own
recommendation: >-
  An earlier codev reads `triggers:` as an unknown key: a warning, which
  fails its `codev validate --strict`. Tell teams to raise
  `codev_version` when they add the key.
history:
  - status: open
    by: migration
  - status: fixed
    by: product
    ref: proposal.md release note, the `triggers:` item says an earlier codev warns and to raise `codev_version`
  - status: fixed
    by: migration
    note: verified; the configuration chapter must say it too (task 6.1)
```

## code.review

### security

No finding.

### performance

#### R-performance-2 Every untracked file is stat'ed, noise included

```yaml
severity: low
confidence: high
location: "crates/codev-engine/src/diff.rs:100-102"
evidence: |
  for path in untracked.split('\0').filter(|p| !p.is_empty()) {
      let full = root.join(path);
      let regular = fs.is_regular_file(&full);
recommendation: >-
  A repository that does not ignore its `node_modules` lists tens of
  thousands of untracked files, each asked for its type and size although
  the core leaves `vendored`, `lockfile`, `generated` and `planning` paths
  out of every trigger. Classify the path first and skip the metadata calls
  for those.
history:
  - status: open
    by: performance
  - status: fixed
    by: developer
    ref: crates/codev-engine/src/diff.rs, an untracked path the core excludes from every trigger gets no `is_regular_file` or `len` call; test `untracked_noise_is_neither_stated_nor_read`
  - status: fixed
    by: performance
    note: verified
```

### migration

#### R-migration-2 This repository now uses a key its pinned codev does not know

```yaml
severity: medium
confidence: high
location: "_codev/config.yaml:14"
evidence: |
  codev_version: ">=0.6"
recommendation: >-
  `triggers:` is added to this repository's configuration while
  `codev_version` still accepts 0.6.x, which warns `unknown_config_key` and
  fails its `codev validate --strict`. Raise `codev_version` to the release
  that ships `triggers:`.
history:
  - status: open
    by: migration
  - status: deferred
    by: developer
    to: the release that ships this change (raise `codev_version` in `_codev/config.yaml` to it); the dev binary is 0.6.1, so raising it now fails `codev doctor --strict`, as R-migration-1 of review-record
```

### newcomer

#### R-newcomer-1 A git failure is told apart by comparing its message

```yaml
severity: low
confidence: high
location: "crates/codev-engine/src/diff.rs:81"
evidence: |
  Err(reason) if reason == GIT_MISSING => return Err(reason),
recommendation: >-
  `Git::run` returns its reason as a `String`, and the caller recognizes
  "git is missing" by string equality: a reworded message silently changes
  the behavior. Return a small enum (`Missing`, `Failed(reason)`).
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: crates/codev-engine/src/diff.rs, `Git::run` returns `GitError::{Missing, Failed}`; the caller matches `GitError::Missing`
  - status: fixed
    by: newcomer
    note: verified, no message is compared any more
```

#### R-newcomer-2 `filter_by_stack` assembles its output in two branches that mostly repeat

```yaml
severity: low
confidence: high
location: "crates/codev-core/src/roles.rs:652-669"
evidence: |
  let mut out: Vec<&str> = lines[..start].to_vec();
  if !kept.is_empty() {
      out.push(lines[start]);
recommendation: >-
  Both branches end by pushing the blank separator before the next
  section; only the kept section differs. Push the section when something
  is kept, then trim and separate once, so the reader sees one path.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: crates/codev-core/src/roles.rs `filter_by_stack`, the kept section is pushed when there is one, then one trim and one separator for both cases
  - status: fixed
    by: newcomer
    note: verified, one path, the five filter tests unchanged and green
```
