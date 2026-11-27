# Design: A replayed history stays as clean as it was

## Context

`bring_through_rename(dir)` (`evals/src/runner.rs`) is called on every
tree extracted from a commit: by `setup_plan` for the fixtures of the
planning, update, implementation and merge cases, before the commit
`state at <ref>`, and by `implement_run::extract`, which `calibrate` uses
for the `merged` and `ref` trees. When the tree holds the former planning
root, it moves every path holding the former name and rewrites every text
file through `legacy::through_rename` (codev → compagnon, Codev →
Compagnon, CODEV → COMPAGNON).

Observed on `implement-decision-accept-command` (merged commit
`9dc1f45`):

- extracted as is, `cargo fmt --all --check` passes (rustfmt 1.9.0);
  renamed, it fails on lines the longer name pushes past the width, e.g.
  `crates/compagnon-agents/src/claude.rs:230`;
- `compagnon validate --strict` on the renamed tree reports
  `decision_seal_mismatch` on 0001–0006: their bodies name `codev`, and
  `_compagnon/decisions/seal.yaml` holds the hashes of the bodies before
  the rename.

`compagnon_core::decisions::seal` offers `parse_seal_file`,
`render_seal_file` and `body_hash`; the seal file is
`<planning root>/decisions/seal.yaml`, a list of `{id, bodySha256,
sealedAt}`.

## Goals / Non-Goals

**Goals:**

- A renamed tree passes the checks it passed as committed: the format
  and the seals.
- A defect the commit had stays: a seal that did not match, code that was
  not formatted.

**Non-Goals:**

- Re-recording the baseline: the next paid run of `code.implement` does
  it, by the user's decision.
- Changing `legacy::through_rename` or the CLI's `compagnon migrate`.

## Decisions

### Decision: Seals are carried through the rename, not recomputed for all

Before rewriting the texts, `bring_through_rename` reads the seal file
under the former planning root and, for each sealed decision, whether its
body's hash matched the recorded one. After the rename, it records the
new body hash for those that matched, and only those, keeping `sealedAt`,
and writes the file with `render_seal_file`. A decision edited after its
seal in the commit keeps its broken seal, so a case that tests a broken
seal keeps testing it.

**Alternatives considered**: `compagnon decision seal <id> --force` for
each decision — rejected: it needs the binary under test, which a
`calibrate` of an older tree has not built yet, and it would also
"repair" seals broken on purpose.

### Decision: Only the Rust files the rename changed are formatted, with rustfmt

After the rename, when the tree has a `Cargo.toml` at its root, each `.rs`
file whose text the rename changed, and that a crate formats — under a
crate's `src/`, `tests/`, `benches/` or `examples/`, or its `build.rs`, as
`cargo fmt --all` sees them — is formatted by passing its text to
`rustfmt --edition <edition> --emit stdout` on standard input, run from
the file's directory so that rustfmt finds the project's `rustfmt.toml`,
and writing the output back. Through standard input rustfmt does not
follow `mod x;` declarations, so a module file the rename did not change
is left as committed (given a path, rustfmt would format it too). The
edition is the crate's: the `edition` of the nearest `Cargo.toml` above
the file, the root's `[workspace.package]` one when it says
`edition.workspace = true`, `2021` when absent. Other `.rs` files — test
data, templates — are left as renamed, as `cargo fmt --all` leaves them. A missing `rustfmt`, or one that fails, is an error of the
fixture's preparation, named: a run on a tree that is not formatted would
measure the rename again.

**Alternatives considered**: `cargo fmt --all` — rejected: it runs
`cargo metadata`, which needs the dependencies the fixture's isolated
`CARGO_HOME` has not fetched yet at that point, offline; formatting the
whole tree would also correct code the commit left unformatted.

## Risks / Trade-offs

- [The runner's rustfmt differs from the one the commit was formatted
  with] → the `format` check uses the same rustfmt, so a fixture and its
  check agree; `calibrate` keeps reporting a merged commit the installed
  rustfmt rejects for another reason.
- [Editions or targets the plan reads differently from cargo] → the
  rule follows what `cargo fmt --all` formats; the cases at hand have
  every crate on the workspace edition and every changed file in a
  target, and a case that differs would show in `calibrate`.
- [A seal file the parser rejects] → left as it is, the rename goes on;
  `validate` reports it, as the commit would.
