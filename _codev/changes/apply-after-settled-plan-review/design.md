# Design: No implementation over an unsettled plan review

## Context

- `codev-core::review::gate` (`crates/codev-core/src/review/gate.rs`)
  judges a record for `codev archive`: missing record or step while roles
  are active, a format newer than this codev reads, and every finding
  whose `blocks_archive()` is true — `high` and `open` or `disputed` —
  whatever its step. `codev-engine::review::check` reads `review.md`,
  resolves the active roles, checks each finding's evidence against the
  tree, and calls the gate.
- `/codev-apply` (`assets/workflows/apply.md`, rendered to
  `.claude/skills/codev-apply/SKILL.md`) runs `codev status --change
  <name> --json` in its step 1 and stops only when `isPlanningComplete`
  is `false`. It never reads the record before coding.
- `codev status` is also read by `/codev-propose` (steps 3 and 5, before
  the record exists) and by `/codev-update`. Its JSON (`StatusV1` in
  `crates/codev-cli/src/contract.rs`) is a public contract read by skills
  already installed: additive changes only.
- A human already has the override: `accepted-risk` (with a `note`) or
  `deferred` (with `to:`), `by: human` or `human:<name>`, in the
  finding's history. The skills never write either on their own.
- `change.yaml` is parsed with `deny_unknown_fields`: a new key there
  breaks every older codev that reads the change. A parallel change,
  `forward-compatibility-policy`, is relaxing that policy; it has not
  landed.

## Goals / Non-Goals

**Goals**

- `/codev-apply` refuses, before any task, while the plan review holds a
  `high` finding `open` or `disputed`, with the stable code
  `plan_review_unsettled`.
- The judgement is codev's, deterministic and tested in the binary; the
  skill only reads it and acts.
- No new call for the skill, no new setting, no new file.

**Non-Goals**

- An approval command, a plan fingerprint or hash, a link to a pull
  request or any forge, identity management. A solo developer approves by
  running `/codev-apply`; a team approves through its pull request
  review. codev stays standalone.
- Blocking on a missing record or a missing `plan.challenge` section:
  `codev archive` keeps doing that.
- Changing `codev archive`, `codev review`, or `/codev-propose`'s text.

## Decisions

### Decision: The gate is always on, with no setting

The refusal applies to every project. A `high` finding is defined as one
that "must not be archived until it is fixed or a human accepts the
risk"; implementing over it is the costliest moment to discover that the
plan must change. The conscious override already exists and stays a
human act recorded in `review.md`: `accepted-risk` with its
justification, or `deferred` with where it goes. A team that does not
want plan findings at all already turns off the plan's roles (`skeptic:
never` and no domain), and then no finding is written.

No key is added to `change.yaml`, so the change does not depend on
`forward-compatibility-policy` and no older codev breaks on a change
written by this one. No key is added to `config.yaml` either: there is no
behavior to configure that the record does not already express.

**Alternatives considered**:

- *Opt-in through `config.yaml` (`review: { gate_apply: true }`)*: the
  projects that most need it would not know to turn it on, and a key under
  `review:` is read by older codevs as an unknown key of a mapping they
  validate; rejected.
- *A per-change switch in `change.yaml`*: breaks older codevs (decision
  context above) and gives the agent a one-line way around the gate,
  which is the opposite of a human decision; rejected.
- *A flag on the skill invocation (`/codev-apply --force`)*: an agent in a
  run without a user could pass it; the override must leave a trace in
  the record, with a justification and a human's name.

### Decision: Exposed as `planReview` in `codev status --json`, present only with a record

`StatusV1` gains `planReview: { settled, blockers: [{ code, message }] }`,
serialized only when the change has a `review.md`. The skill already
runs `codev status --change <name> --json` at step 1: the gate costs it
no extra call and no extra turn (decision 0010: savings and costs are
decided by calls and turns, not text). Without a record, the field is
absent, which the skill reads as "nothing blocks" — exactly what a codev
that predates the field gives, so the skill has one rule for both.

Present only with a record also keeps the document byte-identical for
every `codev status` run before the review exists: `/codev-propose` at
`plan.write`, and every existing implementation case, whose fixtures
hold no `review.md`. It does not for `/codev-update`, which runs `codev
status --json` on changes already reviewed: two `plan.update` cases
(`update-tasks-add`, `update-specs-ripple-tasks`) hold a `review.md`, one
with a `high` finding. The change therefore touches `code.implement`
(skill text and JSON) and `plan.update` (JSON only), both measured as
decision 0008 requires. `/codev-propose` reads the new human line only
at its step 8, after its review is written, and only when a `high`
finding is left unsettled: it prints it, and nothing in its steps acts
on it.

`codev status` keeps exiting 0: `/codev-propose` calls it in its loop,
and a failing status would stop planning. The refusal code
`plan_review_unsettled` is the skill's, stated in its reply; the blocker
codes inside it are the archive gate's (`review_unresolved`,
`review_format_unsupported`), so a reader recognises them.

The human `codev status` adds one `Plan review: not settled — …` line
only when unsettled, after the planning lines, naming the findings and
`codev review --change <name>`.

**Alternatives considered**:

- *An `apply` gate object in `codev review --json`, next to `archive`*:
  the skill would make a second call at step 1, for every run, settled or
  not; rejected for its turn.
- *Always present, `settled: true` without a record*: changes the JSON
  read at every `plan.write` run as well, which would bring that step
  under decision 0008's evaluation for no behavior; rejected.
- *A failing command (`codev status` exits 1, or a new `codev
  apply-check`)*: breaks propose's loop, or adds a command and a turn.

### Decision: The archive gate's judgement, restricted to `plan.challenge`

`codev-core::review` gains a pure function, `plan_gate(record) ->
Vec<Blocker>`, built from the same pieces as `gate`: the format check
(`review_format_unsupported`), then every finding of the
`plan.challenge` step whose `blocks_archive()` is true
(`review_unresolved`, same message shape), and every `high` finding of
`plan.challenge` whose last entry is `accepted-risk` or `deferred` while
its history breaks the record's rules for that decision — not by a
human, no `note`, no `to` — (`review_decision_incomplete`, reusing the
history check that `codev validate` already runs). Without that last
rule, an agent appending `accepted-risk` by a role would settle the plan
in the binary's eyes, and only `codev archive` would catch it, after the
code. `gate` and `plan_gate` share a
helper for the unresolved blockers, filtered by step, so the two can
never disagree on what "unresolved" means (decision 0001: the core
decides, without I/O).

It does not take the active roles: a missing record or step is not a
blocker here (see Non-Goals), and an unresolved finding blocks whether
its role is still active or not, as it does for the archive.

`codev-engine` gains `review::plan_review(fs, layout, change) ->
Result<Option<Vec<Blocker>>>`: `None` without `review.md`, otherwise it
reads and parses the record and calls `plan_gate`. The only error is a
`review.md` that exists but cannot be read: it fails `codev status` with
the engine's existing unreadable-file error, as any unreadable file of
the change does — an I/O failure, not a state of the review, so the
"exit code 0 whether settled or not" of the specs is not about it. It does not resolve roles nor
compare evidence with the tree, as `review::check` does: `codev status`
runs often, and neither is needed to judge a status. `commands::status`
fills `StatusOutcome.plan_review`; the CLI maps it to `PlanReviewV1` in
`contract.rs`, never serializing the core's `Blocker` directly.

Findings of `code.review` never count: on a second `/codev-apply` (a
resumed session), the record already holds code findings, which the
skill's own step 5 answers.

**Alternatives considered**:

- *Calling `review::check` and filtering its blockers*: it reads every
  cited file and resolves the roles on each status, and its blockers mix
  in `review_record_missing` / `review_step_missing`, which must not
  block here.
- *Generalizing `gate` with a step parameter*: changes the archive's
  signature and tests for one caller; the shared helper gives the same
  guarantee.

### Decision: The skill refuses, names the way out, and never settles it itself

In `assets/workflows/apply.md`, step 1 gains a paragraph right after the
`isPlanningComplete` check: if `planReview` is present and `settled` is
`false`, stop with `plan_review_unsettled`, list each blocker's message
and code, point to `codev review --change <name>`, and give the ways out
— fix the plan with `/codev-update` then record the fix in `review.md`,
or a human records `accepted-risk` with a justification or `deferred`
with where it goes — modify no file, check no task. Even in a run without
a user, the skill does not write the human's decision. Absent
`planReview` means proceed.

**Alternatives considered**:

- *Asking the user to confirm and going on*: an answer in the
  conversation leaves no trace; the record is where a risk is accepted.

### Decision: Measured on `code.implement` with a second refusal case

Decision 0008 applies: the change alters what `/codev-apply` reads and
does. A new case, `implement-refuse-unsettled-plan-review`, lays a
complete plan (taken from an existing implementation case, with its
unchecked `tasks.md`) and a `review.md` whose `plan.challenge` holds a
`high` finding `open`. The case format learns two things: a refusing
case may hold a `tasks.md` provided it holds the `review.md` that makes
it refuse, and `expected.points_to` names what the refusal must name
(`plan_review_unsettled` here; `/codev-propose` by default, so the
existing refusal case is unchanged).

The candidate is run with `--step code.implement --repeat 5 --jobs 2`
and compared with `compare --quality-only` against `evals/baseline/` on
the four existing cases: the pass and acceptance shares must stay within
the baseline's range, and the refusing share must not fall. The new
case's five runs, recorded with the candidate, join the baseline (there
is nothing earlier to compare it to); `codev-evals docs` regenerates the
reliability chapter. `plan.update` is run too, with `--quality-only`
against the baseline, because two of its cases now see `planReview`.
`plan.write` is not run: its JSON is byte-identical, as the previous
decision shows, and its skill text does not change.

**Alternatives considered**:

- *No new case, a unit test of the rendered skill only*: shows the text
  is there, not that an agent obeys it; decision 0008 asks for the
  measure.
- *Running `plan.write` as well*: about 1.65 USD per case for an input
  that does not change.

## Risks / Trade-offs

- [A change in flight, planned before this release, holds a `high` plan
  finding still open] → `/codev-apply` now refuses it. Mitigation: the
  refusal names the finding and the two ways out; the release note says
  so.
- [A record from a newer codev blocks the implementation] → consistent
  with the archive, and the message says to update codev; without it,
  an older codev would implement over findings it cannot read.
- [The agent ignores the refusal and implements anyway] → the eval case
  measures exactly that, and `codev archive` still refuses the change
  afterwards.
- [An unparsable finding (a `review_invalid` error) is not counted] →
  same as the archive gate today; `codev validate` and `codev review`
  report the error, and the archive pre-flight refuses a change that
  fails validation.
- [Teams that already accept the risk informally in a pull request] →
  they record it in `review.md` with `by: human:<name>`; one line, and
  it is the record the archive gate already demands.

## Migration Plan

Additive for the JSON contract: older skills ignore `planReview`; a newer
skill under an older codev sees no field and proceeds. No data
migration. Rollback: revert the release; records are unchanged, since
the change writes nothing to them.

## Open Questions

- Should `/codev-propose`'s summary say, under "To decide", that
  `/codev-apply` will refuse until each `high` finding is settled? It
  would change propose's text and bring `plan.challenge` under decision
  0008's evaluation; left to a later change.
- Is an agent run on a record whose `high` plan finding is
  `accepted-risk` worth its cost as a third implementation refusal-free
  case? The go-on path is decided by the binary (tasks 1.1 and 2.2) and
  stated in the skill (task 3.1); a case can be added later without
  changing this plan.
