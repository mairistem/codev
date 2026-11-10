## ADDED Requirements

### Requirement: Two local decisions never share an id, nor two seals

`codev validate` SHALL report as the error `decision_id_duplicate` every
id carried by two or more local decisions, naming each of their files and
the next free id — the largest numeric local id plus one — to
renumber all but one of them. It SHALL report as the error
`decision_seal_duplicate` every id that `_codev/decisions/seal.yaml`
seals more than once. A local decision and an inherited one sharing an id
remain the warning `decision_id_collision`. Each of the decisions sharing an
id is still checked against the seal: when the id is sealed more than
once, a decision whose body matches any of its entries is not reported
as `decision_seal_mismatch`.

#### Scenario: Two branches each created decision 0013

- **GIVEN** a project holding `0013-cache-the-index.md` and
  `0013-sign-the-releases.md`, both with `id: "0013"`, and no decision
  numbered above `0013`
- **WHEN** the user runs `codev validate`
- **THEN** an error `decision_id_duplicate` names `0013` and both files
- **AND** its message proposes `0014` for the renumbering
- **AND** the exit code is non-zero

#### Scenario: The seal file seals one id twice

- **GIVEN** a `_codev/decisions/seal.yaml` holding two entries with
  `id: '0013'`
- **WHEN** the user runs `codev validate`
- **THEN** an error `decision_seal_duplicate` names `0013`
- **AND** the exit code is non-zero

#### Scenario: Both decisions sharing an id are checked against the seal

- **GIVEN** two local accepted decisions `0013`, and a `seal.yaml` whose
  single `0013` entry matches the body of the first one only
- **WHEN** the user runs `codev validate`
- **THEN** `decision_seal_mismatch` is reported for the second file
- **AND** `decision_id_duplicate` is reported

#### Scenario: Two decisions sharing an id, each with its own seal entry

- **GIVEN** two local accepted decisions `0013` and a `seal.yaml` holding
  two `0013` entries, each matching the body of one of them
- **WHEN** the user runs `codev validate`
- **THEN** `decision_id_duplicate` and `decision_seal_duplicate` are
  reported
- **AND** no `decision_seal_mismatch` is reported

#### Scenario: Distinct ids raise nothing

- **GIVEN** a project whose local decisions carry the ids `0001` to `0013`,
  each once, each sealed once
- **WHEN** the user runs `codev validate`
- **THEN** neither `decision_id_duplicate` nor `decision_seal_duplicate` is
  reported

#### Scenario: A local id shared with an inherited source stays a warning

- **GIVEN** a local decision `0007` and an inherited decision `0007`
- **WHEN** the user runs `codev validate`
- **THEN** the warning `decision_id_collision` is reported
- **AND** no `decision_id_duplicate` is reported

## MODIFIED Requirements

### Requirement: `decision seal` adds or renews a seal entry

`codev decision seal <id>` MUST create a seal entry for a local ADR that
does not have one — this is the migration operation. For an already
sealed ADR whose body has changed, the command MUST refuse without
`--force` and recall that immutability is intentional. With `--force`, it
rewrites the `bodySha256` and updates `sealedAt`, and leaves a single
entry for that id when `seal.yaml` held several. An inherited ADR cannot
be sealed by the consuming project (sealing belongs to the source
project).

#### Scenario: Migration of an unsealed ADR

- **GIVEN** a project containing an ADR `0001 accepted` without an entry in
  `seal.yaml`
- **WHEN** the user runs `codev decision seal 0001`
- **THEN** `seal.yaml` now carries an entry for `0001` whose
  `bodySha256` matches the current body of the file

#### Scenario: Refusal to re-seal without `--force`

- **GIVEN** a project whose ADR `0001` is sealed, and whose body was
  subsequently edited in place
- **WHEN** the user runs `codev decision seal 0001`
- **THEN** `seal.yaml` remains unchanged
- **AND** the error message names the stable code `seal_conflict` and
  recalls that `--force` deliberately rewrites the seal

#### Scenario: Re-seal with `--force`

- **GIVEN** the same context
- **WHEN** the user runs `codev decision seal 0001 --force`
- **THEN** the `0001` entry in `seal.yaml` now carries the new
  `bodySha256` and a `sealedAt` date matching the current date

#### Scenario: Refusal to seal an inherited decision

- **GIVEN** a project inheriting from a source containing `path:~/shared/0100`
- **WHEN** the user runs `codev decision seal path:~/shared/0100`
- **THEN** no write takes place
- **AND** the error message names the stable code
  `cannot_seal_inherited` and recalls that sealing belongs to the
  source project

#### Scenario: Re-seal with `--force` collapses a duplicated entry

- **GIVEN** a `seal.yaml` holding two `0013` entries, left by a text merge,
  and a single local decision `0013`
- **WHEN** the user runs `codev decision seal 0013 --force`
- **THEN** `seal.yaml` holds one `0013` entry, matching that decision's body
- **AND** `codev validate` reports no `decision_seal_duplicate`
