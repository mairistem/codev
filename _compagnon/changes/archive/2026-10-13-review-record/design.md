# Design: A review record with evidence and severity that gates archiving

## Context

The reviewing steps are `plan.challenge` (the `skeptic` and the active
experts) and `code.review` (the `newcomer` and the active experts).
`codev roles --at <step> --change <name>` already resolves who is active
for a change (`codev-engine::roles::resolve`, the change's settings and
domains applied). Their findings exist only in the conversation and in
the skills' summaries (`assets/workflows/propose.md` step 7,
`apply.md` step 5).

`codev archive` runs `codev-engine::sync::plan_sync`, whose validation
pre-flight refuses with `validation_failed`, then moves the change folder
whole (`codev-engine::archive`). `codev status` derives the artifacts'
states from the schema's `generates` only, `codev sync` reads only
`specs/`, `codev changelog` reads only the proposal: none of them looks
at an unknown file of the change folder, which is what lets older
binaries ignore `review.md`.

Validation codes are `&'static str` constants in `codev-core`, gathered
by the uniqueness test `finding_codes_are_unique`; YAML is parsed with
`serde_norway` (decision 0006); the core does no I/O and does not derive
`Serialize` (decision 0001); the JSON contract lives in
`codev-cli::contract::v1` (decision 0002).

## Goals / Non-Goals

**Goals:**

- One file per change that a human reads top to bottom and that codev
  parses without guessing.
- Every check that needs no judgement done by the binary: format,
  history, locations, evidence, completeness, archive gate.
- Judgement — what is a finding, its severity, the fix, the argument —
  stays in the skills.

**Non-Goals:**

- Checking that a `fixed` reference is a real commit, or that a `deferred`
  destination exists: free text, read by humans.
- Deduplicating findings across roles, running reviewers in isolation
  (`isolated-review`), project knowledge and a risk register
  (`team-practices`).
- Records for steps other than `plan.challenge` and `code.review`: the
  tracer fixes gaps directly, as today.
- Showing the record in `codev status`.

## Decisions

### Decision: Markdown headings for structure, one YAML block per finding

```markdown
# Review record: add-sessions

<!-- codev-review: 1 -->

## plan.challenge

### skeptic

No finding.

### security

#### R-security-1 Session tokens are stored in clear

```yaml
severity: high
confidence: high
location: design.md#Decision: Sessions live in the database
evidence: |
  The session token is stored in the `sessions` table.
recommendation: Store a SHA-256 of the token and compare digests.
history:
  - status: open
    by: security
  - status: fixed
    by: product
    ref: design.md, the decision now stores a digest
  - status: fixed
    by: security
    note: verified
```
```

The step and the role come from the headings, never repeated in the
block, so they cannot disagree. The title follows the id on the
heading, so a rendered record reads as a list of titled findings. The
status is the last history entry: one source of truth, and the history is
the audit trail. The block is parsed as a `serde_norway::Value` and read
field by field, so that an unknown field is a warning, not a parse
failure (forward compatibility), and messages name the field.

**Alternatives considered**: a single YAML or JSON file (machine-perfect,
unpleasant to review in a pull request); a strict `- **Field**: value`
layout (multi-line evidence and history need ad-hoc escaping); repeating
step and role in each block (two sources that can drift).

### Decision: `codev-core::review` is pure; `codev-engine::review` reads

`codev-core::review` holds the scales (`Severity`, `Confidence`, each
with its one-sentence definition), `Status`, the parser
(`parse(&str) -> Parsed<Record>`), the history rules, the evidence matcher
(`match_lines(file, quote, start, end) -> EvidenceMatch`, `section_lines`
for plan locations), the completeness check (given the active roles per
step) and the gate (given the active roles per step and whether the
record exists). `codev-engine::review::check(fs, layout, config, ctx)`
reads `review.md` and each cited file once (a cache by path), resolves
the roles with `roles::resolve`, and returns the record, the located
findings and the gate. `validate_change` adds the findings; `archive`
asks for the gate after the pre-flight. Decision 0001 is followed: the
core gets strings, the engine does the reading.

**Alternatives considered**: checks in `codev-engine::validate` directly
(untestable without files, and the gate would duplicate them).

### Decision: Who may set a status is checked, judgement is not

The history rules are the table of the spec: a closed transition list,
`accepted-risk` only `by: human` or `human:<name>`, a `high` finding
deferred only by a human, `rejected` and the verification of a fix only by
the finding's role or a human. Missing `ref`, `note` or `to` is
`review_decision_incomplete`, distinct from `review_invalid`, so that a
skill can tell a broken file from a decision recorded without its reason.
`by` is otherwise free: the author is the role owning the artifact at
`plan.challenge` (from `codev instructions`) and `developer` at
`code.review`; codev does not check that the right author answered.

**Alternatives considered**: a separate `decided_by` field (a second
"who" next to `by`); no transition rules (a `rejected` by the author
would pass silently).

### Decision: Evidence is matched by normalized lines within five lines

A quote matches when its non-blank lines, each trimmed with whitespace
runs collapsed and `\r` removed, are each contained in consecutive
non-blank lines of the file, normalized the same way, starting between
`start - 5` and `end + 5`. Containment rather than equality lets a
reviewer quote the fragment that matters without the rest of a long
line. Found elsewhere: `review_evidence_moved` (warning, with
the line); nowhere: `review_evidence_not_found` (error). Five lines
absorb an import added above the cited code without accepting a quote
from another function. Only `open`, `disputed`, `accepted-risk` and
`deferred` findings are checked: they describe the current tree. A plan
location's section runs from its heading to the next heading of the same
or a higher level.

**Alternatives considered**: exact match (fails on reformatting and
CRLF); fuzzy similarity (not deterministic enough to explain); a content
hash of the cited lines (fails on any edit, and a human cannot read it).

### Decision: Validation checks the record; archive checks the gate

`codev validate` reports defects of the record — never an `open` finding,
which is the normal state of a change under review. The gate —
`review_record_missing`, `review_step_missing`, `review_unresolved`,
`review_format_unsupported` — is computed by `codev review` and enforced
by `codev archive` only, with one `EngineError::ReviewBlocked` whose code
`review_blocked` is stable and whose message lists the reasons, as
`validation_failed` points to `codev validate`. A step needs a section
only when at least one role is active at it for the change; the roles are
resolved as `codev roles` does. `codev sync` stays ungated: it publishes
specs early, it does not close the change.

**Alternatives considered**: the gate inside `codev validate` (every
change under review would fail `--strict`); a gate in `sync` too (blocks
the documented early-sync use); a `--force` on archive (the escape is an
`accepted-risk` decision, recorded with a justification).

### Decision: `codev review` reports; its exit code reflects record errors only

`codev review [--change <name>] [--json]`: the change is resolved like
`status`; without a record it succeeds with `exists: false`. `ReviewV1`:

```json
{
  "changeName": "add-sessions",
  "path": "_codev/changes/add-sessions/review.md",
  "exists": true,
  "format": 1,
  "steps": [
    { "step": "plan.challenge", "recorded": true, "activeRoles": ["skeptic", "security"],
      "roles": [ { "role": "skeptic", "state": "no-finding" },
                 { "role": "security", "state": "findings" } ] }
  ],
  "findings": [
    { "id": "R-security-1", "title": "Session tokens are stored in clear",
      "role": "security", "step": "plan.challenge", "line": 14,
      "severity": "high", "confidence": "high", "reported": true,
      "location": "design.md#Decision: Sessions live in the database",
      "evidence": "The session token is stored in the `sessions` table.\n",
      "recommendation": "…", "status": "fixed", "awaiting": null,
      "history": [ { "status": "open", "by": "security", "note": null, "ref": null, "to": null } ] }
  ],
  "issues": [ { "severity": "warning", "code": "review_evidence_moved", "path": "…", "line": 20, "message": "…" } ],
  "archive": { "ready": false, "blockers": [ { "code": "review_step_missing", "message": "…" } ] },
  "status": []
}
```

`state` is `findings`, `no-finding`, `disabled` or `missing`; `awaiting`
is `author`, `reviewer`, `human` or `null`. Exit code 1 on an error issue
only: the gate is a property of the change, read by the skills and
enforced by `archive`. The skills keep `Bash(codev:*)` (decision 0004: no
new tool, no command file).

**Alternatives considered**: exit 1 when not archive-ready (the command
would fail during the whole plan phase, when `code.review` is legitimately
missing); folding it into `codev status` (the planning contract would
change for every consumer).

### Decision: One shared procedure for the skills, the scales generated from the core

The record-writing procedure and the answer loop live once, in
`assets/workflows/_review-record.md`, substituted for `{{REVIEW_RECORD}}`
in `propose.md` (step 7) and `apply.md` (step 5) at rendering, as the
update notice is. Inside it, `{{REVIEW_SCALE}}` is replaced by the
definitions of `codev-core::review` — the single source the docs test
also compares the English Roles chapter with. `archive.md` handles
`review_blocked` like `validation_failed`: one fixed sentence, no retry.

**Alternatives considered**: two copies in the two workflows (they would
drift); serving the scales in `codev roles --json` (a contract change for
data that never varies per project).

### Every case, planned

| Case | Behavior |
|---|---|
| No git repository | Nothing uses git: checks read the working tree. Scenario "Evidence at the cited lines" runs on the in-memory file system. |
| A change with no diff yet | `code.review` records `No finding.` per role; nothing to cite. Covered by "A well-formed record". |
| A docs-only change | Same rules; a markdown file is cited with `path:line`. No special case. |
| Generated, vendored files | Cited and checked like any file. |
| Binary files | Not UTF-8: `review_evidence_unchecked` (scenario "A binary file"). |
| Very large files | Over 4 MiB: `review_evidence_unchecked`, the file is not compared (test `a_file_over_the_size_cap_is_not_compared`). |
| Huge diffs / many findings | No cap on findings in the record; each cited file is read once. The caps on what is shown stay in the skills (five, the high-severity exception). |
| Monorepos with several stacks | Paths are relative to the codev project root; a file outside it cannot be cited (`..` refused). Out of scope: citing a sibling package outside the root. |
| Windows paths and CRLF | `\` in a location is read as `/`; `\r` ignored in the record and in cited files (scenario "Windows line endings", test `evidence_ignores_crlf`). |
| Offline | No network. |
| A role disabled by configuration | Its section is optional, `Disabled by configuration.` accepted; written for an active role, it is `review_role_missing` (scenarios "A role disabled by configuration", "Every review role turned off"). |
| A step re-run (idempotence) | The skill reads the record first and does not re-raise a recorded finding; a role or step written twice is `review_invalid`; ids are never reused. `codev review` and `validate` are read-only. |
| An interrupted run resumed | The record is the state; `awaiting` says who acts next (scenario "A resumed session"). |
| Inherited sources | Roles are resolved as `codev roles` does, inherited configuration included; no record is inherited. Out of scope: records in inherited sources. |
| Older installed skills | An `archive` skill generated before this change relays the message of any code it does not know: `review_blocked` reaches the user with its reasons. |
| Older binaries reading newer files | A codev without this change ignores `review.md` (it reads no unknown file of a change); a format marker above 1 is `review_format_unsupported` and blocks archiving; an unknown field is a warning. The rule for later versions: a new optional field is additive; a new status, step or required field raises the format marker, so that an older codev refuses cleanly instead of calling the record invalid. Pin `codev_version` to make a team upgrade together. |
| Hostile record | A location path is judged as text, the same on every OS (no leading `/`, no `:` in the first segment, no empty, `.` or `..` segment), before any read — `std::path` would accept `C:x` on Unix and `/x` differently on Windows; only found/not found is reported, never file content; YAML parsed by `serde_norway`, whose alias expansion is bounded. |

## Risks / Trade-offs

- [Active changes without a record cannot be archived after upgrading] →
  **BREAKING** in the release note; two documented ways out: run the
  review steps again, or set the review roles to `never` in the change's
  `change.yaml` — an explicit, versioned "not reviewed".
- [An agent records `accepted-risk` itself to get past the gate] → it
  needs `by: human`, which the skill text forbids writing without the
  human's answer; the record is in git, so the forgery is visible in
  review. codev cannot prove who typed it.
- [Evidence tolerance hides a quote from the wrong place] → five lines
  only, and an exact (normalized) consecutive match.
- [The skill texts grow] → the procedure is shared once and the scales are
  generated.
- [A reviewer writes evidence that is not verbatim] → it fails validation
  (`review_evidence_not_found`), which is the point.

## Migration Plan

Nothing is migrated: archived changes are never checked. An active change
started earlier gets `review_blocked` at archive until it has a record or
its review roles are off; a team that does not use the skills at all sets
`skeptic` and `newcomer` to `never` in `_codev/config.yaml`. A team mixing
codev versions raises `codev_version` to the release that ships the
record, so that nobody archives with a codev that ignores the gate. Rollback: revert the commit; `review.md` files
are then ignored.

## Open Questions

None.
