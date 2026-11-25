## ADDED Requirements

### Requirement: `codev decision promote` command extracts a design block into an ADR

`codev decision promote <change> <title>` MUST create a new local ADR
under `_codev/decisions/NNNN-<slug>.md`, with `status: accepted`,
sealed by K3, whose body reproduces **verbatim** the content of the
`### Decision: <title>` block found under the `## Decisions` section of
the change's `design.md`. The new ADR contains a `## Decision` section
carrying that body, and the `## Context`, `## Consequences` and
`## Rejected Alternatives` sections are emitted with a
`<!-- placeholder -->` inviting the author to split the content.

The "verbatim" body of the block means: all the text that follows the
`### Decision: <title>` line up to the next line starting with `### `
or `## ` (excluded), without normalization.

Explicit refusals — stable codes:

- `unknown_change`: the change is not in `codev list`.
- `cannot_promote_from_archived`: the target points to a folder under
  `changes/archive/` — an archived design is history.
- `design_missing`: the change has no `design.md`.
- `decision_heading_not_found`: no `### Decision: <title>` block
  matches in the design.
- `ambiguous_decision_heading`: several blocks carry the same title —
  the user clarifies by editing.

#### Scenario: Successful promotion

- **GIVEN** a change `add-auth` whose `design.md` contains, under
  `## Decisions`, a block `### Decision: Use JWT` with two paragraphs
  of rationale
- **WHEN** the user runs `codev decision promote add-auth
  "Use JWT"`
- **THEN** a new ADR is created under
  `_codev/decisions/NNNN-use-jwt.md` with `status: accepted`, today's
  `date`, and a valid frontmatter
- **AND** its body contains a `## Decision` section with the two
  paragraphs of rationale, byte for byte
- **AND** an entry is added to `_codev/decisions/seal.yaml` for this
  ADR — consistent with K3

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

- **GIVEN** a `design.md` containing **two** blocks
  `### Decision: Library choice` (for example a revised version of the
  first block during the discussion)
- **WHEN** the user runs `codev decision promote <c> "Library
  choice"`
- **THEN** no file is written
- **AND** the error message names the stable code
  `ambiguous_decision_heading` and invites editing one of the two
  titles

### Requirement: The design is updated with a traceable reference

After a successful promotion, the `### Decision: <title>` block of the
`design.md` MUST be replaced by the same title followed by **a single
line** of textual quote referencing the new ADR:

```
### Decision: <title>

> Promoted to ADR **NNNN** — see `_codev/decisions/NNNN-<slug>.md`.
```

The reference is plain text (`` ` `` for the path), not a markdown
link — a `[..](../..)` link would break when the change is archived
(where the depth of the `..` changes). The block's title is preserved
so that a reader of the design can understand the subject that was
promoted.

#### Scenario: Block content replaced by the reference

- **GIVEN** the same context as the "Successful promotion" scenario
- **WHEN** the user runs the command
- **THEN** the change's `design.md` now contains, at the location of
  the block:
  ```
  ### Decision: Use JWT

  > Promoted to ADR **NNNN** — see `_codev/decisions/NNNN-use-jwt.md`.

  ```
- **AND** the rest of the file (other sections, other `###` blocks,
  spacing) is identical to the character
- **AND** the spacing around the block stays preserved — no blank line
  added or removed

#### Scenario: Two successive promotions on the same design

- **GIVEN** a design containing two distinct blocks:
  `### Decision: A` and `### Decision: B`
- **AND** the user has already promoted `A` to an ADR
- **WHEN** the user runs `codev decision promote <c> "B"`
- **THEN** block `B` is promoted in turn
- **AND** the reference line of `A` is NOT altered by this second
  promotion — each promotion only acts on its own block
