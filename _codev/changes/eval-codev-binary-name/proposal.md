# Proposal: The runner refuses a codev binary that is not named codev

## Why

`codev-evals run --codev <binary>` puts the binary's folder first in the
search path, and the agent calls `codev` by name. A binary frozen under
another name, as `ie-bin` was for the implementation baseline of
2026-10-20, is never found: the agent runs whatever other `codev` the
operator has installed — here the 0.6.1 release, without `roles` nor
`review` — while the runner's own calls use the binary under test. All
40 implementation runs measured with it, on Opus and on Sonnet, ran the
review step with the wrong codev and must be recorded again.

## What Changes

- Before any case runs, `codev-evals run` exits with code 2 when the
  `--codev` binary's file is not named `codev` (`codev.exe` on Windows),
  naming the file.

## Capabilities

### New Capabilities

### Modified Capabilities

- role-evaluation

### Removed Capabilities

## Impact

- `evals/src/runner.rs` (`preflight`) and `CONTRIBUTING.md` /
  `CONTRIBUTING.fr.md`, which say how to freeze a codev binary for a
  comparison. `evals/` is not shipped.
- The implementation baseline is recorded again with a binary named
  `codev`.

## Domains

None

## Release note

None
