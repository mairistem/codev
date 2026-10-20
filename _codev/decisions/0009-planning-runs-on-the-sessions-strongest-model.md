---
id: "0009"
title: "Planning runs on the session's strongest model"
status: accepted
date: 2026-10-20
---

## Context

The planning step of `/codev-propose` decides which existing requirements
a change modifies, which it adds, and which decisions it follows. Measured
on the evaluation suite (25 planning cases, three repetitions each, on
2026-10-20), the same skills on Sonnet instead of Opus cost 59 % less
(17.51 against 42.26 USD) but found 0.62 of the primary requirements
instead of 0.82, 0.28 of the collateral ones instead of 0.50, and left
10 probable duplicates instead of 5 — below the quality the suite had
before `spec-search`.

## Decision

`/codev-propose` sets no model of its own: planning runs on the session's
model, which this project keeps on its strongest one (Opus). A change
that moves planning to another model is measured on `plan.write` first,
as decision 0008 requires, and must keep the primary recall within the
baseline's range.

## Consequences

- Planning keeps its quality, and stays the most expensive step per run
  (about 1.7 USD per plan on Opus).
- The savings on planning must come from elsewhere — what the agent
  reads, how many turns it takes — and are measured the same way.
- The roles of `plan.write` (product, QA, architect, lead) carry no model:
  they run in the session.

## Alternatives considered

- Planning on Sonnet: −59 % in cost, but a fifth of the requirements to
  modify missed and twice the duplicates.
- Exploration on a cheaper model, judgement on Opus: not measured yet; a
  later change can test it on `plan.write`.
