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
the role evaluations of the roles concerned and pastes the output of
`cargo run -p codev-evals -- compare --quality-only` against the committed
baseline, `evals/baseline/`, in its description: each role's recall,
false positives and evidence validity must stay within the range the
baseline's repetitions spanned. A pull request meant to make the skills
consume less pastes the output of `cargo run -p codev-evals -- compare`,
without `--quality-only`: the quality must hold and the cost must fall
under the baseline's floor, each case at its cheapest run. Any other
pull request that changes what a skill does — a file of
`assets/workflows/`, or the JSON a skill reads,
`crates/codev-cli/src/contract.rs` — pastes the output of
`cargo run -p codev-evals -- compare --quality-only`, which still reports
what its runs consume. Run both on the cases of the steps the pull
request touches, with `--step` (repeatable): `review` for the review
steps, `plan.write` for the planning of `/codev-propose`, `plan.update`
for the revisions of `/codev-update`, `code.implement` for the
implementation of `/codev-apply`. Accepted
decision 0008 makes this the rule for every change to the workflow.
Record the candidate with `--repeat 3` at least, and with the baseline's `--jobs`
(4), so that both runs share the prompt cache alike.

The suite needs [Claude Code](https://claude.com/claude-code) installed and
signed in (or `ANTHROPIC_API_KEY` set). It calls a model: a run of the
40 review cases costs about fifteen US dollars, each case capped at 3 by
default; a planning case costs about 1.65 dollars and takes several
minutes, capped at 6 dollars and 1,800 seconds by default, as is an update
case; an implementation case writes, builds and reviews a whole change,
capped at 30 dollars and 5,400 seconds by default.

```bash
cargo build --bin codev
cargo run -p codev-evals -- list                       # the cases
cargo run -p codev-evals -- run --role security --repeat 3 --jobs 4 --out target/evals/mine
cargo run -p codev-evals -- run --step plan.write --repeat 3 --jobs 4 --out target/evals/plan
cargo run -p codev-evals -- run --step plan.update --repeat 3 --jobs 4 --out target/evals/update
cargo run -p codev-evals -- run --step code.implement --repeat 3 --jobs 2 --out target/evals/implement
cargo run -p codev-evals -- calibrate implement-decision-accept-command   # prove a script, no model
cargo run -p codev-evals -- score target/evals/mine    # score again, no model
cargo run -p codev-evals -- compare evals/baseline target/evals/mine --quality-only
```

To measure a build other than `target/debug/codev`, or to keep one fixed
while you keep working, copy it into a folder of its own and pass it with
`--codev <folder>/codev`: the file keeps the name `codev`. The runner
puts that folder first in the search path, and the agent calls `codev`
by name; under another name, the agent would run whatever other `codev`
is installed. `codev-evals run` refuses such a binary.

The agent runs in Claude Code's sandbox, as a user working with it would:
the shell commands Claude Code knows, and `codev`, run without a prompt,
but they can write only in the case's directory and the system's
temporary directory — which holds the other cases' directories too —
reach no host but the Anthropic API, and not read the API key; the file
tools, outside the sandbox, edit only inside the case's directory. Any
other call is denied and counted in the report, and a run that modifies
its repository's `.git/config` is recorded in error. The sandbox bounds
an agent, it is not an isolation: run the suite on a machine or in a
container you can throw away. On macOS the sandbox is built in. On
Linux it needs `bubblewrap` and `socat` (`sudo apt-get install bubblewrap
socat`) and, on Ubuntu 24.04, an AppArmor profile that lets `bwrap` create
user namespaces, as `.github/workflows/evals.yml` installs it; without
them each case stops in error rather than run unconfined.

`run` writes `report.md` and `report.json` in its output directory and
exits with 1 when a role is under a threshold (`--min-recall`,
`--max-false-positives`, `--min-evidence`) or a run stopped. `--repeat N`
runs each case N times; the report gives each measure's median over the
runs, with its range, and what the runs consumed (cost, durations, turns,
tokens, permission denials, calls to the model with their input and
output per call), and where the tokens go: each source — the skill read,
each `codev` subcommand, the files, the searches, the model's own output
— with its share of the cost. The output never decides a comparison: it
carries the model's reasoning, which the quality measures guard. A directory that already holds observations
is resumed: only the missing runs, and those in error, are made. `CODEV_EVAL_MODEL` picks the
model (default: Claude Code's), `CODEV_EVAL_BUDGET_USD` and
`CODEV_EVAL_TIMEOUT_SECS` bound each case, whatever its step. Compare on the same model as the
baseline. Maintainers can also start the **Role evaluations** workflow by
hand; it needs the `ANTHROPIC_API_KEY` repository secret and skips
otherwise.

The review cases run in the `inline` mode by default. `--review-mode
isolated` plays them in the `isolated` mode: each case's
`_codev/config.yaml` gets `review: { mode: isolated }` before the skills
are rendered, the sub-agent tool (`Agent`) is granted, and a case whose
role did not run alone — no `Mode: isolated.` line, a `Fallback:` line,
or no sub-agent started — ends in error. The tokens the session's totals
hold beyond its own calls go to the source `sub-agents`; a run whose
totals fall below its own calls ends in error. `--fixture-config <file>`
appends a YAML mapping to every review case's `_codev/config.yaml`, for a
setting a measure needs; it may not set `review:`, which only
`--review-mode` gives. Planning and update cases ignore both options.
`run.json` records them, and a run resumed with other values stops.
`compare` refuses two runs whose review cases ran in different modes,
and names both runs' appended configurations, saying when they differ.
To measure a change of model, pass `--across-models`: `compare` then
compares two runs whose models differ and names both at the top of its
report. Record both runs the same day, in the same mode:

```bash
cargo run -p codev-evals -- run --step review --review-mode isolated --repeat 3 --jobs 4 --out target/evals/opus
CODEV_EVAL_MODEL=claude-sonnet-5-5 cargo run -p codev-evals -- run --step review --review-mode isolated --repeat 3 --jobs 4 --out target/evals/sonnet
cargo run -p codev-evals -- compare target/evals/opus target/evals/sonnet --across-models --quality-only
```

The documentation's [reliability chapter](docs/en/src/reliability.md)
shows the baseline to readers. It is generated, never edited by hand:
each time `evals/baseline/` changes — recorded or scored again — run
`cargo run -p codev-evals -- docs`, which rewrites
`docs/en/src/reliability.md` and `docs/fr/src/reliability.md`. The tests
fail while the pages differ from the baseline.

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

### Writing a planning case

A planning case replays a request against this repository as it was
before an archived change: its directory holds only `case.yaml`, with
`step: plan.write`, `kind: seeded`, `ref` (the commit the request is
planned against — the parent of the change's merge commit), `request`
(the user's words, in any language, never copied from the archived
proposal: a test refuses more than twelve words in a row of it),
`summary`, and `expected` — `modified`, the requirements the request is
about, `collateral`, those the real change modified in passing, each as
`<capability>::<requirement name>`, and `decisions`, each with its `id`
and `expect` (`follow` or `supersede`). The runner rebuilds the fixture
from `git archive <ref>` and installs the skills of the codev under
test, so the suite needs the full history: a shallow clone fails to load
the cases and names the commit (`git fetch --unshallow`). Loading also
refuses a requirement or a decision that did not exist at `ref`, and a
`ref` whose tree already holds the case.

### Writing an update case

An update case replays an author's request to revise one artifact of an
active change with `/codev-update`. No commit of `main` holds an active
change — pull requests are squash-merged with their archive — so the case
carries the change: its `change/` holds the files of an archived change,
copied as the archive keeps them, which the runner writes to
`_codev/changes/<change>/` on the tree of `ref`, the parent of the commit
that brought the change to `main`. `case.yaml` gives `step: plan.update`,
`kind: seeded`, `ref`, `change` (the change's name), `artifact`
(`proposal`, `specs`, `design` or `tasks`), `request` (the author's words,
in any language), `summary`, and `expected`, whose paths are relative to
the change's folder: `revised`, the files the revision must change;
`kept`, those it must leave byte-identical; `ripple`, those whose
consequence the agent must name without editing them; `contains`, text the
revised files must hold; and `refuse`, for a request the skill must refuse
(it then expects no `revised` path and no `contains`). Loading refuses a
path `change/` does not hold, and a `ref` whose tree already holds the
change or the case.

Before the run, the runner records `codev validate <change> --json` on the
fixture; the prompt names the skill, the change, the artifact and the
request, and says to ask no question, to report a ripple without applying
it, and to modify nothing outside the change's folder. A run is scored
without a model, by checks a reader can redo by hand: each `revised` file
differs from `change/`; each `kept` and `ripple` file is unchanged; each
`ripple` path is named in the agent's messages — every message it wrote,
tool calls and their outputs left out; each `contains` text is in a
revised file, ignoring case and runs of whitespace; `codev validate`
reports no error the fixture did not already have; and a refusal leaves the
change's folder as it was and names `/codev-propose`. A run passes when
every check passes. The report's `plan.update` section gives, per
repetition, the share of runs passing, the share of checks passing and the
runs modifying a file outside the change's folder, and each run's checks;
`compare` fails when the share of runs passing falls under the baseline's
minimum, or the runs modifying a file outside the step rise above its
maximum.

### Writing an implementation case

An implementation case replays the implementation of an archived change
with `/codev-apply`. `case.yaml` gives `step: code.implement`,
`kind: seeded`, `ref` (the parent of the commit that brought the change to
`main`), `merged` (that commit), `change`, `summary`, and `expected`:
`refuse`, for a plan the skill must refuse to implement; `docs`, when the
run must touch both `docs/en/` and `docs/fr/`; and `acceptance`, the names
of the acceptance assertions. Its `change/` holds the archived change's
planning files with every task unchecked and without `review.md`; a task
whose verification needs a tool the sandbox does not hold, such as
`mdbook`, is reworded into one the sandbox can run, and `summary` says so.
A refusing case holds no `tasks.md` and no script.

Next to `case.yaml`, `acceptance.sh` is the hidden check: written from the
scenarios of the change's delta specs, it is run by `bash` in an empty
directory with `CODEV` naming the built binary, under 300 seconds, and
prints one line per assertion, `ok <name>` or `not ok <name>`; an
assertion it does not print `ok` for fails. Before a case is used,
`cargo run -p codev-evals -- calibrate <case>` builds `merged` and `ref`
and must exit with 0: every assertion `ok` on `merged`, its format, lints
and tests passing with today's toolchain, and at least one assertion
failing on `ref`.

The runner lays `change/` on the tree of `ref`, installs the skills,
fetches and builds the dependencies into a Cargo home inside the fixture,
records `codev validate <change> --json`, then runs Claude Code offline in
its sandbox, granted the tools the installed `codev-apply` skill grants.
After the run it formats, lints, tests and builds the fixture itself, then
runs the script. These commands run code the agent wrote outside the
sandbox: each starts with a cleared environment — the search path, the
home directory, the Rust toolchain's own variables, the temporary
directory and the locale, never the API key or a token — and a timeout of
1,200 seconds. Run this step only on your own cases, on a machine where
that is acceptable. A run passes when every task is checked, the four
commands pass, every assertion is `ok`, the review record holds
`code.review`, `codev validate` adds no error, nothing changed under
`_codev/` outside the change, the change is not archived, the documentation
is touched when expected, and the agent read nothing of the repository the
cases come from — its reads and searches are recorded from its tool calls.
The report's `code.implement` section keeps the refusing cases apart;
`compare` fails when the share of runs passing, the share of acceptance
assertions passing or the share of refusals passing falls under the
baseline's minimum.

Changing a term of the vocabulary changes the scores: score the baseline
again (`cargo run -p codev-evals -- score evals/baseline`) and say which
findings it re-classifies.

Do not tune a role and its cases in the same pull request: a case states
what a reviewer should find, independently of the role's wording. When the
scoring rules or the cases change, record the baseline again, locally,
with the Claude Code version `.github/workflows/evals.yml` pins
(`npm install -g @anthropic-ai/claude-code@<that version>`; a test checks
`evals/baseline/run.json` against the pin) — remove `evals/baseline/`, then
`cargo run -p codev-evals -- run --out evals/baseline --repeat 5 --jobs 4`
(about 270 dollars and several hours, planning cases included) — which
the tests check: five runs of every case, none in error. The cases of a
new step — the update cases when `plan.update` came, the implementation
cases with `code.implement` — may be recorded
alone, into the existing baseline, when the other cases and the scoring are
unchanged: with the pinned Claude Code and the codev the baseline names,
`cargo run -p codev-evals -- run --step plan.update --out evals/baseline --repeat 5 --jobs 4`
resumes it with the new cases' runs only (a resume refuses another Claude
Code version), then `cargo run -p codev-evals -- score evals/baseline`
and `cargo run -p codev-evals -- docs`. A changed case or a changed scoring
still requires recording the whole baseline again. Claude Code's
raw output of each run (`claude.jsonl`, `claude.stderr`) stays out of
version control.

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
