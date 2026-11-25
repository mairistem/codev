## ADDED Requirements

### Requirement: The Roles chapter documents triggers, suggestions and the stack filter

The Roles chapter of each language SHALL describe the three kinds of
trigger (`paths`, `manifests`, `patterns`), the built-in triggers of
each expert in outline, the change's diff and what it leaves out, the
states of a suggestion and how to justify an unticked domain, the
warning `domain_suggested`, the project's `triggers:` key, and the stack
filter with `--all-stacks`. The configuration reference of each language
SHALL document the `triggers:` key.

#### Scenario: Both languages document them

- **GIVEN** the `roles.md` and `reference/configuration.md` chapters of
  `docs/en` and `docs/fr`
- **WHEN** the test suite of `codev-cli` runs
- **THEN** it fails if a `roles.md` does not name `paths`, `manifests`,
  `patterns`, `domain_suggested` and `--all-stacks`, or if a
  `configuration.md` has no `triggers` section

#### Scenario: A chapter that forgets the flag

- **GIVEN** a French `roles.md` that does not name `--all-stacks`
- **WHEN** the test suite of `codev-cli` runs
- **THEN** it fails and names the French chapter
