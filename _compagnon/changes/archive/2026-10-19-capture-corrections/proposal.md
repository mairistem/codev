# Proposal: Capture the author's corrections in the change

## Why

compagnon is meant to spare the team from repeating how things are done here.
But when the author corrects the agent during `/compagnon-propose`,
`/compagnon-apply` or `/compagnon-update` ("no, here we do it this way"), the
instruction is written nowhere in the repository. It gets repeated at the
next change, and no later step can learn from it. This change is the first
half of a lessons loop. It records these corrections where the team, and a
later `/compagnon-configure` mode, can read them.

## What Changes

- While `/compagnon-propose`, `/compagnon-apply` and `/compagnon-update` run, when the
  author corrects the agent on a way of working specific to the project
  (a convention, a technical choice, an expectation on an artifact), the
  agent appends one entry to the change's `corrections.md`. The entry says
  where, what the agent did, and what the author asked, in one or two
  lines. It is not a transcript.
- The skills say what counts as a correction: a reusable instruction about
  the project. A one-off preference about this change, a typo, a change of
  scope, or a personal preference does not count. They also say what goes
  into compagnon, as project knowledge shared through git, and what stays in
  Claude Code's personal memory, so that one instruction is never written
  in both places.
- `corrections.md` gets a versioned format that the binary reads.
  `compagnon validate` checks it in active changes, and `compagnon review` lists
  its entries. `compagnon status`, `compagnon sync` and `compagnon changelog` ignore it.
  `compagnon archive` moves it with the change.
- The second change, which turns review findings and corrections into
  durable knowledge through a `/compagnon-configure` mode and runs a one-time
  catch-up on this repository, is described in the design and planned
  separately.

## Capabilities

### New Capabilities

- corrections

### Modified Capabilities

- skills
- validation
- review-record
- planning-evaluation
- review-mode

### Removed Capabilities

## Impact

- `compagnon-core`: a parser for `corrections.md` and its checks.
- `compagnon-engine`: validation of active changes, and the corrections in
  `compagnon review`.
- `compagnon-cli`: the corrections in `compagnon review`, human output and JSON
  contract (an addition).
- `assets/workflows/`: a shared fragment for capturing corrections,
  rendered into the `propose`, `apply` and `update` skills.
- Documentation in `docs/en` and `docs/fr`: the skills chapter, the
  reference for `corrections.md`, and the validation codes.
- Evaluation: `compagnon-evals` runs of `plan.write` and of the review steps,
  compared with `compagnon-evals compare`.

## Domains

- [ ] security — no new input reaches a command or the shell; the file is written by the agent with tools it already has
- [x] personal-data — an entry states an instruction, never a person, but git ties each commit of `corrections.md` to its author; the purpose is project knowledge shared within the team
- [ ] user-interface — no interface beyond the CLI's text output, whose layout follows the existing sections
- [x] performance
- [ ] operations — no log, metric, timeout or deployment changes
- [x] migration

## Release note

- Added: When you correct the agent on how your project does things during `/compagnon-propose`, `/compagnon-apply` or `/compagnon-update`, it notes the correction in the change's `corrections.md`, which `compagnon validate` checks and `compagnon review` lists, so that the instruction is not lost when the change is archived.
