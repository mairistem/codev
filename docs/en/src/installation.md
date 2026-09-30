# Installation

codev is a single binary. Prebuilt binaries are published on
[GitHub Releases](https://github.com/mairistem/codev/releases) for these
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
curl -sSL https://raw.githubusercontent.com/mairistem/codev/main/install.sh | sh
```

The script detects your OS and architecture, downloads the matching archive
and the `SHA256SUMS` file from the latest GitHub Release, verifies the
archive's SHA-256, and installs the binary to `~/.local/bin/codev`. It
refuses to install if the checksum does not match, and it creates
`~/.local/bin` if needed.

When `~/.local/bin` is not on your `PATH`, the script prints the exact line to
add to `~/.zshrc` or `~/.bashrc`:

```bash
export PATH="$HOME/.local/bin:$PATH"
```

To install a specific version, set `CODEV_VERSION`:

```bash
curl -sSL https://raw.githubusercontent.com/mairistem/codev/main/install.sh | CODEV_VERSION=0.4.0 sh
```

The script requires `curl`, `tar`, and `sha256sum` or `shasum`.

## Windows: install.ps1

In PowerShell 5.1 or later (including PowerShell 7):

```powershell
iwr -useb https://raw.githubusercontent.com/mairistem/codev/main/install.ps1 | iex
```

The script downloads the `.zip` archive and `SHA256SUMS`, verifies the
archive with `Get-FileHash`, and copies `codev.exe` to
`%LOCALAPPDATA%\Programs\codev\`.

It does **not** modify your `PATH`. When the folder is missing from your user
`PATH`, it prints the command to add it:

```powershell
[Environment]::SetEnvironmentVariable('PATH', [Environment]::GetEnvironmentVariable('PATH', 'User') + ";$env:LOCALAPPDATA\Programs\codev", 'User')
```

This writes the *user* `PATH` only. Avoid `setx PATH "$env:PATH;..."`: it
truncates values longer than 1024 characters and copies the machine `PATH`
into the user `PATH`. Open a new PowerShell window afterwards so the change
takes effect.

To install a specific version:

```powershell
$env:CODEV_VERSION = '0.4.0'
iwr -useb https://raw.githubusercontent.com/mairistem/codev/main/install.ps1 | iex
```

Only x86_64 (`AMD64`) is available prebuilt on Windows. On ARM64, build from
source.

## Manual download from GitHub Releases

If you prefer not to pipe a script into your shell, download the files
yourself from the
[latest release](https://github.com/mairistem/codev/releases/latest). Each
release contains one archive per target, named
`codev-<version>-<target>.tar.gz` (`.zip` on Windows), and a `SHA256SUMS`
file.

**macOS and Linux.** Download your archive and `SHA256SUMS` into the same
folder, then:

```bash
# Verify the checksum — on macOS: shasum -a 256 -c SHA256SUMS --ignore-missing
sha256sum -c SHA256SUMS --ignore-missing

tar -xzf codev-*.tar.gz
mkdir -p ~/.local/bin
cp codev-*/codev ~/.local/bin/
chmod 755 ~/.local/bin/codev
```

**Windows.** Download `codev-<version>-x86_64-pc-windows-msvc.zip` and
`SHA256SUMS`, then in PowerShell:

```powershell
# Verify the checksum
$zip = Get-Item codev-*-x86_64-pc-windows-msvc.zip
$expected = (Select-String -Path SHA256SUMS -Pattern $zip.Name).Line.Split(' ')[0]
$actual = (Get-FileHash $zip -Algorithm SHA256).Hash.ToLower()
if ($actual -ne $expected) { throw "SHA-256 mismatch" }

# Extract and install
Expand-Archive $zip -DestinationPath .
$dst = "$env:LOCALAPPDATA\Programs\codev"
New-Item -ItemType Directory -Path $dst -Force | Out-Null
Copy-Item codev-*\codev.exe $dst
```

Then add the folder to your user `PATH` as shown above.

## From source: cargo install

With a Rust toolchain (1.89 or later), you can build codev from a clone of the
repository. This is the path for contributors and for platforms without a
prebuilt binary.

```bash
git clone https://github.com/mairistem/codev.git
cd codev
cargo install --path crates/codev-cli
```

The binary is installed to `~/.cargo/bin/codev`.

## Check the installation

```bash
codev --version
```

On macOS and Linux, `command -v codev` shows which binary your shell runs. On
Windows, use `Get-Command codev`. If the command is not found, the install
folder is not on your `PATH`; see the instructions above for your platform.

## Shell completions

`codev completions <SHELL>` prints a completion script to stdout for `bash`,
`zsh`, `fish`, `powershell` or `elvish`:

```bash
# bash
codev completions bash > ~/.local/share/bash-completion/completions/codev

# zsh — then run `compinit`
codev completions zsh > "${fpath[1]}/_codev"

# fish
codev completions fish > ~/.config/fish/completions/codev.fish
```

```powershell
# PowerShell, current session
codev completions powershell | Out-String | Invoke-Expression
```

## Requirements

- **Claude Code**, to use the skills. The CLI itself works without it.
- **git**, only for [inherited sources](guides/inherited-sources.md) hosted in
  a git repository.

## Upgrading

Run the installer again, or download a newer release. Then, in each project,
regenerate the skills so they match the new version:

```bash
codev update
```

See [`codev update`](reference/cli.md#codev-update) for how hand-edited skills
are handled.
