# Proposal: codev documentation as standalone HTML, openable via `codev docs`

## Why

The CLI help (`codev --help`, `codev <cmd> --help`) gives the
surface — every subcommand, every flag. It **does not give**:

- an overview of the **propose → apply → sync → archive** cycle;
- the concepts (capabilities, decisions, delta operations, K3 seal,
  K6 drift, K7 promotion);
- the installation procedures (cargo install, shell completions,
  MCP configuration);
- a document to share by email, Slack or Confluence to convince or
  onboard a colleague.

What is missing is **a single, shareable document** that codev can
open by itself. It is useful to the future JVS users who will come
along, and immediately useful to Ludovic for socializing the tool.

## What Changes

- **New `docs/codev.md` file** — the source of truth for the
  documentation, versioned in the repository. Structured in ~10
  sections, it covers everything from "what is codev" to
  "architecture decisions" and "MCP extensions". Written by hand, in
  a tone aligned with the proposals/designs we have been writing for
  15 changes.
- **New dependency `pulldown-cmark = "0.10"`** in
  `crates/codev-cli/Cargo.toml` — the de facto standard markdown →
  HTML converter in Rust, no JS runtime, no external assets.
- **New dependency `open = "5"`** — a mini-crate that delegates to
  `open` (macOS), `xdg-open` (Linux), `start` (Windows) to open a
  file in the default app.
- **Embedding of the markdown** in the binary via
  `include_str!("../../docs/codev.md")` — the documentation is
  **always in sync** with the binary's version, without depending on
  an external file.
- **New `codev docs` subcommand** with three forms:
  - `codev docs` (default) — writes a `codev-docs-<version>.html`
    file in the system temporary directory
    (`std::env::temp_dir()`) and opens it in the system browser.
    The HTML embeds its CSS and is standalone.
  - `codev docs --write <PATH>` — writes the HTML to the given path
    and opens nothing. Useful for targeted distribution
    (`--write ./docs.html` in a shared drive).
  - `codev docs --print` — prints the raw **markdown** to stdout
    (for piping into `less`, `bat`, or an LLM that prefers text).
- **A minimal home-made CSS** of ~120 lines — a readable techdoc
  style (Times/Georgia for the body, sans-serif for headings,
  monospace code, table of contents on the left on desktop). No
  framework, no CDN, strict self-containment.

## Capabilities

### New Capabilities

- `docs` — describes the contract of the `codev docs` subcommand:
  output formats, self-containment of the HTML, output paths, no
  dependency on an initialized repository.

### Modified Capabilities

None.

### Removed Capabilities

None.

## Impact

- **Code**:
  - New `docs/codev.md` file (versioned).
  - New `codev-cli::docs` module that exposes:
    - `MARKDOWN_SOURCE: &str` (the embed).
    - `fn render_html(md: &str, version: &str) -> String`: converts
      via `pulldown-cmark`, wraps in an HTML template with inline
      CSS.
    - `fn open_default(version: &str) -> io::Result<PathBuf>`:
      writes into `temp_dir()` + `open::that()`.
    - `fn write_to(path: &Path, version: &str) -> io::Result<()>`.
  - New variant `Command::Docs { print: bool, write:
    Option<PathBuf> }` in `main.rs`.
- **JSON contract**: nothing. The subcommand falls outside the
  global contract (like `completions`) — HTML is not structured
  JSON, and a `--json` would make no sense.
- **File written**: only the HTML at the chosen path
  (`temp_dir()` by default, or `--write <PATH>`). No write into
  `_codev/`, no system write. Idempotent: calling again overwrites
  the file.
- **Migration**: none. A purely additive feature.
- **Out of scope**:
  - **PDF** — two formats to maintain when one is enough for V1.
    Can be deferred if a real need appears (formal printing).
  - **JS full-text search** — the browser's Ctrl+F is already there.
    Can be deferred if the documentation becomes long.
  - **Translation / i18n** — French only for V1.
  - **Automatic extraction from clap** for the CLI section — the
    editorial difference is worth the manual maintenance. Can be
    deferred.
  - **Generation at build time via `build.rs`** — runtime conversion
    costs a few milliseconds, painless. The build stays simple.
