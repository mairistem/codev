# Design: `configure` skill + hint

## Context

See `proposal.md`. New workflow (the 8th), pure function
`is_config_thin` in `codev-core`, two hints in `codev-cli`,
update of the `onboard` body.

## Decisions

### Decision: "thin" threshold = `context < 200 chars AND empty rules`

Two conjunctive criteria. A project where a user has filled in one
or the other is considered "already configured" — the
`configure` skill has nothing fundamental left to offer.

The 200-character threshold is arbitrary but informed by the minimal
output of `codev init` on a bare Rust project: `Rust project, 2024.`
is 19 characters. The 200 threshold easily lets through a
detailed `context:` (stack + one sentence of context), and cuts
stubs short.

**Rejected alternative A**: threshold on `context` alone. Rejected — a
user who wrote `rules:` without `context:` has explicitly
chosen their configuration.

**Rejected alternative B**: threshold configurable via `_codev/config.yaml`.
Rejected — over-engineering; the threshold is a default, not a policy.

### Decision: `configure` has `Bash(codev:*), Read, Glob, Grep, Write, Edit` — but not general Bash

The skill must:

- Read files (`Read`, `Glob`, `Grep`).
- Write a `_codev/config.yaml` (`Write`).
- Edit precisely — preserve comments — (`Edit`).
- Call `codev status`, `codev list --specs` to get its bearings
  (`Bash(codev:*)`).

It **does not have** general `Bash`: it runs no tests, no
`git`, no external tool. The invariant rule "only `apply` has
general Bash" stays preserved.

**Rejected alternative**: grant general Bash to let Claude
read the history via `git log`. Rejected — the `.git/index` file +
`git log` is rarely enough to understand a project, and `Read` on
`README.md` + a sample of code does 90% of the work.

### Decision: the skill shows the diff, writes on confirmation — never a silent write

The body of `configure.md` MUST guide Claude through a strict sequence:

1. Gather what is to be proposed (YAML patch).
2. **Show** the diff to the user — line by line, with the
   leading comment "here is what I propose".
3. Ask "Apply? [yes/no]".
4. Write only if yes.

The sequence is explicit in the body — not an implicit convention.
A skill that writes without confirmation would betray trust.

**Rejected alternative**: an `--auto` mode that writes without confirmation
for scripting. Rejected — this is precisely a skill, not a CLI. A
user who wants to script edits `_codev/config.yaml` by hand
via a template.

### Decision: the `is_config_thin` function lives in `codev-core::config`

It is a pure function over `ProjectConfig` — no I/O, no
environment. It belongs to the core.

Having it take a type from `codev-engine` (`ProjectConfig`)
would break the dependency direction. **Solution**: the function takes
`context: Option<&str>` and `rules_empty: bool` directly as parameters,
not the whole `ProjectConfig`. Each caller reads these two fields and
passes them. Simple, testable on its own.

**Rejected alternative**: duplicate `ProjectConfig` in `codev-core`
just for this function. Rejected — type duplication is a
high cost for a tiny gain.

### Decision: the hint in `codev status` appears only when no change is active

`codev status` has several output modes — with an active change, without,
with several. The hint only makes sense in the "project
initialized, no active change" case — that is, the typical case after
a `codev init`. Inserting it in the "active change" case would clutter
the screen of a user in the middle of their work.

Technically: the hint is shown only in the
`no_active_change` error branch of the human output, never in the
JSON report.

### Decision: project reading by the skill — an explicit budget

The body of `configure.md` MUST set explicit limits so
that Claude does not read the whole project:

- **README** — full read.
- **CONTRIBUTING.md** — full read if present.
- **docs/** — glob of `*.md`, read the 3-5 shortest
  files.
- **Source files** — up to 8 files, prioritized by commit
  recency (`git log --since='6 months ago' --pretty=format: --name-only`)
  filtered by the usual extension of the detected language.
- **Structure** — `ls _codev/`, `ls src/` or equivalent — a single
  level.

This budget contains the token cost and output variability. Without a
budget, Claude would read at random and propose inconsistent
contexts.

**Rejected alternative**: "let Claude read whatever it wants". Rejected
— experience shows that without bounds, the output becomes unpredictable.

### Decision: update of the `onboard` body — new branch BEFORE the others

In the decision table of `onboard.md`, the "thin config" branch
must be placed **before** the branches about changes, because a
freshly initialized project by definition has no active change —
otherwise we would already be in another branch.

Final order:

1. `_codev/` absent → `codev init`
2. Thin config, no change → `/codev-configure` then `/codev-propose`
3. No change → read README then `/codev-propose`
4. One active change, planning incomplete → `/codev-propose <name>`
5. One active change, planning complete → `/codev-apply <name>`
6. Several changes → list

## Risks / Trade-offs

- **The "thin" threshold ages** — a project may have a context of
  180 characters that is perfect, and the hint will be wrong. →
  **Mitigation**: the hint is gentle ("Recommended next
  step" / "hint"), never blocking. The user can
  ignore it.
- **The skill may propose a bad context** — Claude may read
  a project and get its conventions wrong. → **Mitigation**:
  mandatory confirmation with a diff. What the user sees and
  accepts is what they get.
- **The `configure.md` body is notoriously fragile** — like all
  skills, an instruction that is too loose produces erratic output.
  → **Mitigation**: the explicit reading budget + the strictly
  described sequence + the typed preserved/forbidden lists.
- **`Bash(codev:*), Write, Edit` is almost `apply`** — the skill is
  powerful. → **Mitigation**: it touches only one file
  (`_codev/config.yaml`) and the body states so. An invariant test
  checks that the body explicitly cites this constraint.

## Migration Plan

No migration needed. An existing project:

- Sees the 8th skill appear on the next `codev update`.
- Receives the hint on its next `codev init` or `codev status`
  **only** if its `_codev/config.yaml` is thin.

A project whose user filled in the config manually will
never see the hint — the experience is exactly identical to
today.
