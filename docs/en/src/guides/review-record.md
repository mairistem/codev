# Review record

When the skeptic and the experts challenge a plan, or the newcomer and the
experts review the code, what they find is not left in the conversation.
It goes to the change's review record, `_codev/changes/<name>/review.md`:
each finding with its severity, the evidence it rests on, and every answer
it received — who fixed it, who disputed it and why, who accepted the risk.
codev checks what can be checked, and `codev archive` will not close a
change whose review is not settled.

## What a finding holds

````markdown
## code.review

### security

#### R-security-1 The session lookup concatenates the token

```yaml
severity: high
confidence: high
location: "src/auth/session.rs:42-44"
evidence: |
  let query = format!("SELECT * FROM sessions WHERE token = '{}'", token);
recommendation: Bind the token as a query parameter.
history:
  - status: open
    by: security
```
````

- **The id**, `R-security-1`, is stable for the life of the change: the
  role's id and the next free number. It is never renumbered or reused, so
  a discussion can cite it.
- **The severity** says how bad it is if the reviewer is right, and **the
  confidence** how sure the reviewer is — see the scales in
  [Roles](../roles.md#findings-severity-confidence-and-the-review-record).
  A low-confidence finding is kept and answered, but not shown to you
  unless you must decide it.
- **The location** is `path:line` or `path:start-end` from the project
  root for code, or `<file>#<heading>` from the change folder for the plan
  (`"design.md#Decision: Sessions live in the database"`). Quote it: YAML
  reads `: ` as a new key.
- **The evidence** is the cited text, copied verbatim — whole lines or the
  fragment that matters. A reviewer proves what it says.

The full format is in [File formats](../reference/file-formats.md#reviewmd).

## Isolated reviews

In [isolated mode](../roles.md#inline-or-isolated-review) the record also
says how the step ran, and who found what:

````markdown
## code.review

Mode: isolated.
Summarized: src/generated/schema.rs — over the material limit.

### security

#### R-security-1 The session lookup concatenates the token

```yaml
severity: high
confidence: high
location: "src/auth/session.rs:42-44"
evidence: |
  let query = format!("SELECT * FROM sessions WHERE token = '{}'", token);
recommendation: Bind the token as a query parameter.
raised_by: [security, newcomer]
history:
  - status: open
    by: security
```

### newcomer

Fallback: the sub-agent was stopped twice by its time limit.

No finding.
````

- **`Mode:`** — `inline` or `isolated`, under the step's heading.
- **`Summarized:`** — one line per file the reviewers received as a
  summary rather than whole, because the material was over its limit.
- **`raised_by`** — when several reviewers found the same defect, the
  finding is written once, under the role that rated it most severe, and
  lists every role that raised it, its own first. That role re-checks
  the answers. A role whose findings were all merged says `No finding.`;
  `codev review` shows it as credited on the shared finding.
- **`Fallback:`** — the role's sub-agent failed twice and it reviewed
  inline, in the conversation.

`codev validate` reports a `Mode:` other than `inline` or `isolated`, a
second `Mode:` or `Fallback:` line, and a `raised_by` that is not a list
of distinct role ids starting with the finding's own role
(`review_invalid`). A codev that predates these lines ignores them, and
warns about `raised_by` as an unknown field (`review_unknown_field`).

## The answer loop

A finding starts `open`. Then:

1. **The author answers.** The role that owns the cited artifact at
   `plan.challenge` — product owner, QA analyst, architect, tech lead —
   or the developer at `code.review`, fixes it (`fixed`, with `ref`: the
   commit or the file change) or disputes it (`disputed`, with `note`: the
   argument).
2. **The reviewer checks the answer, once.** A fix is verified (`fixed`
   again, by the reviewer) or reopened (`open`, with why). A dispute is
   accepted (`rejected`) or maintained (`disputed`, with why).
3. **What is left comes to you.** The skill's summary lists, under **To
   decide**, every dispute the reviewer maintained and every high-severity
   finding still open, with both arguments. You fix it, accept the risk
   (`accepted-risk`, with a justification) or defer it (`deferred`, with
   `to`: the change or the issue it goes to). The skill records your
   decision as `by: human:<your name>`; it never decides for you.

| Status | Set by | Requires |
|---|---|---|
| `open` | the reviewer (first entry, or a fix that does not hold) | `note` when it reopens a fix |
| `fixed` | the author, then the reviewer to verify it | `ref` |
| `disputed` | the author, or the reviewer maintaining it | `note` |
| `rejected` | the reviewer, or a human, after a dispute | — |
| `accepted-risk` | a human | `note`: the justification |
| `deferred` | the author, or a human for a `high` finding | `to` |

`accepted-risk`, `deferred` and `rejected` are final: to revisit one, raise
a new finding.

## What codev checks

`codev validate` reads the record of every active change. It never judges
a finding — an `open` finding is the normal state of a review — but it
checks everything that needs no judgement:

- **The format**: the headings, one `yaml` block per finding, the fields,
  the ids, the locations (`review_invalid`), and an unknown field
  (`review_unknown_field`, a warning).
- **The history**: the first entry is `open` by the finding's role, each
  transition is in the table above (`review_invalid`), and each entry
  carries what its status requires, decided by whom it must be
  (`review_decision_incomplete`).
- **The evidence**, for `open`, `disputed`, `accepted-risk` and `deferred`
  findings, against the files as they are now: the file, the line or the
  heading exists (`review_location_missing`); each quoted line is found in
  consecutive lines of the file, whitespace and line endings ignored,
  starting no more than 5 lines from the cited ones
  (`review_evidence_not_found`, or `review_evidence_moved` when it is
  further away). A `fixed` or `rejected` finding is not compared: its code
  was meant to change. A file that is not UTF-8 text, holds NUL bytes or
  is larger than 4 MiB is not compared (`review_evidence_unchecked`).
- **The sections**: when the record holds a step, every role active at it
  for the change has its section (`review_role_missing`). A role turned
  off may say `Disabled by configuration.`.

No check uses git or the network: they read the working tree.

## codev review

```bash
codev review --change add-sessions
```

Shows the record step by step — each role, each finding with its severity,
status and location —, who must act next on each one (`author`,
`reviewer` or `human`), the record's defects, and whether the review lets
the change be archived. The skills read this printed output to resume an
interrupted review and to check the record; `--json` gives the
[`ReviewV1`](../reference/json-output.md#review) document, for scripts. It exits with 1 only when the record has
an error.

## Archiving

`codev archive` checks the review after validation. It refuses, with the
code `review_blocked`, when:

- roles were active at `plan.challenge` or `code.review` and the record
  does not exist (`review_record_missing`), or has no section for that
  step (`review_step_missing`);
- a `high` finding is still `open` or `disputed` (`review_unresolved`);
- the record comes from a newer codev (`review_format_unsupported`).

A step with no active role needs no section. `codev sync` does not look
at the record.

## Changes started before the record

A change planned with an earlier codev has no record, so archiving it is
refused. Either run the review steps again — `/codev-apply` reviews the
code and writes the record — or say explicitly that the change was not
reviewed by turning its review roles off in its `change.yaml`:

```yaml
roles:
  skeptic: never
  newcomer: never
```

A team that does not use the skills at all sets the same in
`_codev/config.yaml`. To keep everyone on a codev that enforces the gate,
raise [`codev_version`](../reference/configuration.md) to the release that
ships it: an older codev ignores `review.md` and archives anyway.

## Edge cases

- **Re-running a step** adds to the record; a finding already recorded is
  not raised again, and a step or a role written twice is an error.
- **An interrupted session** resumes from the record: `awaiting` says who
  must answer next.
- **A docs-only change, or one with no code yet**, still has its review
  sections — `No finding.` is a valid answer.
- **Generated, vendored or binary files** can be cited; binary and very
  large files are not compared.
- **Windows**: `\` in a location reads as `/`, and `\r\n` line endings are
  ignored in the record and in the cited files.
- **Monorepos**: paths are relative to the codev project root; a file
  outside it cannot be cited.
