# Design: spec and delta parser

## Context

See `proposal.md` for the motivation. The parser lives in `codev-core`, so
it is pure — no `std::fs`, no clock — in line with decision
[0001 — Functional core, imperative shell](../../decisions/0001-coeur-fonctionnel-coquille-imperative.md).

It will be consumed by `codev-engine` (through upcoming modules: `validate`,
`sync`, `archive`), not by the CLI directly. The crate graph is enough to
enforce the direction of the flow, cf.
[0002 — The crate graph enforces the dependency rule](../../decisions/0002-graphe-de-crates-comme-regle-de-dependance.md).

## Goals / Non-Goals

This design covers only the parser — the shape of its AST, its error
boundaries, its handling of literal zones. Semantic merging
(`sync`/`archive`) and cross-file validation rules remain out of scope:
they will live in their own modules in `codev-engine`, each with its own
design.

## Decisions

### Decision: hand-written line-by-line parser, no markdown dependency

The format is a strict subset of CommonMark that we control: a few level 2,
3 and 4 headings, plus special handling of code blocks and HTML comments.
The OpenSpec reference does all its parsing in ~1,200 lines of TypeScript
with no markdown dependency.

A line-by-line pass, with a precomputed fence mask, covers 100% of the need
and keeps `codev-core` free of any new dependency.

**Alternatives considered**:

- **`pulldown-cmark`** — CommonMark pull parser, lightweight, widely used.
  Rejected: it produces generic CommonMark events from which our concepts
  must be extracted, and its notion of "position" is a `Range<usize>` per
  event that has to be grouped by hand. We would pay for the dependency
  without saving any code.
- **`comrak`** — full GitHub-Flavored Markdown AST. Rejected: heavier, and a
  tree is unnecessary for this flat structure.
- **`markdown` 1.0** — CommonMark in Rust with an AST. Same objections as
  `comrak`, less well known.

### Decision: two distinct output types, `Spec` and `Delta`

A main spec and a delta look alike on the surface, but a `MODIFIED` that
showed up in a main spec is an error, and a `Purpose` in a delta only makes
sense for a new capability. Two types give an API where the wrong mix does
not compile, rather than a common type where every consumer re-checks what
it holds.

Planned signature:

```rust
pub fn parse_spec(source: &str) -> Parsed<Spec>;
pub fn parse_delta(source: &str) -> Parsed<Delta>;
```

No `Result` in the output: a catastrophically unreadable file is hard to
tell apart from a partially recoverable one, and the latter is the more
frequent case. The distinction is made through `Parsed`.

### Decision: defect report carried by the result, never by an error

`Parsed<T>` carries both the reconstructed AST — even partial — and a list
of structural `Finding`s typed `{ line, column, code, message }`. A consumer
such as `validate` reports the findings to the user; `sync` and `archive`
refuse to write as soon as there is one with `Error` severity.

This is the same logic as the CLI's JSON contract: the message may be
reworded without notice, the `code` is stable and testable. It aligns with
[0001](../../decisions/0001-coeur-fonctionnel-coquille-imperative.md) —
deciding is not executing — since the parser *describes* the defects and
leaves it to the consumer to *decide* what to do about them.

### Decision: spans in bytes **and** in lines, without borrowing the source

Each AST node carries two views of the same interval: `byte_range:
Range<usize>` for character-exact rewriting, `line_range: Range<u32>`
for a readable error message.

The AST **does not hold** `&str`s borrowed from the source: its fields are
`String`s or `Range<usize>`s. A consumer that wants to rewrite holds the
original source and combines it with the spans. This allows a `Parsed` to
be `Send + 'static` — a necessary condition for crossing a function
boundary without lifetimes in the public signatures.

**Alternative**: borrow the source (`Spec<'a>`). Zero copy, but it forces
lifetimes on everything that handles an AST — including `codev-engine`,
which would become generic over lifetimes for a negligible gain, since a
`Requirement` is a few hundred bytes and there are a few dozen per file.

### Decision: fences follow the "first opened, first closed" rule

A fence opened by ` ``` ` closes on ` ``` ` (or longer, of the same
character) and does not accept `~~~` as a closer. A fence opened by `~~~`
closes symmetrically. The content inside — including other apparent fences
of the **same** marker but shorter — is treated as literal. This behavior
follows CommonMark and OpenSpec.

HTML comments are detected from their `<!--` to their `-->`, across
multiple lines if needed, outside fences only (inside, they are already
literal).

## Risks / Trade-offs

- **Mixed line endings** — A CRLF file yields skewed span positions if
  converted internally. → **Mitigation**: the parser operates on `&str`
  without normalization, and spans are in bytes over the input source as
  is.
- **Bug window between parser spec and semantic merge** — A `Finding`
  missed here becomes a destructive rewrite in `archive`. →
  **Mitigation**: golden tests from this change on (input files + expected
  serialized AST), and an invariant test that checks that writing each
  block through its span and then re-concatenating reproduces the source
  byte for byte.
- **Delta extensibility** — Adding a fifth operation would require
  modifying the enumeration. → **Accepted trade-off**: the four operations
  are set in stone in the format; a fifth would deserve a decision in the
  `_codev/decisions/` sense, not a mere added `enum` variant.

## Migration Plan

Not applicable: new capability, no code to evolve.
