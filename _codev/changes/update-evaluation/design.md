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
(this change records it); covering `sync`, `archive` or `apply` outside
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
final reply, `codev validate`, and the modified-outside-the-step rule. No
model judges the revision: a check a reader can redo by hand is what lets
`compare` tell a regression from noise. What the checks cannot see,
whether the new prose is good, is left to the author's review of the
skill, as for planning.

**Alternatives considered**: a model as judge — costly, itself noisy, and
a second source of drift to calibrate. Scoring only validity — it would
pass a run that edits the ripple instead of reporting it, which is the
skill's main guardrail.

### Decision: The reply is the agent's last message

Claude Code's JSON output carries the final `result`. The ripple check
looks for each `ripple` path, relative to the change's folder, in it. The
prompt asks the agent to end with what was revised and the ripple found,
which the skill's summary step already does.

**Alternatives considered**: reading the whole transcript — a path read
in passing would count as reported.

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

### Decision: The baseline gains the update runs, and #48 is judged on them

Following decision 0008, this change records the update runs of the
current `main` in `evals/baseline/`, three repetitions per case. Then
`capture-corrections` (#48), which adds text to `/codev-update`, is run on
the update cases and compared with `--quality-only`, the same way as its
planning and review steps.

## Risks / Trade-offs

- [Six cases are few, so one flaky run moves the pass rate by a sixth] →
  Three repetitions, and `compare` judges against the baseline's minimum,
  as for planning.
- [The ripple check misses a ripple named in other words, such as "the
  doctor spec"] → The prompt asks for paths; the check is documented, and
  a miss shows in the per-run report for a reader to see.
- [A case's request leaves room for two valid revisions] → The checks ask
  only what any valid revision must do: change the target, leave the
  rest, report the ripple.
