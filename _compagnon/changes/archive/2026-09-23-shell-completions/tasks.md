# Tasks

## 1. Dependency

- [x] 1.1 Add `clap_complete = "4"` to the dependencies of
      `crates/codev-cli/Cargo.toml`. Check that the major version
      matches the one of `clap` already in use (currently `4`).

## 2. CLI subcommand

- [x] 2.1 Add the variant `Command::Completions { shell:
      clap_complete::Shell }` to the `Command` enum in
      `crates/codev-cli/src/main.rs`. Clap documentation: "Generate
      a shell completion script for local installation."
- [x] 2.2 Document, at the top of the variant, the typical
      installation commands per shell (bash, zsh, fish, powershell,
      elvish) — the text appears in `codev completions --help`.
- [x] 2.3 Add the `match` arm in `run(cli)`:
      ```rust
      Command::Completions { shell } => {
          use clap::CommandFactory;
          let mut cmd = Cli::command();
          clap_complete::generate(shell, &mut cmd, "codev", &mut std::io::stdout());
          0
      }
      ```

## 3. Tests

- [x] 3.1 Test in `main.rs` (or a dedicated module) that, for each of
      the five shells, calls generation via `clap_complete::generate`
      into a buffer and checks:
      - the output is non-empty (> 200 bytes, the typical
        lower bound);
      - the output contains `codev` at least once.
- [x] 3.2 Refusal test for an unknown shell — verified
      structurally: `clap` refuses at argument parsing (via
      `ValueEnum` on `clap_complete::Shell`), there is nothing to write
      on our side. A test "`codev completions nushell` surfaces a
      clap error" is optional.

## 4. Documentation

- [x] 4.1 The `codev completions --help` message lists the
      installation procedures per shell (bash, zsh, fish,
      powershell). Placed in the variant's doc-comment.
- [x] 4.2 The repository `README.md` (or a dedicated section) briefly
      mentions `codev completions` and points to `--help` for the
      details.

## 5. Dogfooding and workspace integration

- [x] 5.1 After `cargo install --path crates/codev-cli`, run
      `codev completions zsh | head -20` — visually check that the
      output is a coherent zsh script.
- [x] 5.2 Actually install into the shell (`codev completions zsh
      > "${fpath[1]}/_codev"` then `compinit`), type `codev de<TAB>`
      in a new shell — completion must list `decision`,
      `deviate`, etc.
- [x] 5.3 `cargo test --workspace` stays green, gains at least 1 test
      (generation for the five shells).
- [x] 5.4 `cargo clippy --workspace --all-targets` stays free of
      warnings.
- [x] 5.5 `codev validate --strict` on this repository stays green.
