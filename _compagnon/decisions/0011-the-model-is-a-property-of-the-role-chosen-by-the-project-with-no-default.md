---
id: "0011"
title: "The model is a property of the role, chosen by the project, with no default"
status: superseded
date: 2026-10-20
---

## Context

Every review ran on the session's model, and a team had no way to give a
reviewer a cheaper or a stronger one. Measured on the evaluation suite in
the isolated review mode (#50), 40 review cases three times each on
2026-10-20: moving the tracer, the skeptic, the newcomer and the security,
performance and migration experts to Sonnet cost 20.76 USD against
22.34 USD with every role on Opus (−7 %), with recall 1.00 for every role,
but one false positive more than the reference for `migration` and the
skeptic, and two runs where the Sonnet sub-agent broke the finding format
twice. Most of an isolated review's cost is the session's, not the
reviewer's.

## Decision

A role may carry a model, set by the project under `roles:` in
`_codev/config.yaml` (`model: opus | sonnet | haiku | inherit`); no
built-in role sets one, so every role runs on the session's model until a
project chooses otherwise. The model applies where a role runs as its own
sub-agent, in the isolated review mode.

## Consequences

- A team chooses each reviewer's model, and the suite can measure any
  choice with `--fixture-config`.
- Nothing changes for a project that sets no model.
- An older codev refuses `model:` in a role's mapping: a project raises
  `codev_version` before setting it, and a shared source waits until its
  consumers run this release; an inherited source's `roles:` is no longer
  parsed, so the next key added there cannot drop its rules.
- The cost lever this measure points to is the review mode, not the
  reviewer's model: the isolated mode cost about 73 % more than the inline
  mode on the same cases.

## Alternatives considered

- Sonnet by default for the six roles measured unchanged in single-session
  runs: on the isolated path, a 7 % saving, under decision 0008's floor,
  and two roles slipping.
- Not shipping the mechanism: a team that wants a model per role would
  have no way to say so.
- A full model id instead of an alias: it would tie the configuration to a
  model release.
