# Review record: rename-order-status

<!-- codev-review: 1 -->

## plan.challenge

### migration

#### R-migration-1 Public field removed with no deprecation period

```yaml
severity: high
confidence: high
location: "design.md#Decision: The field is renamed in place"
evidence: |
  `OrderResponse.status` becomes `OrderResponse.state`; the old name is not
  kept. The OpenAPI file is regenerated.
recommendation: >-
  Use expand and contract: return both `status` and `state` (same value) and
  mark `status` deprecated in openapi.yaml with a removal date, then remove
  `status` in a later change once partners have moved. Add a scenario saying
  the response still holds `status` during the deprecation period, and a
  task with a test for it.
history:
  - status: open
    by: migration
```

The design states the consumer: "`GET /orders/{id}` is public: partners'
integrations read `status` to know when to ship." After the rename, a partner
integration calling `GET /orders/42` on a paid order reads `status`, gets
`undefined`, and never ships — with no error to warn it. The rejected
alternative ("keeping both fields (duplication)") is exactly the parallel
change that would keep those consumers working.

#### R-migration-2 Breaking rename not marked BREAKING in the release note

```yaml
severity: medium
confidence: high
location: "proposal.md#Release note"
evidence: |
  - Changed: The order's `status` field is now called `state`.
recommendation: >-
  Mark the item **BREAKING** and say what partners must do: read `state`
  instead of `status` on `GET /orders/{id}`, and by when if a deprecation
  period is kept.
history:
  - status: open
    by: migration
```

Partners reading the release note see a "Changed" item with no action asked
of them; nothing tells them their shipping integration stops working.
