# Tasks

## 1. Dependencies

- [x] 1.1 Add `pulldown-cmark = "0.10"` to the dependencies of
      `crates/codev-cli/Cargo.toml`, with the default feature being
      sufficient (tables, footnotes built in).
- [x] 1.2 Add `open = "5"` to the dependencies of
      `crates/codev-cli/Cargo.toml`.

## 2. Documentation source

- [x] 2.1 Create `docs/codev.md` — the documentation's markdown
      source, versioned.
- [x] 2.2 Write the 10 sections defined in the design:
      Introduction, Installation, The cycle, The 7 skills, The
      concepts, The CLI, Configuration, Architecture decisions,
      MCP extensions, FAQ. Target length: 2000-3000 words.
- [x] 2.3 Tone and style: aligned with the proposals/designs we
      have been writing for 16 changes. French, clear sentences, no
      gratuitous anglicisms.

## 3. `codev-cli::docs` module

- [x] 3.1 Create `crates/codev-cli/src/docs.rs` with:
      - `const MARKDOWN_SOURCE: &str = include_str!("../../../docs/codev.md");`
      - `const CSS: &str = include_str!("../assets/docs.css");`
      - `pub fn render_html(md: &str, version: &str) -> String`:
        calls `pulldown_cmark::Parser::new_ext(md, Options::ENABLE_TABLES | Options::ENABLE_FOOTNOTES)`,
        pushes into `html::push_html`, wraps in the HTML5 template.
      - `pub fn default_output_path(version: &str) -> PathBuf`:
        `std::env::temp_dir().join(format!("codev-docs-{version}.html"))`.
      - `pub fn write_to(path: &Path, version: &str) -> io::Result<()>`:
        renders + `std::fs::write` (creates the parent if its parent
        exists, refuses otherwise).
      - `pub fn open_default(version: &str) -> io::Result<PathBuf>`:
        `write_to(default_output_path, version)` + `open::that(path)`.
- [x] 3.2 Create `crates/codev-cli/assets/docs.css` — ~120 lines of
      standalone techdoc CSS (typography, monospace code, readable
      tables, monochrome palette).
- [x] 3.3 Add `pub mod docs;` to `crates/codev-cli/src/main.rs`.

## 4. CLI subcommand

- [x] 4.1 Add the variant `Command::Docs { print: bool, write:
      Option<PathBuf> }` to the `Command` enum in `main.rs`. The
      two flags are mutually exclusive (`conflicts_with`).
- [x] 4.2 Doc comment of the variant: "Open the codev documentation
      in the browser, or write it to a given path, or print the
      markdown source to stdout."
- [x] 4.3 Add the match arm in `run(cli)`:
      ```rust
      Command::Docs { print, write } => {
          if print {
              use std::io::Write;
              let _ = std::io::stdout().write_all(docs::MARKDOWN_SOURCE.as_bytes());
              return 0;
          }
          if let Some(path) = write {
              match docs::write_to(&path, VERSION) {
                  Ok(()) => 0,
                  Err(e) => { eprintln!("error: cannot write: {e}"); 1 }
              }
          } else {
              match docs::open_default(VERSION) {
                  Ok(p) => { eprintln!("Opened {}", p.display()); 0 }
                  Err(e) => {
                      eprintln!(
                          "error: cannot open the browser: {e}\n\
                           help: try `codev docs --print` or `codev docs --write <PATH>`"
                      );
                      1
                  }
              }
          }
      }
      ```

## 5. Tests

- [x] 5.1 Unit test `docs::render_html_produit_un_html5_complet`:
      calls `render_html("# Title\n\nParagraph.", "0.1.0")`,
      checks that the output starts with `<!doctype html>`,
      contains the `codev` title, the version, the paragraph, and
      **no** `http` URL (self-containment).
- [x] 5.2 Test `docs::render_html_gere_tables_et_code`: markdown
      input with a table and a code block, checks that the output
      contains `<table>` and `<code>`.
- [x] 5.3 Test `docs::write_to_ecrit_le_fichier`: in a `tempdir`,
      calls `write_to`, checks that the file exists and does
      contain HTML.
- [x] 5.4 Test `docs::markdown_source_est_non_vide`: checks that
      `MARKDOWN_SOURCE.len() > 1000` (tracer for a broken embed).

## 6. CLI docs

- [x] 6.1 The `--help` of `codev docs` mentions the three forms,
      with one example per form.
- [x] 6.2 The repository README mentions `codev docs` with one
      sentence and a link to `docs/codev.md`.

## 7. Dogfooding and workspace integration

- [x] 7.1 After `cargo install --path crates/codev-cli`, run
      `codev docs --print | head -30` — check visually that the
      documentation is coherent.
- [x] 7.2 Run `codev docs --write /tmp/manuel.html`, open the
      file in a browser (macOS `open
      /tmp/manuel.html`), check the rendering (styles, table of
      contents if JS, content).
- [x] 7.3 Run `codev docs` (without arguments) and check that the
      browser actually opens on the temporary file.
- [x] 7.4 `cargo test --workspace` stays green, gains at least 4
      dedicated tests.
- [x] 7.5 `cargo clippy --workspace --all-targets` stays free of
      warnings.
- [x] 7.6 `codev validate --strict` on this repository stays green.
