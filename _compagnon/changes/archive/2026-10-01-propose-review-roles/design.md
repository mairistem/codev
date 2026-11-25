# Design: Explicit roles, traceability check and contrarian pass in /codev-propose

## Context

The `spec-driven` schema in `assets/schemas/spec-driven/schema.yaml` is
embedded in the binary; `codev instructions` returns each artifact's
`instruction` verbatim, with the project's `context` and the artifact's
`rules` next to it. The `propose` workflow in `assets/workflows/propose.md`
is rendered into `.claude/skills/codev-propose/SKILL.md` by `codev update`.
Each instruction already asks one question (WHY, WHAT, HOW, breakdown) but
names no role and no completion criteria. The workflow ends at "Show the
final status" with no review of what was written.

## Goals / Non-Goals

**Goals:**

- Make each artifact's owner and its completion criteria explicit and
  checkable, where a custom schema can redefine them.
- Give `/codev-propose` two review steps that run before the plan is shown,
  and a bounded output for what needs a human eye.

**Non-Goals:**

- A new CLI command or a machine check of traceability: the check is done by
  the agent, against artifacts it just wrote.
- Separate agents or skills per role.
- Changing `codev validate`: the new criteria are guidance, not findings.

## Decisions

### Decision: Roles live in the schema instructions, as a question and criteria

Each instruction starts with a `Role: <role> — owns <what>.` line and a
`Done when:` list, then keeps its existing text. The schema is where a
project already customizes artifacts (`_codev/schemas/`), so a custom schema
defines its own roles — or none — without codev imposing them. Roles are
written as ownership plus checkable criteria, not as role-play: a criterion
can be verified, a persona cannot.

**Alternatives considered**: roles in `propose.md` (they would apply to any
schema, including custom artifacts they do not fit, and `/codev-update`
would not see them); a new `role:` field in the schema format (a format
change and a JSON contract change for what is plain guidance); persona
prompts such as "you are a senior architect" (not checkable, and they add
tone rather than criteria).

### Decision: Two sequential review steps in the workflow, traceability first

The traceability check runs first because it is mechanical and fixes
coverage gaps; the contrarian pass then reviews a plan whose coverage is
already sound, so it can spend its attention on need, scope and
assumptions. Both re-read the artifacts from disk, as the workflow already
requires for dependencies. Both stay inside the planning boundary: they edit
artifacts only.

**Alternatives considered**: a single merged review step (the mechanical
coverage check would crowd out the judgment questions); running the review
in a separate sub-agent (a heavier mechanism that Claude Code skills do not
need for this, and the skill must stay a single prompt).

### Decision: The contrarian pass corrects or records, never adds scope

Unambiguous fixes go directly into the artifacts; what needs a human
decision goes into `design.md` Open Questions, or the proposal when there is
no design. The user is not asked mid-way unless scope changes materially —
the existing rule of step 1. The conditional lenses apply only when the
proposal's Impact touches them, so a CLI change does not receive an
accessibility review; project-specific lenses come from `rules:`, which the
skill already receives through `codev instructions`.

**Alternatives considered**: asking the user about each finding (breaks the
one-pass promise of `/codev-propose`); a fixed list of every lens on every
change (noise that trains the reader to skip the block).

### Decision: "Points to challenge" is capped at five and may be empty

The summary lists at most five one-line items ranked by impact, or says "No
point to challenge found". A cap keeps the block readable at approval time;
allowing an empty result removes the incentive to invent points. The
summary stays in the conversation's language, like the rest of the chat
output; the artifacts keep following `language`.

**Alternatives considered**: no cap (the block grows into a second review
document); a mandatory minimum of points (invites invented findings).

The approach follows `_codev/decisions/0004-single-identity-for-skill-and-command.md`:
the steps are added to the existing `codev-propose` skill, not shipped as a
new skill or command.

## Risks / Trade-offs

- [The skill grows longer and costs more tokens per run] → The new steps
  are short checklists; the guardrails take one line each.
- [The contrarian pass rewrites what the user asked for] → Guardrail "it
  never adds scope"; ambiguous findings are recorded, not applied.
- [Tests or docs quoting the old instruction text break] → The existing
  text is kept after the role block; the instruction tests are updated.
- [A project already using `rules:` sees them applied twice, at writing and
  at review] → Intended: a rule is a constraint at writing time and a check
  at review time.
