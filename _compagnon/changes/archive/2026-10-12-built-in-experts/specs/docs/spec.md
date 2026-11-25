## MODIFIED Requirements

### Requirement: The Roles chapter lists exactly the catalog's roles

The Roles chapter of each language SHALL hold a table of the built-in
roles with one row per role of the catalog — the built-in step roles, the
experts and the artifact roles of the default `spec-driven` schema —
giving its id, its display name in that language, and its step or steps.
The table MUST NOT list any other role. The chapter SHALL also describe
how to configure roles: the short form, extra criteria, the per-change
override, and which roles cannot be disabled; and the experts: the domain
of each, the `## Domains` section that activates them, the `on_domain`
activation, and the sections of a role's knowledge.

#### Scenario: The catalog and the documentation agree

- **GIVEN** the role catalog and the `roles.md` chapters of `docs/en` and
  `docs/fr`
- **WHEN** the test suite of `codev-cli` runs
- **THEN** it fails if a role of the catalog is missing from either table,
  or listed without its display name in that language

#### Scenario: A role that does not exist

- **GIVEN** a `roles.md` whose table of built-in roles lists an id the
  catalog does not hold
- **WHEN** the test suite of `codev-cli` runs
- **THEN** it fails and names that id

#### Scenario: Every expert is documented with its domain

- **GIVEN** the role catalog and the `roles.md` chapters of `docs/en` and
  `docs/fr`
- **WHEN** the test suite of `codev-cli` runs
- **THEN** it fails if an expert's row does not name its domain
