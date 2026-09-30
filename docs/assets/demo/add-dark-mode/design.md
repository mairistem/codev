# Design: Add a dark mode

## Context

Colors are hard-coded in the stylesheet; preferences are stored per user.

## Goals / Non-Goals

**Goals:** a dark theme, system default, remembered choice.

**Non-Goals:** custom color palettes.

## Decisions

### Decision: CSS custom properties for colors

One set of variables per theme, switched by a `data-theme` attribute on the
root element.

## Risks / Trade-offs

- [Third-party widgets ignore the variables] → Style them explicitly.

## Migration Plan

None.

## Open Questions

None.
