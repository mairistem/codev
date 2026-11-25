# Proposal: Specify behaviors that shipped without a codev change

## Why

Four observable behaviors reached `main` through plain pull requests instead
of a codev change, so the main specs do not describe them: decisions saved
with Windows line endings, coral warning callouts in `codev docs`, a silent
`codev docs --print` when its reader exits, and forward slashes in the paths
`codev init --inherit` writes. codev is developed with codev: the specs must
describe what the system does, so these behaviors are specified now.

## What Changes

- The `decisions` spec states that ADR files with CRLF line endings are
  parsed like LF files, and that changing a decision's status keeps its
  sealed body byte for byte whatever the line endings.
- The `docs` spec states that warning callouts render distinctly from
  notes in the offline manual, and that `--print` exits silently with code
  0 when its output is closed early.
- The `init` spec states that local paths written by `--inherit` use
  forward slashes on every platform.
- No code changes: the behaviors and their tests already exist.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `decisions` — CRLF ADR files.
- `docs` — warning callouts offline; silent broken pipe on `--print`.
- `init` — portable paths for `--inherit`.

### Removed Capabilities

None.

## Impact

- `_codev/specs/decisions/spec.md`, `_codev/specs/docs/spec.md`,
  `_codev/specs/init/spec.md` only.
