## MODIFIED Requirements

### Requirement: Creating a decision through the CLI

`codev decision new <title>` SHALL create a new ADR in the project, with
an automatically generated numeric `id` (the largest numeric `id` found
in the project + 1, on four digits), a file named `NNNN-<slug>.md` where
`<slug>` is derived from the title in kebab-case, and a valid frontmatter
including the status and the current date. Without `--status`, the
status MUST be `proposed`: the ADR is created unsealed, so that its body
can be written before it is accepted with `codev decision accept`.

#### Scenario: Creation in a project without ADRs

- **GIVEN** a project whose `_codev/decisions/` directory is empty
- **WHEN** the user runs `codev decision new "A first choice"`
- **THEN** the file `_codev/decisions/0001-a-first-choice.md` is created
- **AND** its frontmatter carries `id: "0001"`, `title: "A first choice"`,
  `status: proposed`, and a date in `YYYY-MM-DD` format
- **AND** `_codev/decisions/seal.yaml` carries no entry for `0001`

#### Scenario: Sequential numbering

- **GIVEN** a project whose `_codev/decisions/` directory already contains
  six ADRs numbered `0001` to `0006`
- **WHEN** the user runs `codev decision new "A seventh choice"`
- **THEN** the file `_codev/decisions/0007-a-seventh-choice.md` is created
- **AND** its `id` is `"0007"`

#### Scenario: Customizable status

- **GIVEN** a project
- **WHEN** the user runs `codev decision new "Settled choice"
  --status accepted`
- **THEN** the created ADR carries `status: accepted`
- **AND** its body is sealed in `_codev/decisions/seal.yaml`

#### Scenario: Empty title rejected

- **GIVEN** a project
- **WHEN** the user runs `codev decision new ""`
- **THEN** no file is written
- **AND** an error message names the stable code `empty_title` and
  asks for a non-empty title

### Requirement: Stable JSON contract for all commands

When `--json` is requested, each command MUST write exactly one JSON
document to stdout whose shape is frozen per version, with a root
`status` array for execution errors — the same rules as the rest of the
codev contract.

#### Scenario: Shape of a `decision new --json`

- **GIVEN** a project
- **WHEN** the user runs `codev decision new "X" --json`
- **THEN** stdout carries a JSON document containing `decision: { id:
  "0001", qualifiedId: "project/0001", path: "…", title: "X", status:
  "proposed" }`, the `path` of the created file, and a `status: []`
- **AND** it carries no `bodySha256`, since a proposed decision is not
  sealed

#### Scenario: Shape of a `decision show` failure on an unknown id

- **GIVEN** a project
- **WHEN** the user runs `codev decision show 9999 --json`
- **THEN** stdout carries a single JSON document containing
  `decision: null` and `status: [{ code: "unknown_decision_id", … }]`

### Requirement: Body seal recorded by the CLI

The CLI SHALL maintain a `_codev/decisions/seal.yaml` file — versioned
with the project — that records the sealed local decisions, with, for
each entry, the `id` of the decision, the SHA-256 hash of the ADR's
**body** (everything that follows the `---` separator closing the
frontmatter), and the date on which the seal was applied. Only local
decisions are sealed — inherited decisions belong to the source project,
not to the consumer.

The file contains only the schema version and the list of seals —
a seal is an `id`, a `bodySha256` prefixed with `sha256:`, and a
`sealedAt` in `YYYY-MM-DD` format.

#### Scenario: Seal written when a decision is created

- **GIVEN** a project in which `_codev/decisions/seal.yaml` does not exist
- **WHEN** the user runs `codev decision new "A first choice" --status
  accepted`
- **THEN** the file `_codev/decisions/seal.yaml` is created
- **AND** it contains an entry whose `id` is `"0001"` and whose
  `bodySha256` matches the hexadecimal SHA-256 of the body of the file
  `0001-a-first-choice.md` (prefixed with `sha256:`)

#### Scenario: Seal file format

- **GIVEN** the `_codev/decisions/seal.yaml` file created by the CLI
- **WHEN** a human opens it
- **THEN** it contains a `version: 1` key at the top
- **AND** a `seals` key carrying a list in which each entry has
  exactly the fields `id`, `bodySha256`, `sealedAt`
- **AND** no other unknown field

### Requirement: `decision new` writes the ADR and the seal in the same plan

When `codev decision new <title>` creates a decision with the status
`accepted` or `superseded`, the operation MUST produce an effect plan
that writes both the ADR **and** the corresponding entry in
`_codev/decisions/seal.yaml`. If either one fails, neither is written
— the command never leaves an ADR without a seal nor a seal without an
ADR. A decision created with another status, `proposed` included, is
written without a seal entry.

#### Scenario: A failed seal write cancels the creation

- **GIVEN** a project in which `_codev/decisions/seal.yaml` is
  write-locked by the system
- **WHEN** the user runs `codev decision new "Title" --status accepted`
- **THEN** no ADR file is written to `_codev/decisions/`
- **AND** the `seal.yaml` file remains unchanged

#### Scenario: A proposed decision is created without a seal

- **GIVEN** a project
- **WHEN** the user runs `codev decision new "Title"`
- **THEN** the ADR is written with `status: proposed`
- **AND** `_codev/decisions/seal.yaml` is neither created nor modified

## ADDED Requirements

### Requirement: `decision accept` accepts and seals a proposed decision

`codev decision accept <id>` SHALL turn a local decision whose status is
`proposed` into an `accepted` decision and seal its body. The operation
MUST produce a single effect plan that rewrites the frontmatter `status`
to `accepted` **and** writes the decision's entry in
`_codev/decisions/seal.yaml`; if the plan cannot be computed, neither is
written. The body of the ADR — everything after the frontmatter — MUST
stay byte for byte identical, so the recorded hash is the hash of the
text the user wrote. The command MUST refuse, without writing anything:

- an unknown identifier, with the stable code `unknown_decision_id`;
- an inherited decision, with the stable code `cannot_accept_inherited`,
  since inherited decisions are read-only;
- a decision whose status is not `proposed`, with the stable code
  `decision_not_proposed`; for an `accepted` decision the message says
  there is nothing to do and points to `codev decision supersede` to
  replace it.

With `--json`, the command MUST write a document carrying the accepted
`decision`, the `path` of its file and its `bodySha256`.

#### Scenario: A proposed decision is accepted and sealed

- **GIVEN** a project containing a local ADR `0007` with `status:
  proposed` whose body has been written, and no seal entry for `0007`
- **WHEN** the user runs `codev decision accept 0007`
- **THEN** the ADR now carries `status: accepted`
- **AND** its body is identical to what it was before the command
- **AND** `seal.yaml` carries an entry for `0007` whose `bodySha256`
  matches that body
- **AND** `codev validate` reports neither `decision_seal_mismatch` nor
  `decision_unsealed` for `0007`

#### Scenario: JSON output of an acceptance

- **GIVEN** the same project
- **WHEN** the user runs `codev decision accept 0007 --json`
- **THEN** stdout carries a JSON document with `decision.id: "0007"`,
  `decision.status: "accepted"`, the `path` of the file, a `bodySha256`
  prefixed with `sha256:`, and `status: []`

#### Scenario: Refusal to accept an inherited decision

- **GIVEN** a project inheriting from a source containing
  `path:~/shared/0100` with `status: proposed`
- **WHEN** the user runs `codev decision accept path:~/shared/0100`
- **THEN** no write takes place
- **AND** the error names the stable code `cannot_accept_inherited`

#### Scenario: Refusal to accept a decision that is already accepted

- **GIVEN** a project containing a local ADR `0003` with `status:
  accepted`
- **WHEN** the user runs `codev decision accept 0003`
- **THEN** neither the ADR nor `seal.yaml` is modified
- **AND** the error names the stable code `decision_not_proposed` and
  points to `codev decision supersede`

#### Scenario: Refusal to accept a decision with another status

- **GIVEN** a project containing a local ADR `0004` with `status:
  rejected`
- **WHEN** the user runs `codev decision accept 0004`
- **THEN** no write takes place
- **AND** the error names the stable code `decision_not_proposed`

#### Scenario: Status and seal are written together or not at all

- **GIVEN** a project containing a local `proposed` ADR `0007`
- **AND** a `_codev/decisions/seal.yaml` that cannot be read as a seal
  file
- **WHEN** the user runs `codev decision accept 0007`
- **THEN** the command fails
- **AND** the ADR still carries `status: proposed`
- **AND** `seal.yaml` is unchanged
