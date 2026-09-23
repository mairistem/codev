# Tâches

## 1. Cœur — parseur de blocs `### Décision : ...`

- [x] 1.1 Nouveau module `codev-engine::design` (fonction pure — même
      motif que les autres parseurs). Signature :
      ```rust
      pub struct DecisionBlock {
          pub title: String,
          pub body: String,
          pub byte_range: Range<usize>,
          pub line: u32,
      }
      pub fn extract_decision_blocks(source: &str) -> Vec<DecisionBlock>;
      ```
      Repère `## Décisions`, puis boucle sur les lignes `### Décision :
      <titre>` (H3 exact, deux-points, espace), jusqu'au prochain `### `
      ou `## `. `title` = ce qui suit `Décision : ` **trimmé**. `body`
      = octets entre la fin de la ligne de titre et le début du bloc
      suivant, **verbatim**.
- [x] 1.2 Tests : bloc unique, deux blocs, aucun bloc, bloc avec
      formatting markdown (tableau, code fence) — le corps est
      byte-pour-byte.

## 2. Cœur — recherche par titre + refus d'ambiguïté

- [x] 2.1 Fonction `find_decision_block(blocks: &[DecisionBlock], title:
      &str) -> Result<usize, PromoteLookupError>`. Erreurs typées :
      `NotFound(String)`, `Ambiguous(String, Vec<u32>)` avec les
      numéros de ligne.
- [x] 2.2 Tests : trouvé, non trouvé, ambigu (deux positions).

## 3. Coquille — `plan_promote` dans `decisions_actions`

- [x] 3.1 Nouvelle struct `PromotePlan` : `plan: Plan`, `new_id`,
      `new_path`, `body_sha256`, `source_change: String`.
- [x] 3.2 Nouvelle variante d'`ActionError` : `CannotPromoteFromArchived
      { change: String }`, `DesignMissing { change: String }`,
      `DecisionHeadingNotFound { title: String }`,
      `AmbiguousDecisionHeading { title: String, lines: Vec<u32> }`.
      Codes stables correspondants.
- [x] 3.3 Fonction `plan_promote(index, existing_seal, change_id,
      heading, design_source, today, layout, change_dir)`. Elle :
      - refuse si le titre est vide (`EmptyTitle`) ;
      - parse `design_source` via `design::extract_decision_blocks` ;
      - cherche le bloc par titre ; retourne l'erreur adéquate ;
      - construit le corps de l'ADR : sections `## Contexte` (vide,
        commentaire placeholder), `## Décision` (corps verbatim),
        `## Conséquences` (placeholder), `## Alternatives écartées`
        (placeholder) ;
      - calcule `next_local_id` et le slug ;
      - calcule `body_sha256` et ajoute l'entrée de sceau ;
      - construit le `new_design_source` avec substitution du bloc
        (préserve espacement) ;
      - retourne un plan à 3 writes : ADR (CreateOnly), seal.yaml
        (Overwrite), design.md (Overwrite).
- [x] 3.4 Tests : promotion basique, corps verbatim, refus d'un titre
      absent, refus d'un titre ambigu, refus d'un titre vide.

## 4. Coquille — `execute_promote` côté engine

- [x] 4.1 Fonction `execute_promote(fs, env, layout, config, clock,
      change_id, heading)` qui compose : `change::load` (récupère
      `ChangeContext`, vérifie que le change existe), lecture du
      design, appel `plan_promote`, exécution.
- [x] 4.2 Refus explicite `cannot_promote_from_archived` : si le
      `change_dir` calculé est sous `changes/archive/`. Détecté via un
      simple check de chemin (le layout ne connaît que les changes
      actifs sous `changes/<nom>/`, un change archivé n'est pas
      atteignable par `change::load` — l'erreur `UnknownChange` est
      remontée aujourd'hui). **Nouveau** : `codev list` seul montre les
      actifs ; on ajoute une passe manuelle qui vérifie l'existence
      d'un dossier sous `_codev/changes/archive/*-<nom>/` pour
      distinguer les deux cas et retourner le bon code stable.
- [x] 4.3 `design_missing` : si `<change_dir>/design.md` n'existe pas
      → code dédié.
- [x] 4.4 Tests intégration : promotion end-to-end sur un projet en
      mémoire, vérifie l'ADR créé, la seal.yaml, la substitution dans le
      design.

## 5. CLI — sous-commande `codev decision promote`

- [x] 5.1 `DecisionCommand::Promote { change, title, json }`.
      Documentation clap : « Promeut un bloc `### Décision : <titre>`
      du design.md d'un change actif en ADR de premier ordre, scellé
      par K3, référencé depuis le design. »
- [x] 5.2 Route vers `commands::decision_promote(ctx, change, title)`.
      Renvoie un `DecisionPromotedOutcome` (root, decision summary,
      path, body_sha256, source_change).
- [x] 5.3 Rendu humain :
      ```
      ✓ Décision « <titre> » promue en ADR 0007 : <titre>
        Fichier    : _codev/decisions/0007-<slug>.md
        Hash       : sha256:...
        Source     : _codev/changes/<change>/design.md

      Ventile le corps en Contexte / Décision / Conséquences /
      Alternatives écartées avant d'archiver le change.
      ```
- [x] 5.4 Tests d'intégration CLI : succès, refus d'un archivé, refus
      d'un titre absent, refus d'un titre ambigu.

## 6. Contrat JSON

- [x] 6.1 Nouveau `DecisionPromotedV1` :
      ```rust
      { root, decision: Option<DecisionV1>, path, bodySha256,
        sourceChange, status: [] }
      ```
      Additif complet.
- [x] 6.2 Nouveau `decision_promoted_shape()` dans `main.rs`.
- [x] 6.3 Test contrat : le JSON de succès porte tous les champs, celui
      d'échec porte le shape avec les codes stables.

## 7. Intégration workspace

- [x] 7.1 `cargo test --workspace` reste vert, +12 tests nouveaux
      (parser + lookup + plan + execute + CLI).
- [x] 7.2 `cargo clippy --workspace --all-targets` sans avertissement.
- [x] 7.3 `codev validate --strict --all` reste vert.
- [x] 7.4 Test à la main : sur le change de test, créer un design.md
      avec un bloc de décision, lancer la promotion, vérifier que
      l'ADR est créé, scellé, référencé, et que `codev decision list`
      le liste bien.
