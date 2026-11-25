## ADDED Requirements

### Requirement: `review` is a known configuration key

`review` SHALL be a known top-level key of `_codev/config.yaml`: it never
produces an `unknown_config_key` warning. A key under `review:` other
than `mode` SHALL be ignored with an `unknown_config_key` warning naming
it `review.<key>`, as a key of `mcp` is, and `codev validate` and
`codev doctor` report it the same way.

#### Scenario: No warning for `review`

- **GIVEN** a configuration with `review: { mode: isolated }`
- **WHEN** the user runs `codev status --json`
- **THEN** `status` holds no `unknown_config_key` warning

#### Scenario: An unknown key under `review`

- **GIVEN** a configuration with `review: { mode: inline, budget: 3 }`
- **WHEN** the user runs `codev status --json`
- **THEN** the exit code is 0 and `status` holds an `unknown_config_key`
  warning naming `review.budget`
