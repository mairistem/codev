# Self Update Specification

## Purpose

Lets a Compagnon user learn that a newer Compagnon release exists, decide when to
install it, and install it in one explicit command — and lets a team state
which Compagnon versions its project accepts. Compagnon never updates itself
without being asked.

## Requirements

### Requirement: `compagnon status` checks for a newer release at most once a day

`compagnon status`, in human and JSON output, SHALL know the latest published
Compagnon release by reading a per-user cache, stored in the Compagnon cache
directory (`$XDG_CACHE_HOME/compagnon/`, otherwise `~/.cache/compagnon/`), that
records the latest release (version and page URL) and the time of the
last check. When the last check is older than 24 hours, or there is
none, `compagnon status` MUST first make one HTTPS GET request to GitHub's
public "latest release" API of the Compagnon repository, with a timeout of
2 seconds, and record the result and the time in the cache.

The request MUST carry no information about the user or the project
beyond what any anonymous HTTPS request carries. A failed check —
offline, timeout, error status, unreadable answer — MUST NOT fail the
command nor print anything; it is recorded as a check at that time, so
that no new attempt is made for 24 hours, and the release previously
cached, if any, stays known.

#### Scenario: A recent check is reused

- **GIVEN** a cache whose last check is 2 hours old
- **WHEN** the user runs `compagnon status --change <name> --json`
- **THEN** no network request is made
- **AND** the `update` object reflects the cached release

#### Scenario: A stale check is refreshed once

- **GIVEN** a cache whose last check is 25 hours old
- **WHEN** the user runs `compagnon status --change <name>`
- **THEN** exactly one request to the latest release API is made
- **AND** the cache records the release it returned and the time of the
  check

#### Scenario: Offline, the check fails silently and is not retried

- **GIVEN** no cache and no network
- **WHEN** the user runs `compagnon status --change <name> --json` twice
- **THEN** both commands succeed with the same output as without the
  check, and no `update` object
- **AND** only the first one attempts a request

### Requirement: The update check is skipped in CI, on request, and outside `compagnon status`

`compagnon status` MUST NOT make the update check, nor read its cache, when
the environment variable `CI` is set to any value, or when
`COMPAGNON_NO_UPDATE_CHECK` is set to `1`; it then reports no update. No
command other than `compagnon status` and `compagnon self-update` SHALL ever
contact the release API.

#### Scenario: CI disables the check

- **GIVEN** `CI=true` in the environment and a stale cache
- **WHEN** the user runs `compagnon status --change <name> --json`
- **THEN** no network request is made
- **AND** the document has no `update` object

#### Scenario: The opt-out variable disables the check

- **GIVEN** `COMPAGNON_NO_UPDATE_CHECK=1` and no cache
- **WHEN** the user runs `compagnon status --change <name>`
- **THEN** no network request is made and no update line is printed

#### Scenario: Other commands never check

- **GIVEN** no cache
- **WHEN** the user runs `compagnon validate --json`, `compagnon list --json` or
  `compagnon doctor --json`
- **THEN** no network request is made

### Requirement: `compagnon status` reports an available update

When the latest release is known, `compagnon status --json` SHALL carry an
`update` object with `current` (the running version), `latest` (the
latest release version, without a leading `v`), `url` (the release page),
`available` (`true` when `latest` is greater than `current`) and
`snoozed` (`true` when the user snoozed this `latest` version and the
snooze has not expired). When the latest release is unknown, the
`update` object MUST be absent; every other field of the document is
unchanged. When `available` is `true` and `snoozed` is `false`, the human
`compagnon status` MUST print on stderr the line
``Compagnon <latest> is available (you have <current>) — run `compagnon self-update` ``.

#### Scenario: Update available

- **GIVEN** Compagnon 0.5.0 running and a cache knowing release 0.6.0
- **WHEN** the user runs `compagnon status --change <name> --json`
- **THEN** `update` is `{"current": "0.5.0", "latest": "0.6.0", "url":
  "<release page>", "available": true, "snoozed": false}`
- **AND** `compagnon status --change <name>` prints on stderr
  ``Compagnon 0.6.0 is available (you have 0.5.0) — run `compagnon self-update` ``

#### Scenario: Already up to date

- **GIVEN** Compagnon 0.6.0 running and a cache knowing release 0.6.0
- **WHEN** the user runs `compagnon status --change <name>`
- **THEN** `update.available` is `false` in the JSON
- **AND** no update line is printed

#### Scenario: Snoozed update

- **GIVEN** an available release 0.6.0 snoozed 2 days ago
- **WHEN** the user runs `compagnon status --change <name>`
- **THEN** `update.snoozed` is `true` in the JSON
- **AND** no update line is printed

### Requirement: `compagnon self-update` installs a release, verified, in place of the running binary

`compagnon self-update` SHALL resolve the target version — the one given with
`--version X.Y.Z`, otherwise the latest release, always asked to the
release API whatever the cache, `CI` or `COMPAGNON_NO_UPDATE_CHECK` — and,
when it differs from the running version, download the release archive
for the running platform and the release's `SHA256SUMS`, verify the
archive's SHA-256 against its `SHA256SUMS` entry, and replace the running
binary with the archive's binary. The replacement MUST be atomic: at every
moment, the binary's path holds either the previous binary or the new
one. On Windows, the previous binary is moved aside to `compagnon.old.exe`
next to it, and the next Compagnon run removes it.

On an interactive terminal without `--yes`, the command MUST ask for
confirmation before downloading; `--yes` skips the question, and a
non-interactive run proceeds, invoking the command being the consent.

After a successful replacement, inside a Compagnon project, the command
SHALL run the new binary's `compagnon update` in the project root, so that
skills, managed sections and the version pinned in
`.github/workflows/compagnon.yml` match the new version; a failure of that
step is reported as a warning `update_project_refresh_failed`, with the
fix `compagnon update`, and the exit code stays 0. Outside a project, that
step is skipped.

Failures MUST leave the installed binary untouched, exit with code 1 and
carry a stable code:

| situation | code |
|---|---|
| the release API cannot be reached or answers no release | `update_check_failed` |
| no published binary for this platform | `update_unsupported_platform` |
| the archive or `SHA256SUMS` cannot be downloaded | `update_download_failed` |
| `SHA256SUMS` has no entry for the archive, or the SHA-256 differs | `update_checksum_mismatch` |
| the archive cannot be extracted or holds no Compagnon binary | `update_extract_failed` |
| the binary's folder cannot be written | `update_not_writable` |
| `--version` is not `X.Y.Z` | `invalid_version` |

#### Scenario: Update to the latest release

- **GIVEN** Compagnon 0.5.0 installed by the installer, inside a Compagnon project,
  and a published release 0.6.0
- **WHEN** the user runs `compagnon self-update --yes`
- **THEN** the archive's SHA-256 is verified against `SHA256SUMS`
- **AND** the binary's path now holds Compagnon 0.6.0
- **AND** `compagnon update` of Compagnon 0.6.0 ran in the project root
- **AND** the exit code is 0

#### Scenario: Already up to date

- **GIVEN** Compagnon 0.6.0 running and 0.6.0 the latest release
- **WHEN** the user runs `compagnon self-update`
- **THEN** it prints that Compagnon 0.6.0 is up to date
- **AND** nothing is downloaded and the exit code is 0

#### Scenario: Checksum mismatch

- **GIVEN** a release whose archive does not match its `SHA256SUMS` entry
- **WHEN** the user runs `compagnon self-update --yes`
- **THEN** the command fails with the code `update_checksum_mismatch`
- **AND** the installed binary is unchanged

#### Scenario: Download failure

- **GIVEN** a release whose archive cannot be downloaded
- **WHEN** the user runs `compagnon self-update --yes`
- **THEN** the command fails with the code `update_download_failed`
- **AND** the installed binary is unchanged

#### Scenario: Read-only installation folder

- **GIVEN** a binary installed in a folder the user cannot write
- **WHEN** the user runs `compagnon self-update --yes`
- **THEN** the command fails with the code `update_not_writable`
- **AND** the installed binary is unchanged

#### Scenario: The project refresh fails after the update

- **GIVEN** a successful replacement inside a project whose
  `compagnon update` fails
- **WHEN** `compagnon self-update --yes` completes
- **THEN** a warning `update_project_refresh_failed` names `compagnon update`
- **AND** the exit code is 0

#### Scenario: Previous Windows binary cleaned up

- **GIVEN** a Windows installation just updated, with `compagnon.old.exe` next
  to `compagnon.exe`
- **WHEN** the user runs any Compagnon command
- **THEN** `compagnon.old.exe` is removed

### Requirement: `compagnon self-update` refuses to downgrade unless asked

When the target version is lower than the running version, `compagnon
self-update` MUST refuse with the code `update_downgrade_refused` unless
the version was given explicitly with `--version`, in which case it
installs it as any other version.

#### Scenario: Latest release older than the running binary

- **GIVEN** Compagnon 0.7.0 running, built ahead of the latest release 0.6.0
- **WHEN** the user runs `compagnon self-update --yes`
- **THEN** the command fails with the code `update_downgrade_refused`
- **AND** nothing is downloaded

#### Scenario: Explicit downgrade

- **GIVEN** Compagnon 0.6.0 running
- **WHEN** the user runs `compagnon self-update --version 0.5.0 --yes`
- **THEN** Compagnon 0.5.0 replaces the binary

### Requirement: `compagnon self-update` leaves other installation methods alone

`compagnon self-update` MUST NOT replace a binary it did not install. When the
running binary lives in cargo's binary folder (`$CARGO_HOME/bin`, otherwise
`~/.cargo/bin`), in a cargo build folder (`target/debug`,
`target/release`), or under a package manager's tree (Homebrew, Nix,
Scoop, or `/usr/bin`), it SHALL print which method installed it and the
command that upgrades it — `cargo install --path crates/compagnon-cli` from an
up-to-date clone for cargo, a rebuild for a build folder, the package
manager otherwise — and exit with code 0 without downloading anything.

#### Scenario: Installed with cargo

- **GIVEN** Compagnon running from `~/.cargo/bin/compagnon`
- **WHEN** the user runs `compagnon self-update`
- **THEN** it prints `cargo install --path crates/compagnon-cli`
- **AND** nothing is downloaded or replaced, and the exit code is 0

#### Scenario: Installed by the installer script

- **GIVEN** Compagnon running from `~/.local/bin/compagnon`
- **WHEN** the user runs `compagnon self-update --check`
- **THEN** the installation method is reported as standalone and the
  update can proceed

### Requirement: `compagnon self-update --check` and `--snooze` decide nothing for the user

`compagnon self-update --check` SHALL print the running version, the latest
release and whether an update is available, without downloading the
archive, and exit with code 0 whether or not an update is available; it
exits with code 1 only when the check itself fails
(`update_check_failed`). It refreshes the cache.

`compagnon self-update --snooze` SHALL record in the cache that the latest
known release is snoozed for 7 days, during which `compagnon status` reports
`snoozed: true` for that version and prints no update line. A release
newer than the snoozed one is not snoozed. When no update is available,
`--snooze` records nothing and says so; the exit code is 0 in both cases.

#### Scenario: Check with an update available

- **GIVEN** Compagnon 0.5.0 running and a published release 0.6.0
- **WHEN** the user runs `compagnon self-update --check`
- **THEN** it prints `0.5.0` and `0.6.0` and that an update is available
- **AND** the exit code is 0 and the binary is unchanged

#### Scenario: Check offline

- **GIVEN** no network
- **WHEN** the user runs `compagnon self-update --check`
- **THEN** the command fails with the code `update_check_failed`

#### Scenario: Snooze expires after 7 days

- **GIVEN** release 0.6.0 snoozed 8 days ago
- **WHEN** the user runs `compagnon status --change <name>`
- **THEN** the update line is printed again

#### Scenario: A newer release ends the snooze

- **GIVEN** release 0.6.0 snoozed yesterday and a cache now knowing 0.7.0
- **WHEN** the user runs `compagnon status --change <name> --json`
- **THEN** `update.latest` is `0.7.0` and `update.snoozed` is `false`

#### Scenario: Nothing to snooze

- **GIVEN** Compagnon up to date
- **WHEN** the user runs `compagnon self-update --snooze`
- **THEN** it says there is no update to snooze, and the exit code is 0

### Requirement: `compagnon self-update --json` follows the JSON contract v1

`compagnon self-update --json` SHALL print exactly one JSON document with
`current`, `latest` (`null` when unknown), `url` (`null` when unknown),
`action` — one of `updated`, `up_to_date`, `available` (with `--check`),
`snoozed`, `nothing_to_snooze`, `managed_elsewhere` —, `installMethod`
(`standalone`, `cargo`, `build`, `package_manager`), `path` (the binary's
path), `hint` (the upgrade command for another installation method,
otherwise `null`), `projectUpdated` (`true` when `compagnon update` ran in a
project) and the `status` array of every v1 document, which carries the
error codes and warnings. `--json` implies `--yes`.

#### Scenario: JSON after an update

- **WHEN** the user runs `compagnon self-update --json` and an update is
  installed
- **THEN** stdout is one JSON document whose `action` is `updated`

#### Scenario: JSON failure keeps the shape

- **GIVEN** no network
- **WHEN** the user runs `compagnon self-update --check --json`
- **THEN** stdout is one JSON document with `latest: null` and a `status`
  entry whose code is `update_check_failed`

### Requirement: A project can require a range of Compagnon versions

`_compagnon/config.yaml` SHALL accept an optional `compagnon_version` key holding a
semantic version requirement in Cargo's syntax (`">=0.5"`, `"^0.6"`,
`">=0.5, <0.8"`). When it is set, `compagnon status --json` SHALL carry a
`versionRequirement` object with `requirement` (the key's value) and
`satisfied` (whether the running version matches it); the object is
absent otherwise. A value that is not a valid requirement MUST make every
command that reads the configuration fail with the code
`invalid_compagnon_version`, naming the value.

#### Scenario: Requirement satisfied

- **GIVEN** `compagnon_version: ">=0.5"` and Compagnon 0.6.0 running
- **WHEN** the user runs `compagnon status --change <name> --json`
- **THEN** `versionRequirement` is `{"requirement": ">=0.5",
  "satisfied": true}`

#### Scenario: Requirement not satisfied

- **GIVEN** `compagnon_version: ">=0.7"` and Compagnon 0.6.0 running
- **WHEN** the user runs `compagnon status --change <name> --json`
- **THEN** `versionRequirement.satisfied` is `false`

#### Scenario: Invalid requirement

- **GIVEN** `compagnon_version: "latest"`
- **WHEN** the user runs `compagnon status --change <name>`
- **THEN** the command fails with the code `invalid_compagnon_version`

### Requirement: The commands that give a verdict refuse a Compagnon outside `compagnon_version`

When `_compagnon/config.yaml` sets `compagnon_version` and the running Compagnon does
not satisfy it, `compagnon validate`, `compagnon sync` and `compagnon archive` SHALL
refuse to run: they check and write nothing, exit with code 1 and report
the code `compagnon_version_unsatisfied`. The message MUST name the
requirement, the running version and the fix the `compagnon_version` check of
`compagnon doctor` gives for the same case — `compagnon self-update`, or
`compagnon self-update --version <X.Y.Z>` when the running Compagnon is above the
requirement. With `--json`, the command writes its usual document, the
code in its `status` array.

Every other command keeps working and reports the requirement as before:
`versionRequirement` in `compagnon status --json`, the `compagnon_version` check of
`compagnon doctor`. A project without `compagnon_version` is unaffected, and an
invalid `compagnon_version` still fails with `invalid_compagnon_version`. A Compagnon
that predates this requirement does not refuse; that cannot be tested from
this Compagnon, and the design records what it does instead.

#### Scenario: Validation refused below the requirement

- **GIVEN** a project with `compagnon_version: ">=99"`
- **WHEN** the user runs `compagnon validate --json`
- **THEN** the exit code is 1 and `status[0].code` is
  `compagnon_version_unsatisfied`
- **AND** the message names `>=99`, the running version and
  `compagnon self-update`

#### Scenario: Archive refused, nothing moved

- **GIVEN** a project with `compagnon_version: ">=99"` and an active change
  `add-auth` that passes validation
- **WHEN** the user runs `compagnon archive add-auth`
- **THEN** the command fails with `compagnon_version_unsatisfied`
- **AND** `_compagnon/changes/add-auth/` is still in place and the main specs
  are unchanged

#### Scenario: Sync refused, specs unchanged

- **GIVEN** a project with `compagnon_version: ">=99"` and an active change
  `add-auth` with a delta
- **WHEN** the user runs `compagnon sync add-auth`
- **THEN** the command fails with `compagnon_version_unsatisfied`
- **AND** the main specs are unchanged

#### Scenario: Above the requirement, the fix names a version

- **GIVEN** a project with `compagnon_version: "=0.0.1"`
- **WHEN** the user runs `compagnon validate`
- **THEN** the command fails with `compagnon_version_unsatisfied`
- **AND** the message names `compagnon self-update --version 0.0.1`

#### Scenario: Other commands keep working

- **GIVEN** a project with `compagnon_version: ">=99"` and an active change
  `add-auth`
- **WHEN** the user runs `compagnon status --change add-auth --json`
- **THEN** the command succeeds and `versionRequirement.satisfied` is
  `false`

#### Scenario: No requirement, no refusal

- **GIVEN** a project without `compagnon_version`, with no finding
- **WHEN** the user runs `compagnon validate`
- **THEN** the exit code is 0
