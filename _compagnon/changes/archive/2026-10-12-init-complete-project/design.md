# Design: A complete project from `codev init` — inherited conventions, repository integration, `codev doctor`

## Context

`commands::init` in `codev-cli` writes `_codev/config.yaml` only when it is
missing, from `codev_core::config::from_detected` and `render` (a hand-written
YAML renderer with provenance comments), then executes the scaffolding plan
and the skills plan of `codev-agents::claude::ClaudeCode`. `codev update`
runs the scaffolding and skills plans only. Every write goes through a
`Plan` executed by `codev_engine::apply::execute`, which reports created,
overwritten and untouched files and skips an `Overwrite` whose content is
identical.

Inherited sources are declared as `inherits:` entries with `path:` or
`git:` + a **mandatory** `ref:` (`config::classify` refuses a git source
without one; `GitSourceInput::from_inherits` ignores it). The existing code
cannot resolve a remote's default branch: `run_sources_update` runs
`git ls-remote <url> <ref>`, fetches the SHA into a bare cache and writes
`_codev/codev.lock`. It is the only code path that spawns `git`, and it
returns an error as soon as one source fails.

The init probe (`codev_engine::detect::run`) is read-only and spawns no
process; it already detects a non-empty `.github/workflows/`. Nothing
detects a forge from the git remote, `.gitlab-ci.yml`, or a pull request
template. The skills carry their generator version in their frontmatter
(`stamped_version`, private to `claude.rs`).

## Goals / Non-Goals

**Goals:**

- Every integration write is planned by a pure function and executed by
  the shell, with in-memory tests.
- Re-running `codev init` or `codev update` is idempotent and never
  rewrites a byte outside codev's markers.
- `codev doctor` is read-only and offline.

**Non-Goals:**

- Resolving a remote's default branch: `main` is the default ref, an
  explicit `@<ref>` covers the rest.
- Editing an existing `config.yaml` to add `--inherit` sources (comments
  and layout would not survive a YAML round trip).
- Merge request templates and CI files for other forges (GitLab gets a
  printed snippet only), `--dry-run` for integration.
- Doctor fixing anything, or checks beyond the ten listed in the spec.

## Decisions

### Decision: Managed sections are a pure splice in `codev-core`

A new `codev_core::managed` module holds the markers and one pure function,
`splice(existing, body, mode) -> Splice`, returning `Created`, `Appended`,
`Refreshed`, `Unchanged` or `Malformed`, with the new file content. The
begin marker is a line starting with `<!-- codev:begin` (so the marker line
can carry a short explanation), the end marker the line
`<!-- codev:end -->`. Exactly one begin followed by one end is a section;
anything else is `Malformed` and the file is left alone. `mode` is `Init`
(create or append when absent) or `Refresh` (only replace an existing
section, never create or append) — that is how `codev update` honours an
opt-out it has no record of. For the workflow file, which codev owns
entirely, the marker is a first line `# codev:managed`; a file without it
belongs to the user.

Following `_codev/decisions/0001-functional-core-imperative-shell.md`, the
splice decides and the plan carries the result as an `Overwrite` or
`CreateOnly` write; `apply::execute` stays the only writer.

**Alternatives considered**: a whole-file `CLAUDE.md` owned by codev (would
fight every team that already has one); a separate `CLAUDE.codev.md`
imported from `CLAUDE.md` (still needs an edit of the user's file, and two
files to keep in sync); markers without an end line, "everything after the
marker is codev's" (any text a user adds below would be erased).

### Decision: `CLAUDE.md` belongs to `codev-agents`, CI and PR template to `codev-engine`

`CLAUDE.md` is Claude Code's file, and its section lists the installed
skills: `ClaudeCode::plan_claude_md` renders it next to the skills it
already plans. The CI workflow and the pull request template depend on the
forge, not on the agent: `codev_engine::integration` probes the repository
(`RepoFacts`: `.github/`, `.gitlab-ci.yml`, the `url =` lines of
`.git/config`, the workflow files, the template candidates) and plans both
files with pure functions over those facts. Forge detection from the git
config text is a pure parser in `codev_core::detect::forge`. The CLI
combines the three plans into one and reports each step as an
`IntegrationStep { step, action, path, reason }`. This follows the crate
graph of `_codev/decisions/0002-crate-graph-as-dependency-rule.md`.

`--no-detect` does not disable this probe: it reads only the files the
integration is about to write or must not overwrite, as the skills plan
reads the existing skills.

**Alternatives considered**: everything in `codev-cli` (pure planning in
the shell, untestable without a harness); reading `git remote -v` (a
process spawn in a step the init spec keeps process-free).

### Decision: CI is skipped when a workflow already runs `codev validate`

The CI plan scans `.github/workflows/*.yml|*.yaml` for a line naming
`codev` then `validate` (case-sensitive words, so
`cargo run --bin codev -- validate` counts). If one is found outside
`codev.yml`, nothing is written and the reason names that file — this is
what keeps codev's own repository, whose `ci.yml` already validates, free
of a duplicate workflow. The same matcher serves the doctor `ci` check, so
init and doctor can never disagree. The generated workflow installs codev
with `install.sh` and `CODEV_VERSION=<running version>`, adds
`~/.local/bin` to `GITHUB_PATH`, and runs `codev validate --strict` on
`push` and `pull_request`. `codev update` rewrites it only when its first
line is `# codev:managed`, which bumps the pinned version after an upgrade.

**Alternatives considered**: always writing `codev.yml` (duplicate jobs);
a reusable action (`uses: mairistem/codev@v…`, a new artifact to publish
and maintain); `cargo install codev` in CI (minutes of compilation per run).

### Decision: `--inherit` is parsed in core, validated before any write, pinned through `run_sources_update`

`codev_core::config::inherit::parse(raw) -> Result<InheritArg, String>`
classifies the value (git prefixes, `.git` suffix after stripping the ref)
and splits `@<ref>` at the last `@` of the *path part* — after `://host/`
or after the `:` of an scp-like `user@host:path` — so the user name is
never taken for a ref. `commands::init` runs it, together with the
local-path existence check that needs the file system, before the first plan
is executed, and fails with `invalid_inherit` — not as a clap value parser,
whose error would carry no stable code and no JSON document. A relative path is made
absolute against the current folder, because `config::resolve` reads paths
relative to the process's folder and init may be the only time the user's
intent is unambiguous.

`GeneratedConfig` gains `inherits: Vec<GeneratedValue<InheritArg>>`,
rendered after `mcp:` with the provenance comment once above `inherits:`;
the engine round-trip test reads it back.

Once the configuration and the scaffolding are written, and before the
skills resolve the configuration (so that resolution already sees the pins
and raises no `git_source_unlocked` warning), init calls the existing
`sources::run_sources_update` with the git sources it has just written,
through the `ProcessRunner` port now carried by `Ctx`. Following
`_codev/decisions/0005-read-only-inherited-sources.md`, which makes
`codev sources update` the only command that **moves** a pin, init pins
only when `_codev/codev.lock` does not exist yet: it can add pins, never
move one. Any error becomes the `inherit_pin_failed` warning and the human
output adds `help: run codev sources update`. Inherited content stays
read-only, pinned by SHA, and markdown/YAML only — nothing here changes
what is read from a source.

**Alternatives considered**: `ref: HEAD` by default (works with
`ls-remote`, but records a ref that reads as "whatever is current", which
0005 argues against); `git ls-remote --symref` to discover the default
branch (a second network round trip, and the result would still be written
as a branch name — deferred); failing init when pinning fails (an offline
laptop could not initialize a project).

### Decision: `codev doctor` gathers facts in the shell and evaluates them in core

`codev_core::doctor::evaluate(&DoctorFacts) -> Vec<Check>` is pure: facts
in (rules present, language set, skill states, spec count, accepted
decision count, source states and stale pins, managed-section presence,
CI files matching, forge, validate error and warning counts), checks out,
each with `id`, `group`, `status`, `code`, `message`, `fix`. The CLI's
`commands::doctor` gathers the facts through existing engine functions
(`config::resolve`, `specs`, `decisions` index, `sources::list_source_states`
plus the lock, `validate`), `ClaudeCode::skill_states` (which exposes the
stamped version) and the `integration` probe. Doctor never runs
`run_sources_update`, so it never spawns `git`.

The JSON contract gains `DoctorReportV1` in `contract.rs` (v1, additive);
`SetupV1` gains `integration: Vec<IntegrationStepV1>`. Ids and codes are
`&'static str` constants, listed in the JSON reference.

`codev init` and `codev status` reuse `commands::doctor` for their hint and
ignore its failure: a hint must never break the command.

**Alternatives considered**: doctor as a skill (not scriptable, not usable
in CI with `--strict`); folding the checks into `codev validate` (validate
judges the planning artifacts and fails CI; most doctor checks are
adoption advice that must not fail a build); a fourth status
`not_applicable` for `pr_template` (the check is simply omitted outside
GitHub, which keeps three statuses).

### Decision: `/codev-onboard` reads `codev doctor --json`

The onboard workflow adds one step after the state commands. It already
has `Bash(codev:*)`, so no permission changes, and it relays the `fix`
strings instead of re-deriving them: the CLI stays the single source of
truth (`_codev/decisions/0004-single-identity-for-skill-and-command.md`:
no new skill).

**Alternatives considered**: a `/codev-doctor` skill (a second entry point
for the same report).

## Risks / Trade-offs

- [Re-running `codev init` recreates a `codev.yml` or `CLAUDE.md` section a
  user deleted on purpose] → `--no-ci`, `--no-claude-md`; `codev update`
  never recreates; documented in the FAQ.
- [The generated workflow pins a version that is not released yet (a dev
  build)] → The version is the running binary's, as the spec requires;
  `codev update` after installing a release rewrites it.
- [A user edits inside the markers and loses the edit on `codev update`]
  → The begin marker line says the block is managed and refreshed by
  `codev update`.
- [The doctor hint in `codev status` costs a full validation] → Status is
  human-only for the hint; the JSON path skips the doctor entirely.
- [`main` is wrong for a repository whose default branch is `master`] →
  Pinning fails with the ls-remote message, init succeeds, and the warning
  says to fix the ref and run `codev sources update`; `@<ref>` is
  documented.
- [Init now spawns `git`, which the sources module said only
  `sources update` does] → Only with `--inherit` and a git source; the
  module documentation is updated.

## Open Questions

- Should init resolve the remote's default branch (`ls-remote --symref`)
  instead of assuming `main`? Deferred: `@<ref>` covers it today.
- Should a relative `--inherit` path be written relative to the project
  root rather than absolute (portable across machines) once
  `config::resolve` resolves relative paths against the project root?
