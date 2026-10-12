# Proposal: <change title>

## Why

<!-- The problem or opportunity, in one or two sentences. Why now? -->

## What Changes

<!-- Bullet list, specific about the capabilities added, modified or removed.
     Mark any breaking change with **BREAKING**. -->

## Capabilities

### New Capabilities

<!-- One line per capability, in the form `path/of-the-capability`. Each one
     produces a `specs/<path>/spec.md` file. Leave empty if none. -->

### Modified Capabilities

<!-- One line per capability whose REQUIREMENTS change, with its exact path
     under `_codev/specs/`. Leave empty if none. -->

### Removed Capabilities

<!-- One line per capability this change removes entirely. Each entry is the
     exact path under `_codev/specs/`. Requires `retire_capabilities: true`
     in `change.yaml` and a `## REMOVED Requirements` section that empties
     the spec. Without the marker, sync/archive refuses rather than perform
     an irreversible operation. Leave empty if none. -->

## Impact

<!-- Affected code, APIs, dependencies and systems. -->

## Domains

<!-- Tick every domain the change touches: its expert reviews the plan and
     the code. Write the single line `None` if it touches none of them. -->

- [ ] security
- [ ] personal-data
- [ ] user-interface
- [ ] performance
- [ ] operations
- [ ] migration

## Release note

<!-- What end users will read in the changelog. One bullet per item:
     `- <Category>: <one or two sentences>`, Category among Added, Changed,
     Deprecated, Removed, Fixed, Security; start a breaking item's text with
     **BREAKING**. A purely internal change writes the single line `None`. -->
