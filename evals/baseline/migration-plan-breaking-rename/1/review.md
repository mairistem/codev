# Review record: rename-order-status

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### skeptic

Disabled by configuration.

### migration

#### R-migration-1 Public field removed in place, with no deprecation or side-by-side period

```yaml
severity: high
confidence: high
location: "design.md#Decision: The field is renamed in place"
evidence: |
  `OrderResponse.status` becomes `OrderResponse.state`; the old name is not
  kept. The OpenAPI file is regenerated.
recommendation: >-
  design.md#Context states that partners' integrations read `status` from the
  public `GET /orders/{id}` to know when to ship (src/orders.ts:7 and
  openapi.yaml:18 `required: [id, status]` today). On deploy, every partner
  integration reads a missing `status` and stops shipping orders, with no
  warning. Use expand and contract: return both `status` and `state` with the
  same value for a deprecation period (mark `status` `deprecated: true` in
  openapi.yaml), announce a removal date, and remove `status` in a later
  change. Add a requirement and scenario to specs/orders-api/spec.md that the
  response still holds `status` during the period, and a task in tasks.md
  with a test that covers it. The "duplication" alternative rejected in the
  design is exactly this path and should be reconsidered.
history:
  - status: open
    by: migration
```

#### R-migration-2 Breaking rename not marked BREAKING, release note says nothing about what partners must do

```yaml
severity: high
confidence: high
location: "proposal.md#Release note"
evidence: |
  - Changed: The order's `status` field is now called `state`.
recommendation: >-
  Partners reading the release note see an ordinary "Changed" item and are not
  told their integration breaks or what to do. Mark the item **BREAKING**,
  state that clients must read `state` instead of `status` (and, with
  R-migration-1, until which date `status` is still served), and mark the same
  item in proposal.md#What Changes.
history:
  - status: open
    by: migration
```
