# Tâches

## 1. Cœur — parser ADR étendu

- [x] 1.1 Ajouter le champ `deviates_from: Vec<String>` à
      `codev_core::decisions::Decision` (à côté de `supersedes:
      Vec<String>`, mêmes conventions).
- [x] 1.2 Étendre `parse_decision` pour lire `deviates_from` du
      frontmatter YAML : liste optionnelle, chaque entrée doit être une
      chaîne. Champ absent → liste vide.
- [x] 1.3 Nouveau finding stable `decision_field_type_mismatch` (warning
      côté parseur, code exposé dans `codev_core::parser::codes` si le
      module est le bon endroit ; sinon dans le module décisions) —
      émis quand `deviates_from` n'est pas une liste ou porte une
      entrée non-string.
- [x] 1.4 Tests : ADR avec `deviates_from` à une entrée, à plusieurs
      entrées, absent, mal formé (chaîne au lieu d'une liste), entrée
      non-string dans la liste.

## 2. Cœur — action `plan_deviate`

- [x] 2.1 Nouvelle fonction pure `plan_deviate(index, existing_seal,
      qualified_target, title, today, layout) -> Result<DeviatePlan,
      ActionError>` dans `codev-engine::decisions_actions`. Produit un
      plan qui écrit l'ADR local (`deviates_from: ["<qualified>"]`,
      `status: accepted`) **et** l'entrée de sceau, dans un même plan
      atomique.
- [x] 2.2 Refus explicites — codes stables :
      - `cannot_deviate_from_local` si `qualified_target` commence par
        `projet/` — message qui renvoie vers `codev decision supersede` ;
      - `unknown_decision_id` si la cible n'est pas indexée ;
      - `ambiguous_decision_id` si deux sources exposent le même
        qualified ;
      - `empty_title` si le titre est vide.
- [x] 2.3 Extension du `render_frontmatter` (ou fonction dédiée) pour
      émettre le champ `deviates_from: ["…"]` quand la liste est non
      vide — même style que `supersedes`.
- [x] 2.4 Tests : plan produit deux écritures (ADR + seal) ; ADR contient
      bien la ligne `deviates_from: ["path:~/partage/0100"]` ; les
      refus remontent le bon code.

## 3. Cœur — index et occultation

- [x] 3.1 Étendre `IndexEntry` avec un champ dérivé `deviated_by:
      Option<QualifiedId>`. Calculé, pas persisté.
- [x] 3.2 Nouvelle passe `resolve_deviations(&mut entries, &mut
      findings)` dans `codev-engine::decisions`, appelée après la
      résolution des supersessions. Deux effets :
      - Pour chaque ADR local `accepted` avec `deviates_from`, marquer
        chaque cible héritée d'un `deviated_by = <qualified-local>` ;
      - Retirer les cibles ainsi marquées du calcul `in_effect`.
- [x] 3.3 `decision_dangling_deviation` (warning) émis pendant cette
      passe pour chaque cible non trouvée dans l'index.
- [x] 3.4 `decision_conflicting_deviations` (erreur) émis pendant cette
      passe pour toute cible référencée par au moins deux ADR locaux
      `accepted`. La cible reste marquée `deviated_by` du premier
      trouvé (ordre stable par `qualified_id`), mais le finding rend le
      conflit visible.
- [x] 3.5 Tests : dérive de path:, dérive de git:, deux dérives sur
      même cible (conflit), dérive orpheline (dangling), ADR proposed
      avec deviates_from (aucun effet).

## 4. Cœur — injection dans instructions `design`

- [x] 4.1 Le rendu du tableau `decisions[]` dans les instructions doit
      filtrer les entrées dont `deviated_by.is_some()` en plus du
      filtre `in_effect` déjà en place. Test : instructions design d'un
      change où une héritée a été déviée ne mentionne que la locale.
- [x] 4.2 Idem pour la section humaine « Décisions en vigueur » du
      rendu texte.
- [x] 4.3 Test golden : la ligne `deviates_from: [...]` dans le
      frontmatter d'un ADR local **rend invisible** l'héritée référencée,
      pour tous les changes actifs — pas seulement le change courant.

## 5. Coquille — CLI `decision deviate`

- [x] 5.1 Nouvelle sous-commande `codev decision deviate
      <qualified-id> <titre> [--json]`. Route vers l'action de
      l'engine.
- [x] 5.2 Rendu humain de succès : `✓ Dérive locale de « <qualified> »
      créée : <local-id> <titre>` + `  Fichier : <path>`.
- [x] 5.3 Contrat JSON : nouvelle struct `DecisionDeviatedV1` (proche
      de `DecisionCreatedV1`) portant la décision créée et
      `bodySha256`. Nouveau shape `decision_deviated_shape()` dans
      `main.rs`.
- [x] 5.4 Tests d'intégration CLI : dérive OK, dérive d'une locale
      (refus `cannot_deviate_from_local`), dérive d'un id inconnu
      (refus `unknown_decision_id`).

## 6. Contrat JSON — additions

- [x] 6.1 `DecisionV1` gagne `deviatesFrom: Vec<String>` (additif — les
      ADR sans le champ sortent `[]`).
- [x] 6.2 Les entrées de `codev decision list --json` (issues de
      `build_summaries`) gagnent `deviatedBy: Option<String>`,
      sérialisé seulement quand non `None`
      (`skip_serializing_if = "Option::is_none"`).
- [x] 6.3 Test contrat : liste JSON d'un projet avec une dérive expose
      bien `deviatedBy` sur la cible et `deviatesFrom` sur la source.

## 7. Validate — findings intégrés

- [x] 7.1 Le rapport `validate_decisions` remonte les findings
      `decision_dangling_deviation` et `decision_conflicting_deviations`
      calculés par `resolve_deviations` en même temps que les autres
      findings de scellement.
- [x] 7.2 Tests dans `validate::tests` : conflict → `has_errors()` est
      `true` ; dangling seul → warning uniquement, `has_errors()` est
      `false`.

## 8. Dogfooding

- [x] 8.1 `cargo test --workspace` reste vert, gagne au moins 15 tests
      nouveaux (parseur + plan_deviate + index + CLI).
- [x] 8.2 `cargo clippy --workspace --all-targets` reste sans
      avertissement.
- [x] 8.3 `codev validate --all` reste vert sur ce dépôt (pas de
      dérive locale, aucune source héritée déclarée).
