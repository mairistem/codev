# Design: Evaluate the implementation of `/codev-apply`

## Context

codev-evals has two families of cases. Review cases build a small
fixture from `base/` and `change/`. Planning (`plan.write`) and update
(`plan.update`) cases replay this repository's history:
`Step::replays_history` sends them to `setup_plan`, which extracts the
tree at `ref` with `git archive`, lays an update case's `change/` into
`_codev/changes/<change>/`, commits, installs the skills of the codev
under test with `codev update --force`, and commits again. `run_case`
then runs Claude Code in the sandbox of `SANDBOX_SETTINGS`. Each case's
directory and the temporary directory are writable; the only reachable
host is the Anthropic API. Scoring is pure: `update.rs` turns an
`Observed` into checks, without a model. Decision 0008 requires each
change to the workflow to be measured this way, and `compare` gives the
verdict.

An implementation run differs in three ways. The agent must build and
test Rust code. Its result is code, which a text comparison cannot judge.
And it takes many times longer than a planning run.

## Goals / Non-Goals

**Goals**

- A measure of `/codev-apply` precise enough to compare two models, or
  two versions of the skill, with `compare`.
- Checks a reader can redo by hand on the run's directory.

**Non-Goals**

- Judging the code's style or design: the skill's own `code.review` step
  does that, and its record is checked only for being there.
- Running implementation cases in CI: they need a model, like the
  others, and run on demand.
- Projects other than this repository, and languages other than Rust.

## Decisions

### Decision: An implementation case replays an archived change, through the update cases' fixture

An implementation case is a `plan.update`-like case: `ref` is the commit
before the pull request, and `change/` holds the archived change's
planning files from `merged`: `proposal.md`, the delta specs,
`design.md`, and `tasks.md` with every box unchecked. Its `review.md` is
left out. `code.implement` joins `replays_history`, so `setup_plan` builds
it unchanged. Then a step of its own prepares the build.

The plan replayed is the plan as merged. It may hold tasks added during
the review, so it is the plan the change actually delivered. A run is
asked to implement that plan, not to rediscover it. A task whose
verification needs a tool the sandbox does not hold would make every
run stop on a blocker, whatever the model, and the measure would show
the environment instead of the implementation. One example is
`forward-compatible-config`'s "both books building", which needs
`mdbook`. Such a verification is reworded in the case's `change/` into
one the sandbox can run, such as `codev validate` or a test, and the
case's `summary` says so. The smoke run shows whether one was missed.

**Alternatives considered**: writing synthetic changes on a small
fixture project, as the review cases do. Rejected: the measure must
reflect the work `/codev-apply` does on a real codebase, with its specs,
conventions and tests. A change small enough for a synthetic fixture
would not separate two models.

### Decision: Quality is judged by a hidden black-box script against the built binary

Each case holds an `acceptance.sh`, written from the scenarios of the
change's delta specs. It drives `$CODEV` in an empty directory, as a user
would, and prints `ok <name>` / `not ok <name>`. The fixture does not
hold the script: the case's own folder, `evals/cases/<case id>/`, is
absent at `ref`, and the loader refuses a case whose `ref` holds it. The
operator's repository does hold the script and the merged commit, and
the sandbox limits the network, not reads. So the runner records every
file the agent read or searched outside the case's directory, taken from
its tool calls, and a run that touched the repository the cases come
from fails a check of its own. That is detection, not prevention. It is
enough for an evaluation the maintainers run on their own cases, where
cheating can only come from the model, and the record shows it.

`codev-evals calibrate` proves the script on the real change: every
assertion passes on `merged` and at least one fails on `ref`. It also
proves that the project's checks pass on `merged` with the installed
toolchain. A newer Clippy can reject an older commit, as `7ddb31e`
showed, and the run would then fail a check no agent could pass.

**Alternatives considered**:

- Replaying the pull request's own tests: they name internal functions
  and modules that a correct implementation is free to name otherwise,
  so a valid run would fail them.
- A model as judge: not deterministic, and it would make every
  comparison depend on a second model. Decision 0008 asks for measures a
  reader can redo.
- Comparing the diff with the merged one: two correct implementations
  rarely share lines.

### Decision: The agent builds offline, from a Cargo home prepared in the case's directory

Before Claude Code starts, the runner sets `CARGO_HOME` to `.cargo-home/`
inside the case's directory, which `.git/info/exclude` hides from
`git status`. It then runs `cargo fetch --locked` and `cargo build
--workspace --all-targets --locked`, outside the sandbox. Claude Code
inherits `CARGO_HOME` and `CARGO_NET_OFFLINE=true`. Its builds read the
fetched crates, reuse the built dependencies in `target/`, and write
only inside the case's directory, which the sandbox allows.

**Alternatives considered**:

- Allowing `crates.io` and `static.crates.io` in the sandbox's network:
  this widens what the agent can reach, for every step, beyond the
  Anthropic API that role-evaluation allows.
- Sharing the operator's `~/.cargo`: the sandbox forbids writing outside
  the case's directory, and Cargo writes its lock and its extracted
  sources there.

### Decision: The runner's own commands run without secrets and under a timeout

The preparation, the checks, the build of the binary and the acceptance
script all run code the agent wrote or may have changed: `build.rs`,
proc macros, tests, the binary itself. They run outside the sandbox, so
each one starts with a cleared environment that keeps only `PATH`,
`HOME`, the `RUSTUP_*` and `CARGO_*` variables the runner sets, `TMPDIR`
and `LANG`. The API key and any other secret of the operator's shell
are therefore out of reach. The existing `tool_command` only removes the
API key, which is enough for `git` and `codev` but not for code an agent
wrote.

Each command runs in its own process group, as `run_claude` does, under
1,200 seconds. A test the agent wrote that never ends then fails its
check instead of blocking a job forever.

The runner's own `git` and `codev` calls (`tool_command`) get the same
cleared environment, at every step. Before calling `git` on an
implementation fixture, the runner compares what git reads under `.git/`
— `config`, `config.worktree`, `commondir` and `info/` — with what it
held before the agent ran. The files git writes as it works, such as
`AUTO_MERGE` after a stash, are left out: the first baseline runs showed
that an agent using git changes them. A run that changed it, such as a
`commondir` pointing to a configuration with a filter driver, is an
error: `git status` would otherwise run that filter outside the sandbox.

**Alternatives considered**: reusing `tool_command` as it is. Rejected:
it leaves every other variable of the operator's environment in place.
Comparing `.git/config` alone, as the review steps do: a `commondir` or
an attribute file changes what git runs without touching it.

### Decision: The runner runs the checks itself, after the run

The agent's own report of a passing test is not a measure. After the
run, the runner runs `cargo fmt --all --check`, `cargo clippy
--workspace --all-targets --offline -- -D warnings`, `cargo test
--workspace --offline` and `cargo build -p codev-cli --offline` with the
same Cargo home. It records each outcome with the last 40 lines of its
output, then runs `acceptance.sh` against `target/debug/codev`. A check
that cannot run, such as a build that fails, fails its own check and
every acceptance assertion; the run is still scored. Only a failure of
Claude Code itself, a timeout or the budget make the run an error,
rerun as for the other steps.

**Alternatives considered**: reading the agent's last messages for test
results. Rejected for the reason above.

### Decision: The run carries the whole skill, its review included, in the fixture's own review mode

The prompt asks for the whole of `/codev-apply`: the tasks, then the
`code.review` step. The run is granted the tools the installed skill's
`allowed-tools` grants a user who invokes it, the shell and the
sub-agent tool included: the agent reads the skill file instead of
invoking it, and the review cases' grant, `codev` alone, would leave
every `cargo` command waiting for an approval no one gives, as the
first smoke run showed. It must ask no question and must not archive or
sync. The review is part of what a developer pays for and of what may
change with the model. Its mode is the one the fixture's
`_codev/config.yaml` holds at `ref`, so a case keeps the review of its
own time. No role has a model by default (decision 0011), so the
session's model, chosen with `CODEV_EVAL_MODEL`, does all the work.
That is what comparing Opus and Sonnet needs.

**Alternatives considered**: stopping before `code.review`. Rejected:
the cost measured would not be what a change costs, and the review step
already has its own cases.

### Decision: An implementation run gets 30 USD and 5,400 seconds by default

A planning run uses about 2 USD and 10 minutes out of its 6 USD and
1,800 seconds. Implementing the chosen changes means writing about a
thousand lines, and several builds and test runs. The defaults leave
room for the slowest model. A run that hits them is an error, as at
every step. `CODEV_EVAL_BUDGET_USD` and `CODEV_EVAL_TIMEOUT_SECS` still
override them.

**Alternatives considered**: no budget. Rejected: one stuck run could
spend without bound.

### Decision: An implementation run's build outputs are removed once recorded

`target/` and `.cargo-home/` take about a gigabyte per run. Once the
run ends, the runner removes both from the case's directory, whether
the run produced an observation or ended in an error. The code and the
plan stay, so that a run can be read again.

## Risks / Trade-offs

- [Four cases at five repetitions are a coarse measure.] → The refusal
  is kept out of the pass and acceptance shares. With three replays, the
  pass share of a repetition is 0, 1/3, 2/3 or 1. A candidate that fails
  one more replay than the baseline's worst repetition, in at least three
  repetitions out of five, falls under the baseline's minimum and fails.
  The acceptance share counts every assertion of the three scripts, a
  finer grain that also catches a partial implementation. A smaller
  regression needs more cases. The suite may grow, case by case, as
  CONTRIBUTING allows.

- [The runner builds and runs, outside the sandbox, code an agent wrote:
  `build.rs`, proc macros and tests.] → The cases come only from this
  repository's history and are played on demand by a maintainer, who
  takes the same risk when testing a branch the agent wrote. The agent
  itself ran in the sandbox, so it could reach no host but the Anthropic
  API to fetch anything. The runner's own commands get no secret (see
  the decision above). CONTRIBUTING says so where it explains how to run
  the step.
- [The dependency fetch reaches the network before each run.] → It runs
  with `--locked`, so it fetches only what `Cargo.lock` at `ref` pins,
  verified by Cargo's checksums.
- [Cargo or rustup writes outside the case's directory inside the
  sandbox, such as a toolchain update or a lock under `~/.rustup`.] →
  A smoke run of one case (task 2.5) checks, before any baseline, that
  the agent's `cargo test` succeeds offline and records no permission
  denial.
- [An acceptance script tests what the agent could not know.] → The
  script follows the delta specs' scenarios only, and `calibrate` shows
  the merged change passes it.
- [Five runs of four cases are long and costly, on two models.] → The
  runner resumes after a usage limit, and the jobs run in parallel, each
  in its own directory (`unique-eval-workdir`).

## Open Questions

- Whether Sonnet holds the quality is what the baseline and its
  comparison will tell. That decision belongs to a later change.
