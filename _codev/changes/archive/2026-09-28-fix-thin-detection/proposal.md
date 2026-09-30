# Proposal: "thin" detection looks only at the rules

## Why

The `configure-skill-with-nudge` batch introduced the function
`is_config_thin(context, rules_empty) -> bool` with a threshold on the
length of `context:` (200 characters). The intent was:
"a long context OR rules being present mean that the user has filled
in their config".

Tested on a real project, **mira** (TypeScript, ~10 dependencies), the
observed behavior breaks that intent:

- `codev init` detects the stack and writes a context of **280
  characters** — made up solely of the list of detected dependencies
  and "CI GitHub Actions active".
- `is_config_thin` returns `false` — the 200 threshold is crossed.
- **No hint appears**, even though the config has in fact received no
  human contribution at all and `/codev-configure` has exactly
  something to offer.

The threshold on the context is the wrong criterion. It does not
distinguish "context auto-detected by the probe" from "context written
by the user". Yet only the latter reflects an intent.

## What Changes

**Simplification**: `is_config_thin` takes **a single primitive** —
`rules_empty: bool` — and returns exactly `rules_empty`. The signature
becomes:

```rust
pub fn is_config_thin(rules_empty: bool) -> bool
```

**Rationale**: per-artifact `rules:` are always a user choice — never
auto-detected, never filled in by the probe. Their presence is a
reliable, binary indicator:

- empty `rules:` → the user has not filled in their config yet → the
  hint is useful → thin.
- non-empty `rules:` → the user took the trouble to write at least one
  rule → their config is no longer blank → not thin.

The `context:` field is ignored because, without a marker, what comes
from the probe cannot be told apart from what comes from the user —
and we do not want to add a marker for a signal that remains
secondary.

**Observable effect**: on a project that has not yet been enriched by
`/codev-configure` (or by hand), the hint appears **systematically** —
in `codev init`, in `codev status`, in `/codev-onboard`. As soon as a
rule is written (by `configure` or by hand), the hint disappears.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- **`configure`** — a sentence of the `## Purpose` mentioned the
  context threshold; to be removed from the skill body only (not from
  the spec, which does not mention it).
- **`init`** — the Requirement "prompts for `/codev-configure` when the
  generated config is thin" and its scenario mention "context < 200
  characters AND empty rules". To be rewritten to talk only about an
  empty `rules:` key.
- **`skills`** — the Requirement "Skill `onboard` presents codev"
  mentions "context < 200 chars, empty rules". To be rewritten.

### Removed Capabilities

None.

## Impact

- **Code**:
  - `codev-core::config::is_config_thin` — simplified signature.
  - Unit tests — 3 cases rewritten.
  - Three call sites simplified:
    - `codev-cli::commands::install_skills` (in the computation of
      `SetupOutcome.config_thin`).
    - `codev-cli::main::config_is_thin` (helper for the `codev status`
      hint).
    - No other — the `onboard` skill reads the YAML directly, not the
      Rust function.
  - `assets/workflows/onboard.md` — the mention of the "context < 200
    chars" threshold removed; only the empty `rules:` check remains.
  - `assets/workflows/configure.md` — does not mention the threshold,
    but check that nothing implicitly depends on the new definition.
- **JSON contract**: nothing changes. `is_config_thin` does not appear
  in the contract.
- **Backward compat**: perfect. Existing configs simply see a stricter
  evaluation of "thin" — in the sense that "a config with rules is no
  longer thin". A config without rules stays thin (as before). What
  changes: a config without rules but with a long auto-detected
  context becomes thin again (that was the bug).
- **Migration**: none.
- **Out of scope**:
  - A `# written by /codev-configure` marker — deferred; the
    `rules_empty`-only simplification is enough to fix the observed
    symptom.
  - Introducing a `--force-nudge` flag mode — not useful, the user can
    always ignore the hint.
