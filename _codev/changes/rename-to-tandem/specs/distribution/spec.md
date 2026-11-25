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
- File names follow `tandem-<version>-<target>.<ext>` — for example
  `tandem-0.7.0-x86_64-pc-windows-msvc.zip`.
- A `SHA256SUMS` file listing the SHA-256 of **all** the archives, in
  standard `sha256sum` format.
- The contents of each archive: the binary (`tandem` on macOS/Linux,
  `tandem.exe` on Windows), `README.md`, `README.fr.md`, `LICENSE` and
  `CHANGELOG.md`. The documentation itself is embedded in the binary
  (`tandem docs`).

For a version of the `0.7` series only, the release MUST also contain,
for each of the four targets, a legacy archive named
`codev-<version>-<target>.<ext>` that extracts to the folder
`codev-<version>-<target>/` holding the same binary named `codev`
(`codev.exe` on Windows) and the same documents, listed in `SHA256SUMS`
as well. These archives let a codev 0.6 binary `self-update` to Tandem.
From `0.8.0` on, no legacy archive is published.

The workflow MUST NOT write to the repository.

#### Scenario: A `v0.8.0` tag triggers the release with the four binaries

- **GIVEN** a developer pushes a `v0.8.0` tag on `main`
- **AND** the version in the workspace `Cargo.toml` is `0.8.0`
- **WHEN** the `.github/workflows/release.yml` workflow runs
- **THEN** a GitHub Release named `v0.8.0` is created
- **AND** it contains exactly four archives:
  `tandem-0.8.0-aarch64-apple-darwin.tar.gz`,
  `tandem-0.8.0-x86_64-apple-darwin.tar.gz`,
  `tandem-0.8.0-x86_64-unknown-linux-musl.tar.gz`,
  `tandem-0.8.0-x86_64-pc-windows-msvc.zip`
- **AND** it contains a `SHA256SUMS` file listing the four archives
  with their SHA-256

#### Scenario: A `v0.7.0` tag also publishes the legacy archives

- **GIVEN** a developer pushes a `v0.7.0` tag on `main`
- **WHEN** the release workflow runs
- **THEN** the release contains the four `tandem-0.7.0-*` archives
- **AND** the four archives `codev-0.7.0-<target>.<ext>`, each holding
  `codev-0.7.0-<target>/codev` (`codev.exe` on Windows)
- **AND** `SHA256SUMS` lists all eight archives

#### Scenario: A codev 0.6 binary updates itself across the rename

- **GIVEN** codev 0.6.1 installed at `~/.local/bin/codev` and the
  release `v0.7.0` published
- **WHEN** the user runs `codev self-update --yes`
- **THEN** the legacy archive is downloaded, verified and installed at
  `~/.local/bin/codev`
- **AND** running `codev --version` afterwards prints the Tandem
  version and a line telling the user to rename the executable `tandem`

#### Scenario: The Windows archive contains `tandem.exe`

- **GIVEN** the `tandem-<version>-x86_64-pc-windows-msvc.zip` archive
  of the release
- **WHEN** a user extracts it on Windows
- **THEN** a `tandem.exe` file is present
- **AND** the files `README.md`, `README.fr.md`, `LICENSE` and
  `CHANGELOG.md` are present as well

#### Scenario: A push without a tag publishes nothing

- **GIVEN** a developer pushes a commit on `main` without a tag
- **WHEN** GitHub Actions runs
- **THEN** no GitHub Release is created
- **AND** no binary is published
