## MODIFIED Requirements

### Requirement: Supersession resolved as a chain

The index MUST resolve the `supersedes` field: each decision that an
`accepted` ADR supersedes is marked `superseded_by(<id>)` in the index,
and is not in effect. A chain `A ← B ← C` leaves `A` and `B` superseded;
only `C` remains in effect. The `supersedes` of an ADR that is not
`accepted` — a `proposed` supersession not accepted yet, in particular —
has no effect: the decisions it lists keep their place in the index.

#### Scenario: Direct supersession

- **GIVEN** an ADR `0003 accepted` and an ADR `0007 accepted supersedes: [0003]`
- **WHEN** the index is computed
- **THEN** `0003` is marked `superseded_by(0007)` and does not appear among
  the decisions in effect
- **AND** `0007` appears among the decisions in effect

#### Scenario: Three-link chain

- **GIVEN** three `accepted` ADRs where `B` supersedes `A` and `C` supersedes `B`
- **WHEN** the index is computed
- **THEN** only `C` is in effect

#### Scenario: Supersession target missing

- **GIVEN** an ADR `0007 supersedes: [9999]` while `9999` does not exist
- **WHEN** the index is computed
- **THEN** a finding with code `decision_supersedes_unknown` reports
  the phantom identifier
- **AND** `0007` remains in effect (the lost link does not disqualify it)

#### Scenario: A proposed supersession has no effect yet

- **GIVEN** an ADR `0003 accepted` and an ADR `0007 proposed
  supersedes: [0003]`
- **WHEN** the index is computed
- **THEN** `0003` appears among the decisions in effect and carries no
  `superseded_by`
- **AND** `0007` does not appear among the decisions in effect

### Requirement: Superseding a decision through the CLI

`codev decision supersede <old-id> <new-title>` MUST create a new local
decision with `status: proposed` that references the old one in its
`supersedes`. The command MUST NOT modify the old decision: its
frontmatter, its body and its seal entry stay as they are, and it
remains in effect until the new decision is accepted with
`codev decision accept`, which marks it `superseded`. The new decision
is not sealed, so that its body can be written before it is accepted.

The command MUST refuse, without writing anything:

- an unknown identifier, with the stable code `unknown_decision_id`;
- an inherited decision, with the stable code
  `cannot_supersede_inherited`, pointing to `codev decision deviate`;
- a local decision whose status is not `accepted`, with the stable code
  `predecessor_not_accepted` — only a decision in effect can be
  superseded, and the new decision could not be accepted anyway.

#### Scenario: Local supersession

- **GIVEN** a project containing an ADR `0003 accepted` titled "Old
  choice"
- **WHEN** the user runs `codev decision supersede 0003 "New
  choice"`
- **THEN** a new ADR `0007-new-choice.md` (or the next available `id`)
  is created with `supersedes: ["0003"]` and `status: proposed`
- **AND** the `0003` file is identical down to the character, its
  frontmatter still carrying `status: accepted`
- **AND** `0003` is still among the decisions in effect
- **AND** the output names `codev decision accept 0007` as the next step

#### Scenario: Superseding an unknown id rejects everything

- **GIVEN** a project in which `9999` does not exist
- **WHEN** the user runs `codev decision supersede 9999 "X"`
- **THEN** no file is written
- **AND** the `unknown_decision_id` error message names `9999`

#### Scenario: Superseding an inherited decision rejected

- **GIVEN** a project inheriting from a source containing `path:~/shared/0100`
- **WHEN** the user runs `codev decision supersede path:~/shared/0100
  "Our alternative"`
- **THEN** no file is written
- **AND** the `cannot_supersede_inherited` error message explains that an
  inherited decision is read-only and suggests deviating from it with
  `codev decision deviate`

#### Scenario: Superseding a decision that is not accepted rejected

- **GIVEN** a project containing an ADR `0004 proposed`
- **WHEN** the user runs `codev decision supersede 0004 "X"`
- **THEN** no file is written
- **AND** the error names the stable code `predecessor_not_accepted`
  and the status of `0004`

### Requirement: Accepting a supersession seals the new ADR without touching the old one's seal

`codev decision supersede <id> <title>` MUST NOT write
`_codev/decisions/seal.yaml`: the ADR it creates is `proposed`. When
that ADR is accepted with `codev decision accept`, the operation MUST
add a seal entry for it and MUST NOT modify the seal entry of the old
one — since the old one's body remains identical down to the character
(a requirement already in effect), its seal remains valid.

#### Scenario: Supersession writes no seal

- **GIVEN** a project containing a sealed ADR `0003 accepted` referenced
  in `seal.yaml` under `id: "0003"`
- **WHEN** the user runs `codev decision supersede 0003 "New choice"`
- **THEN** `seal.yaml` is identical down to the character

#### Scenario: Acceptance seals only the new one

- **GIVEN** the same project, after `codev decision supersede 0003
  "New choice"` created `0007`
- **WHEN** the user runs `codev decision accept 0007`
- **THEN** the `seal.yaml` file now contains two entries: the one for
  `0003` (unchanged) and a new one for `0007`
- **AND** the `0003` entry has exactly the same `bodySha256` as before the
  supersession

### Requirement: The `codev decision deviate` command creates a deviation ADR

`codev decision deviate <qualified-id> <title>` MUST create a local
`proposed` ADR with `deviates_from: ["<qualified-id>"]` and a valid
frontmatter (`id`, `title`, `status: proposed`, `date`), without a seal
entry, so that its body can be written before it is accepted. The
deviation takes effect — the inherited decision is set aside — only once
the ADR is accepted with `codev decision accept`, which seals it like
every accepted decision.

The command MUST refuse:

- if `<qualified-id>` designates a local decision (`project/…`): stable
  code `cannot_deviate_from_local`, with a message pointing to
  `codev decision supersede`.
- if `<qualified-id>` matches no indexed decision: stable code
  `unknown_decision_id` (already existing).
- if `<title>` is empty: stable code `empty_title` (already existing).

#### Scenario: Deviation from an inherited `path:` decision

- **GIVEN** a project inheriting from a source `path: ~/shared` containing
  an ADR `path:~/shared/0100`
- **WHEN** the user runs
  `codev decision deviate path:~/shared/0100 "Our local alternative"`
- **THEN** a new local ADR is created under
  `_codev/decisions/NNNN-our-local-alternative.md` with
  `deviates_from: ["path:~/shared/0100"]` and `status: proposed`
- **AND** `_codev/decisions/seal.yaml` is neither created nor modified
- **AND** `path:~/shared/0100` is still in effect and carries no
  `deviatedBy`
- **AND** the output names `codev decision accept NNNN` as the next step

#### Scenario: The deviation takes effect once accepted

- **GIVEN** the same project, after the deviation `NNNN` was created
- **WHEN** the user runs `codev decision accept NNNN`
- **THEN** `NNNN` carries `status: accepted` and an entry is added to
  `_codev/decisions/seal.yaml` for it
- **AND** `codev decision list --json` shows `path:~/shared/0100` with
  `deviatedBy: "project/NNNN"` and `inEffect: false`

#### Scenario: Refusal to deviate from a local decision

- **GIVEN** a project containing a local ADR `0003 accepted`
- **WHEN** the user runs `codev decision deviate project/0003 "…"`
- **THEN** no file is written
- **AND** the error message names the stable code
  `cannot_deviate_from_local` and suggests `codev decision supersede`

#### Scenario: Refusal of an unknown target

- **GIVEN** a project without any inherited source
- **WHEN** the user runs
  `codev decision deviate path:~/unknown/0100 "…"`
- **THEN** no file is written
- **AND** the error message names the stable code `unknown_decision_id`

### Requirement: The `codev decision promote` command extracts a design block into an ADR

`codev decision promote <change> <title>` MUST create a new local ADR
under `_codev/decisions/NNNN-<slug>.md`, with `status: proposed` and
without a seal entry, whose body reproduces **verbatim** the content of
the `### Decision: <title>` block found under the `## Decisions` section
of the change's `design.md`. The new ADR contains a `## Decision` section
carrying that body, and the `## Context`, `## Consequences` and
`## Alternatives considered` sections are emitted with a
`<!-- placeholder -->` inviting the author to fill them in. The ADR is
meant to be reviewed and reworked, then accepted — and sealed — with
`codev decision accept`; the output of the command says so.

The "verbatim" body of the block means: all the text that follows the
`### Decision: <title>` line up to the next line starting with `### ` or
`## ` (excluded), without normalization.

Explicit refusals — stable codes:

- `unknown_change`: the change is not in `codev list`.
- `cannot_promote_from_archived`: the target points to a directory under
  `changes/archive/` — an archived design is history.
- `design_missing`: the change has no `design.md`.
- `decision_heading_not_found`: no `### Decision: <title>` block matches
  in the design.
- `ambiguous_decision_heading`: several blocks carry the same title —
  the user disambiguates by editing.

#### Scenario: Successful promotion

- **GIVEN** a change `add-auth` whose `design.md` contains, under
  `## Decisions`, a `### Decision: Use JWT` block with two
  paragraphs of rationale
- **WHEN** the user runs `codev decision promote add-auth
  "Use JWT"`
- **THEN** a new ADR is created under
  `_codev/decisions/NNNN-use-jwt.md` with `status: proposed`, the current
  `date`, and a valid frontmatter
- **AND** its body contains a `## Decision` section with the two
  paragraphs of rationale, byte for byte
- **AND** `_codev/decisions/seal.yaml` is neither created nor modified
- **AND** the output invites the user to review the ADR, then run
  `codev decision accept NNNN`

#### Scenario: The promoted ADR can be reworked, then accepted

- **GIVEN** the ADR `NNNN` created by the promotion
- **AND** the user has split its body across Context, Decision,
  Consequences and Alternatives considered
- **WHEN** the user runs `codev decision accept NNNN`, then
  `codev validate`
- **THEN** `NNNN` carries `status: accepted` and is sealed with its
  reworked body
- **AND** `codev validate` reports neither `decision_seal_mismatch` nor
  `decision_unsealed` for `NNNN`

#### Scenario: Refusal of an archived change

- **GIVEN** a change that lives under `_codev/changes/archive/…-<name>/`
- **WHEN** the user runs `codev decision promote <name> "..."`
- **THEN** no file is written
- **AND** the error message names the stable code
  `cannot_promote_from_archived`

#### Scenario: Title not found

- **GIVEN** a change `add-auth` whose `design.md` does not mention a
  "Use Kerberos" block
- **WHEN** the user runs `codev decision promote add-auth
  "Use Kerberos"`
- **THEN** no file is written
- **AND** the error message names the stable code
  `decision_heading_not_found`

#### Scenario: Ambiguous title

- **GIVEN** a `design.md` containing **two**
  `### Decision: Library choice` blocks (for example a revised version
  of the first block written during the discussion)
- **WHEN** the user runs `codev decision promote <c> "Library
  choice"`
- **THEN** no file is written
- **AND** the error message names the stable code
  `ambiguous_decision_heading` and invites the user to edit one of the
  two titles

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
the hash of the text the user wrote.

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

## RENAMED Requirements

- FROM: `### Requirement: `decision supersede` seals the new ADR without touching the old one's seal`
- TO: `### Requirement: Accepting a supersession seals the new ADR without touching the old one's seal`
