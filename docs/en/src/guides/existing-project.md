# Adopting Compagnon on an existing project

Your project already has code, tests and a history. You do not need to stop
and describe it all before using Compagnon: you set it up in a few minutes,
then the specs grow one change at a time, starting with the parts you touch.

## What compagnon init adds to your project

Run it at the root of your [repository](../glossary.md#repository):

```bash
compagnon init
```

It looks at what is already there (the project's manifests such as
`Cargo.toml` or `package.json`, the license, `.github/workflows/`, your
locale, the MCP servers you declared), asks at most two questions — which
skills to install, and a short description of the project — then adds:

| What | Where |
|---|---|
| Compagnon's folder: specs, decisions, changes, archive, schemas, and the configuration | `_compagnon/`, `_compagnon/config.yaml` |
| The Claude Code skills | `.claude/skills/compagnon-<name>/SKILL.md` |
| A section telling Claude to plan before changing behavior | `CLAUDE.md`, created if missing |
| On GitHub, a [CI](../glossary.md#ci) job that runs `compagnon validate --strict` on every push and [pull request](../glossary.md#pull-request) | `.github/workflows/compagnon.yml` |
| On GitHub, a Compagnon section in the pull request template | your existing template, or `.github/PULL_REQUEST_TEMPLATE.md` |

Your own text is never replaced. Compagnon's sections in `CLAUDE.md` and the
pull request template sit between `<!-- compagnon:begin -->` and
`<!-- compagnon:end -->` markers, added at the end of your file; everything
outside them stays yours. If one of your workflows already runs
`compagnon validate`, or `.github/workflows/compagnon.yml` is a file you
wrote, Compagnon leaves the CI alone. On GitLab it never edits
`.gitlab-ci.yml`: it prints a job for you to paste. To skip any of these
files, pass `--no-claude-md`, `--no-ci`, `--no-pr-template` or
`--no-integration`. Details: [What `compagnon init`
does](../reference/cli.md#what-compagnon-init-does) and
[Repository integration and compagnon doctor](repository-integration.md).

If `_compagnon/config.yaml` already exists, `compagnon init` leaves it as it
is and only installs or refreshes the skills and these sections.

## Nothing to document up front

Right after `compagnon init`, `compagnon doctor` reports, among other
things:

```text
Planning
  ✗ specs            no main spec in _compagnon/specs/
      fix: /compagnon-propose <a change> — documenting existing behavior is planned work too
```

That is expected, and it does not block you. Compagnon works with no specs
at all: `compagnon validate` passes, and you can plan your first change
right away. The fix says it in its own words: documenting existing
behavior is planned work too.

The specs then grow change by change. Each change carries a
[delta](../glossary.md#delta): the requirements it adds, modifies, removes or
renames, describing the behavior it touches. When you archive the change,
its delta is merged into `_compagnon/specs/`. After a few months, the parts
of the product that moved are described; the parts nobody touched are not,
and do not need to be until someone does.

## Starting with a first change

Pick one of these:

- **A small real change.** Something you were going to do anyway, such as
  a bug fix or a small feature. In Claude Code:

  > /compagnon-propose Let users export their invoices as CSV

  The plan describes the new behavior; archiving it creates the first spec.
  [Your first change](../quickstart.md) walks through the four steps.
- **A change that documents what exists.** Choose a part of the product
  that changes often, or that newcomers get wrong, and ask for a plan that
  writes down how it behaves today:

  > /compagnon-propose Document how invoice numbering works today, without changing it

  The plan's requirements describe the current behavior; once archived,
  they are the spec of that part, and the next change to it starts from
  there.

Before either, `/compagnon-configure` can read your README, your
contributing guide, your documentation and a sample of your code, then
propose a `context:` and `rules:` for `_compagnon/config.yaml`: the stack,
the conventions, what each part of a plan must cover. It shows you the
diff and writes nothing until you confirm. See
[Configuration](../reference/configuration.md#context).

## Recording the decisions you already made

Your project already rests on choices: a database, a framework, how
modules depend on each other. Record the ones Claude must respect as
[decisions](../concepts.md#decision):

```bash
compagnon decision new "Use PostgreSQL for persistence"
# write the context, the choice and its consequences in the new file
compagnon decision accept 0001
```

`compagnon decision new` creates the file in `_compagnon/decisions/`,
`proposed`, so you can edit it freely; `compagnon decision accept` puts it
into effect and seals its text. From then on, every design Claude writes
follows it, or proposes explicitly to replace it. Start with two or three
that matter; `compagnon doctor` only warns when there are none.

## Next steps

- [Your first change](../quickstart.md): the four steps, from request to
  archive.
- [Configuration](../reference/configuration.md): every setting of
  `_compagnon/config.yaml`.
- [Concepts](../concepts.md): specs, changes, deltas and decisions.
