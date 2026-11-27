# Design: The evaluation suite covers explore and onboard

## Context

The configure step (`configure.enrich`) is the closest model: a case is a
small project in `base/` plus `case.yaml`; `runner::setup_configure`
(`evals/src/runner.rs`) makes the repository and installs the skills;
the prompt names the skill and the evaluation mode; the observation holds
the agent's messages and the files touched; `evals/src/configure.rs`
scores it without a model; `report.rs`, `compare.rs` and `reliability.rs`
carry its measures. The step is wired through `Step::ConfigureEnrich` in
`case.rs`, `observed.rs`, `runner.rs`, `score.rs`, `report.rs` and
`reliability.rs`, with its coverage checked in `evals/tests/suite.rs`.

`explore` (`assets/workflows/explore.md`) writes no file (its
`allowed-tools` have no `Write` nor `Edit`) and ends by saying one of
three outcomes. `onboard` (`assets/workflows/onboard.md`) runs only
`compagnon` read commands and recommends the next action from a table of
six states (`_compagnon/` missing; thin configuration, no change;
configured, no change; one change, planning incomplete; one change,
planning complete; several active changes, listed for the user to
choose). Neither includes the update-check fragment
(`VERSION_CHECK_WORKFLOWS` in `crates/compagnon-agents/src/workflows.rs`
lists propose, apply, update, sync and archive only). Today, `onboard` names `compagnon init`,
`/compagnon-configure`, `/compagnon-propose` and `/compagnon-apply`;
`explore` names `/compagnon-propose`. The baseline's configure runs cost
about 0.28 USD each (median of 25).

## Goals / Non-Goals

**Goals:**

- A measure of each skill as it is, before its text changes.
- Checks that hold whatever wording the skill uses, now or after the
  conversion to plain words.

**Non-Goals:**

- Changing either skill's text or description (the following change).
- Judging the quality of an exploration by a model: the checks stay
  deterministic, as everywhere in the suite.

## Decisions

### Decision: One read-only family, two steps

Both steps share the case format, the setup, the observation and the
scoring. As for configure, the format lives in `case.rs` (the steps, the
fields and their loading errors), the setup, the prompt and the
observation in `runner.rs` and `observed.rs`, the selection in `case.rs`
and `main.rs`; `evals/src/read_only.rs` holds the scoring and the
measures, which `score.rs`, `report.rs`, `compare.rs` and
`reliability.rs` carry. Two steps rather than
one, so that `--step` selects each, `compare` gives each its verdict, and
the following change measures the skill it converts.

**Alternatives considered**: folding them into `configure.enrich`'s
module — rejected, configure writes a file and is scored on its shape.

### Decision: The final message is checked, for its own outcome and against the others

`contains` and `absent` are the only checks on the reply, beside the
files, and they read the agent's final message only: the narration
between tool calls names files, commands and the doctor's fix lines,
which would let a wrong conclusion pass. Each case holds both lists: what
its outcome must show — for `crystallizes`, the way to propose
(`/compagnon-propose`, which stays the shortcut after the conversion); for
`not-worth`, the file that already does it; for `missing`, the thing
missing; for an onboard state, its recommendation — and, in `absent`, the
marks of the other outcomes or states, so that a reply concluding on the
wrong one fails. The outcome and the state are kept in `case.yaml` for
the report and the coverage, not parsed from the reply.

### Decision: The evaluation mode lets the skill end as it would

A headless run has no user to answer. The prompt says so and asks the
agent to end, where the skill would ask, as the skill says — not to
assume, which would push `explore` away from its `missing` outcome. It
says nothing of updates: neither skill checks the version.

**Alternatives considered**: recognizing the three outcomes from fixed
phrases — rejected, the skill does not fix their wording and the
conversion will change it.

### Decision: Every file counts, `.git` excepted

Both skills are read-only: any file modified, created or removed fails
the run, as does a folder under `_compagnon/changes/` (a change created
by explore). The skills' own folder `.claude/` is committed with the
project, so a skill rewriting itself is caught too.

### Decision: The baseline is recorded once, with the user's agreement

Nine cases (three explore, six onboard), five repetitions, at about 0.3
to 0.5 USD a run: 15 to 25 USD, after a trial run of each case once,
about 3 USD. The task asks the user before running, as decision 0008 and the
previous changes did, and the baseline is committed with the cases.

## Risks / Trade-offs

- [A case's `contains` too narrow, failing a good reply] → each case is
  run once without recording (`run --repeat 1`) and its texts adjusted
  before the paid baseline.
- [Explore asks a question instead of concluding] → the prompt says the
  user cannot answer and to end as the skill says; the trial run shows
  it, and a final message that asks counts as no conclusion: its
  `contains` fails.
