## Purpose

Make codev installable on the machines of users who **do not have
the Rust toolchain** — through prebuilt binaries published on
GitHub Releases and an `install.sh` script that fetches them in a
single command.

## ADDED Requirements

### Requirement: A `v*.*.*` tag publishes a GitHub Release with the target binaries

A GitHub Actions workflow SHALL be triggered by a git tag in the
format `v<major>.<minor>.<patch>` (for example `v0.2.0`). The
workflow MUST produce a GitHub Release containing:

- One prebuilt binary per **supported target** (V1: macOS arm64,
  macOS x86_64, static Linux x86_64 musl), packaged in a
  `.tar.gz` archive named `codev-<version>-<target>.tar.gz`.
- A `SHA256SUMS` file listing the SHA-256 of each archive,
  in the standard `sha256sum` format (`<hex>  <name>`).
- The contents of each archive: the `codev` binary, `README.md`,
  `LICENSE`, and a copy of the doc's markdown source
  (`docs/codev.md`).

The workflow MUST NOT write to the repository (no commit,
no push) — it only produces release artifacts.

#### Scenario: A `v0.2.0` tag triggers the release

- **GIVEN** a developer pushes a `v0.2.0` tag on `main`
- **AND** the version in the workspace `Cargo.toml` is `0.2.0`
- **WHEN** the `.github/workflows/release.yml` workflow runs
- **THEN** a GitHub Release named `v0.2.0` is created
- **AND** it contains three archives:
  `codev-0.2.0-aarch64-apple-darwin.tar.gz`,
  `codev-0.2.0-x86_64-apple-darwin.tar.gz`,
  `codev-0.2.0-x86_64-unknown-linux-musl.tar.gz`
- **AND** it contains a `SHA256SUMS` file listing the three
  archives with their SHA-256

#### Scenario: A push without a tag publishes nothing

- **GIVEN** a developer pushes a commit on `main` without a tag
- **WHEN** GitHub Actions runs
- **THEN** no GitHub Release is created
- **AND** no binary is published

### Requirement: The `install.sh` script installs codev in one command, without Rust

An `install.sh` script at the repository root SHALL allow a
user to install codev via:

```
curl -sSL https://raw.githubusercontent.com/mairistem/codev/main/install.sh | sh
```

The script MUST:

- detect the OS (`uname -s`: `Darwin` or `Linux`) and the architecture
  (`uname -m`: `arm64`/`aarch64` or `x86_64`);
- resolve the version to install: if `$CODEV_VERSION` is set,
  use it; otherwise query the GitHub Releases API for the
  latest;
- download the matching `codev-<version>-<target>.tar.gz` archive
  and the `SHA256SUMS` file;
- **verify the SHA-256** of the archive against `SHA256SUMS` and
  refuse to install if the verification fails;
- extract the binary into a temporary folder, then copy it
  to `~/.local/bin/codev` with `755` permissions;
- if `~/.local/bin` does not exist, create it;
- display, on success, the appropriate message depending on whether
  `~/.local/bin` is already in `$PATH` or not (adding it to `.zshrc`/
  `.bashrc` recommended if missing).

The script MUST refuse (non-zero exit) if:

- the OS or arch matches no supported target;
- the verified SHA-256 does not match;
- `curl` or `tar` are not available.

#### Scenario: macOS Apple Silicon install, PATH ready

- **GIVEN** a user on macOS Apple Silicon whose `~/.local/bin`
  is already in `$PATH`
- **WHEN** they run
  `curl -sSL https://raw.githubusercontent.com/mairistem/codev/main/install.sh | sh`
- **THEN** the script detects `aarch64-apple-darwin`
- **AND** downloads, verifies the SHA-256, extracts the binary
- **AND** copies `codev` into `~/.local/bin/`
- **AND** the final message invites the user to type `codev --version`
  to check

#### Scenario: Incomplete PATH — instructive message

- **GIVEN** the same user but with `~/.local/bin` missing from `$PATH`
- **WHEN** the installation finishes
- **THEN** the script displays the exact line to add to `~/.zshrc`
  (for example `export PATH="$HOME/.local/bin:$PATH"`)
- **AND** the message reminds them to open a new shell or
  run `source ~/.zshrc`

#### Scenario: Unsupported platform refused

- **GIVEN** a user on Windows
- **WHEN** they run the script via WSL with `uname` returning an
  unsupported OS or an unsupported arch
- **THEN** the script exits with a non-zero exit code
- **AND** the error message names the detected platform and points
  to the `cargo install --path` path as a fallback

#### Scenario: Corrupted SHA-256 → refusal

- **GIVEN** an environment where the downloaded file would be tampered with
  (manual test or fixture)
- **WHEN** the script verifies the SHA-256
- **THEN** the installation is refused
- **AND** no file is copied into `~/.local/bin/`

### Requirement: The documentation lists the three installation paths

The Installation section of the `docs/codev.md` file MUST list, in
this order:

1. **Recommended for users** — the command
   `curl -sSL … | sh` (a single line).
2. **Alternative** — manual download from GitHub Releases,
   with instructions for manually verifying the SHA-256.
3. **For contributors** — `cargo install --path
   crates/codev-cli`, as today.

The repository README MUST mention at least the first path, with a
link to `docs/codev.md` for the details.

#### Scenario: Installation section up to date

- **GIVEN** the documentation generated by `codev docs`
- **WHEN** one looks for the "Installation" section
- **THEN** the three paths are present in the expected order
- **AND** none depends on Rust except the third
