# Proposal: Add a dark mode

## Why

Users working at night find the interface too bright, and several asked for a
dark theme.

## What Changes

- The interface follows the operating system's color scheme by default.
- Users can pick a theme explicitly, and the choice is remembered.

## Capabilities

### New Capabilities

- `ui/theme` — choosing and remembering the color theme.

### Modified Capabilities

None.

### Removed Capabilities

None.

## Impact

- Front-end theme provider and settings page.
- User preferences API: one new field, `theme`.
