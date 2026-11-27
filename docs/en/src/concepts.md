# Concepts

Compagnon keeps three kinds of records in your project's
[repository](glossary.md#repository), next to the code:

- **specs** say what the system does;
- **decisions** say why it is built the way it is;
- **changes** are the work in progress. Each change carries a
  [delta](glossary.md#delta): the edits it will make to the specs.

<figure class="diagram">
<div class="scroll">
<svg viewBox="0 0 760 300" role="img" aria-label="The project's records: specs say what the system does; decisions say why it is built this way; changes are the work in progress. A change holds a delta, with ADDED, MODIFIED and REMOVED requirements, which is merged into the specs when the change is synced or archived.">
<defs><marker id="dg-concepts-arrow" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" style="fill:var(--dg-line)"/></marker></defs>
<rect x="8" y="20" width="228" height="112" rx="12" style="fill:var(--dg-box);stroke:var(--dg-line)" stroke-width="2"/>
<circle cx="30" cy="58" r="5" fill="#00c39b"/>
<text x="44" y="64" font-size="18" font-weight="600" style="fill:var(--dg-ink)">Specs</text>
<text x="24" y="100" font-size="14" style="fill:var(--dg-muted)">What the system does</text>
<rect x="266" y="20" width="228" height="112" rx="12" style="fill:var(--dg-box);stroke:var(--dg-line)" stroke-width="2"/>
<circle cx="288" cy="58" r="5" fill="#ffab03"/>
<text x="302" y="64" font-size="18" font-weight="600" style="fill:var(--dg-ink)">Decisions</text>
<text x="282" y="100" font-size="14" style="fill:var(--dg-muted)">Why it is built this way</text>
<rect x="524" y="20" width="228" height="112" rx="12" style="fill:var(--dg-box);stroke:var(--dg-claude)" stroke-width="2"/>
<circle cx="546" cy="58" r="5" fill="#ff5331"/>
<text x="560" y="64" font-size="18" font-weight="600" style="fill:var(--dg-ink)">Changes</text>
<text x="540" y="100" font-size="14" style="fill:var(--dg-muted)">The work in progress</text>
<line x1="638" y1="134" x2="638" y2="158" style="stroke:var(--dg-line)" stroke-width="2" stroke-dasharray="4 4"/>
<text x="648" y="151" font-size="12" style="fill:var(--dg-muted)">holds</text>
<rect x="524" y="160" width="228" height="128" rx="12" style="fill:var(--dg-file);stroke:var(--dg-claude)" stroke-width="2"/>
<text x="540" y="186" font-size="15" font-weight="600" style="fill:var(--dg-ink)">A delta</text>
<text x="540" y="214" class="mono" font-size="13" style="fill:var(--dg-ink)">## ADDED</text>
<text x="650" y="214" font-size="12" style="fill:var(--dg-muted)">new rules</text>
<text x="540" y="240" class="mono" font-size="13" style="fill:var(--dg-ink)">## MODIFIED</text>
<text x="650" y="240" font-size="12" style="fill:var(--dg-muted)">rewritten rules</text>
<text x="540" y="266" class="mono" font-size="13" style="fill:var(--dg-ink)">## REMOVED</text>
<text x="650" y="266" font-size="12" style="fill:var(--dg-muted)">deleted rules</text>
<path d="M520 224 H122 V138" fill="none" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-concepts-arrow)"/>
<text x="320" y="212" text-anchor="middle" font-size="13" style="fill:var(--dg-muted)">merged into the specs (sync or archive)</text>
</svg>
</div>
<figcaption>The project's records: the specs say what, the decisions say why, and a change's delta is merged into the specs once the work is done.</figcaption>
</figure>

Here is where each one lives:

```text
_compagnon/
├── config.yaml     # project configuration
├── specs/          # behavior contracts — WHAT the system does
├── decisions/      # architecture decisions — WHY it is built this way
├── changes/        # work in progress, which modifies the two above
│   ├── <name>/     #   proposal.md, design.md, tasks.md, specs/** (deltas)
│   └── archive/    #   finished changes, dated
├── schemas/        # custom workflow schemas (optional)
└── compagnon.lock      # pinned inherited git sources (when used)
```

You can run every Compagnon command from any folder inside the project.
Compagnon looks upward, folder by folder, until it finds `_compagnon/`.

## Spec

The specs are organized by [capability](glossary.md#capability): one thing
the system does that someone can see from the outside, such as signing in
or the theme setting. Each capability has a name made of parts separated by
slashes, such as `ui/theme`, each part in
[kebab-case](glossary.md#kebab-case). Group your capabilities the way your
business is organized, not the way your code is.

Each capability has one spec, in `_compagnon/specs/<capability>/spec.md`.
It is the reference for what that capability does today. It has two
sections: `## Purpose`, one or two sentences on what the capability is for,
and `## Requirements`, the rules it follows.

```markdown
# Theme Specification

## Purpose

Let users choose the color theme of the interface.

## Requirements

### Requirement: Theme follows the system preference by default

The application SHALL render with the operating system's color scheme until the user picks a theme.

#### Scenario: No theme chosen yet

- **GIVEN** a user who has never changed the theme setting
- **WHEN** their operating system is set to dark mode
- **THEN** the application renders with the dark theme
```

Each requirement is a firm rule: it uses `SHALL` or `MUST` (`DOIT` or
`DOIVENT` in a spec written in French). Each has at least one scenario: a
short example of what someone can see happen.

A spec describes what the system does, never how the code does it. A simple
test: if the code can change without the behavior changing, the spec should
not change either.

You never edit the project's specs directly. They change only when a change
is synced or archived. That way, every edit to a spec comes with the change
that explains it.

## Change

A change is one piece of work, in its own folder
`_compagnon/changes/<name>/`. It holds the plan: a proposal (why and what),
its deltas (the edits to the specs), a design (how) and a task list, plus a
`change.yaml` file with its details. These parts are the change's
[artifacts](glossary.md#artifact); [How it works](workflow.md) says who
writes each one and when. The [schema](glossary.md#schema), `spec-driven`
by default, sets which parts a plan holds and the
[workflow](glossary.md#workflow) a change goes through; a project can
define its own, see [Custom schemas](guides/custom-schemas.md).

A change is **active** while its folder is in `_compagnon/changes/<name>/`.
Archiving moves it to `_compagnon/changes/archive/<YYYY-MM-DD>-<name>/`,
where it stays as a record of why and how the specs changed.

## Delta

A delta is the part of a change that edits one spec. It lives at
`_compagnon/changes/<name>/specs/<capability>/spec.md`, under the same
capability name as the spec it edits. It holds up to four kinds of edit,
each in its own section:

| Section | What it does when merged into the project's spec |
|---|---|
| `## ADDED Requirements` | Adds the requirements at the end of the spec |
| `## MODIFIED Requirements` | Replaces the requirement with the same name, where it stands |
| `## REMOVED Requirements` | Deletes the requirement, giving a **Reason** and a **Migration** (what to do instead) |
| `## RENAMED Requirements` | Changes a requirement's title, without touching its text |

Here is a delta that modifies one requirement and renames another:

```markdown
## MODIFIED Requirements

### Requirement: Theme follows the system preference by default

The application SHALL render with the operating system's color scheme until the user picks a theme, and SHALL switch live when that preference changes.

#### Scenario: No theme chosen yet

- **GIVEN** a user who has never changed the theme setting
- **WHEN** their operating system is set to dark mode
- **THEN** the application renders with the dark theme

## RENAMED Requirements

- FROM: The chosen theme is remembered
- TO: The chosen theme follows the user
```

Be careful with `MODIFIED`: it replaces the whole requirement. First copy the
entire requirement from the project's spec, scenarios included, then edit the
copy. Anything you leave out is lost when the delta is merged. If you only
add new behavior, use `ADDED`.

The merge touches only the requirements the delta names. Everything else in
the project's spec stays as it was.

A delta for a **new** capability starts with its own `## Purpose`; merging
it creates the capability's spec. To remove a capability entirely, write a
`REMOVED` delta that empties its spec and set `retire_capabilities: true`
in `change.yaml`: only [git](glossary.md#git), the project's version
history, can undo that deletion.

The exact syntax is in [File formats](reference/file-formats.md).

## Decision

A decision, or ADR (architecture decision record), records a choice about
how the system is built: what led to it, the choice itself, its
consequences and the other options considered. Each one is a file in
`_compagnon/decisions/`, named with a number and a short title
(`NNNN-<slug>.md`).

Claude receives the decisions in effect every time it writes a `design.md`,
so designs follow them instead of debating them again. An accepted
decision is [sealed](glossary.md#seal): to change it, you replace it with a new one. Some
decisions come from another repository, through an
[inherited source](guides/inherited-sources.md).

How a decision is proposed, accepted, replaced, set aside or promoted from
a design is explained in [Decisions](guides/decisions.md).

## Go further

- [How it works](workflow.md): the steps of a change and who writes each
  part of its plan.
- [Decisions](guides/decisions.md): the life of a decision.
- [File formats](reference/file-formats.md): the exact syntax of specs,
  deltas and decisions.
- [Inherited sources](guides/inherited-sources.md): rules and decisions
  shared between repositories.
- [Glossary](glossary.md): every term in a few words.
