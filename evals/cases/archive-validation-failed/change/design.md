# Design: Forward-compatible configuration

## Context

- `codev-engine::config::ProjectConfig` and its nested `McpConfig`,
  `ChangelogConfig` and `InheritSource` all carry
  `#[serde(deny_unknown_fields)]`; `config::load` turns any serde error into
  `EngineError::Invalid` (code `invalid`). Every command reads the file
  through `config::load` or `config::resolve`, so one unknown key stops
  them all, `codev doctor` included.
- `config::resolve` already returns non-fatal `warnings` (inherited sources)
  that `codev status`, `codev new change`, `codev init` and `codev update`
  report, in human output and in the JSON `status` array.
- `codev validate` reports items of kind `change`, `spec` and `decisions`;
  `--strict` fails on any warning finding.
- `codev doctor` gathers `DoctorFacts` in the CLI and `codev-core::doctor`
  evaluates them (decision 0001). Its `codev_version` check always gives the
  fix `codev self-update`, while `codev self-update` refuses a downgrade
  without `--version` (`codev-core::update::decide`). The update cache only
  records the latest release, not a list of releases.
- codev 0.5 and earlier ship the strict parser: nothing done here changes
  how they read a newer configuration.

## Goals / Non-Goals

Goals: an older codev (from this release on) keeps working on a newer
configuration and says what it needs; typos at the top level stay visible
and fail `codev validate --strict`; doctor's version fix is a command that
works.

Non-goals: tolerating unknown keys in other files (`change.yaml`,
`schema.yaml`, `seal.yaml`, whose strictness is unchanged); a
`--version` fix for the "below" case; fetching the release list to find
the highest satisfying release (doctor stays offline).

## Decisions

### Decision: Tolerance per section, by what ignoring a key would do

The criterion: a section is tolerant when ignoring an unknown key only
means "this feature is absent", and strict when ignoring it would make
codev silently do something different from what the file asks.

| Section | Unknown key | Why |
|---|---|---|
| top level | warning | each top-level key is an independent feature; ignoring one leaves the others intact, and the top level is where a newer codev adds keys (`codev_version` was one) |
| `mcp` | warning | built to grow (one tool name per integration); ignoring an unknown tool only leaves that integration unconfigured, which the skills already handle with their "not configured" fallback |
| `inherits` entries | error | an entry identifies a source; ignoring a key (a future pin, filter or subpath variant) would read a different source than intended and inject its context into prompts (decision 0005: inherited content is security-sensitive) |
| `changelog` | error | `codev changelog --write` rewrites `CHANGELOG.md`; ignoring a key that narrows or reshapes the output would write a different file without saying so |
| `rules` | not applicable | a free map of artifact ids, already open |

The top level and `mcp` lose `deny_unknown_fields`; `InheritSource` and
`ChangelogConfig` keep it. Unknown keys are found by reading the file a
second time as a generic YAML mapping and comparing its keys with the known
lists (`PROJECT_KEYS`, `MCP_KEYS`), declared next to the structs. A test
loads a configuration holding every known key and asserts that none is
reported, so a field added to a struct without its name fails a test.

Inherited sources are parsed with the same tolerant code, but their unknown
keys are dropped silently: only their `context` and `rules` are read, every
other key — known or not — is already ignored.

**Alternatives considered**: `#[serde(flatten)] extra: BTreeMap<…>` on
`ProjectConfig` — rejected: flatten buffers the whole map and degrades
serde's error messages for every nested field, which decision 0006 values
for hand-written YAML; it also does not reach into `mcp`. The
`serde_ignored` crate — rejected: a new dependency for what a comparison of
two key lists does. Tolerating every section — rejected for `inherits` and
`changelog` for the reasons in the table. Keeping everything strict and only
improving the error message — rejected: the old binary would still be
unusable, which is the problem.

### Decision: One warning per key, built from the file alone

`config::load` returns the unknown keys (dotted for `mcp`: `mcp.design_tool`)
with their line, found by scanning the text for `key:` at the right
indentation (line 1 when not found, e.g. a flow mapping). `config::resolve`
turns each project key into a `Warning { code: "unknown_config_key" }`
whose message names the key, says the project may need a newer codev,
quotes `codev_version` when the file sets it and points to
`codev self-update`, or to correcting a typo. The message does not name the
running version: the engine does not know it, and the doctor check and
`versionRequirement` already compare versions.

When the strict parse fails (a nested strict section, a wrong type), the
error is enriched the same way: the file is read leniently as a YAML
mapping and, if it holds a string `codev_version`, the message adds
"this project requires codev_version: "…"; run `codev self-update` if your
codev is older". The code stays `invalid`.

**Alternatives considered**: one warning listing all keys — rejected: one
finding per key gives each its line in `codev validate`, and JSON consumers
match on one code per fact. A dedicated error code for the enriched error —
rejected: the failure is still an invalid file; the hint is in the message.

### Decision: A `config` item in `codev validate`, present only when needed

`validate_all` appends an `ItemReport` of the new kind `ItemKind::Config`
(`"config"` in human and JSON output), path `_codev/config.yaml`, with one
`Warning` finding per unknown key, only when there is at least one. The
scoped runs (`--changes`, `--specs`, a named item) do not include it: they
are about one kind of item.

A warning, not an error, so plain `codev validate` (the pre-commit and
teammate case) passes while `codev validate --strict` (CI) fails: in CI, an
unknown key is either a typo — which must not slip through now that it no
longer fails parsing — or a CI codev older than the configuration, which
must be updated. The `kind` value is additive to the v1 contract; consumers
that switch on `kind` already face three values.

**Alternatives considered**: always emitting the item, empty when clean —
rejected: it changes every existing report and test for no information.
An error finding — rejected: it would make the older teammate's plain
`codev validate` fail, the situation this change removes.

### Decision: A `config.keys` doctor check, a warning

A new check `config.keys` in the Configuration group, right after
`config.language`: `ok` when the project's configuration has no unknown
key, otherwise a `warning` with the code `unknown_config_key`, a message
naming the keys, and a fix that depends on the version check: `codev
self-update` when the running codev does not satisfy `codev_version`,
otherwise "correct or remove the key in `_codev/config.yaml`, or run
`codev self-update` if a newer codev added it". A warning, because the
command works and doctor's `--strict` fails only on what is missing.

**Alternatives considered**: folding the keys into `config.rules` or
`validate` — rejected: the check would mean two things, and its code would
not say which.

### Decision: The downgrade target is computed in the core

`codev-core::update` gains `requirement_side(requirement, current)`
(`Below` / `Above`, `None` when satisfied or unparsable) and
`downgrade_target(requirement, latest_known)`. A comparator that fails
marks `Above` when it is an upper bound: `<`, `<=`, or `=`, `~`, `^`, `*`
whose base version is below the running one; `>` and `>=` mark `Below`.
The target is, in order: the cached latest release when it satisfies the
requirement; the version of an inclusive upper bound written in full
(`<=0.6.2`, `=0.6.2`); the lowest version admitted by a lower bound
(`>=0.5`, `^0.5`, `~0.5.1`, missing parts as 0), checked against the whole
requirement; otherwise none. `core::doctor::evaluate` then writes
`codev self-update --version <target>`, or, without a target,
`codev self-update --version <X.Y.Z> with a release that satisfies "<req>"`.
`VersionFacts` gains `latest_known`, the cached latest release whether or
not it is newer, read offline as today.

**Alternatives considered**: always the lowest admitted version — rejected:
the cache, when it satisfies the requirement, is a release that exists and
is the most recent acceptable one. Fetching the release list — rejected:
doctor is offline by spec. Changing `codev self-update` to accept a
requirement (`--version "<0.7"`) — rejected here: a new interface for a
case the existing `--version` covers.

## Risks / Trade-offs

- [A top-level typo no longer stops the command] → it is reported on every
  run of `codev status`, by `codev doctor`, and fails
  `codev validate --strict`, which CI runs.
- [The known-key lists drift from the structs] → the "every known key"
  test fails as soon as a field is added without its name.
- [The lowest admitted version may not be a published release] → it is the
  third choice, after the cache and an exact bound; `codev self-update
  --version` fails cleanly with `update_download_failed` if it does not
  exist, and the docs say to pick a release from the releases page.
- [Line numbers come from a text scan, not the YAML parser] → only used to
  point at the key; a miss falls back to line 1, never to a wrong file.
- [A new doctor check changes the count of every doctor report] → ids are
  additive in the v1 contract; docs samples are updated.

## Migration Plan

None for projects. Teams that add a key known only to this release or a
later one should set `codev_version` accordingly: codev 0.5 and earlier
still reject the file, and the documentation says so.
