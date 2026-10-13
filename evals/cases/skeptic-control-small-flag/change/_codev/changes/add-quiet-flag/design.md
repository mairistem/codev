# Design: `--quiet`

## Context

`ledger import` prints one progress line per 1 000 rows on standard error
through `src/cli/progress.rs`; errors go through `src/cli/errors.rs`, a
separate path.

## Decisions

### Decision: The progress reporter takes a `quiet` switch

`Progress::new(quiet)` does nothing when `quiet` is set; errors keep their
own path, so they cannot be silenced by mistake.

**Alternatives considered**: redirecting standard error (would hide errors).
