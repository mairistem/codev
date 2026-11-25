## MODIFIED Requirements

### Requirement: A git source is never read from a floating branch

The validator MUST refuse to expose the content of a `git:` source until
a SHA has been locked in `_codev/codev.lock`. No everyday command
(`list`, `show`, `status`, `instructions`, `validate`, `sync`, `archive`)
SHALL contact an inherited source's remote — only `codev sources update`
moves a pin. The daily update check of `codev status`, a request to the
codev release API that never involves an inherited source, is the only
network access of an everyday command.

#### Scenario: Git source declared but not locked

- **GIVEN** a project declaring `inherits: [{git: "…", ref: main}]`
- **AND** a `codev.lock` that is missing or has no entry for this source
- **WHEN** the user runs `codev instructions design --change <name>`
- **THEN** the `decisions` array contains only the local decisions
- **AND** the `status[]` field of the response carries a warning with the
  stable code `git_source_unlocked` inviting the user to run
  `codev sources update`

#### Scenario: `codev status` does not contact the network

- **GIVEN** a project declaring a `git:` source with a locked SHA
  that is not in the cache
- **WHEN** the user runs `codev status --change <name>` while the
  network is unavailable
- **THEN** the command does not fail for network reasons
- **AND** no `git` call is made during execution
