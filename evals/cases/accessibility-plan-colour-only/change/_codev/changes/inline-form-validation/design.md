# Design: Inline form validation

## Context

The checkout form (`web/src/checkout/Form.tsx`) validates on submit only and
lists errors at the top of the page.

## Decisions

### Decision: Invalid fields turn red

Each field validates on `blur`. An invalid field gets the class
`field--invalid`, a red border (`#e5484d`); the list of errors at the top of
the page is removed, since the red border already shows where the problem
is. No message text is added, to keep the form compact.

**Alternatives considered**: a message under each field (takes vertical
space on mobile).

### Decision: Rules live next to the form

Validation rules are plain functions in `web/src/checkout/rules.ts`.

**Alternatives considered**: a schema library (one more dependency).
