## MODIFIED Requirements

### Requirement: The runner runs each case alone, headlessly, with Claude Code

`codev-evals run` SHALL, for each selected case, in a new directory
outside the repository:

- for a review case, create a git repository whose branch `main` holds
  `base/` with codev initialized (configuration and skills,
  non-interactive), and a branch holding `change/` on top of it, both
  committed; a planning case builds its fixture as the
  `planning-evaluation` capability says;
- for a review case, write the change's `change.yaml` so that every reviewing role serving
  the case's step other than the evaluated one is set to `never` — the
  evaluated role reviews alone;
- run Claude Code non-interactively in that directory, reading the stream
  of events it prints rather than its final document alone, with a
  prompt that names the skill step to run and the evaluation mode: at
  `plan.challenge` and `code.review`, write the findings to the review
  record and leave every finding `open`, without the author's answer and
  without modifying any other file; at `plan.trace`, fix the gaps in the
  plan as the step says;
- run it as a user who works with Claude Code's sandbox would: the
  shell commands Claude Code approves in its sandbox, and `codev`, which
  the skills grant, run without a prompt, in a sandbox that lets them
  write only in the case's directory and the temporary directory and
  reach no host but the Anthropic API; file edits inside the case's
  directory are accepted; any other call — a write elsewhere, a network
  tool, a program neither approves — is denied and counted;
  the runner SHALL check, before any case runs, that the sandbox can
  run on the machine — on Linux, `bubblewrap` and `socat` — and exit with
  code 2 naming what is missing, rather than run without it;
- record what it observed: the review record as `codev review --json`
  reports it, the files the case checks, the files the run modified
  outside what the step allows, what the run consumed as Claude Code
  reports it — the cost, the duration and the API duration, the number
  of turns, the input tokens read without the cache, written to the
  cache and read from the cache, and the output tokens, each summed over
  every model the run used, the models it used, the number of
  permission denials, the number of tool calls, and how many of them
  came before the run first wrote a file under `_codev/changes/` (its
  orientation; absent when it wrote none), and every call to the model —
  or the error that stopped it. For each call, the observation SHALL hold
  the tokens it wrote to the cache, read from it, read without it and
  produced, the tool calls it asked for, and where the tokens it wrote
  came from: each tool result returned since the previous call, and the
  previous call's own output, gets a share of the call's cache writes in
  proportion to its length, under its source — `skill` (a file under
  `.claude/skills/`), `codev <subcommand>` (a shell command that runs
  codev), `file` (another file read), `search` (a search over files),
  `shell` (another shell command), `edit` (the result of a write or an
  edit), `output` (the model's previous output) — and the first call's
  writes go to `prompt` (Claude Code's own prompt and the request). A measure Claude
  Code does not report is recorded as absent, never as zero.

With `--repeat N` (default 1, at least 1), the runner SHALL run each
selected case N times, each repetition in a directory of its own, and
record each repetition's observation separately. A value of `--repeat`
that is not a whole number of at least 1 SHALL exit with code 2 before
any case runs, naming the flag.

The model SHALL be the one named by the environment variable
`CODEV_EVAL_MODEL`, else Claude Code's default. Each case SHALL run under
a budget (`CODEV_EVAL_BUDGET_USD`, default 3 for a review case) and a
timeout (`CODEV_EVAL_TIMEOUT_SECS`, default 600 for a review case); a
planning case's defaults are given by the `planning-evaluation`
capability. Before any case runs, the
runner SHALL check that `claude`, `git` and the `codev` binary can be
executed, and exit with code 2 naming the missing one otherwise; and that
the `codev` binary's file is named `codev` (`codev.exe` on Windows),
exiting with code 2 naming the file otherwise: the agent calls codev by
name, through the search path whose first folder is the binary's, so a
binary under another name would leave the agent running whatever other
`codev` the search path holds. A case
that fails or times out is recorded as an error; the others still run —
except when Claude Code reports that the account's usage limit is reached:
the runner then records that run as an error, starts no other run, lets
the runs already started end, and exits with 1 saying the limit was
reached, so that running it again on the same directory resumes where it
stopped.

#### Scenario: Only the evaluated role reviews

- **GIVEN** a `code.review` case evaluating `security`, at a step where
  `newcomer` and `security` serve
- **WHEN** the runner prepares the change
- **THEN** its `change.yaml` sets `newcomer` to `never` and leaves
  `security` as configured

#### Scenario: The prompt names the step and the evaluation mode

- **GIVEN** a `plan.challenge` case on the change `add-sessions`
- **WHEN** the runner builds its prompt
- **THEN** the prompt names the `codev-propose` skill, its
  `plan.challenge` step, the change `add-sessions`, and says to leave
  every finding `open` and to modify no file but the review record

#### Scenario: The model and the budget come from the environment

- **GIVEN** `CODEV_EVAL_MODEL=claude-sonnet-4-5` and
  `CODEV_EVAL_BUDGET_USD=1.5`
- **WHEN** the runner builds the Claude Code command
- **THEN** it passes that model and that budget; without the variables,
  it passes no model and a budget of 3

#### Scenario: A run that strays outside the step

- **GIVEN** a `code.review` case whose run modified `src/users.ts` and
  wrote `review.md`
- **WHEN** its observation is recorded
- **THEN** it lists `src/users.ts` as modified outside the step, not
  `review.md`, and the report shows it for that case

#### Scenario: Claude Code is not installed

- **GIVEN** a `claude` command that cannot be executed
- **WHEN** the user runs `codev-evals run`
- **THEN** it exits with code 2 before setting up any case, and the
  message names `claude`

#### Scenario: A compound command is not denied

- **GIVEN** a case whose agent runs `cd src && cat users.ts | head -20`
- **WHEN** the runner runs the case
- **THEN** the command runs, and the observation records 0 permission
  denials

#### Scenario: A write outside the case and a network call are denied

- **GIVEN** a case whose agent runs `echo x > $HOME/probe.txt`, then
  `curl https://example.com`, then writes `$HOME/probe2.txt` with its
  file tool
- **WHEN** the runner runs the case
- **THEN** none of the three takes effect, and the observation records
  3 permission denials

#### Scenario: A machine without the sandbox

- **GIVEN** a Linux machine where `socat` is not installed
- **WHEN** the user runs `codev-evals run`
- **THEN** it exits with code 2 before setting up any case, and the
  message names the sandbox and `socat`

#### Scenario: What a run consumed is recorded

- **GIVEN** a run for which Claude Code reports 9 turns, and per model:
  `claude-opus-5-5` with 14 uncached input tokens, 29 349 written to the
  cache, 304 094 read from it and 2 713 output tokens, and
  `claude-haiku-4-5-20251001` with 1 173 uncached input tokens and 15
  output tokens
- **WHEN** its observation is recorded
- **THEN** it holds 9 turns, 1 187 uncached input tokens, 29 349 written
  to the cache, 304 094 read from it and 2 728 output tokens, and both
  models, next to the cost and the durations

#### Scenario: A run whose output reports no usage

- **GIVEN** a run that times out before Claude Code prints its result
- **WHEN** its observation is recorded
- **THEN** its tokens and turns are absent, not zero, and the case is in
  error

#### Scenario: The account's usage limit is reached

- **GIVEN** a run of 40 cases, one job at a time, whose third run ends
  with Claude Code reporting "You've hit your session limit"
- **WHEN** the runner records it
- **THEN** that run is recorded as an error, no fourth run starts, and the
  runner exits with 1 saying the usage limit was reached
- **AND** running the same command again runs that case and the 37 others

#### Scenario: Each case repeated

- **GIVEN** two selected cases
- **WHEN** the user runs `codev-evals run --repeat 3`
- **THEN** six runs are made, and each case holds three observations

#### Scenario: An invalid repetition count

- **WHEN** the user runs `codev-evals run --repeat 0`
- **THEN** it exits with code 2 before setting up any case, and the
  message names `--repeat`

#### Scenario: Tool calls and orientation are recorded

- **GIVEN** a run whose events hold 27 tool calls, the twelfth of which
  writes `_codev/changes/add-search/proposal.md`
- **WHEN** its observation is recorded
- **THEN** it holds 27 tool calls and an orientation of 11

#### Scenario: Where a call's tokens come from

- **GIVEN** a call that wrote 6,000 tokens to the cache after two tool
  results: `codev roles --at code.review --json`, 9,000 characters long,
  and a read of `src/users.ts`, 3,000 characters long, the previous call
  having produced no text
- **WHEN** its observation is recorded
- **THEN** the call attributes 4,500 tokens to `codev roles` and 1,500 to
  `file`

#### Scenario: A run that writes nothing under the changes

- **GIVEN** a run whose agent writes no file under `_codev/changes/`
- **WHEN** its observation is recorded
- **THEN** its orientation is absent, not zero

#### Scenario: A codev binary under another name

- **GIVEN** `--codev /tmp/frozen/codev-candidate`
- **WHEN** the user runs `codev-evals run`
- **THEN** it exits with code 2 before any case runs, naming
  `codev-candidate` and saying the file must be named `codev`
