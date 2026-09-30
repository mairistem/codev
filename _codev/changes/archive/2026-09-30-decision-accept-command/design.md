# Design: `decision new` creates a proposed decision, `decision accept` seals it

## Context

Every command that creates a decision computes a pure plan in
`codev-engine::decisions_actions` (`plan_new`, `plan_supersede`,
`plan_seal`, `plan_deviate`, `plan_promote`), and the CLI executes it with
`apply::execute`. `plan_new` already skips the seal for statuses other than
`accepted` and `superseded`, and `rewrite_frontmatter_status` already
rewrites a status while keeping the body byte for byte — `supersede` relies
on it so the predecessor's seal stays valid. The seal hashes the body only,
never the frontmatter.

## Goals / Non-Goals

**Goals:**

- A decision is sealed when its text is final, through one explicit command.
- Accepting writes the status and the seal in the same plan.

**Non-Goals:**

- Changing `supersede`, `deviate` or `promote`: they keep creating sealed
  `accepted` decisions.
- A general status-transition command (`deprecate`, `reject`): only the
  `proposed` → `accepted` transition carries a seal.

## Decisions

### Decision: `plan_accept` is a pure plan, like the other decision actions

Following `_codev/decisions/0001-functional-core-imperative-shell.md`,
`plan_accept` takes the index, the current seal file and a reader for the
ADR's source, and returns a plan with two writes: the ADR rewritten by
`rewrite_frontmatter_status` (`Overwrite`) and `seal.yaml` (`Overwrite`).
Every refusal — unknown id, inherited decision, status other than
`proposed`, unreadable seal file — happens while computing the plan, so
nothing is written. The body hash is computed on the rewritten content;
since only the frontmatter changes, it equals the hash of the body the user
wrote, which is what makes the seal meaningful. The shell reads files; the
function decides.

**Alternatives considered**: reusing `plan_seal` after a separate status
rewrite (two plans, so a failure between them leaves an accepted, unsealed
decision); letting `decision seal` also flip the status (mixes two
intentions in one command, and `seal` works on already accepted decisions).

### Decision: Inherited decisions are refused with their own code

Inherited sources are read-only (`_codev/decisions/0005-read-only-inherited-sources.md`).
Each decision command refusing an inherited target has its own code
(`cannot_supersede_inherited`, `cannot_seal_inherited`), so an agent can
tell which command it called wrongly; `decision accept` follows the pattern
with `cannot_accept_inherited`. A status other than `proposed` gets
`decision_not_proposed`, whose message depends on the status: an
`accepted` decision has nothing to accept and is replaced through
`supersede`.

**Alternatives considered**: reusing `cannot_seal_inherited` (its message
talks about sealing, which is only half of what `accept` does); making
`accept` a no-op on an accepted decision (it would hide a decision whose
body no longer matches its seal).

### Decision: `proposed` becomes the default of `decision new`

The clap default changes from `accepted` to `proposed`. `--status accepted`
keeps the old behavior for scripts and for decisions whose text is already
final. The JSON contract needs no new field: `decision new` already omitted
`bodySha256` for a `proposed` decision. `decision accept --json` returns a
new `DecisionAcceptedV1` shaped like `DecisionCreatedV1` (`decision`,
`path`, `bodySha256`), which keeps v1 additive.

## Risks / Trade-offs

- [A script relying on `decision new` sealing by default] → The CHANGELOG
  names the change and the `--status accepted` flag; `validate` reports a
  forgotten acceptance through the decision's `proposed` status, not
  through an error.
- [A proposed decision never accepted is not in effect] → This is the
  intended semantics of `proposed`; the human output of `decision new`
  names the `accept` command as the next step.
- [`deviate` and `promote` still seal a body that is meant to be edited] →
  Out of scope here; they are the same problem and can adopt `proposed` +
  `accept` in a later change.
