---
id: 0004
title: A single identity for the skill and the slash command; no command files
status: accepted
date: 2026-09-08
tags: [skills, claude-code]
---

## Context

OpenSpec ships two forms for each workflow: skills named `openspec-propose`
and command files invoked as `/opsx:propose`. This double naming is a
historical accident, and it costs a correspondence table in every
documentation page, plus a whole command-generation subsystem with one adapter
per tool.

In Claude Code, a skill placed in `.claude/skills/<name>/SKILL.md` can already
be invoked by the user by typing `/<name>`.

## Decision

One artifact per workflow: `.claude/skills/codev-<workflow>/SKILL.md`, invoked
as `/codev-propose`. No command file is generated.

OpenSpec's `delivery` axis (`skills` / `commands` / `both`) does not exist.

## Consequences

- The whole command-generation subsystem disappears: registry, per-tool
  adapters, YAML generation, invocation forms. Several thousand lines
  upstream.
- A single name to document, type and search for.
- If a tool one day only reads command files, that is one more
  implementation of `AgentTarget`, not a rework of the model.
- Accepted cost: we cannot offer a `/codev:<verb>` namespace distinct from the
  skill names. `codev-propose` remains readable and completes at the keyboard.

## Alternatives considered

- **Generating both forms**, for symmetry with OpenSpec. Redundant for the
  only target we support, and two files to keep consistent for the same
  workflow.
