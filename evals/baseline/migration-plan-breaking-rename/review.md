# Review record: rename-order-status

<!-- codev-review: 1 -->

## plan.challenge

### migration

#### R-migration-1 Removing the public `status` field breaks partner integrations with no deprecation

```yaml
severity: high
confidence: high
location: "design.md#Decision: The field is renamed in place"
evidence: |
  `OrderResponse.status` becomes `OrderResponse.state`; the old name is not
  kept. The OpenAPI file is regenerated.
recommendation: Apply expand and contract — return both `status` and `state` for a deprecation period, mark `status` deprecated in openapi.yaml, and remove it only in a later change once partners have moved; record the dated removal plan in the design.
history:
  - status: open
    by: migration
```

The design's own context says the field has external consumers that act on it
(`partners' integrations read `status` to know when to ship`), yet the old
name is dropped in the same release that introduces the new one. The only
alternative considered ("keeping both fields (duplication)") is dismissed
without weighing the consumer impact: from the deploy onward every partner
reads `undefined` and stops shipping, with no warning.

#### R-migration-2 The breaking rename is not marked BREAKING nor tells users what to do

```yaml
severity: high
confidence: high
location: "proposal.md#Release note"
evidence: |
  - Changed: The order's `status` field is now called `state`.
recommendation: Mark the item **BREAKING** in the release note and in What Changes, and say what consumers must do (read `state` instead of `status`, by which date the old field disappears).
history:
  - status: open
    by: migration
```

`proposal.md#What Changes` (`- The order JSON's `status` field is renamed `state`.`)
carries no **BREAKING** marker either.

#### R-migration-3 No rollout order keeping old and new consumers working side by side

```yaml
severity: high
confidence: high
location: "tasks.md#1. Rename"
evidence: |
  - [ ] 1.1 Rename the field and regenerate `openapi.yaml`, verified by tests "A paid order" and "An unknown order"
recommendation: Split the work into ordered tasks — add `state` alongside `status` and deprecate `status` (with a test that both fields are returned), announce to partners, then a separate later task or change that removes `status`.
history:
  - status: open
    by: migration
```

A single task ships the rename as a big-bang step; no task covers a transition
period, a test of the migration path, or partner communication. The spec
likewise only requires `state` (`spec.md#Requirement: Orders expose their state`)
and has no scenario for clients still reading `status`.

#### R-migration-4 The API version is not bumped for a breaking change

```yaml
severity: medium
confidence: high
location: "openapi.yaml:4"
evidence: |
  version: 1.4.0
recommendation: Plan a major version bump (2.0.0) of the OpenAPI `info.version` and `package.json` when `status` is removed, or keep the release minor by only adding `state` and deprecating `status`.
history:
  - status: open
    by: migration
```

Removing a required property (`required: [id, status]`, openapi.yaml:18) is a
major change under semantic versioning, and neither the design nor the tasks
mention a version change.

### skeptic

Disabled by configuration.
