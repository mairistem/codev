## ADDED Requirements

### Requirement: The `propose` skill reads the existing requirements its instructions name

The `codev-propose` skill SHALL tell the agent, before it writes the delta
specs and the design, to read each requirement its instructions list among
the existing requirements the change likely touches, and to modify, in a
`MODIFIED` block copied whole from its spec, every one the change alters,
rather than add a requirement beside it. It SHALL say that the list is a
starting point ranked by relevance, not a complete one: a requirement it
does not name may still be concerned, and that with no listed
requirement it works from the request and the code. The skill SHALL name
no search command the agent must run: it may look further as it sees fit.

#### Scenario: The propose skill reads what its instructions name

- **GIVEN** the `codev-propose` skill as `codev update` renders it
- **WHEN** its step that writes each artifact is read
- **THEN** it tells the agent to read the existing requirements its
  instructions list, and to modify those the change alters rather than add
  new ones beside them
- **AND** it says the list is a starting point, not a complete one

#### Scenario: No search command is imposed

- **GIVEN** the `codev-propose` skill as `codev update` renders it
- **WHEN** its text is read
- **THEN** it does not require the agent to run `codev search`

#### Scenario: Instructions without related requirements

- **GIVEN** the `codev-propose` skill as `codev update` renders it
- **WHEN** its step that writes each artifact is read
- **THEN** it says that with no listed requirement the agent works from the
  request and the code
