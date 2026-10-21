# Review record: implementation-evaluation

<!-- codev-review: 1 -->

## plan.challenge

Mode: isolated.

### skeptic

#### R-skeptic-1 Replayed merged plans hold tasks that cannot be verified in the offline sandbox

```yaml
severity: medium
confidence: high
location: "design.md#Decision: An implementation case replays an archived change, through the update cases' fixture"
evidence: |
  The plan replayed is the plan as merged. It may hold tasks added during
  the review, so it is the plan the change actually delivered. A run is
  asked to implement that plan, not to rediscover it.
recommendation: Reword in the case's `change/` every task verification the sandbox cannot run (such as `mdbook` for forward-compatible-config's books), say so in the case's summary, and name it in tasks 5.1-5.3.
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    ref: "design.md, the spec's case requirement (MAY differ only by reworded verifications), tasks 5.1 and 2.6 (no task left unchecked for a blocker)"
  - status: fixed
    by: skeptic
```

#### R-skeptic-2 The acceptance script and the merged solution are hidden from the fixture's tree, not from the agent

```yaml
severity: medium
confidence: low
location: "design.md#Decision: Quality is judged by a hidden black-box script against the built binary"
evidence: |
  the script: it lives in `evals/cases/`, which the fixture does not hold.
  That directory is absent at `ref`, and the loader refuses a case whose
recommendation: State what the loader guarantees (only `evals/cases/<case id>/` is absent), and either deny reads of the operator's repository or detect them; have the smoke run check the transcript reads nothing outside the case's directory.
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    ref: "design.md (the sentence corrected; reads outside the case's directory recorded and checked), spec (a scoring check and the scenario 'A run that reads the merged solution'), tasks 2.4, 2.6 and 4.1"
  - status: fixed
    by: skeptic
```

#### R-skeptic-3 Four cases at five repetitions may not give the resolution the stated goal needs

```yaml
severity: medium
confidence: low
location: "design.md#Goals / Non-Goals"
evidence: |
  - A measure of `/codev-apply` precise enough to compare two models, or
    two versions of the skill, with `compare`.
recommendation: State the smallest regression compare detects, keep the refusal out of the pass and acceptance shares, and add a test naming the detectable regression.
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    ref: "design.md#Risks / Trade-offs (the resolution stated), spec (refusals kept out of the shares, judged on their own; scenario 'One more failed implementation in most repetitions'), tasks 4.1 and 4.3"
  - status: fixed
    by: skeptic
```

### security

#### R-security-1 Post-run checks and the acceptance script run agent-written code outside the sandbox without dropping the API key

```yaml
severity: medium
confidence: low
location: "design.md#Decision: The runner runs the checks itself, after the run"
evidence: |
  The agent's own report of a passing test is not a measure. After the
recommendation: Start every runner command on the fixture without the API key, assert it in a test, and warn in CONTRIBUTING about the other secrets of the operator's shell.
history:
  - status: open
    by: security
  - status: fixed
    by: architect
    ref: "design.md#Decision: The runner's own commands run without secrets and under a timeout (a cleared environment, stricter than tool_command), spec (scenario 'The checks never see the API key'), tasks 2.2 and 6.1"
  - status: fixed
    by: security
```

### ops

#### R-ops-1 The runner's post-run checks on agent-written code have no timeout

```yaml
severity: medium
confidence: high
location: "design.md#Decision: The runner runs the checks itself, after the run"
evidence: |
  same Cargo home. It records each outcome with the last 40 lines of its
  output, then runs `acceptance.sh` against `target/debug/codev`. A check
recommendation: Put a timeout on every runner command (preparation, checks, calibrate), in its own process group; a check that times out fails and the run is still scored; add a test.
history:
  - status: open
    by: ops
  - status: fixed
    by: architect
    ref: "design.md#Decision: The runner's own commands run without secrets and under a timeout, spec (1,200 seconds each, scenario 'A test that never ends'), task 2.2 (a_command_that_times_out_fails_its_check)"
  - status: fixed
    by: ops
```

#### R-ops-2 Build outputs are removed only after an observation is written

```yaml
severity: low
confidence: low
location: "design.md#Decision: An implementation run's build outputs are removed once recorded"
evidence: |
  `target/` and `.cargo-home/` take about a gigabyte per run. Once the
recommendation: Remove `target/` and `.cargo-home/` also when the run ends in an error.
history:
  - status: open
    by: ops
  - status: fixed
    by: architect
    ref: "design.md (removed whether or not the run produced an observation), task 2.5"
  - status: fixed
    by: ops
```

## code.review

Mode: isolated.
Summarized: CONTRIBUTING.fr.md — over the material's size limit.
Summarized: CONTRIBUTING.md — over the material's size limit.
Summarized: evals/src/case.rs — over the material's size limit.
Summarized: evals/src/compare.rs — over the material's size limit.
Summarized: evals/src/implement.rs — over the material's size limit.
Summarized: evals/src/implement_run.rs — over the material's size limit.
Summarized: evals/src/main.rs — over the material's size limit.
Summarized: evals/src/report.rs — over the material's size limit.
Summarized: evals/src/runner.rs — over the material's size limit.
Summarized: evals/src/score.rs — over the material's size limit.
Summarized: evals/src/update.rs — over the material's size limit.
Summarized: evals/tests/suite.rs — over the material's size limit.

### security

#### R-security-2 The tamper check compares only `.git/config`, so a run can still change the git configuration that the unsandboxed `git status` reads

```yaml
severity: medium
confidence: low
location: "evals/src/runner.rs:1726-1738"
evidence: |
      if observed.error.is_none() {
          let after = fs::read(work_dir.join(".git").join("config")).ok();
          observed.error = config_tampered(&config_before, after.as_deref());
      }
recommendation: Treat the run as tampered when anything under `.git/` that changes where git reads its configuration has changed (`commondir`, `config.worktree`, `info/attributes`), for instance with a snapshot of `.git/` outside `objects/`, `refs/`, `logs/` and `index`; start the runner's git and codev calls with a cleared environment too; add a test where a run writes `.git/commondir` and is refused.
history:
  - status: open
    by: security
  - status: fixed
    by: developer
    ref: "evals/src/runner.rs git_settings / git_settings_tampered before any git call on the fixture (test a_run_that_writes_a_commondir_is_refused); tool_command clears the environment"
  - status: fixed
    by: security
```

### ops

#### R-ops-3 A failed `git status` after an implementation run is read as "nothing strayed"

```yaml
severity: low
confidence: low
location: "evals/src/runner.rs:1733-1738"
evidence: |
      let status = git(
          tools,
          work_dir,
          &["status", "--porcelain", "-z", "--untracked-files=all"],
      )
      .unwrap_or_default();
recommendation: Put the `git status` error in `observed.error`, as the `codev validate` failures are, so the run shows as stopped and is not scored.
history:
  - status: open
    by: ops
  - status: fixed
    by: developer
    ref: "evals/src/runner.rs observe_change: a failed git status is an error"
  - status: fixed
    by: ops
```

### newcomer

#### R-newcomer-1 `calibrate` picks the build by a magic index and branches on a side's string name inside an immediately invoked closure

```yaml
severity: medium
confidence: high
location: "evals/src/implement_run.rs:544-569"
evidence: |
      for (side, commit) in [("merged", merged), ("before", before)] {
          let dir = work.join(side);
          let side_log = log.join(side);
          fs::create_dir_all(&side_log).map_err(|e| e.to_string())?;
          extract(repo, commit, &dir)?;
          let result = (|| {
              prepare(&dir, &side_log)?;
recommendation: Extract a function that builds a tree, runs the given commands and the script, called once with every check for the merged commit and once with a named `BUILD` command for the commit before; remove the build outputs after each call; say why the commit before needs only the binary.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "evals/src/implement_run.rs: a named BUILD command, calibrate calls build_and_accept once per commit, with the reason the commit before needs only the binary"
  - status: fixed
    by: newcomer
```

#### R-newcomer-2 `run_command` returns an anonymous `(bool, bool, String)` that every caller must decode by position

```yaml
severity: low
confidence: high
location: "evals/src/implement_run.rs:270-277"
evidence: |
  ) -> Result<(bool, bool, String), String> {
recommendation: Have `run_command` take the check's name and return `Result<CommandCheck, String>`.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "evals/src/implement_run.rs: run_command takes the check name and returns a CommandCheck"
  - status: fixed
    by: newcomer
```

#### R-newcomer-3 `run_implementation` loops over a two-item table to tell `validate` from `review`

```yaml
severity: medium
confidence: high
location: "evals/src/runner.rs:1741-1760"
evidence: |
      for (args, key) in [
          (vec!["validate", case.change.as_str(), "--json"], "validate"),
recommendation: Write two plain `codev_json` calls; split `run_implementation` into steps; use the module's real name instead of the `build` alias; write `if let (true, Some(script))` more plainly.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "evals/src/runner.rs: run_implementation split into run_agent, observe_change and check_implementation; two plain codev calls; the module named implement_run"
  - status: fixed
    by: newcomer
```

#### R-newcomer-4 `check_update_history` now also checks implementation cases but keeps its update-only name

```yaml
severity: low
confidence: high
location: "evals/src/case.rs:557-560"
evidence: |
  fn check_update_history(
recommendation: Rename it to something like `check_replayed_change_history`.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "evals/src/case.rs: renamed check_replayed_change_history"
  - status: fixed
    by: newcomer
```

#### R-newcomer-5 `score_implement` holds two unrelated scorings in one `if/else`, and one check's condition does not match its label

```yaml
severity: low
confidence: high
location: "evals/src/implement.rs:147-150"
evidence: |
          check(
              "the change is still in `_codev/changes/`".to_string(),
              file("proposal.md").is_some(),
          );
recommendation: Say why `proposal.md` stands for the change not being archived, and split the function into the refusal's scoring and the implementation's.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "evals/src/implement.rs: score_refusal and score_implementation; a comment on why proposal.md stands for the change not archived"
  - status: fixed
    by: newcomer
```
