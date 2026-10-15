# Review record: rename-order-status

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### migration

#### R-migration-1 Public field removed with no compatibility period

```yaml
severity: high
confidence: high
location: "design.md#Decision: The field is renamed in place"
evidence: |
  `OrderResponse.status` becomes `OrderResponse.state`; the old name is not
  kept. The OpenAPI file is regenerated.
recommendation: >-
  Partners' integrations read `status` from the public `GET /orders/{id}`
  (design.md#Context: "partners' integrations read `status` to know when to
  ship"). On the day this ships, that field disappears: their code reads
  `undefined`, never sees `paid`, and stops shipping orders, with no error
  on either side. Use expand and contract instead: return both `status`
  and `state` with the same value, mark `status` deprecated in
  `openapi.yaml` (`deprecated: true`) with a removal date, and remove it in
  a later change once partners have moved. Add a requirement and a
  scenario to specs/orders-api/spec.md that a paid order still holds
  `"status": "paid"` during the deprecation period, and a task in tasks.md
  with a test for it; list partner integrations in proposal.md#Impact.
history:
  - status: open
    by: migration
```

#### R-migration-2 Breaking rename not marked BREAKING in the release note

```yaml
severity: high
confidence: high
location: "proposal.md#Release note"
evidence: |
  - Changed: The order's `status` field is now called `state`.
recommendation: >-
  Removing or renaming the `status` field of a public response breaks every
  consumer reading it, yet the release note files it as a plain "Changed"
  item: a partner reading it gets no signal that action is required before
  upgrading. Mark it **BREAKING** (or **Deprecated**, if R-migration-1 is
  fixed with a compatibility period), say what partners must do — read
  `state` instead of `status` — and by when `status` will be removed.
history:
  - status: open
    by: migration
```

### skeptic

Disabled by configuration.
