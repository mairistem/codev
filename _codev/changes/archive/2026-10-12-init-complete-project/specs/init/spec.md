## ADDED Requirements

### Requirement: `codev init --inherit` declares inherited sources

`codev init` SHALL accept `--inherit <SOURCE>`, any number of times. A
SOURCE is a git source when it starts with `git@`, `https://`, `http://`,
`ssh://` or `file://`, or ends with `.git` once an optional `@<ref>`
suffix is removed; anything else is a local path. A git source MAY carry an
`@<ref>` suffix (`https://example.com/acme/conventions.git@v2`); the `@`
of a user name before the host (`git@github.com:…`) is not a ref
separator. Without a suffix the ref is `main`.

When `codev init` writes `_codev/config.yaml`, it MUST declare every
source under `inherits:`, in the order given, as `git:` + `ref:` or as
`path:`, with the comment `# set with codev init --inherit` above the
key. A relative local path MUST be written as an absolute path, so that it
resolves the same from any folder of the project. The written file MUST
stay readable by every other codev command.

The command MUST refuse, before writing anything, with the stable code
`invalid_inherit`:

- an empty value;
- a git source with an empty ref (`…/conventions.git@`) or an empty URL;
- a local path that does not exist or is not a folder.

When `_codev/config.yaml` already exists, `codev init` MUST NOT modify it:
the sources are not written, and a warning with the code
`inherit_not_written` names them and tells the user to add them under
`inherits:` by hand.

No source is declared when `--inherit` is absent.

#### Scenario: A git source with a ref and a local path

- **GIVEN** a new project and an existing folder `/home/you/conventions`
- **WHEN** the user runs `codev init --yes --inherit
  git@github.com:acme/conventions.git@v2 --inherit /home/you/conventions`
- **THEN** `_codev/config.yaml` contains, under `inherits:`, an entry
  `git: git@github.com:acme/conventions.git` with `ref: v2`, then an entry
  `path: /home/you/conventions`
- **AND** the line above `inherits:` reads
  `# set with codev init --inherit`
- **AND** `codev status` and `codev sources list` read the file without
  error

#### Scenario: A git source without a ref gets `main`

- **WHEN** the user runs `codev init --yes --inherit
  https://example.com/acme/conventions.git`
- **THEN** the declared source carries `ref: main`

#### Scenario: An invalid value is refused before anything is written

- **GIVEN** a folder without `_codev/`
- **WHEN** the user runs `codev init --yes --inherit ./does-not-exist`
- **THEN** the command fails with the code `invalid_inherit`
- **AND** no file or folder has been created

#### Scenario: An existing configuration is left untouched

- **GIVEN** a project whose `_codev/config.yaml` already exists
- **WHEN** the user runs `codev init --yes --inherit /home/you/conventions`
- **THEN** `_codev/config.yaml` is identical byte for byte
- **AND** a warning `inherit_not_written` names the source

### Requirement: `codev init` pins inherited git sources without failing on the network

After writing the configuration, `codev init` SHALL pin every declared git
source in `_codev/codev.lock` and fetch its content into the cache, with
the same result as `codev sources update`. Local paths are not pinned.

If pinning fails — `git` missing, network or authentication failure,
unknown ref — `codev init` MUST still succeed: the configuration and every
other file are written, the lock is left without the source, a warning
with the code `inherit_pin_failed` gives the reason, and the human output
says `help: run codev sources update`.

#### Scenario: A reachable git source is pinned

- **GIVEN** a reachable git repository whose `main` branch contains a
  `_codev/config.yaml`
- **WHEN** the user runs `codev init --yes --inherit <its URL>`
- **THEN** `_codev/codev.lock` contains an entry for the URL and ref
  `main` with the commit of `main`
- **AND** `codev sources list` reports the source as `locked`
- **AND** `codev instructions` exposes the inherited context with the
  origin `git:<its URL>`

#### Scenario: An unreachable git source leaves the source unpinned

- **GIVEN** a git URL that cannot be reached
- **WHEN** the user runs `codev init --yes --inherit <that URL>`
- **THEN** the command exits with code 0 and the skills are installed
- **AND** the source is declared in `_codev/config.yaml`
- **AND** a warning `inherit_pin_failed` is reported, followed by
  `help: run codev sources update`
- **AND** `codev sources list` reports the source as `unlocked`

### Requirement: `codev init` integrates codev into the repository

Unless opted out, `codev init` SHALL write three integration files, after
the skills:

- **`CLAUDE.md`** at the project root, created if missing, holding a codev
  section that tells Claude Code, in English: the repository follows
  spec-driven development with codev; `_codev/specs/` describes the current
  behavior and is authoritative; `_codev/decisions/` holds accepted
  decisions that are not re-debated; any change of behavior starts with
  `/codev-propose` and is implemented with `/codev-apply` (or by following
  the change's `tasks.md` when `/codev-apply` is not installed), never
  without a codev change; trivial fixes (typos, documentation wording) are
  exempt; `codev validate` runs before committing. The section lists the
  installed skills.
- **A CI workflow**, for a GitHub repository only — one that has a
  `.github/` folder, or a git remote on `github.com`:
  `.github/workflows/codev.yml`, which checks out the repository, installs
  codev with the official installer pinned to the version of the running
  binary (`CODEV_VERSION=<version>`), and runs `codev validate --strict`.
  It is not written when that file exists without the codev marker, nor
  when another workflow of `.github/workflows/` already runs
  `codev validate`; no other workflow file is ever modified. For a GitLab
  repository — a `.gitlab-ci.yml` file or a remote on `gitlab.com` —
  `.gitlab-ci.yml` is never modified and the human output prints a
  ready-to-paste job running `codev validate --strict`. Otherwise the CI
  step is skipped with a hint.
- **A pull request template section**, for a GitHub repository only: in the
  first existing template among `.github/`, the project root and `docs/`
  (file name `pull_request_template.md`, in any case), or in a new
  `.github/PULL_REQUEST_TEMPLATE.md` holding only the section. The section
  has a "codev change" line, to fill with the folder under
  `_codev/changes/` or "trivial fix, no change", and a checkbox
  "`codev validate --strict` passes".

The flags `--no-claude-md`, `--no-ci` and `--no-pr-template` MUST skip the
corresponding file, and `--no-integration` MUST skip all three. No
interactive question is added. `--no-detect` does not affect integration.

#### Scenario: A new GitHub project gets the three files

- **GIVEN** a new project with a `.github/` folder, no `CLAUDE.md` and no
  pull request template
- **WHEN** the user runs `codev init --yes`
- **THEN** `CLAUDE.md` holds the codev section and names
  `/codev-propose` and `/codev-apply`
- **AND** `.github/workflows/codev.yml` runs `codev validate --strict`
  after installing codev with `CODEV_VERSION=<running version>`
- **AND** `.github/PULL_REQUEST_TEMPLATE.md` holds the "codev change" line
  and the `codev validate --strict` checkbox

#### Scenario: A project outside GitHub gets only `CLAUDE.md`

- **GIVEN** a new project with no `.github/` folder, no `.gitlab-ci.yml`
  and no git remote
- **WHEN** the user runs `codev init --yes`
- **THEN** `CLAUDE.md` is written
- **AND** no `.github/` folder is created
- **AND** the summary reports the CI and pull request template steps as
  skipped, with the reason

#### Scenario: GitLab CI gets a snippet, not an edit

- **GIVEN** a project with a `.gitlab-ci.yml` file and no `.github/`
- **WHEN** the user runs `codev init --yes`
- **THEN** `.gitlab-ci.yml` is identical byte for byte
- **AND** the human output prints a job that installs codev and runs
  `codev validate --strict`

#### Scenario: An existing workflow already runs `codev validate`

- **GIVEN** a GitHub project whose `.github/workflows/ci.yml` runs
  `codev validate --strict`
- **WHEN** the user runs `codev init --yes`
- **THEN** `.github/workflows/codev.yml` is not created
- **AND** the summary names `ci.yml` as the reason

#### Scenario: A hand-written `codev.yml` is left alone

- **GIVEN** a GitHub project whose `.github/workflows/codev.yml` exists
  without the codev marker
- **WHEN** the user runs `codev init --yes`
- **THEN** the file is identical byte for byte
- **AND** the summary says it is not managed by codev

#### Scenario: Opting out

- **GIVEN** a new GitHub project
- **WHEN** the user runs `codev init --yes --no-integration`
- **THEN** neither `CLAUDE.md`, nor `.github/workflows/codev.yml`, nor a
  pull request template is created
- **AND WHEN** the user runs `codev init --yes --no-ci` on another new
  GitHub project
- **THEN** `CLAUDE.md` and the pull request template are written, and no
  workflow is

### Requirement: codev's text lives in managed sections that never touch user content

In `CLAUDE.md` and the pull request template, codev's text SHALL sit
between a line starting with `<!-- codev:begin` and a line
`<!-- codev:end -->` — HTML comments, invisible once the markdown is
rendered. The managed workflow file carries a `# codev:managed` comment on
its first line.

- When the file has no managed section, `codev init` appends one, separated
  from the existing content by a blank line; every byte before it is kept.
- When the file has one, `codev init` and `codev update` replace only what
  lies between the markers; every byte outside is kept. Identical content
  is not rewritten.
- When the markers are unbalanced or repeated, the file is not modified
  and a warning with the code `managed_section_malformed` names it.
- `codev update` refreshes the managed sections and the managed workflow
  file that exist, and never creates a file or appends a section: a
  project that opted out stays opted out.
- `--force` is not needed to refresh a managed section.

#### Scenario: A section is appended to an existing `CLAUDE.md`

- **GIVEN** a project whose `CLAUDE.md` holds the user's own instructions
  and no codev markers
- **WHEN** the user runs `codev init --yes`
- **THEN** `CLAUDE.md` starts with the user's instructions, unchanged
- **AND** ends with the codev managed section

#### Scenario: `codev update` refreshes only the section

- **GIVEN** a project whose `CLAUDE.md` has user text before and after a
  codev section whose content is outdated
- **WHEN** the user runs `codev update`
- **THEN** the text between the markers is the current codev section
- **AND** the user text before and after it is unchanged byte for byte
- **AND** running `codev update` a second time changes nothing

#### Scenario: `codev update` adds nothing to an opted-out project

- **GIVEN** a project initialized with `--no-integration`
- **WHEN** the user runs `codev update`
- **THEN** no `CLAUDE.md`, workflow or pull request template is created

#### Scenario: `codev update` bumps the managed workflow's version

- **GIVEN** a `.github/workflows/codev.yml` carrying `# codev:managed`,
  written by an earlier codev version
- **WHEN** the user runs `codev update` with a newer codev
- **THEN** the workflow installs the newer version

#### Scenario: Unbalanced markers leave the file alone

- **GIVEN** a `CLAUDE.md` with a `<!-- codev:begin` line and no
  `<!-- codev:end -->` line
- **WHEN** the user runs `codev init --yes`
- **THEN** `CLAUDE.md` is identical byte for byte
- **AND** a warning `managed_section_malformed` names `CLAUDE.md`

### Requirement: The init summary reports each integration step

The human output of `codev init` and `codev update` SHALL list each
integration file with what happened to it — written, refreshed, unchanged
or skipped — and, when skipped, why. `codev init` MUST end with a one-line
hint to run `codev doctor` when `codev doctor` would report a missing
check.

The JSON report of `codev init` and `codev update` MUST carry an
`integration` array, one entry per step, with the stable fields `step`
(`claude_md`, `ci`, `pr_template`), `action` (`created`, `appended`,
`refreshed`, `unchanged`, `skipped`), `path` (relative to the project, or
`null`) and `reason` (or `null`). The doctor hint is human output only.

#### Scenario: The summary explains a skipped step

- **GIVEN** a project outside GitHub
- **WHEN** the user runs `codev init --yes`
- **THEN** the summary reports `CLAUDE.md` as written and the CI step as
  skipped, with the reason
- **AND** the last line suggests `codev doctor`

#### Scenario: The JSON report lists the integration steps

- **WHEN** the user runs `codev init --yes --json` on a new GitHub project
- **THEN** `integration` holds three entries with `action: "created"`
- **AND** no entry mentions `codev doctor`
