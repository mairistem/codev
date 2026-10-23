---
id: "0012"
title: "The model is a property of the role, chosen by the project, with no default; the developer's model implements the tasks"
status: accepted
date: 2026-10-23
supersedes: ["0011"]
---

## Context

Decision 0011 gave each role a model, chosen by the project, applying
where a role runs as its own sub-agent: the reviewers, in the isolated
review mode. The implementation stayed in the session. The
implementation evaluation (`code.implement`, 4 past changes of codev, 5
runs each, 2026-10-23) then measured it. With the whole session on
Sonnet, the quality was identical to Opus, every measure at 1.00, for
5.68 USD against 13.02 USD (−56 %). Planning stays on the strongest model
(decision 0009).

## Decision

A role may carry a model, set by the project under `roles:` in
`_codev/config.yaml` (`model: opus | sonnet | haiku | inherit`). No
built-in role sets one, so every role runs on the session's model until
a project chooses otherwise. A reviewer's model applies where it runs as
its own sub-agent, in the isolated review mode. The `developer`'s model
applies at `code.implement` in either review mode: `/codev-apply` then
hands the tasks to one sub-agent on that model, and the session keeps
the review and the summary. Only the `developer` is dispatched this way.

We recommend `developer: sonnet` to projects. The implementation
evaluation, with an Opus session and the developer on Sonnet (this
change), met the bar set beforehand: the full `codev-evals compare`
verdict passes against the Opus baseline. On the three cases whose tree
accepts the appended configuration, every quality measure is at 1.00, and
the cost is 7.00 against 9.29 USD (−25 %), under the baseline's floor.
On the fourth case, `decision-accept-command`, the code and every
acceptance assertion passed for −45 %. Its last task failed in 3 runs
because its September codev rejects the appended `roles:` key. That is
an artifact of the measure, not of the delegation. Across the four
cases, the saving is about 30 %. It is lower than a whole Sonnet
session's, since the review stays on the session's model.

## Consequences

- A project chooses its implementation's model with one line, and keeps
  its planning and its review on the session's model.
- Nothing changes for a project that sets no model on the developer.
- A model on any other role that serves no reviewing step is still
  reported as `role_model_unused`.
- The run takes longer in wall time (+34 % on the three cases): the
  sub-agent's work adds to the session's.
- `--fixture-config` on an implementation case needs the case's tree to
  accept the appended keys: a tree from before codev tolerated unknown
  keys rejects them in its own checks.

## Alternatives considered

- A built-in default `developer: sonnet`: decision 0011's rule holds. A
  model is a project's choice, and the recommendation says what was
  measured.
- Asking users to switch the session to Sonnet before `/codev-apply`:
  the review and the summary would run on Sonnet too, and the choice
  would not be the project's.
- One sub-agent per task: it would re-read the plan and the code for
  every task.
