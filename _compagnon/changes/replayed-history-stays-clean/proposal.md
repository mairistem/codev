# Proposal: A replayed history stays as clean as it was

## Why

The evaluation of `plain-words-in-skills` (its `evals.md`, task 4.1) found
0 of 12 implementation runs passing, never on the code the change asked
for: the `format` check failed on code no run touched, and the last task
(`validate --strict`) stopped on the seals of decisions 0001–0006.
`compagnon-evals calibrate implement-decision-accept-command`, with no
model, fails `format` on the case's merged commit too, while
`cargo fmt --all --check` passes on that same commit extracted as is.

The cause is the fixture's rename: a tree from before the product's
rename is brought through it (`bring_through_rename` in
`evals/src/runner.rs`), every `codev` becoming `compagnon`. The longer
name pushes lines past rustfmt's width (`crates/compagnon-agents/src/claude.rs`:
`"/p/.claude/skills/compagnon-propose/SKILL.md"`), and it rewrites the
bodies of accepted decisions, whose hashes in
`_compagnon/decisions/seal.yaml` no longer match
(`decision_seal_mismatch` on 0001–0006). Every implementation case, and
every planning, update or merge case replayed from before the rename,
starts from a tree its own checks reject.

## What Changes

- After renaming a replayed tree, the runner re-seals every decision
  whose seal matched its body before the rename, so a seal broken by the
  rename alone holds again; a seal that did not match before stays broken.
- After renaming a replayed tree that is a Rust workspace, the runner
  formats with rustfmt the Rust files the rename changed, and only them,
  before the fixture's base commit and before `calibrate` checks the
  merged commit, so that `format` measures only what the skill changed.
- `calibrate` on the implementation cases passes `format` again on the
  merged commits; the baseline is not re-recorded by this change.

## Capabilities

### New Capabilities

### Modified Capabilities

- implementation-evaluation

### Removed Capabilities

## Impact

- `evals/src/runner.rs` (`bring_through_rename`), `evals/src/implement_run.rs`
  (`extract`, used by `calibrate`), their tests in `evals/tests/fixture.rs`.
- `compagnon_core::decisions::seal` is used as it is (`body_hash`); no
  change to the CLI or the skills, so decision 0008 asks for no paid run.
- No documentation changes: the chapters describe the checks, which stay
  the same.
- The next paid evaluation of `code.implement` measures the skill, not
  the rename; the baseline recorded before stays as it is until then.

## Domains

- [ ] security
- [ ] personal-data — the match is the word "phone" in the home page's example (`docs/site/index.html`), from another change on the branch
- [ ] user-interface — the matches are the documentation and its stylesheet, changed by other changes on the branch; this change touches no interface
- [ ] performance — the match is « cache » in French text of `evals/src/reliability.rs`, from another change on the branch; formatting a fixture once adds about a second per case
- [ ] operations
- [ ] migration

## Release note

None
