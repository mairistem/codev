# Tâches

## 1. Cœur — module `seal` et hash

- [x] 1.1 Créer `crates/codev-core/src/decisions/seal.rs` — types :
      `Seal { id: DecisionId, body_sha256: String, sealed_at: NaiveDate }`,
      `SealFile { version: u32, seals: Vec<Seal> }`. Frontmatter attribué
      `#[serde(deny_unknown_fields)]` sur les deux. `version` figé à `1`
      pour cette livraison.
- [x] 1.2 Fonction `body_hash(source: &str) -> Result<String,
      SealError>` — trouve le `\n---\n` (ou `\n---\r\n`) fermant le
      frontmatter, calcule SHA-256 sur tout ce qui suit **byte pour
      byte**, retourne la chaîne `"sha256:<hex>"`. Erreur typée si le
      séparateur est introuvable. Tests : ADR standard, ADR sans
      contenu après le frontmatter (corps vide), ADR avec CRLF, ADR sans
      frontmatter (erreur).
- [x] 1.3 Fonction `parse_seal_file(source: &str) -> Result<SealFile,
      SealError>` via `serde_norway` ; retourne `SealFile { version: 1,
      seals: vec![] }` sur source vide. Tests : forme canonique, champ
      inconnu rejeté, version différente de 1 rejetée avec code
      stable `seal_version_unsupported`.
- [x] 1.4 Fonction `render_seal_file(seal: &SealFile) -> String` —
      YAML canonique avec `version:` en tête, `seals:` en liste, ordre
      des entrées par `id` croissant (déterminisme pour git). Test
      round-trip parse→render→parse.
- [x] 1.5 Fonction pure `plan_seal_new(existing: &SealFile, id:
      DecisionId, body_hash: String, today: NaiveDate) -> SealFile` —
      insère la nouvelle entrée en préservant l'ordre par id. Test :
      insertion au milieu, refuse un id déjà scellé (retour
      `Err(SealError::AlreadySealed)`).
- [x] 1.6 Fonction pure `plan_seal_force(existing: &SealFile, id:
      DecisionId, body_hash: String, today: NaiveDate) -> SealFile` —
      remplace l'entrée existante, met à jour `sealed_at`. Test :
      remplace ; refuse un id absent (retour `Err(SealError::Unknown)`).
- [x] 1.7 Fonction pure `verify(seal: &SealFile, present_ids:
      &[DecisionId], body_hashes: &HashMap<DecisionId, String>) ->
      Vec<Finding>` — émet `decision_unsealed`, `decision_seal_mismatch`,
      `decision_orphan_seal` selon les cas. Tests : chaque cas isolé, cas
      combinés, chaîne de supersession (les deux ADR de la chaîne
      doivent être scellés).

## 2. Cœur — intégration dans plans existants

- [x] 2.1 Étendre `plan_new_decision` : signature reçoit maintenant
      `existing_seal: SealFile` et `today: NaiveDate` (déjà en argument).
      Retourne un `Plan` qui écrit l'ADR **et** `seal.yaml` mis à jour.
      Tests existants adaptés — le plan a désormais 2 writes au lieu
      d'1.
- [x] 2.2 Étendre `plan_supersede` : ajoute un write pour `seal.yaml`
      portant la nouvelle entrée du nouvel ADR ; l'entrée de l'ancien
      reste identique. Test dédié : après `plan_supersede`, `seal.yaml`
      contient N+1 entrées et l'ancienne est bit-identique.
- [x] 2.3 Nouvelle fonction `plan_seal(existing_adr: &Decision,
      existing_seal: &SealFile, force: bool, today: NaiveDate) ->
      Result<Plan, SealError>` — modèle des deux modes. Tests : ajout
      neuf, refus sans force sur changement, réécriture avec force,
      no-op si sceau déjà correct.

## 3. Coquille — engine et coordonnées

- [x] 3.1 `crates/codev-engine/src/decisions_actions.rs` : la fonction
      qui construit le plan `new` doit d'abord lire `seal.yaml` via le
      port `FileSystem`, appeler `plan_new_decision` en lui passant le
      contenu parsé, puis exécuter le plan. Idem pour `supersede`.
- [x] 3.2 Nouvelle action `seal(id: DecisionId, force: bool)` dans
      l'engine, qui compose lecture ADR + lecture seal + calcul hash +
      appel de `plan_seal`, puis exécution. Erreurs remappées vers les
      codes stables `seal_conflict`, `cannot_seal_inherited`,
      `unknown_decision_id`.
- [x] 3.3 Extension de `crates/codev-engine/src/validate.rs` : après le
      chargement de l'index de décisions, lire `seal.yaml` via le port
      `FileSystem`, appeler `verify`, ajouter ses findings au rapport de
      validation. Tests : projet où seal.yaml est absent → tous les ADR
      remontent `decision_unsealed` ; projet où seal.yaml a été rempli à
      la main mais un ADR a été modifié → `decision_seal_mismatch`.
- [x] 3.4 `_codev/decisions/seal.yaml` doit être **créé au layout**
      côté engine — le lister comme fichier de vérité connu, éviter
      qu'il soit interprété comme un ADR par erreur (extension `.yaml`
      donc de toute façon ignoré par le lecteur d'ADR qui filtre `.md`,
      mais autant l'affirmer par test).

## 4. CLI — commande `decision seal`

- [x] 4.1 Nouvelle sous-commande `codev decision seal <id> [--force]
      [--json]`. Route vers l'action de l'engine. `--json` produit
      `{ "decision": { "id": …, "bodySha256": …, "sealedAt": … },
      "status": [] }` en succès, `{ "decision": null, "status": [{code,
      message}] }` en échec.
- [x] 4.2 Rendu humain : succès neuf → `Scellé : 0001 (sha256:abcd…)` ;
      re-sceau → `Re-scellé : 0001 (sha256:…)` ; no-op → `Déjà à jour :
      0001` ; conflit sans force → message avec le code `seal_conflict`
      qui rappelle l'usage de `--force`.
- [x] 4.3 `codev decision new --json` gagne un champ `bodySha256` dans
      son entrée `decision`. Le champ apparaît aussi dans la sortie
      humaine sur sa propre ligne (`Hash : sha256:…`) — utile pour le
      copier-coller si migration.
- [x] 4.4 Tests d'intégration CLI : `decision seal` en trois modes
      (neuf, no-op, conflit avec/sans force) sur un dépôt de test.

## 5. Contrat JSON

- [x] 5.1 Ajouter la struct `SealEntryV1 { id, bodySha256, sealedAt }`
      dans `codev-cli::contract::v1`. La struct `DecisionEntryV1` (utilisée
      par `decision new`) gagne un `bodySha256: Option<String>` optionnel
      — additif, jamais breaking.
- [x] 5.2 Documenter les nouveaux codes de finding dans le module
      `codev-cli::contract::v1::status` : `decision_unsealed`,
      `decision_seal_mismatch`, `decision_orphan_seal`, `seal_conflict`,
      `cannot_seal_inherited`, `seal_version_unsupported`,
      `unknown_decision_id` (existe déjà).

## 6. Migration du dépôt lui-même

- [x] 6.1 Après implémentation et `cargo install`, lancer `codev
      validate` — vérifier que les 6 warnings `decision_unsealed`
      remontent.
- [x] 6.2 Boucler `codev decision seal <id>` pour les 6 ADR ; commit
      unique portant `_codev/decisions/seal.yaml`.
- [x] 6.3 Vérifier qu'un `codev validate` post-migration est
      complètement vert côté décisions.
- [x] 6.4 Éditer volontairement le corps d'un ADR (par exemple ajouter
      « TEST-A-EFFACER » dans le contexte), relancer `codev validate` —
      vérifier que `decision_seal_mismatch` est bien émis avec un exit
      code non nul. Retirer l'édition de test avant le commit.

## 7. Intégration workspace

- [x] 7.1 `cargo test --workspace` reste vert, gagne au moins 15 tests
      nouveaux (module seal + intégrations).
- [x] 7.2 `cargo clippy --workspace --all-targets` reste sans
      avertissement.
- [x] 7.3 `codev validate --all` reste vert (une fois la migration du
      point 6 faite).
