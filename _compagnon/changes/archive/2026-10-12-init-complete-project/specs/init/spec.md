## ADDED Requirements

### Requirement: `compagnon init --inherit` declares inherited sources

`compagnon init` SHALL accept `--inherit <SOURCE>`, any number of times. A
SOURCE is a git source when it starts with `git@`, `https://`, `http://`,
`ssh://` or `file://`, or ends with `.git` once an optional `@<ref>`
suffix is removed; anything else is a local path. A git source MAY carry an
`@<ref>` suffix (`https://example.com/acme/conventions.git@v2`); the `@`
of a user name before the host (`git@github.com:…`) is not a ref
separator. Without a suffix the ref is `main`.

When `compagnon init` writes `_compagnon/config.yaml`, it MUST declare every
source under `inherits:`, in the order given, as `git:` + `ref:` or as
`path:`, with the comment `# set with compagnon init --inherit` above the
key. A relative local path MUST be written as an absolute path, so that it
resolves the same from any folder of the project. The written file MUST
stay readable by every other compagnon command.

The command MUST refuse, before writing anything, with the stable code
`invalid_inherit`:

- an empty value;
- a git source with an empty ref (`…/conventions.git@`) or an empty URL;
- a local path that does not exist or is not a folder.

When `_compagnon/config.yaml` already exists, `compagnon init` MUST NOT modify it:
the sources are not written, and a warning with the code
`inherit_not_written` names them and tells the user to add them under
`inherits:` by hand.

No source is declared when `--inherit` is absent.

#### Scenario: A git source with a ref and a local path

- **GIVEN** a new project and an existing folder `/home/you/conventions`
- **WHEN** the user runs `compagnon init --yes --inherit
  git@github.com:acme/conventions.git@v2 --inherit /home/you/conventions`
- **THEN** `_compagnon/config.yaml` contains, under `inherits:`, an entry
  `git: git@github.com:acme/conventions.git` with `ref: v2`, then an entry
  `path: /home/you/conventions`
- **AND** the line above `inherits:` reads
  `# set with compagnon init --inherit`
- **AND** `compagnon status` and `compagnon sources list` read the file without
  error

#### Scenario: A git source without a ref gets `main`

- **WHEN** the user runs `compagnon init --yes --inherit
  https://example.com/acme/conventions.git`
- **THEN** the declared source carries `ref: main`

#### Scenario: An invalid value is refused before anything is written

- **GIVEN** a folder without `_compagnon/`
- **WHEN** the user runs `compagnon init --yes --inherit ./does-not-exist`
- **THEN** the command fails with the code `invalid_inherit`
- **AND** no file or folder has been created

#### Scenario: An existing configuration is left untouched

- **GIVEN** a project whose `_compagnon/config.yaml` already exists
- **WHEN** the user runs `compagnon init --yes --inherit /home/you/conventions`
- **THEN** `_compagnon/config.yaml` is identical byte for byte
- **AND** a warning `inherit_not_written` names the source

### Requirement: `compagnon init` pins inherited git sources without failing on the network

After writing the configuration, `compagnon init` SHALL pin every declared git
source in `_compagnon/compagnon.lock` and fetch its content into the cache, with
the same result as `compagnon sources update`. Local paths are not pinned.

If pinning fails — `git` missing, network or authentication failure,
unknown ref — `compagnon init` MUST still succeed: the configuration and every
other file are written, the lock is left without the source, a warning
with the code `inherit_pin_failed` gives the reason, and the human output
says `help: run compagnon sources update`.

#### Scenario: A reachable git source is pinned

- **GIVEN** a reachable git repository whose `main` branch contains a
  `_compagnon/config.yaml`
- **WHEN** the user runs `compagnon init --yes --inherit <its URL>`
- **THEN** `_compagnon/compagnon.lock` contains an entry for the URL and ref
  `main` with the commit of `main`
- **AND** `compagnon sources list` reports the source as `locked`
- **AND** `compagnon instructions` exposes the inherited context with the
  origin `git:<its URL>`

#### Scenario: An unreachable git source leaves the source unpinned

- **GIVEN** a git URL that cannot be reached
- **WHEN** the user runs `compagnon init --yes --inherit <that URL>`
- **THEN** the command exits with code 0 and the skills are installed
- **AND** the source is declared in `_compagnon/config.yaml`
- **AND** a warning `inherit_pin_failed` is reported, followed by
  `help: run compagnon sources update`
- **AND** `compagnon sources list` reports the source as `unlocked`

### Requirement: `compagnon init` integrates compagnon into the repository

Unless opted out, `compagnon init` SHALL write three integration files, after
the skills:

- **`CLAUDE.md`** at the project root, created if missing, holding a compagnon
  section that tells Claude Code, in English: the repository follows
  spec-driven development with compagnon; `_compagnon/specs/` describes the current
  behavior and is authoritative; `_compagnon/decisions/` holds accepted
  decisions that are not re-debated; any change of behavior starts with
  `/compagnon-propose` and is implemented with `/compagnon-apply` (or by following
  the change's `tasks.md` when `/compagnon-apply` is not installed), never
  without a compagnon change; trivial fixes (typos, documentation wording) are
  exempt; `compagnon validate` runs before committing. The section lists the
  installed skills.
- **A CI workflow**, for a GitHub repository only — one that has a
  `.github/` folder, or a git remote on `github.com`:
  `.github/workflows/compagnon.yml`, which checks out the repository, installs
  compagnon with the official installer pinned to the version of the running
  binary (`COMPAGNON_VERSION=<version>`), and runs `compagnon validate --strict`.
  It is not written when that file exists without the compagnon marker, nor
  when another workflow of `.github/workflows/` already runs
  `compagnon validate`; no other workflow file is ever modified. For a GitLab
  repository — a `.gitlab-ci.yml` file or a remote on `gitlab.com` —
  `.gitlab-ci.yml` is never modified and the human output prints a
  ready-to-paste job running `compagnon validate --strict`. Otherwise the CI
  step is skipped with a hint.
- **A pull request template section**, for a GitHub repository only: in the
  first existing template among `.github/`, the project root and `docs/`
  (file name `pull_request_template.md`, in any case), or in a new
  `.github/PULL_REQUEST_TEMPLATE.md` holding only the section. The section
  has a "compagnon change" line, to fill with the folder under
  `_compagnon/changes/` or "trivial fix, no change", and a checkbox
  "`compagnon validate --strict` passes".

The flags `--no-claude-md`, `--no-ci` and `--no-pr-template` MUST skip the
corresponding file, and `--no-integration` MUST skip all three. No
interactive question is added. `--no-detect` does not affect integration.

#### Scenario: A new GitHub project gets the three files

- **GIVEN** a new project with a `.github/` folder, no `CLAUDE.md` and no
  pull request template
- **WHEN** the user runs `compagnon init --yes`
- **THEN** `CLAUDE.md` holds the compagnon section and names
  `/compagnon-propose` and `/compagnon-apply`
- **AND** `.github/workflows/compagnon.yml` runs `compagnon validate --strict`
  after installing compagnon with `COMPAGNON_VERSION=<running version>`
- **AND** `.github/PULL_REQUEST_TEMPLATE.md` holds the "compagnon change" line
  and the `compagnon validate --strict` checkbox

#### Scenario: A project outside GitHub gets only `CLAUDE.md`

- **GIVEN** a new project with no `.github/` folder, no `.gitlab-ci.yml`
  and no git remote
- **WHEN** the user runs `compagnon init --yes`
- **THEN** `CLAUDE.md` is written
- **AND** no `.github/` folder is created
- **AND** the summary reports the CI and pull request template steps as
  skipped, with the reason

#### Scenario: GitLab CI gets a snippet, not an edit

- **GIVEN** a project with a `.gitlab-ci.yml` file and no `.github/`
- **WHEN** the user runs `compagnon init --yes`
- **THEN** `.gitlab-ci.yml` is identical byte for byte
- **AND** the human output prints a job that installs compagnon and runs
  `compagnon validate --strict`

#### Scenario: An existing workflow already runs `compagnon validate`

- **GIVEN** a GitHub project whose `.github/workflows/ci.yml` runs
  `compagnon validate --strict`
- **WHEN** the user runs `compagnon init --yes`
- **THEN** `.github/workflows/compagnon.yml` is not created
- **AND** the summary names `ci.yml` as the reason

#### Scenario: A hand-written `compagnon.yml` is left alone

- **GIVEN** a GitHub project whose `.github/workflows/compagnon.yml` exists
  without the compagnon marker
- **WHEN** the user runs `compagnon init --yes`
- **THEN** the file is identical byte for byte
- **AND** the summary says it is not managed by compagnon

#### Scenario: Opting out

- **GIVEN** a new GitHub project
- **WHEN** the user runs `compagnon init --yes --no-integration`
- **THEN** neither `CLAUDE.md`, nor `.github/workflows/compagnon.yml`, nor a
  pull request template is created
- **AND WHEN** the user runs `compagnon init --yes --no-ci` on another new
  GitHub project
- **THEN** `CLAUDE.md` and the pull request template are written, and no
  workflow is

### Requirement: compagnon's text lives in managed sections that never touch user content

In `CLAUDE.md` and the pull request template, compagnon's text SHALL sit
between a line starting with `<!-- compagnon:begin` and a line
`<!-- compagnon:end -->` — HTML comments, invisible once the markdown is
rendered. The managed workflow file carries a `# compagnon:managed` comment on
its first line.

- When the file has no managed section, `compagnon init` appends one, separated
  from the existing content by a blank line; every byte before it is kept.
- When the file has one, `compagnon init` and `compagnon update` replace only what
  lies between the markers; every byte outside is kept. Identical content
  is not rewritten.
- When the markers are unbalanced or repeated, the file is not modified
  and a warning with the code `managed_section_malformed` names it.
- `compagnon update` refreshes the managed sections and the managed workflow
  file that exist, and never creates a file or appends a section: a
  project that opted out stays opted out.
- `--force` is not needed to refresh a managed section.

#### Scenario: A section is appended to an existing `CLAUDE.md`

- **GIVEN** a project whose `CLAUDE.md` holds the user's own instructions
  and no compagnon markers
- **WHEN** the user runs `compagnon init --yes`
- **THEN** `CLAUDE.md` starts with the user's instructions, unchanged
- **AND** ends with the compagnon managed section

#### Scenario: `compagnon update` refreshes only the section

- **GIVEN** a project whose `CLAUDE.md` has user text before and after a
  compagnon section whose content is outdated
- **WHEN** the user runs `compagnon update`
- **THEN** the text between the markers is the current compagnon section
- **AND** the user text before and after it is unchanged byte for byte
- **AND** running `compagnon update` a second time changes nothing

#### Scenario: `compagnon update` adds nothing to an opted-out project

- **GIVEN** a project initialized with `--no-integration`
- **WHEN** the user runs `compagnon update`
- **THEN** no `CLAUDE.md`, workflow or pull request template is created

#### Scenario: `compagnon update` bumps the managed workflow's version

- **GIVEN** a `.github/workflows/compagnon.yml` carrying `# compagnon:managed`,
  written by an earlier compagnon version
- **WHEN** the user runs `compagnon update` with a newer compagnon
- **THEN** the workflow installs the newer version

#### Scenario: Unbalanced markers leave the file alone

- **GIVEN** a `CLAUDE.md` with a `<!-- compagnon:begin` line and no
  `<!-- compagnon:end -->` line
- **WHEN** the user runs `compagnon init --yes`
- **THEN** `CLAUDE.md` is identical byte for byte
- **AND** a warning `managed_section_malformed` names `CLAUDE.md`

### Requirement: The init summary reports each integration step

The human output of `compagnon init` and `compagnon update` SHALL list each
integration file with what happened to it — written, refreshed, unchanged
or skipped — and, when skipped, why. `compagnon init` MUST end with a one-line
hint to run `compagnon doctor` when `compagnon doctor` would report a missing
check.

The JSON report of `compagnon init` and `compagnon update` MUST carry an
`integration` array, one entry per step, with the stable fields `step`
(`claude_md`, `ci`, `pr_template`), `action` (`created`, `appended`,
`refreshed`, `unchanged`, `skipped`), `path` (relative to the project, or
`null`) and `reason` (or `null`). The doctor hint is human output only.

#### Scenario: The summary explains a skipped step

- **GIVEN** a project outside GitHub
- **WHEN** the user runs `compagnon init --yes`
- **THEN** the summary reports `CLAUDE.md` as written and the CI step as
  skipped, with the reason
- **AND** the last line suggests `compagnon doctor`

#### Scenario: The JSON report lists the integration steps

- **WHEN** the user runs `compagnon init --yes --json` on a new GitHub project
- **THEN** `integration` holds three entries with `action: "created"`
- **AND** no entry mentions `compagnon doctor`
