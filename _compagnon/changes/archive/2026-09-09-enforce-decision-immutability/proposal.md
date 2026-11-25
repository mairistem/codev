# Proposal: enforce decision immutability through the CLI

## Why

An `accepted` decision must be a frozen trace: "here is the choice we
settled and that we rely on". Today, this immutability is only social —
the file `_codev/decisions/0003-*.md` can be edited in place, silently,
without either the CLI or `validate` noticing. A `git blame` will
eventually see it, but all of the tool's reasoning that relies on the
decisions in effect (injection into the `design` instructions, index,
supersession resolution) runs from the **current** content of the file,
not from the content that was accepted.

This is the prerequisite for K6 (permitted deviations from inherited
decisions): a deviation can only be reasoned about against an immutable
base.

## What Changes

- **New file `_codev/decisions/seal.yaml`** — versioned with the
  project, kept up to date by the CLI. One entry per local `accepted` or
  `superseded` ADR, carrying the `id`, the SHA-256 hash of the ADR's
  **body** (what follows the frontmatter), and the date on which the
  seal was applied.
- **`codev decision new` seals as it writes** — a single effect plan
  carries both writes (ADR + seal entry), either both succeed or
  neither does.
- **`codev decision supersede` seals the new ADR** — since the old one's
  body stays byte-identical (already required by the spec), its seal
  stays valid without any manipulation.
- **`codev validate` surfaces three new stable findings** —
  `decision_unsealed` (warning: ADR without a seal entry, migration
  expected), `decision_seal_mismatch` (**error**: the body no longer
  matches its seal, someone edited in place), and
  `decision_orphan_seal` (warning: seal entry for an ADR that no longer
  exists).
- **New command `codev decision seal <id>`** — for the initial
  migration (the repository's 6 current ADRs will be flagged `unsealed`
  on the first `validate`) and to re-approve a body that deliberately
  changed (`--force` required if a different seal already existed).
- **Nothing that breaks the existing JSON contract** — current commands
  gain at most one additional field in their response (the hash where
  relevant), no field is removed or renamed.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `decisions` — four new requirements for sealing: format and location
  of the seal, writing by `decision new`, writing by
  `decision supersede`, `validate` findings, and the
  `decision seal` command.

## Impact

- **Code**: new module `codev-core::decisions::seal` (hash computation,
  parsing/writing of `seal.yaml`, comparison), extension of the plan
  produced by `plan_new_decision` and `plan_supersede` to include the
  seal write, extension of `validate` to emit the three findings.
- **JSON contract**: addition of a `bodySha256` field (optional) in the
  `decision` entry of `decision new --json` and `decision seal --json`.
  The `status` array gains the three new `code`s, following the format
  already versioned.
- **File written**: `_codev/decisions/seal.yaml`, YAML format aligned
  with `codev.lock` (a source-of-truth file kept by the CLI, editable if
  needed but normally not handled by hand).
- **Migration**: on the first `validate` after this delivery, the 6
  ADRs of the current repository surface as `decision_unsealed`. A
  `codev decision seal --all` (or six sequential `codev decision seal
  <id>`) is enough to close the migration.
- **Out of scope**:
  - Sealing **inherited** decisions — the source project is responsible
    for that, not the consumer project. K6 will say how the consumer
    project can *deviate* from an inherited decision without claiming
    to modify its content.
  - Cryptographic signing (GPG, signify) — the seal attests integrity,
    not authenticity. Can be deferred if the need arises.
  - A `--strict` mode for `validate` that would turn every warning into
    an error — that is the separate change E5.
