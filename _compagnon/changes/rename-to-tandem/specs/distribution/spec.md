## MODIFIED Requirements

### Requirement: A `v*.*.*` tag publishes a GitHub Release with the target binaries

A GitHub Actions workflow SHALL be triggered by a git tag in the
format `v<major>.<minor>.<patch>` (for example `v0.2.0`). The
workflow MUST produce a GitHub Release containing:

- One prebuilt binary per **supported target**: macOS arm64, macOS
  x86_64, statically linked Linux x86_64 musl, **Windows x86_64 MSVC**.
- One archive format **per OS family**: `.tar.gz` for macOS and
  Linux (Unix convention, `tar` is universal); `.zip` for Windows
  (Windows convention, native `Expand-Archive` since PowerShell 5.1).
- File names follow `compagnon-<version>-<target>.<ext>` — for example
  `compagnon-0.7.0-x86_64-pc-windows-msvc.zip`.
- A `SHA256SUMS` file listing the SHA-256 of **all** the archives, in
  standard `sha256sum` format.
- The contents of each archive: the binary (`compagnon` on macOS/Linux,
  `compagnon.exe` on Windows), `README.md`, `README.fr.md`, `LICENSE` and
  `CHANGELOG.md`. The documentation itself is embedded in the binary
  (`compagnon docs`).

For a version of the `0.7` series only, the release MUST also contain,
for each of the four targets, a legacy archive named
`codev-<version>-<target>.<ext>` that extracts to the folder
`codev-<version>-<target>/` holding the same binary named `codev`
(`codev.exe` on Windows) and the same documents, listed in `SHA256SUMS`
as well. These archives let a codev 0.6 binary `self-update` to Compagnon.
From `0.8.0` on, no legacy archive is published.

The workflow MUST NOT write to the repository.

#### Scenario: A `v0.8.0` tag triggers the release with the four binaries

- **GIVEN** a developer pushes a `v0.8.0` tag on `main`
- **AND** the version in the workspace `Cargo.toml` is `0.8.0`
- **WHEN** the `.github/workflows/release.yml` workflow runs
- **THEN** a GitHub Release named `v0.8.0` is created
- **AND** it contains exactly four archives:
  `compagnon-0.8.0-aarch64-apple-darwin.tar.gz`,
  `compagnon-0.8.0-x86_64-apple-darwin.tar.gz`,
  `compagnon-0.8.0-x86_64-unknown-linux-musl.tar.gz`,
  `compagnon-0.8.0-x86_64-pc-windows-msvc.zip`
- **AND** it contains a `SHA256SUMS` file listing the four archives
  with their SHA-256

#### Scenario: A `v0.7.0` tag also publishes the legacy archives

- **GIVEN** a developer pushes a `v0.7.0` tag on `main`
- **WHEN** the release workflow runs
- **THEN** the release contains the four `compagnon-0.7.0-*` archives
- **AND** the four archives `codev-0.7.0-<target>.<ext>`, each holding
  `codev-0.7.0-<target>/codev` (`codev.exe` on Windows)
- **AND** `SHA256SUMS` lists all eight archives

#### Scenario: A codev 0.6 binary updates itself across the rename

- **GIVEN** codev 0.6.1 installed at `~/.local/bin/codev` and the
  release `v0.7.0` published
- **WHEN** the user runs `codev self-update --yes`
- **THEN** the legacy archive is downloaded, verified and installed at
  `~/.local/bin/codev`
- **AND** running `codev --version` afterwards prints the Compagnon
  version and a line telling the user to rename the executable `compagnon`

#### Scenario: The Windows archive contains `compagnon.exe`

- **GIVEN** the `compagnon-<version>-x86_64-pc-windows-msvc.zip` archive
  of the release
- **WHEN** a user extracts it on Windows
- **THEN** a `compagnon.exe` file is present
- **AND** the files `README.md`, `README.fr.md`, `LICENSE` and
  `CHANGELOG.md` are present as well

#### Scenario: A push without a tag publishes nothing

- **GIVEN** a developer pushes a commit on `main` without a tag
- **WHEN** GitHub Actions runs
- **THEN** no GitHub Release is created
- **AND** no binary is published

## ADDED Requirements

### Requirement: Installers and self-update still reach the releases published as codev

`install.sh` and `install.ps1` SHALL read the version to install from
`COMPAGNON_VERSION`, and, when it is unset, from `CODEV_VERSION`. When the
version to install is below `0.7.0`, they MUST download
`codev-<version>-<target>.<ext>` and install the binary under the name
`codev`, exactly as the codev installers did; from `0.7.0` on they
download `compagnon-*` and install `compagnon`. When only `CODEV_VERSION` is
set, they SHALL also install the binary under the name `codev`, so that
a CI job written for codev keeps finding the command it calls.
`compagnon self-update --version X.Y.Z` with `X.Y.Z` below `0.7.0` MUST
download the `codev-*` archive of that release, so that a user can go
back to codev 0.6.

#### Scenario: A CI job pinned to codev 0.6 keeps working

- **GIVEN** a workflow running
  `curl -sSL <installer URL> | CODEV_VERSION=0.6.1 sh` then
  `codev validate --strict`
- **WHEN** the job runs after this change is merged
- **THEN** `codev-0.6.1-<target>.tar.gz` is downloaded and verified
- **AND** `~/.local/bin/codev` is installed and `codev validate` runs

#### Scenario: A codev pin to the bridge release

- **GIVEN** `CODEV_VERSION=0.7.0` and no `COMPAGNON_VERSION`
- **WHEN** the installer runs
- **THEN** the Compagnon 0.7.0 binary is installed as `~/.local/bin/codev`

#### Scenario: Going back to codev 0.6

- **GIVEN** Compagnon 0.7.0 installed by its installer
- **WHEN** the user runs `compagnon self-update --version 0.6.1 --yes`
- **THEN** `codev-0.6.1-<target>.<ext>` is downloaded and verified
