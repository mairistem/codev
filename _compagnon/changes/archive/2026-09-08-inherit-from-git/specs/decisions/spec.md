## ADDED Requirements

### Requirement: Inheriting from a remote git repository

A project SHALL be able to declare `inherits: git:` in its
`_codev/config.yaml` to inherit from a remote git repository. The
declaration supports the fields `git` (URL, mandatory), `ref` (branch or
tag, mandatory), and `subpath` (path within the repository, optional).

#### Scenario: Decisions from a git repository inherited and indexed

- **GIVEN** a project whose `_codev/config.yaml` declares `inherits: [{git:
  "git@github.com:acme/codev-shared.git", ref: main}]`
- **AND** a `codev.lock` file locking a SHA `9f2c1ab7`
- **AND** a local cache under `~/.cache/codev/content/9f2c1ab7/` containing
  an ADR `0100 accepted`
- **WHEN** the validator computes the decision index
- **THEN** ADR `0100` appears in the index
- **AND** its `origin` is `git:git@github.com:acme/codev-shared.git`
- **AND** its `qualifiedId` is
  `git:git@github.com:acme/codev-shared.git/0100`

#### Scenario: Inherited decisions injected into the design instructions

- **GIVEN** the same project
- **WHEN** the user runs `codev instructions design --change <name>`
- **THEN** the response's `decisions` array contains the local decisions
  **and** the inherited `accepted` decisions from the locked git repository

### Requirement: Git source never read from a floating branch

The validator MUST refuse to expose the content of a `git:` source as long
as no SHA has been locked in `_codev/codev.lock`. No everyday command
(`list`, `show`, `status`, `instructions`, `validate`, `sync`, `archive`)
SHALL contact the network — `codev sources update` alone moves a pin.

#### Scenario: Git source declared but not locked

- **GIVEN** a project declaring `inherits: [{git: "…", ref: main}]`
- **AND** a `codev.lock` that is missing or has no entry for this source
- **WHEN** the user runs `codev instructions design --change <name>`
- **THEN** the `decisions` array contains only the local decisions
- **AND** the response's `status[]` field carries a warning with the
  stable code `git_source_unlocked` inviting the user to run
  `codev sources update`

#### Scenario: `codev status` does not contact the network

- **GIVEN** a project declaring a `git:` source with a locked SHA that is
  not in the cache
- **WHEN** the user runs `codev status --change <name>` while the network
  is unavailable
- **THEN** the command does not fail for network reasons
- **AND** no `git` call is made during execution

### Requirement: `codev sources update` resolves and locks

The `codev sources update` command MUST, for each declared `git:` source,
resolve the requested `ref` into a SHA via `git ls-remote`, download the
content if the SHA is not in the cache, and write a new
`_codev/codev.lock` where each source's SHA matches the current
resolution. It MUST display a diff of the SHA changes before writing.

#### Scenario: First update on a project without a lock

- **GIVEN** a project declaring a `git:` source but without `codev.lock`
- **WHEN** the user runs `codev sources update`
- **THEN** `git ls-remote` is called to resolve the `ref`
- **AND** a cache is populated with the SHA's content
- **AND** a new `_codev/codev.lock` is written carrying the resolved line

#### Scenario: Update without changes

- **GIVEN** a project whose lock already locks the SHA currently resolved
  by `git ls-remote`
- **WHEN** the user runs `codev sources update`
- **THEN** no additional download is performed
- **AND** the `codev.lock` file is not rewritten (content-to-content
  comparison)

#### Scenario: Diff before writing a moved pin

- **GIVEN** a project whose lock carries `commit: aaaa1111` but
  `git ls-remote` now returns `bbbb2222`
- **WHEN** the user runs `codev sources update`
- **THEN** the human output shows `aaaa1111 → bbbb2222` for this source
  before the lock is written

#### Scenario: git missing from the PATH

- **GIVEN** a system where the `git` binary cannot be found
- **WHEN** the user runs `codev sources update`
- **THEN** the command fails with the stable code `git_not_found`
- **AND** the message recalls that `codev sources update` is the only
  command that needs `git`

### Requirement: `codev sources list` and `codev sources show`

`codev sources list` MUST list all declared sources with their state;
`codev sources show <ref>` MUST display the details of one specific source,
identified by its URL (for a `git:` source) or its path (for a `path:`
one).

#### Scenario: List shows the state of each source

- **GIVEN** a project with a `path:` source and a locked `git:` source
- **WHEN** the user runs `codev sources list`
- **THEN** two entries appear, each with its type (`path` or `git`), its
  address, and its state (`resolved`, `locked`, or `unlocked`)

#### Scenario: Show points to the resolved cache

- **GIVEN** a project with a `git:` source locked on `9f2c1ab7`
- **WHEN** the user runs `codev sources show
  "git@github.com:acme/codev-shared.git"`
- **THEN** the output contains the URL, the requested `ref`, the locked
  SHA, and the resolved path in the cache
- **AND** lists the exposed files (decisions, inherited specs)

### Requirement: No inherited executable content

The loader SHALL expose to consumers (decision index, inherited spec
index, inherited config) only files with the `.md` and `.yaml`
extensions — even if the source repository contains others. A `.sh`,
`.py`, or `.rs` file, an executable, a hook, SHALL NEVER be loaded from an
inherited source.

#### Scenario: A script in the inherited repository is ignored

- **GIVEN** an inherited git repository that contains
  `_codev/decisions/hook.sh`
- **WHEN** the validator computes the index
- **THEN** no index element references `hook.sh`
- **AND** no codev command runs this file
