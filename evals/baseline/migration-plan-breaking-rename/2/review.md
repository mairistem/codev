# Review record: rename-order-status

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### skeptic

Disabled by configuration.

### migration

#### R-migration-1 Public field removed with no deprecation or transition period

```yaml
severity: high
confidence: high
location: "design.md#Decision: The field is renamed in place"
evidence: |
  `OrderResponse.status` becomes `OrderResponse.state`; the old name is not
  kept. The OpenAPI file is regenerated.
recommendation: >-
  Use expand and contract: return both `status` and `state` (same value) for a
  stated deprecation period, mark `status` deprecated in `openapi.yaml`, and
  remove it only in a later change once partners have moved. Add a task and a
  scenario that check a response still holds `status` during the transition,
  so the migration path is tested. The rejected alternative "keeping both
  fields (duplication)" is the compatibility path the public contract needs.
history:
  - status: open
    by: migration
```

Failure scenario: `GET /orders/{id}` is public and, per `design.md#Context`,
"partners' integrations read `status` to know when to ship". The day this
ships, a partner integration reading `order.status` gets `undefined` for a
paid order, never sees `paid`, and stops shipping orders — with no warning and
no window to update its code.

#### R-migration-2 Breaking rename not marked BREAKING in the release note

```yaml
severity: high
confidence: high
location: "proposal.md#Release note"
evidence: |
  - Changed: The order's `status` field is now called `state`.
recommendation: >-
  Mark the item **BREAKING**, state what partners must do (read `state`
  instead of `status`, and by when the old field disappears if a deprecation
  period is adopted), and name the affected endpoint `GET /orders/{id}`. Bump
  the API's major version if it follows semantic versioning.
history:
  - status: open
    by: migration
```

Failure scenario: a partner reading the release note sees a "Changed" item
phrased as a rename of wording, not as a removal of the field their
integration reads; nothing tells them their code breaks or what to change, so
they find out when orders stop shipping.
