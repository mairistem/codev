# Review record: rename-order-status

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### skeptic

Disabled by configuration.

### migration

#### R-migration-1 The field is removed in place, with no deprecation or side-by-side period for partners

```yaml
severity: high
confidence: high
location: "design.md#Decision: The field is renamed in place"
evidence: |
  `OrderResponse.status` becomes `OrderResponse.state`; the old name is not
  kept. The OpenAPI file is regenerated.
recommendation: >-
  Use expand and contract. Ship `state` next to `status` (both in
  `OrderResponse` and in `openapi.yaml`, with `status` marked
  `deprecated: true`), announce a removal date, and remove `status` only
  in a later change once partners have moved. Add a scenario to
  `specs/orders-api/spec.md` and a test in `tasks.md` showing that a paid
  order still holds `"status": "paid"` during the deprecation period, so
  the migration path is tested. Revisit the rejected alternative
  ("keeping both fields (duplication)"): the duplication is temporary and
  is what keeps existing integrations working.
history:
  - status: open
    by: migration
```

Scenario: a partner integration calls `GET /orders/{id}` today and reads
`status` (design.md: "partners' integrations read `status` to know when to
ship"; openapi.yaml lists `required: [id, status]`). After deploy, the
response holds only `state`; the partner reads `undefined`, never sees
`paid`, and stops shipping orders — with no warning and no window to adapt.

#### R-migration-2 The breaking rename is not marked BREAKING and the release note says nothing of what partners must do

```yaml
severity: high
confidence: high
location: "proposal.md#Release note"
evidence: |
  - Changed: The order's `status` field is now called `state`.
recommendation: >-
  Mark the item **BREAKING** in `## Release note` and in `## What Changes`,
  and say what consumers must do: read `state` instead of `status` on
  `GET /orders/{id}`, by which date the old field disappears (per
  R-migration-1), and that `openapi.yaml`'s `Order` schema changes its
  required field. Also list `orders-api` under Modified rather than New
  Capabilities if the endpoint already exists for consumers.
history:
  - status: open
    by: migration
```

Scenario: a partner reads the release note, sees a plain "Changed:" line
with no **BREAKING** marker and no action, does not update the integration,
and discovers the rename only when order shipping fails in production.
