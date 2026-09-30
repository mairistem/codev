---
id: 0001
title: Functional core, imperative shell — deciding is not executing
status: accepted
date: 2026-09-08
tags: [architecture]
---

## Context

`archive` is the most dangerous operation of the tool: it rewrites the main
specs, possibly deletes a file, then moves a directory. In OpenSpec,
`archive.ts` is 2,129 lines long, largely because deciding and writing are
intertwined — hence the destination-reservation code, the rollback and the
verified backup copy, all born from the fact that a write can fail after
others have already happened.

## Decision

The core never touches the disk. Every operation is split in two steps:

1. a **pure** function that produces a complete, serializable plan of effects
   (`ArchivePlan`, `ScaffoldPlan`, `SyncPlan`);
2. an **execution** function in the shell, dumb and transactional, the only
   one that writes.

Enforced corollary: `codev-core` depends neither on `std::fs`, nor on
`std::env`, nor on a clock, nor on the network. Effects go through the ports
described in decision [0002](0002-crate-graph-as-dependency-rule.md).

## Consequences

- `--dry-run` and the `--json` preview come for free: they are the plan,
  rendered.
- Atomicity becomes structural: the plan is validated **entirely** before the
  first write, so there is no longer any intermediate state to recover from.
- Spec merging is tested without a temporary directory, which makes golden
  tests practical — and they are the only real protection against a
  destructive markdown rewrite.
- Accepted cost: two functions instead of one, and one plan type per
  operation.

## Alternatives considered

- **Layered Clean Architecture** (`domain` / `application` /
  `infrastructure`, a Repository pattern over the file system, `*UseCase`
  structs). It pays for itself when the infrastructure is volatile — a change
  of database, ORM or third-party API. Here the infrastructure is the file
  system, and it will not change. We would have paid for the ceremony without
  getting anything for it.
- **Direct writes with rollback**, OpenSpec's approach. It works, but each new
  failure case adds a recovery path, and none of those paths can be tested
  without causing a real write failure.
