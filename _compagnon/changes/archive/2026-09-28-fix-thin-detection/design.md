# Design: fix for thin detection

## Context

See `proposal.md`. Bug observed on a real TypeScript project (mira):
the hint does not appear even though no rule has been written, because
the probe filled in a `context:` of 280 characters.

## Decisions

### Decision: `is_config_thin(rules_empty: bool)` — signature reduced to a single parameter

The `context: Option<&str>` parameter disappears. The function returns
strictly `rules_empty`.

This is a deliberate weakening: we give up the nuance "a long context
counts as configured" because it produces false negatives on projects
with a rich stack (TypeScript, Java Maven, any project with more than
5-6 declared dependencies).

In practice, the weakening is neutral:

- A user who has the time to write a detailed `context:` will almost
  always also write at least one rule — the "long context, empty
  rules" case is rare and often unintentional.
- A user who has written neither context nor rules loses nothing by
  receiving the hint a second time — they dismiss it at a glance.

**Rejected alternative A**: bump the threshold from 200 to 1000
characters. Rejected — it pushes the problem back, it does not solve
it. A polyglot project can exceed 1000 chars of detected deps.

**Rejected alternative B**: distinguish auto-detected context from
user context via a marker (`# written by /codev-configure`). More
precise, but introduces complexity that brings little: detection on
`rules_empty` covers the 99th percentile.

### Decision: the `configure` body does not change contract

The `assets/workflows/configure.md` body currently mentions no
threshold on `context`. The rework of `is_config_thin` does not affect
it — the body describes what the skill does when it is invoked, not
when it is **recommended**.

The only body to update is `onboard.md`, which described how to
compute the hint (Read on the YAML). The formula becomes: "read the
YAML, check whether it carries at least one entry in `rules:` —
otherwise, it is thin".

### Decision: the unit tests become binary

The 3 `is_config_thin_*` tests are rewritten into 2:

- `is_config_thin_vrai_quand_rules_vides` — assertion: the function
  returns `true` for `true`.
- `is_config_thin_faux_quand_rules_presentes` — assertion: the
  function returns `false` for `false`.

Removal of the test on the 200-character threshold — it no longer
corresponds to anything.

## Risks / Trade-offs

- **"Permanent" hint for a user who refuses `configure`** — someone
  who does not want the skill will see the hint on every `codev init`
  / `codev status`. → **Mitigation**: two ways to make it disappear —
  either run `configure` once (rules written), or add by hand in the
  YAML a `rules: {}` entry of the "I know what I'm doing" kind. The
  second option is not documented but works mechanically.
- **Divergence between the old documented threshold and the new
  behavior** — people who have read `docs/codev.md` expect a threshold
  of 200. → **Mitigation**: the documentation is to be updated at the
  same time.

## Migration Plan

None. The fix only changes the observable behavior of the hint (it
appears more often). No file format changes.
