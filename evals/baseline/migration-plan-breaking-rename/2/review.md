# Review record: rename-order-status

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### skeptic

Disabled by configuration.

### migration

#### R-migration-1 Public field removed in place, no compatibility period for partners

```yaml
severity: high
confidence: high
location: "design.md#Decision: The field is renamed in place"
evidence: |
  `OrderResponse.status` becomes `OrderResponse.state`; the old name is not
  kept. The OpenAPI file is regenerated.
recommendation: >-
  The design's own context says "partners' integrations read `status` to know
  when to ship": on the deploy that ships this, every partner reading
  `response.status` from `GET /orders/{id}` gets `undefined` and stops
  shipping, with no warning (today `src/orders.ts:19` sends `status`, and
  `openapi.yaml:18` lists it as required). Use expand and contract: return
  both `status` and `state` with the same value, mark `status` as
  `deprecated: true` in `openapi.yaml` (and drop it from `required` only when
  it is removed), announce a removal date, and remove `status` in a later
  change once partners have moved. Add a task and a scenario checking that a
  paid order holds both `"status": "paid"` and `"state": "paid"` during the
  deprecation period. The rejected alternative "keeping both fields
  (duplication)" is exactly this migration path; its cost is temporary.
history:
  - status: open
    by: migration
```

#### R-migration-2 Breaking rename not marked BREAKING, release note gives no action

```yaml
severity: medium
confidence: high
location: "proposal.md#Release note"
evidence: |
  - Changed: The order's `status` field is now called `state`.
recommendation: >-
  Renaming a field of the public order JSON breaks partners' integrations,
  yet the release note files it under "Changed" with no **BREAKING** marker
  and does not tell consumers what to do. A partner reading the note cannot
  tell that their integration needs an update, nor by when. Mark the item
  **BREAKING** (or, with R-migration-1's fix, announce `status` as
  deprecated with its removal date) and state the action: read `state`
  instead of `status` in `GET /orders/{id}` responses. Also mark the item
  in proposal.md's `## What Changes` as **BREAKING**.
history:
  - status: open
    by: migration
```
