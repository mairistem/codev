## ADDED Requirements

### Requirement: `codev decision promote` finds a decision written in any catalog language

`codev decision promote <change> <title>` SHALL find the block of the
given title written with any catalog language's keywords — a
`### Décision : <title>` block under a `## Décisions` section — and SHALL
apply to it the same verbatim extraction and the same refusals as to a
`### Decision: <title>` block under `## Decisions`.

#### Scenario: Promoting a French decision

- **GIVEN** a change `add-auth` whose `design.md` holds, under
  `## Décisions`, a `### Décision : Utiliser JWT` block
- **WHEN** the user runs `codev decision promote add-auth "Utiliser JWT"`
- **THEN** a new ADR is created whose `## Decision` section carries the
  block's body verbatim

#### Scenario: Same title in both languages is ambiguous

- **GIVEN** a `design.md` holding a `### Decision: Library` block and a
  `### Décision : Library` block
- **WHEN** the user runs `codev decision promote <change> Library`
- **THEN** the command fails with `ambiguous_decision_heading`
