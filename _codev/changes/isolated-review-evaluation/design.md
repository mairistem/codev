# Design: Evaluate the reviews in the isolated mode

## Context

A review case's fixture is a git repository with codev initialized and the
skills of the codev under test rendered; its `change.yaml` disables every
role but the evaluated one. The runner runs Claude Code headlessly with
`--allowedTools "Bash(codev:*)"` in a sandbox, and records `run.json`
(model, Claude Code version, repeat, jobs). `compare` refuses runs whose
models differ, asked or used. Claude Code's JSON output reports the
session's cost and tokens, its sub-agents included.

## Goals / Non-Goals

**Goals**: play the existing review cases in the `isolated` mode with no
new case; measure a change of model on that path; keep every guard that
stops comparing unlike runs.

**Non-goals**: a separate baseline of the isolated mode — `role-models`
compares two runs recorded the same day; isolated planning steps
(`plan.trace` is a review case and is covered).

## Decisions

### Decision: The mode and extra settings go into the fixture's configuration

The runner appends `review: { mode: isolated }` and the `--fixture-config`
text to `_codev/config.yaml`, then renders the skills again (`codev update
--force`) so that they grant the sub-agent tool, before committing. The
case is then a project in that mode, as a user's would be; nothing in the
prompt changes. The runner adds `Agent` to `--allowedTools` in that mode,
as the rendered skill grants it.

**Alternatives considered**: a prompt asking for sub-agents — it would
measure the prompt, not the skill. New isolated cases — the same reviews
in another mode need no new expected findings.

### Decision: Runs across models compare only when asked, runs across modes never

`--across-models` lifts the model guard for one comparison and names both
runs' models at the top of the report, so a reader sees the comparison is
across models. A comparison across review modes has no such need: a
change measured on one mode is compared with a reference in the same mode.

**Alternatives considered**: dropping the model guard — it protects every
other comparison from a silent model change.

## Risks / Trade-offs

- [A sub-agent's tokens are not in the session's figures] → Claude Code
  reports the session's cost with its sub-agents; the change checks it on
  one isolated run before any measure, by comparing the session's cost
  with the per-call sum of its transcript.
- [Appended YAML duplicates a key the fixture already has] → The review
  fixtures' configuration, from `codev init`, holds neither `review:` nor
  `roles:`; a duplicate key fails the case's codev commands, which the run
  reports as an error.
