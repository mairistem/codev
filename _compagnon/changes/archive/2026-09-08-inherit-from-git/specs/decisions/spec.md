## ADDED Requirements

### Requirement: Inheriting from a remote git repository

A project SHALL be able to declare `inherits: git:` in its
`_compagnon/config.yaml` to inherit from a remote git repository. The
declaration supports the fields `git` (URL, mandatory), `ref` (branch or
tag, mandatory), and `subpath` (path within the repository, optional).

#### Scenario: Decisions from a git repository inherited and indexed

- **GIVEN** a project whose `_compagnon/config.yaml` declares `inherits: [{git:
  "git@github.com:acme/compagnon-shared.git", ref: main}]`
- **AND** a `compagnon.lock` file locking a SHA `9f2c1ab7`
- **AND** a local cache under `~/.cache/compagnon/content/9f2c1ab7/` containing
  an ADR `0100 accepted`
- **WHEN** the validator computes the decision index
- **THEN** ADR `0100` appears in the index
- **AND** its `origin` is `git:git@github.com:acme/compagnon-shared.git`
- **AND** its `qualifiedId` is
  `git:git@github.com:acme/compagnon-shared.git/0100`

#### Scenario: Inherited decisions injected into the design instructions

- **GIVEN** the same project
- **WHEN** the user runs `compagnon instructions design --change <name>`
- **THEN** the response's `decisions` array contains the local decisions
  **and** the inherited `accepted` decisions from the locked git repository

### Requirement: Git source never read from a floating branch

The validator MUST refuse to expose the content of a `git:` source as long
as no SHA has been locked in `_compagnon/compagnon.lock`. No everyday command
(`list`, `show`, `status`, `instructions`, `validate`, `sync`, `archive`)
SHALL contact the network — `compagnon sources update` alone moves a pin.

#### Scenario: Git source declared but not locked

- **GIVEN** a project declaring `inherits: [{git: "…", ref: main}]`
- **AND** a `compagnon.lock` that is missing or has no entry for this source
- **WHEN** the user runs `compagnon instructions design --change <name>`
- **THEN** the `decisions` array contains only the local decisions
- **AND** the response's `status[]` field carries a warning with the
  stable code `git_source_unlocked` inviting the user to run
  `compagnon sources update`

#### Scenario: `compagnon status` does not contact the network

- **GIVEN** a project declaring a `git:` source with a locked SHA that is
  not in the cache
- **WHEN** the user runs `compagnon status --change <name>` while the network
  is unavailable
- **THEN** the command does not fail for network reasons
- **AND** no `git` call is made during execution

### Requirement: `compagnon sources update` resolves and locks

The `compagnon sources update` command MUST, for each declared `git:` source,
resolve the requested `ref` into a SHA via `git ls-remote`, download the
content if the SHA is not in the cache, and write a new
`_compagnon/compagnon.lock` where each source's SHA matches the current
resolution. It MUST display a diff of the SHA changes before writing.

#### Scenario: First update on a project without a lock

- **GIVEN** a project declaring a `git:` source but without `compagnon.lock`
- **WHEN** the user runs `compagnon sources update`
- **THEN** `git ls-remote` is called to resolve the `ref`
- **AND** a cache is populated with the SHA's content
- **AND** a new `_compagnon/compagnon.lock` is written carrying the resolved line

#### Scenario: Update without changes

- **GIVEN** a project whose lock already locks the SHA currently resolved
  by `git ls-remote`
- **WHEN** the user runs `compagnon sources update`
- **THEN** no additional download is performed
- **AND** the `compagnon.lock` file is not rewritten (content-to-content
  comparison)

#### Scenario: Diff before writing a moved pin

- **GIVEN** a project whose lock carries `commit: aaaa1111` but
  `git ls-remote` now returns `bbbb2222`
- **WHEN** the user runs `compagnon sources update`
- **THEN** the human output shows `aaaa1111 → bbbb2222` for this source
  before the lock is written

#### Scenario: git missing from the PATH

- **GIVEN** a system where the `git` binary cannot be found
- **WHEN** the user runs `compagnon sources update`
- **THEN** the command fails with the stable code `git_not_found`
- **AND** the message recalls that `compagnon sources update` is the only
  command that needs `git`

### Requirement: `compagnon sources list` and `compagnon sources show`

`compagnon sources list` MUST list all declared sources with their state;
`compagnon sources show <ref>` MUST display the details of one specific source,
identified by its URL (for a `git:` source) or its path (for a `path:`
one).

#### Scenario: List shows the state of each source

- **GIVEN** a project with a `path:` source and a locked `git:` source
- **WHEN** the user runs `compagnon sources list`
- **THEN** two entries appear, each with its type (`path` or `git`), its
  address, and its state (`resolved`, `locked`, or `unlocked`)

#### Scenario: Show points to the resolved cache

- **GIVEN** a project with a `git:` source locked on `9f2c1ab7`
- **WHEN** the user runs `compagnon sources show
  "git@github.com:acme/compagnon-shared.git"`
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
  `_compagnon/decisions/hook.sh`
- **WHEN** the validator computes the index
- **THEN** no index element references `hook.sh`
- **AND** no compagnon command runs this file
