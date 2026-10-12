# Design: Documentation and tests in the definition of done

## Context

The `spec-driven` schema (`assets/schemas/spec-driven/schema.yaml`) is
embedded in the binary; each artifact instruction opens with the role block
introduced by `propose-review-roles`. The `propose`, `apply` and `configure`
workflows are Markdown bodies in `assets/workflows/`, rendered into
`.claude/skills/codev-<id>/SKILL.md` by `codev update`. They are prompts:
every line costs tokens on every run, and a line repeated in two places
drifts. `apply.md` already says "verify, then check off" (step 3d, the
"Verify before checking off" guardrail) but not which command runs the tests
nor that the whole suite runs at the end. `configure.md` caps its reading
at 5 docs and 8 source files and has no Bash beyond `codev`.

The init probe (`codev_core::detect::stack`) extracts the language, edition,
crate count and up to ten dependency names from the manifest; `codev init`
renders them into the generated `context:`.

## Goals / Non-Goals

**Goals:**

- Documentation and automated tests become explicit completion criteria of
  every plan written with the default schema, checked at planning time by
  `/codev-propose` and enforced at implementation time by `/codev-apply`.
- Test conventions come from the project first, from a short reference
  second.

**Non-Goals:**

- A machine check of these criteria in `codev validate`: they are guidance
  for the agent, like the rest of the role block.
- Recording the test framework in the init probe (see the last decision).
- Running tests from `/codev-configure`: it stays without general Bash.

## Decisions

### Decision: The criteria live in the tech lead's role block

The three new criteria are appended to the `tasks` "Done when" list, and the
`proposal` Impact guidance names documentation, in the same format as the
existing ones. The schema is where a project already redefines artifacts, so
a custom schema keeps its own definition of done. `/codev-propose` and
`/codev-update` both read these instructions, so both inherit the criteria.

**Alternatives considered**: putting the criteria in `propose.md` only (a
custom schema with no tests artifact would get them anyway, and
`/codev-update` would not see them); a project `rules:` default written by
`codev init` (only new projects would get it, and it duplicates the schema).

### Decision: Each skill adds only what it alone does

`propose.md` extends its existing traceability bullets — the scenario bullet
gains "names an automated test", one bullet covers documentation — instead
of a new step. `apply.md` amends steps 3c and 3d in place (tests after the
existing suite; the test command found from `context`/`rules`, manifest or
CI, never guessed) and adds one step before the final summary: run the full
suite once. The existing "verify before checking off" guardrail already
states the check-off rule, so no new guardrail is added. `configure.md`
adds testing rules to the `rules:` it drafts, a small allowance to read
existing tests within its budget, and the conventions reference.

This follows `_codev/decisions/0004-single-identity-for-skill-and-command.md`:
the existing skills are extended, no new skill is shipped.

**Alternatives considered**: a dedicated "testing" step in each skill
(repeats the role block and the guardrails, longer prompts for no new
behavior); a shared include file for the conventions (the workflow bodies
are self-contained `include_str!` assets and have no include mechanism).

### Decision: The conventions reference lives in `configure.md`, after the principle

The reference is a compact list, one or two lines per language (Rust,
TypeScript/JavaScript, Python, Go, Java), preceded by "the project's existing
practice wins". It is in `configure` because that is where rules are
written once and then reach every artifact through `codev instructions`;
`apply` reads `rules` and the existing suite, so it needs no copy.

**Alternatives considered**: the reference in `apply.md` (applied on every
task, but invisible to the plan and repeated at each run); in the schema
(it would apply to every project whatever its language).

### Decision: The stack probe is left unchanged

Recording the test framework in the detected context would change the
`init` golden outputs and the `init` spec for little gain: `/codev-configure`
reads the manifest and the existing tests anyway, and it is the step that
turns them into rules. The probe stays pure and its output stable
(`_codev/decisions/0001-functional-core-imperative-shell.md` is unaffected
either way).

**Alternatives considered**: parsing dev-dependencies for vitest, jest,
pytest and others in `stack.rs` (more parsing per manifest format, a new
field in the generated `context:`, and a second source of truth next to
the rules `configure` writes).

## Risks / Trade-offs

- [Longer prompts cost tokens on every run] → Each skill gains a handful of
  lines; the reference is one line per language.
- [The full suite is slow on large projects] → It runs once, at the end of
  `apply`, not after each task; each task runs only the verification it
  cites.
- [The agent invents a test command] → `apply` names where to find it and
  asks the user when none is found.
- [A small change gets a documentation task it does not need] → The
  criterion accepts "the plan states none is affected".
- [Tests quoting the old instruction or skill text break] → The existing
  text is kept; only additions, asserted by new tests.
