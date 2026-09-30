# Design: interactive codev init with detection

## Context

See `proposal.md`. New `init` capability, modification of `skills` to
reverse `DEFAULT_WORKFLOWS`, rework of `codev-cli::commands::init`,
new module `codev-core::detect`, new module
`codev-core::config::generate`. New dependency `dialoguer`.

## Decisions

### Decision: `dialoguer` rather than `inquire`

Both Rust prompt libraries are mature. `dialoguer` is smaller (about
ten dependencies, no tokio), stable since 2018, used by `cargo`,
`rustup`, `clap` and `k8s`. `inquire` is richer (inline validation,
autocomplete) but those features are of no use to us here — two
questions, no sophisticated validation.

Cost for codev: ~10 additional transitive dependencies. Cost of
`inquire`: ~20. For a dev-tool CLI, `dialoguer` wins.

**Rejected alternative A**: `inquire`. Rejected on the grounds of
weight disproportionate to the needs.

**Rejected alternative B**: roll our own prompt via `println!`
+ `io::stdin().read_line()`. Rejected — clean handling of the TTY, of
keyboard selection, of redraw on backspace, costs more than a
battle-tested dependency.

### Decision: detection in `codev-core`, prompts in `codev-cli`

Detection is **pure** — it transforms `&[u8]` (manifest contents) into
a `struct Detected`. It needs no I/O beyond reading the files, which
goes through the existing `FileSystem` port. It lives in
`codev-core::detect` (submodules `stack`, `mcp`, `license`, `ci`,
`git`).

Prompts are **impure** — they read stdin, write stdout, probe the TTY
state. They live in `codev-cli::init_prompts` (new module).

The orchestration (sniff → prompt → generate → scaffold → install)
lives in `codev-cli::commands::init`.

**Rejected alternative**: put detection in `codev-engine` because it
"does I/O". Rejected — reading a `Cargo.toml` through the
`FileSystem` port is pure reading as seen from the domain. Placed in
`codev-engine`, it would mix detection and effects, whereas detection
is precisely the kind of computation a pure core knows how to do.

### Decision: generating `_codev/config.yaml` is a **manual YAML rendering**, not `serde_norway::to_string`

The generated file carries **provenance comments** above certain keys
(`# detected from Cargo.toml`). No YAML serialization library
preserves comments — a non-issue for reading (they ignore them), but
an insurmountable problem for writing.

The `codev-core::config::generate` function MUST therefore render the
YAML **character by character**: it assembles a string from a typed
`GeneratedConfig` that carries, for each field, its value AND its
optional provenance comment.

The rendering is deterministic (stable field order) and testable by
golden: same inputs, same bytes out.

**Rejected alternative**: generate bare YAML with `serde_norway`, then
post-process the string to insert the comments via regex/split.
Rejected — too fragile and no shorter than manual rendering.

**Cross-check**: the generated YAML MUST go through
`serde_norway::from_str::<ProjectConfig>()` in a unit test, to
guarantee that it stays readable by the rest of codev. We generate by
hand, we read back with the library.

### Decision: `Detected` contains `Option`s — never invention

Each field of `Detected` is an `Option<T>`. A missing or unreadable
manifest produces `None`, never a default invented value. The
generation of `context:` only mentions the `Some` fields. A project
with no known manifest and no `.github/workflows/` produces a minimal
`context:` — this is not an error.

**Rejected alternative**: produce a default `context:` ("generic
project") when nothing is detected. Rejected — better to say nothing
than to assert something false.

### Decision: MCP resolution order — project wins over user

MCP sources scanned, in this order:

1. `<project>/.mcp.json`
2. `~/.claude.json`
3. `<project>/.claude/settings.json`
4. `<project>/.claude/settings.local.json`

The `mcpServers` entries are merged; if the same name appears in two
files, **the first one found wins** — the project config takes
precedence over the global user config. This choice follows the
Claude Code convention (project files have priority).

`~/.claude.json` is read **best-effort** — if missing, silence; if
malformed, silent warning, we continue.

**Rejected alternative**: reverse merge (global wins). Rejected —
counter-intuitive for a dev who has set up an MCP specifically for
this project.

### Decision: Stack detection — first manifest wins, no vote

Search order: `Cargo.toml` → `package.json` → `pyproject.toml`
→ `go.mod` → `pom.xml`. The **first one found** sets the primary
stack; the others are ignored. A polyglot repository shows the stack
of the root manifest.

This convention is arbitrary but stable. The user can always amend
the generated `context:` by hand.

**Rejected alternative**: mention **all** the manifests found in the
`context:`. Rejected — typical noise from a `node_modules/` or a
submodule that would pollute detection.

### Decision: The "Context" prompt offers to open `$EDITOR`

A useful context is often more than one sentence. Forcing the user to
type the line at the prompt pushes them to minimize. The Git
convention is to open `$EDITOR` on a pre-filled temporary file, which
becomes the input once closed.

`dialoguer` supports this via `Editor::new().edit()`. We use it only
at that prompt (the workflows stay as a `Select`).

**Rejected alternative**: accept only the short line at the prompt.
Rejected — too much friction for a field that benefits from being
detailed.

### Decision: Reversing `DEFAULT_WORKFLOWS` = 7 workflows — non-gradual switch

The new default breaks the strictly chronological reading of the
`skills` spec: an existing test ("Default catalog includes onboard"
with exactly 3 workflows) becomes false. The spec is therefore
**MODIFIED**, not supplemented by an ADDED — the contract changes, it
is not an extension. The corresponding test is rewritten.

Side effect for existing projects: `codev update` on a project that
has no explicit `workflows:` key installs the 4 missing skills all at
once. This is the intended behavior — an under-configured project
catches up on what it should have had. A project that insists on
staying on the 3 initial skills can add the explicit `workflows:` key
(opt-out path).

Documented in `CHANGELOG.md` as a **notable behavior change** (but
non-breaking: the old behavior remains accessible through explicit
declaration).

## Risks / Trade-offs

- **Prompt hangs on a false-positive TTY** — an exotic emulator that
  claims to be a TTY but does not accept input makes the command
  hang. → **Mitigation**: `--yes` remains the kill switch. The
  "non-TTY stdin implies --yes" rule catches the common case (CI,
  pipe).
- **MCP detection false positive** — a server named `"my-jira-mock"`
  is wrongly matched as a real Jira MCP. → **Mitigation**: detection
  only proposes; the user confirms in interactive mode. In `--yes`, a
  false positive produces a `mcp.jira_tool:` that points to a
  non-existent tool — the skills that call it will fail cleanly with a
  clear message (already covered by the `skills` propose spec).
- **Changing the default breaks an existing test** — this is the
  expected price, not a risk. The "other opt-in ones stay opt-in" test
  is removed in the MODIFIED.
- **Manual YAML generation is tedious to maintain** — each new field
  has to be added to the renderer. → **Mitigation**: the renderer is
  short (~80 lines), typed, tested by golden. A forgotten key is
  immediately visible.

## Migration Plan

No migration tool needed. An existing project that has no explicit
`workflows:` key will see its skills completed on the next
`codev update`. A project that has one may want to add the missing
workflows (internal ADR — no constraint).

`CHANGELOG.md` documents the change with a short sentence:
"The default of `codev init`/`update` goes from 3 to 7 skills —
projects that want fewer declare an explicit `workflows:`."
