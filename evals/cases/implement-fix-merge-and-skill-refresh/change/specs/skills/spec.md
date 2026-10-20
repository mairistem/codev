## MODIFIED Requirements

### Requirement: Frontmatter contract of a codev skill

Every skill shipped by codev MUST carry a valid YAML frontmatter whose
`name` matches the name of the `.claude/skills/<name>/` directory, whose
`allowed-tools` field includes at least `Bash(codev:*)`, and whose
`metadata.version` matches the version of the binary that generated it,
and whose `metadata.checksum` is the SHA-256, in hexadecimal, of the
skill's whole content with that `checksum` line left out. A skill whose
content no longer matches its checksum has been edited by hand.

#### Scenario: Frontmatter parsed by a third-party YAML reader

- **GIVEN** a skill shipped by the current version of the binary
- **WHEN** its frontmatter is extracted and passed to a standard YAML parser
- **THEN** the parser returns `name`, `allowed-tools`, `metadata.version`
  and `metadata.checksum` without error
- **AND** `metadata.version` equals the version the binary reports

#### Scenario: Hand edit detected on update

- **GIVEN** a skill whose body a user has edited by hand, without
  changing its version
- **WHEN** the user runs `codev update` again without `--force`
- **THEN** the skill is not overwritten
- **AND** the update report flags it as preserved

#### Scenario: The checksum matches the content

- **GIVEN** a skill as the binary renders it
- **WHEN** the SHA-256 of its content without the `checksum` line is
  computed
- **THEN** it equals `metadata.checksum`

### Requirement: `codev update` regenerates the skills of the configured workflows

`codev update` SHALL write one skill per workflow listed by `workflows:` in
`_codev/config.yaml`, or per workflow of the default catalog when the key is
absent, at `.claude/skills/codev-<workflow>/SKILL.md`, rendered by the
running codev:

- a missing skill is created;
- a skill identical to the rendering is left untouched;
- a skill stamped with another codev version is rewritten;
- among the skills stamped with the running version, one whose content
  matches its own checksum is codev's rendering and is rewritten; one
  whose content no longer matches its checksum is edited by hand and is
  preserved; one without a checksum, rendered before checksums, is
  preserved as edited by hand unless it differs from codev's rendering
  only by the review mode, as the review-mode requirement says, the
  checksum line left out of the comparison;
- with `--force`, every skill is rewritten.

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

#### Scenario: A skill rendered by an earlier build of the same version

- **GIVEN** a project whose `codev-apply` skill was rendered by a build
  of the same version whose skill text differed, untouched since, with a
  `checksum` line matching its content
- **WHEN** the user runs `codev update --json`
- **THEN** `updated` lists the `codev-apply` skill and `preserved` is
  empty

#### Scenario: A hand edit of a checksummed skill

- **GIVEN** a project whose `codev-apply` skill a user edited by hand
  after codev rendered it with its checksum
- **WHEN** the user runs `codev update --json`
- **THEN** `preserved` lists the `codev-apply` skill, left as edited

#### Scenario: A hand edit survives no upgrade

- **GIVEN** a project whose `codev-apply` skill a user edited by hand,
  stamped with an earlier codev version
- **WHEN** the user runs `codev update --json` with a newer codev
- **THEN** `updated` lists the `codev-apply` skill, as before checksums

#### Scenario: A skill rendered before checksums

- **GIVEN** a project whose `codev-apply` skill has no `checksum` line,
  is stamped with the running version and differs from its rendering
- **WHEN** the user runs `codev update --json`
- **THEN** `preserved` lists it, as before checksums existed
