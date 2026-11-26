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
file whose text the rename changed is formatted with `rustfmt --edition
<edition>`, the edition read from the root `Cargo.toml`
(`[workspace.package]` or `[package]`, `2021` when absent). The other files
are as the commit left them; rustfmt finds the project's `rustfmt.toml` by
itself. A missing `rustfmt`, or one that fails, is an error of the
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
- [A seal file the parser rejects] → left as it is, the rename goes on;
  `validate` reports it, as the commit would.
