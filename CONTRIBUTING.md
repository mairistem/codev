# Contributing to codev

English · [Français](CONTRIBUTING.fr.md)

Thank you for helping improve codev. This guide explains how to propose a
change, set up a development environment, and get a pull request merged.

## Two ways to contribute

**Small fixes — a plain pull request.** Typos, documentation wording, a
comment, a test that was missing: open a pull request directly. No codev
change is needed.

**Everything else — a codev change.** codev is developed with codev. A new
feature, a behavior change, a new flag or a refactor starts as a change under
`_codev/changes/`, reviewed together with the code:

1. For anything substantial, open an issue first to discuss the idea.
2. In your clone, run `/codev-propose <your idea>` in Claude Code — or create
   the change by hand with `codev new change <name>` and write its artifacts.
3. Implement it, with `/codev-apply` or by hand, checking off `tasks.md`.
4. Archive the change with `codev archive --change <name>` once the work is
   done, so that the main specs in `_codev/specs/` describe the new behavior.
5. Open a pull request that contains the archived change and the code.

If you are not sure which path applies, open the pull request anyway and ask:
we would rather help than turn a contribution away.

## Development setup

You need:

- **Rust**, stable. The minimum supported version is 1.89; the workspace uses
  the 2024 edition.
- **git**.
- **Claude Code**, to use the skills while contributing (optional).

```bash
git clone https://github.com/mairistem/codev.git
cd codev
cargo build
cargo run --bin codev -- --help
```

To use your build on other projects, install it:

```bash
cargo install --path crates/codev-cli
```

The [architecture chapter](docs/en/src/architecture.md) explains how the code
is organized.

## Checks

CI runs the following on every pull request. Run them locally before pushing:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo run --locked --quiet --bin codev -- validate --strict
```

CI also runs the tests on Linux, macOS and Windows, checks that the workspace
builds with Rust 1.89 (`cargo check --workspace --locked`), and audits the
dependencies with `cargo audit`. Warnings are treated as errors.

The last command validates codev's own `_codev/` tree: specs, active changes
and decision seals must stay valid.

## Commits

Commit messages follow [Conventional Commits](https://www.conventionalcommits.org/),
in English:

```text
feat(cli): add --lang to codev docs
fix(engine): name the missing requirement when a MODIFIED delta fails
docs: explain inherited git sources
```

Common types are `feat`, `fix`, `docs`, `refactor`, `test` and `chore`; the
scope is the crate or area concerned (`core`, `engine`, `agents`, `cli`,
`skills`, `docs`). Keep the subject line short and in the imperative mood.

If an agent co-wrote a commit, add a `Co-Authored-By:` trailer.

## Documentation

The documentation exists in English (`docs/en/`) and French (`docs/fr/`), with
the same files and the same structure. When a pull request changes behavior,
update both languages — or say in the pull request that the French side needs
a follow-up, so a maintainer can take care of it.

Each chapter starts with a single `# ` title, and chapters are listed in
`docs/<lang>/src/SUMMARY.md`. `codev docs` embeds the chapters of that list, in
order, into a single page, so links between chapters must be relative
(`concepts.md#delta`) and point to headings that are unique across the book.

To preview the site, install [mdBook](https://rust-lang.github.io/mdBook/) and
run `mdbook serve docs/en`.

User-visible changes get an entry under `## [Unreleased]` in
[CHANGELOG.md](CHANGELOG.md).

## Release process

For maintainers:

1. Make sure `main` is green and `CHANGELOG.md` is up to date.
2. Bump `version` in the `[workspace.package]` section of the root
   `Cargo.toml`, and run `cargo build` to update `Cargo.lock`.
3. In `CHANGELOG.md`, rename `## [Unreleased]` to `## [X.Y.Z] - YYYY-MM-DD`,
   add a new empty `## [Unreleased]` section above it, and update the links at
   the bottom of the file.
4. Commit as `chore(release): vX.Y.Z`.
5. Tag and push:

   ```bash
   git tag vX.Y.Z
   git push origin main --tags
   ```

The release workflow builds the four prebuilt binaries (macOS arm64 and x86_64,
Linux x86_64 musl, Windows x86_64), computes `SHA256SUMS`, and publishes the
GitHub Release with generated notes. The installers pick up the new version
immediately.

## Security

Do not open a public issue for a vulnerability. Follow
[SECURITY.md](SECURITY.md) instead.

## Code of conduct

Participation in this project is governed by the
[code of conduct](CODE_OF_CONDUCT.md).
