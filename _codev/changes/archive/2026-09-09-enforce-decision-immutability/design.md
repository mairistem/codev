# Design: enforce decision immutability

## Context

See `proposal.md`. The stake: make immutability **mechanically**
verifiable — not merely stated in a convention. Without it, K6
(permitted deviations from inherited decisions) has no solid base to
reason on.

## Goals / Non-Goals

This design frames the location of the seal, what the hash covers, the
core/shell separation, and the CLI commands affected. It does not frame
cryptographic signing (GPG and the like), nor the promotion of
decisions from `design.md` (that is K7).

## Decisions

### Decision: seal in a separate file `_codev/decisions/seal.yaml`

Two options considered:

| Option | Pro | Con |
|---|---|---|
| **A. `bodySha256` field in each ADR's frontmatter** | Self-contained; `git diff` directly reveals tampering | Adds a "machine" field that clutters human reading of an ADR; forces answering the question "and the hash itself, is it in the hash?" (no — but it is confusing on first read) |
| **B. Separate `seal.yaml` file next to the ADRs** | The ADR stays clean for the human eye; aligned with the pattern already in place (`codev.lock` for sources) | One more file to keep consistent |

**Chosen: B.** The pattern goes beyond this case — codev already has a
source-of-truth file kept by the CLI (`codev.lock` for sources), we
repeat the pattern instead of inventing a new mode. The `seal.yaml` file
lives under `_codev/decisions/` to stay as close as possible to its
subject, and `serde_norway` parses it like the rest of the YAML —
consistent with decision
[0006](../../decisions/0006-serde-norway-for-yaml.md).

### Decision: the hash covers the **body**, not the whole file

An ADR's frontmatter is designed to evolve legitimately — an `accepted`
ADR becomes `superseded` through an explicit write by
`codev decision supersede`. Hashing the whole file would require
re-sealing on every transition, which would deprive the requirement
"supersede does not touch the body" (already in the spec) of its
verifiable attribute.

**Corollary**: the hash computation cuts at the first `\n---\n` (or
`\n---\r\n` for Windows) following the opening `---` line, and hashes
everything that comes after, byte for byte, without normalization. An
implicit normalization (trim, LF↔CRLF) would break the promise
"identical to the character".

### Decision: `plan_new_decision` and `plan_supersede` return a plan
that includes the seal write

Directly aligned with decision
[0001](../../decisions/0001-functional-core-imperative-shell.md):
the core produces a complete `Plan { writes, moves, … }`; the shell
executes it in one go. Adding the `seal.yaml` write to the plan
preserves atomicity — either the ADR and the seal are written, or
nothing is, without having to invent a compensation.

**Corollary**: the pure function that produces the plan needs to read
the **current** `seal.yaml` to merge it with the new entry. It receives
its content as an argument (the shell's `FileSystem` port read it
beforehand), it does not read it itself — decision
[0002](../../decisions/0002-crate-graph-as-dependency-rule.md).

### Decision: `validate` surfaces findings, not exceptions

`decision_unsealed` is a **warning** (exit code 0), not an error: on an
existing project, all ADRs start unsealed — a non-zero errno would
prevent every other flow (`codev status`, `codev sync`,
`codev archive`) from running until sealing is done. The warning draws
attention without blocking.

`decision_seal_mismatch` is an **error** (non-zero exit code): a body
that no longer matches the seal is tampering (or a deliberate edit not
re-approved) — the decision index can no longer be considered reliable
until it is resolved.

`decision_orphan_seal` is a **warning**: an ADR may have been deleted
deliberately (unlikely but possible); the orphan lock alone does not
seriously compromise anything.

**Rejected alternative**: align everything as errors. Makes migration
impractical — the first post-delivery `codev validate` fails on the 6
existing ADRs, breaking `sync`, `archive` and the rest. Costs too much
for what we gain.

### Decision: a single CLI command `codev decision seal <id>`, with `--force`

A single command, two modes depending on state:

- Unsealed ADR → adds the entry without discussion (migration case).
- Sealed ADR and the body has changed → **refuses** without `--force`,
  with the stable code `seal_conflict`; with `--force`, rewrites the
  `bodySha256` and refreshes `sealedAt`.
- Sealed ADR and the body is unchanged → silent no-op (the seal is
  already correct).

A `codev decision seal --all` (bulk) command for the initial migration
is deferred: `for id in $(codev decision list --json | jq
-r …); do codev decision seal "$id"; done` does the job on the 6 ADRs
of this repository without requiring a dedicated path in the CLI. If
the pattern becomes recurrent, we will add it later.

### Decision: inherited decisions are **not** sealed by the consumer

A consumer project cannot apply a seal to an ADR it did not write —
that would usurp the source project's act of acceptance. The seal lives
in the source project; the consumer, when it indexes inherited
decisions (already in place), may optionally verify their remote
`seal.yaml` if there is one (can be deferred, not in this change).

**Aligned** with decision
[0005](../../decisions/0005-read-only-inherited-sources.md):
inherited sources are read-only. `codev decision seal
path:~/shared/0100` therefore returns `cannot_seal_inherited`.

## Risks / Trade-offs

- **Silently missed migration.** If the user does not see the
  `decision_unsealed` warnings (for example because they never run
  `validate` manually — they go through `sync` or `archive`, which do a
  `validate` pre-flight), they could leave their ADRs unsealed for a
  long time. → **Mitigation**: `codev status` (which does not yet have
  a validate pre-flight) will gain the warning count in its human
  summary, in a future change. For now, the mention in the `sync` /
  `archive` summary is sufficient.
- **False positive on line endings.** The byte-for-byte hash will catch
  an accidental `LF → CRLF` (git config `core.autocrlf`, an editor that
  reformats). → **Accepted trade-off**: `codev decision seal --force` is
  the official path. Document it in the error message.
- **The seal does not protect against an author who edits knowingly.**
  A developer can run `--force` without thinking. → **Accepted
  trade-off**: the seal is a technical guardrail, not an access
  control. `git blame` remains the ultimate trace.

## Migration Plan

After delivery:

1. `codev validate` surfaces 6 `decision_unsealed` warnings on this
   repository.
2. Short loop: `codev decision list --json | jq -r '.decisions[] |
   select(.origin == "project") | .id' | while read id; do codev
   decision seal "$id"; done`. A single commit carries the 6 seals
   added in `_codev/decisions/seal.yaml`.
3. `codev validate` returns to 0 warnings on the decisions side.
