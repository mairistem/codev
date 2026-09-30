---
id: 0003
title: The planning root is `_codev/`, visible
status: accepted
date: 2026-09-08
tags: [ergonomics, layout]
---

## Context

The planning root holds the specs, the decisions and the changes. It is the
only place in the repository **meant to be read**: by a human in review, and
by the agent. Three names were in play: `codev/`, `.codev/`, `_codev/`.

## Decision

`_codev/` at the root of the repository.

## Consequences

- Visible to `ripgrep`, `fd`, `ls`, and therefore to Claude Code's search
  tools, which ignore hidden directories by default. This is the deciding
  point: a source of truth the agent cannot find is useless.
- The `_` prefix sets it apart visually from the source code and pins it to
  the top of most listings.
- The name is a **single constant** in `codev-core`. Changing it costs a
  recompilation, not a refactoring.
- Harmless side effect: some static site generators (Jekyll, Hugo) exclude
  `_`-prefixed directories from their output — which is the desired behavior
  here.

## Alternatives considered

- **`.codev/`**, the initial choice. Rejected after finding that search tools
  ignore hidden directories by default: hiding the source of truth from the
  agent contradicts the purpose of the tool.
- **`codev/`**, in the style of OpenSpec. Sound, but it blends in visually
  with the code directories (`crates/`, `docs/`).
