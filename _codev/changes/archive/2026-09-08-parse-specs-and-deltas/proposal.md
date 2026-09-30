# Proposal: spec and delta parser

## Why

Today codev can create and list changes, but nothing can read a
hand-written spec or delta. The three batch-1 commands still missing —
`validate`, `sync`, `archive` — all depend on it.

## What Changes

- **New main spec parser**: `## Purpose`, `## Requirements`,
  `### Requirement: <name>`, `#### Scenario: <name>` with its
  **WHEN** / **THEN** / **AND** lines.
- **New delta parser**: `## ADDED Requirements`,
  `## MODIFIED Requirements`, `## REMOVED Requirements` (with `**Reason**` /
  `**Migration**`), `## RENAMED Requirements` (`FROM:` / `TO:`).
- **Masking of code fences and HTML comments** shared by both:
  a ` ``` ` block containing `### Requirement:` must not produce a phantom
  requirement — the classic, silent trap.
- **Span-carrying AST**: each block keeps its `[start, end)` interval in the
  source text. This is the building block that will make the `MODIFIED`
  merge non-destructive: rewrite one block without reformatting the rest of
  the file.
- Pure API in `codev-core`: the function takes a `&str`, returns a typed
  AST, and never touches the disk.

## Capabilities

### New Capabilities

- `spec-parsing`

### Modified Capabilities

None — the project has no main spec yet.

## Impact

- **Code**: new module `codev-core::parser` (`spec.rs`, `delta.rs`,
  `fence.rs`, `ast.rs`). No changes to `codev-engine`, `codev-agents`
  or `codev-cli` in this change — the consumers (`validate`, `sync`,
  `archive`) will each get their own change.
- **Dependencies**: the design will decide between a hand-written
  line-by-line parser (OpenSpec's approach, ~1,200 lines in total) and an
  existing markdown dependency. Nothing is committed here.
- **Out of scope**: the `tasks.md` parser (enabled once `validate --archived`
  is implemented) and the `proposal.md` parser (enabled once proposal
  validation needs it). Today these two artifacts serve the human and the
  agent, not codev, and no batch-1 command requires parsing them.
