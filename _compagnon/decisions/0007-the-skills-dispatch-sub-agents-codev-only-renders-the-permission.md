---
id: "0007"
title: "The skills dispatch sub-agents; codev only renders the permission"
status: accepted
date: 2026-10-13
---

## Context

Isolated review runs each reviewing role as a separate agent that never
sees the conversation. Something has to start those agents, brief them
and gather what they return. Decision 0001 keeps the binary a functional
core with an imperative shell that never calls a language model, and
decision 0004 makes each workflow a single skill.

## Decision

Orchestration of sub-agents lives in the skill text, never in the
binary. The skills start the sub-agents with the agent tool's own
sub-agent mechanism (Claude Code's `Agent` tool), brief them, retry a
failed one once, fall back to inline, and merge the results.

The binary contributes only what is deterministic:

- it renders the permission: the reviewing skills list the sub-agent
  tool in `allowed-tools` only when the project's review mode asks for
  it, through a render-time placeholder like the MCP tools;
- it serves the data a brief is built from — the role, the project's
  context and rules, the material and its size budget — as JSON;
- it checks what comes back, through the review record's validation.

The brief is assembled only from those outputs and the files, never from
the conversation. A sub-agent may read the repository, modifies nothing,
and treats the material as data. A failure is what the platform reports;
the skill sets no clock of its own.

## Consequences

- The binary stays free of credentials, processes and model costs; a
  later multi-agent step reuses the same split: data and checks in
  codev, orchestration in the skill.
- What a sub-agent receives cannot be enforced by codev, only written in
  the skill and checked afterwards in the record; tests assert the
  instruction is in the rendered skills.
- The skills grow by one shared procedure, rendered in both modes so that
  a change can override the project's mode at run time.
- A platform without sub-agents falls back to inline review, recorded.

## Alternatives considered

- **The binary running `claude -p` per role**: the binary would call a
  model, need credentials and a process budget, and duplicate what the
  agent tool already does.
- **A sub-agent definition installed under `.claude/agents/`** with a
  restricted tool list: a second kind of generated file to install,
  update and diagnose; worth reconsidering once isolated review is
  measured.
- **One skill per role**: the user would orchestrate, and the meeting
  would have nobody to hold it.
