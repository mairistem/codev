# Design: Rename codev to Tandem

## Context

The name "codev" is spread across ~1,500 files and ~52,000 occurrences:
the four crates and the evaluation crate, the planning root constant
(`PLANNING_DIR = "_codev"` and `LOCK_FILE = "codev.lock"` in
`crates/codev-core/src/layout.rs`), the skill prefix
(`SKILL_PREFIX` in `crates/codev-agents/src/claude.rs`), the managed
markers, the JSON codes (`no_codev_root`, `codev_version_unsatisfied`, …),
fifteen `CODEV_*` variables, the update URLs (`RELEASES_API`,
`DOWNLOAD_BASE` in `crates/codev-core/src/update.rs`), the asset names
(`asset_name`, `archive_root`, `binary_name`), `INSTALLER_URL` and the
generated `.github/workflows/codev.yml` (`integration.rs`), the installers,
the release workflow — and, by volume, the evaluation baselines (31k), the
archived changes (7.7k), the evaluation cases (3.4k), the specs (2.2k) and
the documentation (3.2k).

Three behaviors of the current code matter for the rename:

- `root::discover` takes the first ancestor holding the planning root; with
  a renamed constant, a codev project would simply look like "no project".
- `changelog` lists the archive with `git ls-tree` **at the current
  path**: at a tag taken before the rename, `_tandem/changes/archive/` does
  not exist, so every archived change would be listed as new.
- `self-update` of an installed codev 0.6 asks the old repository's latest
  release for `codev-<version>-<target>.<ext>`; GitHub redirects the
  renamed repository's API and downloads, but a release holding only
  `tandem-*` archives would leave it with "no asset".

The evaluation runner builds each case from this repository's tree at a
pinned `ref`: every ref before the rename holds `_codev/`.

Decisions in force that frame the work: 0001 (functional core, imperative
shell), 0002 (the crate graph is the dependency rule), 0003 (the planning
root is a visible `_`-prefixed folder, one constant), 0004 (one identity
for skill and command), 0005 (inherited sources are read-only), 0008
(every change to what the agents read is evaluated), 0013 (files written
for a newer version).

## Goals / Non-Goals

**Goals**

- After the change, `rg -i codev` over the repository returns only the
  deliberate exceptions listed below.
- A codev 0.6 user reaches a working Tandem project with two commands:
  `codev self-update` (or the installer), then `tandem migrate`.
- Nothing is read from or written to a codev layout without the user
  asking for it.

**Non-Goals**

- No long-lived compatibility: no alias binary, no fallback reading of
  `_codev/`, `CODEV_*` or `codev_version`. The legacy archives stop at 0.8.
- No change of behavior other than the rename, the migration and the
  bridge; the JSON contract stays v1 (codes renamed, shapes unchanged).
- `tandem migrate` does not rewrite the user's prose.

## Decisions

### Decision: The legacy names live in one module, the only place "codev" survives in code

`tandem-core` gains a `legacy` module holding every former name the
migration and the bridge need: `PLANNING_DIR = "_codev"`,
`LOCK_FILE = "codev.lock"`, `VERSION_KEY = "codev_version"`,
`SKILL_PREFIX = "codev"`, the former managed markers, the former workflow
path and the former binary stem. Every other constant is renamed in
place (`layout.rs`, `claude.rs`, `update.rs`, `integration.rs`, …), as
0003 intended: "changing it costs a recompilation, not a refactoring".

The deliberate exceptions to "no codev left" are therefore: this module
and its tests, the rename notice (whose text lives in the module), the
migration guide and the `tandem migrate` page of both CLI references, the
0.7.0 CHANGELOG entry and release note, the legacy-archive step of
`release.yml` and the legacy branch of both installers, decisions 0003
and 0014 (see below), and this change's whole folder. Tests anywhere else
that need a former name build it from the `legacy` constants, never from
a literal, so the sweep cannot rewrite their fixtures. The one
`MODIFIED` heading holding the former name — the changelog delta's — is
renamed by hand together with the main spec's.

**Alternatives considered**: keeping legacy names next to each current
constant — scatters the exceptions and makes the final `rg -i codev`
audit unreadable.

### Decision: `tandem migrate` is a pure plan executed by the engine

Following 0001, `tandem-core::migrate::plan(facts) -> MigrationPlan`
decides from facts the engine gathers (which roots exist, the config
text, the lock file, the installed skills with their generated/hand-edited
state as `update` already computes it, the `CLAUDE.md` managed block, the
workflow file) and returns ordered effects — computed from what is still present, so a
second run completes an interrupted one — `Rename` (root, then lock),
`Write` (config with the key renamed by a line edit that keeps comments —
the same line-preserving approach as `yaml_lines.rs`; review and
correction markers; skills; managed blocks of `CLAUDE.md` and of the pull
request template; workflow), `Remove`
(generated `codev-*` skills and `codev.yml`), `Keep` (hand-edited ones,
reported `kept_modified`). The planner refuses up front, with
`migration_version_unsatisfied`, a `codev_version` the running version
does not satisfy. The engine executes the effects with the existing
`FileSystem` port and stops at the first failure with
`migration_incomplete`, listing what was applied; `--dry-run` prints the plan without executing it; the
JSON output is a new `migrate` document in contract v1.

Removing a skill leaves an empty folder: the `FileSystem` port gains
`remove_dir` (empty directories only), implemented by the real and the
in-memory file systems (0002: still one port, one more method).

Skills and the managed blocks are rendered by the same code as
`tandem init` and `tandem update`, so a migrated project is identical to
a freshly initialized one.

**Alternatives considered**: a shell script shipped with the release —
untestable with the in-memory ports and blind to hand-edited skills; doing
the migration implicitly on the first command — rejected by the user:
nothing happens on a codev layout without being asked.

### Decision: Root discovery knows the former root only to refuse it

`init` refuses with `legacy_root` when its target holds `_codev/` and no
`_tandem/`, so it never creates a second, empty root beside a codev one.
`root::discover` checks, at each ancestor, `_tandem/` first; if absent and
`_codev/` is present it returns `EngineError::LegacyRoot { dir }`
(`legacy_root`), naming `tandem migrate`. `tandem migrate` uses its
own discovery that accepts the former root. The sources loader applies the
same test to each resolved source and fails with `legacy_source`;
it never writes to a source (0005).

**Alternatives considered**: reading `_codev/` with a warning — rejected
by the user (no silent use of the former layout) and it would make every
command carry two layouts.

### Decision: Below 0.7.0, installers and self-update use the former names

Codev projects' CI jobs fetch `install.sh` from `main` with
`CODEV_VERSION=<pin>` and then call `codev`
(`integration.rs`, `render_gitlab_snippet`). The installers therefore
read `TANDEM_VERSION`, else `CODEV_VERSION`; for a version below `0.7.0`
they fetch `codev-*` archives and install `codev`; with only
`CODEV_VERSION` set they install under the name `codev`. In the core,
`asset_name`, `archive_root` and `binary_name` take the version and pick
the `legacy` names below `0.7.0`, so `tandem self-update --version 0.6.1`
is a working rollback. This is release history, not a project layout:
the no-silent-use rule is about projects.

**Alternatives considered**: publishing the new installers under a new
path and freezing the old one — two installers to keep, and the old one
would never learn about Tandem.

### Decision: The bridge is a set of legacy-named archives in the 0.7 releases

`release.yml` builds `tandem` once per target and, when the tag starts
with `v0.7.`, also packages the same binary as `codev[.exe]` in
`codev-<version>-<target>/` and archives it as
`codev-<version>-<target>.<ext>`, listed in `SHA256SUMS`. A codev 0.6
binary's unchanged `self-update` then finds, verifies and installs it in
place. At start-up, `tandem-cli` compares the executable's file stem with
`legacy::BINARY_STEM` and, when it matches, prints one notice on stderr
before running the command.

GitHub's redirect from `mairistem/codev` to `mairistem/tandem` serves the
old API and download URLs; the bridge relies on it and on nothing else.

**Alternatives considered**: a last codev 0.6.2 release announcing the
rename — reaches only users who update before 0.7.0 is published, and
needs a second release; a `codev` wrapper binary — a second artifact to
build and maintain for a one-time move.

### Decision: `tandem changelog` reads the former archive path at refs that predate the rename

`ls_tree` at a ref first lists `_tandem/changes/archive/`; when git reports
that path absent at that ref, it lists `_codev/changes/archive/`. This is
history read from git, not a layout the user works in, so it does not
contradict the no-silent-use rule. The one archived change whose folder
name contains "codev" (`2026-10-23-eval-codev-binary-name`) is renamed by
the sweep and added to `changelog.exclude` under its new name, so that it
is not listed as new in 0.7.0.

**Alternatives considered**: renaming nothing in the archive — contradicts
the request; excluding the whole pre-rename archive in config — a growing
list to maintain by hand.

### Decision: The repository is converted by dogfooding `tandem migrate`, then a mechanical sweep

Order of operations on this repository:

1. Implement the code changes under the old crate names (tests green).
2. Rename crates, binary and identifiers (`git mv crates/codev-* …`,
   `Cargo.toml`, `use codev_core` → `use tandem_core`).
3. Run the freshly built `tandem migrate` on this repository: the planning
   root, lock, key, markers, skills, `CLAUDE.md` and workflows move as any
   user project would — this change's own folder moves to
   `_tandem/changes/rename-to-tandem/`.
4. Sweep every remaining text file with an ordered, case-aware mapping
   (`CODEV`→`TANDEM`, `Codev`→`Tandem`, `codev`→`tandem`), the exceptions
   above excluded, and rename every path holding `codev` with `git mv`.
   The product name is written **Tandem** in prose and `tandem` in
   commands, paths and identifiers.
5. Re-seal the decisions whose body the sweep changed with
   `tandem decision seal <id> --force` — the sanctioned way to rewrite a
   sealed body — and record why in a new ADR 0014 "The product is named
   Tandem", accepted, which supersedes 0003 on the name of the planning
   root (keeping its visible, `_`-prefixed rationale) and states that the
   other bodies were re-sealed for the rename only. 0003's body is left
   as written: it is the record of a choice made under the former name,
   and rewriting it would make it state a false history.
6. `rg -i codev` audit: every hit must be one of the listed exceptions.

The main specs are converted by the sweep, not by `MODIFIED` deltas: the
rename touches about 250 of the 278 requirements and 69 of their headings,
and copying them into deltas would bury the four capabilities whose
behavior really changes (`migration`, `changelog`, `distribution`,
`docs`). Those deltas keep the existing headings verbatim, so that they
still match the main specs at validation time; after the sweep, the
changelog delta's heading is renamed by hand like the main spec's, so
that `tandem validate --strict` still finds every `MODIFIED` target
before archive. The delta bodies, which describe the former layout on
purpose, are not swept.

**Alternatives considered**: `MODIFIED` blocks for every requirement — a
delta of the size of the specs, unreviewable; leaving specs, decisions and
archives untouched as history — contradicts the request.

### Decision: Evaluation fixtures are migrated, then compared, as 0008 requires

The runner (`tandem-evals`) runs `tandem migrate` on a case's directory
right after building it from its `ref` when that tree holds `_codev/` —
the same path a user takes — then writes the case's `change/`/`overlay/`
files under `_tandem/`. The sweep renames the cases and the baselines'
recorded prompts. Because every skill text changes (0008), the PR pastes
`tandem-evals compare` for each step; a step whose quality leaves the
baseline's range, or whose token counts move beyond the baseline's spread
because "tandem" and "codev" tokenize differently, is re-recorded with
`tandem-evals baseline`.

**Alternatives considered**: re-pinning every case to a post-rename ref —
loses the states the cases were written for.

### Decision: The Tandem mark — two wheels, one frame

A square pictogram, built like the Mairistem mark from circles and
straight bars on a 250-unit grid: two wheels of equal size, the rear one
corail `#ff5331` (the person), the front one vert `#00c39b` (the agent),
joined by a bleu `#004c66` frame whose front end points forward, with a
jaune `#ffab03` dot for the shared plan. Files: `docs/assets/tandem-mark.svg`
(color, light backgrounds), `docs/assets/tandem-mark-dark.svg` (bleu
parts in white), `docs/assets/tandem-logo.svg` and
`tandem-logo-dark.svg` (mark plus the word "tandem"), and
`docs/theme/favicon.svg` replaced by the mark. The README uses a
`<picture>` with `prefers-color-scheme`; the mdBook header and the landing
page inline the mark as an SVG `<symbol>` next to the existing Mairistem
symbol, with CSS custom properties for the dark variant, as `header.hbs`
already does for the Mairistem logo. The offline manual inlines it.

**Alternatives considered**: a wordmark alone — the user chose a
dedicated mark; keeping only the Mairistem logo — gives the product no
identity of its own.

## Risks / Trade-offs

- [A codev 0.6 user never runs `self-update` before 0.8.0] → the update
  notice of 0.6 still points to the release page, which the redirect
  serves; the README, the migration guide and the 0.7.0 release note give
  the installer command.
- [GitHub's redirect is lost if a new `mairistem/codev` repository is ever
  created] → the manual rename task says never to re-create that name.
- [The sweep rewrites a string that must not change, e.g. inside a
  baseline's hash or a third-party name containing "codev"] → the sweep
  prints every file changed; `rg -i codev` before the sweep is reviewed
  for false positives (none found in the investigation outside our own
  names), and the test suite plus `tandem validate --strict` plus
  `tandem-evals compare` run after it.
- [Re-sealing decisions weakens their immutability guarantee] → limited
  to the rename, diff reviewed to contain only name changes, recorded by
  ADR 0014.
- [A user's shared source repository is not migrated yet, blocking
  consumers] → `legacy_source` names the source and the fix; the
  migration guide tells source maintainers to migrate first.
- [The name `tandem` collides with another command on users' `PATH`] →
  checked by hand before release (task); the installers install to the
  same folders as before, by full path.
- [Evaluation re-recording costs money — about 1.65 USD per planning
  case] → only steps whose compare fails are re-recorded.
- [JSON consumers outside the skills match `no_codev_root`] → listed as
  **BREAKING** in the release note and the migration guide.

## Migration Plan

1. Merge the change; rename the GitHub repository to `mairistem/tandem`
   (repository settings) right before tagging.
2. Tag `v0.7.0`: release with the `tandem-*` and the legacy `codev-*`
   archives.
3. Users: `codev self-update` (or the installer), rename the binary or
   reinstall as `tandem`, then `tandem migrate --dry-run` and
   `tandem migrate` in each project; source repositories first.
4. `v0.8.0` drops the legacy archives (the condition in `release.yml`);
   the `legacy` module stays, since `tandem migrate` needs it.

Rollback: before tagging, revert the merge. After tagging, a project
migrated by mistake goes back with `git checkout` of the files listed by
`tandem migrate` (its output names every path), and the binary with
`tandem self-update --version 0.6.1`. If the bridge fails for codev 0.6
binaries after `v0.7.0` is out, `v0.7.1` ships corrected legacy archives
and the release note gives the installer command meanwhile; before
tagging, the legacy archive layout is rehearsed with a 0.6.1 binary
against a draft release.

## Open Questions

- The final drawing of the mark (proportions, stroke weights) is
  validated visually by the user during implementation; the concept,
  palette and files above are settled.
