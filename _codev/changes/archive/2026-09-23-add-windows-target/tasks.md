# Tasks

## 1. Release workflow — add Windows

- [x] 1.1 Edit `.github/workflows/release.yml`. In
      `strategy.matrix.include`, add:
      ```yaml
      - runner: windows-latest
        target: x86_64-pc-windows-msvc
      ```
- [x] 1.2 Adapt the "Package tarball" step to branch on the OS:
      - On macOS/Linux, keep the current behavior: `tar czf …
        .tar.gz`.
      - On Windows, produce a `.zip` instead. Two options:
        - **A. A dedicated step `if: runner.os == 'Windows'`** using
          PowerShell `Compress-Archive`.
        - **B. A single step that branches (`if`) on `runner.os`**,
          with `run:` in `pwsh` or `bash` depending on the case.
      - Option **A** preferred for readability.
- [x] 1.3 Adapt the "Upload artifact" step so that the pattern
      matches `.zip` or `.tar.gz` depending on the job. Today:
      `path: codev-${{ steps.version.outputs.version }}-${{
      matrix.target }}.tar.gz`. Make it parametric via
      `ext: tar.gz` / `zip` in the matrix, or two conditional
      uploads.
- [x] 1.4 Adapt the "Compute SHA-256 sums" step of the `release` job:
      `shasum -a 256 *.tar.gz > SHA256SUMS` →
      `shasum -a 256 *.tar.gz *.zip > SHA256SUMS`.
- [x] 1.5 The binary name on Windows is `codev.exe`. The packaging
      step must copy `target/<target>/release/codev.exe` (with the
      `.exe`) — otherwise the file will not exist.

## 2. New `install.ps1` script

- [x] 2.1 Create `install.ps1` at the repository root. UTF-8
      encoding (no BOM), CRLF or LF line endings (PowerShell
      accepts both, LF more git-portable).
- [x] 2.2 Script sections, in order:
      - Header with description, usage
        (`iwr -useb … | iex`, `CODEV_VERSION` env var).
      - `$ErrorActionPreference = 'Stop'` to stop hard on any
        exception.
      - Architecture detection: `$env:PROCESSOR_ARCHITECTURE` →
        `AMD64` = `x86_64-pc-windows-msvc`. Any other value = refusal.
      - Version resolution: `$env:CODEV_VERSION` or GitHub API
        (`Invoke-RestMethod https://api.github.com/repos/mairistem/codev/releases/latest`).
      - Download of the archive and of `SHA256SUMS` via
        `Invoke-WebRequest`.
      - SHA-256 verification: `(Get-FileHash <archive> -Algorithm
        SHA256).Hash` compared (case-insensitively) to the sha listed
        in `SHA256SUMS`.
      - Extraction via `Expand-Archive`.
      - Copy of `codev.exe` into
        `$env:LOCALAPPDATA\Programs\codev\`, with `New-Item -ItemType
        Directory -Force` if missing.
      - PATH check: `$env:PATH -split ';' -contains
        '$env:LOCALAPPDATA\Programs\codev'`.
      - Final message: instructions for adding to the PATH via
        `setx PATH "$env:PATH;$env:LOCALAPPDATA\Programs\codev"` (to
        be run in a separate shell), or System UI → Environment
        Variables.
- [x] 2.3 No `Set-ExecutionPolicy` — a script piped from stdin runs
      without a policy.

## 3. Documentation

- [x] 3.1 Rework the "Installation" section of `docs/codev.md`
      with **4 paths** in order:
      1. **Recommended Unix** — `curl … | sh` (already there).
      2. **Recommended Windows** — `iwr -useb … | iex` (new).
      3. **Manual path** — download + SHA-256 check
        (Unix AND Windows parts).
      4. **Contributor path** — `cargo install` (already there).
- [x] 3.2 Complete the manual path for Windows:
      `Expand-Archive`, `Get-FileHash`, copy into
      `%LOCALAPPDATA%\Programs\codev\`.
- [x] 3.3 Add a short "PATH on Windows" paragraph: how to check, how
      to add `%LOCALAPPDATA%\Programs\codev\` (via `setx`, via
      System UI).

## 4. README

- [x] 4.1 Add the Windows one-liner to the "Getting started" block
      right after the Unix curl:
      ```powershell
      # Windows
      iwr -useb https://raw.githubusercontent.com/mairistem/codev/main/install.ps1 | iex
      ```
- [x] 4.2 Possibly rename the Unix curl comment to
      "# macOS or Linux" to clarify the distinction.

## 5. Validation

- [x] 5.1 `codev validate --strict` stays green.
- [x] 5.2 `cargo test --workspace` stays green (nothing Rust has
      changed, but we check).
- [x] 5.3 Manual YAML lint of the workflow (matrix indentation,
      PowerShell `if:`, quoting of `${{ … }}` variables).

## 6. Delivery

- [x] 6.1 After merge: bump `Cargo.toml` to `0.2.0`, `git tag
      v0.2.0`, `git push origin main v0.2.0`.
- [x] 6.2 Check the release on GitHub: **four** assets
      (`.tar.gz` × 3 + `.zip`) + a `SHA256SUMS` that references all
      four.
- [x] 6.3 Real-world test:
      - On macOS/Linux: `curl … | sh` must still work (the
        3 former targets).
      - On Windows (VM or physical machine): `iwr … | iex` must
        install, the suggested PATH command must work, then
        `codev --version` must return `0.2.0`.
