# Design: `codev docs` and its standalone HTML bundle

## Context

See `proposal.md`. A documentation bundle that answers the three
questions settled by default:

- **Generation**: runtime, not build-time (the build stays simple).
- **Visual design**: minimal and home-made, no CDN.
- **Content**: manual, written in the same tone as the codev
  artifacts.

## Goals / Non-Goals

This design covers the markdown → HTML conversion, system opening,
the three forms of the subcommand, the embedding of the markdown,
and the CSS skeleton. It does not cover PDF generation, JS full-text
search, or generating the CLI section from clap.

## Decisions

### Decision: **runtime** conversion via `pulldown-cmark`

Two options:

| Option | Pro | Con |
|---|---|---|
| **A. Build-time via `build.rs`** — the HTML is embedded ready to use | Zero cost at runtime, precomputed HTML | Build cache invalidated on every documentation edit, an extra `build.rs` to maintain |
| **B. Runtime — the markdown is embedded, converted at call time** | Build stays simple, markdown source caching identical to the other `include_str!` | ~5-10 ms of conversion on every `codev docs` |

**Chosen: B.** The 5-10 ms are painless compared to opening the
browser (~500 ms). The build stays homogeneous — no `build.rs` that
becomes a special place for a single thing.

### Decision: `pulldown-cmark` version 0.10, no custom renderer

`pulldown-cmark` is the mainstream markdown → HTML converter in
Rust: used by mdBook, docs.rs, GitBook alternatives. Its
`html::push_html` handles the six elements we need (headings,
paragraphs, lists, tables, code, blockquotes) with the **tables**
and **footnotes** extensions enabled.

**Rejected alternative**: `comrak` (strict CommonMark compliant).
Heavier, less necessary.

### Decision: minimal home-made HTML template, no framework

A template of about 150 lines, with:

- HTML5 doctype.
- `<meta charset="utf-8">` and mobile viewport.
- Inline `<style>` with ~120 lines of CSS.
- `<header>`: name + version.
- `<nav>`: table of contents auto-generated from the `h2`/`h3` (via
  30 lines of JS; without JS, the table of contents does not appear
  but the documentation remains navigable via Ctrl+F).
- `<main>`: the content.
- **No** external `<link>`.

**CSS style**: techdoc typography, `system-ui` for headings,
`Georgia`/`serif` for the body (comfortable on-screen reading),
`ui-monospace` for code. Monochrome palette (sufficient contrast,
printable).

**Rejected alternative**: Tailwind Play CDN. Breaks
self-containment (first opening requires internet).

### Decision: three exclusive forms — default, `--print`, `--write`

Resolution ordered by precedence:

1. `--print` alone → markdown on stdout, nothing else.
2. `--write <PATH>` → HTML at the given path, no opening.
3. Default (no flag) → HTML in `temp_dir()` + opening.

The `--print --write` combinations are refused by clap
(`conflicts_with`).

### Decision: default path = `temp_dir()/codev-docs-<version>.html`

Calling `codev docs` again overwrites the file from the previous
session. Idempotent. A file name that carries the version
distinguishes versions installed side by side (rare but possible).

**Rejected alternative**: `~/.local/share/codev/docs.html`.
Persistent but pollutes `$HOME`, which is not the role of a docs
command.

### Decision: system opening via the `open` crate

`open = "5"` does exactly one thing: `open::that(path)` → delegates
to `open` / `xdg-open` / `start`. 200 lines of source code, zero
runtime dependencies.

**Rejected alternative**: a home-made
`std::process::Command::new("open").arg(…)`. Works on macOS but
breaks on Linux/Windows. The crate absorbs these differences for
next to nothing.

### Decision: manual content, ten sections

Outline of the documentation, to be written in `docs/codev.md`:

1. **Introduction** — what codev is, why this cycle.
2. **Installation** — cargo install, PATH, `~/.cargo/bin`,
   completions.
3. **The cycle** — propose → apply → sync → archive, ASCII diagram.
4. **The 7 Claude Code skills** — propose, explore, apply, sync,
   archive, update, onboard. Each one: role, `allowed-tools`, when
   to use it.
5. **The concepts** — capability, decision, change, delta operation
   (`ADDED`, `MODIFIED`, `REMOVED`, `RENAMED`).
6. **The CLI** — every command with an example, grouped by family
   (project, changes, decisions, sources, validation, docs).
7. **Configuration** — `_codev/config.yaml`: schema, workflows,
   context, rules, inherits, mcp.
8. **Architecture decisions** — the 6 root ADRs, K3/K6/K7.
9. **MCP extensions** — Jira/Atlassian today, how to add other
   MCPs.
10. **FAQ / troubleshooting** — common memory lapses, failure
    cases.

A single tone aligned with the artifacts we have been writing for 16
changes.

## Risks / Trade-offs

- **A user on a system where `open`/`xdg-open`/`start` does not
  exist** — an edge case (headless servers). The `open` crate
  surfaces the error; the command prints "cannot open the browser —
  use `--print` or `--write`."
- **The HTML weighs 100-300 KB once embedded with the CSS and the
  documentation** — measurable at the first build, acceptable
  compared to a binary that already weighs ~4 MB.
- **Future documentation changes will break any golden tests** —
  that is **the point**. A golden test would be replaced by
  structural tests: "the HTML starts with `<!doctype html>`", "it
  cites no external URL", "it contains the name `codev`".

## Migration Plan

None. An additive, opt-in feature.
