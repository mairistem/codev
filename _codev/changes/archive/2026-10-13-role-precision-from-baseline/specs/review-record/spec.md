## ADDED Requirements

### Requirement: A finding presented to the human shows its impact

The review record procedure that the `propose` and `apply` skills carry
SHALL tell every reviewing role, next to the severity and confidence
scales, that:

- a finding of severity `medium` or `high` recorded with confidence
  `high` quotes the line it is about and states the scenario that fails —
  the input, state or operation, and what goes wrong;
- a concern that depends on something the reviewer could not see — load,
  data volume, a caller, a configuration, a later change — and a
  preference with no failure are recorded with confidence `low`;
- what is not a finding: a concern another role owns, code or data the
  change neither adds nor modifies, what the plan explicitly excludes
  unless excluding it breaks one of the change's own requirements, a
  missing test when the role's criteria do not ask for one, and a matter
  of taste or an alternative design when the current one works;
- each defect is reported once, its consequences in its recommendation,
  while two defects that need two different fixes are two findings, even
  at the same place.

#### Scenario: The evidence rule reaches the skills

- **GIVEN** the codev workflow catalog
- **WHEN** the `propose` and `apply` skills are rendered
- **THEN** each says that a `medium` or `high` finding with confidence
  `high` quotes the line and states the scenario that fails
- **AND** that a concern depending on what the reviewer could not see,
  or a preference, is recorded with confidence `low`

#### Scenario: What is not a finding reaches the skills

- **GIVEN** the codev workflow catalog
- **WHEN** the `propose` and `apply` skills are rendered
- **THEN** each lists as not a finding a concern another role owns, code
  the change does not touch, what the plan excludes, a missing test the
  role's criteria do not ask for, and a matter of taste

#### Scenario: One finding per defect

- **GIVEN** the codev workflow catalog
- **WHEN** the `propose` and `apply` skills are rendered
- **THEN** each says that a defect is reported once with its consequences
  in its recommendation, and that two defects needing two fixes are two
  findings even at the same place
