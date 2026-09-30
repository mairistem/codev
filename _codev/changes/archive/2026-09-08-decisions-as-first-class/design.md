# Design: decisions as first-class objects

## Context

See `proposal.md` for the motivation. The spec parser and its span-carrying
AST (delivered by `parse-specs-and-deltas`) are reusable as is for reading
an ADR's sections — no need to write a second markdown parser. The YAML
frontmatter is handled by `serde_norway`, already present as chosen by
decision [0006](../../decisions/0006-serde-norway-for-yaml.md).

## Goals / Non-Goals

This design covers:

- the core / engine split — pure parser in `codev-core::decisions`, index
  and merging of sources in `codev-engine::decisions`;
- the public shape of the JSON contract for decisions;
- the injection strategy into `instructions design`, without breaking what
  exists.

It does **not** cover the `codev decision …` CLI commands (K5), immutability
detection (K3), deviation (K6), or promotion from design (K7). These four
points form a coherent batch to be handled next.

## Decisions

### Decision: pure parser in `codev-core::decisions`

Like `parser`, `validate`, `merge` — one more module in `codev-core`,
without I/O. It exposes `parse_decision(source: &str) -> Parsed<Decision>`
which returns a `Decision { id, title, status, date, tags, supersedes,
sections, span }` along with the list of structural `Finding`s encountered.

The frontmatter parser borrows the already proven machinery: `serde_norway`
on the block between the two `---`. The markdown body is parsed into
top-level `##` sections via the building blocks of `parser::shared` —
without rewriting literal-zone recognition. Follows directly from decision
[0001](../../decisions/0001-functional-core-imperative-shell.md) —
pure core — and the crate graph of decision
[0002](../../decisions/0002-crate-graph-as-dependency-rule.md).

**Rejected alternative**: a dedicated ADR parser in a new crate. The
volume is too small to justify one more boundary, and `codev-core` stays
free of new dependencies.

### Decision: index and supersession in `codev-engine::decisions`

The engine coordinates reading the disk and merging with inherited
sources. It exposes `pub fn index(fs, layout, config) -> DecisionIndex`
which returns:

```rust
pub struct DecisionIndex {
    pub entries: Vec<IndexEntry>,     // all parsed decisions
    pub in_effect: Vec<QualifiedId>,  // qualified source+id
    pub findings: Vec<Finding>,       // id conflicts, phantom supersedes...
}
```

Computing `in_effect` is a simple graph traversal: an `accepted` decision
not superseded by another `accepted` decision is in effect. The
implementation is a `BTreeMap` of qualifiers plus a linear sweep — the size
(a few dozen ADRs at most in practice) justifies nothing more.

**Rationale**: coordination touches the disk, so it lives in the engine.
Follows decision
[0001](../../decisions/0001-functional-core-imperative-shell.md).

### Decision: qualified identifiers `origin/id`

A `QualifiedId` is an `(origin, id)` — `origin` being `"project"` or
`"path:<declared path>"`. This is what allows two sources to coexist
without collision. JSON serialization produces a short `id` field (the
local number) and a `qualifiedId` field (`"project/0007"` or
`"path:~/shared/0100"`) — so that simple consumers can make do with the
former and multi-source consumers get the latter.

**Rejected alternative**: namespace the `id` directly (`"shared-0100"`).
It changes the ADR format on read, and would force a migrating project to
rename its files. The external qualifier keeps files as they are.

### Decision: the project's version wins on an id collision

When the same `id` appears in the project and in an inherited source, a
`decision_id_collision` finding is emitted but **the project's version is
kept as the one in effect** — the inherited version becomes "masked". The
finding is enough to surface the problem without blocking rendering.

**Rationale**: the project's version is closer to the author and most
likely more recent. Refusing rather than masking would stop the command on
a recoverable case.

### Decision: `decisions[]` field added to `InstructionsV1`, not merged into `context`

A new dedicated field at the root level:

```json
{
  "changeName": "…",
  "artifact": "design",
  "context": [ … ],
  "rules": [ … ],
  "decisions": [
    { "id": "0007", "qualifiedId": "project/0007",
      "title": "…", "status": "accepted", "tags": ["architecture"],
      "path": "_codev/decisions/0007-….md", "origin": "project" }
  ],
  …
}
```

Backward compatible: an added field, no break. The field is present and
empty in the responses for non-`design` artifacts.

**Rationale**: merging into `context` would require going through
`origin: "decisions"` and serializing all the content — losing the usable
structure (id, status, tags). A typed field is clearer on the agent side:
"read these decisions before drafting", with no confusion with free-form
project context.

### Decision: the human rendering adds a "Decisions in effect" section

A single section, one line per decision: `- <id> <title>`. Neither the
content nor the path — the agent producing the design will read the files.
The human rendering is for a quick glance; the richness is on the JSON
side.

**Rationale**: stays compact, aligned with how `context` and `rules` are
rendered. A consumer who wants more has `--json`.

## Risks / Trade-offs

- **A project with 100 decisions** would inject 100 entries into every
  `instructions design` call. → **Accepted trade-off**: the list is
  filtered to `in_effect`, in practice < 30 in the known cases. If that
  ever becomes too much, a filter by `tags` will be added — out of scope
  for this change.
- **The frontmatter parser tolerates unknown keys by default with
  `serde_norway`**. → **Chosen approach**: `deny_unknown_fields` on the
  frontmatter struct, consistent with the rest of the project (`Schema`,
  `ChangeMetadata`, `ProjectConfig`). A faulty key surfaces immediately.
- **Circular supersession chain** (`A supersedes B` and `B supersedes
  A`). → **Mitigation**: detected when building the index, finding
  `decision_supersession_cycle`, chains ignored for the `in_effect`
  computation (neither enters it) — forces the author to fix it.
- **An ADR with valid frontmatter but whose id is not kebab-numeric**
  (for instance `id: my-decision`). → **Chosen approach**: accept any
  non-empty `id`, without imposing a format. The 6 existing ADRs use
  `0001…0006` by documentation convention, but the tool does not enforce
  it.

## Migration Plan

Not applicable — new capability. The 6 ADRs already present in the
repository will be indexed by the first `codev instructions design` call
after compilation, with no manual intervention. This is the change's first
real test.
