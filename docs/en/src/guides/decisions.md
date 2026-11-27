# Decisions

An architecture decision records a choice the whole project follows — a
library, a way to structure the code — and why it was made. This chapter
explains how a decision is proposed, accepted, replaced and, when needed,
departed from.

A [decision](../glossary.md#decision), or ADR (architecture decision
record), is a file in `_compagnon/decisions/`, named with a number and a
short title (`NNNN-<slug>.md`). It holds the context that led to the
choice, the choice itself, its consequences and the other options
considered. Claude receives the decisions **in effect** every time it
writes a `design.md`, so designs follow them instead of debating them
again.

<figure class="diagram">
<div class="scroll">
<svg viewBox="0 0 760 280" role="img" aria-label="The life of a decision: it is proposed, and you can rewrite it freely; accepting it seals it and puts it in effect; when a successor is accepted, it is superseded and kept as history. Side branches: a choice written in design.md is promoted into a proposed decision; once a local deviation is accepted, an inherited decision is deviated from and no longer applies in this project.">
<defs><marker id="dg-decisions-arrow" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" style="fill:var(--dg-line)"/></marker></defs>
<rect x="8" y="40" width="200" height="80" rx="14" style="fill:var(--dg-box);stroke:var(--dg-claude)" stroke-width="2"/>
<text x="108" y="74" text-anchor="middle" font-size="17" font-weight="600" style="fill:var(--dg-ink)">Proposed</text>
<text x="108" y="98" text-anchor="middle" font-size="13" style="fill:var(--dg-muted)">rewrite it freely</text>
<line x1="210" y1="80" x2="276" y2="80" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-decisions-arrow)"/>
<text x="243" y="68" text-anchor="middle" class="mono" font-size="12" style="fill:var(--dg-muted)">accept</text>
<rect x="280" y="40" width="200" height="80" rx="14" style="fill:var(--dg-box);stroke:var(--dg-you)" stroke-width="2"/>
<circle cx="302" cy="58" r="5" style="fill:var(--dg-you)"/>
<text x="380" y="74" text-anchor="middle" font-size="17" font-weight="600" style="fill:var(--dg-ink)">Accepted</text>
<text x="380" y="98" text-anchor="middle" font-size="13" style="fill:var(--dg-muted)">sealed, in effect</text>
<line x1="482" y1="80" x2="548" y2="80" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-decisions-arrow)"/>
<text x="515" y="68" text-anchor="middle" font-size="12" style="fill:var(--dg-muted)">replaced</text>
<rect x="552" y="40" width="200" height="80" rx="14" style="fill:var(--dg-box);stroke:var(--dg-line)" stroke-width="2"/>
<text x="652" y="74" text-anchor="middle" font-size="17" font-weight="600" style="fill:var(--dg-ink)">Superseded</text>
<text x="652" y="98" text-anchor="middle" font-size="13" style="fill:var(--dg-muted)">kept as history</text>
<rect x="8" y="190" width="200" height="72" rx="12" style="fill:var(--dg-file);stroke:var(--dg-line)" stroke-width="2"/>
<text x="108" y="220" text-anchor="middle" font-size="15" font-weight="600" style="fill:var(--dg-ink)">Promoted</text>
<text x="108" y="244" text-anchor="middle" class="mono" font-size="12" style="fill:var(--dg-muted)">from design.md</text>
<line x1="108" y1="188" x2="108" y2="124" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-decisions-arrow)"/>
<text x="118" y="160" class="mono" font-size="12" style="fill:var(--dg-muted)">promote</text>
<path d="M380 122 V226 H548" fill="none" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-decisions-arrow)"/>
<text x="392" y="176" font-size="12" style="fill:var(--dg-muted)">your deviation accepted</text>
<rect x="552" y="190" width="200" height="72" rx="12" style="fill:var(--dg-file);stroke:var(--dg-line)" stroke-width="2"/>
<text x="652" y="220" text-anchor="middle" font-size="15" font-weight="600" style="fill:var(--dg-ink)">Deviated from</text>
<text x="652" y="244" text-anchor="middle" font-size="12" style="fill:var(--dg-muted)">inherited, set aside here</text>
</svg>
</div>
<figcaption>The life of a decision: written while proposed, sealed once accepted, superseded rather than edited; a design can promote one, and a local deviation sets an inherited one aside.</figcaption>
</figure>

A decision has one of five statuses: `proposed`, `accepted`, `superseded`,
`deprecated` or `rejected`. Each one has a short id, such as `0007`, and a
full id that also says where it comes from: your project, or another
repository it inherits from.

<details>
<summary>Under the hood: the full id of a decision</summary>

The qualified id names the decision's origin: `project/0007` for a decision
of the project itself, `path:~/shared/0100` or
`git:git@github.com:acme/shared.git/0100` for a decision inherited from
another repository.

</details>

## Proposed

Every new decision starts `proposed`, whichever way it is created: from
scratch, to replace another decision, as a deviation or promoted from a
design. While it is `proposed`, you can write and rewrite it freely. It
does not replace or set aside anything yet.

<details>
<summary>Under the hood: creating a decision</summary>

```bash
compagnon decision new "Use PostgreSQL for persistence"
```

`compagnon decision supersede`, `deviate` and `promote`, below, also create
a `proposed` decision. `compagnon decision list` shows every decision and
its status.

</details>

## Accepted and sealed

Once its text is final, you accept the decision. Accepting is the only step
that puts a decision into effect. A decision is **in effect** when it is
`accepted` and no other decision replaces it.

Accepting also seals it. The [seal](../glossary.md#seal) is a fingerprint of
the decision's text: if someone edits the text later, `compagnon validate`
sees that the fingerprint no longer matches and reports it. It also warns
about an accepted decision that has no seal. A `proposed` decision is not
sealed.

The seal does not cover the decision's
[frontmatter](../glossary.md#frontmatter), the short header at the top of
the file that holds its status. So the status can move from `accepted` to
`superseded` without breaking the seal.

> **Note**
> To change what an accepted decision says, [supersede](#superseded) it
> instead of editing it. If you edited a sealed decision on purpose, seal
> it again; [Error codes and fixes](../reference/errors.md#decision_seal_mismatch)
> says how.

<details>
<summary>Under the hood: accepting, and how the seal is recorded</summary>

```bash
compagnon decision accept 0001
compagnon decision seal 0001 --force   # record the new text of an edited decision
```

When a decision becomes accepted through a `compagnon decision` command —
`accept`, or `new --status accepted` — Compagnon records a SHA-256 hash of
the decision's body in `_compagnon/decisions/seal.yaml`.
`compagnon decision seal` does the same for a decision you accepted by
hand. `compagnon validate` compares the body of every local `accepted` or
`superseded` decision with its hash, and reports an edit as a
`decision_seal_mismatch` error and a missing seal as a `decision_unsealed`
warning. A decision created with `new --status accepted` is sealed straight
away, with the template's placeholder text: prefer `new`, then `accept`.

</details>

## Superseded

To change a decision, do not edit it: replace it with a new one. Compagnon
calls this superseding it.

The new decision starts `proposed`, and names the one it replaces. The old
one stays `accepted`, and in effect, while you write the new one. When you
accept the new decision, the old one is marked `superseded` in the same
step. Its text stays exactly as it was, so its seal stays valid and the
history of the reasoning is kept.

Only an `accepted` decision can be replaced. If another decision replaced
it in the meantime, accepting yours is refused and nothing is written: see
[`predecessor_not_accepted`](../reference/errors.md#predecessor_not_accepted).

<details>
<summary>Under the hood: superseding a decision</summary>

```bash
compagnon decision supersede 0001 "Use SQLite for persistence"
compagnon decision accept 0002
```

The first command creates `0002` with `supersedes: ["0001"]` in its
frontmatter and does not touch `0001`. The second accepts `0002` and marks
`0001` `superseded`.

</details>

## Deviated from

Some decisions come from another repository: they are
[inherited](inherited-sources.md). You cannot replace one of those from
your project, because you cannot write to its source. Instead, record a
local exception, called a deviation.

The deviation is a `proposed` decision of your project that points at the
inherited one. It takes effect when you accept it. From then on, the
inherited decision still appears in the list of decisions, but it no longer
applies to your project, and Claude no longer receives it when writing
designs. Until you accept the deviation, the inherited decision still
applies.

<details>
<summary>Under the hood: deviating from an inherited decision</summary>

```bash
compagnon decision deviate path:~/shared/0100 "Services log in logfmt"
compagnon decision accept 0003
```

The new decision's `deviates_from` field holds the inherited decision's
full id.

</details>

## Promoted

While a change is being planned, its design often makes a choice that
deserves to outlive the change. Write that choice in `design.md` as a
`### Decision: <title>` block, then promote it to a decision of its own.

Compagnon creates a `proposed` decision whose `## Decision` section holds
the block's text, word for word. In `design.md`, the block's text is
replaced by a reference to the new decision. Reread the new decision and
spread the promoted text across its sections (context, decision,
consequences). Then accept it, which seals it — ideally before archiving
the change.

<details>
<summary>Under the hood: promoting a design decision</summary>

```bash
compagnon decision promote add-audit-log "Append-only audit table"
```

The first argument is the change's name, the second the title of the
`### Decision:` block in its `design.md`.

</details>

## Go further

- [Concepts](../concepts.md): specs, decisions and changes, and how they
  fit together.
- [Inherited sources](inherited-sources.md): decisions shared from another
  repository.
- [Command-line interface](../reference/cli.md): every
  `compagnon decision` command and its options.
- [Error codes and fixes](../reference/errors.md): what to do when a
  decision is reported.

## Next step

[Roles](../roles.md)
