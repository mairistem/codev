## MODIFIED Requirements

### Requirement: `onboard` is part of the default catalog

The `DEFAULT_WORKFLOWS` array of `codev-agents::workflows` MUST
contain the **full list of the 7 codev workflows**: `propose`,
`explore`, `onboard`, `apply`, `sync`, `archive` and `update`. A user
who runs `codev init --yes` (or from a non-interactive pipe) on a new
project, without a `workflows:` key in their `config.yaml`, therefore
gets all the available skills immediately.

A project that wants to restrict the catalog MUST declare an explicit
`workflows:` key with a chosen subset — this is the opt-out path,
rather than the former opt-in path.

This switch solves a discovery problem: under the old default (3
workflows), a user who typed `/codev-apply` after `/codev-propose`
did not find the skill and believed it did not exist.

#### Scenario: Default catalog includes the 7 workflows

- **GIVEN** a project whose `config.yaml` has no `workflows:` key
- **WHEN** `select(None)` is called on the catalog
- **THEN** the list of returned `id`s is exactly
  `["propose", "explore", "onboard", "apply", "sync", "archive", "update"]`
- **AND** no warning is emitted

#### Scenario: Opt-out restriction via explicit workflows

- **GIVEN** a project whose `config.yaml` contains
  `workflows: [propose, explore, onboard]`
- **WHEN** `select` is called with that list
- **THEN** only those three skills are returned
- **AND** `apply`, `sync`, `archive`, `update` are **not** installed
