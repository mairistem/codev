# Tasks

## 1. Simplification of `is_config_thin`

- [x] 1.1 In `crates/codev-core/src/config/mod.rs`, change the signature of `is_config_thin`: go from `is_config_thin(context: Option<&str>, rules_empty: bool) -> bool` to `is_config_thin(rules_empty: bool) -> bool` — return `rules_empty` directly. Remove the doc of the 200 threshold.
- [x] 1.2 Rewrite the unit tests: delete the 3 threshold-based tests, replace them with two binary tests (`is_config_thin_vrai_quand_rules_vides`, `is_config_thin_faux_quand_rules_presentes`). Verified by `cargo test -p codev-core is_config_thin` green.

## 2. Adapting the call sites

- [x] 2.1 In `crates/codev-cli/src/commands.rs`, `install_skills`: replace the computation using `context_total` with a simple `let config_thin = codev_core::config::is_config_thin(config.rules.is_empty());`. Remove the lines that build `context_total`.
- [x] 2.2 In `crates/codev-cli/src/main.rs`, `config_is_thin`: simplify likewise — no need to rebuild `context_total` anymore, a single call `is_config_thin(cfg.rules.is_empty())`. Remove the intermediate lines.
- [x] 2.3 Check that the `render::setup` tests (unit test on `config_thin: true/false`) stay green without modification, since the `config_thin: bool` field on `SetupOutcome` has not changed type.

## 3. Updating the `onboard` body

- [x] 3.1 In `assets/workflows/onboard.md`, section "Recommend the next action": rewrite the table row and the explanatory subsection to talk only about "empty or missing `rules:`". Remove any mention of "200 characters" or of the context length.
- [x] 3.2 Adjust the `onboard_est_dans_le_catalogue_et_a_les_bons_outils` test of `codev-agents::workflows`: the assertion `body.contains("thin") || body.contains("200")` must remain true (the body still mentions "thin"). Add a complementary assertion that the body no longer mentions "200 characters".

## 4. Documentation

- [x] 4.1 `docs/codev.md` §2 (subsection "The `codev init` experience"): the sentence "If the generated config is thin (short context, no `rules:`)" becomes "If the generated config does not have any `rules:` yet". Adjust.
- [x] 4.2 `CHANGELOG.md`: new `[Unreleased]` entry at the top, `### Fixed` section: "`is_config_thin` detection: no longer looks at the length of `context:`, bases the decision solely on `rules.is_empty()`. A project with a rich stack (TypeScript, Java Maven, etc.) received a long auto-detected context that suppressed the hint, even though no rule had been written."

## 5. Final checks

- [x] 5.1 `cargo test --workspace` green (the adapted tests + the existing tests).
- [x] 5.2 `cargo clippy --workspace --all-targets` with no warning.
- [x] 5.3 `codev validate --strict` green.
- [x] 5.4 Manual test: rebuild codev, rerun on a TypeScript project or one with Cargo and multiple dependencies. The `/codev-configure` hint MUST be displayed in the output of `codev init` and of `codev status`, whereas it was previously absent. *(Verified on `~/Sources/mira` — both hints are displayed correctly, whereas they were absent in v0.3.1.)*

## 6. Delivery

- [ ] 6.1 Bump `Cargo.toml` to `0.3.2` (patch — pure fix). Tag `v0.3.2`, `git push origin main --tags`.
- [ ] 6.2 Date the `[0.3.2] — <date>` entry in `CHANGELOG.md`.
