## Purpose

Let a project set up with codev, the product's former name, become a
Tandem project in one explicit command, and make sure Tandem never works
on a codev layout without saying so.

## ADDED Requirements

### Requirement: A codev project is detected and never used silently

When the first ancestor of the working directory holding a planning root
holds `_codev/` and no `_tandem/`, every command that needs a project
SHALL fail with the code `legacy_root`, naming the directory found
and telling the user to run `tandem migrate`. No such command MUST read
or write anything under `_codev/`; `tandem doctor` included, which fails
with the same code. `tandem migrate`, `tandem docs`,
`tandem self-update`, `tandem completions` and `tandem --version` are not
affected. When a directory holds both `_codev/` and `_tandem/`,
`_tandem/` is the project and `_codev/` is ignored.

#### Scenario: A command run in a codev project

- **GIVEN** a directory `/p` holding `_codev/config.yaml` and no `_tandem/`
- **WHEN** the user runs `tandem status --json` from `/p/src`
- **THEN** the exit code is 1 and `status[0].code` is `legacy_root`
- **AND** the message names `/p` and `tandem migrate`
- **AND** nothing under `/p/_codev/` is modified

#### Scenario: Doctor in a codev project

- **GIVEN** a directory holding `_codev/` and no `_tandem/`
- **WHEN** the user runs `tandem doctor`
- **THEN** the command fails with the code `legacy_root`

#### Scenario: A leftover codev root next to a Tandem root

- **GIVEN** a directory holding both `_codev/` and `_tandem/`
- **WHEN** the user runs `tandem status`
- **THEN** the project is read from `_tandem/`

#### Scenario: No planning root at all

- **GIVEN** a directory with neither `_codev/` nor `_tandem/` in its
  ancestors
- **WHEN** the user runs `tandem status --json`
- **THEN** `status[0].code` is `no_tandem_root`

### Requirement: `tandem migrate` converts a codev project in place

`tandem migrate` SHALL, from any directory under a codev project:

- rename `_codev/` to `_tandem/`, keeping every file it holds;
- rename `_tandem/codev.lock` to `_tandem/tandem.lock`;
- rename the key `codev_version` to `tandem_version` in
  `_tandem/config.yaml`, keeping its value, its comments and the rest of
  the file byte for byte;
- rewrite, in the planning files, the markers `<!-- codev-review: N -->`
  and `<!-- codev-corrections: N -->` to their `tandem` form;
- remove the skills `.claude/skills/codev-*/` whose content is the one
  codev generated, and install the `tandem-*` skills of the configured
  workflows;
- replace the managed block of `CLAUDE.md` and the generated
  `.github/workflows/codev.yml` by their Tandem versions
  (`.github/workflows/tandem.yml`), leaving any hand-written content
  outside the managed block untouched.

A `codev-*` skill or a `codev.yml` workflow modified by hand SHALL be
kept and reported as `kept_modified`, never deleted. The command MUST
print every path it moved, wrote, removed or kept, and support `--json`
(contract v1) and `--dry-run`, which prints the same plan and writes
nothing. Run where `_tandem/` exists and `_codev/` does not, it SHALL do nothing, exit 0,
and say the project is already migrated. Run where there is no planning
root at all, it SHALL fail with `no_tandem_root`. When `_codev/` and
`_tandem/` both exist it SHALL fail with `migration_conflict` and modify
nothing.

The text of specs, decisions and changes is the user's: `tandem migrate`
MUST NOT rewrite the prose of the planning files.

#### Scenario: Migrating a codev project

- **GIVEN** a project with `_codev/config.yaml` holding
  `codev_version: ">=0.6"`, `_codev/codev.lock`,
  `.claude/skills/codev-propose/SKILL.md` as codev generated it, and a
  `CLAUDE.md` with a managed block
- **WHEN** the user runs `tandem migrate`
- **THEN** `_tandem/config.yaml` holds `tandem_version: ">=0.6"` and
  `_codev/` no longer exists
- **AND** `_tandem/tandem.lock` exists and `_tandem/codev.lock` does not
- **AND** `.claude/skills/tandem-propose/SKILL.md` exists and
  `.claude/skills/codev-propose/` does not
- **AND** the managed block of `CLAUDE.md` names `/tandem-propose`

#### Scenario: Dry run

- **GIVEN** a codev project
- **WHEN** the user runs `tandem migrate --dry-run --json`
- **THEN** the planned moves, writes and removals are listed
- **AND** no file of the project is modified

#### Scenario: A hand-edited skill is kept

- **GIVEN** a codev project whose `.claude/skills/codev-apply/SKILL.md`
  was edited by hand
- **WHEN** the user runs `tandem migrate`
- **THEN** `.claude/skills/codev-apply/SKILL.md` still exists
- **AND** it is reported as `kept_modified`
- **AND** `.claude/skills/tandem-apply/SKILL.md` is installed

#### Scenario: Already migrated

- **GIVEN** a project with `_tandem/` and no `_codev/`
- **WHEN** the user runs `tandem migrate`
- **THEN** the exit code is 0, nothing is written, and the output says the
  project is already migrated

#### Scenario: Both roots present

- **GIVEN** a directory holding both `_codev/` and `_tandem/`
- **WHEN** the user runs `tandem migrate --json`
- **THEN** the exit code is 1, `status[0].code` is `migration_conflict`
- **AND** no file is modified

#### Scenario: No planning root

- **GIVEN** a directory with neither `_codev/` nor `_tandem/` in its
  ancestors
- **WHEN** the user runs `tandem migrate --json`
- **THEN** the exit code is 1 and `status[0].code` is `no_tandem_root`

#### Scenario: Prose is left alone

- **GIVEN** a codev project whose spec says "run `codev validate`"
- **WHEN** the user runs `tandem migrate`
- **THEN** the moved spec still says "run `codev validate`"

### Requirement: An inherited source still laid out for codev is refused with its fix

When a source declared under `sources:` in `_tandem/config.yaml` holds
`_codev/config.yaml` and no `_tandem/config.yaml`, the commands that read
sources SHALL fail with `legacy_source`, naming the source and
saying that its maintainers must run `tandem migrate` in it. Tandem MUST
NOT read it, and MUST NOT modify it.

#### Scenario: A shared standards repository not migrated yet

- **GIVEN** a Tandem project whose source `standards` is a repository
  holding only `_codev/config.yaml`
- **WHEN** the user runs `tandem validate --json`
- **THEN** `status[0].code` is `legacy_source` and the message names
  `standards` and `tandem migrate`
- **AND** the source's files are unchanged

#### Scenario: A migrated source is read as usual

- **GIVEN** a Tandem project whose source `standards` holds
  `_tandem/config.yaml`, with or without a leftover `_codev/`
- **WHEN** the user runs `tandem validate`
- **THEN** the source is read from its `_tandem/` folder and no
  `legacy_source` is reported

### Requirement: The binary run under its former name says it is Tandem

When the running executable's file name, without extension, is `codev`,
every command SHALL first print on stderr one line saying that codev is
now Tandem and that the executable should be renamed `tandem` or
reinstalled with the installer, then behave exactly as `tandem`. With
`--json`, stdout MUST stay a valid JSON document.

#### Scenario: Run as `codev` after a self-update

- **GIVEN** the Tandem binary installed at `~/.local/bin/codev`
- **WHEN** the user runs `codev --version`
- **THEN** stderr holds one line naming `tandem`
- **AND** stdout holds the Tandem version

#### Scenario: JSON output stays clean

- **GIVEN** the Tandem binary installed under the name `codev`
- **WHEN** the user runs `codev status --json`
- **THEN** stdout parses as a JSON document
