# Proposal: Explicit roles, traceability check and contrarian pass in /codev-propose

## Why

`/codev-propose` writes the four artifacts in sequence and presents them:
the roles behind each artifact stay implicit, nobody challenges the plan, and
nothing checks that requirements, scenarios and tasks cover each other. The
human approval step has to find the weak points alone.

## What Changes

- Each artifact instruction of the `spec-driven` schema starts with a role
  block: the question the role owns and checkable "Done when" criteria —
  product owner for `proposal`, QA analyst for `specs`, architect for
  `design`, tech lead for `tasks`. Existing instruction content is kept.
  A custom schema defines its own roles in its own instructions.
- `/codev-propose` gains a traceability check once the artifacts are
  complete: capabilities against spec files, a nominal and an error or edge
  scenario per requirement, every scenario covered by a task's verification,
  every verification concrete. Gaps are fixed in the artifacts.
- `/codev-propose` gains a contrarian pass after it: a fixed six-point grid
  (need, scope, specs, decisions, assumptions, tasks), conditional lenses
  applied only when the proposal's Impact touches them (security & privacy,
  compatibility & migration, operability, performance, accessibility & UX),
  plus the project's `rules:`. It fixes what is unambiguous and records what
  needs a human decision; it never adds scope.
- The final summary of `/codev-propose` ends with "Points to challenge": at
  most five one-line items ranked by impact, or "No point to challenge
  found".
- The documentation presents `rules:` as the way to add project review
  lenses.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `skills` — the default schema's role blocks, the traceability check and
  the contrarian pass of the `propose` skill, and its "Points to challenge"
  output.

### Removed Capabilities

None.

## Impact

- `assets/schemas/spec-driven/schema.yaml` (instructions of the four
  artifacts) and `assets/workflows/propose.md` (two new steps, output,
  guardrails); regenerated `.claude/skills/codev-propose/SKILL.md`.
- Tests asserting on instruction or skill text in `codev-engine` and
  `codev-agents`.
- Documentation in English and French: skills, workflow, configuration
  reference, concepts; CHANGELOG.
- No change to the CLI, the JSON contract or the file formats. Projects with
  a custom schema keep their own instructions unchanged.
