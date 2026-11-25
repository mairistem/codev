## RENAMED Requirements

- FROM: `### Requirement: The documentation cites the three installation paths`
- TO: `### Requirement: The documentation cites the four installation paths`

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
