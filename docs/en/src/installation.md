# Installation

Compagnon is a single binary. Prebuilt binaries are published on
[GitHub Releases](https://github.com/mairistem/compagnon/releases) for these
platforms:

| Platform | Target |
|---|---|
| macOS, Apple Silicon | `aarch64-apple-darwin` |
| macOS, Intel | `x86_64-apple-darwin` |
| Linux, x86_64 (statically linked) | `x86_64-unknown-linux-musl` |
| Windows, x86_64 | `x86_64-pc-windows-msvc` |

There are four ways to install it. The first two are the shortest and do not
require Rust.

## macOS and Linux: install.sh

```bash
curl -sSL https://raw.githubusercontent.com/mairistem/compagnon/main/install.sh | sh
```

The script detects your OS and architecture, downloads the matching archive
and the `SHA256SUMS` file from the latest GitHub Release, verifies the
archive's SHA-256, and installs the binary to `~/.local/bin/compagnon`. It
refuses to install if the checksum does not match, and it creates
`~/.local/bin` if needed.

When `~/.local/bin` is not on your `PATH`, the script prints the exact line to
add to `~/.zshrc` or `~/.bashrc`:

```bash
export PATH="$HOME/.local/bin:$PATH"
```

To install a specific version, set `COMPAGNON_VERSION`:

```bash
curl -sSL https://raw.githubusercontent.com/mairistem/compagnon/main/install.sh | COMPAGNON_VERSION=0.6.1 sh
```

The script requires `curl`, `tar`, and `sha256sum` or `shasum`.

## Windows: install.ps1

In PowerShell 5.1 or later (including PowerShell 7):

```powershell
iwr -useb https://raw.githubusercontent.com/mairistem/compagnon/main/install.ps1 | iex
```

The script downloads the `.zip` archive and `SHA256SUMS`, verifies the
archive with `Get-FileHash`, and copies `compagnon.exe` to
`%LOCALAPPDATA%\Programs\compagnon\`.

It does **not** modify your `PATH`. When the folder is missing from your user
`PATH`, it prints the command to add it:

```powershell
[Environment]::SetEnvironmentVariable('PATH', [Environment]::GetEnvironmentVariable('PATH', 'User') + ";$env:LOCALAPPDATA\Programs\compagnon", 'User')
```

This writes the *user* `PATH` only. Avoid `setx PATH "$env:PATH;..."`: it
truncates values longer than 1024 characters and copies the machine `PATH`
into the user `PATH`. Open a new PowerShell window afterwards so the change
takes effect.

To install a specific version:

```powershell
$env:COMPAGNON_VERSION = '0.6.1'
iwr -useb https://raw.githubusercontent.com/mairistem/compagnon/main/install.ps1 | iex
```

Only x86_64 (`AMD64`) is available prebuilt on Windows. On ARM64, build from
source.

## Manual download from GitHub Releases

If you prefer not to pipe a script into your shell, download the files
yourself from the
[latest release](https://github.com/mairistem/compagnon/releases/latest). Each
release contains one archive per target, named
`compagnon-<version>-<target>.tar.gz` (`.zip` on Windows), and a `SHA256SUMS`
file.

**macOS and Linux.** Download your archive and `SHA256SUMS` into the same
folder, then:

```bash
# Verify the checksum — on macOS: shasum -a 256 -c SHA256SUMS --ignore-missing
sha256sum -c SHA256SUMS --ignore-missing

tar -xzf compagnon-*.tar.gz
mkdir -p ~/.local/bin
cp compagnon-*/compagnon ~/.local/bin/
chmod 755 ~/.local/bin/compagnon
```

**Windows.** Download `compagnon-<version>-x86_64-pc-windows-msvc.zip` and
`SHA256SUMS`, then in PowerShell:

```powershell
# Verify the checksum
$zip = Get-Item compagnon-*-x86_64-pc-windows-msvc.zip
$expected = (Select-String -Path SHA256SUMS -Pattern $zip.Name).Line.Split(' ')[0]
$actual = (Get-FileHash $zip -Algorithm SHA256).Hash.ToLower()
if ($actual -ne $expected) { throw "SHA-256 mismatch" }

# Extract and install
Expand-Archive $zip -DestinationPath .
$dst = "$env:LOCALAPPDATA\Programs\compagnon"
New-Item -ItemType Directory -Path $dst -Force | Out-Null
Copy-Item compagnon-*\compagnon.exe $dst
```

Then add the folder to your user `PATH` as shown above.

## From source: cargo install

With a Rust toolchain (1.89 or later), you can build Compagnon from a clone of the
repository. This is the path for contributors and for platforms without a
prebuilt binary.

```bash
git clone https://github.com/mairistem/compagnon.git
cd compagnon
cargo install --path crates/compagnon-cli
```

The binary is installed to `~/.cargo/bin/compagnon`.

## Check the installation

```bash
compagnon --version
```

On macOS and Linux, `command -v compagnon` shows which binary your shell runs. On
Windows, use `Get-Command compagnon`. If the command is not found, the install
folder is not on your `PATH`; see the instructions above for your platform.

## Shell completions

`compagnon completions <SHELL>` prints a completion script to stdout for `bash`,
`zsh`, `fish`, `powershell` or `elvish`:

```bash
# bash
compagnon completions bash > ~/.local/share/bash-completion/completions/compagnon

# zsh — then run `compinit`
compagnon completions zsh > "${fpath[1]}/_compagnon"

# fish
compagnon completions fish > ~/.config/fish/completions/compagnon.fish
```

```powershell
# PowerShell, current session
compagnon completions powershell | Out-String | Invoke-Expression
```

## Requirements

- **Claude Code**, to use the skills. The CLI itself works without it.
- **git**, only for [inherited sources](guides/inherited-sources.md) hosted in
  a git repository.

## Upgrading

Run `compagnon self-update`:

```bash
compagnon self-update                  # asks for confirmation on a terminal
compagnon self-update --check          # only report the running and latest versions
compagnon self-update --version 0.5.0  # a specific version, downgrades included
```

It downloads the latest release for your platform, verifies its SHA-256
against the release's `SHA256SUMS` — as the installers do — and replaces the
binary in place. Then, inside a Compagnon project, it runs `compagnon update` with
the new binary. It needs `curl` and `tar`, which macOS, most Linux
distributions and Windows 10 or later include.

`compagnon self-update` only replaces a binary installed by `install.sh`,
`install.ps1` or by hand. When Compagnon was installed with `cargo install`,
built from a clone, or installed by a package manager, it prints the command
that upgrades it and changes nothing. It never downgrades unless you pass
`--version`. On Windows, the previous binary is kept as `compagnon.old.exe` next
to the new one until the next Compagnon command removes it.

`compagnon status` tells you when a newer release exists, at most once a day; see
[Does Compagnon check for updates?](faq.md#does-compagnon-check-for-updates) to turn
that off.

You can still run the installer again, or download a newer release. Then, in
each project, regenerate the skills so they match the new version:

```bash
compagnon update
```

`compagnon update` also refreshes Compagnon's managed sections in `CLAUDE.md` and the
pull request template, and the compagnon version pinned in
`.github/workflows/compagnon.yml`, where `compagnon init` wrote them. See
[`compagnon update`](reference/cli.md#compagnon-update) for how hand-edited skills
are handled, and `compagnon doctor` to check the result.
