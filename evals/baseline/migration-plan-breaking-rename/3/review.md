# Review record: rename-order-status

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### skeptic

Disabled by configuration.

### migration

#### R-migration-1 Public field removed in place, with no transition for partner integrations

```yaml
severity: high
confidence: high
location: "design.md#Decision: The field is renamed in place"
evidence: |
  `OrderResponse.status` becomes `OrderResponse.state`; the old name is not
  kept. The OpenAPI file is regenerated.
recommendation: >-
  Partners' integrations read `status` from `GET /orders/{id}` "to know when
  to ship" (design.md#Context); on the day this deploys, `status` is absent
  from the response (src/orders.ts:19 today sets `status: rows[0].state`), so
  their code reads `undefined` and stops shipping orders, with no error from
  the API. Use expand and contract: return both `status` and `state` with the
  same value, mark `status` `deprecated: true` in openapi.yaml, announce a
  removal date, and remove `status` only in a later change. Add a task and a
  scenario that check a response still holds `"status": "paid"` during the
  deprecation window, so the migration path is tested. Revisit the rejected
  alternative: the duplication is temporary and is the migration path.
history:
  - status: open
    by: migration
```

#### R-migration-2 Release note does not mark the rename BREAKING nor say what partners must do

```yaml
severity: high
confidence: high
location: "proposal.md#Release note"
evidence: |
  - Changed: The order's `status` field is now called `state`.
recommendation: >-
  The rename breaks every consumer of the public `GET /orders/{id}` response,
  yet the note files it as a plain "Changed" item. A partner reading it does
  not learn that their integration breaks nor what to do. Mark it
  **BREAKING**, name the endpoint, and state the action: read `state`
  instead of `status`, by the removal date if a deprecation window is kept
  (see R-migration-1). Bump the API's major version in openapi.yaml
  (`info.version: 1.4.0`) if the project versions its public API
  semantically.
history:
  - status: open
    by: migration
```
