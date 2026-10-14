# Review record: rename-order-status

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### migration

#### R-migration-1 Public field removed in the same release, with no deprecation window

```yaml
severity: high
confidence: high
location: "design.md#Decision: The field is renamed in place"
evidence: |
  `OrderResponse.status` becomes `OrderResponse.state`; the old name is not
  kept. The OpenAPI file is regenerated.
recommendation: >-
  Use expand then contract. In this change, `GET /orders/{id}` returns both
  `state` and `status` with the same value. Mark `status` `deprecated: true`
  in `openapi.yaml` and take it out of `required: [id, status]`. Announce a
  removal date, and remove `status` in a later change. Add a spec scenario
  "A paid order still exposes status during deprecation"
  (`"status": "paid"` next to `"state": "paid"`) and a task that tests it.
  Today the design calls the dual field "duplication" and rejects it,
  although a partner integration cannot change at the moment the API does.
history:
  - status: open
    by: migration
```

Today `src/orders.ts:19` builds `{ id: rows[0].id, status: rows[0].state }`.
The design says partners "read `status` to know when to ship" (`design.md#Context`).
Once this ships, their integrations read `response.status` and get `undefined`.
A partner that waits for `status === "shipped"` or `"paid"` never ships, and
gets no error. A client generated from the current `openapi.yaml`, which
requires `status`, rejects the response.

#### R-migration-2 Breaking rename not marked BREAKING, no action for consumers

```yaml
severity: high
confidence: high
location: "proposal.md#Release note"
evidence: |
  - Changed: The order's `status` field is now called `state`.
recommendation: >-
  Mark the item **BREAKING** in the release note and in `## What Changes`.
  Say what partners must do: read `state` instead of `status`, before the
  removal date. If R-migration-1 is fixed, say also that `status` stays
  deprecated until then. Add a task to publish the note, and to bump the
  API version in `openapi.yaml` (`info.version: 1.4.0`) so that the
  contract change is visible.
history:
  - status: open
    by: migration
```

The rename removes a field from a public response, `GET /orders/{id}`.
Partners who read this note see only a neutral "Changed" item. Nothing
tells them their integration will break, or what to change.

### skeptic

Disabled by configuration.
