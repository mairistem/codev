## ADDED Requirements

### Requirement: The `propose` skill locates the existing behavior with `codev search`

The `codev-propose` skill, at `plan.investigate`, SHALL tell the agent to locate the requirements and decisions a
request concerns with `codev search`, its query written in the language of
the specs whatever the language of the request, and to search again with
other words when the results do not name what the request is about. It
SHALL tell it to read the requirements the results name with `codev show
<capability>::<requirement>`, and the whole spec of a capability only when
the change rewrites a large part of it, and SHALL say that the search
points to where to read and never replaces reading: a requirement to modify
is read before its delta is written. A requirement the plan modifies SHALL
be copied whole into the delta from what the agent read, as the delta rules
already require.

The skill SHALL keep `codev list --specs` as the way to see every
capability, and SHALL not require `codev search` when the project has no
specs yet.

#### Scenario: The propose skill searches before writing

- **GIVEN** the `codev-propose` skill as `codev update` renders it
- **WHEN** its `plan.investigate` step is read
- **THEN** it tells the agent to run `codev search` with words in the
  language of the specs, then to read the requirements named with
  `codev show`
- **AND** it says the search never replaces reading the requirement to
  modify

#### Scenario: A request in French on English specs

- **GIVEN** a project whose specs are written in English
- **WHEN** the `codev-propose` skill plans a request written in French
- **THEN** it tells the agent to write its `codev search` query in English

#### Scenario: A project without specs

- **GIVEN** a project whose `_codev/specs/` is empty
- **WHEN** the `codev-propose` skill investigates
- **THEN** it does not require a search, and goes on with the code and the
  request
