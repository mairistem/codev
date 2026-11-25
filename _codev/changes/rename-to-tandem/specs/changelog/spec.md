## MODIFIED Requirements

### Requirement: `codev changelog` selects the changes archived between two git refs

`tandem changelog` SHALL list the changes present in
`_tandem/changes/archive/` at `--until` — by default, in the working tree —
and absent from it at `--since`. Presence at a ref MUST be read from git,
never from dates. At a ref whose tree holds no `_tandem/` but holds the
former planning root `_codev/`, presence SHALL be read from
`_codev/changes/archive/` at that ref, so that a change archived before the
project was migrated is never listed again. The default `--since` is the
highest `v*` tag by version order, excluding `--until` and any tag of a
higher version when `--until` names a version tag; with no such tag, every
archived change is in range. Changes are listed in archive order. Changes
named under `changelog.exclude` in `_tandem/config.yaml` are left out
without a warning.

Outside a git repository, every archived change of the working tree SHALL
be listed with the warning `not_a_git_repository`; passing `--since` or
`--until` there fails with `not_a_git_repository`. A ref git does not know
fails with `unknown_ref`. A change present at `--until` but missing from
the working tree is skipped with the warning `archived_change_unreadable`.

#### Scenario: Default range since the latest tag

- **GIVEN** a git repository with tags `v0.9.0` and `v0.10.0`, change `a`
  archived before `v0.9.0`, `b` between the two tags, and `c` archived
  after `v0.10.0`, uncommitted
- **WHEN** the user runs `tandem changelog`
- **THEN** only the items of `c` are listed

#### Scenario: Range between two tags

- **GIVEN** the same repository
- **WHEN** the user runs `tandem changelog --until v0.10.0`
- **THEN** only the items of `b` are listed

#### Scenario: A range that spans the migration

- **GIVEN** a git repository tagged `v0.6.1` while its planning root was
  `_codev/`, holding the archived change `a`, then migrated to `_tandem/`,
  then given the archived change `b`
- **WHEN** the user runs `tandem changelog`
- **THEN** only the items of `b` are listed

#### Scenario: No tag yet

- **GIVEN** a git repository with no `v*` tag and two archived changes
- **WHEN** the user runs `tandem changelog`
- **THEN** the items of both changes are listed

#### Scenario: Excluded change

- **GIVEN** a change in range named under `changelog.exclude`
- **WHEN** the user runs `tandem changelog`
- **THEN** none of its items is listed and no warning names it

#### Scenario: Change missing from the working tree

- **GIVEN** a change present at `--until` whose folder was since deleted
- **WHEN** the user runs `tandem changelog --until <ref>`
- **THEN** the warning `archived_change_unreadable` names it

#### Scenario: Outside a git repository

- **GIVEN** a Tandem project that is not a git repository, with two
  archived changes
- **WHEN** the user runs `tandem changelog`
- **THEN** the items of both changes are listed
- **AND** the warning `not_a_git_repository` is reported

#### Scenario: Unknown ref

- **GIVEN** a git repository without a `v9.9.9` ref
- **WHEN** the user runs `tandem changelog --since v9.9.9`
- **THEN** the command fails with the code `unknown_ref`
