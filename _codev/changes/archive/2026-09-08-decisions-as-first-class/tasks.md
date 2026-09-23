# Tâches

## 1. Parseur pur dans `codev-core::decisions`

- [x] 1.1 Créer `codev-core::decisions` (`decisions/mod.rs`,
      `decisions/ast.rs`, `decisions/parser.rs`), exposer `Decision`,
      `DecisionStatus`, `parse_decision`. Vérifié par `cargo build -p
      codev-core`.
- [x] 1.2 Types AST : `Decision { id: String, title: String, status:
      DecisionStatus, date: String, tags: Vec<String>, supersedes:
      Vec<String>, sections: Vec<Section>, span: Span }`, énumération
      `DecisionStatus { Accepted, Superseded, Proposed, Deprecated,
      Rejected, Unknown(String) }`. Vérifié par
      `decisions::les_types_dast_se_construisent_a_la_main`.
- [x] 1.3 Codes stables des findings dans `parser/codes.rs` (module
      centralisé) : `decision_missing_frontmatter`,
      `decision_missing_field`, `decision_unknown_status`,
      `decision_supersedes_unknown`, `decision_id_collision`,
      `decision_supersession_cycle`. Ajoutés à la constante `ALL` pour que
      l'invariant d'unicité de `validate` les couvre. Vérifié par
      `validate::codes_de_findings_sont_uniques`.
- [x] 1.4 `parse_decision` : split sur `---\n`, extraction du frontmatter
      via `serde_norway::from_str` avec `deny_unknown_fields`, parsing du
      corps en sections `##` via `parser::shared`. Vérifié par
      `decisions::parse_adr_bien_forme` (scénario homonyme).
- [x] 1.5 `parse_decision` sans frontmatter → finding
      `decision_missing_frontmatter`, résultat vide. Vérifié par
      `decisions::parse_sans_frontmatter_signale`.
- [x] 1.6 `parse_decision` avec champ obligatoire manquant → finding
      `decision_missing_field` nommant le champ. Vérifié par
      `decisions::parse_sans_titre_signale`.
- [x] 1.7 `parse_decision` avec `status` inconnu → variant `Unknown(...)`
      + finding `decision_unknown_status` listant les statuts reconnus.
      Vérifié par `decisions::status_inconnu_est_signale`.

## 2. Index et supersession dans `codev-engine::decisions`

- [x] 2.1 Créer `codev-engine::decisions` (`decisions/mod.rs`). Types
      publics : `IndexEntry { qualified_id, decision, origin, path }`,
      `Origin { Project, Path(String) }`, `DecisionIndex { entries,
      in_effect, findings }`, `pub fn index(fs, layout, config) ->
      DecisionIndex`. Vérifié par
      `engine_decisions::index_vide_sur_projet_sans_adr`.
- [x] 2.2 `index` parcourt `_codev/decisions/` du projet et, pour chaque
      source `inherits: path:`, `<chemin>/_codev/decisions/`. Origine
      respective marquée. Vérifié par
      `engine_decisions::adr_du_projet_et_dune_source_apparaissent_avec_leur_origin`.
- [x] 2.3 Calcul de `in_effect` : une entrée `accepted` non supersedée par
      une autre entrée `accepted` est en vigueur ; les autres statuts ne
      le sont jamais. Vérifié par
      `engine_decisions::supersession_directe_masque_la_source` (scénario
      « Supersession directe ») et
      `engine_decisions::chaine_a_trois_maillons_laisse_le_dernier` (scénario
      homonyme).
- [x] 2.4 `supersedes` pointant vers un id absent → finding
      `decision_supersedes_unknown`, la décision reste en vigueur. Vérifié
      par `engine_decisions::supersedes_vers_absent_est_signale` (scénario
      « Cible de supersession absente »).
- [x] 2.5 Collision d'id entre projet et source → finding
      `decision_id_collision`, version du projet retenue. Vérifié par
      `engine_decisions::collision_projet_source_projet_gagne` (scénario
      homonyme).
- [x] 2.6 Cycle de supersession → finding `decision_supersession_cycle`,
      aucune des décisions du cycle n'entre dans `in_effect`. Vérifié par
      `engine_decisions::cycle_de_supersession_est_signale`.

## 3. Injection dans les instructions

- [x] 3.1 Étendre `Instructions` (`codev-engine::instructions`) d'un champ
      `pub decisions: Vec<DecisionRef>` avec `DecisionRef { id,
      qualified_id, title, status, tags, path: PathBuf, origin }`. Vérifié
      par `instructions::instructions_portent_un_champ_decisions_meme_vide`.
- [x] 3.2 `for_artifact` : pour l'artefact `design`, remplit `decisions`
      avec les entrées `in_effect` de l'index. Pour tout autre artefact,
      laisse vide. Vérifié par
      `instructions::design_recoit_les_decisions_en_vigueur` et
      `instructions::proposal_ne_recoit_pas_les_decisions` (à ce stade —
      un futur change pourra l'étendre).

## 4. Contrat JSON v1

- [x] 4.1 Nouveau `DecisionRefV1` dans `contract::v1` en camelCase :
      `id`, `qualifiedId`, `title`, `status`, `tags`, `path`, `origin`.
      Ajouté à `InstructionsV1` comme champ `decisions: Vec<DecisionRefV1>`.
      Vérifié par `contract::instructions_v1_expose_decisions_en_camel_case`.
- [x] 4.2 Test de rétrocompatibilité :
      `contract::instructions_v1_sans_decisions_reste_valide` — les autres
      artefacts ont bien `decisions: []` mais le champ existe.

## 5. Rendu humain

- [x] 5.1 `render::instructions` ajoute une section « Décisions en vigueur »
      quand `decisions` n'est pas vide, une ligne par entrée au format
      `- <id> <title>`. Vérifié par
      `render::instructions_design_liste_les_decisions_en_vigueur`.
- [x] 5.2 Aucune section quand `decisions` est vide. Vérifié par
      `render::instructions_sans_decisions_nest_pas_seche_de_titre_vide`.

## 6. Dogfooding et intégration workspace

- [x] 6.1 Créer un mini change de test `poc-design-with-decisions` (une
      capacité fictive), lancer `codev instructions design --change …
      --json`, vérifier que le tableau `decisions` contient les 6 ADR déjà
      présents dans le dépôt (`0001` à `0006`). Puis supprimer ce change
      de test — il ne reste pas dans le dépôt.
- [x] 6.2 `cargo test --workspace` reste vert et compte au moins 15 tests
      supplémentaires (parseur + index + injection + contrat + rendu).
- [x] 6.3 `cargo clippy --workspace --all-targets` reste sans avertissement.
- [x] 6.4 `codev validate --all` reste vert.
