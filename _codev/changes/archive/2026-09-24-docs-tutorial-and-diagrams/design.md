# Design: tutorial + Mermaid diagrams

## Context

See `proposal.md`. A **documentation content** change — one file
modified (`docs/codev.md`), zero Rust, `skip_specs: true`.

## Decisions

### Decision: the tutorial simulates a concrete Rust case, not an abstract example

The tutorial uses "add a `--json` option to `codev list`" as a
fictional change. Two reasons:

- **It speaks to a Rust dev** — it is exactly the kind of contribution
  a reader may have in mind.
- **It is verifiable** — the reader can literally follow the
  process on codev itself; it is pedagogical dogfooding. It
  leads to a realistic delta (a `MODIFIED` requirement on the
  `cli-status` or `cli-list` capability, plus a `--json` scenario).

**Rejected alternative**: a non-Rust example ("add a `/health`
route to an API"). Rejected — breaks consistency, forces us to
invent a complete fictional project. A reader who codes in Rust
understands; one who codes in something else translates effortlessly.

### Decision: the tutorial covers both paths (Claude Code / pure CLI) side by side

Each tutorial step lists **both** possible commands:

```markdown
In Claude Code:

    /codev-propose add-list-json

Or with the pure CLI:

    codev new change add-list-json --goal "…"
    $EDITOR _codev/changes/add-list-json/proposal.md
```

This avoids duplicating the tutorial in two versions, and it shows that
codev works **without** Claude Code (the agent is not mandatory).

**Rejected alternative**: two separate tutorials (a "Claude Code" one
and a "pure CLI" one). Costly to maintain, twice the work to keep up to date
for every change.

### Decision: Mermaid, despite the degradation in the embedded HTML

`pulldown-cmark` renders ` ```mermaid` blocks as preformatted text,
not as a diagram. In the `codev docs` HTML, the user therefore
sees the **Mermaid source** as a code block — readable but
not graphical.

This is acceptable because:

- On **GitHub** (the main reading surface), Mermaid has rendered
  natively since 2022 — the main target audience.
- The **Mermaid source is readable** as plain text for anyone who knows the
  syntax (`stateDiagram-v2`, `graph LR`, `sequenceDiagram`).
- The embedded HTML manual remains **usable offline**; the
  diagram serves there as a textual rather than graphical description.

**Rejected alternative A**: adding a Mermaid JS renderer (via CDN) to the
embedded HTML. Rejected — breaks the "standalone HTML, no external
request, offline" requirement (`docs` spec, `Requirement: The HTML is
standalone`, `Scenario: The HTML is standalone`).

**Rejected alternative B**: generating static SVGs via
`mermaid-cli` and embedding them as `data:` URIs in the HTML. Rejected —
requires a build pipeline, breaks the principle "the docs are
a single hand-edited `.md`".

### Decision: three diagrams, not five

- State machine of a change — in §3 (The cycle).
- Crate graph — in §5 (Concepts), "Architecture" subsection.
- Lifecycle of a delta — in §5 (Concepts), "Deltas"
  subsection.

Set aside at this stage — they do not move the needle enough:

- Diagram of the dependencies between a change's artifacts
  (`proposal → design → tasks`) — too simple, already clear in the
  text.
- Diagram of the inherited sources resolution path (K5) —
  interesting but niche.
- Diagram of the `codev validate --strict` flow — interesting for a
  contributor, but off the topic "what codev does for its
  user".

### Decision: the tutorial goes in §3.5, not at the top of the document

A reader who wants to "understand in 30 seconds" reads section 1
(Why codev) and 2 (Installation). One who wants to "learn in 5
minutes" enters through §3 (The cycle) then §3.5 (the tutorial).

Placing the tutorial at the top (before §1) would make it more visible but
would mix two audiences: "I'm discovering" and "I'm about to start".
The document's current progression (why → install → understand
→ practice → reference) is the right one.

**Rejected alternative**: moving the tutorial into a separate
`docs/tutorial.md` file. Rejected — breaks the principle "a single
embedded markdown source" (§4 of the `docs` spec).

## Risks / Trade-offs

- **The tutorial can age** — a change in the output of
  `codev status` invalidates the block's "expected output". →
  **Mitigation**: keep outputs short (one or two lines),
  and add at the end of the section a sentence "the exact output may
  differ from one version to another — what matters is …".
- **Mermaid renders poorly in the embedded HTML** — perceived as a
  regression by those who read `codev docs` rather than GitHub. →
  **Mitigation**: each diagram is preceded by a **prose
  caption** containing the essential information. The diagram
  enriches, it does not replace.
- **The tutorial takes up space** — probably 150-200 lines
  added to a file already at 693 lines. → **Accepted trade-off**:
  the total length of the docs remains reasonable (~900 lines), and
  the tutorial is an entry point, not a sequential read.

## Migration Plan

None. Additive evolution of the content of `docs/codev.md`.
