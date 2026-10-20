# Proposal: A working folder of its own for every evaluation run

## Why

`codev-evals run` builds its fixtures under a default working folder named
after the current second. Two runs started in the same second — a
reference and a candidate launched together, as for `role-models` on
2026-10-20 — shared that folder and overwrote each other's fixtures: 63
of 120 runs ended in error and the scored ones could not be trusted. The
`role-evaluation` capability already says each case runs "in a new
directory"; the default broke that promise.

## What Changes

- The default working folder also carries the process id, so two runs
  never share it, whatever the second they start in. `--work` is
  unchanged.

## Capabilities

### New Capabilities

### Modified Capabilities

### Removed Capabilities

## Impact

- `evals/src/main.rs` and `evals/src/runner.rs`. `evals/` is not shipped.
- No documentation describes the default folder's name.

## Domains

None

## Release note

None
