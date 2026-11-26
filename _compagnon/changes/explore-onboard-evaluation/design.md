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
five states (`_compagnon/` missing; thin configuration, no change;
configured, no change; one change, planning incomplete; one change,
planning complete). Today, `onboard` names `compagnon init`,
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
scoring: one module `evals/src/read_only.rs` holds them, the step deciding
only the prompt and the case's required fields. Two steps rather than
one, so that `--step` selects each, `compare` gives each its verdict, and
the following change measures the skill it converts.

**Alternatives considered**: folding them into `configure.enrich`'s
module — rejected, configure writes a file and is scored on its shape.

### Decision: The reply is checked by texts the case names, not by an outcome parser

`contains` and `absent` are the only checks on the reply, beside the
files. An explore case names what its outcome must show — for
`crystallizes`, the way to propose (`/compagnon-propose`, which stays the
shortcut after the conversion, so the case holds both now and after); for
`not-worth`, the file that already does it; for `missing`, the thing
missing. `absent` holds what must never be said: an offer to implement
for explore, a fix run for onboard. The outcome is kept in `case.yaml`
for the report and the coverage, not parsed from the reply.

**Alternatives considered**: recognizing the three outcomes from fixed
phrases — rejected, the skill does not fix their wording and the
conversion will change it.

### Decision: Every file counts, `.git` excepted

Both skills are read-only: any file modified, created or removed fails
the run, as does a folder under `_compagnon/changes/` (a change created
by explore). The skills' own folder `.claude/` is committed with the
project, so a skill rewriting itself is caught too.

### Decision: The baseline is recorded once, with the user's agreement

Nine cases, five repetitions, at about 0.3 to 0.5 USD a run: 15 to 25
USD. The task asks the user before running, as decision 0008 and the
previous changes did, and the baseline is committed with the cases.

## Risks / Trade-offs

- [A case's `contains` too narrow, failing a good reply] → each case is
  run once without recording (`run --repeat 1`) and its texts adjusted
  before the paid baseline.
- [Explore asks a question instead of concluding] → the prompt says to
  ask none and to state the assumption, as the configure prompt does.
