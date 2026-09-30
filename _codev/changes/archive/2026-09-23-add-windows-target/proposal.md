# Proposal: add Windows to the distribution pipeline

## Why

The `v0.1.1` release publishes three binaries for macOS (arm64 +
x86_64) and Linux (musl x86_64). Windows users — still the majority
in the enterprise, including at JVS — have **no install path**
without going through WSL or the Rust toolchain. That gap is
incompatible with the goal "installable in one command, without
Rust".

Adding Windows means:

- **one more target** in the release workflow matrix
  (`x86_64-pc-windows-msvc`);
- a different **archive format** (`.zip` — the Windows convention,
  native extraction via `Expand-Archive`);
- a **PowerShell install script** (`install.ps1`) that follows the
  rustup / Scoop / winget pattern:
  `iwr <url> -useb | iex`.

The 3 existing targets (macOS/Linux) and the `install.sh` script are
**not touched**. This batch is purely additive.

## What Changes

- **Workflow `.github/workflows/release.yml`** — new matrix row:
  ```yaml
  - runner: windows-latest
    target: x86_64-pc-windows-msvc
  ```
  A Windows-specific packaging step produces
  `codev-<version>-x86_64-pc-windows-msvc.zip` (instead of `.tar.gz`)
  containing `codev.exe`, `README.md`, `LICENSE`, `docs/codev.md`.
- **New `install.ps1` script** at the repository root, pipeable:
  ```powershell
  iwr -useb https://raw.githubusercontent.com/mairistem/codev/main/install.ps1 | iex
  ```
  The script:
  - detects the architecture (`$env:PROCESSOR_ARCHITECTURE`);
  - resolves the version (`$env:CODEV_VERSION` or the latest via the
    GitHub API);
  - downloads the target `.zip` and the `SHA256SUMS` via
    `Invoke-WebRequest`;
  - verifies the SHA-256 via `Get-FileHash -Algorithm SHA256`;
  - extracts via `Expand-Archive`;
  - copies `codev.exe` into `$env:LOCALAPPDATA\Programs\codev\`,
    creating the folder if missing;
  - **prints** the instructions for adding the folder to the user
    PATH (does not modify the environment variable — same stance as
    `install.sh`, which does not touch `.zshrc`).
- **Installation section of `docs/codev.md`** — the "recommended
  path" splits in two: `curl … | sh` for macOS/Linux and
  `iwr … | iex` for Windows. A table or two sub-blocks, to be decided
  at render time.
- **README** — add the Windows one-liner alongside.
- **`SHA256SUMS`** — now includes the Windows `.zip` in addition to
  the three `.tar.gz`.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `distribution` — two MODIFIED requirements:
  - "A `v*.*.*` tag publishes a GitHub Release" — the list of
    targets goes from 3 to 4; the archive format is `.tar.gz` for
    macOS/Linux and `.zip` for Windows.
  - "The documentation lists the installation paths" — two
    recommended paths (Unix + Windows).
- `distribution` — one ADDED requirement: the `install.ps1` script
  and its contract (arch detection, SHA-256 check, install into
  `%LOCALAPPDATA%\Programs\codev\`).

### Removed Capabilities

None.

## Impact

- **Code**: nothing in the Rust crates — once again, the change
  touches only the CI infrastructure and the install scripts.
  - `.github/workflows/release.yml`: new matrix row + `.zip`
    packaging branch.
  - `install.ps1`: new root file.
  - `docs/codev.md`: rework of the Installation section.
  - `README.md`: add the Windows one-liner.
- **JSON contract**: nothing.
- **File written**: on the release side, one more asset (the Windows
  `.zip`) + the updated `SHA256SUMS`. On the Windows user side, the
  binary in `%LOCALAPPDATA%\Programs\codev\codev.exe`.
- **Migration**: none. This batch is additive — the macOS/Linux
  binaries and `install.sh` keep existing unchanged.
- **Out of scope**:
  - **Windows ARM64 (`aarch64-pc-windows-msvc`)** — can be deferred
    until there is a concrete request.
  - **Windows package manager** (Scoop, Chocolatey, winget) —
    heavier to maintain; a user can go through the direct install in
    the meantime.
  - **Automatic modification of the user PATH** — the script prints
    the instructions, does not modify `[Environment]`. Symmetry with
    `install.sh`.
  - **Automatically installed PowerShell completions** — the user
    can run `codev completions powershell | Out-String | iex` if they
    want.
