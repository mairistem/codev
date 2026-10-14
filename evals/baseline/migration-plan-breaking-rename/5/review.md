# Review record: rename-order-status

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### migration

#### R-migration-1 The public `status` field is removed with no deprecation period

```yaml
severity: high
confidence: high
location: "design.md#Decision: The field is renamed in place"
evidence: |
  `OrderResponse.status` becomes `OrderResponse.state`; the old name is not
  kept. The OpenAPI file is regenerated.
recommendation: >-
  The design's own context says "partners' integrations read `status` to know
  when to ship": on the day this ships, every partner integration calling
  `GET /orders/{id}` reads `status` as undefined (it is also `required` in
  `openapi.yaml`, so schema-validating clients reject the response) and stops
  shipping orders, with no warning. Use expand then contract: return both
  `state` and `status` (marked `deprecated: true` in `openapi.yaml`) for a
  stated deprecation period, then remove `status` in a later change. Add a
  requirement and a scenario to `specs/orders-api/spec.md` that `status` is
  still returned with the same value during that period, and a task in
  `tasks.md` that tests it — today the only tasks and scenarios cover `state`
  alone, so no migration path is tested.
history:
  - status: open
    by: migration
```

#### R-migration-2 The release note does not mark the rename as BREAKING nor say what partners must do

```yaml
severity: high
confidence: high
location: "proposal.md#Release note"
evidence: |
  - Changed: The order's `status` field is now called `state`.
recommendation: >-
  Renaming a field of the public `GET /orders/{id}` response breaks every
  consumer that reads `status`, yet the note reads as an ordinary change, so
  partners reading it get no signal to act before their integrations fail.
  Mark the item **BREAKING**, state what consumers must do (read `state`
  instead of `status`) and by when the old field disappears if a deprecation
  period is kept (see R-migration-1).
history:
  - status: open
    by: migration
```

### skeptic

Disabled by configuration.
