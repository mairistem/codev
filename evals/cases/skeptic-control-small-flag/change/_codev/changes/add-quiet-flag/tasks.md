# Tasks

## 1. Flag

- [ ] 1.1 `--quiet` read by `src/main.rs`, added to `FLAGS` in `src/cli/import.rs` and passed to `Progress::new(quiet)`, verified by tests `a_quiet_import_prints_nothing` ("A quiet import") and `a_quiet_import_still_prints_errors` ("A quiet import that fails")
- [ ] 1.2 Document `--quiet` in `docs/cli.md`, verified by `cargo test docs_mention_every_flag`
