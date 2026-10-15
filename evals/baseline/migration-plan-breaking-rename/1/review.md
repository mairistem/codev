# Review record: rename-order-status

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### migration

#### R-migration-1 Public field removed in place, with no deprecation or compatibility period

```yaml
severity: high
confidence: high
location: "design.md#Decision: The field is renamed in place"
evidence: |
  `OrderResponse.status` becomes `OrderResponse.state`; the old name is not
  kept. The OpenAPI file is regenerated.
recommendation: >-
  Use expand and contract. In this change, return both `state` and `status`
  (same value), mark `status` deprecated in `openapi.yaml`, and add a scenario
  to `specs/orders-api/spec.md` stating that `status` is still returned during
  the deprecation period. Add a test task to `tasks.md` that checks an existing
  consumer reading `status` still gets `"paid"`. Remove `status` in a later
  change, after partners have been notified and given a removal date. The
  "duplication" listed under Alternatives considered is the cost of that
  compatibility period, not a reason to reject it.
history:
  - status: open
    by: migration
```

Scenario: the design's own context says partners' integrations read `status`
from the public `GET /orders/{id}` "to know when to ship". When the change
is deployed, a partner integration reading `body.status` for a paid order
gets `undefined` (the field is no longer in `src/orders.ts:19`'s response).
Its check for `"paid"` never matches, so it stops shipping orders without
any error.

#### R-migration-2 Breaking rename not marked BREAKING in the release note, with no action for consumers

```yaml
severity: high
confidence: high
location: "proposal.md#Release note"
evidence: |
  - Changed: The order's `status` field is now called `state`.
recommendation: >-
  Mark the item **BREAKING** and tell consumers what to do. For example:
  "**BREAKING** (when `status` is removed): read `state` instead of `status`
  in `GET /orders/{id}`; `status` is deprecated and will be removed on
  <date>." Mention the rename and the deprecation in "What Changes" as well.
history:
  - status: open
    by: migration
```

Scenario: a partner who reads the release note before the deploy sees a
plain "Changed" item. Nothing tells them that their integration breaks or
that they must switch to `state`. They find out only when shipping stops
(see R-migration-1).

### skeptic

Disabled by configuration.
