# Proposal: Evaluate `/codev-sync`, `/codev-archive` and `/codev-configure`

## Why

Decision 0008 asks that a change to what codev's agents read be measured
on the steps it touches, and that a step the suite does not cover be
covered first, by its own change. No case runs `/codev-sync`,
`/codev-archive` or `/codev-configure`, and three planned changes now wait
on them: `version-check-stops-verdict-actions` (the update check's
**Later** answer that leads sync and archive into a refusal, finding
`R-migration-2` of `forward-compatibility-policy`), the relay of the
unchecked-task warning by `/codev-archive` (left out of `task-progress`),
and `lessons-from-reviews`, which adds an invitation line to
`/codev-archive` and a `lessons` mode to `/codev-configure` (finding
`R-skeptic-3`). Each must be measured against a baseline that exists
before it.

## What Changes

- Two new kinds of case replay a past state of this repository with an
  active change laid on it, and run one skill on it: `change.sync`
  (`/codev-sync`) and `change.archive` (`/codev-archive`). A case states
  the outcome expected — the merge done, or a refusal — and the texts the
  agent's messages must and must not hold.
- A third kind, `configure.enrich`, runs the existing default mode of
  `/codev-configure` on a small project fixture, with the author's answer
  to its confirmation question set by the case. It states the outcome
  expected — the configuration written, left as it was, or a refusal —
  and the texts the written `context:` and `rules:` must and must not
  hold.
- `codev-evals run --step change.sync`, `--step change.archive` and
  `--step configure.enrich` replay them with the skills of the codev under
  test, headlessly, as for the other steps. The answer to the update
  check's question is set by the run: **Later**.
- Each run is scored by deterministic checks, without a model. For sync
  and archive, the project as the run leaves it is compared with the one
  `codev sync` or `codev archive` leaves when the runner calls it itself,
  so a merge made by hand or a step skipped is caught. For configure, the
  fields the skill must preserve, its comments, the shape of the proposal
  and the expected texts. The report, `codev-evals compare` and the
  reliability chapter give the measures of the three steps, judged as the
  update step is.
- Ten sync and archive cases and five configure cases, among them an
  archive of a change whose tasks are not all checked, and a sync and an
  archive under a `codev_version` the running codev does not satisfy —
  the baselines the three waiting changes are measured against — and
  five runs of each recorded in the baseline.

## Capabilities

### New Capabilities

- sync-archive-evaluation
- configure-evaluation

### Modified Capabilities

- role-evaluation

### Removed Capabilities

## Impact

- `evals/`: the case format, the runner's fixtures, prompts and
  reference runs for the three steps, the scoring, the report, the
  comparison, the reliability chapter, fifteen cases under
  `evals/cases/`, and `evals/baseline/`.
- No change to codev itself, nor to a skill: `evals/` is not shipped.
- Follow-up, not built here: `evaluate-review-meeting`, described in the
  design, the multi-role review cases `opposed-findings` waits on.
- Documentation: the generated reliability page (`evals/src/reliability.rs`,
  `docs/{en,fr}/src/reliability.md`) and `CONTRIBUTING.md` /
  `CONTRIBUTING.fr.md`.

## Domains

- [ ] security — the cases run in the sandbox the suite already uses; the runner's own call to `codev sync` or `codev archive` runs in a copy of the fixture, with the environment it already gives codev
- [ ] personal-data — the cases hold only this repository's own artifacts and small synthetic projects
- [ ] user-interface — no interface beyond the report's tables
- [ ] performance — the runs cost what the suite already pays per case, and only when their step is selected
- [ ] operations — no log, metric, timeout or deployment changes beyond the runner's existing ones
- [ ] migration — `evals/` is not shipped, and the report gains sections without changing the others

## Release note

None
