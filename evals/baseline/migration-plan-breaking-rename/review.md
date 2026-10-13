# Review record: rename-order-status

<!-- codev-review: 1 -->

## plan.challenge

### migration

#### R-migration-1 The breaking rename of a public field is not marked BREAKING

```yaml
severity: high
confidence: high
location: "proposal.md#Release note"
evidence: |
  - Changed: The order's `status` field is now called `state`.
recommendation: Mark the item **BREAKING** in the release note and say what partners must do (read `state` instead of `status`, by which date or version), so the change is announced as a breaking API change rather than a plain "Changed" item.
history:
  - status: open
    by: migration
```

#### R-migration-2 The field is removed in the same release that introduces its replacement, with no deprecation

```yaml
severity: high
confidence: high
location: "design.md#Decision: The field is renamed in place"
evidence: |
  `OrderResponse.status` becomes `OrderResponse.state`; the old name is not
  kept. The OpenAPI file is regenerated.
recommendation: Use expand and contract — return both `status` (marked deprecated in `openapi.yaml`) and `state` for a stated deprecation window, then remove `status` in a later change. The design's own context says partners read `status` to know when to ship, so removing it at once breaks their integrations without warning; "duplication" alone does not justify rejecting the side-by-side alternative.
history:
  - status: open
    by: migration
```

#### R-migration-3 No migration path for existing consumers is planned or tested

```yaml
severity: medium
confidence: high
location: "tasks.md#1. Rename"
evidence: |
  - [ ] 1.1 Rename the field and regenerate `openapi.yaml`, verified by tests "A paid order" and "An unknown order"
recommendation: Add a task and a spec scenario covering existing consumers — e.g. a response still carries `status` with the same value as `state` during the deprecation window — and a task to notify partners; the only tests planned check the new field name.
history:
  - status: open
    by: migration
```
