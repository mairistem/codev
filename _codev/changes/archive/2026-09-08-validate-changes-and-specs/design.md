# Design: validation of changes and specs

## Context

See `proposal.md` for the motivation. The parser already emits `Finding`s
per file — validation complements them, coordinates disk reading, and
produces a report that the CLI renders in two forms (human and JSON with a
stable contract).

## Goals / Non-Goals

This design covers:

- where each rule lives (pure core vs shell), and why;
- the shape of the `Finding` extended with a `path`, without losing the
  contract already used by the parser;
- the orchestration on the engine side and the JSON shape on the CLI side.

It does **not** cover `--strict` nor `--archived` nor `--concurrency`
(deferred, cf. proposal), nor the semantic merge that `sync` will perform
(upcoming change).

## Decisions

### Decision: pure rules in `codev-core`, coordination in `codev-engine`

The additional rules (missing SHALL/MUST, requirement without a scenario,
cross-section consistency) are pure functions on the AST — same inputs,
same outputs, with no clock or disk. They therefore live in
`codev-core::validate`. Coordination — finding the files, reading them,
grouping them, computing an exit code — lives in `codev-engine::validate`
behind the `FileSystem` and `Env` ports.

This follows directly from decision
[0001](../../decisions/0001-coeur-fonctionnel-coquille-imperative.md):
deciding is not executing. The rules describe, the engine performs the
reading, the CLI formats.

**Alternatives considered**:

- **Everything in `codev-engine`.** Makes the rules opaque to pure unit
  tests — one has to build an in-memory `FileSystem` to test
  "requirement without a scenario", when all we do is inspect a
  `Requirement`.
- **Everything in `codev-core`.** Forces `codev-core` to know how to walk
  a folder, which contradicts the "no I/O" contract of
  [0001](../../decisions/0001-coeur-fonctionnel-coquille-imperative.md).

### Decision: `trait Rule` with a static registry

```rust
pub trait Rule {
    fn code(&self) -> &'static str;
    fn check_spec(&self, spec: &Spec) -> Vec<Finding> { Vec::new() }
    fn check_delta(&self, delta: &Delta, change: &ChangeMetadata) -> Vec<Finding> {
        Vec::new()
    }
}
```

Each rule is an implementation, registered in a `pub static
RULES: &[&dyn Rule]`. The validator applies them all and concatenates the
findings. Adding an E4 rule (batch 2 warnings) will be one more struct in
the registry, without touching the rest.

**Alternatives considered**:

- **An `enum RuleId` + a giant `match`.** More concise at first,
  unmanageable at fifteen rules. Also loses the extension point announced
  in [0002](../../decisions/0002-graphe-de-crates-comme-regle-de-dependance.md).
- **Free functions, without a trait.** Loses the single registration
  point, and thus the guarantee that a new rule is actually wired into the
  command.

### Decision: `Finding` kept, `LocatedFinding` added

The parser's `Finding` stays as it is (stable code on the parser's public
API side, `code + line + severity + message`). The engine enriches it with
a project-relative `path` and an `item_kind`, producing a `LocatedFinding`
— this is the type that goes through the JSON contract.

**Rationale**: breaking the existing `Finding` to add a `path` to it would
break its presence in the contract of the parser's golden tests, and would
force every parser call site to carry a path that only makes sense at the
scale of a project. The separation costs a trivial conversion and avoids
this coupling.

### Decision: a single parsing call per file

The engine opens each file only once, produces a `Parsed<Spec>` or
`Parsed<Delta>`, and derives from it both the parser's findings and the
pure rules' findings. This is what makes batch validation linear in the
number of files, and not quadratic.

### Decision: `ValidateReport` report without deep nesting

```
ValidateReport
├── root: PathBuf
├── items: Vec<ItemReport>       // one per file or change
│    ├── kind: "change" | "spec"
│    ├── name: String            // "add-auth" or "user-auth"
│    ├── path: String            // project-relative
│    └── findings: Vec<LocatedFinding>
└── status: Vec<StatusEntry>     // execution errors only
```

The CLI iterates over `items` for the human rendering, and serializes as
is for the JSON — same "exactly one document on stdout" rule as the
existing contract.

**Rationale**: a change → file → finding hierarchy looks clean, but a
`_codev/specs/x/spec.md` has no parent "change". A flat model, with an
explicit `kind`, serves both cases without a conditional branch in the
consumer.

### Decision: binary exit code, never confused with a misconfigured `sh`

`0` without error, `1` with at least one `Finding` of severity `Error`
**or** with an execution error. A richer code (2 for warnings, 3 for
usage, …) is tempting, but batch 2 will introduce `--strict`, which
promotes warnings to errors; reserving several codes now would be a choice
that would have to be undone.

## Risks / Trade-offs

- **Code/message divergence between the parser and the rules.** Two
  places produce findings — a contributor could duplicate a code, or pick
  an incompatible one. → **Mitigation**: a test that iterates over `RULES`
  and the parser, checks that each `code` is unique, and rejects an
  intersection.
- **Cross-section duplicate detection costly for a huge delta.**
  Theoretically O(n²) in the number of requirements. → **Accepted
  trade-off**: the practical threshold is very low (<50 requirements per
  delta); we will revisit it when a real case shows otherwise.
- **Human output sensitive to narrow terminals.** A long path + a long
  message fits poorly in 80 columns. → **Mitigation**: the message goes on
  its own line, prefixed with its location; no fragile ASCII table.

## Migration Plan

Not applicable: new capability, no existing consumer to migrate. The
`status` field of the JSON contract reuses the shape already held by
`codev status` and `codev instructions`, so nothing new for an agent that
already knew the contract.
