# Decisions Specification

## Purpose

Make architecture decisions usable by codev: read them, index them,
resolve the chain of supersessions, and inject them into the instructions
of the `design` artifact so that an agent drafting it sees the choices
already settled from the outset.

## Requirements

### Requirement: ADR format recognized

The parser SHALL recognize an ADR written as a YAML frontmatter followed by
free-form markdown sections. The frontmatter carries at least the fields
`id`, `title`, `status`, `date`, and optionally `tags` (a list) and
`supersedes` (an identifier or an array of identifiers). A file without a
frontmatter, or whose frontmatter lacks a mandatory field, is reported,
not silently parsed.

#### Scenario: Well-formed ADR

- **GIVEN** a file starting with `---`, containing a YAML frontmatter
  with `id: 0007`, `title: "…"`, `status: accepted`, `date: 2026-09-08`,
  `tags: [architecture]`, then a `---` separator, then free-form markdown
- **WHEN** the parser reads the file
- **THEN** the result exposes the identifier `0007`, the title, the status
  `accepted`, the date and the tag `architecture`

#### Scenario: ADR without frontmatter

- **GIVEN** a markdown file without a `---` header
- **WHEN** the parser reads the file
- **THEN** a finding with the stable code `decision_missing_frontmatter`
  reports that the file does not have the expected format

#### Scenario: Mandatory field missing

- **GIVEN** a file whose frontmatter has no `title`
- **WHEN** the parser reads the file
- **THEN** a finding with code `decision_missing_field` names the missing
  field

### Requirement: Recognized statuses and effect

The validator MUST recognize the statuses `accepted`, `superseded`,
`proposed`, `deprecated`, `rejected`; only `accepted` and `superseded`
are taken into account when computing the effect — the other three are
exposed as is in the index and never considered "in effect".

#### Scenario: Unknown status reported

- **GIVEN** an ADR whose `status` is `pending`
- **WHEN** the parser reads the file
- **THEN** a finding with code `decision_unknown_status` reports the value
  and recalls the list of recognized statuses

#### Scenario: A proposed status does not take effect

- **GIVEN** an ADR with status `proposed`, without any supersession link
- **WHEN** the index is computed
- **THEN** this decision does not appear among the "decisions in effect"

### Requirement: Supersession resolved as a chain

The index MUST resolve the `supersedes` field: each decision that an ADR
supersedes is marked `superseded_by(<id>)` in the index, and is not in
effect. A chain `A ← B ← C` leaves `A` and `B` superseded; only `C`
remains in effect.

#### Scenario: Direct supersession

- **GIVEN** an ADR `0003 accepted` and an ADR `0007 accepted supersedes: [0003]`
- **WHEN** the index is computed
- **THEN** `0003` is marked `superseded_by(0007)` and does not appear among
  the decisions in effect
- **AND** `0007` appears among the decisions in effect

#### Scenario: Three-link chain

- **GIVEN** three `accepted` ADRs where `B` supersedes `A` and `C` supersedes `B`
- **WHEN** the index is computed
- **THEN** only `C` is in effect

#### Scenario: Supersession target missing

- **GIVEN** an ADR `0007 supersedes: [9999]` while `9999` does not exist
- **WHEN** the index is computed
- **THEN** a finding with code `decision_supersedes_unknown` reports
  the phantom identifier
- **AND** `0007` remains in effect (the lost link does not disqualify it)

### Requirement: Inherited decisions taken into account

When a project declares `inherits: path: <path>` in its
`_codev/config.yaml`, the validator MUST also parse the ADRs of
`<path>/_codev/decisions/` and merge them into the index with their
`origin` visible.

#### Scenario: ADR from an inherited source appears in the index

- **GIVEN** a project that inherits from a source `path: ~/shared` containing
  an ADR `0100 accepted`
- **WHEN** the index is computed
- **THEN** the entry carries the `origin` `path:~/shared`
- **AND** its qualified identifier, used to avoid collisions, is
  `path:~/shared/0100`

#### Scenario: Id collision between the project and a source

- **GIVEN** a local ADR `0007 accepted` **and** an ADR `0007 accepted` in
  an inherited source
- **WHEN** the index is computed
- **THEN** a finding with code `decision_id_collision` reports the duplicate
- **AND** the project's version wins (it is closer to the author)

### Requirement: Injection into the design instructions

The call `codev instructions design --change <name>` MUST enrich its
response with a `decisions[]` field carrying the decisions in effect. Each
entry exposes `id`, `title`, `status`, `tags`, `path` (relative to the
project) and `origin`. The full content stays in the file — no
duplication in the response.

#### Scenario: Design instructions carry the decisions in effect

- **GIVEN** a project with 6 `accepted` ADRs and no supersession
- **WHEN** the user runs `codev instructions design --change <name>
  --json`
- **THEN** the JSON response contains a `decisions` array with exactly
  6 entries, each carrying `id`, `title`, `status`, a `path` relative to
  the project, and `origin: "project"`

#### Scenario: Superseded decisions absent from the instructions

- **GIVEN** a project where `0003` is superseded by `0007`
- **WHEN** the user runs `codev instructions design --change <name>
  --json`
- **THEN** `0003` does not appear in the `decisions` array
- **AND** `0007` does appear in it

#### Scenario: Human rendering lists the decisions

- **GIVEN** the same context
- **WHEN** the user runs `codev instructions design --change <name>`
  (without `--json`)
- **THEN** the output contains a "Decisions in effect" section listing
  each decision on its own line with its `id` and its `title`

#### Scenario: No decision, no section

- **GIVEN** a new project without any ADR
- **WHEN** the user runs `codev instructions design --change <name>`
- **THEN** no "Decisions in effect" section is added to the output
- **AND** the `decisions[]` field in the JSON response is present and empty

### Requirement: Creating a decision through the CLI

`codev decision new <title>` SHALL create a new ADR in the project, with
an automatically generated numeric `id` (the largest numeric `id` found
in the project + 1, on four digits), a file named `NNNN-<slug>.md` where
`<slug>` is derived from the title in kebab-case, and a valid frontmatter
including `status: accepted` and the current date.

#### Scenario: Creation in a project without ADRs

- **GIVEN** a project whose `_codev/decisions/` directory is empty
- **WHEN** the user runs `codev decision new "A first choice"`
- **THEN** the file `_codev/decisions/0001-a-first-choice.md` is created
- **AND** its frontmatter carries `id: "0001"`, `title: "A first choice"`,
  `status: accepted`, and a date in `YYYY-MM-DD` format

#### Scenario: Sequential numbering

- **GIVEN** a project whose `_codev/decisions/` directory already contains
  six ADRs numbered `0001` to `0006`
- **WHEN** the user runs `codev decision new "A seventh choice"`
- **THEN** the file `_codev/decisions/0007-a-seventh-choice.md` is created
- **AND** its `id` is `"0007"`

#### Scenario: Customizable status

- **GIVEN** a project
- **WHEN** the user runs `codev decision new "Idea to explore"
  --status proposed`
- **THEN** the created ADR carries `status: proposed`

#### Scenario: Empty title rejected

- **GIVEN** a project
- **WHEN** the user runs `codev decision new ""`
- **THEN** no file is written
- **AND** an error message names the stable code `empty_title` and
  asks for a non-empty title

### Requirement: Listing decisions through the CLI

`codev decision list` MUST display all local and inherited decisions,
with, for each one, its identifier, its title, its status, its effect
state, and its origin. The `--json` mode MUST output exactly one JSON
document whose shape is frozen per version.

#### Scenario: List with local decisions and one supersession

- **GIVEN** a project containing three `accepted` ADRs where `0007`
  supersedes `0003`
- **WHEN** the user runs `codev decision list --json`
- **THEN** the `decisions` array contains the three entries
- **AND** the `0003` entry carries `inEffect: false` and `supersededBy:
  "project/0007"`
- **AND** the `0007` entry carries `inEffect: true`

#### Scenario: Concise human rendering

- **GIVEN** the same context
- **WHEN** the user runs `codev decision list`
- **THEN** each decision appears on its own line with, in order, an
  effect marker (`•` for in effect, `–` otherwise), its `id`, its title,
  and its status

### Requirement: Showing a decision through the CLI

`codev decision show <id>` MUST display the full content of the requested
decision. Resolution accepts a short `id` when it is unambiguous, or a
qualified identifier `<origin>/<id>` in case of collision.

#### Scenario: Show of an id without collision

- **GIVEN** a project containing a unique ADR `0001`
- **WHEN** the user runs `codev decision show 0001`
- **THEN** the output contains the frontmatter header (`id`, `title`,
  `status`, `date`) followed by the markdown body of the decision

#### Scenario: Show of an ambiguous id

- **GIVEN** a project containing a local ADR `0007` **and** an ADR `0007`
  in an inherited source `path:~/shared`
- **WHEN** the user runs `codev decision show 0007`
- **THEN** no content is displayed
- **AND** the `ambiguous_decision_id` error message lists both
  qualified identifiers (`project/0007`, `path:~/shared/0007`) and asks
  the user to be more specific

#### Scenario: Show with a qualified identifier

- **GIVEN** the same context
- **WHEN** the user runs `codev decision show path:~/shared/0007`
- **THEN** the inherited decision is displayed

### Requirement: Superseding a decision through the CLI

`codev decision supersede <old-id> <new-title>` MUST create a new
decision that references the old one in its `supersedes`, and rewrite the
frontmatter of the old one so that its `status` becomes `superseded`. Both
writes happen in the same plan — either both succeed, or neither is
applied.

#### Scenario: Local supersession

- **GIVEN** a project containing an ADR `0003 accepted` titled "Old
  choice"
- **WHEN** the user runs `codev decision supersede 0003 "New
  choice"`
- **THEN** a new ADR `0007-new-choice.md` (or the next available `id`)
  is created with `supersedes: ["0003"]` and `status: accepted`
- **AND** the frontmatter of the `0003` file now has `status:
  superseded`
- **AND** the body of the `0003` file — Context, Decision, everything
  that follows the frontmatter — has remained identical down to the
  character

#### Scenario: Superseding an unknown id rejects everything

- **GIVEN** a project in which `9999` does not exist
- **WHEN** the user runs `codev decision supersede 9999 "X"`
- **THEN** no file is written
- **AND** the `unknown_decision_id` error message names `9999`

#### Scenario: Superseding an inherited decision rejected

- **GIVEN** a project inheriting from a source containing `path:~/shared/0100`
- **WHEN** the user runs `codev decision supersede path:~/shared/0100
  "Our alternative"`
- **THEN** no file is written
- **AND** the `cannot_supersede_inherited` error message explains that an
  inherited decision is read-only and suggests deviating from it with
  `codev decision deviate`

### Requirement: Stable JSON contract for all commands

When `--json` is requested, each command MUST write exactly one JSON
document to stdout whose shape is frozen per version, with a root
`status` array for execution errors — the same rules as the rest of the
codev contract.

#### Scenario: Shape of a `decision new --json`

- **GIVEN** a project
- **WHEN** the user runs `codev decision new "X" --json`
- **THEN** stdout carries a JSON document containing `changeName: null`
  (this command does not act on a change), `decision: { id: "0001",
  qualifiedId: "project/0001", path: "…", title: "X", status: "accepted"
  }`, and a `status: []`

#### Scenario: Shape of a `decision show` failure on an unknown id

- **GIVEN** a project
- **WHEN** the user runs `codev decision show 9999 --json`
- **THEN** stdout carries a single JSON document containing
  `decision: null` and `status: [{ code: "unknown_decision_id", … }]`

### Requirement: Inheritance from a remote git repository

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
- **THEN** the ADR `0100` appears in the index
- **AND** its `origin` is `git:git@github.com:acme/codev-shared.git`
- **AND** its `qualifiedId` is
  `git:git@github.com:acme/codev-shared.git/0100`

#### Scenario: Inherited decisions injected into the design instructions

- **GIVEN** the same project
- **WHEN** the user runs `codev instructions design --change <name>`
- **THEN** the `decisions` array of the response contains the local
  decisions **and** the `accepted` inherited decisions from the locked git
  repository

### Requirement: A git source is never read from a floating branch

The validator MUST refuse to expose the content of a `git:` source until
a SHA has been locked in `_codev/codev.lock`. No everyday command
(`list`, `show`, `status`, `instructions`, `validate`, `sync`, `archive`)
SHALL contact the network — only `codev sources update` moves a pin.

#### Scenario: Git source declared but not locked

- **GIVEN** a project declaring `inherits: [{git: "…", ref: main}]`
- **AND** a `codev.lock` that is missing or has no entry for this source
- **WHEN** the user runs `codev instructions design --change <name>`
- **THEN** the `decisions` array contains only the local decisions
- **AND** the `status[]` field of the response carries a warning with the
  stable code `git_source_unlocked` inviting the user to run
  `codev sources update`

#### Scenario: `codev status` does not contact the network

- **GIVEN** a project declaring a `git:` source with a locked SHA
  that is not in the cache
- **WHEN** the user runs `codev status --change <name>` while the
  network is unavailable
- **THEN** the command does not fail for network reasons
- **AND** no `git` call is made during execution

### Requirement: `codev sources update` resolves and locks

The `codev sources update` command MUST, for each declared `git:` source,
resolve the requested `ref` to a SHA via `git ls-remote`, download the
content if the SHA is not in the cache, and write a new
`_codev/codev.lock` in which the SHA of each source matches the current
resolution. It MUST display a diff of the SHA changes before
writing.

#### Scenario: First update on a project without a lock

- **GIVEN** a project declaring a `git:` source but without `codev.lock`
- **WHEN** the user runs `codev sources update`
- **THEN** `git ls-remote` is called to resolve the `ref`
- **AND** a cache is populated with the content of the SHA
- **AND** a new `_codev/codev.lock` is written carrying the resolved
  line

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
- **THEN** the human output shows `aaaa1111 → bbbb2222` for that
  source before the lock is written

#### Scenario: git missing from PATH

- **GIVEN** a system on which the `git` binary cannot be found
- **WHEN** the user runs `codev sources update`
- **THEN** the command fails with the stable code `git_not_found`
- **AND** the message recalls that `codev sources update` is the only
  command that needs `git`

### Requirement: `codev sources list` and `codev sources show`

`codev sources list` MUST list all declared sources with their state;
`codev sources show <ref>` MUST display the details of a specific source,
identified by its URL (for a `git:` source) or its path (for a `path:`
source).

#### Scenario: List shows the state of each source

- **GIVEN** a project with a `path:` source and a locked `git:` source
- **WHEN** the user runs `codev sources list`
- **THEN** two entries appear, each with its type (`path` or
  `git`), its address, and its state (`resolved`, `locked`, or `unlocked`)

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
`.py` or `.rs` file, an executable, or a hook SHALL NEVER be loaded from
an inherited source.

#### Scenario: A script in the inherited repository is ignored

- **GIVEN** an inherited git repository that contains `_codev/decisions/hook.sh`
- **WHEN** the validator computes the index
- **THEN** no element of the index references `hook.sh`
- **AND** no codev command runs that file

### Requirement: Body seal recorded by the CLI

The CLI SHALL maintain a `_codev/decisions/seal.yaml` file — versioned
with the project — that records the sealed local decisions, with, for
each entry, the `id` of the decision, the SHA-256 hash of the ADR's
**body** (everything that follows the `---` separator closing the
frontmatter), and the date on which the seal was applied. Only local
decisions are sealed — inherited decisions belong to the source project,
not to the consumer.

The file contains only the schema version and the list of seals —
a seal is an `id`, a `bodySha256` prefixed with `sha256:`, and a
`sealedAt` in `YYYY-MM-DD` format.

#### Scenario: Seal written when a decision is created

- **GIVEN** a project in which `_codev/decisions/seal.yaml` does not exist
- **WHEN** the user runs `codev decision new "A first choice"`
- **THEN** the file `_codev/decisions/seal.yaml` is created
- **AND** it contains an entry whose `id` is `"0001"` and whose
  `bodySha256` matches the hexadecimal SHA-256 of the body of the file
  `0001-a-first-choice.md` (prefixed with `sha256:`)

#### Scenario: Seal file format

- **GIVEN** the `_codev/decisions/seal.yaml` file created by the CLI
- **WHEN** a human opens it
- **THEN** it contains a `version: 1` key at the top
- **AND** a `seals` key carrying a list in which each entry has
  exactly the fields `id`, `bodySha256`, `sealedAt`
- **AND** no other unknown field

### Requirement: `decision new` writes the ADR and the seal in the same plan

The `codev decision new <title>` operation MUST produce an effect plan
that writes both the ADR **and** the corresponding entry in
`_codev/decisions/seal.yaml`. If either one fails, neither is written
— the command never leaves an ADR without a seal nor a seal without an
ADR.

#### Scenario: A failed seal write cancels the creation

- **GIVEN** a project in which `_codev/decisions/seal.yaml` is
  write-locked by the system
- **WHEN** the user runs `codev decision new "Title"`
- **THEN** no ADR file is written to `_codev/decisions/`
- **AND** the `seal.yaml` file remains unchanged

### Requirement: `decision supersede` seals the new ADR without touching the old one's seal

The `codev decision supersede <id> <title>` operation MUST add a seal
entry for the newly created ADR, and MUST NOT modify the seal entry of
the old one — since its body remains identical down to the character (a
requirement already in effect), its seal remains valid.

#### Scenario: Supersession seals only the new one

- **GIVEN** a project containing a sealed ADR `0003 accepted` referenced
  in `seal.yaml` under `id: "0003"`
- **WHEN** the user runs `codev decision supersede 0003 "New choice"`
- **THEN** the `seal.yaml` file now contains two entries: the one for
  `0003` (unchanged) and a new one for the created ADR (for example
  `0007`)
- **AND** the `0003` entry has exactly the same `bodySha256` as before the
  supersession

### Requirement: `validate` detects body alterations

`codev validate` MUST emit three new findings with stable codes
to report discrepancies between ADRs and their seal:

- `decision_unsealed` — **warning** emitted for any local ADR with status
  `accepted` or `superseded` that has no entry in `seal.yaml`. The
  message recalls that migration is done with `codev decision seal`.
- `decision_seal_mismatch` — **error** emitted for any ADR whose computed
  `bodySha256` no longer matches the one recorded in `seal.yaml`. The
  message names the ADR concerned and recalls that a deliberate
  modification goes through `codev decision seal --force`.
- `decision_orphan_seal` — **warning** emitted for any `seal.yaml` entry
  whose referenced ADR no longer exists in `_codev/decisions/`.

#### Scenario: Detection of a body modified in place

- **GIVEN** a project whose ADR `0001` is sealed
- **AND** a human has edited the body of the file `0001-*.md` without
  updating `seal.yaml`
- **WHEN** the user runs `codev validate`
- **THEN** the output contains a finding with code `decision_seal_mismatch`
  naming `0001`
- **AND** the exit code of the command is non-zero

#### Scenario: Detection of an unsealed ADR

- **GIVEN** a project containing six local `accepted` ADRs, none of which
  has an entry in `seal.yaml` (initial migration state)
- **WHEN** the user runs `codev validate`
- **THEN** the output contains six findings with code `decision_unsealed`,
  one per ADR
- **AND** the exit code of the command is zero (warnings, not errors)

#### Scenario: Detection of an orphan seal

- **GIVEN** a project whose `seal.yaml` contains an entry for
  the ADR `0004`
- **AND** the file `0004-*.md` has been deleted
- **WHEN** the user runs `codev validate`
- **THEN** the output contains a `decision_orphan_seal` finding naming
  `0004`
- **AND** the exit code of the command is zero (warning, not error)

### Requirement: `decision seal` adds or renews a seal entry

`codev decision seal <id>` MUST create a seal entry for a local ADR that
does not have one — this is the migration operation. For an already
sealed ADR whose body has changed, the command MUST refuse without
`--force` and recall that immutability is intentional. With `--force`, it
rewrites the `bodySha256` and updates `sealedAt`. An inherited ADR cannot
be sealed by the consuming project (sealing belongs to the source
project).

#### Scenario: Migration of an unsealed ADR

- **GIVEN** a project containing an ADR `0001 accepted` without an entry in
  `seal.yaml`
- **WHEN** the user runs `codev decision seal 0001`
- **THEN** `seal.yaml` now carries an entry for `0001` whose
  `bodySha256` matches the current body of the file

#### Scenario: Refusal to re-seal without `--force`

- **GIVEN** a project whose ADR `0001` is sealed, and whose body was
  subsequently edited in place
- **WHEN** the user runs `codev decision seal 0001`
- **THEN** `seal.yaml` remains unchanged
- **AND** the error message names the stable code `seal_conflict` and
  recalls that `--force` deliberately rewrites the seal

#### Scenario: Re-seal with `--force`

- **GIVEN** the same context
- **WHEN** the user runs `codev decision seal 0001 --force`
- **THEN** the `0001` entry in `seal.yaml` now carries the new
  `bodySha256` and a `sealedAt` date matching the current date

#### Scenario: Refusal to seal an inherited decision

- **GIVEN** a project inheriting from a source containing `path:~/shared/0100`
- **WHEN** the user runs `codev decision seal path:~/shared/0100`
- **THEN** no write takes place
- **AND** the error message names the stable code
  `cannot_seal_inherited` and recalls that sealing belongs to the
  source project

### Requirement: `deviates_from` field recognized in an ADR frontmatter

The ADR parser SHALL recognize an optional `deviates_from` field in the
YAML frontmatter — a list of qualified identifiers (`<origin>/<id>`)
pointing to the decisions from which this new ADR locally departs. The
field is additive: its absence preserves the current semantics of ADRs.
A value that is not a list, or whose entries are not strings, is
reported.

#### Scenario: ADR with `deviates_from`

- **GIVEN** a local ADR whose frontmatter carries
  `deviates_from: ["path:~/shared/0100"]`
- **WHEN** the parser reads the file
- **THEN** the result exposes the list `["path:~/shared/0100"]` in the
  `deviates_from` field
- **AND** no finding is emitted for this field

#### Scenario: Malformed `deviates_from`

- **GIVEN** an ADR whose frontmatter carries `deviates_from: "not-a-list"`
- **WHEN** the parser reads the file
- **THEN** a finding with code `decision_field_type_mismatch` reports the
  `deviates_from` field

### Requirement: The `codev decision deviate` command creates a deviation ADR

`codev decision deviate <qualified-id> <title>` MUST create a local
`accepted` ADR with `deviates_from: ["<qualified-id>"]` and a valid
frontmatter (`id`, `title`, `status: accepted`, `date`), sealed by the
same effect plan, like every accepted decision.

The command MUST refuse:

- if `<qualified-id>` designates a local decision (`project/…`): stable
  code `cannot_deviate_from_local`, with a message pointing to
  `codev decision supersede`.
- if `<qualified-id>` matches no indexed decision: stable code
  `unknown_decision_id` (already existing).
- if `<title>` is empty: stable code `empty_title` (already existing).

#### Scenario: Deviation from an inherited `path:` decision

- **GIVEN** a project inheriting from a source `path: ~/shared` containing
  an ADR `path:~/shared/0100`
- **WHEN** the user runs
  `codev decision deviate path:~/shared/0100 "Our local alternative"`
- **THEN** a new local ADR is created under
  `_codev/decisions/NNNN-our-local-alternative.md` with
  `deviates_from: ["path:~/shared/0100"]` and `status: accepted`
- **AND** an entry is added to `_codev/decisions/seal.yaml` for this
  new ADR

#### Scenario: Refusal to deviate from a local decision

- **GIVEN** a project containing a local ADR `0003 accepted`
- **WHEN** the user runs `codev decision deviate project/0003 "…"`
- **THEN** no file is written
- **AND** the error message names the stable code
  `cannot_deviate_from_local` and suggests `codev decision supersede`

#### Scenario: Refusal of an unknown target

- **GIVEN** a project without any inherited source
- **WHEN** the user runs
  `codev decision deviate path:~/unknown/0100 "…"`
- **THEN** no file is written
- **AND** the error message names the stable code `unknown_decision_id`

### Requirement: The index hides deviated inherited decisions and exposes `deviated_by`

When the decision index computes the entries in effect, each inherited
decision referenced by a `deviates_from` of a local `accepted` ADR MUST
be marked `deviated_by: <local-qualified-id>` in the index, removed from
the `in_effect` array, and appear neither in the `decisions[]` array of
the `design` instructions nor in the human "Decisions in effect" section
of their rendering.

The inherited entry remains visible in `codev decision list` —
transparency takes precedence over hiding.

#### Scenario: Design instructions do not carry the deviated decision

- **GIVEN** a project that inherits from a source containing
  `path:~/shared/0100 accepted`, and a local ADR `0007 accepted` whose
  frontmatter carries `deviates_from: ["path:~/shared/0100"]`
- **WHEN** the user runs
  `codev instructions design --change <name> --json`
- **THEN** the `decisions` array of the response contains `project/0007`
- **AND** the `decisions` array does NOT contain `path:~/shared/0100`

#### Scenario: `decision list` exposes the deviation

- **GIVEN** the same context
- **WHEN** the user runs `codev decision list --json`
- **THEN** the `path:~/shared/0100` entry carries `deviatedBy:
  "project/0007"` and `inEffect: false`
- **AND** the `project/0007` entry carries `deviatesFrom:
  ["path:~/shared/0100"]` and `inEffect: true`

#### Scenario: A local `proposed` ADR does not cause a deviation

- **GIVEN** a local ADR `0007 proposed` with `deviates_from:
  ["path:~/shared/0100"]`
- **WHEN** the index is computed
- **THEN** `path:~/shared/0100` remains in effect (the proposed ADR is not
  yet committed, so it cannot deviate)
- **AND** the `path:~/shared/0100` entry carries no `deviatedBy`

### Requirement: `validate` detects degenerate deviations

`codev validate` MUST emit two new findings with stable codes:

- `decision_dangling_deviation` — **warning** — when a local ADR has a
  `deviates_from: ["<qualified-id>"]` whose target does not exist or no
  longer exists in the index (source removed, SHA moved, id changed).
- `decision_conflicting_deviations` — **error** — when two local
  `accepted` ADRs reference the same target in their `deviates_from`.
  The rule is: "one target, one deviation".

#### Scenario: Dangling deviation

- **GIVEN** a project in which a local ADR `0007 accepted` carries
  `deviates_from: ["path:~/unknown/9999"]`, without any source exposing
  that `qualified-id`
- **WHEN** the user runs `codev validate`
- **THEN** the output contains a finding with code
  `decision_dangling_deviation` naming `0007` and its missing target
- **AND** the exit code of the command is zero (warning)

#### Scenario: Two deviations on the same target

- **GIVEN** a project with two local `accepted` ADRs, `0007` and
  `0008`, both with `deviates_from: ["path:~/shared/0100"]`
- **WHEN** the user runs `codev validate`
- **THEN** the output contains a
  `decision_conflicting_deviations` finding that names both ADRs and the
  conflicting target
- **AND** the exit code of the command is non-zero (error)

### Requirement: The `codev decision promote` command extracts a design block into an ADR

`codev decision promote <change> <title>` MUST create a new local ADR
under `_codev/decisions/NNNN-<slug>.md`, with `status: accepted`, sealed
like every accepted decision, whose body reproduces **verbatim** the content of the
`### Decision: <title>` block found under the `## Decisions` section of
the change's `design.md`. The new ADR contains a `## Decision` section
carrying that body, and the `## Context`, `## Consequences` and
`## Alternatives considered` sections are emitted with a
`<!-- placeholder -->` inviting the author to fill them in.

The "verbatim" body of the block means: all the text that follows the
`### Decision: <title>` line up to the next line starting with `### ` or
`## ` (excluded), without normalization.

Explicit refusals — stable codes:

- `unknown_change`: the change is not in `codev list`.
- `cannot_promote_from_archived`: the target points to a directory under
  `changes/archive/` — an archived design is history.
- `design_missing`: the change has no `design.md`.
- `decision_heading_not_found`: no `### Decision: <title>` block matches
  in the design.
- `ambiguous_decision_heading`: several blocks carry the same title —
  the user disambiguates by editing.

#### Scenario: Successful promotion

- **GIVEN** a change `add-auth` whose `design.md` contains, under
  `## Decisions`, a `### Decision: Use JWT` block with two
  paragraphs of rationale
- **WHEN** the user runs `codev decision promote add-auth
  "Use JWT"`
- **THEN** a new ADR is created under
  `_codev/decisions/NNNN-use-jwt.md` with `status: accepted`, the current
  `date`, and a valid frontmatter
- **AND** its body contains a `## Decision` section with the two
  paragraphs of rationale, byte for byte
- **AND** an entry is added to `_codev/decisions/seal.yaml` for this
  ADR

#### Scenario: Refusal of an archived change

- **GIVEN** a change that lives under `_codev/changes/archive/…-<name>/`
- **WHEN** the user runs `codev decision promote <name> "..."`
- **THEN** no file is written
- **AND** the error message names the stable code
  `cannot_promote_from_archived`

#### Scenario: Title not found

- **GIVEN** a change `add-auth` whose `design.md` does not mention a
  "Use Kerberos" block
- **WHEN** the user runs `codev decision promote add-auth
  "Use Kerberos"`
- **THEN** no file is written
- **AND** the error message names the stable code
  `decision_heading_not_found`

#### Scenario: Ambiguous title

- **GIVEN** a `design.md` containing **two**
  `### Decision: Library choice` blocks (for example a revised version
  of the first block written during the discussion)
- **WHEN** the user runs `codev decision promote <c> "Library
  choice"`
- **THEN** no file is written
- **AND** the error message names the stable code
  `ambiguous_decision_heading` and invites the user to edit one of the
  two titles

### Requirement: The design is updated with a traceable reference

After a successful promotion, the `### Decision: <title>` block of
`design.md` MUST be replaced by the same title followed by **a single
line** of textual quotation referencing the new ADR:

```
### Decision: <title>

> Promoted to ADR **NNNN** — see `_codev/decisions/NNNN-<slug>.md`.
```

The reference is plain text (`` ` `` for the path), not a markdown
link — a `[..](../..)` link would break when the change is archived
(where the depth of the `..` changes). The block title is preserved so
that a reader of the design can understand the subject that was
promoted.

#### Scenario: Block content replaced by the reference

- **GIVEN** the same context as the "Successful promotion" scenario
- **WHEN** the user runs the command
- **THEN** the change's `design.md` now contains, in place of the
  block:
  ```
  ### Decision: Use JWT

  > Promoted to ADR **NNNN** — see `_codev/decisions/NNNN-use-jwt.md`.

  ```
- **AND** the rest of the file (other sections, other `###` blocks,
  whitespace) is identical down to the character
- **AND** the whitespace around the block remains preserved — no blank
  line added or removed

#### Scenario: Two successive promotions on the same design

- **GIVEN** a design containing two distinct blocks:
  `### Decision: A` and `### Decision: B`
- **AND** the user has already promoted `A` to an ADR
- **WHEN** the user runs `codev decision promote <c> "B"`
- **THEN** block `B` is promoted in turn
- **AND** the reference line of `A` is NOT altered by this
  second promotion — each promotion acts only on its own block
