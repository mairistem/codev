# Design: Evaluate the `/codev-update` skill

## Context

`codev-evals` replays two kinds of cases: review cases, whose fixture is
a small `base/` with a `change/` laid on it, and planning cases, whose
fixture is this repository at a past commit (`git archive <ref>`). Both
install the skills of the codev under test (`codev update --force`) and
run Claude Code headlessly in the sandbox, then score the observation
without a model. Planning cases are scored by codev's own parser.

`/codev-update` revises one artifact of an **active** change. This
repository squash-merges each pull request with its archive commit, so
no commit of `main` holds an active change. A case cannot point at one.

## Goals / Non-Goals

**Goals**: cover `/codev-update` as decision 0008 requires, with checks a
reader can verify by hand, in the same runner, report and comparison as
the other steps.

**Non-goals**: judging the prose of a revision (no model as judge);
measuring `/codev-update`'s cost against a floor before a baseline exists
(this change records it); judging `capture-corrections` (#48), which does
that in its own pull request; covering `sync`, `archive` or `apply` outside
`code.review`.

## Decisions

### Decision: A case carries its change folder, laid on the state before the change

The case holds `change/`, the files of `_codev/changes/<change>/` as the
archive keeps them: the proposal, the delta specs, the design and the
tasks as the author approved them. They are laid on `ref`, the commit
before the change was merged, where the main specs are those the deltas
were written against. The fixture is then the repository as it stood
while the change was active.

**Alternatives considered**: pointing at a commit where the change is
active — none exists on `main` (squash merges). Rebuilding the change from
its pull request's branch — branches are deleted after merge.

### Decision: Deterministic checks, like the planning scorer

A run is scored by file comparisons, a substring search in the agent's
messages, `codev validate`, and the modified-outside-the-step rule. No
model judges the revision: a check a reader can redo by hand is what lets
`compare` tell a regression from noise. What the checks cannot see,
whether the new prose is good, is left to the author's review of the
skill, as for planning.

**Alternatives considered**: a model as judge — costly, itself noisy, and
a second source of drift to calibrate. Scoring only validity — it would
pass a run that edits the ripple instead of reporting it, which is the
skill's main guardrail.

### Decision: The ripple is looked for in every message the agent wrote

The skill reports the ripple at its own step, before it runs `codev
validate`, and its summary lists the files touched, the verdict and the
next action, not the ripple. So the check reads the text of every message
the agent wrote, from the transcript, and looks for each `ripple` path,
relative to the change's folder, in it. Tool calls and their outputs are
left out, so a path read in passing does not count. The prompt does not
ask for a summary of the ripple: the check measures what the skill makes
the agent do.

**Alternatives considered**: the final `result` alone — it holds the
summary, where the skill does not put the ripple; asking for the ripple
in the final reply — it would measure the prompt, not the skill.

### Decision: Validity counts only the errors the run adds

The archived changes were written by earlier codevs. Laid on `ref`, one
may already have a validation error under the codev being measured, for
example a review record without a section for a role active today. So the
runner validates the fixture before the run, and the check fails only on
an error that was not there before.

**Alternatives considered**: requiring no error at all — a case would
fail whatever the agent does. Editing the archived files until they
validate — the case would no longer be the change as it was.

### Decision: Six cases from archived changes

| Case | From | Artifact | What it checks |
|---|---|---|---|
| `update-proposal-ripple` | `forward-compatible-config` | proposal | dropping a capability: the proposal is revised, its delta spec is named as orphaned and left alone |
| `update-specs-scenario` | `docs-and-tests-in-done` | specs | a scenario added to a delta, which still validates |
| `update-design-decision` | `newcomer-readability-review` | design | a decision replaced, the other artifacts untouched |
| `update-tasks-add` | `fewer-review-turns` | tasks | a task added, the plan untouched |
| `update-specs-ripple-tasks` | `isolated-review` | specs | a requirement removed whose tasks cite it: the tasks are named, not edited |
| `update-refuse-missing` | `specify-core-commands` | design | a design the change does not have: refused, pointing to `/codev-propose` |

Each `ref` is the parent of the commit that brought the change to `main`.

### Decision: The baseline gains the update runs alone, five of each

The baseline holds five runs of every case, recorded with the Claude Code
version `.github/workflows/evals.yml` pins (2.1.289), with codev 0.6.1.
This change adds the update cases and leaves the existing cases and their
scoring untouched. So it records only the update cases, five repetitions
each, with the same Claude Code and the same codev, into `evals/baseline/`.
It then scores the baseline again, committing `report.json` and
`report.md`. CONTRIBUTING says when that is enough: a new step's cases,
recorded with the pinned version, while changed cases or a changed scoring
still require a full recording.

`capture-corrections` (#48) is then measured on the update cases in its
own pull request, as decision 0008 puts the comparison in the change that
touches the workflow.

**Alternatives considered**: recording the whole baseline again — about
270 dollars for runs whose inputs did not change. Three repetitions — the
suite's tests require five, and the medians would be thinner.

## Risks / Trade-offs

- [Six cases are few, so one flaky run moves the pass rate by a sixth] →
  Three repetitions, and `compare` judges against the baseline's minimum,
  as for planning.
- [The ripple check misses a ripple named in other words, such as "the
  doctor spec"] → The skill names files, the check is documented, and a
  miss shows in the per-run report for a reader to see.
- [A case's request leaves room for two valid revisions] → The checks ask
  only what any valid revision must do: change the target, leave the
  rest, report the ripple.
