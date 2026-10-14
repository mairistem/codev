# Review record: rename-order-status

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### skeptic

Disabled by configuration.

### migration

#### R-migration-1 Public field removed in one step, with no transition for partners

```yaml
severity: high
confidence: high
location: "design.md#Decision: The field is renamed in place"
evidence: |
  `OrderResponse.status` becomes `OrderResponse.state`; the old name is not
  kept. The OpenAPI file is regenerated.
recommendation: >-
  The design's own context says "partners' integrations read `status` to know
  when to ship"; on the day this deploys, every partner reading
  `GET /orders/{id}` today (`src/orders.ts:19` builds `status: rows[0].state`,
  `openapi.yaml:18` lists `required: [id, status]`) gets no `status` field,
  reads undefined and stops shipping orders, with no warning. Use expand and
  contract: return both `status` and `state` with the same value in this
  change, mark `status` `deprecated: true` in `openapi.yaml`, and remove it
  in a later change after a stated deprecation period. Reflect it in the
  spec (a requirement or scenario that `status` is still returned, equal to
  `state`, while deprecated) and in tasks.md (a task that keeps `status`,
  verified by a test that a paid order holds both `"status": "paid"` and
  `"state": "paid"`). The alternative rejected as "duplication" is the
  migration path, not a cost to avoid.
history:
  - status: open
    by: migration
```

#### R-migration-2 Breaking rename not marked BREAKING, no action for consumers

```yaml
severity: high
confidence: high
location: "proposal.md#Release note"
evidence: |
  - Changed: The order's `status` field is now called `state`.
recommendation: >-
  A partner reading the release note sees an ordinary "Changed" item, not a
  break, and is not told to act, so their integration fails in production
  when the field disappears. Mark the item **BREAKING** (also in
  `## What Changes`), and say what consumers must do and by when: read
  `state` instead of `status`, `status` kept until the announced removal
  date (see R-migration-1).
history:
  - status: open
    by: migration
```
