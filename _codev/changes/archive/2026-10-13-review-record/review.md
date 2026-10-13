# Review record: review-record

<!-- codev-review: 1 -->

## plan.challenge

### skeptic

#### R-skeptic-1 Teams that do not use the skills have no stated way out of the gate

```yaml
severity: medium
confidence: high
location: "proposal.md#Release note"
evidence: |
  started with an earlier codev, run the review again or turn its review
  roles off in its `change.yaml`.
recommendation: >-
  Say what a team that archives by hand, without the skills, must do:
  turn the review roles off for the project, an explicit and versioned
  "not reviewed".
history:
  - status: open
    by: skeptic
  - status: fixed
    by: product
    ref: proposal.md release note and design.md Migration Plan name `skeptic` and `newcomer` set to `never` in `_codev/config.yaml`
  - status: fixed
    by: skeptic
    note: verified in both artifacts
```

#### R-skeptic-2 Evidence must be whole lines while reviewers quote fragments

```yaml
severity: medium
confidence: high
location: "specs/review-record/spec.md#Requirement: The evidence of an unresolved finding is found in the current tree"
evidence: |
  evidence's non-blank lines appear consecutively in the file, comparing
recommendation: >-
  Match each quoted line as contained in the corresponding file line, so
  a reviewer can quote the fragment that matters; keep the consecutive
  order and the five-line window.
history:
  - status: open
    by: skeptic
  - status: fixed
    by: qa
    ref: spec and design now match each quoted line by containment, with a scenario quoting the start of a line
  - status: fixed
    by: skeptic
    note: verified, the scenario "Evidence at the cited lines" quotes a fragment
```

### security

#### R-security-1 "Absolute" depends on the OS running codev

```yaml
severity: medium
confidence: high
location: "specs/review-record/spec.md#Requirement: A finding states where, what and why, with a verbatim quote"
evidence: |
  of one of its headings, for the plan. A path MUST NOT be absolute nor
  contain `..`; `\` is read as `/`.
recommendation: >-
  Judge the location path as text, identically on every OS: no leading
  `/`, no `:` in the first segment (drive, scheme), no empty, `.` or `..`
  segment — checked before any read. `std::path` accepts `C:x` as
  relative on Unix (CWE-22, path traversal).
history:
  - status: open
    by: security
  - status: fixed
    by: qa
    ref: spec rule rewritten as a text rule, scenario adds `C:\secrets.txt:1`; design's hostile-record line updated
  - status: fixed
    by: security
    note: verified; the YAML side is also bounded (serde_norway 0.9.42 has a recursion and a repetition limit)
```

### migration

#### R-migration-1 A teammate on an older codev archives past the gate

```yaml
severity: medium
confidence: high
location: "design.md#Migration Plan"
evidence: |
  Nothing is migrated: archived changes are never checked. An active change
recommendation: >-
  Raise `codev_version` in this repository's `_codev/config.yaml` to the
  release that ships the record, and document the advice for teams.
history:
  - status: open
    by: migration
  - status: deferred
    by: architect
    to: the release that ships this change (bump `codev_version` to it in `_codev/config.yaml`); the dev binary is 0.6.1, so raising it now fails `codev doctor --strict`
```

## code.review

### newcomer

#### R-newcomer-1 `Fields::finding` takes seven parameters

```yaml
severity: low
confidence: high
location: "crates/codev-core/src/review/parse.rs:438-446"
evidence: |
  fn finding(
      &mut self,
      mapping: &Mapping,
      id: String,
      title: String,
      step: String,
      role: String,
recommendation: >-
  Pass the heading's data as one value: the four strings and the line are
  "where the finding sits", a missing type.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: crates/codev-core/src/review/parse.rs, `Fields::finding(&mut self, mapping, heading)` takes a `Heading` value
  - status: fixed
    by: newcomer
    note: verified, two parameters and a named type
```

#### R-newcomer-2 Tests strip a section the shared fixture should not have added

```yaml
severity: low
confidence: high
location: "crates/codev-engine/src/archive.rs:191"
evidence: |
  .replacen("### newcomer\n\nNo finding.\n\n", "", 1),
recommendation: >-
  Let `record` take the whole `## code.review` body, with a named constant
  for the newcomer's `No finding.`, so each test says what its record holds
  instead of patching the helper's output.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: crates/codev-engine/src/review.rs tests, `record(code_review)`, `record_with(plan, code_review)` and `NEWCOMER_NO_FINDING`; no test patches the helper's output any more
  - status: fixed
    by: newcomer
    note: verified in archive, sync, validate and review tests
```

#### R-newcomer-3 `ReviewV1::from` mixes steps, findings, issues and the gate

```yaml
severity: low
confidence: high
location: "crates/codev-cli/src/contract.rs:1069"
evidence: |
  impl From<&crate::commands::ReviewOutcome> for ReviewV1 {
recommendation: >-
  Extract the step and the finding mappings into functions so the
  conversion reads as a list of fields.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: crates/codev-cli/src/contract.rs, `review_step`, `review_finding` and `severity_label` extracted
  - status: fixed
    by: newcomer
    note: verified, the conversion reads as its fields
```

### security

#### R-security-2 A cited file is read whole before the size cap applies

```yaml
severity: medium
confidence: high
location: "crates/codev-engine/src/review.rs:154-156"
evidence: |
  match fs.read_to_string(path) {
      Err(_) => Cited::Unchecked("it is not UTF-8 text"),
      Ok(text) if text.len() > MAX_FILE_BYTES => {
recommendation: >-
  Ask the file system for the size first and skip the read above 4 MiB: a
  record citing a multi-gigabyte file (a dump, a build artifact) would
  otherwise load it whole into memory on every `codev validate`
  (CWE-400, uncontrolled resource consumption).
history:
  - status: open
    by: security
  - status: fixed
    by: developer
    ref: crates/codev-engine/src/ports.rs `FileSystem::len` (metadata on disk); review.rs asks it before reading; test `a_file_over_the_size_cap_is_not_compared` fails if the file is read
  - status: fixed
    by: security
    note: verified, the read happens only under the cap
```

#### R-security-3 A symlink inside the project lets a record probe a file outside it

```yaml
severity: low
confidence: high
location: "crates/codev-engine/src/review.rs:168"
evidence: |
  fn join(base: &Path, relative: &str) -> PathBuf {
recommendation: >-
  Canonicalize the joined path and refuse it when it leaves the project
  root; today `link -> /etc` plus `link/passwd:1` tells whether a quote is
  in that file, and `review_evidence_moved` names the line.
history:
  - status: open
    by: security
  - status: disputed
    by: developer
    note: >-
      Whoever can commit a symlink and a review.md to the repository can
      already run code on the reader's machine through the build and the
      tests the skills run; codev's answer only reaches the local user who
      runs it, who can read that file anyway, and no content is printed.
      Canonicalizing would need a new port and would also refuse the
      symlinks monorepos use on purpose.
  - status: rejected
    by: security
    note: agreed, no privilege is crossed; the text rule on the location stays the boundary
```

### migration

#### R-migration-2 A status added later would break older binaries without a format bump

```yaml
severity: low
confidence: high
location: "design.md#Every case, planned"
evidence: |
  a format marker above 1 is `review_format_unsupported` and blocks archiving; an unknown field is a warning.
recommendation: >-
  State the rule: a new field is additive (older binaries warn and ignore
  it), but a new status, step or required field is not — it raises the
  format marker, so older binaries refuse cleanly instead of reporting the
  record as invalid.
history:
  - status: open
    by: migration
  - status: fixed
    by: architect
    ref: design.md "Every case, planned" states the rule; reference/file-formats.md (en, fr) says it to users
  - status: fixed
    by: migration
    note: verified in the design and both file-formats pages
```
