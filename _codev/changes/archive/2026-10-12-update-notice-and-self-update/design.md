# Design: Update notice and `codev self-update`

## Context

- codev ships as one static binary per target, published by
  `.github/workflows/release.yml` as `codev-<version>-<target>.tar.gz`
  (`.zip` on Windows) next to a `SHA256SUMS` file. `install.sh` and
  `install.ps1` resolve the latest release through
  `https://api.github.com/repos/mairistem/codev/releases/latest`, download,
  verify the SHA-256 and copy the binary to `~/.local/bin` or
  `%LOCALAPPDATA%\Programs\codev`.
- The engine reaches the outside world only through ports
  (`FileSystem`, `Env`, `Clock`, `ProcessRunner`), with in-memory doubles
  (`MemoryFileSystem`, `FixedEnv`, `FixedClock`, `MockProcessRunner`).
  The only network access so far is `git`, driven through `ProcessRunner`
  by `codev sources update` (decision 0005).
- A per-user cache already exists for inherited sources:
  `codev_engine::sources::cache::root` (`$XDG_CACHE_HOME/codev/`, otherwise
  `~/.cache/codev/`).
- `codev doctor` is specified offline; `codev status --json` is read by
  the installed skills, through the v1 contract in `codev-cli::contract`.
- `ProjectConfig` denies unknown fields.

## Goals / Non-Goals

Goals: one daily, anonymous, cached check; one explicit command that
installs a verified release; every decision unit-tested without network
or disk.

Non-goals: background or silent updates; updating installations codev did
not make (cargo, package managers); signature verification beyond the
`SHA256SUMS` the installers already trust; a `--json` on every command for
the notice (only `status` carries it).

## Decisions

### Decision: Decide in the core, execute in the shell

Following decision 0001, `codev-core::update` is pure and owns every
decision: version and requirement parsing, whether to check (`CI`,
opt-out, 24-hour freshness), the notice (`available`, `snoozed`), the
snooze record, the target asset name for a target triple, the
`SHA256SUMS` lookup and the SHA-256 comparison, the installation method
from the binary's path, and the self-update decision (`up to date`,
`refuse downgrade`, `install X`). `codev-engine::update` only gathers
facts and executes: it reads and writes the cache, calls the `Http` port,
writes the staged binary, swaps it, and runs the new binary's
`codev update`. `codev-core` gains no I/O; the cache format is a plain
struct serialized with `serde_json`, already a core dependency.

**Alternatives considered**: a single `self_update()` function mixing
decisions and effects — rejected by decision 0001, and it would make the
downgrade and checksum rules testable only with real downloads.

### Decision: HTTP through `curl`, behind a new `Http` port

A new `Http` port (`get(url, timeout) -> bytes`) is implemented by
`CurlHttp`, which runs `curl -fsSL --max-time <s> -H "Accept: …"` through
the existing `ProcessRunner` and returns stdout as bytes; tests use an
in-memory `MemoryHttp` keyed by URL. Archives are extracted with `tar`
through the same port (`tar -xzf` for `.tar.gz`; `tar -xf` reads `.zip`
with the bsdtar shipped in Windows 10 1803+ and macOS).

Binary size delta: **0 bytes** of dependencies for HTTP and extraction.
The measured alternative — `ureq` 3 with `rustls`, `ring` and
`webpki-roots` — adds about **2.2 MB** to a release binary (0.43 MB →
2.70 MB for a minimal GET program), before `flate2`, `tar` and `zip` for
extraction, and brings `ring`'s C and assembly build into the musl and
Windows cross builds. `curl` and `tar` are already the installers'
requirements (`install.sh` needs both); Windows 10+ ships both
(`curl.exe`, `tar.exe`). The same pattern as `git` for inherited sources
(decision 0005): a system binary behind a port, which reuses the user's
proxy configuration (`HTTPS_PROXY`, `.curlrc`) for free.

The only new crate is `semver` (pure, no dependencies), for Cargo's
requirement syntax in `codev_version` — the syntax Rust developers already
know, and one we should not reimplement.

**Alternatives considered**: `ureq` + `rustls` (+2.2 MB, cross-build
cost, own TLS roots to keep current); `reqwest` (tokio, larger still);
reimplementing semver requirements (error-prone edge cases with `^0.x`
and pre-releases).

### Decision: An `Executable` port for the running binary

Replacing a binary needs bytes, permissions and platform-specific swaps
that `FileSystem` (text-only, deliberately narrow) does not offer. A new
`Executable` port exposes `current_exe`, `temp_dir`, `write_bytes`,
`read_bytes`, `remove_dir_all`, `remove_file` and `replace(staged,
target, aside)`. `RealExecutable` implements them with `std::fs`;
`MemoryExecutable` records bytes and swaps for tests.

Atomicity: the new binary is written next to the target
(`<dir>/.codev-<version>.new`), made executable (0755 on Unix), then
renamed over the target. A same-directory `rename` is atomic on every
supported platform, and it gives the new binary a new inode, which keeps
macOS's code-signing cache from killing the replaced process. On Windows a
running `.exe` cannot be overwritten but can be renamed: the target is
first renamed to `codev.old.exe`, the staged file renamed into place, and
on failure the old one is renamed back. Every codev start on Windows
removes a leftover `codev.old.exe` next to the running binary, ignoring
errors. Write permission is detected when the staged file cannot be
written (`update_not_writable`), before anything is moved.

**Alternatives considered**: overwriting the file in place (not atomic,
fails on Windows, breaks macOS signatures); the `self-replace` crate
(another dependency for ~40 lines we can test); asking the user to rerun
the installer (the very chore this change removes).

### Decision: Installation method from the binary's path

`install_method(exe, cargo_home, home)` classifies the canonical path of
the running binary, separators normalized to `/`: under `$CARGO_HOME/bin`
or `~/.cargo/bin` → `cargo`; containing `/target/debug/` or
`/target/release/` → `build`; under `/opt/homebrew/`, `/usr/local/Cellar/`,
`/home/linuxbrew/`, `/nix/store/`, `/scoop/apps/` or `/usr/bin/` →
`package_manager`; anything else → `standalone`. Only `standalone` is
replaced. A path test is the only signal available offline, and it errs on
the side of not replacing.

**Alternatives considered**: a marker file written by the installers
(older installs have none, and it can drift); asking the package manager
(slow, platform-specific).

### Decision: `codev status` is the only command that checks; doctor reads the cache

The check runs in `codev status` (human and JSON) and in `codev
self-update`, nowhere else, so every other machine output stays byte-for-
byte stable and only one command pays the timeout, at most once a day.
`codev doctor` keeps its offline guarantee: its `codev_version` check
reads the cache only. A failed check is recorded with its time and the
previously known release, so an offline laptop makes one attempt a day,
not one per command.

The cache lives at `<codev cache root>/update-check.json`, reusing
`sources::cache::root`: `{ "checkedAt": <unix seconds>, "latest":
"0.6.0", "url": "…", "snoozedVersion": "0.6.0", "snoozedUntil": <unix
seconds> }`. A missing or unreadable cache is treated as "never checked";
a write failure is ignored (the check simply runs again next time).

The `Clock` port gains `now_unix()`, whose default derives midnight UTC
from `today()`, so `FixedClock` stays a one-liner in tests and
`SystemClock` overrides it with the system time.

**Alternatives considered**: checking in every human command (noisy, and
the brief's "machine outputs stay stable" would need per-command
exceptions); checking in doctor (breaks its offline spec); a background
thread (a detached process outliving the command is precisely the kind of
behind-the-back activity we reject).

### Decision: `self-update` runs the new binary's `codev update`

After the swap, the running process is still the old version: calling
`commands::update` in-process would stamp the skills with the old version.
The engine runs `<exe> update` through `ProcessRunner` in the project
root, so the new binary regenerates skills, managed sections and the
version pinned in `.github/workflows/codev.yml`. A non-zero exit becomes
the warning `update_project_refresh_failed`.

**Alternatives considered**: in-process update (wrong version stamp);
printing "now run `codev update`" (one more manual step, the one users
forget).

### Decision: The skill notice is one shared paragraph

The notice lives once in `assets/workflows/_update-notice.md` and is
substituted for a `{{UPDATE_NOTICE}}` placeholder in the five action
workflows by `codev-agents::claude`, like `{{JIRA_MCP_TOOL}}`. It is
placed right after each workflow's first `codev status --json`; the apply
workflow already calls status before reading tasks, so the question comes
before the first task. "Update now" runs `codev self-update --yes`
because the skill already asked; allowed tools stay `Bash(codev:*)`
(decision 0004 is untouched: skill and command keep one identity).

**Alternatives considered**: copying the paragraph into each workflow
(five copies to keep in sync); a separate `/codev-self-update` skill (the
user would have to know it exists, which is the problem).

### Decision: `codev_version` is validated at load, like `language`

`ProjectConfig` gains `codev_version: Option<String>`, parsed with
`semver::VersionReq` in `config::load`; an invalid value fails with the
new `EngineError::InvalidCodevVersion` (`invalid_codev_version`), so the
error surfaces on the first command rather than only in doctor. The value
is project-only, never inherited: the team writing in this repository
owns it, as with `language` and `mcp`.

**Alternatives considered**: a doctor-only check (an invalid value would
silently disable the alignment it is meant to enforce); inheriting it
from sources (an inherited source pinning tool versions for every
inheriting project is a policy decision nobody asked for).

### Decision: `--check` exits 0 when an update is available

An available update is information, not a failure, the same rule as
doctor's warnings; `status --json`'s `update` and `self-update --check
--json`'s `action` are the machine-readable signals. Exit 1 means the
check itself failed.

**Alternatives considered**: exit 1 when an update exists, like
`git diff --exit-code` — it would make a scripted `--check` fail CI jobs
for a non-problem.

## Risks / Trade-offs

- [`curl` or `tar` missing on a minimal Linux image] → the background
  check fails silently; `self-update` fails with
  `update_download_failed` / `update_extract_failed` naming the missing
  program, and the installers already require both.
- [Codev 0.5 and older reject `codev_version` (unknown field)] → the
  error names the key; the docs say to add `codev_version` only once the
  team runs 0.6+, and this repository's CI builds codev from source.
- [GitHub API rate limit (60 requests/hour/IP, anonymous)] → one check
  per user per day; a 403 is a failed check, recorded and silent.
- [A compromised release replaces binaries and `SHA256SUMS` together] →
  same trust model as the installers (HTTPS to github.com); signatures
  are out of scope and noted in the FAQ.
- [`codev_version` with an upper bound below the running version] →
  doctor reports `codev_version_unsatisfied`; `self-update` refuses the
  implicit downgrade, and the docs show `--version X.Y.Z`.
- [Path heuristics misclassify an installation] → it errs towards "not
  standalone", printing a command instead of replacing; `--version` does
  not bypass it.

## Migration Plan

No migration: the cache is created on the first check, `codev_version` is
opt-in, the JSON objects are additive, and existing skills keep working
until `codev update` regenerates them. Rollback: `codev self-update
--version <previous>`.

## Open Questions

- Should the notice also appear in other human commands (e.g. `codev
  list`)? Deferred: `codev status` covers the skills and the terminal
  workflow; widening later changes no contract.
