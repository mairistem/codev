# Design: distributing codev through prebuilt binaries

## Context

See `proposal.md`. Change **outside the Rust crates** — it touches
only the GitHub Actions workflow, the `install.sh` script, and the
documentation. Not a single line of Rust moves.

## Goals / Non-Goals

This design frames: the supported targets, the artifact format, the
workflow mechanics, the behavior of the `install.sh` script, the
integrity check. It does not frame Windows, Linux ARM64, Homebrew,
auto-update, or GPG signing.

## Decisions

### Decision: three targets for V1 — macOS arm64/x86_64, Linux x86_64 musl

The target audience (JVS teams + curious open source contributors) is
≥ 95% on these three targets. Going further adds CI matrix entries
(slow builds, cache to manage) with no measured benefit.

**Rejected alternative**: adding `aarch64-unknown-linux-musl` and
`x86_64-pc-windows-msvc` from the start. Rejected for V1 — easy
extensions once a real user shows up.

### Decision: static musl for Linux, not glibc

A binary compiled against musl is **statically linked** — it runs on
any Linux distribution, whatever the glibc version. This is the
pattern adopted by ripgrep, fd, bat, sccache, and most infrastructure
Rust CLIs.

A dynamic glibc binary could break on older systems
("GLIBC_2.34 not found"). musl trades 10-15% of binary size for full
portability — a trade-off worth taking.

### Decision: `.tar.gz` format, not `.zip`

`.tar.gz` is native on macOS and Linux — tar is present everywhere,
the user has nothing to install to extract. `.zip` would require a
separate `unzip` on minimal Linux.

Windows will use `.zip` **when** Windows is supported. For V1, a
single format simplifies the install script.

### Decision: SHA-256 in a `SHA256SUMS`, no GPG signature

Two possible levels of guarantee:

| Option | Pro | Con |
|---|---|---|
| **A. Nothing** | Simple | An attacker who compromises the CDN can serve a modified binary |
| **B. SHA-256 in a `SHA256SUMS`** published by the same workflow | Defends against tampering in transit; standard `sha256sum -c` format | Does not defend against a compromise of the GH Actions workflow itself |
| **C. GPG / sigstore signature** | Strong defense against any attacker outside the maintainers | Key to manage, publish, rotate. For an internal tool this is disproportionate in V1. |

**Chosen: B.** The pattern of mainstream Rust CLIs (ripgrep, fd,
starship). Room to move toward C if the context hardens (public
release, regulated sector, etc.).

### Decision: `install.sh` downloads the SHA256SUMS separately and verifies

The script does not trust the server — it downloads the
`SHA256SUMS` published in the release, computes the SHA-256 of the
file it downloaded, compares, and refuses on mismatch. Standard
practice.

**Rejected alternative**: downloading only the archive, without
verifying. Rejected — the cost is zero (one more `curl`, a local
`sha256sum`).

### Decision: destination `~/.local/bin/`, not `/usr/local/bin/`

`~/.local/bin/` is the standard XDG folder for a binary installed by
the user, without root privileges. Compatible with macOS/Linux, does
not require `sudo`.

**Rejected alternative**: `/usr/local/bin/`. Requires `sudo` on many
systems; possible conflict with Homebrew.

### Decision: version in the archive name

`codev-0.2.0-aarch64-apple-darwin.tar.gz` — the version is visible
without having to decompress. Useful for a user who keeps several
versions side by side, and for the `install.sh` script, which builds
the name from the resolved version.

### Decision: the GH Actions workflow pushes nothing to the repository

No auto commit, no automatic push. The workflow reads the repo,
produces artifacts, publishes to GitHub Releases. That's all.

**Rationale**: a workflow that pushed commits (for example to bump
the version) creates a potentially recursive push → CI → push cycle.
We prefer a simple model where the developer bumps the version
locally, tags, pushes the tag, and the workflow publishes the release.

## Risks / Trade-offs

- **A user pipes `curl … | sh` from a compromised source.**
  → **Mitigation**: the script verifies the SHA-256 from the same
  server, which defends against a CDN compromise but not against a
  compromise of the repo itself. For a stronger guarantee, the user
  can download the script, read it, then run it (`sh
  install.sh`). Documented in the docs.
- **`curl -sSL … | sh` remains a debated practice** — but standard
  for rustup, oh-my-zsh, Homebrew, starship. The ergonomics/security
  trade-off is accepted by the community.
- **The three-target matrix doubles CI time** — V1 probably runs
  in 5-10 min end-to-end. Not blocking.
- **The version in `Cargo.toml` must match the tag** — otherwise
  `install.sh` will not find the archive. A small `xtask
  release <version>` script or a note in `docs/codev.md` can help. No
  automatic mechanism for V1 — a human checklist is enough.

## Migration Plan

For a user who has **already** installed codev via `cargo install`
and wants to migrate to the curl path:

1. `rm ~/.cargo/bin/codev` (optional, replaced at the next
   `cargo install`)
2. `curl -sSL https://…/install.sh | sh`
3. Check that `~/.local/bin/codev` is used
   (`which codev` must return that path, provided the PATH orders
   `~/.local/bin` before `~/.cargo/bin`)

For a **new** user: follow the Installation section of
`codev docs` (recommended: `curl … | sh`).
