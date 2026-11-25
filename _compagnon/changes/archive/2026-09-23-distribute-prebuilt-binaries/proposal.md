# Proposal: distribute codev without requiring Rust

## Why

Today, the only documented installation path is
`cargo install --path crates/codev-cli`. That works for Ludovic and
any developer who already has the Rust toolchain — but **most of the
target JVS users do not have Rust**. They have git, Claude Code,
often Node or Python, never rustc.

The binary itself is **self-contained** once compiled: ~4 MB, no Rust
runtime dependency, a few standard system dependencies. The obstacle
lies purely in the install step.

This batch ships two additional install paths, without breaking the
`cargo install` path, which remains useful to contributors:

- **GitHub Releases** with prebuilt binaries for macOS (arm64 and
  x86_64) and Linux (static x86_64 musl) — the user downloads,
  extracts, puts it on their PATH.
- **`install.sh` script** pipeable via `curl … | sh` — detects
  OS/arch, downloads the right binary, verifies its SHA-256, installs
  into `~/.local/bin/`. A user discovering codev types **a single
  line** to get it.

## What Changes

- **New GitHub Actions workflow** `.github/workflows/release.yml`
  triggered by a `v*.*.*` tag:
  - Three matrix jobs, one per target: `aarch64-apple-darwin`,
    `x86_64-apple-darwin`, `x86_64-unknown-linux-musl`.
  - Each job runs `cargo build --release --bin codev --target <target>`.
  - Packaging: `codev-<version>-<target>.tar.gz` containing the
    binary, `README.md`, `LICENSE`, and the file
    `docs/codev.md` (the doc source).
  - A final job aggregates the SHA-256 checksums into a `SHA256SUMS`
    file and publishes the GitHub Release with the artifacts.
- **New root script `install.sh`**:
  - Detects the OS via `uname` (macOS/Linux) and the arch via
    `uname -m` (arm64/x86_64).
  - Resolves the requested version (`$CODEV_VERSION` env var or the
    latest release via the GitHub API).
  - Downloads the target tar.gz and the `SHA256SUMS`.
  - Verifies the SHA-256 of the downloaded file against the one in
    `SHA256SUMS`.
  - Extracts and copies `codev` into `~/.local/bin/`. Creates the
    folder if missing.
  - Checks that `~/.local/bin/` is in `$PATH` — otherwise, prints
    the line to add to `.zshrc`/`.bashrc`.
- **"Installation" section of `docs/codev.md`** — reworked with
  **three paths**: `curl … | sh` (recommended), manual download
  from GitHub Releases, `cargo install` (contributors).
- **Repository README.md** — the first path mentioned becomes
  `curl -sSL https://raw.githubusercontent.com/mairistem/codev/main/install.sh | sh`.

## Capabilities

### New Capabilities

- `distribution` — describes the distribution contract: which targets
  are supported, which artifact format, which install script,
  which integrity check.

### Modified Capabilities

None.

### Removed Capabilities

None.

## Impact

- **Code**: nothing in the Rust crates. The change touches only:
  - `.github/workflows/release.yml` (new)
  - `install.sh` (new)
  - `docs/codev.md` (Installation section)
  - `README.md` (mention)
- **JSON contract**: nothing. Distribution lives outside the binary.
- **Files written**: the GH Actions workflow writes artifacts to
  GitHub Releases; `install.sh` writes `~/.local/bin/codev` on the
  user's machine.
- **Migration**: none. The `cargo install` path remains documented
  and works.
- **Out of scope**:
  - **Windows** — JVS users are mostly on macOS/Linux. A Windows
    user who shows up later will trigger a dedicated change.
  - **Linux ARM64** — deferrable bonus. Cargo install works in the
    meantime.
  - **Homebrew tap** — heavier to maintain (dedicated repo for the
    tap, Ruby formula, versioning). Deferrable until explicitly
    requested.
  - **Binary auto-update** — a `rustup update`-style pattern. Not
    needed for V1; `codev-<version>` reinstalls easily.
  - **GPG / cosign signing** — SHA-256 verification against a file
    published by the same workflow is enough for V1. A real
    signature would require a key to manage and publish.
  - **Publishing to crates.io** — requires cargo on the user side,
    does not solve the original problem.
