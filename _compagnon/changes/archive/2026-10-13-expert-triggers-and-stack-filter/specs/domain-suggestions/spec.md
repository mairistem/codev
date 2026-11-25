## Purpose

Suggests the domains a change touches from what its diff actually
contains, so that an expert cannot be left out silently: each suggestion
is either ticked in the proposal, kept unticked with a reason, or
reported.

## ADDED Requirements

### Requirement: Experts carry triggers, each naming the domain it suggests

A built-in role MAY carry triggers: groups, each naming one domain among
`security`, `personal-data`, `user-interface`, `performance`,
`operations` and `migration`, and holding at least one of:

- `paths` — globs matched against the path of a changed file relative to
  the project root, `/`-separated, ignoring case; `*` stays within a path
  segment and `**` spans segments;
- `manifests` — file names of dependency manifests, matched against the
  last segment of a changed file's path, at any depth;
- `patterns` — regular expressions matched against each line the diff
  adds.

Each of the six experts SHALL carry at least one group, and each of its
groups SHALL name the expert's own domain. The `security` expert's
triggers SHALL include the dependency manifests `Cargo.toml`,
`package.json`, `pyproject.toml`, `requirements.txt`, `go.mod`, `pom.xml`
and `build.gradle`. The other roles carry no trigger.

#### Scenario: A path trigger suggests its domain

- **GIVEN** a change `add-login` whose diff adds the file
  `src/auth/session.rs`
- **WHEN** the user runs `codev roles --change add-login --json`
- **THEN** `suggestions` holds `security`, with a match of kind `paths`,
  source `security`, and path `src/auth/session.rs`

#### Scenario: A manifest trigger matches at any depth

- **GIVEN** a change whose diff modifies `services/api/package.json`
- **WHEN** the user runs `codev roles --change <name> --json`
- **THEN** `suggestions` holds `security`, with a match of kind
  `manifests` and trigger `package.json`

#### Scenario: A pattern trigger names the added line

- **GIVEN** a change whose diff adds, at line 12 of `src/db.rs`, the line
  `sqlx::query("ALTER TABLE users DROP COLUMN phone")`
- **WHEN** the user runs `codev roles --change <name> --json`
- **THEN** `suggestions` holds `migration`, with a match of kind
  `patterns`, path `src/db.rs` and line 12

#### Scenario: Every expert's triggers suggest its own domain

- **GIVEN** the built-in role catalog
- **WHEN** the test suite of `codev-core` runs
- **THEN** it fails if an expert carries no trigger group, if a group names
  a domain other than its expert's, if a glob or a regular expression does
  not compile, or if a non-expert role carries triggers

#### Scenario: A removed line suggests nothing

- **GIVEN** a change whose diff only removes lines holding `password`
- **WHEN** the user runs `codev roles --change <name> --json`
- **THEN** no pattern match is reported for those lines

### Requirement: The change's diff is read from git, never failing

For a change, codev SHALL read a diff made of: the changes between the
merge-base of `HEAD` and the default branch and the working tree —
committed, staged and unstaged alike — plus the untracked files that git
does not ignore. The default branch SHALL be the one `origin/HEAD` points
to, else the local branch `main`, else `master`. Only files under the
project root are read, and their paths are relative to it,
`/`-separated, on every OS. Line endings `\r\n` count as `\n`. codev
reads the repository and never contacts a remote: an out-of-date
`origin/HEAD` is used as it is.

When the project is not in a git repository, git is not installed,
`HEAD` has no commit, no default branch exists, or a git command fails,
there SHALL be no diff: the command reports it as unavailable, with the
reason, suggests no domain, and does not fail because of it.

#### Scenario: Every kind of change is in the diff

- **GIVEN** a branch whose default branch is `main`, with a file
  committed since the merge-base, a staged file, a modified unstaged file
  and an untracked file
- **WHEN** the user runs `codev roles --change <name> --json`
- **THEN** `diff.available` is `true`, `diff.base` is `main`, and
  `diff.files` counts the four files

#### Scenario: The default branch from `origin/HEAD`

- **GIVEN** a repository where `origin/HEAD` points to `origin/trunk`
- **WHEN** the user runs `codev roles --change <name> --json`
- **THEN** `diff.base` is `origin/trunk`

#### Scenario: Not a git repository

- **GIVEN** a project outside any git repository
- **WHEN** the user runs `codev roles --change <name> --json`
- **THEN** the exit code is 0, `diff.available` is `false`,
  `diff.reason` says the project is not in a git repository, and
  `suggestions` is empty

#### Scenario: No default branch

- **GIVEN** a git repository with neither `origin/HEAD`, `main` nor
  `master`
- **WHEN** the user runs `codev roles --change <name> --json`
- **THEN** `diff.available` is `false` and `diff.reason` names the
  branches it looked for

#### Scenario: A project in a subdirectory of the repository

- **GIVEN** a project whose root is the directory `packages/api` of a git
  repository, and a diff touching `packages/api/src/auth.rs` and
  `packages/web/src/auth.ts`
- **WHEN** the user runs `codev roles --change <name> --json`
- **THEN** only `src/auth.rs` is in the diff

#### Scenario: Windows line endings

- **GIVEN** an untracked file whose lines end with `\r\n`
- **WHEN** the patterns are matched against its lines
- **THEN** a pattern anchored at the end of a line matches as it does with
  `\n`

### Requirement: Noise is left out and the scan is capped

Before matching, codev SHALL leave out of every trigger the files whose
path is:

- under the project's planning directory (`_codev/`) or a codev-generated
  skill (`.claude/skills/codev-*/`) — category `planning`;
- under a directory named `node_modules`, `vendor`, `third_party`,
  `third-party` or `.yarn` — category `vendored`;
- a lock file (`Cargo.lock`, `package-lock.json`, `npm-shrinkwrap.json`,
  `yarn.lock`, `pnpm-lock.yaml`, `bun.lockb`, `poetry.lock`,
  `Pipfile.lock`, `uv.lock`, `go.sum`, `Gemfile.lock`, `composer.lock`,
  `gradle.lockfile`) — category `lockfile`;
- generated by a tool: `*.min.js`, `*.min.css`, `*.map`, `*.pb.go`,
  `*_pb2.py`, `*_pb2_grpc.py`, `*.generated.*`, or under a directory named
  `__generated__` — category `generated`.

The patterns SHALL also skip binary files and untracked entries that are
not regular files (category `binary`), prose files
(`.md`, `.markdown`, `.rst`, `.txt`, `.adoc` — category `prose`), files
adding more than 5000 lines and untracked files over 1 MiB (category
`large`), and lines longer than 2000 characters; their paths still meet
the `paths` and `manifests` triggers. An untracked entry that is not a
regular file — a symbolic link, a device — SHALL never be read.

At most 2000 changed files SHALL be considered, in path order, and the
patterns SHALL stop after 2 MiB of added lines; when either cap is
reached, the diff SHALL say it was truncated. The diff's summary SHALL
count the files changed, the files whose lines the patterns read, and
the files skipped per category.

#### Scenario: A lock file suggests nothing

- **GIVEN** a change whose diff only modifies `Cargo.lock`
- **WHEN** the user runs `codev roles --change <name> --json`
- **THEN** `suggestions` is empty and `diff.skipped.lockfile` is 1

#### Scenario: Vendored and planning files suggest nothing

- **GIVEN** a change whose diff adds `vendor/auth/token.go` and
  `_codev/changes/<name>/design.md`, both holding `password`
- **WHEN** the user runs `codev roles --change <name> --json`
- **THEN** `suggestions` is empty and `diff.skipped` counts one `vendored`
  and one `planning` file

#### Scenario: A docs-only change

- **GIVEN** a change whose diff only modifies `docs/security.md`, which
  mentions `password`
- **WHEN** the user runs `codev roles --change <name> --json`
- **THEN** no pattern match is reported and `diff.skipped.prose` is 1

#### Scenario: A binary file is matched by its path only

- **GIVEN** a change whose diff adds the binary file
  `src/components/logo.png`
- **WHEN** the user runs `codev roles --change <name> --json`
- **THEN** `suggestions` holds `user-interface` through a `paths` match,
  and `diff.skipped.binary` is 1

#### Scenario: A huge diff is truncated

- **GIVEN** a change whose diff touches 2500 files
- **WHEN** the user runs `codev roles --change <name> --json`
- **THEN** `diff.files` is 2500, `diff.truncated` is `true`, and the files
  after the first 2000 in path order are matched by no trigger

#### Scenario: An untracked symbolic link is not read

- **GIVEN** an untracked symbolic link `notes` pointing outside the project
- **WHEN** the user runs `codev roles --change <name> --json`
- **THEN** the link's target is not read and no pattern match names
  `notes`

### Requirement: Each suggested domain has a state

For a change, a domain SHALL be suggested when at least one trigger
naming it matches the change's diff. The triggers of an expert the
project or the change sets to `never` SHALL NOT be evaluated. A suggested
domain's state SHALL be:

- `ticked` when the proposal's `## Domains` section ticks it;
- `justified` when the section lists it unticked with text after its
  name (`- [ ] security — only renames a variable`), that text being the
  justification;
- `unticked` otherwise — including when the proposal does not exist yet,
  has no `## Domains` section, or says `None`.

With `--change`, `codev roles --json` SHALL carry `diff` — `available`,
`reason` (`null` when available), `base` (the default branch used,
`null` when unavailable), `files`, `scanned`, `skipped` (a count per
category, only the categories met) and `truncated` — and `suggestions`,
in the order of the domains list, each with `domain`, `state`,
`justification` (`null` unless `justified`), `matchCount` and `matches`:
at most five, in path then line order, each with `source` (the role id,
or `project`), `kind` (`paths`, `manifests` or `patterns`), `trigger`
(the glob, file name or expression as written), `path` and `line` (the
added line for a pattern, `null` otherwise). Without `--change`, `diff`
is `null` and `suggestions` is empty. Without `--json`, with `--change`,
the command SHALL print the diff's summary or why there is none, then one
line per suggested domain with its state and its first match.

#### Scenario: A suggested domain the proposal ticks

- **GIVEN** a change whose proposal ticks `security` and whose diff adds
  `src/auth/session.rs`
- **WHEN** the user runs `codev roles --change <name> --json`
- **THEN** the `security` suggestion has `state: "ticked"`

#### Scenario: A justified domain

- **GIVEN** a change whose proposal lists
  `- [ ] security — only renames a variable` and whose diff adds
  `src/auth/session.rs`
- **WHEN** the user runs `codev roles --change <name> --json`
- **THEN** the `security` suggestion has `state: "justified"` and
  `justification: "only renames a variable"`

#### Scenario: `None` does not justify a suggestion

- **GIVEN** a change whose `## Domains` section says `None` and whose diff
  adds `src/auth/session.rs`
- **WHEN** the user runs `codev roles --change <name> --json`
- **THEN** the `security` suggestion has `state: "unticked"`

#### Scenario: A disabled expert suggests nothing

- **GIVEN** a configuration with `roles: { accessibility: never }` and a
  change whose diff adds `src/components/Button.tsx`
- **WHEN** the user runs `codev roles --change <name> --json`
- **THEN** `suggestions` holds no `user-interface`

#### Scenario: Without a change

- **WHEN** the user runs `codev roles --at plan.challenge --json`
- **THEN** `diff` is `null` and `suggestions` is empty

#### Scenario: Suggestions in the human output

- **GIVEN** a change whose diff adds `src/auth/session.rs` and whose
  proposal ticks nothing
- **WHEN** the user runs `codev roles --change <name>`
- **THEN** it prints the diff's summary and a line naming `security`,
  `unticked`, the trigger `**/auth/**` and `src/auth/session.rs`

#### Scenario: Run twice, the same answer

- **GIVEN** a change and an unchanged working tree
- **WHEN** the user runs `codev roles --change <name> --json` twice
- **THEN** both documents are identical, and no file was written

### Requirement: The project adds its own triggers under `triggers:`

`_codev/config.yaml` SHALL accept a top-level `triggers:` list of groups
of the same shape as an expert's — `domain`, `paths`, `manifests`,
`patterns` — whose matches have the source `project`. They are evaluated
whatever the project's or the change's role settings. The `triggers:` key
of an inherited source's configuration MUST be ignored.

`codev validate` SHALL report, at the line of the project's `triggers:`
key, the error `invalid_trigger` for a group whose domain is missing or
not one of the six, a group with no trigger, or a glob or regular
expression that does not compile or exceeds codev's size limit; and the
warning `invalid_trigger` for a key of a group codev does not know. A
group with an error is ignored by every command, which reports it as an
`invalid_trigger` warning in its `status`; a group with an unknown key is
used without that key.

#### Scenario: A project trigger

- **GIVEN** a configuration whose `triggers:` holds a group
  `{ domain: personal-data, paths: ["src/billing/**"] }` and a change
  whose diff adds `src/billing/invoice.rs`
- **WHEN** the user runs `codev roles --change <name> --json`
- **THEN** `suggestions` holds `personal-data` with a match whose source
  is `project`

#### Scenario: An invalid project trigger

- **GIVEN** a configuration whose `triggers:` holds a group with
  `domain: secrets` and a group whose pattern is `(unclosed`
- **WHEN** the user runs `codev validate`
- **THEN** two `invalid_trigger` errors are reported on
  `_codev/config.yaml`, one naming `secrets`, one naming `(unclosed`
- **AND** `codev roles --change <name> --json` succeeds with an
  `invalid_trigger` warning in `status`

#### Scenario: An unknown key in a group

- **GIVEN** a configuration whose `triggers:` group holds the key
  `exclude`
- **WHEN** the user runs `codev validate`
- **THEN** an `invalid_trigger` warning names `exclude`
- **AND** the group's other triggers still suggest their domain

#### Scenario: `triggers` is a known key

- **GIVEN** a configuration with a valid `triggers:` list
- **WHEN** the user runs `codev status --json`
- **THEN** `status` holds no `unknown_config_key` warning

#### Scenario: Triggers of an inherited source are ignored

- **GIVEN** a project inheriting from a source whose configuration holds
  `triggers:` with a group `{ domain: operations, paths: ["**"] }`
- **WHEN** the user runs `codev roles --change <name> --json`
- **THEN** no match has the source `project`
