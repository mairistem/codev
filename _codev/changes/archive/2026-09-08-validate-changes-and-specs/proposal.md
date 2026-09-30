# Proposal: validate changes and specs without false positives

## Why

The `codev-core` parser already produces structural `Finding`s at the
level of an isolated file — missing Purpose, requirement outside a
section, scenario with three hashes, duplicate within a section. What the
tool lacks is checking a change or a spec as a whole, on demand, with a
report readable by a human as well as by an agent. Without it, `sync` and
`archive` cannot know before writing whether they are dealing with a
consistent delta.

## What Changes

- **New command** `codev validate [item]`, with `--all` / `--changes` /
  `--specs` for batch runs, and `--json` for a machine report.
- **Additional structural rules**, pure, run on the parser's AST:
  requirement without `SHALL`/`MUST`, requirement without a scenario, main
  spec without a requirement.
- **Delta consistency**: requirement present in two sections
  (`ADDED`/`MODIFIED`, `MODIFIED`/`REMOVED`, `ADDED`/`REMOVED`), collision
  of `RENAMED.TO` with a same-named `ADDED`, `MODIFIED` referencing an old
  `RENAMED` name.
- **"Zero delta" rule**: a change whose `specs/` folder contains no delta
  fails, unless its `change.yaml` declares `skip_specs: true`;
  conversely, `skip_specs: true` with deltas present is a conflict.
- **Stable output contract**: each `Finding` carries a `code`
  (`requirement_no_shall`, `requirement_no_scenario`, `cross_section_conflict`,
  `zero_delta_without_marker`, `skip_specs_conflict`, …), a
  project-relative `path`, a `line`, a `severity`. Stable codes — a
  consumer can rely on them; messages free to be reworded.
- **Exit code**: `0` if no error, `1` otherwise. The warnings of batch 2
  will not change this code.

## Capabilities

### New Capabilities

- `validation`

### Modified Capabilities

- `spec-parsing` — not modified. The additional rules live on top of the
  parser; the contract of the `Finding`s already emitted does not change.

## Impact

- **Code**: new module `codev-core::validate` for the pure rules
  (input: AST + change metadata; output: `Vec<Finding>`), new
  module `codev-engine::validate` for coordination
  (reading the disk, grouping by file, exit code), new subcommand
  in `codev-cli`, frozen shape `contract::v1::ValidateReport`.
- **Dependencies**: none new — everything relies on what exists.
- **Out of scope**: `--strict` (E5, promotion of warnings to errors),
  merge pre-flight of `MODIFIED` against the main spec (E6),
  `--archived` (E7), bounded parallel validation (E8). These flags will
  arrive when their consumers (archive, pre-commit hook) are implemented.
