# Design: `codev validate --strict`

## Context

See `proposal.md`. A small change with high value for automation — the
core barely changes; the public API gains a flag and a field.

## Goals / Non-Goals

This design covers the flag's behavior, the exit-code logic, the JSON
signature and the human rendering. It does not cover `--archived`
(E6), `--strict-level` (configurable), nor a `--fix` mode.

## Decisions

### Decision: strict mode changes the exit code, not the severities

Two options:

| Option | Pro | Con |
|---|---|---|
| **A. Promote warnings to errors in the report** | Human and JSON renderings directly reflect the verdict | Misleading for a reader who would see "error" on a `decision_unsealed` just because the CI passed `--strict` |
| **B. Change only the exit code** | The report says what is; strict mode says what it triggers | The nuance has to be explained in the docs |

**Chosen: B.** A finding has an intrinsic severity — the fact that a
caller treats it as an error belongs to its contract, not to the
nature of the finding. This also aligns the behavior with classic
linters (clippy `--deny warnings`), which leave warnings as warnings
in the output and just change the return code.

### Decision: `has_warnings()` method on `ValidateReport`, like `has_errors()`

Consistent with the existing method. The CLI composes:

```rust
let exit_code = if strict {
    if report.has_errors() || report.has_warnings() { 1 } else { 0 }
} else if report.has_errors() {
    1
} else {
    0
};
```

Or shorter, a dedicated method `is_fail(strict: bool)` — but the
literal expression stays more readable in a CLI that already has six
similar branches. I keep the computation inline.

### Decision: the JSON field is named `hasWarnings`, additive

Naming consistent with the rest of the contract (`hasErrors` implicit
via `has_errors()` — but **not** in the JSON contract today: `items[]`
and `findings[]` are enough). Add `hasWarnings` only — a `hasErrors`
field would be redundant (the consumer who parses `findings[]`
already knows).

**Rejected alternative**: add both, `hasErrors` and `hasWarnings`,
for symmetry. Cost: one more field, a potential source of divergence
if `has_errors()` and the content of `findings` say different things.
Refused.

### Decision: `--strict` applies to all forms of `validate`

The flag is global to the subcommand, not restricted to `--all`. A
user who validates **one** item with `--strict` must get the same
exit rule. The CLI places `--strict` on `Command::Validate` (not on a
variant), no dispatching.

## Risks / Trade-offs

- **A caller migrates from "exit 0 → OK" to "exit 0 → OK unless
  strict"** — not a risk: current callers do not pass `--strict`, so
  their exit code does not move. The flag is opt-in.
- **A JSON consumer confuses `hasWarnings` with the verdict** — the
  field is documented as informational. The real verdict is the exit
  code. This is explained in the spec and recalled in the CLI docs.

## Migration Plan

None. The flag is opt-in; the JSON field is additive.
