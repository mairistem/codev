# Tasks

## 1. Skill content

- [x] 1.1 Write `assets/workflows/apply.md` — the skill body. Expected
      content, framed by the spec's scenarios:
      - input: optional change name; implicit resolution if only one is
        active; question to the user if ambiguous (list of candidates);
      - opening guardrails: this is an implementation workflow, it modifies
        project code; it does not modify other changes; it neither archives
        nor syncs — that is an explicit next step;
      - reading: `codev status --change <name> --json` first, to check that
        planning is complete; then `Read` on `tasks.md`;
      - loop: for each `- [ ]` task in file order, read it aloud, do it,
        check it with `Edit` (`- [ ]` → `- [x]` on the exact line), short
        announcement;
      - stop on material ambiguity or blockage; do not check the box until
        clarification is obtained; suggest a split into `X.Y.a`/`X.Y.b`
        without imposing it;
      - end: when everything is checked, summarize and invite to
        `/codev-archive` or `codev archive` as an explicit next step.
      Verified by the file's presence and by the CATALOG invariants
      (tasks 2.x).

## 2. Catalog entry

- [x] 2.1 Add a `Workflow { id: "apply", description: "…", allowed-tools:
      "Bash(codev:*), Read, Write, Edit, Glob, Grep, Bash", body:
      include_str!("../../../assets/workflows/apply.md") }` to the `CATALOG`
      of `codev-agents::workflows`. The description is longer than 60
      characters and clearly describes when to invoke the skill
      ("Implement the tasks of a change… modifies neither the specs nor
      the other changes"). Verified by the existing CATALOG invariants
      (`chaque_workflow_a_un_corps_et_une_description_utilisables`).
- [x] 2.2 Add a dedicated test
      `workflows::apply_est_dans_le_catalogue_et_a_les_bons_outils` that
      checks: `find("apply")` returns `Some`, `allowed_tools` contains both
      `Bash(codev:*)` and general `Bash` (the only workflow in the catalog
      to request the latter).
- [x] 2.3 Frontmatter invariant test:
      `claude::le_frontmatter_de_chaque_skill_est_du_yaml_valide` (existing)
      must pass without modification — its loop iterates over `CATALOG` and
      the new workflow will be covered automatically. Verified by
      `cargo test -p codev-agents`.

## 3. Config and default catalog

- [x] 3.1 Do NOT add `apply` to `DEFAULT_WORKFLOWS` — cf. the design
      decision. Verified by
      `workflows::sans_demande_installe_le_catalogue_par_defaut`, which
      stays unchanged.
- [x] 3.2 Update the example comment in
      `crates/codev-engine/src/scaffold.rs::DEFAULT_CONFIG` to also list
      `apply` in the commented `# workflows:` block, without uncommenting
      it. Verified by `scaffold::la_config_par_defaut_est_valide_et_relisible`,
      which must stay green (the block is commented out, YAML ignores it).

## 4. This repository's config and dogfooding

- [x] 4.1 Add `- apply` to the `workflows` list of this project's
      `_codev/config.yaml`, so that `codev init` / `codev update` installs it.
- [x] 4.2 Run `cargo install --path crates/codev-cli` then `codev update`
      in this repository. Check that `.claude/skills/codev-apply/SKILL.md`
      appears, with the right frontmatter and the expected body.
- [x] 4.3 Check that the generated frontmatter is valid YAML via
      `python3 -c "import yaml; yaml.safe_load(open('.claude/skills/codev-apply/SKILL.md').read().split('---')[1])"`
      (or equivalent — the unit test already covers the invariant; this
      one serves as confirmation on the actual written file).

## 5. Workspace integration

- [x] 5.1 `cargo test --workspace` stays green and counts at least 2
      additional tests (added invariants).
- [x] 5.2 `cargo clippy --workspace --all-targets` stays warning-free.
- [x] 5.3 `codev validate --all` stays green (the change still active is
      this change itself, plus two main specs).
