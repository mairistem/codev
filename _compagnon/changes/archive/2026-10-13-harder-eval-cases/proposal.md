# Proposal: Harder evaluation cases, matched by category

## Why

The first baseline of the role evaluation suite cannot tell a better role
from a worse one: every role found every seeded defect (recall 1.00
everywhere), about nine of its false positives came from fixtures whose
plans cite files and tests the fixture does not contain, and a finding is
paired with an expected defect by location alone, so it can pair with the
wrong defect two lines away (findings R-skeptic-4, R-skeptic-5 and
R-skeptic-6 of the archived change `role-evaluation-suite`). Before any
role is tuned against it (`role-precision-from-baseline`), the measure
itself must discriminate.

## What Changes

- **Fixture lint.** A case is refused when its planning artifacts cite a
  file, a directory or a test the fixture does not contain, unless the
  case declares it as `planned` — something a plan step introduces; a
  `code.review` case, whose tasks are all done, may declare nothing
  `planned`. Every existing case is completed so that it passes.
- **Matching by category.** An expected finding names a `category` from a
  vocabulary kept with the suite; a finding matches it only when the
  role, the location and the category agree — the category is recognised
  from the finding's title and recommendation. An expected finding may
  list `alternatives`, other places a reviewer may rightly cite the same
  defect. Expected findings and findings are paired so that the most
  defects are found, whatever the order of the record. A finding at an
  expected location but naming another issue is reported as such.
- **Harder seeded cases**: subtle defects — authorization checked on the
  list endpoint but not on the detail endpoint, personal data reaching
  the logs through an error message, an N+1 query hidden behind a
  helper, a keyboard trap in a custom dropdown, a serialized field name
  changed by an attribute, retries without backoff — placed among
  plausible code that is fine; for each expert, a case holding two
  defects of different severity; Python cases for security and privacy.
- **More clean controls**: realistic code that looks suspicious but is
  correct — a query built dynamically from an allow-list with bound
  values, logs of identifiers that are not personal data, a custom
  listbox that handles the keyboard, a helper called in a loop over data
  loaded in one query, a renamed field that keeps its old name readable.
- **Coverage raised**: per expert, at least three seeded cases, two
  controls and one case with two defects of different severity; security
  and privacy each have a Python case.
- **A new baseline**: the whole suite run again on Claude Code's default
  model, replacing `evals/baseline/`; the design records each role's
  scores before and after. No role file changes.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `role-evaluation`

### Removed Capabilities

None.

## Impact

- `evals/src/` (case loading, fixture lint, category vocabulary,
  matching, report), `evals/categories.yaml` (new),
  `evals/cases/` (existing cases completed, new cases),
  `evals/baseline/` (recorded again), `evals/tests/`.
- `CONTRIBUTING.md`, `CONTRIBUTING.fr.md` (how to write a case: the
  vocabulary, `planned`, `alternatives`); `docs/en/src/roles.md`,
  `docs/fr/src/roles.md` (matching by category).
- A full run costs about the previous run per case; it is never part of
  regular CI.
- The roles, the skills and the `codev` binary are unchanged.

## Domains

- [ ] security — the security fixtures hold deliberately vulnerable code that is never run; the runner, its permissions and the workflow's secret handling are unchanged
- [ ] personal-data — the privacy fixtures hold fictitious personal-data fields only; nothing is stored or sent
- [ ] user-interface — the accessibility fixtures are evaluation inputs, not an interface of codev
- [ ] performance — the performance fixtures are evaluation inputs; a run stays bounded per case by its budget and timeout
- [ ] operations — the workflow is unchanged, and its 240-minute job timeout still covers the larger suite's worst case
- [ ] migration — `case.yaml` is internal to the repository and loads with the runner of the same commit; `report.json` only gains fields; the baseline is recorded again in the same format

## Release note

- Changed: The role evaluation suite has harder cases — subtle defects,
  two defects of different severity per expert, Python cases and more
  clean controls — and pairs a finding with a seeded defect only when
  they name the same category of issue. The committed baseline in
  `evals/baseline/report.md` is recorded again.
