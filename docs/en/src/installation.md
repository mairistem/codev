# Installation

You will type two commands, once: one installs Compagnon, the other
prepares your project's folder. After that, you only talk to Claude. If
someone in your team is used to a [terminal](glossary.md#terminal) (the
window where you type commands), they can do this part for you.

<figure class="diagram">
<div class="scroll">
<svg viewBox="0 0 760 140" role="img" aria-label="Installing in four steps: 1, you install Compagnon with one line, once, in a terminal; 2, you set up your folder with compagnon init; 3, you ask Claude in your own words, in Claude Code; 4, Claude writes a plan and waits for your approval.">
<g stroke-width="2" style="fill:var(--dg-box)">
<rect x="4" y="16" width="168" height="118" rx="14" style="stroke:var(--dg-you)"/>
<rect x="198" y="16" width="168" height="118" rx="14" style="stroke:var(--dg-you)"/>
<rect x="392" y="16" width="168" height="118" rx="14" style="stroke:var(--dg-you)"/>
<rect x="586" y="16" width="168" height="118" rx="14" style="stroke:var(--dg-claude)"/>
</g>
<path d="M175,75h18m-6,-5l6,5l-6,5M369,75h18m-6,-5l6,5l-6,5M563,75h18m-6,-5l6,5l-6,5" fill="none" stroke-width="2" style="stroke:var(--dg-line)"/>
<g class="mono" font-size="12" style="fill:var(--dg-muted)">
<text x="20" y="44">1 · YOU</text><text x="214" y="44">2 · YOU</text><text x="408" y="44">3 · YOU</text><text x="602" y="44">4 · CLAUDE</text>
</g>
<g font-size="16" font-weight="600" style="fill:var(--dg-ink)">
<text x="20" y="78">Install</text><text x="214" y="78">Set up</text><text x="408" y="78">Ask Claude</text><text x="602" y="78">Plans</text>
</g>
<g font-size="13" style="fill:var(--dg-muted)">
<text x="20" y="102">one line, once</text><text x="20" y="120">in a terminal</text><text x="214" y="102">compagnon init</text><text x="214" y="120">in your folder</text><text x="408" y="102">in your own words</text><text x="408" y="120">in Claude Code</text><text x="602" y="102">and waits for</text><text x="602" y="120">your approval</text>
</g>
</svg>
</div>
<figcaption>Two commands, once; after that, you only talk to Claude.</figcaption>
</figure>

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

<details>
<summary>Under the hood: what init writes</summary>

```bash
compagnon init --yes --language en
```

```text
compagnon initialized in /home/you/acme-app

  Structure   _compagnon/ — specs, decisions, changes, schemas
  Skills      compagnon-propose, compagnon-explore, compagnon-onboard, compagnon-apply, compagnon-sync, compagnon-archive, compagnon-update, compagnon-configure → .claude/skills/

  Repository integration
    + CLAUDE.md                          created
    + .github/workflows/compagnon.yml        created
    + .github/PULL_REQUEST_TEMPLATE.md   created

  17 file(s) created, 0 updated

→ Recommended next step: in Claude Code, ask
  "configure Compagnon for this project" (shortcut: /compagnon-configure).
```

The probe, the questions and every file written are described in
[What `compagnon init` does](reference/cli.md#what-compagnon-init-does).
`compagnon doctor` then lists what the setup still lacks, with the fix for
each item.

</details>

## Your first request

Open Claude Code on your project folder, for example from the Claude
desktop app, choosing that folder. Then say, in your own words, what you
want, and ask for a plan first:

<div class="example">
<p class="example-title">Example</p>

```text
> Prepare a plan for a sign-up form for an event
⏺ I will write the plan in _compagnon/changes/event-signup/: why, what
  changes, how, and the tasks.
⏺ The plan is ready. Read proposal.md first; nothing is built before
  you approve it.
```

Claude writes a plan and stops: it builds nothing yet. The next page reads it with you.

</div>

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

## Next step

[Your first change](quickstart.md)
