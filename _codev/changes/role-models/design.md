# Design: Give each role the model its reviews run on

## Context

Roles are Markdown files under `assets/roles/` with a YAML frontmatter
(`id`, `name`, `at`, `question`, `done_when`, …), read by
`codev-core::roles`, configured under `roles:` and served by `codev
roles`. In the `isolated` review mode, the skills start each reviewing
role as a sub-agent with Claude Code's `Agent` tool, which takes a
`model` (decision 0007: the orchestration is skill text, the binary
serves the data). In the `inline` mode, the session reviews alone.

The evaluation suite measured the review roles on Sonnet
(`CODEV_EVAL_MODEL=sonnet`, 40 cases × 3, compared with the baseline on
Opus):

| Role | Recall | False positives (baseline max → Sonnet) | Cost |
|---|---|---|---|
| tracer | 1.00 | 1 → 0 | −59 % |
| skeptic | 1.00 | 6 → 6 | −46 % |
| newcomer | 1.00 | 0 → 0 | −47 % |
| security | 1.00 | 3 → 1 | −51 % |
| performance | 1.00 | 0 → 0 | −48 % |
| migration | 1.00 | 2 → 1 | −48 % |
| accessibility | 1.00 | 0 → **1** | −50 % |
| ops | 1.00 | 1 → **2** | −48 % |
| privacy | 1.00 | 2 → **3** | −49 % |

All review roles together: 12.89 → 6.60 USD (−49 %). Evidence validity
stays 1.00 everywhere. Planning on Sonnet: primary recall 0.62 against
0.82 on Opus.

## Goals / Non-Goals

**Goals**: each reviewer on the cheapest model the suite measured
unchanged for it; a team free to choose otherwise; nothing changed for
planning and implementation.

**Non-goals**: a model for the session's own steps (planning,
implementation), which Claude Code sets per session, not per role;
implementation on Sonnet, which needs an evaluation of implementation
first; tightening the three experts' criteria so they hold on Sonnet, a
change of its own.

## Decisions

### Decision: The model is a property of the role, chosen by the project, with no default

A role may carry `model:`, set by the project under `roles:`, next to
`when:` and `done_when:`; `inherit` means none. No built-in role sets one.
The measure on the isolated path (below) decided it: the saving a Sonnet
default brings there is 7 %, under decision 0008's cost floor, and two
roles raise one false positive more than the reference. The mechanism
ships so that a team can choose — a cheaper model for a role it trusts on
it, a stronger one for another — and the suite can measure any choice.
Four values only: Claude Code's sub-agent tool accepts these aliases, and
a full model id would tie the configuration to a release.

**Alternatives considered**: Sonnet by default for the six roles measured
unchanged in single-session runs — the isolated measure shows a small
saving and a quality slip. Not shipping the mechanism — a team that wants
a model per role would have no way to say so.

### Decision: The model applies to sub-agents, in the isolated mode

`codev roles --json` serves `model`. The isolated review procedure starts
each role's sub-agent with it, or without a model when it is `null`. The
`inline` mode keeps every role in the session: a session cannot switch
models for a step, and decision 0007 keeps orchestration in the skills.
The summarizing sub-agents are unchanged.

**Alternatives considered**: running a role with a model as a sub-agent
even in the inline mode — it would change what `inline` means, and its
cost, without a measure.

### Decision: Measured on the isolated path, after the suite covers it

`isolated-review-evaluation` (#50) made the suite play the review cases in
the isolated mode. Two runs the same day, 40 review cases × 3, each in its
own working folder:

| | Reference: every role on the session's model (Opus) | Candidate: Sonnet for tracer, skeptic, newcomer, security, performance, migration |
|---|---|---|
| Cost | 22.34 USD | 20.76 USD (−7 %) |
| Recall | 1.00 for every role | 1.00 for every role |
| False positives over the reference's maximum | — | `migration` 1 (max 0), `skeptic` 5 (max 4) |
| Format fallbacks | 0 | 2 runs where the Sonnet sub-agent broke the finding format twice |

In the isolated mode most of the cost is the session's — reading the
skill, building the brief, consolidating, writing and checking the
record — so a reviewer's model moves the bill little. The measure also
shows the isolated mode costing 73 % more than the inline mode on the
same cases (12.89 USD inline in the baseline), with the same recall. So
the change ships the mechanism without a default; the cost lever this
measure points to is the review mode, a change of its own.

**Alternatives considered**: shipping on the single-session figures
(−49 % with the whole session on Sonnet) — they measure a different
setup, and 0008 does not allow it.

### Decision: A source's `roles:` is not parsed, and a model off the review steps is flagged

The `roles:` mapping of the project is strict, and an inherited source's
was deserialized the same way although it is ignored. A source written
for a newer codev — a `model:` today, another key tomorrow — would then
fail to load in a project on an older codev, and its context and rules
would be dropped. The source's `roles:` is no longer parsed, only skipped.
A model on a role that never runs as a sub-agent is a warning,
`role_model_unused`: accepted, so that a project is never blocked, and
named, so that nobody believes it applies.

**Alternatives considered**: an error for a model off the review steps —
it would block a project for a setting with no effect.

## Risks / Trade-offs

- [Sonnet misses a defect the suite does not hold] → The defaults cover
  only the roles measured unchanged; a project can set `opus` per role; a
  new case that Sonnet misses is a reason to change a default, measured.
- [An older codev reads a configuration with `model:`] → The `roles:`
  mapping is strict, so an older codev fails on the key. The release
  note says to raise `codev_version` before using it, and that a shared
  source should not set it until its consumers run this release; from
  this release on, a source's `roles:` is not parsed at all.
- [Inline projects see no saving] → The release note and the roles
  chapter say the model applies in the isolated mode.

## Migration Plan

No file to migrate. `codev update` brings the skill text. A project that
sets `model:` should require the new version in `codev_version`. The
roles document gains `model` and `modelOrigin`, additions installed
skills ignore.
