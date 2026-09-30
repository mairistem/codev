Enrich `_codev/config.yaml` by analyzing the project — propose a detailed
`context:` and per-artifact `rules:`, without ever touching the
workflows, the MCPs or the schema.

**Strict boundary.** This skill:

- Reads the project at surface level, within a **budget** defined below.
- Modifies **only** the `context` and `rules` fields of
  `_codev/config.yaml`.
- **Preserves** `schema`, `workflows`, `mcp`, `inherits` and every
  existing comment — the `codev init` probe filled those fields; they
  are not replayed.
- **Writes nothing without confirmation** — show the diff, ask, write.

It **refuses to act** if `_codev/config.yaml` is missing: in that case,
point the user to `codev init` and stop.

---

## Input

None. If the user asks a question, answer within the same boundary — no
writing until the final confirmation.

## Steps

### 1. Check that `_codev/config.yaml` exists

```bash
codev status --json
```

If the command fails with `no_codev_root`, say:

> This project has no `_codev/`. Run `codev init` first, then
> come back to `/codev-configure`.

Then stop.

Otherwise, read the file:

```
Read _codev/config.yaml
```

Note the current value of `context:` and `rules:` — it is the baseline
for comparison.

### 2. Explore the project — within the budget

**Mandatory reading** (if present):

- Root `README.md` — read in full.
- Root `CONTRIBUTING.md` — read in full.

**Targeted reading**:

- `docs/**/*.md` — at most **5** files, prioritized by increasing size
  (the shortest are often indexes and overviews).
- **At most 8 source files**, prioritized by recency
  (`git log --since='6 months ago' --pretty=format: --name-only | sort | uniq -c | sort -rn | head -20`
  then filter by the extension of the detected language).
- `ls _codev/` and `ls src/` (or equivalent for the stack) — **one
  level only**, just to grasp the structure.

**Do not**:

- Open the `target/`, `node_modules/`, `dist/`, `.git/` folders
  (except `.git/config` as a last resort, never necessary).
- Read more files than the budget — it makes the output drift.
- Run sweeping `grep`s.

### 3. Draft the proposal

Two blocks to produce, kept separate:

**`context:`** — 2 to 5 lines, factual, focused on "what an agent must
know before writing". It must cover:

- The stack beyond the language (frameworks, structuring libraries).
- The error-handling style (result-based, exceptions, panic-free,
  etc.).
- The API conventions if the project exposes any (REST, gRPC, GraphQL,
  CLI…).
- The tone of comments (language, format, what they explain).
- One structuring choice of the project — no more than one or two.

**Do not repeat** what the detected stack already contains. Complement it.

**`rules:`** — 1 to 2 rules per artifact, for `specs`, `design`,
`tasks`. Each rule MUST be:

- **Positive** — say what is wanted, not what is not.
- **Checkable on review** — no "clean", no "elegant".
- **Grounded** in what the project does, not generic.

### 4. Show the diff

Expected format:

```
Here is the proposed patch for _codev/config.yaml:

--- context: (current) ---
Rust project, 2024.

--- context: (proposed) ---
Rust workspace (4 crates), edition 2024. Typed errors with
thiserror in the libraries, anyhow only in the CLI. Comments
in English; they explain the why.

--- rules: (current) ---
(empty)

--- rules: (proposed) ---
specs:
  - Describe observable behavior, never an implementation.
design:
  - Cite a decision from _codev/decisions/ that constrains the choice.
tasks:
  - Each task states how to verify that it is done.
```

### 5. Ask for confirmation, then write

Exact question:

> Apply this patch to `_codev/config.yaml`? [yes/no]

If `yes`:

- **Edit** `_codev/config.yaml` — replace only the `context:` and
  `rules:` sections. **Preserve** `schema`, `workflows`, `mcp`,
  `inherits`, and **every comment**.
- Add a comment above `context:`:
  `# written by /codev-configure`. If there was already one
  (e.g. "detected from Cargo.toml"), replace it with the new one.
- Confirm: "✓ `_codev/config.yaml` enriched."

If `no`:

- Write nothing.
- Say: "No changes made. Run `/codev-configure` again whenever you
  want to retry."

## Output

The proposed diff, the confirmation question, and depending on the
answer: a write acknowledgment or a polite refusal.

## Guardrails

- **No writing before explicit confirmation** — not even partially, not
  even "just to test".
- **Strictly preserved fields**: `schema`, `workflows`, `mcp`,
  `inherits`. If the patch touched anything else, that is a bug in the
  skill: stop and report it to the user.
- **Refuse if `_codev/config.yaml` is missing** — this skill never
  creates it from scratch.
- **Respect the reading budget** — at most 5 docs and 8 source files,
  a single level of `ls`. A bigger project does not deserve more
  reading: what matters fits in the most-touched files.
