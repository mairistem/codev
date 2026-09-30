# Tasks

## 1. Diagram — state machine of a change (§3)

- [x] 1.1 Edit `docs/codev.md`: in **section 3 (The cycle)**,
      just before the existing ASCII art, insert a
      ` ```mermaid` block of type `stateDiagram-v2` covering:
      - states: `proposed`, `applied`, `synced`, `archived`;
      - transitions: `propose` (initial → proposed), `apply`
        (proposed → applied), `sync` (applied → synced), `archive`
        (applied → archived, synced → archived);
      - notes on the key guards: `skip_specs`, `validate --strict`,
        "planning complete" before apply.
- [x] 1.2 Add a caption sentence under the diagram:
      "`sync` is a variant of `archive` that does not move the
      folder — useful to make a new capability available
      without filing the change away."

## 2. Tutorial — §3.5 "Your first change, in five minutes"

- [x] 2.1 Add at the end of section 3 (just before the `---`
      separator that precedes section 4) a new subsection:
      ```markdown
      ### 3.5. Your first change, in five minutes
      ```
- [x] 2.2 Introduction (2-3 sentences) — set up the concrete case: "we
      are going to add a `--json` option to `codev list`". Specify:
      a deliberately simple case, serving as a common thread to see the whole
      cycle in practice. Two paths offered at each step:
      Claude Code (skill) or pure CLI.
- [x] 2.3 Step 1 — **create the change**. Skill path:
      `/codev-propose add-list-json`. CLI path:
      `codev new change add-list-json --goal "Add --json to codev list"`.
      Expected output (`text` block):
      ```
      Change "add-list-json" created
        Location  <path>/_codev/changes/add-list-json
        Schema    spec-driven
      ```
- [x] 2.4 Step 2 — **write the artifacts**. Redirect to §3.1
      for the full format. Show a mini-example of
      `proposal.md` (5 lines) and the skeleton of the delta
      `specs/cli-list/spec.md`.
- [x] 2.5 Step 3 — **statuses**. `codev status --change add-list-json`,
      typical output:
      ```
        [x] proposal
        [x] specs
        [x] design
        [x] tasks
      Planning: 4/4 artifacts
      ```
- [x] 2.6 Step 4 — **implement**. Skill path: `/codev-apply
      add-list-json` (Claude Code reads `tasks.md`, checks items off as it goes).
      Manual path: edit the code, check the boxes by hand.
- [x] 2.7 Step 5 — **validate**. `codev validate add-list-json`,
      typical output ✓ no defects.
- [x] 2.8 Step 6 — **archive**. `/codev-archive add-list-json` or
      `codev archive --change add-list-json`. Paste a simplified
      excerpt of the `ArchiveReportV1` as output.
- [x] 2.9 Closing: a sentence "what you just did can be
      repeated identically for any change, from a one-line
      fix to the refactor of an entire capability", plus a link
      to §5 (Concepts) and §6 (CLI) to go further.

## 3. Diagram — crate graph (§5)

- [x] 3.1 Open `docs/codev.md` §5 (Concepts). Check that there is
      indeed an "Architecture" subheading (or equivalent); otherwise,
      add one (`### Architecture`).
- [x] 3.2 Insert a ` ```mermaid` `graph LR` block covering:
      - nodes: `codev-core` (labeled "pure, no I/O"),
        `codev-engine` (labeled "effects via ports"), `codev-cli`
        (labeled "imperative shell"), `codev-agents` (labeled
        "Claude Code target");
      - edges: `codev-cli --> codev-engine`,
        `codev-engine --> codev-core`, `codev-cli --> codev-agents`,
        `codev-agents --> codev-core`;
      - a style that distinguishes `codev-core` (the core) from the three
        others.
- [x] 3.3 Caption under the diagram (2-3 lines): "the arrows
      point to dependencies. The graph is acyclic and
      Cargo checks it at compile time — this is our concrete
      application of the dependency rule (see ADR 0002)."

## 4. Diagram — lifecycle of a delta (§5)

- [x] 4.1 Find in §5 the "Deltas" subsection (existing) or
      the sub-part that describes `ADDED/MODIFIED/REMOVED/RENAMED`.
- [x] 4.2 Insert a ` ```mermaid` `sequenceDiagram` block covering:
      - actors: `Agent` (or `You`), `codev-cli`, `change spec`,
        `main spec`;
      - sequence: Agent → cli (`propose`), cli → change-spec
        (creates `specs/<capa>/spec.md`); later Agent → cli
        (`archive`), cli → main-spec (merge), cli → Agent
        (`ArchiveReportV1` report).
- [x] 4.3 Caption under the diagram (2-3 lines): "the main
      spec is never edited directly — it receives its
      modifications by merging deltas at `sync` or
      `archive` time."

## 5. Final checks

- [x] 5.1 `codev validate --strict` stays green.
- [x] 5.2 `cargo test --workspace` stays green (nothing in Rust has
      changed, but we check).
- [x] 5.3 Rebuild and run `codev docs --write /tmp/manuel.html`,
      grep:
      - `Your first change` (the tutorial is indeed embedded);
      - `stateDiagram-v2` (the Mermaid source appears in the HTML);
      - `graph LR` (same).
- [ ] 5.4 Push and open `docs/codev.md` on github.com — visually check
      that the three Mermaid diagrams render, and that
      the tutorial is correctly formatted.

## 6. Delivery

- [ ] 6.1 Bump `Cargo.toml` to `0.2.2` (patch — doc content). Tag
      `v0.2.2`, `git push origin main --tags`. The release workflow
      publishes automatically.
- [ ] 6.2 Add the `[0.2.2] — 2026-09-24` entry at the top of
      `CHANGELOG.md`.
