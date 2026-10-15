# Review record: rename-order-status

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### skeptic

Disabled by configuration.

### migration

#### R-migration-1 Public field removed in place with no deprecation or side-by-side period

```yaml
severity: high
confidence: high
location: "design.md#Decision: The field is renamed in place"
evidence: |
  `OrderResponse.status` becomes `OrderResponse.state`; the old name is not
  kept. The OpenAPI file is regenerated.
recommendation: >-
  Use expand and contract: return both `status` and `state` with the same
  value, mark `status` deprecated in `openapi.yaml` with a removal date, and
  remove it in a later change once partners have moved. Add a task and a
  scenario that check `GET /orders/{id}` still holds `"status": "paid"`
  alongside `"state": "paid"` during the deprecation period, so the
  migration path is tested.
history:
  - status: open
    by: migration
```

The design's own context says `GET /orders/{id}` is public and that
"partners' integrations read `status` to know when to ship". On the release
that ships this change, a partner integration reading `status` from a paid
order gets `undefined`/a missing key, and stops shipping orders — with no
warning, since the old field disappears in the same release that introduces
its replacement. The only alternative considered ("keeping both fields")
was dismissed as duplication, without weighing the consumers.

#### R-migration-2 Breaking rename not marked BREAKING and no action given to users

```yaml
severity: high
confidence: high
location: "proposal.md#Release note"
evidence: |
  - Changed: The order's `status` field is now called `state`.
recommendation: >-
  Mark the item **BREAKING** in the release note and in `## What Changes`,
  and say what partners must do: read `state` instead of `status`, by which
  date or version `status` is removed, and that both are served until then
  (see R-migration-1). If the API is versioned, bump its major version or
  announce the change in the API changelog partners follow.
history:
  - status: open
    by: migration
```

A partner reading the release note sees an ordinary "Changed" entry, not a
breaking one, and is given no step to take; they find out when their
integration stops reading the order's lifecycle in production.
