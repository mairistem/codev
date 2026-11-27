# Design: The baseline records the Compagnon version of each step

## Context

`RunInfo` (`evals/src/observed.rs`) is `run.json`: format, model, date,
`claudeVersion`, `compagnonVersion`, repeat, jobs, review mode, fixture
configuration. `main.rs` builds it from `runner::preflight` and, on a
resumed run, checks the model and the Claude Code version against the
recorded one (`runner::check_resume`), then `runner::ensure_run_info`
writes `run.json` only when the directory has none: a resumed run keeps
the original fields, whatever Compagnon it ran. `reliability.rs` prints
the run block from `b.run` with `version(raw, name)`, which trims
"compagnon " and "(Claude Code)" from the recorded strings.

The committed baseline's `run.json` records "compagnon 0.6.1" (2026-10-15).
The `idea.explore` and `project.onboard` observations were added to it on
2026-11-26 by a run built from the branch, whose `preflight` reported
"compagnon 0.7.0" (`_compagnon/changes/archive/2026-11-26-explore-onboard-evaluation/evals.md`).

## Goals / Non-Goals

**Goals:**

- `run.json` says which Compagnon each step ran with, without changing
  what it records of the first run.
- The page says it too.

**Non-Goals:**

- Re-recording any case.
- Recording the Claude Code version per step: a resumed run already
  refuses another Claude Code (`check_resume`).

## Decisions

### Decision: An `added` list in `run.json`, appended by a resumed run

`RunInfo` gains `added: Vec<AddedSteps>` (`serde(default, skip_serializing_if
= "Vec::is_empty")`), each `{ steps: [..], compagnonVersion, date }`, the
steps in the order `Step` lists them. On a resume whose `compagnon_version`
differs from the recorded one, `main.rs` appends one entry with the steps
the run selects — those of `--step`, or every step of the cases it runs —
and rewrites `run.json` with the recorded fields unchanged. A resume with
the same Compagnon appends nothing. The `format` of `run.json` stays 2:
the field is optional and read as empty when absent.

**Alternatives considered**: a version in each `observed.json` — rejected,
it would take a new observation format and re-reading every observation
for one line of the page.

### Decision: The page names the added steps after the run's version

The run block's Compagnon line becomes, when `added` is not empty,
"compagnon: 0.6.1 (`idea.explore`, `project.onboard`: 0.7.0, 2026-11-26)"
in English and the same with « : » in French, each entry in order.

### Decision: The committed baseline gets its entry by hand, once

`evals/baseline/run.json` gains
`added: [{steps: ["idea.explore", "project.onboard"], compagnonVersion:
"compagnon 0.7.0", date: "2026-11-26"}]`, the values of the archived
`explore-onboard-evaluation` evaluation report; the chapter is
regenerated. `the_baseline_agrees_with_the_scoring` and
`the_reliability_pages_follow_the_baseline` keep holding.

## Risks / Trade-offs

- [A resumed run that selects steps already in `added` again] → a new
  entry is appended with its date: the page shows both, the later last.
