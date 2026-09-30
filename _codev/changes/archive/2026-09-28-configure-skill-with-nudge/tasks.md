# Tasks

## 1. `is_config_thin` function

- [x] 1.1 Create `codev-core::config::is_config_thin(context: Option<&str>, rules_empty: bool) -> bool`: returns true if `context` is absent or is < 200 characters AND `rules_empty` is true. Pure function, in `crates/codev-core/src/config/mod.rs`. Unit test: three cases — thin context + empty rules → true, long context → false, non-empty rules → false.

## 2. Skill body — `configure`

- [x] 2.1 Create `assets/workflows/configure.md` — the skill body. Expected sections:
      - Intro: "Enrich `_codev/config.yaml` by analyzing the project, without ever touching workflows/MCPs/schema."
      - **Input** — none; refuses if `_codev/config.yaml` is absent (points to `codev init`).
      - **Steps**:
        1. Read the existing `_codev/config.yaml`.
        2. Explore while respecting the **budget**: README (full), CONTRIBUTING (if present), `docs/**/*.md` (3-5 files), 8 source files max prioritized by git recency, `ls _codev/` + `ls <src-root>/` — one level only.
        3. Draft a patch: new `context:` (2-5 lines) + per-artifact `rules:` (1-2 each).
        4. **Show the diff** line by line, without writing.
        5. Ask "Apply? [yes/no]" — write only if yes.
      - **Guardrails**: preserves `schema`, `workflows`, `mcp`, `inherits`, comments; touches only `context` and `rules`; refuses if the config is absent.

## 3. Catalog — add `configure`

- [x] 3.1 Add the `configure` entry to `CATALOG` in `crates/codev-agents/src/workflows.rs`. Short description, `allowed_tools = "Bash(codev:*), Read, Write, Edit, Glob, Grep"` (no general `Bash`), `body = include_str!("../../../assets/workflows/configure.md")`.
- [x] 3.2 `DEFAULT_WORKFLOWS` goes from 7 to 8 by adding `"configure"` at the end of the list. Also remove the old comment that listed the 7 workflows.
- [x] 3.3 Rewrite the `sans_demande_installe_le_catalogue_par_defaut` test with the 8 workflows. Check that the "opt-out restriction" test stays green.
- [x] 3.4 Add an invariant test checking that `configure` has `Write` and `Edit` but **not** general `Bash` — to lock the rule "only `apply` has general Bash".

## 4. Update the `onboard` body

- [x] 4.1 Modify `assets/workflows/onboard.md` — add a new branch in the decision table, placed **before** the "no active change" branch: if `is_config_thin` is true (the body must explain how to compute it via `Read` on the YAML), recommend `/codev-configure` first with the sentence "Claude will enrich the config from the project", then `/codev-propose <idea>` second.
- [x] 4.2 Update the `onboard_est_dans_le_catalogue_et_a_les_bons_outils` test in `codev-agents::workflows` if the body content is tested (to cover the new `configure` mention).

## 5. Hint in `codev init`

- [x] 5.1 In `crates/codev-cli/src/render.rs`, `setup` output (render of `SetupOutcome`): after the call that writes "Restart Claude Code…", read the project's `_codev/config.yaml`, compute `is_config_thin` via the `context` and `rules` fields of `ProjectConfig`. If thin, replace the final line with the call to action "→ Recommended next step: /codev-configure". Otherwise keep the current line. *(Field `config_thin` added to `SetupOutcome`, computed in `install_skills` on the resolved config.)*
- [x] 5.2 Test the branch via a unit test in `codev-cli`: inject a `SetupOutcome` + a thin config → output contains `/codev-configure`. A non-thin config → output does not contain it.
- [x] 5.3 Confirm that the JSON output (`--json`) does not change — no field added to the contract. *(The `config_thin` field is on the `SetupOutcome` side, not propagated to `SetupV1` in `main.rs::setup_v1`.)*

## 6. Hint in `codev status`

- [x] 6.1 In `crates/codev-cli/src/render.rs`, "no active change" output: add a conditional third line. If `is_config_thin` on the resolved config → "hint: config barely filled in — /codev-configure can enrich it.". Otherwise nothing. *(Implemented in `main.rs` after the status `fail()`, in human output only.)*
- [x] 6.2 Check via a test that the `status` JSON does not change. *(The hint is emitted via `eprintln!` only in the `!json` branch of the `Command::Status` dispatch; the `StatusV1` contract and `status_shape()` stay identical.)*

## 7. Documentation

- [x] 7.1 `docs/codev.md` §4 (The Claude Code skills) — add a row in the table for `codev-configure` (role: "Enrich _codev/config.yaml by analyzing the project", `allowed-tools`: `Bash(codev:*), Read, Write, Edit, Glob, Grep`, default: ✓).
- [x] 7.2 `docs/codev.md` §2 (Installation, subsection "The `codev init` experience") — add a sentence at the end describing the hint: "If the generated config is thin, the output explicitly recommends `/codev-configure` as the next step — the user can follow or ignore it."
- [x] 7.3 `CHANGELOG.md` — new `[Unreleased]` entry at the top, `### Added` section: "`/codev-configure` skill that enriches `_codev/config.yaml` by analyzing the project (README, CONTRIBUTING, docs, code sample), with a diff + mandatory confirmation before writing. Automatic hint in `codev init` and `codev status` when the config is thin. The default catalog grows to 8 workflows."

## 8. Final checks

- [x] 8.1 `cargo test --workspace` green.
- [x] 8.2 `cargo clippy --workspace --all-targets` with no warnings.
- [x] 8.3 `codev validate --strict` green.
- [x] 8.4 Manual test: in a scratchpad, `cargo init --lib` then `codev init --yes` — the human output must contain `/codev-configure` (thin config by construction). *(Verified — the call to action "→ Recommended next step: /codev-configure" is displayed.)*
- [x] 8.5 Manual test: `codev status` on the same empty project must display the line "hint: config barely filled in — /codev-configure can enrich it." *(Verified — the line is displayed on stderr, JSON output unchanged.)*

## 9. Delivery

- [ ] 9.1 Bump `Cargo.toml` to `0.3.1` (patch — skill added, default behavior unchanged on a project whose config is non-thin). Tag `v0.3.1`, `git push origin main --tags`.
- [ ] 9.2 Date the `[0.3.1] — <date>` entry in `CHANGELOG.md`.
