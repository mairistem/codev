# Review record: rename-order-status

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### migration

#### R-migration-1 Public field rename removes `status` with no deprecation or side-by-side period

```yaml
severity: high
confidence: high
location: "design.md#Decision: The field is renamed in place"
evidence: |
  `OrderResponse.status` becomes `OrderResponse.state`; the old name is not
  kept. The OpenAPI file is regenerated.
recommendation: >-
  Use expand and contract: return both `status` and `state` (same value) for a
  stated deprecation period, mark `status` as deprecated in `openapi.yaml`,
  and remove it only in a later change once partners have moved. Add a spec
  scenario that a paid order still holds `"status": "paid"` during the
  deprecation window, and a task with a test verifying it. If removal in one
  step is kept, the design must say why partners can be cut over at once and
  how they are coordinated (alternatively, a versioned endpoint).
history:
  - status: open
    by: migration
```

The design's own context says `GET /orders/{id}` is public and that
"partners' integrations read `status` to know when to ship". On deploy, a
partner integration reading `order.status` gets the field missing (undefined /
null) for every order, so its "is paid → ship" check never matches: orders stop
shipping with no error raised. The only alternative considered, keeping both
fields, is dismissed as "duplication", which is the standard migration path
rather than a defect.

#### R-migration-2 Breaking rename not marked BREAKING and the release note gives partners no action

```yaml
severity: high
confidence: high
location: "proposal.md#Release note"
evidence: |
  - Changed: The order's `status` field is now called `state`.
recommendation: >-
  Mark the item **BREAKING** in the proposal's What Changes and release note,
  and state what consumers must do: read `state` instead of `status`, by which
  date or version `status` disappears, and that the values (`open`, `paid`,
  `shipped`, `cancelled`) are unchanged. Announce it to partners ahead of the
  release.
history:
  - status: open
    by: migration
```

`proposal.md#What Changes` likewise lists "- The order JSON's `status` field is
renamed `state`." with no **BREAKING** marker. A partner reading the release
note sees a "Changed" item with no instruction and no deadline, and learns of
the break only when its shipping integration stops working.
