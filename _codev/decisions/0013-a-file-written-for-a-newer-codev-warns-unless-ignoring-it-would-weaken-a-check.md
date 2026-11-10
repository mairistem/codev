---
id: "0013"
title: "A file written for a newer codev warns, unless ignoring it would weaken a check"
status: accepted
date: 2026-10-26
---

## Context

No decision said what a codev does with a file written for another
version, so each change chose: `_codev/config.yaml` warned on an unknown
key, while `change.yaml`, a project `schema.yaml`, a role's settings and
an ADR frontmatter refused it, and `inherits` and `changelog` refused it
on purpose. The migration reviewer raised the same problem in four changes
(`R-migration-*` of `expert-triggers-and-stack-filter`, `review-record`,
`isolated-review`, `fix-merge-and-skill-refresh`), and decision 0012
measured its cost: an older case tree rejected an appended `roles:` key.

## Decision

One repository is read by several codev versions at once — teammates who
updated at different times, a CI pinned to a release. What a codev does
with a file written for another version follows these rules.

1. **A hand-written file shared by the team tolerates unknown keys.** In
   `_codev/config.yaml`, a change's `change.yaml`, a project's
   `schema.yaml` and an ADR frontmatter — and in the fields of a
   `review.md` or `corrections.md` of a format it reads — a key the
   running codev does not know is ignored with a warning that names it
   and points to `codev_version` and `codev self-update`. It is never a
   hard error. `codev validate --strict` fails on these warnings, so a
   typo stays caught in CI. A file inherited from a source is not the
   team's to fix: its unknown keys are not reported.
2. **Fail closed, not open.** When ignoring a key or a value could weaken
   a check — the archive gate, inheritance, security — the verdict must
   not be given by a codev that ignores it. A key of that kind requires a
   minimum `codev_version` in `_codev/config.yaml`, and the commands whose
   success is a verdict on the project (`codev validate`, `codev sync`,
   `codev archive`) refuse to run when the running codev does not satisfy
   it. Other commands may still run without the key; a key whose ignoring
   would mislead them too (what the skills are told, which reviewers run)
   must make those commands strict as well, and its change says so. The keys of an `inherits` entry and of `changelog`, which predate
   this policy, fail closed by themselves: ignoring them would read
   another source or write another file in every command, not only in a
   gate.
3. **A file codev writes carries a format marker.** `seal.yaml`
   (`version`), `review.md` (`<!-- codev-review: n -->`) and
   `corrections.md` (`<!-- codev-corrections: n -->`). A field an older
   codev may safely ignore is added under point 1. A change it would
   misread — a value added to a closed set (a status, a severity, a step),
   a field that changes what another field means — is a format change:
   the marker goes up, and an older codev does not check the file as if it
   understood it. It says so, naming the update, and the gate that needs
   the file (the archive for `review.md`; every command for `seal.yaml`)
   refuses it.
4. **Content shipped inside the binary may stay strict.** The built-in
   schema and roles are versioned with the binary that reads them; an
   unknown key there is a defect of that release, caught by its tests.
5. **Every change that adds a key, a value or a format says what an older
   codev does.** Its design states it; its release note gives the minimum
   `codev_version`; if it fixes data already produced, it gives a way to
   detect and repair it. The migration expert checks this in review.
6. **A codev update never rewrites the project's files.** Specs,
   decisions, configuration and changes are left as they are; a newer
   codev reads old formats as they are written. A real format migration
   is an explicit command the user runs, which shows what it will change,
   announced in the release note — as `codev decision seal` was for the
   seals.

## Consequences

- A teammate one release behind keeps working on a repository a newer
  codev wrote, and is told which version to install.
- A typo in a hand-written file is a warning locally and a failure in CI
  (`codev validate --strict`), no longer a failure of every command.
- `codev_version` becomes a gate: raising it stops `validate`, `sync` and
  `archive` on older codevs, CI pins included, until they update. Raise
  it only once the release that satisfies it is out.
- The policy only binds codevs that ship it; earlier releases keep their
  strictness, and the documentation says which ones.
- Each new key costs its change one paragraph — what an older codev does
  — and, for a key of point 2, a `codev_version` bump announced in the
  release note.

## Alternatives considered

- **Strict everywhere** (`deny_unknown_fields`): typos are caught at
  once, but every new key breaks every teammate who has not updated, which
  is what the four findings were about.
- **Tolerant everywhere**, gating keys included: an older codev would
  archive past a check it does not know.
- **Fail closed by a per-key marker** that an older codev recognizes
  without knowing the key (a reserved prefix, a `requires:` list in the
  file): more precise than `codev_version`, but a second versioning
  mechanism to learn, and `codev_version` already exists, with its doctor
  check and the skills' offer to update.
- **Automatic migration on update**: a newer codev rewriting files the
  team shares would force every teammate to update at once and hide what
  changed.
