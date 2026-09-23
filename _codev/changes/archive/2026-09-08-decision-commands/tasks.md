# Tâches

## 1. Squelette embarqué

- [x] 1.1 Créer `assets/templates/decision.md` avec les quatre sections
      (Contexte, Décision, Conséquences, Alternatives écartées), plus un
      placeholder pour le frontmatter au format `{{FRONTMATTER}}` que
      `plan_new` remplacera. Vérifié par
      `codev_engine::decisions::actions::squelette_est_embarque`.

## 2. `plan_new` — création côté engine

- [x] 2.1 Créer `codev-engine::decisions::actions` avec `pub fn
      plan_new(index: &DecisionIndex, title: &str, status: DecisionStatus,
      today: &str, layout: &Layout) -> Result<CreatePlan, ActionError>`
      qui rend `CreatePlan { plan: Plan, new_id: String, new_path:
      PathBuf }`. Vérifié par `actions::plan_new_dans_projet_vide_produit_0001`.
- [x] 2.2 Numérotation : le prochain id est `max(entries.decision.id
      interprétés comme entiers, projet uniquement) + 1`, formaté à
      quatre chiffres. Les décisions héritées ne comptent pas — elles
      vivent dans leur propre espace de numérotation. Vérifié par
      `actions::numerotation_ignore_les_sources_heritees`.
- [x] 2.3 Slug du titre : minuscules, tirets, sans caractères non
      alphanumériques ASCII ; fallback `decision` si le slug est vide.
      Vérifié par `actions::slug_du_titre` (une table de cas :
      « Titre normal » → `titre-normal`, « émoji : 🎉 » → `emoji`,
      « ??? » → `decision`).
- [x] 2.4 Titre vide → `ActionError::EmptyTitle` (code stable
      `empty_title`), aucun plan produit. Vérifié par
      `actions::titre_vide_refuse`.
- [x] 2.5 Le plan produit une seule écriture, `WriteMode::CreateOnly` sur
      `_codev/decisions/<id>-<slug>.md`. Si le fichier existe déjà, l'exec
      échouera avec l'erreur port `already_exists` — géré par
      `apply::execute` existant. Vérifié par
      `actions::plan_new_est_create_only`.

## 3. `plan_supersede` — supersession côté engine

- [x] 3.1 `pub fn plan_supersede(index: &DecisionIndex, source_content:
      impl Fn(&Path)->io::Result<String>, old_id: &str, new_title: &str,
      today: &str, layout: &Layout) -> Result<SupersedePlan,
      ActionError>` qui rend `SupersedePlan { plan: Plan, new_id, new_path,
      old_qualified_id }`. Vérifié par
      `actions::plan_supersede_produit_deux_ecritures`.
- [x] 3.2 Résolution de l'ancien : si le `old_id` correspond à une seule
      entrée locale, on prend celle-là ; sinon `ActionError::UnknownDecisionId`
      (code `unknown_decision_id`). Une entrée héritée →
      `ActionError::CannotSupersedeInherited` (code
      `cannot_supersede_inherited`) — le message suggère la déviation.
      Vérifié par `actions::supersede_id_inconnu_refuse` et
      `actions::supersede_source_heritee_refuse`.
- [x] 3.3 Nouveau frontmatter du prédécesseur : reprend tous les champs
      d'origine mais change `status: superseded` (sans ajouter `date_of_supersession`
      ou autre champ non documenté). Vérifié par
      `actions::frontmatter_supersede_conserve_champs_dorigine`.
- [x] 3.4 Réécriture ciblée du prédécesseur : le plan porte un
      `WriteMode::Overwrite` sur le fichier existant avec le contenu
      `<nouveau frontmatter>\n<corps original inchangé>`. Le corps est le
      slice `source[frontmatter_span.end..]` — pas de re-parse, préserve
      le contenu à l'octet près. Vérifié par
      `actions::supersede_ne_touche_pas_au_corps_du_predecesseur`
      (golden test).
- [x] 3.5 Le plan porte AUSSI le `WriteMode::CreateOnly` du nouvel ADR,
      qui référence l'ancien via `supersedes: [<old_id>]`. Vérifié par
      `actions::plan_supersede_inclut_la_nouvelle_decision`.

## 4. Contrat JSON v1

- [x] 4.1 Trois nouveaux types dans `contract::v1`, camelCase :
      `DecisionV1 { id, qualifiedId, title, status, date, tags,
      supersedes, path, origin, inEffect, supersededBy }`,
      `DecisionCreatedV1 { decision, path, status }`,
      `DecisionSupersededV1 { newDecision, oldId, oldPath, status }`.
      Vérifié par `contract::decision_v1_shape_stable`.
- [x] 4.2 Test que `list --json` rend un tableau `decisions:
      [DecisionV1]`. Vérifié par
      `contract::decision_list_report_shape_stable`.

## 5. Sous-commandes CLI

- [x] 5.1 Ajouter le groupe `Command::Decision` avec sous-commandes
      `New`, `List`, `Show`, `Supersede`. Vérifié par
      `cli_decision::sous_commandes_declarees`.
- [x] 5.2 `codev decision new <title> [--status <s>]` : construit
      `plan_new`, exécute, rend `DecisionCreatedV1` ou son équivalent
      humain. Exit 0 sur succès. Vérifié par
      `cli_decision::new_dans_projet_vide_cree_0001` (via harnais en
      mémoire).
- [x] 5.3 `codev decision list [--json]` : lit l'index et rend
      `DecisionV1` par entrée. Vérifié par
      `cli_decision::list_rend_les_decisions_avec_leur_effet`.
- [x] 5.4 `codev decision show <id> [--json]` : résout `id` court ou
      qualifié, lit le fichier, rend son contenu (JSON = `DecisionV1`,
      humain = frontmatter formaté + corps brut). Ambigu →
      `ambiguous_decision_id`. Vérifié par
      `cli_decision::show_ambigu_liste_les_qualifieurs`.
- [x] 5.5 `codev decision supersede <old-id> <new-title> [--status
      <s>]` : construit `plan_supersede`, exécute. Vérifié par
      `cli_decision::supersede_reecrit_lancien_et_cree_le_nouveau`.

## 6. Rendu humain

- [x] 6.1 `render::decision_list` : une ligne par entrée, format
      `<marqueur> <id> <title> [<status>]` où marqueur = `•` si `in_effect`
      sinon `–`. Vérifié par
      `render::liste_marque_les_decisions_en_vigueur`.
- [x] 6.2 `render::decision_show` : frontmatter mis en évidence
      (`ID / Titre / Statut / Date / Tags`), puis corps brut. Vérifié par
      `render::show_expose_frontmatter_puis_corps`.
- [x] 6.3 `render::decision_created` et `render::decision_superseded`
      annoncent l'action, une ligne par fichier touché. Vérifié par
      `render::annonce_created` et `render::annonce_superseded`.

## 7. Dogfooding et intégration

- [x] 7.1 Lancer `codev decision new "Un premier test des commandes"`
      dans ce dépôt, vérifier que `_codev/decisions/0007-un-premier-test-…md`
      est créé avec le bon frontmatter. **Puis le supprimer** — cette
      décision est un artefact de test, pas un vrai choix d'architecture.
- [x] 7.2 `codev decision list` sur ce dépôt affiche les six ADR
      existants avec le marqueur `•` pour chacun.
- [x] 7.3 `codev decision show 0001` affiche l'ADR 0001 en entier.
- [x] 7.4 `cargo test --workspace` reste vert et compte au moins 20 tests
      supplémentaires (plans + contrat + rendu + CLI).
- [x] 7.5 `cargo clippy --workspace --all-targets` reste sans avertissement.
- [x] 7.6 `codev validate --all` reste vert.
