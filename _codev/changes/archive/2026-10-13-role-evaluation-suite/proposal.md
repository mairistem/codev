# Proposal: Measure what each role finds and what it invents

## Why

The reviewing roles — the skeptic, the newcomer, the tracer and the six
experts — are prompts, and nobody knows how often they catch a real
defect, how often they invent one, or whether the evidence they quote is
really in the code. A change to a role file is merged on a reading of the
text alone. Now that findings land in a review record that codev parses
(`review-record`) and experts are activated by the diff
(`expert-triggers-and-stack-filter`), each role's output can be scored
against defects seeded on purpose, so that a role change shows its effect
in numbers before it is merged.

## What Changes

- An **evaluation suite** in `evals/` at the repository root: cases, each
  a small fixture project with a planned codev change — the plan, or the
  plan and its code — holding **seeded defects** and the **expected
  findings** (role, severity, location, category), plus **clean
  controls** with no defect, to count what a role invents. At least two
  seeded cases and one control per expert, two seeded cases and one
  control for the newcomer and the skeptic, one seeded case and one
  control for the tracer; Rust, TypeScript and Python fixtures.
- A **runner**, `codev-evals`, a separate crate of the workspace that is
  never shipped: for each case it builds the fixture in a fresh git
  repository, installs codev's skills, turns off the reviewing roles not
  under evaluation, runs the case's step — `plan.challenge`,
  `code.review` or `plan.trace` — headlessly with Claude Code
  (`claude -p`, model configurable), and keeps what it observed: the
  review record as `codev review --json` reads it, and the files the case
  checks.
- A **deterministic scoring core**, unit-tested on recorded outputs
  without any model: per role, true positives, misses, false positives
  (findings matching no seeded defect, controls included), evidence
  validity (from codev's own evidence check), severity agreement; a
  report in markdown and JSON; an exit code that fails under configurable
  thresholds. Recorded runs can be scored again, and an interrupted run
  resumed.
- A **baseline**: the suite run once against the current roles, committed
  as `evals/baseline/` (report and observations), the reference a role
  change is compared with.
- A GitHub workflow `evals.yml`, started by hand only, that runs the
  suite with an `ANTHROPIC_API_KEY` secret and skips with an explanation
  when the secret is absent.
- `CONTRIBUTING.md` and `CONTRIBUTING.fr.md`: a change to a role file runs
  the evaluations and pastes the scores before and after in its pull
  request. The Roles chapter, in both languages, says how the built-in
  roles are measured and where the baseline is.

## Capabilities

### New Capabilities

- `role-evaluation` — the cases and their format, the runner, the
  scoring rules, the report and thresholds, the baseline, the workflow and
  the contributor rule.

### Modified Capabilities

None.

### Removed Capabilities

None.

## Impact

- New crate `evals/` (`codev-evals`, `publish = false`), a member of the
  workspace so that `cargo test --workspace` runs its scoring tests;
  `release.yml` builds `--bin codev` only, so nothing of it ships.
  Dependencies already in the workspace (`serde`, `serde_json`,
  `serde_norway`, `clap`); no new crate in `Cargo.lock`.
- `evals/cases/` (fixtures), `evals/baseline/` (report and observations).
- `.github/workflows/evals.yml`.
- `CONTRIBUTING.md`, `CONTRIBUTING.fr.md`; `docs/en/src/roles.md`,
  `docs/fr/src/roles.md`.
- Running the suite calls a model and costs money: it never runs in the
  regular CI, only on demand.
- The roles, the skills and the `codev` binary are unchanged.

## Domains

- [x] security — the runner gives an agent write access to fixture copies, and the workflow handles an API key secret
- [ ] personal-data — the privacy fixtures hold fictitious personal-data fields only, nothing is stored or sent
- [ ] user-interface — the accessibility fixtures are evaluation inputs, not an interface of codev
- [ ] performance — the performance fixtures are evaluation inputs; the suite's cost is bounded per case
- [x] operations — a new CI workflow that spends money, with timeouts and budgets per case
- [ ] migration — additive: a new unshipped crate, no format, contract or configuration of codev changes

## Release note

- Added: The built-in roles are measured by an evaluation suite in the
  repository: seeded defects and clean controls score what each role
  finds, misses and invents, and whether its evidence is real. The
  baseline is published in `evals/baseline/report.md`, and the Roles chapter
  explains it.
