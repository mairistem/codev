## ADDED Requirements

### Requirement: `codev update` regenerates the skills of the configured workflows

`codev update` SHALL write one skill per workflow listed by `workflows:` in
`_codev/config.yaml`, or per workflow of the default catalog when the key is
absent, at `.claude/skills/codev-<workflow>/SKILL.md`, rendered by the
running codev:

- a missing skill is created;
- a skill stamped with another codev version is rewritten;
- a skill identical to the rendering is left untouched;
- a skill edited by hand is preserved as the skill frontmatter requirement
  states, unless `--force` is given, in which case it is rewritten.

An unknown workflow name SHALL be ignored with a warning `unknown_workflow`
listing the workflows this codev provides; the other skills are written.
The skill of a workflow no longer listed SHALL be left in place: `codev
update` deletes no skill. `codev update` SHALL also recreate the missing
folders of `_codev/` and a missing `_codev/config.yaml`, without
overwriting any existing file. Its handling of codev's managed sections is
specified by the `init` capability. Outside a codev project it fails with
`no_codev_root` and writes nothing.

The human output SHALL start with `codev updated in <project root>`, list
the installed skills, then say how many files were created and updated, or
`Nothing to do, everything is already in place.`, and finally list the
preserved skills with a hint to rerun with `--force`. With `--json`, the
document SHALL carry `root`, `created`, `updated`, `untouched`,
`preserved`, `skills` (the names of the configured skills), `integration`
and `status` (warnings). The command exits with code 0.

#### Scenario: Missing and outdated skills are written

- **GIVEN** an initialized project whose `codev-explore` skill was deleted
  and whose `codev-sync` skill is stamped with an earlier codev version
- **WHEN** the user runs `codev update --json`
- **THEN** `created` lists the `codev-explore` skill and `updated` the
  `codev-sync` skill
- **AND** both carry the running codev's version stamp

#### Scenario: A second run changes nothing

- **GIVEN** a project where `codev update` just ran
- **WHEN** the user runs `codev update`
- **THEN** `created` and `updated` are empty
- **AND** the human output says `Nothing to do, everything is already in
  place.`

#### Scenario: `--force` rewrites a hand-edited skill

- **GIVEN** a project whose `codev-apply` skill was edited by hand without
  changing its version stamp
- **WHEN** the user runs `codev update --force --json`
- **THEN** `updated` lists the `codev-apply` skill and `preserved` is empty
- **AND** the skill is back to what codev renders

#### Scenario: An unknown workflow is ignored with a warning

- **GIVEN** a configuration with `workflows: [propose, bogus]`
- **WHEN** the user runs `codev update --json`
- **THEN** `skills` is `["codev-propose"]`
- **AND** `status` holds a warning `unknown_workflow` naming `bogus`

#### Scenario: A workflow removed from the configuration keeps its skill

- **GIVEN** a project where every default skill is installed, then
  `workflows: [propose]` in the configuration
- **WHEN** the user runs `codev update`
- **THEN** `.claude/skills/codev-apply/SKILL.md` still exists, unchanged

#### Scenario: A deleted `_codev/` folder is recreated

- **GIVEN** an initialized project whose `_codev/specs/` folder was deleted
- **WHEN** the user runs `codev update --json`
- **THEN** `created` lists `_codev/specs/.gitkeep`
- **AND** `_codev/config.yaml` is listed in `untouched`, unchanged

#### Scenario: Outside a project

- **GIVEN** a folder with no `_codev/` in it or its parents
- **WHEN** the user runs `codev update --json`
- **THEN** the exit code is 1 and `status[0].code` is `no_codev_root`
- **AND** `root` is `null` and `skills` is empty
