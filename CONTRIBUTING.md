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

## Changing a role

The built-in roles (`assets/roles/`) are prompts: their effect is measured,
not guessed. A pull request that modifies a file of `assets/roles/` runs
the role evaluations before and after the change and pastes both scores —
recall, false positives and evidence validity of the roles concerned — in
its description. The reference is the committed baseline,
`evals/baseline/report.md`.

The suite needs [Claude Code](https://claude.com/claude-code) installed and
signed in (or `ANTHROPIC_API_KEY` set). It calls a model: a full run of the
40 cases costs about fifteen US dollars, each case capped at 3 by default.

```bash
cargo build --bin codev
cargo run -p codev-evals -- list                       # the cases
cargo run -p codev-evals -- run --role security        # the cases of one role
cargo run -p codev-evals -- run --out target/evals/mine  # all of them
cargo run -p codev-evals -- score target/evals/mine    # score again, no model
```

`run` writes `report.md` and `report.json` in its output directory and
exits with 1 when a role is under a threshold (`--min-recall`,
`--max-false-positives`, `--min-evidence`) or a case stopped; a directory
that already holds observations is resumed. `CODEV_EVAL_MODEL` picks the
model (default: Claude Code's), `CODEV_EVAL_BUDGET_USD` and
`CODEV_EVAL_TIMEOUT_SECS` bound each case. Compare on the same model as the
baseline. Maintainers can also start the **Role evaluations** workflow by
hand; it needs the `ANTHROPIC_API_KEY` repository secret and skips
otherwise.

### Writing a case

A case is a directory of `evals/cases/`: `case.yaml`, the project on its
default branch (`base/`) and the files the change adds or modifies,
planning artifacts included (`change/`). Each expected finding gives its
`severity`, its `locations` and a `category` from
`evals/categories.yaml`, the shared vocabulary whose terms recognise a
category in a finding's title or recommendation; `alternatives` lists other
places a reviewer may rightly cite the same defect. Two categories
expected in the same case must share no term.

A case is refused when its plan cites, between backquotes, a file or a
test its fixture does not contain: a reviewer would rightly report the
gap, and the role would be charged with a false positive. A plan step
may name what it will create under `planned`; a `code.review` case, whose
tasks are done, plans nothing. `cargo run -p codev-evals -- list` loads
every case and names what is wrong.

Changing a term of the vocabulary changes the scores: score the baseline
again (`cargo run -p codev-evals -- score evals/baseline`) and say which
findings it re-classifies.

Do not tune a role and its cases in the same pull request: a case states
what a reviewer should find, independently of the role's wording. When the
scoring rules or the cases change, record the baseline again — remove `evals/baseline/`, then
`cargo run -p codev-evals -- run --out evals/baseline` — which the tests
check.

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

Callouts are blockquotes opening on a bold title: `> **Note**` or
`> **Warning**` in English, `> **Remarque**` or `> **Attention**` in French.
The website and `codev docs` render notes with a green rule and warnings with
a coral one.

To preview the site, install [mdBook](https://rust-lang.github.io/mdBook/) and
run `mdbook serve docs/en`.

User-visible changes are described in the `## Release note` section of their
proposal. Once the change is archived, `codev changelog --write` adds them to
the `## [Unreleased]` section of [CHANGELOG.md](CHANGELOG.md); `codev doctor`
tells when it is behind.

## Release process

For maintainers:

1. Make sure `main` is green and `codev doctor` reports `changelog` ok.
2. Bump `version` in the `[workspace.package]` section of the root
   `Cargo.toml`, and run `cargo build` to update `Cargo.lock`.
3. Run `codev changelog --release X.Y.Z --write`: the generated
   `[Unreleased]` items become the `## [X.Y.Z] - YYYY-MM-DD` section of
   `CHANGELOG.md`, under a new empty `## [Unreleased]`. Review it, and update
   the links at the bottom of the file.
4. Commit as `chore(release): vX.Y.Z`.
5. Tag and push right away — until the tag exists, `codev changelog` still
   counts the released changes as unreleased:

   ```bash
   git tag vX.Y.Z
   git push origin main --tags
   ```

The release workflow builds the four prebuilt binaries (macOS arm64 and x86_64,
Linux x86_64 musl, Windows x86_64), computes `SHA256SUMS`, and publishes the
GitHub Release. Its body is `codev changelog --until vX.Y.Z --format markdown`,
run by the binary built from the tag, followed by GitHub's generated notes.
The installers pick up the new version immediately.

## Security

Do not open a public issue for a vulnerability. Follow
[SECURITY.md](SECURITY.md) instead.

## Code of conduct

Participation in this project is governed by the
[code of conduct](CODE_OF_CONDUCT.md).
