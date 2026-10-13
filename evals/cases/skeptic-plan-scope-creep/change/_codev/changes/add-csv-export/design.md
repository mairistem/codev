# Design: Invoice export

## Decisions

### Decision: Exports go through a plugin registry

`src/export/registry.ts` discovers modules in `src/export/plugins/` at
start-up; each exports `{ format, mimeType, render(invoices) }`. CSV is the
first plugin. A settings page stores each user's preferred format.

**Alternatives considered**: a single CSV function (not extensible).
