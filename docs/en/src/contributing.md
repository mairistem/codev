# Contributing to Compagnon

Compagnon is developed with Compagnon: its own specs, decisions and changes
are in the `_compagnon/` folder of its
[repository](glossary.md#repository). This page summarises how to
contribute; the full guide is
[CONTRIBUTING.md](https://github.com/mairistem/compagnon/blob/main/CONTRIBUTING.md),
and [Architecture](architecture.md) explains how the code is organized.

## Two ways to contribute

- **A small fix: a plain [pull request](glossary.md#pull-request).** A
  typo, documentation wording, a comment, a missing
  [test](glossary.md#test): open the pull request directly, no Compagnon
  change needed.
- **Everything else: a Compagnon change.** A new feature, a behavior change,
  a new flag or a refactor starts as a change under `_compagnon/changes/`,
  reviewed together with the code.

If you are not sure which applies, open the pull request anyway and ask.

## Setting up and checking your work

You need Rust (stable; the minimum supported version is 1.89, the
workspace uses the 2024 edition), [git](glossary.md#git), and, to use the
skills while contributing, Claude Code (optional).

```bash
git clone https://github.com/mairistem/compagnon.git
cd compagnon
cargo build
cargo run --bin compagnon -- --help
```

To use your build on other projects:

```bash
cargo install --path crates/compagnon-cli
```

[CI](glossary.md#ci) runs these on every pull request; run them before
pushing:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo run --locked --quiet --bin compagnon -- validate --strict
```

The last one validates Compagnon's own `_compagnon/` tree: its specs,
active changes and decision seals must stay valid. CI also runs the tests
on Linux, macOS and Windows, checks that the workspace builds with Rust
1.89 (`cargo check --workspace --locked`), and audits the dependencies with
`cargo audit`. Warnings are errors.

## The change comes first

A change of behavior starts with a Compagnon change, before the code. The
repository's `CLAUDE.md` says so to Claude Code in every session: "Do not
modify behavior without a compagnon change." Trivial fixes — typos,
documentation wording — are exempt.

1. For anything substantial, open an issue first to discuss the idea.
2. In your clone, run `/compagnon-propose <your idea>` in Claude Code, or
   create the change by hand with `compagnon new change <name>` and write
   its plan.
3. Implement it, with `/compagnon-apply` or by hand, checking off
   `tasks.md`.
4. Once the work is done, archive it with `compagnon archive --change
   <name>`, so that `_compagnon/specs/` describes the new behavior.
5. Open a pull request holding the archived change and the code.

Follow the accepted decisions of `_compagnon/decisions/` rather than
debating them again; to change one, a change proposes to supersede it.

## Evaluating a change to the skills

The roles and skills are prompts: their effect is measured, not guessed.
Accepted
[decision 0008](https://github.com/mairistem/compagnon/blob/main/_compagnon/decisions/0008-every-change-to-the-workflow-is-evaluated-per-call-for-quality-and-tokens.md)
makes this the rule: a pull request that changes a role (`assets/roles/`),
a skill's workflow (`assets/workflows/`) or the JSON a skill reads
(`crates/compagnon-cli/src/contract.rs`) runs the evaluation suite,
`compagnon-evals`, on the steps it touches, and pastes the comparison with
the committed baseline (`evals/baseline/`) in its description:

```bash
cargo build --bin compagnon
cargo run -p compagnon-evals -- run --step review --repeat 3 --jobs 4 --out target/evals/mine
cargo run -p compagnon-evals -- compare evals/baseline target/evals/mine --quality-only
```

The quality must stay within the range of the baseline's repetitions. A
pull request meant to make the skills consume less drops `--quality-only`:
its cost must also fall under the baseline's floor. The suite needs Claude
Code installed and signed in, calls a model and costs money — about fifteen
US dollars for the 40 review cases — and runs agents in Claude Code's
sandbox: run it on a machine or in a container you can throw away.
[Reliability of the roles](reliability.md) shows the current baseline;
CONTRIBUTING.md gives every step, option and kind of case.

## Commit messages

[Commit](glossary.md#commit) messages follow
[Conventional Commits](https://www.conventionalcommits.org/), in English:

```text
feat(cli): add --lang to compagnon docs
fix(engine): name the missing requirement when a MODIFIED delta fails
docs: explain inherited git sources
```

Common types are `feat`, `fix`, `docs`, `refactor`, `test` and `chore`; the
scope is the crate or area (`core`, `engine`, `agents`, `cli`, `skills`,
`docs`). Keep the subject short and in the imperative mood. If an agent
co-wrote a commit, add a `Co-Authored-By:` trailer.

A pull request that changes behavior updates the documentation in both
languages, `docs/en/` and `docs/fr/`, or says that the French side needs a
follow-up.

## The full guide

[CONTRIBUTING.md](https://github.com/mairistem/compagnon/blob/main/CONTRIBUTING.md)
covers the rest: writing evaluation cases, the documentation's
conventions, the release process, security reports and the code of
conduct. [Architecture](architecture.md) describes the crates and how they
depend on each other.
