# Proposal: Update notice and `codev self-update`

## Why

Users stay on old codev versions without knowing it, and upgrading means
re-running an installer by hand, then `codev update` in every project.
Silent auto-update is not acceptable for a developer tool (security,
reproducibility, team alignment): codev must detect a new release, say so,
propose it, and install it in one explicit command — never behind the
user's back.

## What Changes

- `codev status` checks for a newer codev release, at most once every 24
  hours, through one anonymous GET of GitHub's public "latest release" API
  with a short timeout; the result is cached per user. The check is skipped
  when `CI` is set, when `CODEV_NO_UPDATE_CHECK=1`, and fails silently when
  offline. No other command checks, so machine outputs stay stable.
- `codev status --json` gains an additive `update` object
  (`current`, `latest`, `url`, `available`, `snoozed`), absent when the
  latest release is unknown, and a `versionRequirement` object
  (`requirement`, `satisfied`) when the project sets `codev_version`. The
  human `codev status` prints one line on stderr when an update is
  available and not snoozed.
- New command `codev self-update [--version X.Y.Z] [--check] [--snooze]
  [--yes] [--json]`: downloads the release archive for the running
  platform, verifies its SHA-256 against the release's `SHA256SUMS`,
  replaces the running binary atomically, then refreshes the current
  project as `codev update` does. It refuses to downgrade without an
  explicit `--version`, and it never replaces a binary installed by cargo,
  a package manager or a source build: it prints the right command instead.
  `--check` reports without installing; `--snooze` silences the notice for
  the latest version for 7 days. Failures carry stable codes
  (`update_check_failed`, `update_download_failed`,
  `update_checksum_mismatch`, `update_not_writable`, …).
- `_codev/config.yaml` accepts an optional `codev_version` — a semver
  requirement such as `">=0.5"` — that every teammate's codev must satisfy;
  an invalid requirement fails with `invalid_codev_version`.
- `codev doctor` gains a `codev_version` check: `missing` when the running
  codev does not satisfy the project's `codev_version`, `warning` when the
  cached update check knows a newer release; doctor stays offline.
- `/codev-propose`, `/codev-apply`, `/codev-update`, `/codev-sync` and
  `/codev-archive` read the `update` and `versionRequirement` objects at the
  start of the action, tell the user in one line, and ask "Update now /
  Later"; "Now" runs `codev self-update --yes` and resumes, "Later" snoozes
  and resumes. Never in the middle of an action.
- `/codev-configure` preserves `codev_version`.
- codev's own `_codev/config.yaml` requires `codev_version: ">=0.5"`.

## Capabilities

### New Capabilities

- `self-update` — the update check and its cache, the `update` and
  `versionRequirement` objects of `codev status`, the human notice,
  `codev self-update` and its flags, installation method detection, the
  `codev_version` requirement.

### Modified Capabilities

- `doctor` — the `codev_version` check; the `codev status` hint
  requirement now allows the additive `update` objects in the JSON.
- `skills` — the update notice and its "Update now / Later" question.
- `configure` — `codev_version` is a preserved field.
- `decisions` — the "no everyday command contacts the network" rule is
  scoped to inherited sources, the update check being the one exception.

### Removed Capabilities

None.

## Impact

- `codev-core`: version and requirement parsing, update check policy,
  cache model, snooze, release parsing, asset naming, checksum lookup,
  installation method detection, self-update decision, the doctor check.
- `codev-engine`: an `Http` port (implemented with `curl` through the
  `ProcessRunner` port), an `Executable` port for the running binary, the
  update cache under the codev cache directory, the self-update
  orchestration, `codev_version` in the project configuration.
- `codev-agents`: a shared update-notice paragraph substituted into five
  workflows; the installed skills are regenerated.
- `codev-cli`: the `self-update` command, `update` and `versionRequirement`
  in `StatusV1`, a new `SelfUpdateV1` document, doctor facts, Windows
  cleanup of the previous binary. New dependency: `semver` (pure).
- `_codev/config.yaml`, `CHANGELOG.md`.
- Documentation in English and French: installation (upgrading), CLI
  reference, configuration, JSON output, skills, FAQ (privacy, CI,
  disabling the check), repository integration (doctor).

## Release note

- Added: `codev self-update` downloads the latest release for your
  platform, verifies its SHA-256, replaces the binary and refreshes the
  current project; `--check` only reports, `--snooze` silences the notice
  for 7 days, and a cargo or package-manager installation is left alone
  with the right command printed instead.
- Added: `codev status` tells you when a newer codev is available — at
  most one anonymous request to GitHub per day, never in CI or with
  `CODEV_NO_UPDATE_CHECK=1` — and `codev status --json` exposes it in an
  `update` object; the skills offer to update at the start of an action.
- Added: `codev_version` in `_codev/config.yaml` sets the codev versions a
  team accepts; `codev doctor` reports a `codev_version` check when the
  running codev does not satisfy it, or when a newer release is known.
