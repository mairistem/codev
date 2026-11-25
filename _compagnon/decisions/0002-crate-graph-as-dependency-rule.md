---
id: 0002
title: The crate graph enforces the dependency rule; four ports for effects
status: accepted
date: 2026-09-08
tags: [architecture]
---

## Context

We want the guarantee that the domain does not depend on the infrastructure.
In the ecosystems where Clean Architecture became popular (Java, C#), layers
are a folder convention that nothing enforces: hence the facade interfaces,
DTOs, mappers and dependency-injection containers, which exist mostly to make
the convention visible.

## Decision

A module's layer is **its position in the crate graph**, and Cargo refuses to
compile a cycle. The dependency rule is therefore checked by the compiler:

```
codev-cli → codev-agents → codev-engine → codev-core
```

Dependency inversion only applies to **effects**, through four ports:
`FileSystem`, `Clock`, `Env`, `ProcessRunner`.

Traits are only used at genuinely open boundaries — `AgentTarget` (one tool =
one implementation) and `Rule` (one validation rule = one implementation).
Elsewhere: generics, and `enum`s for closed sets (delta operations,
severities, statuses).

Promotion rule: a module only becomes a crate once it has its own heavy test
cycle or an external consumer.

## Consequences

- No `domain/application/infrastructure/` folders: they would re-implement by
  hand what the crate graph provides for free.
- `init`, `update` and `archive` are tested with an in-memory `FileSystem`,
  without temporary directories or serialized tests.
- The number of crates starts low (four). A finer split would be refactoring
  churn as long as the boundaries have not been proven.
- Accepted cost: the ports spread as generic parameters through the engine.

## Alternatives considered

- **A `trait SpecRepository` hiding the file system.** It only buys
  testability, which the effect ports already provide, while adding a
  persistence abstraction where there are only markdown files.
- **`Arc<dyn Trait>` by default** for every collaborator. It costs dynamic
  dispatch and allocation for flexibility we only need in two places.
