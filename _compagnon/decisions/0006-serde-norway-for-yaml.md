---
id: 0006
title: serde_norway for YAML
status: accepted
date: 2026-09-08
tags: [dependencies]
---

## Context

YAML is everywhere in the format: `config.yaml`, `schema.yaml`,
`change.yaml`, and the frontmatter of `SKILL.md` files and decisions. It is
written by hand by humans, so the quality of error messages matters as much
as compliance with the standard. Yet `serde_yaml`, the historical reference,
now publishes its version as `0.9.34+deprecated`: it is archived upstream.

## Decision

`serde_norway`, a maintained fork of `serde_yaml`, with the same API.

## Consequences

- Cost-free migration from the examples and documentation of `serde_yaml`,
  which remain valid.
- A single crate covers deserialization, serialization and frontmatter.
- The choice is contained: YAML is only read at the boundaries
  (`codev-core::schema`, `codev-engine::config`). Changing it later touches a
  few modules, not the domain.

## Alternatives considered

- **`serde_yaml`**: archived upstream, as its version says itself.
- **`serde-saphyr`** (1.x, panic-free, good error messages): the most
  attractive option on paper, but a recent implementation with less proven
  serde compatibility. To reconsider if `serde_norway`'s error messages prove
  insufficient for hand-written YAML — that would then be a decision
  superseding this one.
- **Raw `yaml-rust2`**: it would make us write the deserialization layer
  ourselves.
