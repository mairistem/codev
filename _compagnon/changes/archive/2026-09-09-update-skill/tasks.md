# Tasks

## 1. Skill content

- [x] 1.1 Write `assets/workflows/update.md` — the skill's body.
      Expected content, in French, framed by the spec's scenarios:
      - input: optional `<artifact-id>` + free-form description of the
        revision; if no artifact is named, the skill asks which one
        (it does not guess);
      - change resolution: by argument, or implicit if there is only
        one active change; refusal if the name points to an archived
        one;
      - opening safeguards: the skill **never** modifies code,
        **never** creates a missing artifact, **never** touches an
        archived change — each case explicitly redirects to the
        appropriate skill;
      - prior reading: `codev status --change <name> --json` to check
        that the requested artifact exists, then reading from disk
        (never from the conversation);
      - study of what exists: read the change's other artifacts
        before writing, to spot a possible ripple before it surfaces
        at write time;
      - applying the revision: `Edit` for a targeted modification,
        `Write` for a full rewrite, at the skill's judgment depending
        on the scope;
      - detecting and reporting the ripple: if revising one artifact
        makes another inconsistent (capability removed from the
        proposal while a `specs/<capa>/` exists, a decision cited by
        the design that no longer exists, a task referencing a
        vanished scenario…), the skill **names** the inconsistency
        and **proposes** the action (delete, adjust, another
        `/codev-update` call), without applying it without
        confirmation;
      - final safeguard: `codev validate <change>` and relay of the
        report;
      - end: summary — file(s) touched, `validate` verdict,
        recommended next action (often `/codev-apply` or
        `/codev-archive`).
      Verified by the presence of the file and by the CATALOG
      invariant.

## 2. CATALOG entry

- [x] 2.1 Add `Workflow { id: "update", description: "…",
      allowed-tools: "Bash(codev:*), Read, Write, Edit, Glob, Grep",
      body: include_str!("../../../assets/workflows/update.md") }` to
      the `CATALOG`. Description: "Revise an already-written planning
      artifact of an active codev change — proposal, specs, design or
      tasks — while preserving consistency with the other artifacts.
      Modifies no project code, creates no missing artifact, touches
      no archived change."
- [x] 2.2 Dedicated test
      `workflows::update_est_dans_le_catalogue_et_a_les_bons_outils`:
      `find("update")` returns `Some`; `allowed_tools` equals exactly
      `"Bash(codev:*), Read, Write, Edit, Glob, Grep"`; does NOT
      contain general `Bash` — checks that "only `apply` has general
      Bash" stays true.
- [x] 2.3 Complementary test
      `workflows::update_cite_ses_frontieres`: the workflow's `body`
      contains the strings "does not modify" and "archived" — a
      tracer for an accidental renaming or removal of the safeguards.
- [x] 2.4 The existing invariants (`chaque_workflow_a_un_corps_…`,
      `le_frontmatter_de_chaque_skill_est_du_yaml_valide`)
      automatically cover `update` via the loop over `CATALOG`.
      Verified by `cargo test -p codev-agents`.

## 3. Repository config and default catalog

- [x] 3.1 Do NOT add `update` to `DEFAULT_WORKFLOWS`. The test
      `sans_demande_installe_le_catalogue_par_defaut` gains an extra
      check — an array `for opt_in in ["apply", "sync",
      "archive", "update"]`.
- [x] 3.2 Update the example comment in
      `crates/codev-engine/src/scaffold.rs::DEFAULT_CONFIG` to also
      list `update` in the commented-out `# workflows:` block.
- [x] 3.3 Add `- update` to the `workflows` list of this project's
      `_codev/config.yaml`.

## 4. Dogfooding

- [x] 4.1 After `cargo install --path crates/codev-cli` then `codev
      update`, check that `.claude/skills/codev-update/SKILL.md`
      appears with the right frontmatter.
- [x] 4.2 Check that the list of skills announced by `codev update`
      does contain the six workflows: `codev-propose, codev-explore,
      codev-apply, codev-sync, codev-archive, codev-update`.
- [x] 4.3 Once the change is applied and archived, try a real
      `/codev-update` on a future change in the next Claude Code
      session — the skill will be visible at load time.

## 5. Workspace integration

- [x] 5.1 `cargo test --workspace` stays green and counts at least 2
      additional tests (`update_est_dans_le_catalogue…` and
      `update_cite_ses_frontieres`).
- [x] 5.2 `cargo clippy --workspace --all-targets` stays free of
      warnings.
- [x] 5.3 `codev validate --all` stays green.
