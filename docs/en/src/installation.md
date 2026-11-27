# Installation

You will type two commands, once: one installs Compagnon, the other
prepares your project's folder. After that, you only talk to Claude. If
someone in your team is used to a [terminal](glossary.md#terminal) (the
window where you type commands), they can do this part for you.

## Starting from nothing

Two things, before any command:

- **Claude Code.** It is the [Claude Code](glossary.md#claude-code)
  assistant that writes the plan and builds what you ask for. Get it from
  [claude.com/claude-code](https://claude.com/claude-code), which explains
  how to install it. It needs a Claude account with a subscription that
  includes Claude Code; that page says which.
- **A folder for your project** on your computer, empty to begin with if
  you start from an idea. Create it as you would any folder, for example
  `event-signup` in your Documents. Everything Claude builds will go there.

## 1. Install Compagnon

Open a terminal: on macOS, the Terminal app (press Cmd + Space, type
"Terminal", then Enter); on Windows, PowerShell. Copy the line for your
computer, paste it, then press Enter.

On macOS or Linux:

```bash
curl -sSL https://raw.githubusercontent.com/mairistem/compagnon/main/install.sh | sh
```

On Windows, in PowerShell:

```powershell
iwr -useb https://raw.githubusercontent.com/mairistem/compagnon/main/install.ps1 | iex
```

If the installer prints a line to add, or if the next command is "not
found", close the terminal and open a new one; "Under the hood: platforms,
versions and the PATH" below explains why, for the person helping you.

<details>
<summary>Under the hood: what the installer does</summary>

On macOS and Linux, the script downloads the latest release for your
computer from
[GitHub Releases](https://github.com/mairistem/compagnon/releases), checks
its SHA-256 fingerprint, and installs the program to
`~/.local/bin/compagnon`. It refuses to install if the fingerprint does not
match. When `~/.local/bin` is not on your `PATH`, it prints the line to add
to `~/.zshrc` or `~/.bashrc`. It needs `curl`, `tar`, and `sha256sum` or
`shasum`.

On Windows (PowerShell 5.1 or later, PowerShell 7 included), the script
checks the download the same way and copies `compagnon.exe` to
`%LOCALAPPDATA%\Programs\compagnon\`. It does **not** change your `PATH`:
when the folder is missing from it, the script prints the command to add
it. Open a new PowerShell window afterwards. Only x86_64 is prebuilt on
Windows; on ARM64, build from source (below).

To check the installation, `compagnon --version` prints the version. If the
command is still not found in a new terminal, the install folder is not on
your `PATH`: add it as the installer said. `command -v compagnon` (macOS,
Linux) or `Get-Command compagnon` (Windows) shows which program your
terminal runs.

</details>

## 2. Prepare your project's folder

Open a terminal in your project folder: on macOS, right-click the folder
in the Finder, then Services and "New Terminal at Folder"; on Windows,
right-click inside the folder in File Explorer, then "Open in Terminal".
Then type:

```bash
compagnon init
```

It may ask which [skills](glossary.md#skill) to give Claude: the proposed
choice is fine. It then asks for a few words about your project: always
type one sentence about it, because an empty answer opens an awkward text
editor (if that happens, call the person helping you). It adds a
`_compagnon/` folder, where your plans will be kept. You can close the
terminal now: from here on, you talk to Claude.

<details>
<summary>Under the hood: opening Claude Code from the terminal</summary>

In a terminal opened in the project folder, Claude Code starts with:

```bash
claude
```

Start it after `compagnon init`, or restart it, so that it sees the skills.
What `compagnon init` writes besides `_compagnon/` is described in
[Deploying Compagnon for a team](guides/team-rollout.md).

</details>

## Upgrading

`compagnon self-update` installs the latest release, checked like the
first one, then refreshes the skills of the project you are in. See
[`compagnon self-update`](reference/cli.md#compagnon-self-update) and
[`compagnon update`](reference/cli.md#compagnon-update) for the options.

## Other ways to install

Most people never need these.

<details>
<summary>Under the hood: platforms, versions and the PATH</summary>

Ready-made copies (binaries) are published for these platforms:

| Platform | Target |
|---|---|
| macOS, Apple Silicon | `aarch64-apple-darwin` |
| macOS, Intel | `x86_64-apple-darwin` |
| Linux, x86_64 (statically linked) | `x86_64-unknown-linux-musl` |
| Windows, x86_64 | `x86_64-pc-windows-msvc` |

To install a specific version, set `COMPAGNON_VERSION`:

```bash
curl -sSL https://raw.githubusercontent.com/mairistem/compagnon/main/install.sh | COMPAGNON_VERSION=0.6.1 sh
```

```powershell
$env:COMPAGNON_VERSION = '0.6.1'
iwr -useb https://raw.githubusercontent.com/mairistem/compagnon/main/install.ps1 | iex
```

The line `install.sh` asks you to add, when `~/.local/bin` is not on your
`PATH`:

```bash
export PATH="$HOME/.local/bin:$PATH"
```

The command `install.ps1` prints on Windows, which writes the *user*
`PATH` only:

```powershell
[Environment]::SetEnvironmentVariable('PATH', [Environment]::GetEnvironmentVariable('PATH', 'User') + ";$env:LOCALAPPDATA\Programs\compagnon", 'User')
```

Avoid `setx PATH "$env:PATH;..."`: it truncates values longer than 1024
characters and copies the machine `PATH` into the user `PATH`.

[git](glossary.md#git) is needed only for
[inherited sources](guides/inherited-sources.md) hosted in a git repository.

</details>

<details>
<summary>Under the hood: manual download from GitHub Releases</summary>

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

</details>

<details>
<summary>Under the hood: building from source with cargo</summary>

With a Rust toolchain (1.89 or later), you can build Compagnon from a clone
of the [repository](glossary.md#repository). This is the path for
contributors and for platforms without a prebuilt binary.

```bash
git clone https://github.com/mairistem/compagnon.git
cd compagnon
cargo install --path crates/compagnon-cli
```

The binary is installed to `~/.cargo/bin/compagnon`.

</details>

<details id="shell-completions">
<summary>Under the hood: shell completions</summary>

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

</details>

<details>
<summary>Under the hood: what upgrading replaces</summary>

`compagnon self-update` downloads the latest release for your platform,
verifies its SHA-256 against the release's `SHA256SUMS`, replaces the binary
in place, then, inside a Compagnon project, runs `compagnon update` with the
new binary. It needs `curl` and `tar`, which macOS, most Linux distributions
and Windows 10 or later include.

```bash
compagnon self-update                  # asks for confirmation on a terminal
compagnon self-update --check          # only report the running and latest versions
compagnon self-update --version 0.5.0  # a specific version, downgrades included
```

It only replaces a binary installed by `install.sh`, `install.ps1` or by
hand. When Compagnon was installed with `cargo install`, built from a clone,
or installed by a package manager, it prints the command that upgrades it
and changes nothing. It never downgrades unless you pass `--version`. On
Windows, the previous binary is kept as `compagnon.old.exe` next to the new
one until the next Compagnon command removes it.

`compagnon status` tells you when a newer release exists, at most once a
day; see [Does Compagnon check for updates?](faq.md#does-compagnon-check-for-updates)
to turn that off.

You can also run the installer again. Then, in each project, run
`compagnon update`: it regenerates the skills and refreshes, where
`compagnon init` wrote them, Compagnon's managed sections of `CLAUDE.md`
and of the [pull request](glossary.md#pull-request) template, and the
pinned version in `.github/workflows/compagnon.yml`. `compagnon doctor`
checks the result.

</details>

