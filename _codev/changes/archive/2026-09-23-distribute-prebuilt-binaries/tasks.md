# Tasks

## 1. GitHub Actions workflow

- [x] 1.1 Create `.github/workflows/release.yml` with:
      - Trigger `on: push: tags: - 'v*.*.*'`.
      - `build` job as a matrix over 3 targets:
        - `runner: macos-14`, `target: aarch64-apple-darwin`
        - `runner: macos-13`, `target: x86_64-apple-darwin`
        - `runner: ubuntu-24.04`, `target: x86_64-unknown-linux-musl`
      - Steps per job: checkout, install the Rust toolchain with the
        target (via `dtolnay/rust-toolchain@stable`), cargo build
        release with `--target <target>`, packaging
        `codev-<version>-<target>.tar.gz`, upload artifact.
- [x] 1.2 On the Linux musl runner: install
      `musl-tools` (`apt install -y musl-tools`) before the build.
- [x] 1.3 Final `release` job that:
      - `needs: [build]` (depends on the 3 matrix jobs)
      - `download-artifact` retrieves the three tarballs
      - computes the SHA-256 of each tarball → aggregates into `SHA256SUMS`
      - `softprops/action-gh-release@v2` creates the GitHub Release with
        the three tarballs + `SHA256SUMS` as assets, `name: ${{
        github.ref_name }}`, `generate_release_notes: true`.
- [x] 1.4 The workflow writes no commit and requires no secret
      other than `GITHUB_TOKEN` (granted automatically by GH for
      release writes).

## 2. `install.sh` script

- [x] 2.1 Create `install.sh` at the repository root, `#!/bin/sh`,
      POSIX-compatible shebang (no bashisms). Header with
      description and license.
- [x] 2.2 Internal functions:
      - `detect_target()`: combines `uname -s` (Darwin/Linux) and
        `uname -m` (arm64/aarch64/x86_64) to return one of the
        three supported targets, or fails with a message.
      - `resolve_version()`: `${CODEV_VERSION:-$(curl -s
        https://api.github.com/repos/mairistem/codev/releases/latest |
        grep '"tag_name"' | head -1 | cut -d '"' -f 4)}`. Strips the
        leading `v`.
      - `download_and_verify()`: downloads tarball + `SHA256SUMS`,
        verifies via `sha256sum` (Linux) or `shasum -a 256` (macOS).
      - `install_binary()`: extracts into a tempdir, `mkdir -p
        ~/.local/bin`, `cp <tempdir>/codev ~/.local/bin/codev`,
        `chmod 755`, cleanup.
      - `check_path()`: tests whether `~/.local/bin` is in `$PATH`.
- [x] 2.3 Main body chaining these functions, with clear messages
      at each step (`echo "==> ..."`).
- [x] 2.4 Error messages: "unsupported OS or arch" →
      fallback to `cargo install --path`.
- [x] 2.5 Refuse if `curl` or `tar` are missing — clear messages.
- [x] 2.6 The script MUST be testable via `sh install.sh` locally
      (without piping), so that a user can first `curl >
      install.sh`, `cat install.sh`, then `sh install.sh`.

## 3. Installation section of the docs

- [x] 3.1 Rework section 2 ("Installation") of
      `docs/codev.md` with the three paths in order:
      recommended (curl), alternative (manual download), and
      contributor (cargo).
- [x] 3.2 Detail the manual path: link to releases, `tar xzf`,
      copy into `~/.local/bin/`, verification with `sha256sum -c
      SHA256SUMS`.
- [x] 3.3 Add a short paragraph on `$PATH`: how to check it,
      how to add `~/.local/bin` if it is missing.
- [x] 3.4 Rename the "Shell completions" subsection — it
      remains useful, but makes sense after installation, not before.

## 4. README

- [x] 4.1 Add at the top of `README.md` (after any
      description) a "Quick install" block with the
      command `curl -sSL https://raw.githubusercontent.com/mairistem/codev/main/install.sh | sh`.
- [x] 4.2 Point to `docs/codev.md` (or `codev docs`) for the
      details.

## 5. Manual validation

- [x] 5.1 Cross-read the YAML workflow against the GitHub Actions
      documentation to avoid syntax mistakes (finicky YAML
      indentation).
- [x] 5.2 Local test of the script (partial): run
      `install.sh` with `CODEV_VERSION=0.1.0` **once a
      release exists for that tag**. Refuse to install if the SHA
      differs (manual test: tamper with the downloaded file).
- [x] 5.3 If no release exists yet: manual dry-run of the
      script by commenting out the `curl` calls and working on a
      local tarball.
- [x] 5.4 `codev validate --strict` stays green.

## 6. Delivery

- [x] 6.1 After this change is merged: bump the version in
      `Cargo.toml` (workspace) to `0.2.0`, `git tag v0.2.0`, `git
      push origin v0.2.0`. The workflow runs, the release appears.
- [x] 6.2 Check the release on GitHub: three tar.gz assets + one
      `SHA256SUMS`.
- [x] 6.3 Full-scale test:
      `curl -sSL https://raw.githubusercontent.com/mairistem/codev/main/install.sh | sh`
      on a Linux VM or a clean Mac (without Rust). Check:
      `codev --version` returns `0.2.0`, `codev docs` opens, `codev
      list` works in an initialized repository.
