# Design: ship `/codev-apply`

## Context

See `proposal.md` for the motivation. Two workflows already exist —
`propose` and `explore` — each is a pair {entry in the `CATALOG`, markdown
file under `assets/workflows/`}. This design reuses the same mechanism
without adding anything on the code side; the essence is the markdown
content.

## Goals / Non-Goals

This design covers:

- adding an entry to the `CATALOG` — trivial, but listed here to
  explicitly close the loop with the existing invariant tests;
- the structure and invariants of the skill body.

It does **not** cover the `sync`/`archive`/`update` skills (proposal out of
scope), nor adding `apply` to `DEFAULT_WORKFLOWS` — a decision deferred
until feedback is in.

## Decisions

### Decision: `allowed-tools` includes `Bash` in addition to `Bash(codev:*)`

`apply` must be able to run the verification commands written in
`tasks.md` (`cargo test …`, `cargo build …`, etc.). Restricting to
`Bash(codev:*)` like `propose` and `explore` would prevent the verification
that each task specifically requires.

Decided to list explicitly: `Bash(codev:*), Read, Write, Edit, Glob,
Grep, Bash`. The order carries meaning for the user reading the
frontmatter — the `Bash(codev:*)` prefix first documents the main usage;
plain `Bash` last documents the necessary opening.

**Rejected alternative**: restrict to `Bash(cargo:*), Bash(codev:*)`. Too
narrow — a test may require `git`, `npm`, `python`. The project context
decides; codev has no business prejudging the toolbox.

### Decision: the skill checks boxes via `Edit`, not via a CLI command

There is no `codev task check <n>` command — and there will not be one
as long as the `tasks.md` format stays stable, cf. decision
[0001](../../decisions/0001-functional-core-imperative-shell.md): the
core does not touch the disk, and the markdown format is the source of
truth read by `codev status`. The skill therefore uses `Edit` to turn
`- [ ] X.Y` into `- [x] X.Y` on the task's line.

**Rationale**: introducing a dedicated command would duplicate the tasks
parser logic (B3, not yet shipped) with no gain — the agent already sees
the file's lines via `Read`, knows how to make a targeted `Edit`, and the
round-trip is trivial. The day B3 arrives, the skill can migrate without
breaking its public contract.

### Decision: the skill stops at the last `[x]` and invites to archive

An `apply` that automatically triggered `archive` would violate the rule
"a skill does one thing" — and above all, the user wants to review the
result before archiving. The skill states explicitly which change is ready
and leaves the next step to the user.

## Risks / Trade-offs

- **A badly worded task blocks everything**. A box that describes two
  different things forces the skill to stop and ask. → **Accepted
  trade-off**: this is the intended behavior. The alternative — guessing —
  leads to silent debt. The stop message quotes the task and suggests
  splitting it into two `X.Y.a` / `X.Y.b`, without imposing it.
- **The `- [ ]` format is fragile to spacing**. A `-[ ]` without a space,
  an uppercase `- [X]`, a box with `- [-]` — each breaks recognition.
  → **Mitigation**: the skill describes the expected format exactly
  (`- [ ]` with spaces) and invites correction if something else is
  found. Later, B3 may tolerate the variants.
- **Long tasks.md → endless session**. The skill may spend an hour on
  some thirty tasks. → **Accepted trade-off**: that is the very purpose of
  `apply`. A future `apply --batch <N>` will limit it if the need arises.

## Migration Plan

Not applicable — this is a new skill. An existing project that has
`workflows: [propose, explore]` in its `config.yaml` must add `apply` to it
and rerun `codev update` to install it. The `codev update` message already
points out the new skill when it appears.
