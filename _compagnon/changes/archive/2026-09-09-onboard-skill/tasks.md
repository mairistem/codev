# Tasks

## 1. Skill content

- [x] 1.1 Write `assets/workflows/onboard.md` — the skill's body.
      Expected content, in French, framed by the spec's scenarios:
      - **input**: none (the skill is invoked without argument, with or
        without a description);
      - **step 1 — description**: three sentences on what compagnon is
        (versioned planning, propose → apply → archive cycle, drivable
        by Claude Code);
      - **step 2 — state**: read the human outputs of
        `compagnon list`, `compagnon list --specs`, `compagnon decision list` (if
        it already exists via K3 or K6). Count, do not list
        exhaustively.
      - **step 3 — recommendation**: the case tree from the design's
        Decisions section (5 branches, mutually exclusive).
      - **guardrails**: never write, never run `compagnon init` in the
        user's place, never suggest an opt-in skill absent from the
        installed catalog.
      - **output format**: three distinct blocks ("compagnon is",
        "here you have", "what's next"), separated by a blank line, markdown
        format readable in a terminal.
      Checked by the file's presence and by the CATALOG invariant.

## 2. Entry in the CATALOG

- [x] 2.1 Add `Workflow { id: "onboard", description: "…",
      allowed-tools: "Bash(compagnon:*), Read, Glob", body:
      include_str!("../../../assets/workflows/onboard.md") }` to the
      `CATALOG`. Description: "Introduce compagnon to a user discovering
      it: what the tool does, the current state of the project, and
      the recommended next action. Strictly read-only — modifies or
      creates nothing."
- [x] 2.2 Add `"onboard"` to `DEFAULT_WORKFLOWS`. The list goes from
      `["propose", "explore"]` to `["propose", "explore",
      "onboard"]`.
- [x] 2.3 Dedicated test
      `workflows::onboard_est_dans_le_catalogue_et_a_les_bons_outils`:
      `find("onboard")` returns `Some`; `allowed_tools` equals exactly
      `"Bash(compagnon:*), Read, Glob"`; does NOT contain general
      `Bash`.
- [x] 2.4 Complementary test
      `workflows::onboard_est_dans_le_catalogue_par_defaut`:
      `DEFAULT_WORKFLOWS.contains(&"onboard")` is `true`.
- [x] 2.5 Adapt the existing test
      `sans_demande_installe_le_catalogue_par_defaut` — expects a list
      of three entries `["propose", "explore", "onboard"]`; the
      `for opt_in` array drops `"onboard"` (`onboard` is now in the
      default, not opt-in).
- [x] 2.6 Complementary test `onboard_cite_ses_trois_blocs`: the body
      contains the strings "compagnon" (description), "here" (state) and
      "next" or "recommend" (action) — a tracer for an accidental
      rewrite of the body.
- [x] 2.7 The existing invariants (`chaque_workflow_a_un_corps_…`,
      `le_frontmatter_de_chaque_skill_est_du_yaml_valide`) automatically
      cover `onboard`. Checked by `cargo test -p compagnon-agents`.

## 3. Repository config

- [x] 3.1 Add `- onboard` to the `workflows` list of this project's
      `_compagnon/config.yaml` — to dogfood the skill.
- [x] 3.2 Update the example comment `# workflows:` of the
      `DEFAULT_CONFIG` in `crates/compagnon-engine/src/scaffold.rs` to also
      list `- onboard` (the six opt-in workflows + onboard) —
      pedagogical consistency with what new projects see.

## 4. Dogfooding and workspace integration

- [x] 4.1 `cargo test --workspace` stays green, gains at least 3
      dedicated tests (`onboard_est_dans_le_catalogue…`,
      `onboard_est_dans_le_catalogue_par_defaut`,
      `onboard_cite_ses_trois_blocs`).
- [x] 4.2 `cargo clippy --workspace --all-targets` stays free of
      warnings.
- [x] 4.3 After `cargo install --path crates/compagnon-cli` then `compagnon
      update`, check that `.claude/skills/compagnon-onboard/SKILL.md`
      appears with the right frontmatter.
- [x] 4.4 Check that the list of skills announced by `compagnon update`
      does contain the seven workflows: `compagnon-propose,
      compagnon-explore, compagnon-apply, compagnon-sync, compagnon-archive,
      compagnon-update, compagnon-onboard`.
- [x] 4.5 `compagnon validate --strict --all` stays green on this
      repository.
