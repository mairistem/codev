# Distribution Specification

## Purpose

Make codev installable on the machines of users who **do not have
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
- File names follow `codev-<version>-<target>.<ext>` — for example
  `codev-0.2.0-x86_64-pc-windows-msvc.zip`.
- A `SHA256SUMS` file listing the SHA-256 of **all** the archives
  (all four), in standard `sha256sum` format.
- The contents of each archive: the binary (`codev` on macOS/Linux,
  `codev.exe` on Windows), `README.md`, `README.fr.md`, `LICENSE` and
  `CHANGELOG.md`. The documentation itself is embedded in the binary
  (`codev docs`).

The workflow MUST NOT write to the repository.

#### Scenario: A `v0.2.0` tag triggers the release with the four binaries

- **GIVEN** a developer pushes a `v0.2.0` tag on `main`
- **AND** the version in the workspace `Cargo.toml` is `0.2.0`
- **WHEN** the `.github/workflows/release.yml` workflow runs
- **THEN** a GitHub Release named `v0.2.0` is created
- **AND** it contains exactly four archives:
  `codev-0.2.0-aarch64-apple-darwin.tar.gz`,
  `codev-0.2.0-x86_64-apple-darwin.tar.gz`,
  `codev-0.2.0-x86_64-unknown-linux-musl.tar.gz`,
  `codev-0.2.0-x86_64-pc-windows-msvc.zip`
- **AND** it contains a `SHA256SUMS` file listing the four archives
  with their SHA-256

#### Scenario: The Windows archive contains `codev.exe`

- **GIVEN** the `codev-<version>-x86_64-pc-windows-msvc.zip` archive
  of the release
- **WHEN** a user extracts it on Windows
- **THEN** a `codev.exe` file is present
- **AND** the files `README.md`, `README.fr.md`, `LICENSE` and
  `CHANGELOG.md` are present as well

#### Scenario: A push without a tag publishes nothing

- **GIVEN** a developer pushes a commit on `main` without a tag
- **WHEN** GitHub Actions runs
- **THEN** no GitHub Release is created
- **AND** no binary is published

### Requirement: The `install.sh` script installs codev in one command, without Rust

An `install.sh` script at the root of the repository SHALL let a
user install codev via:

```
curl -sSL https://raw.githubusercontent.com/mairistem/codev/main/install.sh | sh
```

The script MUST:

- detect the OS (`uname -s`: `Darwin` or `Linux`) and the architecture
  (`uname -m`: `arm64`/`aarch64` or `x86_64`);
- resolve the version to install: if `$CODEV_VERSION` is set, use it;
  otherwise query the GitHub Releases API for the latest one;
- download the matching `codev-<version>-<target>.tar.gz` archive and
  the `SHA256SUMS` file;
- **verify the SHA-256** of the archive against `SHA256SUMS` and
  refuse to install if verification fails;
- extract the binary into a temporary directory, then copy it to
  `~/.local/bin/codev` with `755` permissions;
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
  `curl -sSL https://raw.githubusercontent.com/mairistem/codev/main/install.sh | sh`
- **THEN** the script detects `aarch64-apple-darwin`
- **AND** downloads, verifies the SHA-256, and extracts the binary
- **AND** copies `codev` into `~/.local/bin/`
- **AND** the final message invites the user to run `codev --version`
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
4. **Contributor path** — `cargo install --path crates/codev-cli`
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

### Requirement: The `install.ps1` script installs codev in one command on Windows

An `install.ps1` script at the root of the repository SHALL let a
Windows user install codev via:

```powershell
iwr -useb https://raw.githubusercontent.com/mairistem/codev/main/install.ps1 | iex
```

The script MUST:

- detect the architecture via `$env:PROCESSOR_ARCHITECTURE`
  (`AMD64` → target `x86_64-pc-windows-msvc`);
- resolve the version: if `$env:CODEV_VERSION` is set, use it;
  otherwise query the GitHub Releases API for the latest one (via
  `Invoke-RestMethod`);
- download the
  `codev-<version>-x86_64-pc-windows-msvc.zip` archive and the
  `SHA256SUMS` file via `Invoke-WebRequest`;
- **verify the SHA-256** via `Get-FileHash -Algorithm SHA256` and
  refuse to install on mismatch;
- extract via `Expand-Archive`;
- copy `codev.exe` to `$env:LOCALAPPDATA\Programs\codev\codev.exe`
  (creating the directory if missing);
- **display** the instructions for adding
  `$env:LOCALAPPDATA\Programs\codev\` to the user PATH (via
  `setx PATH` or the System settings), **without** modifying the PATH
  automatically.

The script MUST refuse (non-zero exit code) if:

- the architecture is not supported (for example `ARM64` in V1);
- the verified SHA-256 does not match;
- a required PowerShell command (`Invoke-WebRequest`,
  `Expand-Archive`, `Get-FileHash`) is not available.

#### Scenario: Installation on Windows x86_64, incomplete PATH

- **GIVEN** a Windows user on PowerShell 5.1+ (or Core 7+)
- **AND** `$env:LOCALAPPDATA\Programs\codev\` is not in their
  PATH
- **WHEN** they run
  `iwr -useb https://raw.githubusercontent.com/mairistem/codev/main/install.ps1 | iex`
- **THEN** the script detects `x86_64-pc-windows-msvc`
- **AND** downloads, verifies the SHA-256, and extracts `codev.exe`
- **AND** copies `codev.exe` into
  `$env:LOCALAPPDATA\Programs\codev\`
- **AND** displays the exact command to type to add the directory
  to the user PATH (for example `setx PATH "$env:PATH;$env:LOCALAPPDATA\Programs\codev"`)
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

- **GIVEN** the documentation generated by `codev docs`
- **WHEN** the "Installation" section is looked up
- **THEN** the four paths are present in the expected order
- **AND** the Windows path cites exactly `iwr -useb https://…/install.ps1 | iex`
- **AND** the Unix path cites exactly `curl -sSL https://…/install.sh | sh`
