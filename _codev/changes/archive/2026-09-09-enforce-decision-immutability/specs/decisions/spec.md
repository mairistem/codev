## ADDED Requirements

### Requirement: Body seal recorded by the CLI

The CLI SHALL maintain a file `_codev/decisions/seal.yaml` — versioned
with the project — that lists the sealed local decisions, with, for
each entry, the decision's `id`, the SHA-256 hash of the ADR's **body**
(everything that follows the `---` separator closing the frontmatter),
and the date on which the seal was applied. Only local decisions are
sealed — inherited decisions belong to the source project, not to the
consumer.

The file contains only the schema version and the list of seals — a
seal is an `id`, a `bodySha256` prefixed with `sha256:`, and a `sealedAt`
in `YYYY-MM-DD` format.

#### Scenario: Seal written when a project creates a decision

- **GIVEN** a project where `_codev/decisions/seal.yaml` does not exist
- **WHEN** the user runs `codev decision new "A first choice"`
- **THEN** the file `_codev/decisions/seal.yaml` is created
- **AND** it contains an entry whose `id` is `"0001"` and whose
  `bodySha256` matches the hexadecimal SHA-256 of the body of the file
  `0001-a-first-choice.md` (prefixed with `sha256:`)

#### Scenario: Seal file format

- **GIVEN** the file `_codev/decisions/seal.yaml` created by the CLI
- **WHEN** a human opens it
- **THEN** it contains a `version: 1` key first
- **AND** a `seals` key carrying a list whose every entry has exactly
  the fields `id`, `bodySha256`, `sealedAt`
- **AND** no other unknown field

### Requirement: `decision new` writes the ADR and the seal in the same plan

The operation `codev decision new <title>` MUST produce an effect plan
that writes both the ADR **and** the corresponding entry in
`_codev/decisions/seal.yaml`. If either fails, neither is written — the
command never leaves an ADR without a seal nor a seal without an ADR.

#### Scenario: Failure to write the seal cancels the creation

- **GIVEN** a project in which `_codev/decisions/seal.yaml` is
  write-locked by the system
- **WHEN** the user runs `codev decision new "Title"`
- **THEN** no ADR file is written in `_codev/decisions/`
- **AND** the `seal.yaml` file stays unchanged

### Requirement: `decision supersede` seals the new ADR without touching the old one's seal

The operation `codev decision supersede <id> <title>` MUST add a seal
entry for the newly created ADR, and MUST NOT modify the seal entry of
the old one — since its body stays identical to the character (a
requirement already in effect), its seal stays valid.

#### Scenario: Supersession seals only the new one

- **GIVEN** a project containing an ADR `0003 accepted` that is sealed
  and referenced in `seal.yaml` under `id: "0003"`
- **WHEN** the user runs `codev decision supersede 0003 "New choice"`
- **THEN** the `seal.yaml` file now contains two entries: the one for
  `0003` (unchanged) and a new one for the created ADR (for example
  `0007`)
- **AND** the `0003` entry has exactly the same `bodySha256` as before
  the supersession

### Requirement: `validate` detects body alterations

`codev validate` MUST emit three new findings with stable codes to
report discrepancies between ADRs and their seal:

- `decision_unsealed` — **warning** emitted for any local ADR with
  status `accepted` or `superseded` that has no entry in `seal.yaml`.
  The message recalls that migration is done with `codev decision seal`.
- `decision_seal_mismatch` — **error** emitted for any ADR whose
  computed `bodySha256` no longer matches the one recorded in
  `seal.yaml`. The message names the ADR concerned and recalls that a
  deliberate modification goes through `codev decision seal --force`.
- `decision_orphan_seal` — **warning** emitted for any `seal.yaml`
  entry whose referenced ADR no longer exists in `_codev/decisions/`.

#### Scenario: Detection of a body modified in place

- **GIVEN** a project whose ADR `0001` is sealed
- **AND** a human has edited the body of the file `0001-*.md` without
  updating `seal.yaml`
- **WHEN** the user runs `codev validate`
- **THEN** the output contains a finding with code
  `decision_seal_mismatch` naming `0001`
- **AND** the command's exit code is non-zero

#### Scenario: Detection of an unsealed ADR

- **GIVEN** a project containing six local `accepted` ADRs, all without
  an entry in `seal.yaml` (initial migration state)
- **WHEN** the user runs `codev validate`
- **THEN** the output contains six findings with code
  `decision_unsealed`, one per ADR
- **AND** the command's exit code is zero (warnings, not errors)

#### Scenario: Detection of an orphan seal

- **GIVEN** a project whose `seal.yaml` contains an entry for ADR
  `0004`
- **AND** the file `0004-*.md` has been deleted
- **WHEN** the user runs `codev validate`
- **THEN** the output contains a `decision_orphan_seal` finding naming
  `0004`
- **AND** the command's exit code is zero (warning, not error)

### Requirement: `decision seal` adds or renews a seal entry

`codev decision seal <id>` MUST create a seal entry for a local ADR
that has none — this is the migration gesture. For an already sealed
ADR whose body has changed, the command MUST refuse without `--force`
and recall that immutability is intentional. With `--force`, it
rewrites the `bodySha256` and updates `sealedAt`. An inherited ADR
cannot be sealed by the consumer project (sealing belongs to the source
project).

#### Scenario: Migration of an unsealed ADR

- **GIVEN** a project containing an ADR `0001 accepted` with no entry
  in `seal.yaml`
- **WHEN** the user runs `codev decision seal 0001`
- **THEN** `seal.yaml` now carries an entry for `0001` whose
  `bodySha256` matches the current body of the file

#### Scenario: Refusal to re-seal without `--force`

- **GIVEN** a project whose ADR `0001` is sealed, and whose body was
  then edited in place
- **WHEN** the user runs `codev decision seal 0001`
- **THEN** `seal.yaml` stays unchanged
- **AND** the error message names the stable code `seal_conflict` and
  recalls that `--force` deliberately rewrites the seal

#### Scenario: Re-seal with `--force`

- **GIVEN** the same context
- **WHEN** the user runs `codev decision seal 0001 --force`
- **THEN** the `0001` entry in `seal.yaml` now carries the new
  `bodySha256` and a `sealedAt` date matching today's date

#### Scenario: Refusal to seal an inherited decision

- **GIVEN** a project inheriting from a source containing
  `path:~/shared/0100`
- **WHEN** the user runs `codev decision seal path:~/shared/0100`
- **THEN** no write takes place
- **AND** the error message names the stable code
  `cannot_seal_inherited` and recalls that sealing belongs to the
  source project
