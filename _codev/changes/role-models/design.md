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

### Decision: The model is a property of the role, with a measured default

A role's frontmatter may hold `model:`. The default is `sonnet` for the
six roles whose every measure stays within the baseline's range on
Sonnet, and none for the others, which keeps the session's model. A
project overrides it per role under `roles:`, next to `when:` and
`done_when:`. `inherit` means none, so a project can also turn a default
back to the session's model. Four values only: Claude Code's sub-agent
tool accepts these aliases, and a full model id would tie the
configuration to a release.

**Alternatives considered**: one model for every reviewer — it would put
the three experts above outside the baseline's range. A model per review
step — the experts serve two steps and the measure is per role. Sonnet
for the three experts as well, accepting one false positive more — that
is a deviation from decision 0008, which only a person accepts; the
project can set it in one line.

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

The per-role figures above come from single-session runs: the whole
review, writing and checking the record included, ran on Sonnet. On the
isolated path, the session stays on Opus and only each reviewer's
sub-agent moves, so the saving there is smaller and its quality is a
sub-agent's, not a session's. Decision 0008 asks that this path be
covered first, by its own change: `isolated-review-evaluation` adds a
runner option that plays the review cases in the `isolated` mode. This
change is then measured on that path, as 0008 says for a change aiming at
consumption:

- the reference: the review cases in the `isolated` mode, every role on
  the session's model (`model: inherit` for each), three repetitions;
- the candidate: the same cases in the same mode with this change's
  defaults, three repetitions, the same day;
- the verdict: quality within the reference's range, cost under its
  floor, each call's tokens traced to their source.

The release note states the saving the candidate shows.

**Alternatives considered**: shipping on the single-session figures — they
measure a different setup, and 0008 does not allow it.

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
