# Distribution Specification

## Purpose

Make Compagnon installable on the machines of users who **do not have
the Rust toolchain** — through prebuilt binaries published on GitHub
Releases and an `install.sh` script that fetches them in a single
command.

## Requirements

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

### Requirement: The `install.sh` script installs Compagnon in one command, without Rust

An `install.sh` script at the root of the repository SHALL let a
user install Compagnon via:

```
curl -sSL https://raw.githubusercontent.com/mairistem/compagnon/main/install.sh | sh
```

The script MUST:

- detect the OS (`uname -s`: `Darwin` or `Linux`) and the architecture
  (`uname -m`: `arm64`/`aarch64` or `x86_64`);
- resolve the version to install: if `$COMPAGNON_VERSION` is set, use it;
  otherwise query the GitHub Releases API for the latest one;
- download the matching `compagnon-<version>-<target>.tar.gz` archive and
  the `SHA256SUMS` file;
- **verify the SHA-256** of the archive against `SHA256SUMS` and
  refuse to install if verification fails;
- extract the binary into a temporary directory, then copy it to
  `~/.local/bin/compagnon` with `755` permissions;
- create `~/.local/bin` if it does not exist;
- display, on success, the appropriate message depending on whether
  `~/.local/bin` is already in `$PATH` or not (adding it to `.zshrc`/
  `.bashrc` is recommended if missing).

The script MUST refuse (non-zero exit) if:

- the OS or the architecture matches no supported target;
- the verified SHA-256 does not match;
- `curl` or `tar` is not available.

#### Scenario: Installation on macOS Apple Silicon, PATH ready

- **GIVEN** a user on macOS Apple Silicon whose `~/.local/bin`
  is already in `$PATH`
- **WHEN** they run
  `curl -sSL https://raw.githubusercontent.com/mairistem/compagnon/main/install.sh | sh`
- **THEN** the script detects `aarch64-apple-darwin`
- **AND** downloads, verifies the SHA-256, and extracts the binary
- **AND** copies `compagnon` into `~/.local/bin/`
- **AND** the final message invites the user to run `compagnon --version`
  to check

#### Scenario: Incomplete PATH — explanatory message

- **GIVEN** the same user but with `~/.local/bin` missing from `$PATH`
- **WHEN** the installation finishes
- **THEN** the script displays the exact line to add to `~/.zshrc`
  (for example `export PATH="$HOME/.local/bin:$PATH"`)
- **AND** the message recalls that a new shell must be opened or
  `source ~/.zshrc` run

#### Scenario: Unsupported platform rejected

- **GIVEN** a user on Windows
- **WHEN** they run the script through WSL with `uname` returning an
  unsupported OS or an unsupported architecture
- **THEN** the script exits with a non-zero exit code
- **AND** the error message names the detected platform and points
  to `cargo install --path` as a fallback

#### Scenario: Corrupted SHA-256 → refusal

- **GIVEN** an environment where the downloaded file has been tampered
  with (manual test or fixture)
- **WHEN** the script verifies the SHA-256
- **THEN** the installation is refused
- **AND** no file is copied into `~/.local/bin/`

### Requirement: The documentation cites the four installation paths

The Installation chapter of each language (`docs/en/src/installation.md`,
`docs/fr/src/installation.md`) MUST cite, in this order:

1. **Recommended Unix path** — `curl -sSL … | sh` for macOS/Linux.
2. **Recommended Windows path** — `iwr -useb … | iex` for Windows
   in PowerShell.
3. **Manual path** — download from GitHub Releases + verification with
   `sha256sum -c` (Unix) or `Get-FileHash` (Windows).
4. **Contributor path** — `cargo install --path crates/compagnon-cli`
   from a clone of the repository.

The repository README MUST mention at least the first **and** the
second path (with the `curl … | sh` and `iwr … | iex` one-liners).

#### Scenario: The Installation chapter lists the four paths in order

- **GIVEN** a reader who opens the Installation chapter, in English or
  in French
- **WHEN** they go through the subsections in order
- **THEN** they successively encounter the Unix path (`curl | sh`),
  the Windows path (`iwr | iex`), the manual path (download from
  GitHub Releases with SHA-256 verification), and the contributor
  path (`cargo install --path`)

#### Scenario: The README points to at least the two "no Rust" paths in its Getting Started section

- **GIVEN** a reader who opens `README.md` at the root of the repository
- **WHEN** they go through the "Getting Started" section
- **THEN** they see the `curl -sSL … | sh` example for macOS/Linux
- **AND** they see the `iwr -useb … | iex` example for Windows in
  PowerShell

### Requirement: The `install.ps1` script installs Compagnon in one command on Windows

An `install.ps1` script at the root of the repository SHALL let a
Windows user install Compagnon via:

```powershell
iwr -useb https://raw.githubusercontent.com/mairistem/compagnon/main/install.ps1 | iex
```

The script MUST:

- detect the architecture via `$env:PROCESSOR_ARCHITECTURE`
  (`AMD64` → target `x86_64-pc-windows-msvc`);
- resolve the version: if `$env:COMPAGNON_VERSION` is set, use it;
  otherwise query the GitHub Releases API for the latest one (via
  `Invoke-RestMethod`);
- download the
  `compagnon-<version>-x86_64-pc-windows-msvc.zip` archive and the
  `SHA256SUMS` file via `Invoke-WebRequest`;
- **verify the SHA-256** via `Get-FileHash -Algorithm SHA256` and
  refuse to install on mismatch;
- extract via `Expand-Archive`;
- copy `compagnon.exe` to `$env:LOCALAPPDATA\Programs\compagnon\compagnon.exe`
  (creating the directory if missing);
- **display** the instructions for adding
  `$env:LOCALAPPDATA\Programs\compagnon\` to the user PATH (via
  `setx PATH` or the System settings), **without** modifying the PATH
  automatically.

The script MUST refuse (non-zero exit code) if:

- the architecture is not supported (for example `ARM64` in V1);
- the verified SHA-256 does not match;
- a required PowerShell command (`Invoke-WebRequest`,
  `Expand-Archive`, `Get-FileHash`) is not available.

#### Scenario: Installation on Windows x86_64, incomplete PATH

- **GIVEN** a Windows user on PowerShell 5.1+ (or Core 7+)
- **AND** `$env:LOCALAPPDATA\Programs\compagnon\` is not in their
  PATH
- **WHEN** they run
  `iwr -useb https://raw.githubusercontent.com/mairistem/compagnon/main/install.ps1 | iex`
- **THEN** the script detects `x86_64-pc-windows-msvc`
- **AND** downloads, verifies the SHA-256, and extracts `compagnon.exe`
- **AND** copies `compagnon.exe` into
  `$env:LOCALAPPDATA\Programs\compagnon\`
- **AND** displays the exact command to type to add the directory
  to the user PATH (for example `setx PATH "$env:PATH;$env:LOCALAPPDATA\Programs\compagnon"`)
- **AND** recalls that a new shell must be opened to reload the PATH

#### Scenario: Unsupported architecture rejected

- **GIVEN** a Windows user on ARM64
- **WHEN** they run the script
- **THEN** the script stops with a non-zero exit code
- **AND** the error message names the detected architecture and
  points to `cargo install --path` as a fallback

#### Scenario: Corrupted SHA-256 → refusal

- **GIVEN** an environment where the downloaded file has been tampered
  with
- **WHEN** the script compares `Get-FileHash` with `SHA256SUMS`
- **THEN** the installation is refused
- **AND** no file is copied into `$env:LOCALAPPDATA`

#### Scenario: Installation section up to date with the two recommended paths

- **GIVEN** the documentation generated by `compagnon docs`
- **WHEN** the "Installation" section is looked up
- **THEN** the four paths are present in the expected order
- **AND** the Windows path cites exactly `iwr -useb https://…/install.ps1 | iex`
- **AND** the Unix path cites exactly `curl -sSL https://…/install.sh | sh`

### Requirement: The GitHub Release body is generated by `compagnon changelog`

The release workflow SHALL write the body of the GitHub Release with
`compagnon changelog --until <tag> --format markdown`, run by a binary built
from the tagged source, so the body lists the release notes of the changes
archived since the previous `v*` tag. GitHub's generated notes (pull
requests and contributors) MUST follow it, below. The workflow MUST still
not write to the repository.

#### Scenario: The release body lists the archived changes

- **GIVEN** a `v0.5.0` tag whose source archives changes with release notes
  since `v0.4.0`
- **WHEN** the `.github/workflows/release.yml` workflow runs
- **THEN** the body of the `v0.5.0` GitHub Release starts with their items
  under `### <Category>` headings
- **AND** GitHub's generated notes follow below

#### Scenario: The workflow runs the binary it built

- **WHEN** the release workflow publishes the release
- **THEN** the binary that generates the body comes from the workflow's own
  Linux build of the tag, not from a previously published release

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
