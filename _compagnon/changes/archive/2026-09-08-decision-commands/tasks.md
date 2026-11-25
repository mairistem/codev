# Tasks

## 1. Embedded skeleton

- [x] 1.1 Create `assets/templates/decision.md` with the four sections
      (Context, Decision, Consequences, Rejected alternatives), plus a
      placeholder for the frontmatter in the form `{{FRONTMATTER}}` that
      `plan_new` will replace. Verified by
      `codev_engine::decisions::actions::squelette_est_embarque`.

## 2. `plan_new` — creation on the engine side

- [x] 2.1 Create `codev-engine::decisions::actions` with `pub fn
      plan_new(index: &DecisionIndex, title: &str, status: DecisionStatus,
      today: &str, layout: &Layout) -> Result<CreatePlan, ActionError>`
      which returns `CreatePlan { plan: Plan, new_id: String, new_path:
      PathBuf }`. Verified by `actions::plan_new_dans_projet_vide_produit_0001`.
- [x] 2.2 Numbering: the next id is `max(entries.decision.id
      interpreted as integers, project only) + 1`, formatted on four
      digits. Inherited decisions do not count — they live in their own
      numbering space. Verified by
      `actions::numerotation_ignore_les_sources_heritees`.
- [x] 2.3 Title slug: lowercase, hyphens, no non-alphanumeric ASCII
      characters; fallback `decision` if the slug is empty. Verified by
      `actions::slug_du_titre` (a table of cases:
      "Normal title" → `normal-title`, "Emoji: 🎉" → `emoji`,
      "???" → `decision`).
- [x] 2.4 Empty title → `ActionError::EmptyTitle` (stable code
      `empty_title`), no plan produced. Verified by
      `actions::titre_vide_refuse`.
- [x] 2.5 The plan produces a single write, `WriteMode::CreateOnly` on
      `_codev/decisions/<id>-<slug>.md`. If the file already exists, the
      exec will fail with the port error `already_exists` — handled by the
      existing `apply::execute`. Verified by
      `actions::plan_new_est_create_only`.

## 3. `plan_supersede` — supersession on the engine side

- [x] 3.1 `pub fn plan_supersede(index: &DecisionIndex, source_content:
      impl Fn(&Path)->io::Result<String>, old_id: &str, new_title: &str,
      today: &str, layout: &Layout) -> Result<SupersedePlan,
      ActionError>` which returns `SupersedePlan { plan: Plan, new_id,
      new_path, old_qualified_id }`. Verified by
      `actions::plan_supersede_produit_deux_ecritures`.
- [x] 3.2 Resolving the old one: if `old_id` matches a single local
      entry, that one is taken; otherwise `ActionError::UnknownDecisionId`
      (code `unknown_decision_id`). An inherited entry →
      `ActionError::CannotSupersedeInherited` (code
      `cannot_supersede_inherited`) — the message suggests deviation.
      Verified by `actions::supersede_id_inconnu_refuse` and
      `actions::supersede_source_heritee_refuse`.
- [x] 3.3 New frontmatter of the predecessor: keeps all the original
      fields but changes to `status: superseded` (without adding
      `date_of_supersession` or any other undocumented field). Verified by
      `actions::frontmatter_supersede_conserve_champs_dorigine`.
- [x] 3.4 Targeted rewrite of the predecessor: the plan carries a
      `WriteMode::Overwrite` on the existing file with the content
      `<new frontmatter>\n<original body unchanged>`. The body is the
      slice `source[frontmatter_span.end..]` — no re-parse, preserves the
      content byte for byte. Verified by
      `actions::supersede_ne_touche_pas_au_corps_du_predecesseur`
      (golden test).
- [x] 3.5 The plan ALSO carries the `WriteMode::CreateOnly` of the new
      ADR, which references the old one via `supersedes: [<old_id>]`.
      Verified by `actions::plan_supersede_inclut_la_nouvelle_decision`.

## 4. JSON contract v1

- [x] 4.1 Three new types in `contract::v1`, camelCase:
      `DecisionV1 { id, qualifiedId, title, status, date, tags,
      supersedes, path, origin, inEffect, supersededBy }`,
      `DecisionCreatedV1 { decision, path, status }`,
      `DecisionSupersededV1 { newDecision, oldId, oldPath, status }`.
      Verified by `contract::decision_v1_shape_stable`.
- [x] 4.2 Test that `list --json` returns a `decisions:
      [DecisionV1]` array. Verified by
      `contract::decision_list_report_shape_stable`.

## 5. CLI subcommands

- [x] 5.1 Add the `Command::Decision` group with subcommands
      `New`, `List`, `Show`, `Supersede`. Verified by
      `cli_decision::sous_commandes_declarees`.
- [x] 5.2 `codev decision new <title> [--status <s>]`: builds
      `plan_new`, executes, returns `DecisionCreatedV1` or its human
      equivalent. Exit 0 on success. Verified by
      `cli_decision::new_dans_projet_vide_cree_0001` (via the in-memory
      harness).
- [x] 5.3 `codev decision list [--json]`: reads the index and returns one
      `DecisionV1` per entry. Verified by
      `cli_decision::list_rend_les_decisions_avec_leur_effet`.
- [x] 5.4 `codev decision show <id> [--json]`: resolves a short or
      qualified `id`, reads the file, returns its content (JSON =
      `DecisionV1`, human = formatted frontmatter + raw body). Ambiguous →
      `ambiguous_decision_id`. Verified by
      `cli_decision::show_ambigu_liste_les_qualifieurs`.
- [x] 5.5 `codev decision supersede <old-id> <new-title> [--status
      <s>]`: builds `plan_supersede`, executes. Verified by
      `cli_decision::supersede_reecrit_lancien_et_cree_le_nouveau`.

## 6. Human rendering

- [x] 6.1 `render::decision_list`: one line per entry, format
      `<marker> <id> <title> [<status>]` where marker = `•` if `in_effect`,
      otherwise `–`. Verified by
      `render::liste_marque_les_decisions_en_vigueur`.
- [x] 6.2 `render::decision_show`: frontmatter highlighted
      (`ID / Title / Status / Date / Tags`), then the raw body. Verified by
      `render::show_expose_frontmatter_puis_corps`.
- [x] 6.3 `render::decision_created` and `render::decision_superseded`
      announce the action, one line per touched file. Verified by
      `render::annonce_created` and `render::annonce_superseded`.

## 7. Dogfooding and integration

- [x] 7.1 Run `codev decision new "A first test of the commands"`
      in this repository, check that `_codev/decisions/0007-a-first-test-…md`
      is created with the right frontmatter. **Then delete it** — this
      decision is a test artifact, not a real architecture choice.
- [x] 7.2 `codev decision list` on this repository shows the six existing
      ADRs with the `•` marker for each.
- [x] 7.3 `codev decision show 0001` shows ADR 0001 in full.
- [x] 7.4 `cargo test --workspace` stays green and counts at least 20
      additional tests (plans + contract + rendering + CLI).
- [x] 7.5 `cargo clippy --workspace --all-targets` stays warning-free.
- [x] 7.6 `codev validate --all` stays green.
