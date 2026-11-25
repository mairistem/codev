# Proposal: The runner refuses a codev binary that is not named codev

## Why

`codev-evals run --codev <binary>` puts the binary's folder first in the
search path, and the agent calls `codev` by name. A binary frozen under
another name, as `ie-bin` was for the implementation baseline of
2026-10-20, is never found: the agent runs whatever other `codev` the
operator has installed — here the 0.6.1 release, without `roles` nor
`review` — while the runner's own calls use the binary under test. All
40 implementation runs measured with it ran the review step with the
wrong codev: the 20 Opus runs of the committed baseline are recorded
again here; the 20 Sonnet runs, kept outside the repository for the
decision on the implementation's model, are measured again outside this
change.

## What Changes

- Before any case runs, `codev-evals run` exits with code 2 when the
  `--codev` binary's file is not named `codev` (`codev.exe` on Windows),
  naming the file.
- An implementation case may reword a task that checks the repository
  with `codev` so that it uses the codev the change builds: with the right
  binary, the baseline showed today's codev failing an older change's
  proposal and doctor reporting the skills installed by the evaluation.
  `decision-accept-command`, `forward-compatible-config` and
  `fix-merge-and-skill-refresh` are reworded so, and their runs recorded
  again; a suite test keeps every case so.

## Capabilities

### New Capabilities

### Modified Capabilities

- role-evaluation
- implementation-evaluation

### Removed Capabilities

## Impact

- `evals/src/runner.rs` (`preflight`) and `CONTRIBUTING.md` /
  `CONTRIBUTING.fr.md`, which say how to freeze a codev binary for a
  comparison. `evals/` is not shipped.
- The implementation baseline is recorded again with a binary named
  `codev`, and `docs/en/src/reliability.md` and
  `docs/fr/src/reliability.md`, generated from it, are regenerated.

## Domains

None

## Release note

None
