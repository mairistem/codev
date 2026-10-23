## MODIFIED Requirements

### Requirement: The runner replays the change's implementation with the skill under test, offline

For an implementation case, `codev-evals run` SHALL build the case's
directory from the tree at `ref`, write `change/` into
`_codev/changes/<change>/`, append the YAML of `--fixture-config`, when
given, to its `_codev/config.yaml`, install the skills of the codev under
test as for a planning case, and commit. `--review-mode` does not apply:
the case keeps the review mode its configuration holds. It SHALL then, before the agent
starts, fetch the project's dependencies into a Cargo home inside the
case's directory, which git ignores, and build the workspace and its
tests, so that the agent's builds need no network; a failure there SHALL
record the run as an error. Every command the runner starts itself on
the case's directory — this preparation, the checks below, the build of
the binary, the acceptance script, and its calls to `git` and `codev` —
SHALL run with an environment
cleared of everything but what the toolchain needs to run — the search
path, the home directory, the Rust toolchain's own variables, the
temporary directory and the locale — so that code the agent wrote never
sees the API key or any other secret of the operator's environment; and
under a timeout of 1,200 seconds each, the script's own aside, stopping
every process it started. A check that times out fails, its output
saying so; a preparation that times out records the run as an error.
Claude Code SHALL run with that Cargo home and Cargo set offline, in the
sandbox every case runs in, granted the tools the installed
`codev-apply` skill grants in its `allowed-tools` — as a user who
invokes the skill, since the agent reads the skill file instead —, with
a prompt
that names the `codev-apply` skill and the change, and the evaluation
mode: ask no question — make the most reasonable assumption —, carry the
skill through its review, and do not archive or sync. Unless
`CODEV_EVAL_BUDGET_USD` and `CODEV_EVAL_TIMEOUT_SECS` say otherwise, an
implementation case SHALL run under a budget of 30 USD and a timeout of
5,400 seconds. `--step code.implement` SHALL select the implementation
cases; `--step review` does not.

Before the run, the runner SHALL record what `codev validate <change>
--json` reports on the fixture. After a run that did not stop on an
error, the runner SHALL itself run, in the case's directory, the
project's checks — `cargo fmt --all --check`, `cargo clippy --workspace
--all-targets -- -D warnings` and `cargo test --workspace`, offline —,
build the `codev` binary, and run the acceptance script against it. The
observation SHALL hold, next to what every case records, each check's
outcome with the end of its output, each assertion's outcome, the files
of the change's folder after the run, the paths the run changed, created
or removed, what `codev validate <change> --json` and `codev review
--change <change> --json` report after the run, and the text of every
message the agent wrote, and the paths of the files the agent read or
searched outside the case's directory. A file changed under `_codev/` outside the
change's folder, and a `corrections.md` written in it, SHALL be recorded
as modified outside the step. A run whose agent changed what git reads
under `.git/` — its configuration, `commondir`, or a file of `info/`
such as the attributes — SHALL be recorded as an error naming the files;
what git writes as it works, such as the objects, the refs, the index or
`AUTO_MERGE`, is not a change of what it reads, before the runner calls `git` on it: git would
run outside the sandbox what such a file sets.

#### Scenario: A run that changes what git reads

- **GIVEN** an implementation run whose agent writes `.git/commondir`
- **WHEN** the run ends
- **THEN** it is recorded as an error naming `commondir`, and `git status`
  is not run on its directory

#### Scenario: The prompt of an implementation case

- **GIVEN** an implementation case for the change `decision-accept-command`
- **WHEN** the runner builds its prompt
- **THEN** it names `codev-apply` and the change, and says to ask no
  question, to carry the skill through its review, and not to archive or
  sync

#### Scenario: The tools of an implementation run

- **GIVEN** a fixture whose `codev-apply` skill grants `Bash(codev:*),
  Read, Write, Edit, Glob, Grep, Bash, Agent`
- **WHEN** the runner builds the Claude Code command of the run
- **THEN** it grants those tools, so that `cargo test` runs in the
  sandbox without a prompt

#### Scenario: The budget of an implementation case

- **GIVEN** no `CODEV_EVAL_BUDGET_USD` nor `CODEV_EVAL_TIMEOUT_SECS`
- **WHEN** the runner builds the Claude Code command of an
  implementation case
- **THEN** it passes a budget of 30 and waits up to 5,400 seconds

#### Scenario: Only the implementation cases

- **GIVEN** a suite of review, planning and implementation cases
- **WHEN** the user runs `codev-evals run --step code.implement`
- **THEN** only the implementation cases run

#### Scenario: A test that never ends

- **GIVEN** an implementation run whose agent wrote a test that loops
- **WHEN** the runner runs `cargo test --workspace` after the run
- **THEN** the test check fails after 1,200 seconds, saying it timed out,
  and the run is still scored

#### Scenario: The checks never see the API key

- **GIVEN** a runner started with `ANTHROPIC_API_KEY` in its environment
- **WHEN** it runs the checks and the acceptance script of a run
- **THEN** none of them has `ANTHROPIC_API_KEY` in its environment

#### Scenario: A run that syncs its change

- **GIVEN** an implementation run that writes `_codev/specs/decisions/spec.md`
- **WHEN** its observation is recorded
- **THEN** it lists that file as modified outside the step

#### Scenario: An implementation run with the developer on Sonnet

- **GIVEN** `--fixture-config` with `roles: { developer: { model: sonnet } }`
- **WHEN** the runner builds an implementation case's fixture
- **THEN** its `_codev/config.yaml` ends with that configuration before
  the skills are installed, so the `codev-apply` skill grants the
  sub-agent tool
