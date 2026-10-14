# Review record: rename-order-status

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### migration

#### R-migration-1 The public field rename is not marked BREAKING and the release note gives users nothing to do

```yaml
severity: high
confidence: high
location: "proposal.md#Release note"
evidence: |
  - Changed: The order's `status` field is now called `state`.
recommendation: >-
  Mark the item **BREAKING** in the release note and say what partners must
  do: read `state` instead of `status` from `GET /orders/{id}`, with the date
  or version after which `status` is gone. Today a partner integration that
  reads `status` to know when to ship (design.md#Context) gets `undefined`
  after deploy, with no warning in the note it reads, and stops shipping
  orders. The API version (openapi.yaml `version: 1.4.0`) should move with
  the announcement so consumers can tell which contract they are on.
history:
  - status: open
    by: migration
```

#### R-migration-2 The rename removes `status` in one step, with no compatibility period for partner integrations

```yaml
severity: high
confidence: high
location: "design.md#Decision: The field is renamed in place"
evidence: |
  `OrderResponse.status` becomes `OrderResponse.state`; the old name is not
  kept. The OpenAPI file is regenerated.
recommendation: >-
  Use expand then contract: in this change, return both `state` and
  `status` (same value), mark `status` as `deprecated: true` in
  openapi.yaml, and announce its removal date; remove `status` in a later
  change once partners have moved. The design's own context says partners'
  integrations read `status` to know when to ship, and `src/orders.ts:19`
  (`status: rows[0].state`) is what they read today: a big-bang rename
  breaks every one of them at deploy, with no window to migrate. Add a task
  and a spec scenario checking that a paid order still exposes
  `"status": "paid"` during the deprecation period, so the migration path
  is tested; tasks.md 1.1 only tests the new field.
history:
  - status: open
    by: migration
```

### skeptic

Disabled by configuration.
