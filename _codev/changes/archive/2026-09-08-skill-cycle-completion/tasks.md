# Tasks

## 1. Content of the two skills

- [x] 1.1 Write `assets/workflows/sync.md` — the body of the `sync` skill.
      Expected content:
      - input: optional change name, implicit resolution (only one active);
      - prior check: `codev status --change <name> --json` to confirm that
        planning is complete (`isPlanningComplete`);
      - action: `codev sync <name> --json`;
      - reading the `SyncReportV1`: `changeName`, `created[]`, `updated[]`,
        `unchanged[]`; structured rendering ("✓ N spec(s) created, M
        updated, K unchanged" then a list per category);
      - conditional ending: if `created` or `updated` is non-empty, add
        "The change is ready to be archived if you want to close the
        cycle."; otherwise, suggest nothing.
      Verified by the file's presence and by the CATALOG invariant.
- [x] 1.2 Write `assets/workflows/archive.md` — the body of the `archive`
      skill. Expected content:
      - input: optional change name, implicit resolution;
      - prior check: `codev status --change <name> --json` to confirm that
        planning is complete;
      - action: `codev archive <name> --json`;
      - reading the `ArchiveReportV1` on success (exit 0): structured
        rendering (counts per category + `movedTo` on its own line);
      - on non-zero exit: read the JSON's root `status` array; if a
        `code == "validation_failed"` is found, say exactly
        "The change has errors. Run `codev validate <name>` to see the
        details."; for any other error code, relay the JSON's `message`
        field as is — no guessing, no retrying.
      Verified by the file's presence and by the CATALOG invariant.

## 2. CATALOG entries

- [x] 2.1 Add `Workflow { id: "sync", description: "…", allowed-tools:
      "Bash(codev:*), Read", body: include_str!("../../../assets/workflows/sync.md") }`
      to the `CATALOG` of `codev-agents::workflows`. Description: "Sync
      the deltas of an already planned codev change into the main specs,
      without moving the change. Use when a new capability must appear in
      the specs before being consumed by another change. Does not
      archive."
- [x] 2.2 Add `Workflow { id: "archive", description: "…", allowed-tools:
      "Bash(codev:*), Read", body: include_str!("../../../assets/workflows/archive.md") }`.
      Description: "Close a codev change: merge its deltas into the main
      specs and move the folder to the dated archive. Refuses to act if
      validation reports errors."

## 3. Invariant tests

- [x] 3.1 Add a test
      `workflows::cycle_completion_skills_present_et_restreintes` that checks:
      `find("sync")` and `find("archive")` return `Some`; their
      `allowed_tools` equal exactly `"Bash(codev:*), Read"`; neither one
      contains `Bash` other than in the `Bash(codev:*)` prefix.
- [x] 3.2 The test `apply_est_dans_le_catalogue_et_a_les_bons_outils` keeps
      counting "exactly zero other workflows" that would have general
      `Bash`. Verified by rerunning `cargo test -p codev-agents`.
- [x] 3.3 The invariant `chaque_workflow_a_un_corps_et_une_description_utilisables`
      covers the two new workflows automatically (loop over `CATALOG`).
      Verified in the same pass.
- [x] 3.4 The invariant `le_frontmatter_de_chaque_skill_est_du_yaml_valide`
      also covers the two new frontmatters automatically. Verified in the
      same pass.
- [x] 3.5 Update `sans_demande_installe_le_catalogue_par_defaut` to assert
      that `DEFAULT_WORKFLOWS` contains neither `"sync"` nor `"archive"` —
      symmetrically to what is done for `apply`.
- [x] 3.6 Add to `assets/workflows/sync.md` and `archive.md` references by
      name to the JSON contract fields (`SyncReportV1`, `ArchiveReportV1`,
      `changeName`, `created`, `updated`, `unchanged`, `movedTo`,
      `status[].code`) — so that a `grep` on these names from a renamed
      `contract.rs` turns up the affected skills. Verified by a test
      `workflows::sync_et_archive_citent_les_champs_du_contrat` that looks
      for these strings in the `body` of both workflows.

## 4. This repository's config and dogfooding

- [x] 4.1 Add `- sync` and `- archive` to the `workflows` list of this
      project's `_codev/config.yaml`.
- [x] 4.2 Run `cargo install --path crates/codev-cli` then `codev update`.
      Check that `.claude/skills/codev-sync/SKILL.md` and
      `.claude/skills/codev-archive/SKILL.md` appear with the right
      frontmatter.
- [x] 4.3 Check that the list of skills announced by `codev update`
      does contain the five workflows: `codev-propose, codev-explore,
      codev-apply, codev-sync, codev-archive`.

## 5. Workspace integration

- [x] 5.1 `cargo test --workspace` stays green and counts at least 1
      additional test (`cycle_completion_skills_present_et_restreintes`).
- [x] 5.2 `cargo clippy --workspace --all-targets` stays warning-free.
- [x] 5.3 `codev validate --all` stays green.
