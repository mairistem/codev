# Proposal: Update notice and `compagnon self-update`

## Why

Users stay on old compagnon versions without knowing it, and upgrading means
re-running an installer by hand, then `compagnon update` in every project.
Silent auto-update is not acceptable for a developer tool (security,
reproducibility, team alignment): compagnon must detect a new release, say so,
propose it, and install it in one explicit command — never behind the
user's back.

## What Changes

- `compagnon status` checks for a newer compagnon release, at most once every 24
  hours, through one anonymous GET of GitHub's public "latest release" API
  with a short timeout; the result is cached per user. The check is skipped
  when `CI` is set, when `COMPAGNON_NO_UPDATE_CHECK=1`, and fails silently when
  offline. No other command checks, so machine outputs stay stable.
- `compagnon status --json` gains an additive `update` object
  (`current`, `latest`, `url`, `available`, `snoozed`), absent when the
  latest release is unknown, and a `versionRequirement` object
  (`requirement`, `satisfied`) when the project sets `compagnon_version`. The
  human `compagnon status` prints one line on stderr when an update is
  available and not snoozed.
- New command `compagnon self-update [--version X.Y.Z] [--check] [--snooze]
  [--yes] [--json]`: downloads the release archive for the running
  platform, verifies its SHA-256 against the release's `SHA256SUMS`,
  replaces the running binary atomically, then refreshes the current
  project as `compagnon update` does. It refuses to downgrade without an
  explicit `--version`, and it never replaces a binary installed by cargo,
  a package manager or a source build: it prints the right command instead.
  `--check` reports without installing; `--snooze` silences the notice for
  the latest version for 7 days. Failures carry stable codes
  (`update_check_failed`, `update_download_failed`,
  `update_checksum_mismatch`, `update_not_writable`, …).
- `_compagnon/config.yaml` accepts an optional `compagnon_version` — a semver
  requirement such as `">=0.5"` — that every teammate's compagnon must satisfy;
  an invalid requirement fails with `invalid_compagnon_version`.
- `compagnon doctor` gains a `compagnon_version` check: `missing` when the running
  compagnon does not satisfy the project's `compagnon_version`, `warning` when the
  cached update check knows a newer release; doctor stays offline.
- `/compagnon-propose`, `/compagnon-apply`, `/compagnon-update`, `/compagnon-sync` and
  `/compagnon-archive` read the `update` and `versionRequirement` objects at the
  start of the action, tell the user in one line, and ask "Update now /
  Later"; "Now" runs `compagnon self-update --yes` and resumes, "Later" snoozes
  and resumes. Never in the middle of an action.
- `/compagnon-configure` preserves `compagnon_version`.
- compagnon's own `_compagnon/config.yaml` requires `compagnon_version: ">=0.5"`.

## Capabilities

### New Capabilities

- `self-update` — the update check and its cache, the `update` and
  `versionRequirement` objects of `compagnon status`, the human notice,
  `compagnon self-update` and its flags, installation method detection, the
  `compagnon_version` requirement.

### Modified Capabilities

- `doctor` — the `compagnon_version` check; the `compagnon status` hint
  requirement now allows the additive `update` objects in the JSON.
- `skills` — the update notice and its "Update now / Later" question.
- `configure` — `compagnon_version` is a preserved field.
- `decisions` — the "no everyday command contacts the network" rule is
  scoped to inherited sources, the update check being the one exception.

### Removed Capabilities

None.

## Impact

- `compagnon-core`: version and requirement parsing, update check policy,
  cache model, snooze, release parsing, asset naming, checksum lookup,
  installation method detection, self-update decision, the doctor check.
- `compagnon-engine`: an `Http` port (implemented with `curl` through the
  `ProcessRunner` port), an `Executable` port for the running binary, the
  update cache under the compagnon cache directory, the self-update
  orchestration, `compagnon_version` in the project configuration.
- `compagnon-agents`: a shared update-notice paragraph substituted into five
  workflows; the installed skills are regenerated.
- `compagnon-cli`: the `self-update` command, `update` and `versionRequirement`
  in `StatusV1`, a new `SelfUpdateV1` document, doctor facts, Windows
  cleanup of the previous binary. New dependency: `semver` (pure).
- `_compagnon/config.yaml`, `CHANGELOG.md`.
- Documentation in English and French: installation (upgrading), CLI
  reference, configuration, JSON output, skills, FAQ (privacy, CI,
  disabling the check), repository integration (doctor).

## Release note

- Added: `compagnon self-update` downloads the latest release for your
  platform, verifies its SHA-256, replaces the binary and refreshes the
  current project; `--check` only reports, `--snooze` silences the notice
  for 7 days, and a cargo or package-manager installation is left alone
  with the right command printed instead.
- Added: `compagnon status` tells you when a newer compagnon is available — at
  most one anonymous request to GitHub per day, never in CI or with
  `COMPAGNON_NO_UPDATE_CHECK=1` — and `compagnon status --json` exposes it in an
  `update` object; the skills offer to update at the start of an action.
- Added: `compagnon_version` in `_compagnon/config.yaml` sets the compagnon versions a
  team accepts; `compagnon doctor` reports a `compagnon_version` check when the
  running compagnon does not satisfy it, or when a newer release is known.
