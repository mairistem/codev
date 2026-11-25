# Design: adding the Windows target

## Context

See `proposal.md`. A purely additive extension of the `distribution`
capability — 4 targets instead of 3, one more install script.

## Goals / Non-Goals

This design covers: the Windows job in the matrix, the Windows
archive format, the mechanics of the `install.ps1` script, SHA-256
verification in PowerShell, the documentation update. It does not
cover Windows ARM64, Windows package managers (Scoop, Chocolatey,
winget), or automatic modification of the user PATH.

## Decisions

### Decision: `.zip` for Windows, `.tar.gz` for Unix

| Format | Windows | Unix |
|---|---|---|
| `.tar.gz` | Extraction possible (native `tar` since Windows 10 build 1803) but culturally Unix | Standard |
| `.zip` | Windows standard, native `Expand-Archive` since PowerShell 5.1 | Requires a separate `unzip` on minimal Linux |

**Chosen**: one format per OS family. This is what ripgrep, fd,
starship, bat do; the pattern is well established.

### Decision: `windows-latest` runner, no pinned version

`windows-latest` currently points to Windows Server 2022. A frozen
`windows-2019` would make us depend on a machine that GitHub will
eventually deprecate — and nothing justifies that precaution.

**Rejected alternative**: `windows-2022`. No additional benefit, a
slight risk if GitHub renames it.

### Decision: `x86_64-pc-windows-msvc`, not GNU

Rust supports two Windows toolchains:

- `x86_64-pc-windows-msvc`: dynamically linked to `msvcrt.dll`
  (Visual C++ Runtime); this is the official Microsoft path.
- `x86_64-pc-windows-gnu`: linked to MinGW; more portable in theory,
  but rarely used.

**Chosen: MSVC.** It is the default on Windows in the Rust
community. The `windows-latest` runners already provide the
Microsoft headers/libs; no additional installation.

### Decision: the `install.ps1` script does not modify the PATH

Symmetry with `install.sh`, which never writes to
`.zshrc`/`.bashrc`. Automatically modifying the user PATH with
`setx` or `[Environment]::SetEnvironmentVariable` is **irreversible
in practice** for the user — if they uninstall codev, the entry
stays in their registry.

The script prints the exact command to type. That is two more lines
for the user, versus permanent noise we would impose on them.

### Decision: `iwr -useb … | iex` as the official incantation

The canonical PowerShell pattern — used by rustup, Scoop,
oh-my-posh, starship. `iwr` (alias of `Invoke-WebRequest`), `-useb`
(alias of `-UseBasicParsing`, bypasses the dependency on IE assets),
`iex` (alias of `Invoke-Expression`) runs the content as a script.

Alternative: `Set-ExecutionPolicy Bypass -Scope Process -Force ; ...`.
Not needed — a script piped from stdin runs without any
`Set-ExecutionPolicy` restriction.

### Decision: destination `$env:LOCALAPPDATA\Programs\codev\`

`$env:LOCALAPPDATA` is the user's local data folder
(`C:\Users\<user>\AppData\Local\` typically). `Programs\` in it hosts
binaries installed at user level — the pattern of Windows Terminal,
VS Code, PowerShell 7.

**Rejected alternative**: `$env:USERPROFILE\.local\bin\` for symmetry
with Unix. Works but is not very idiomatic on Windows (users look for
their binaries in `%LOCALAPPDATA%`).

### Decision: extended matrix, no new workflow

A new row in the `strategy.matrix.include` matrix of the existing
workflow, plus a conditional branch on the OS for packaging (`.zip`
vs `.tar.gz`). An `if: matrix.target == 'x86_64-pc-windows-msvc'`
handles the Windows specifics in the same job.

**Rejected alternative**: two separate workflows (`release-unix.yml`
+ `release-windows.yml`). Duplication of the final aggregating
`release` job. Rejected for the sake of simplicity.

### Decision: the `release` job uses the right binary in SHA256SUMS

The aggregating job computes the SHA-256 of **all** the `.tar.gz`
and `.zip` files in the `dist/` folder, whatever their extension. The
current line `shasum -a 256 *.tar.gz > SHA256SUMS` becomes
`shasum -a 256 *.tar.gz *.zip > SHA256SUMS`.

## Risks / Trade-offs

- **PowerShell 5.1 vs 7** — `Expand-Archive`, `Get-FileHash`,
  `Invoke-WebRequest` all exist since PowerShell 5.1. The script
  requires nothing recent; it works on a vanilla Windows 10.
- **Antivirus blocking the downloaded binary** — Windows Defender
  can be overzealous about an `.exe` downloaded and run from
  `%LOCALAPPDATA%`. Documented in the FAQ: "if blocked, mark
  `codev.exe` as excluded, or sign later."
- **Windows code signing (Authenticode)** — not covered in V1. A
  user who runs `codev.exe` will see an "unknown publisher" warning.
  Can be deferred until requested.

## Migration Plan

None. The 3 existing targets and `install.sh` are preserved. A
Windows user who installed via `cargo install` can migrate in one
command:

```powershell
iwr -useb https://raw.githubusercontent.com/mairistem/codev/main/install.ps1 | iex
```
