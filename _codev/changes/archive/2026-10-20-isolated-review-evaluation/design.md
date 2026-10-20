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
--force`) before committing. The case is then a project in that mode, as a
user's would be; nothing in the prompt changes. The evaluation's agent
reads the skill file rather than invoking the skill, so the runner grants
the sub-agent tool itself in that mode, as it grants `codev`.

The skill decides the mode from `codev roles` and falls back to inline
when a sub-agent cannot run (decision 0007), recording it. So the runner
checks after the run that the role really ran alone — `Mode: isolated.`,
no `Fallback:` line, at least one sub-agent started — and fails the case
otherwise, rather than score an inline review as an isolated one.

**Alternatives considered**: a prompt asking for sub-agents — it would
measure the prompt, not the skill. New isolated cases — the same reviews
in another mode need no new expected findings.

### Decision: The sub-agents' tokens are a source of their own

The per-call reading assumes one conversation. In an isolated run, the
session's totals include the sub-agents, while its own calls do not. The
runner counts the session's own calls as before and reports what the
totals hold beyond them as the source `sub-agents`, so each token stays
traced to a source, as decision 0008 asks, without parsing interleaved
sub-agent events. A run whose totals fall below its own calls is
inconsistent and fails.

**Alternatives considered**: splitting the sub-agents' calls by
`parent_tool_use_id` — finer, but it depends on whether and how Claude
Code streams them, which no observed run shows yet.

### Decision: Runs across models compare only when asked, runs across modes never

`--across-models` lifts the model guard for one comparison and names both
runs' models at the top of the report, so a reader sees the comparison is
across models. A comparison across review modes has no such need: a
change measured on one mode is compared with a reference in the same mode.

**Alternatives considered**: dropping the model guard — it protects every
other comparison from a silent model change.

## Risks / Trade-offs

- [A sub-agent's tokens are not in the session's totals] → The runner
  fails a run whose totals fall below its own calls, and task 1.2 checks on
  a real run that the `sub-agents` source is above zero when a sub-agent
  ran.
- [An isolated case runs inline after the skill's fallback, unnoticed] →
  The runner checks the record and the sub-agent calls after the run, and
  fails the case.
- [Two runs differ by their appended configuration] → `compare` names both
  and says when they differ.
- [Appended YAML duplicates a key the fixture already has] → The review
  fixtures' configuration, from `codev init`, holds neither `review:` nor
  `roles:`; a duplicate key fails the case's codev commands, which the run
  reports as an error.
