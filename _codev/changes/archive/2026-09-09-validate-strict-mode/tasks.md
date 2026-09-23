# Tâches

## 1. Cœur — `has_warnings()` sur `ValidateReport`

- [x] 1.1 Ajouter la méthode `has_warnings(&self) -> bool` sur
      `ValidateReport` dans `codev-engine::validate::report`.
      Implémentation : itère `items[].findings[]`, retourne `true` dès
      qu'un `Finding.severity == Warning`.
- [x] 1.2 Ajouter la même méthode sur `ItemReport` pour symétrie avec
      `has_errors()` existant.
- [x] 1.3 Tests : un rapport avec un seul warning → `true` ;
      un rapport avec seulement des erreurs → `false` (les erreurs ne
      sont pas des warnings) ; un rapport vide → `false`.

## 2. CLI — flag `--strict`

- [x] 2.1 Ajouter `#[arg(long)] strict: bool` sur `Command::Validate`
      dans `codev-cli::main`. Documentation clap : « Traite tout finding
      (Warning inclus) comme un motif d'exit code non-nul. Utile pour
      la CI et l'automation. »
- [x] 2.2 Modifier la logique d'exit code dans le match `Command::Validate`
      pour prendre en compte `strict` :
      ```rust
      let has_fail = if strict {
          report.has_errors() || report.has_warnings()
      } else {
          report.has_errors()
      };
      if has_fail { 1 } else { 0 }
      ```
- [x] 2.3 Sur `--json`, la sévérité affichée dans chaque finding reste
      celle du finding — aucune promotion silencieuse.

## 3. Contrat JSON — `hasWarnings`

- [x] 3.1 Ajouter `has_warnings: bool` sur `ValidateReportV1` (sérialisé
      en `hasWarnings`), toujours présent.
- [x] 3.2 `From<&ValidateReport> for ValidateReportV1` calcule
      `has_warnings` via la méthode ajoutée en 1.1.
- [x] 3.3 Le `validate_shape()` d'échec dans `main.rs` gagne
      `"hasWarnings": false` — cohérence avec les autres champs du
      shape.

## 4. Tests d'intégration CLI

- [x] 4.1 Test dans `codev-cli::commands::tests` : projet propre + `--strict`
      → pas d'erreur, exit 0 (via `validate(&h.ctx(), ValidateArgs::All)`).
- [x] 4.2 Test : projet avec un ADR local `accepted` non scellé + `--strict`
      → `report.has_warnings()` est vrai, `report.has_errors()` est faux.
      La logique CLI d'exit-code sortirait 1 — vérifiable par
      `report.has_warnings() && strict`.
- [x] 4.3 Test contrat JSON : `ValidateReportV1::from(&rapport)` porte
      `has_warnings: true` quand il y a un warning, `false` sinon.

## 5. Dogfooding et intégration workspace

- [x] 5.1 Après `cargo install`, lancer `codev validate --strict` sur
      ce dépôt — doit renvoyer exit 0 (aucun warning aujourd'hui).
- [x] 5.2 Test à la main : introduire volontairement un warning (par
      exemple, retirer une entrée du `seal.yaml`), lancer
      `codev validate --strict` → exit 1 ; sans `--strict` → exit 0.
      Restaurer avant commit.
- [x] 5.3 `cargo test --workspace` reste vert, gagne au moins 5 tests
      nouveaux (report + CLI + JSON).
- [x] 5.4 `cargo clippy --workspace --all-targets` reste sans
      avertissement.
- [x] 5.5 `codev validate --strict --all` reste vert sur ce dépôt.
