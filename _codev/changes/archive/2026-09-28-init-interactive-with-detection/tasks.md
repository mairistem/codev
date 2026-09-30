# Tasks

## 1. `dialoguer` dependency

- [x] 1.1 Add `dialoguer = { version = "0.11", default-features = false, features = ["editor", "fuzzy-select"] }` to the `workspace.dependencies` of `Cargo.toml` (root). Verified by `cargo check --workspace` green.
- [x] 1.2 Declare the dependency in `crates/codev-cli/Cargo.toml` via `dialoguer.workspace = true`. Verified by `cargo tree -p codev-cli | grep dialoguer` non-empty.

## 2. `codev-core::detect` module

- [x] 2.1 Create `crates/codev-core/src/detect/mod.rs` — exposes the `Detected` struct with the fields `stack: Option<Stack>`, `project_name: Option<String>`, `license: Option<String>`, `has_ci: bool`, `is_git_repo: bool`, `mcps: Vec<DetectedMcp>`. Add `pub mod detect;` in `lib.rs`. Verified by `cargo build -p codev-core`.
- [x] 2.2 `detect/stack.rs` — functions `from_cargo_toml(&[u8]) -> Option<Stack>`, `from_package_json`, `from_pyproject_toml`, `from_go_mod`, `from_pom_xml`. Each function returns `Some(Stack { language, edition_or_version, workspace_crate_count, dependencies_summary })`. Golden unit tests per fixture (at least one workspace `Cargo.toml` + one React `package.json`).
- [x] 2.3 `detect/license.rs` — regex on the common names (MIT / Apache-2.0 / BSD-3-Clause / GPL-3.0 / MPL-2.0). Test: each known fixture is recognized; arbitrary text returns `None`.
- [x] 2.4 `detect/mcp.rs` — `parse_mcp_config(&[u8]) -> Vec<DetectedMcp>` that reads `mcpServers` from a JSON, keeps name + command + url + args. `matches_jira(&DetectedMcp) -> bool` with regex `(?i)jira|atlassian`. `tool_id(name, tool_suffix) -> String` that applies the name normalization → `mcp__<name_normalized>__<tool_suffix>`. Tests: three JSON fixtures (Atlassian Rovo, classic Atlassian, a non-Jira one), plus a string of spaces + dots for the normalization.
- [x] 2.5 Public function `detect::run(fs: &dyn FileSystem, root: &Path, home: &Path) -> Detected` that orchestrates the probes: tries Cargo then package.json then…, opens `LICENSE`, tests `.github/workflows/`, opens `.git/`, reads the 4 MCP files and merges them (the first one wins). Integration test with a pre-filled `MemoryFileSystem` reproducing the codev workspace — the output contains a Rust workspace stack + Atlassian MCP. *(Orchestrator placed in `codev-engine::detect` — the `FileSystem` port lives in engine, the "no I/O in core" invariant is preserved. Pure parsers in `codev-core::detect`.)*

## 3. `codev-core::config::generate` module

- [x] 3.1 Create `crates/codev-core/src/config/mod.rs` (or `config.rs` if the module does not exist yet) exposing the `GeneratedConfig` struct with typed fields carrying value + optional provenance comment. Verified: compiles.
- [x] 3.2 `render(&GeneratedConfig) -> String` — assembles a manual YAML, stable key order, provenance comments printed on the line above. Golden test: a fixed `GeneratedConfig` produces exactly the expected YAML (byte for byte).
- [x] 3.3 "render → read back" test — the produced YAML MUST parse without error via `serde_norway::from_str::<ProjectConfig>()` from the `codev-engine` module. Cross-crate integration test. *(Placed in `crates/codev-engine/tests/config_generate_roundtrip.rs` — dependency direction respected.)*

## 4. Bridge Detected → GeneratedConfig

- [x] 4.1 `codev-core::config::from_detected(detected: &Detected, choices: &UserChoices) -> GeneratedConfig` — assembles the final config from the detection + the user choices (workflows, free-form context). Pure function. Unit test: three cases — full detection + full choice, empty detection + minimal choice, MCP-only detection + full choice.

## 5. CLI prompts

- [x] 5.1 Create `crates/codev-cli/src/init_prompts.rs` — exposes `pub fn run(detected: &Detected, opts: &InitOptions) -> Result<UserChoices>`. `InitOptions` carries `--yes`, `--preset`, `--no-detect`. Short-circuits the prompts in non-interactive mode or without a TTY.
- [x] 5.2 TTY detection: use `std::io::IsTerminal` (stable since 1.70). Unit test for the decision function `should_prompt(opts, is_tty) -> bool`.
- [x] 5.3 Workflows prompt: `dialoguer::Select` with the three presets, default "Full (7)". The `Custom` preset opens a `MultiSelect` over the 7 workflows.
- [x] 5.4 Context prompt: `dialoguer::Input` for a short line, or Enter on an empty line switches to `dialoguer::Editor` with a skeleton pre-filled from the detected stack. Manual test documented in `tasks.md` — `dialoguer` functions cannot be unit-tested.
- [x] 5.5 MCP confirmation: if `detected.mcps` has an unambiguous candidate, displays `✓ Jira MCP detected: <tool_id>` without a prompt in `--yes`, with `Confirm` in interactive mode. If several candidates: a short `Select`.

## 6. Rework of `codev-cli::commands::init`

- [x] 6.1 Add the CLI flags in `crates/codev-cli/src/main.rs`: `--yes` (`-y`), `--no-detect`, `--preset <complet|minimal|personnalise>`. `--force` unchanged. Verified by `codev init --help` listing them.
- [x] 6.2 Rework `commands::init` to orchestrate: `detect::run` (except with `--no-detect`) → `init_prompts::run` → `config::from_detected` → `render` → write `_codev/config.yaml` **before** the existing scaffolding, then call `install_skills`. The current logic relying on the commented template disappears.
- [x] 6.3 "Project already initialized" case: if `_codev/config.yaml` exists, `codev init` **does not regenerate** the file; it continues with skill installation only (current idempotent behavior). Unit test *(covered by `relancer_init_ne_change_rien` and `init_respecte_les_workflows_deja_configures`)*.
- [x] 6.4 End-to-end integration test via `MemoryFileSystem`: `codev init --yes` on a folder with a workspace `Cargo.toml` + an Atlassian `.mcp.json` produces a `_codev/config.yaml` containing the 7 workflows, the pre-filled `mcp.jira_tool` key, the `context:` with the detected stack, and the 7 skills installed.

## 7. Reversing `DEFAULT_WORKFLOWS`

- [x] 7.1 Modify `crates/codev-agents/src/workflows.rs` — `DEFAULT_WORKFLOWS` goes from `&["propose", "explore", "onboard"]` to `&["propose", "explore", "onboard", "apply", "sync", "archive", "update"]`.
- [x] 7.2 Rewrite the `sans_demande_installe_le_catalogue_par_defaut` test: the assertion on the 3 workflows becomes an assertion on the 7; the "other opt-in ones stay opt-in" loop disappears. Add a complementary "opt-out restriction via explicit workflows" test that calls `select(Some(&["propose"]))` and checks that a single skill is kept.
- [x] 7.3 Check that the `init_respecte_les_workflows_deja_configures` test of `codev-cli/src/commands.rs` stays green (it sets an explicit `workflows: - explore` config, which must keep producing a single skill).

## 8. Config template file

- [x] 8.1 `crates/codev-engine/src/scaffold.rs` — `DEFAULT_CONFIG` (the YAML template with commented workflows) is no longer used at `init` time (generation replaces it) but **remains used** for `codev update` on a project where the file was deleted by hand. Check that no test depends on it beyond that, and document the residual role in a comment above the constant.

## 9. Main spec — merging the deltas

- [ ] 9.1 Check that `codev validate init-interactive-with-detection` is green before archiving (the two deltas — `init` ADDED and `skills` MODIFIED — are well-formed).
- [ ] 9.2 No action here: the merge into `_codev/specs/init/spec.md` (new) and the update of `_codev/specs/skills/spec.md` are done by `codev archive`, not in `apply`.

## 10. Documentation

- [x] 10.1 `docs/codev.md` §2 (Installation) — add a "The `codev init` experience" subsection that describes the new flow in 15-20 lines: probe, two questions, provenance in the YAML, `--yes` and `--preset` flags.
- [x] 10.2 `docs/codev.md` §7 (Configuration) — update the mention of `workflows:` to reflect the new default (7 instead of 3), presenting it as the **opt-out** path.
- [x] 10.3 `CHANGELOG.md` — new `[Unreleased]` entry at the top with a `### Changed` section: "Default of `codev init` and `codev update`: the 7 workflows are installed by default. A project that wants fewer declares an explicit `workflows:` (opt-out path)." and an `### Added` section: "Interactive `codev init` with automatic detection of the stack and of configured MCPs; `--yes`, `--preset`, `--no-detect` flags."

## 11. Final checks

- [x] 11.1 `cargo test --workspace` green (all the new tests + the adapted existing tests).
- [x] 11.2 `cargo clippy --workspace --all-targets` with no warning.
- [x] 11.3 `codev validate --strict` green.
- [x] 11.4 End-to-end manual test: in a blank scratchpad folder, `mkdir /tmp/codev-manual-test && cd /tmp/codev-manual-test && cargo init --lib` (creates a Cargo.toml), then `codev init --yes`, check that the produced `_codev/config.yaml` contains the 7 workflows and the `context:` with "Rust". *(Done — 7 skills, `context: | Rust project, 2024.`, `codev status` OK.)*
- [ ] 11.5 Interactive manual test: in a TTY, `codev init` without a flag, answer the two prompts, check that the final YAML reflects the choices.

## 12. Delivery

- [ ] 12.1 Bump `Cargo.toml` to `0.3.0` — **minor** because the default behavior changes (the 7 workflows instead of 3 is a notable observable change). SemVer 0.x allows this kind of switch on a minor bump. Tag `v0.3.0`, `git push origin main --tags`.
- [ ] 12.2 Add the `[0.3.0] — <date>` entry in `CHANGELOG.md` with the summarized content of 10.3.
