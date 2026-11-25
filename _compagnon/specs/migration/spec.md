# Migration Specification

## Purpose

Let a project set up with codev, the product's former name, become a
Compagnon project in one explicit command, and make sure Compagnon never works
on a codev layout without saying so.

## Requirements

### Requirement: A codev project is detected and never used silently

When the first ancestor of the working directory holding a planning root
holds `_codev/` and no `_compagnon/`, every command that needs a project
SHALL fail with the code `legacy_root`, naming the directory found
and telling the user to run `compagnon migrate`. No such command MUST read
or write anything under `_codev/`; `compagnon doctor` included, which fails
with the same code, and `compagnon init` run where `_codev/` exists and
`_compagnon/` does not fails with it too and writes nothing. `compagnon migrate`, `compagnon docs`,
`compagnon self-update`, `compagnon completions` and `compagnon --version` are not
affected. When a directory holds both `_codev/` and `_compagnon/`,
`_compagnon/` is the project and `_codev/` is ignored.

#### Scenario: A command run in a codev project

- **GIVEN** a directory `/p` holding `_codev/config.yaml` and no `_compagnon/`
- **WHEN** the user runs `compagnon status --json` from `/p/src`
- **THEN** the exit code is 1 and `status[0].code` is `legacy_root`
- **AND** the message names `/p` and `compagnon migrate`
- **AND** nothing under `/p/_codev/` is modified

#### Scenario: Init in a codev project

- **GIVEN** a directory holding `_codev/config.yaml` and no `_compagnon/`
- **WHEN** the user runs `compagnon init --yes`
- **THEN** the command fails with the code `legacy_root`
- **AND** no `_compagnon/` folder and no `compagnon-*` skill is created

#### Scenario: Doctor in a codev project

- **GIVEN** a directory holding `_codev/` and no `_compagnon/`
- **WHEN** the user runs `compagnon doctor`
- **THEN** the command fails with the code `legacy_root`

#### Scenario: A leftover codev root next to a Compagnon root

- **GIVEN** a directory holding both `_codev/` and `_compagnon/`
- **WHEN** the user runs `compagnon status`
- **THEN** the project is read from `_compagnon/`

#### Scenario: No planning root at all

- **GIVEN** a directory with neither `_codev/` nor `_compagnon/` in its
  ancestors
- **WHEN** the user runs `compagnon status --json`
- **THEN** `status[0].code` is `no_compagnon_root`

### Requirement: `compagnon migrate` converts a codev project in place

`compagnon migrate` SHALL, from any directory under a codev project:

- rename `_codev/` to `_compagnon/`, keeping every file it holds;
- rename `_compagnon/codev.lock` to `_compagnon/compagnon.lock`;
- rename the key `codev_version` to `compagnon_version` in
  `_compagnon/config.yaml`, keeping its value, its comments and the rest of
  the file byte for byte;
- rewrite, in the planning files, the markers `<!-- codev-review: N -->`
  and `<!-- codev-corrections: N -->` to their `compagnon` form;
- remove the skills `.claude/skills/codev-*/` whose content is the one
  codev generated, and install the `compagnon-*` skills of the configured
  workflows;
- replace the managed block of `CLAUDE.md` and the generated
  `.github/workflows/codev.yml` by their Compagnon versions
  (`.github/workflows/compagnon.yml`), and the managed section of the pull
  request template by its Compagnon version, leaving any hand-written content
  outside the managed blocks untouched.

A `codev-*` skill or a `codev.yml` workflow modified by hand SHALL be
kept and reported as kept, never deleted. A skill is judged modified when
it no longer hashes to the checksum it carries; a skill that carries no
checksum, as every skill the 0.6 releases installed, and whose
frontmatter names codev as its generator is taken as generated and
replaced — an edit to it is recovered from version control. The command MUST
print every path it moved, wrote, removed or kept, and support `--json`
(contract v1) and `--dry-run`, which prints the same plan and writes
nothing. Run where `_compagnon/` exists and `_codev/` does not, it SHALL complete
whatever former name is still present — `codev.lock`, the `codev_version`
key, a former marker, a generated `codev-*` skill or `codev.yml`, a former
managed section — and, when none is left, do nothing, exit 0, and say the
project is already migrated. When an operation fails, it SHALL stop, exit
1 with the code `migration_incomplete`, name the failing path and the
system error, list the operations already applied, and tell the user to
run `compagnon migrate` again. Before writing anything, it SHALL check that
the running version satisfies the `codev_version` requirement; when it
does not, it SHALL fail with `migration_version_unsatisfied`, naming the
requirement and asking the user to widen it, and modify nothing. After
migrating a project that declares git sources, it SHALL say to run
`compagnon sources update`. Run where there is no planning
root at all, it SHALL fail with `no_compagnon_root`. When `_codev/` and
`_compagnon/` both exist it SHALL fail with `migration_conflict` and modify
nothing.

The text of specs, decisions and changes is the user's: `compagnon migrate`
MUST NOT rewrite the prose of the planning files.

#### Scenario: Migrating a codev project

- **GIVEN** a project with `_codev/config.yaml` holding
  `codev_version: ">=0.6"`, `_codev/codev.lock`,
  `.claude/skills/codev-propose/SKILL.md` as codev generated it, and a
  `CLAUDE.md` with a managed block
- **WHEN** the user runs `compagnon migrate`
- **THEN** `_compagnon/config.yaml` holds `compagnon_version: ">=0.6"` and
  `_codev/` no longer exists
- **AND** `_compagnon/compagnon.lock` exists and `_compagnon/codev.lock` does not
- **AND** `.claude/skills/compagnon-propose/SKILL.md` exists and
  `.claude/skills/codev-propose/` does not
- **AND** the managed block of `CLAUDE.md` names `/compagnon-propose`

#### Scenario: Dry run

- **GIVEN** a codev project
- **WHEN** the user runs `compagnon migrate --dry-run --json`
- **THEN** the planned moves, writes and removals are listed
- **AND** no file of the project is modified

#### Scenario: A hand-edited skill is kept

- **GIVEN** a codev project whose `.claude/skills/codev-apply/SKILL.md`
  was edited by hand
- **WHEN** the user runs `compagnon migrate`
- **THEN** `.claude/skills/codev-apply/SKILL.md` still exists
- **AND** it is reported as `kept_modified`
- **AND** `.claude/skills/compagnon-apply/SKILL.md` is installed

#### Scenario: A skill installed by codev 0.6 is replaced

- **GIVEN** a codev project whose `.claude/skills/codev-propose/SKILL.md`
  is exactly as codev 0.6.1 wrote it, with no checksum
- **WHEN** the user runs `compagnon migrate`
- **THEN** `.claude/skills/codev-propose/` no longer exists
- **AND** `.claude/skills/compagnon-propose/SKILL.md` is installed

#### Scenario: Already migrated

- **GIVEN** a project with `_compagnon/` and no `_codev/`
- **WHEN** the user runs `compagnon migrate`
- **THEN** the exit code is 0, nothing is written, and the output says the
  project is already migrated

#### Scenario: Both roots present

- **GIVEN** a directory holding both `_codev/` and `_compagnon/`
- **WHEN** the user runs `compagnon migrate --json`
- **THEN** the exit code is 1, `status[0].code` is `migration_conflict`
- **AND** no file is modified

#### Scenario: No planning root

- **GIVEN** a directory with neither `_codev/` nor `_compagnon/` in its
  ancestors
- **WHEN** the user runs `compagnon migrate --json`
- **THEN** the exit code is 1 and `status[0].code` is `no_compagnon_root`

#### Scenario: An interrupted migration is completed by a second run

- **GIVEN** a migration that failed after renaming `_codev/` to
  `_compagnon/`, leaving `_compagnon/codev.lock` and the `codev-*` skills
- **THEN** that run exited 1 with `migration_incomplete`, naming the
  failing path and the operations applied
- **WHEN** the user runs `compagnon migrate` again
- **THEN** the lock is renamed, the skills are replaced, and the project
  matches a migration that never failed

#### Scenario: A version requirement the running Compagnon does not satisfy

- **GIVEN** a codev project whose config holds `codev_version: "^0.6"`
- **WHEN** the user runs `compagnon migrate` with Compagnon 0.7.0
- **THEN** the command fails with `migration_version_unsatisfied` naming
  `^0.6`
- **AND** no file is modified

#### Scenario: The pull request template is migrated

- **GIVEN** a codev project whose `.github/pull_request_template.md`
  holds a codev managed section and hand-written text
- **WHEN** the user runs `compagnon migrate`
- **THEN** the managed section is the Compagnon one and the hand-written
  text is unchanged

#### Scenario: Prose is left alone

- **GIVEN** a codev project whose spec says "run `codev validate`"
- **WHEN** the user runs `compagnon migrate`
- **THEN** the moved spec still says "run `codev validate`"

### Requirement: An inherited source still laid out for codev is refused with its fix

When a source declared under `sources:` in `_compagnon/config.yaml` holds
`_codev/config.yaml` and no `_compagnon/config.yaml`, the commands that read
sources SHALL fail with `legacy_source`, naming the source and both
possible causes with their fix: the source is not migrated yet (its
maintainers run `compagnon migrate` in it), or, for a git source, the pinned
commit predates its migration (run `compagnon sources update`). Compagnon MUST
NOT read it, and MUST NOT modify it.

#### Scenario: A shared standards repository not migrated yet

- **GIVEN** a Compagnon project whose source `standards` is a repository
  holding only `_codev/config.yaml`
- **WHEN** the user runs `compagnon validate --json`
- **THEN** `status[0].code` is `legacy_source` and the message names
  `standards`, `compagnon migrate` and `compagnon sources update`
- **AND** the source's files are unchanged

#### Scenario: A migrated source is read as usual

- **GIVEN** a Compagnon project whose source `standards` holds
  `_compagnon/config.yaml`, with or without a leftover `_codev/`
- **WHEN** the user runs `compagnon validate`
- **THEN** the source is read from its `_compagnon/` folder and no
  `legacy_source` is reported

### Requirement: The binary run under its former name says it is Compagnon

When the running executable's file name, without extension, is `codev`,
every command SHALL first print on stderr one line saying that codev is
now Compagnon and that the executable should be renamed `compagnon` or
reinstalled with the installer, then behave exactly as `compagnon`. With
`--json`, stdout MUST stay a valid JSON document.

#### Scenario: Run as `codev` after a self-update

- **GIVEN** the Compagnon binary installed at `~/.local/bin/codev`
- **WHEN** the user runs `codev --version`
- **THEN** stderr holds one line naming `compagnon`
- **AND** stdout holds the Compagnon version

#### Scenario: JSON output stays clean

- **GIVEN** the Compagnon binary installed under the name `codev`
- **WHEN** the user runs `codev status --json`
- **THEN** stdout parses as a JSON document
