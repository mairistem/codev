# Tâches

## 1. Fondations dans `codev-core`

- [x] 1.1 Créer le module `parser` dans `codev-core` (`parser/mod.rs`,
      `parser/ast.rs`, `parser/fence.rs`, `parser/spec.rs`, `parser/delta.rs`),
      exposer les items publics depuis `lib.rs`, vérifié par `cargo build -p
      codev-core` puis `cargo doc -p codev-core --no-deps`.
- [x] 1.2 Définir les types d'AST — `Spec`, `Delta`, `Requirement`,
      `Scenario`, l'énumération `DeltaOp`, les structures `Finding` et
      `Parsed<T>` — sans logique, avec `#[derive(Debug, Clone, PartialEq)]`,
      vérifié par un test compilant qui construit chaque variante à la main.
- [x] 1.3 Définir `Span { byte_range: Range<usize>, line_range: Range<u32> }`
      et l'attacher à chaque nœud d'AST comme champ, vérifié par un test qui
      construit un `Span` et lit ses deux plages.

## 2. Masquage des zones littérales

- [x] 2.1 Écrire `build_fence_mask(source: &str) -> Vec<bool>` dans
      `parser/fence.rs` — un booléen par ligne, vrai si la ligne fait partie
      d'un bloc de code (fence d'ouverture, fermeture, contenu). Vérifié par
      les tests `fence::mask_reconnait_backticks_et_tildes`,
      `fence::mask_refuse_une_fermeture_de_marqueur_different` et
      `fence::mask_traite_deux_blocs_successifs`.
- [x] 2.2 Étendre le masque aux commentaires HTML `<!-- … -->` multi-lignes
      hors fences, vérifié par le test
      `fence::commentaire_multi_lignes_est_masque` (le commentaire enferme un
      faux `### Requirement:` qui ne doit pas apparaître dans l'AST plus tard).

## 3. Parseur de spec principale

- [x] 3.1 Implémenter `parse_spec(source: &str) -> Parsed<Spec>` — extraction
      de `## Purpose` et `## Requirements`, itération des `### Requirement:`
      dans `## Requirements` uniquement, itération des `#### Scenario:` dans
      chaque exigence. Vérifié par
      `spec::extrait_purpose_et_une_exigence_avec_scenario` (le premier
      scénario du fichier `Purpose et exigences bien formées` de la spec).
- [x] 3.2 Purpose manquant remonté en `Finding` de sévérité `Error` sans
      empêcher l'extraction des exigences, vérifié par
      `spec::purpose_manquant_est_un_finding_localise` (scénario homonyme).
- [x] 3.3 Un `### Requirement:` hors de `## Requirements` produit un `Finding`
      qui nomme la ligne et le fait que la section attendue est
      `## Requirements`, vérifié par
      `spec::exigence_hors_section_est_signalee`.
- [x] 3.4 Un en-tête de delta rencontré dans une spec principale produit un
      `Finding` nommant la ligne, vérifié par
      `spec::header_de_delta_dans_main_spec_est_signale` (scénario
      `En-tête de delta dans une spec principale`).

## 4. Parseur de delta

- [x] 4.1 Implémenter `parse_delta(source: &str) -> Parsed<Delta>` avec la
      reconnaissance des quatre sections `## ADDED|MODIFIED|REMOVED|RENAMED
      Requirements`, vérifié par `delta::reconnait_les_quatre_sections`.
- [x] 4.2 Extraire les blocs d'exigence complets sous `ADDED` et `MODIFIED`
      — nom depuis `### Requirement: <nom>`, texte descriptif jusqu'au
      prochain en-tête, scénarios en `#### Scenario:`. Vérifié par
      `delta::bloc_added_porte_exigence_et_scenario` (scénario
      `Bloc ADDED avec exigence et scénario`).
- [x] 4.3 Extraire sous `REMOVED` le nom, la ligne `**Reason**:` et la ligne
      `**Migration**:`, vérifié par
      `delta::bloc_removed_porte_raison_et_migration`.
- [x] 4.4 Extraire sous `RENAMED` les couples `FROM: <ancien>` / `TO:
      <nouveau>`, vérifié par `delta::bloc_renamed_associe_from_et_to`.
- [x] 4.5 Extraire un `## Purpose` optionnel en tête de delta (nouvelle
      capacité), vérifié par `delta::purpose_de_nouvelle_capacite_est_extrait`
      (scénario `Delta d'une capacité nouvelle avec Purpose`).
- [x] 4.6 Deux exigences de même nom dans une même section produisent un
      `Finding` nommant les deux lignes et la section, vérifié par
      `delta::doublon_dans_added_est_signale` (scénario
      `Exigence dupliquée dans une même section`).

## 5. Zones littérales appliquées

- [x] 5.1 Une exigence apparaissant dans un bloc de code d'une spec principale
      n'apparaît pas dans l'AST, vérifié par
      `spec::exigence_dans_fence_est_ignoree` (scénario
      `Exemple d'exigence à l'intérieur d'un bloc de code`).
- [x] 5.2 Un en-tête de delta dans un commentaire HTML n'est pas compté,
      vérifié par `delta::header_dans_commentaire_html_est_ignore` (scénario
      `En-tête de delta à l'intérieur d'un commentaire`).

## 6. Position d'origine et invariant

- [x] 6.1 Chaque nœud extrait — Purpose, Requirement, Scenario, chaque bloc
      d'opération de delta — porte un `Span` dont `line_range.start` égale la
      ligne (1-indexée) de son en-tête dans la source, vérifié par
      `spans::scenario_expose_sa_ligne_de_debut` (scénario
      `Position en ligne d'un scénario`, ancré sur la ligne 42).
- [x] 6.2 Un scénario écrit avec trois dièses produit un `Finding` de code
      `scenario_wrong_heading_level` et l'exigence apparaît sans ce scénario,
      vérifié par `spec::scenario_trois_dieses_est_signale` (scénario
      `Scénario écrit avec trois dièses`).
- [x] 6.3 Test d'invariant `spans::round_trip_preserve_la_source_a_loctet` :
      pour un fichier d'exemple, prendre chaque nœud, extraire
      `source[node.span.byte_range]`, remplacer chaque plage par elle-même
      dans une nouvelle chaîne, vérifier l'égalité octet à octet avec
      l'original.

## 7. Golden tests

- [x] 7.1 Créer `crates/codev-core/tests/parser_golden.rs` avec au moins un
      exemple de spec principale complète et un exemple de delta complet
      (fichiers d'entrée dans `crates/codev-core/tests/fixtures/`), attendus
      sérialisés en `Debug`. Vérifié par
      `cargo test -p codev-core --test parser_golden`.
- [x] 7.2 Ajouter à `parser_golden` le cas d'une réécriture ciblée d'un bloc
      `MODIFIED` : extraire son `byte_range`, y injecter un nouveau texte,
      vérifier que le reste du fichier — y compris les espacements — est
      identique. Vérifié par `parser_golden::reecriture_ciblee_ne_touche_pas`.

## 8. Intégration workspace

- [x] 8.1 `cargo test --workspace` reste vert et compte au moins 15 tests de
      plus qu'avant ce change.
- [x] 8.2 `cargo clippy --workspace --all-targets` reste sans avertissement,
      hors ceux préexistants documentés.
