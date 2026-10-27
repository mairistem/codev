## MODIFIED Requirements

### Requirement: ADR format recognized

The parser SHALL recognize an ADR written as a YAML frontmatter followed by
free-form markdown sections. The frontmatter carries at least the fields
`id`, `title`, `status`, `date`, and optionally `tags` (a list) and
`supersedes` (an identifier or an array of identifiers). A file without a
frontmatter, or whose frontmatter lacks a mandatory field, is reported,
not silently parsed.

A frontmatter field the parser does not know SHALL NOT reject the
decision: the decision is read from the fields it knows, and the field is
reported as a finding of severity `warning` and code
`decision_unknown_field`, at its line in the ADR's file. Its message MUST
name the field, say that the decision may have been written by a newer
codev, quote the project's `codev_version` when set, and point to
`codev self-update`, or to correcting the field if it is a typo. Like any
warning, it fails `codev validate --strict` and not `codev validate`. An
inherited decision is read the same way, but its unknown fields are not
reported: the source is read-only, and the inheriting project cannot fix
them.

#### Scenario: Well-formed ADR

- **GIVEN** a file starting with `---`, containing a YAML frontmatter
  with `id: 0007`, `title: "…"`, `status: accepted`, `date: 2026-09-08`,
  `tags: [architecture]`, then a `---` separator, then free-form markdown
- **WHEN** the parser reads the file
- **THEN** the result exposes the identifier `0007`, the title, the status
  `accepted`, the date and the tag `architecture`

#### Scenario: ADR without frontmatter

- **GIVEN** a markdown file without a `---` header
- **WHEN** the parser reads the file
- **THEN** a finding with the stable code `decision_missing_frontmatter`
  reports that the file does not have the expected format

#### Scenario: Mandatory field missing

- **GIVEN** a file whose frontmatter has no `title`
- **WHEN** the parser reads the file
- **THEN** a finding with code `decision_missing_field` names the missing
  field

#### Scenario: A frontmatter field this codev does not know

- **GIVEN** an accepted, sealed ADR `0007` whose frontmatter holds
  `review_by: 2027-01-01` on line 6, and no other finding
- **WHEN** the user runs `codev validate --json`
- **THEN** the `decisions` item holds a `warning` finding
  `decision_unknown_field` naming `review_by` at line 6 of the ADR's file,
  and `codev self-update`
- **AND** the exit code is 0
- **AND** `codev decision list --json` lists `0007` as `accepted`

#### Scenario: Strict validation catches a misspelled field

- **GIVEN** the same project
- **WHEN** the user runs `codev validate --strict`
- **THEN** the exit code is 1

#### Scenario: An inherited decision's unknown field is not reported

- **GIVEN** a project inheriting from a local source whose accepted ADR
  `0100` holds the frontmatter field `review_by: 2027-01-01`, and no other
  finding
- **WHEN** the user runs `codev validate --strict --json`
- **THEN** no `decision_unknown_field` finding is reported and the exit
  code is 0
- **AND** `codev decision list --json` lists `path:<source>/0100` as
  `accepted`

### Requirement: `decision accept` accepts and seals a proposed decision

`codev decision accept <id>` SHALL turn a local decision whose status is
`proposed` into an `accepted` decision and seal its body; it is the
single command through which a decision created by `decision new`,
`decision supersede`, `decision deviate` or `decision promote` takes
effect. The operation MUST produce a single effect plan that rewrites
the frontmatter `status` to `accepted` **and** writes the decision's
entry in `_codev/decisions/seal.yaml`; if the plan cannot be computed,
nothing is written. The body of the ADR — everything after the
frontmatter — MUST stay byte for byte identical, so the recorded hash is
the hash of the text the user wrote. A frontmatter field codev does not
know MUST be kept, with its value as written, in every frontmatter the
command rewrites.

When the decision carries a `supersedes`, the same plan MUST also
rewrite the frontmatter of each decision it lists so that its `status`
becomes `superseded`; the body of each predecessor stays byte for byte
identical and its seal entry is left unchanged.

The command MUST refuse, without writing anything:

- an unknown identifier, with the stable code `unknown_decision_id`;
- an inherited decision, with the stable code `cannot_accept_inherited`,
  since inherited decisions are read-only;
- a decision whose status is not `proposed`, with the stable code
  `decision_not_proposed`; for an `accepted` decision the message says
  there is nothing to do and points to `codev decision supersede` to
  replace it;
- a decision one of whose predecessors cannot be superseded: a
  predecessor that is not indexed, with the stable code
  `unknown_decision_id`; an inherited predecessor, with the stable code
  `cannot_supersede_inherited`; a predecessor whose status is no longer
  `accepted` — for example because another decision superseded it in the
  meantime — with the stable code `predecessor_not_accepted`, whose
  message names the predecessor and its status.

With `--json`, the command MUST write a document carrying the accepted
`decision`, the `path` of its file, its `bodySha256`, and a `superseded`
array listing each predecessor marked superseded (its `id`,
`qualifiedId` and `path`) — empty when the decision supersedes nothing.

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
  prefixed with `sha256:`, `superseded: []`, and `status: []`

#### Scenario: Accepting a supersession marks the predecessor superseded

- **GIVEN** a project containing a sealed ADR `0003 accepted` and a
  `proposed` ADR `0007` with `supersedes: ["0003"]`, created by
  `codev decision supersede 0003 "New choice"`
- **WHEN** the user runs `codev decision accept 0007 --json`
- **THEN** `0007` carries `status: accepted` and is sealed
- **AND** the frontmatter of `0003` now carries `status: superseded`,
  its body is identical down to the character, and its seal entry is
  unchanged
- **AND** `0003` is marked superseded by `project/0007` and is no longer
  in effect
- **AND** the JSON document carries `superseded: [{ id: "0003",
  qualifiedId: "project/0003", path: "…" }]`
- **AND** `codev validate` reports no finding for `0003` nor `0007`

#### Scenario: Refusal when the predecessor is no longer accepted

- **GIVEN** a project containing an ADR `0003 accepted` and two
  `proposed` ADRs `0007` and `0008`, both with `supersedes: ["0003"]`
- **AND** the user has already run `codev decision accept 0007`, so
  `0003` now carries `status: superseded`
- **WHEN** the user runs `codev decision accept 0008`
- **THEN** no write takes place: `0008` still carries `status:
  proposed` and `seal.yaml` is unchanged
- **AND** the error names the stable code `predecessor_not_accepted`,
  `0003` and its status `superseded`

#### Scenario: Refusal when the predecessor is inherited

- **GIVEN** a project inheriting from a source containing
  `path:~/shared/0100`, and a local `proposed` ADR `0007` written by hand
  with `supersedes: ["0100"]`, `0100` existing only in the source
- **WHEN** the user runs `codev decision accept 0007`
- **THEN** no write takes place
- **AND** the error names the stable code `cannot_supersede_inherited`

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

#### Scenario: Fields codev does not know survive the acceptance

- **GIVEN** a local `proposed` ADR `0007` whose frontmatter holds
  `review_by: 2027-01-01`, and `supersedes: ["0003"]` where `0003` is an
  accepted ADR whose frontmatter holds `owner: platform-team`
- **WHEN** the user runs `codev decision accept 0007`
- **THEN** the frontmatter of `0007` carries `status: accepted` and still
  holds `review_by: 2027-01-01`
- **AND** the frontmatter of `0003` carries `status: superseded` and still
  holds `owner: platform-team`

## ADDED Requirements

### Requirement: A seal file of a newer format is refused with a way out

`_codev/decisions/seal.yaml` is written by codev, and its `version` is its
format: a newer codev that adds a field or a value to it raises the
`version`. Every command that reads `seal.yaml` — `codev validate`,
`codev decision new`, `codev decision accept`, `codev decision seal` —
SHALL refuse a file whose `version` is greater than the one it reads, with
exit code 1 and the code `seal_version_unsupported`, writing nothing. The
message MUST name the version found and the version this codev reads,
quote the project's `codev_version` when set, and point to
`codev self-update`. A field the file's version does not define still
makes it unreadable, with the code `seal_invalid`: an unknown field in a
seal file of a known version is a corruption, not a newer format.

#### Scenario: A seal file from a newer codev

- **GIVEN** a project whose `_codev/decisions/seal.yaml` holds `version: 2`
- **WHEN** the user runs `codev validate --json`
- **THEN** the exit code is 1 and `status[0].code` is
  `seal_version_unsupported`
- **AND** the message names `2`, `1` and `codev self-update`

#### Scenario: Nothing is sealed against a newer seal file

- **GIVEN** a project whose `seal.yaml` holds `version: 2` and a local
  `proposed` ADR `0007`
- **WHEN** the user runs `codev decision accept 0007`
- **THEN** the command fails with `seal_version_unsupported`
- **AND** the ADR and `seal.yaml` are unchanged

#### Scenario: The other sealing commands refuse a newer seal file

- **GIVEN** a project whose `seal.yaml` holds `version: 2` and an accepted
  local ADR `0003`
- **WHEN** the user runs `codev decision seal 0003`, or
  `codev decision new "A choice" --status accepted`
- **THEN** each command fails with `seal_version_unsupported`
- **AND** no ADR is created and `seal.yaml` is unchanged

#### Scenario: An unknown field in a seal file of a known version

- **GIVEN** a `seal.yaml` with `version: 1` whose entry holds the field
  `sealedBy`
- **WHEN** the user runs `codev decision accept 0007`
- **THEN** the command fails with `seal_invalid` and writes nothing
