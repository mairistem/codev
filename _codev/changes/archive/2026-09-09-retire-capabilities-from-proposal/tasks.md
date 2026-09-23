# Tâches

## 1. Cœur — `Plan.deletions`

- [x] 1.1 Ajouter `deletions: Vec<PathBuf>` à `codev_core::plan::Plan`,
      initialisé vide via `#[derive(Default)]` existant.
- [x] 1.2 Ajouter `Plan::delete(&mut self, path: impl Into<PathBuf>)` —
      dédoublonnage sur chemin exact, comme `dir()`.
- [x] 1.3 Étendre `Plan::merge` pour absorber `other.deletions`.
- [x] 1.4 `Plan::is_empty` inclut maintenant `deletions.is_empty()`.
- [x] 1.5 Tests : `delete` déduplique ; `merge` absorbe ; `is_empty` couvre.

## 2. Cœur — `merge_into_existing` étendu

- [x] 2.1 Nouvelle signature :
      ```rust
      pub fn merge_into_existing(
          spec_source: &str,
          spec: &Spec,
          delta: &Delta,
          retire_capabilities: bool,
      ) -> Result<MergePlan, MergeError>
      ```
- [x] 2.2 Ajouter `should_delete_spec: bool` à `MergePlan` — initialement
      `false`. Vaut `true` quand `retire_capabilities == true` **et**
      qu'après application des REMOVED, la spec serait vide (`removed_count
      >= spec.requirements.len()`).
- [x] 2.3 Le refus historique `WouldLeaveSpecWithoutRequirement` reste
      actif quand `retire_capabilities == false`. Message d'erreur mis à
      jour : « ... utilise `retire_capabilities: true` dans `change.yaml`
      pour retirer la capacité » — retire le « à venir ».
- [x] 2.4 Tests : vide total sans flag → erreur ; vide total avec flag
      → plan avec `should_delete_spec: true`, edits appliqués ; vide
      partiel avec flag → plan normal, `should_delete_spec: false`.
- [x] 2.5 Adapter les appels existants dans `codev-engine::sync.rs` pour
      passer le nouveau paramètre (voir tâche 4.1).

## 3. Coquille — `FileSystem::remove_file`

- [x] 3.1 Nouvelle méthode sur le trait `FileSystem` :
      ```rust
      fn remove_file(&self, path: &Path) -> io::Result<()>;
      ```
      Documentée : « supprime un seul fichier ; un fichier absent renvoie
      `NotFound` — l'exécuteur décide s'il ignore ou remonte ».
- [x] 3.2 `RealFileSystem::remove_file` → `std::fs::remove_file`.
- [x] 3.3 `MemoryFileSystem::remove_file` → retire l'entrée de sa
      structure interne, retourne `NotFound` si absent.
- [x] 3.4 Tests : write puis remove, remove sur absent, remove puis
      exists (résultat `false`).

## 4. Coquille — `apply::execute` gère les deletions

- [x] 4.1 Dans `apply::execute`, après la boucle des writes, boucler
      les deletions et appeler `fs.remove_file`. Ordre : `dirs` → `writes`
      → `deletions` → `moves`.
- [x] 4.2 `AppliedOutcome` gagne un champ `deleted: Vec<PathBuf>` porté
      par l'exécution — chaque suppression réussie est enregistrée.
- [x] 4.3 Un fichier absent lors d'une deletion n'est **pas** une erreur
      (le change peut avoir été appliqué déjà). Silencieux, ne marque
      pas la deletion comme faite.
- [x] 4.4 Tests : deletion d'un fichier existant → supprimé et listé ;
      deletion d'un absent → pas d'erreur, pas de listing.

## 5. Coquille — `sync` propage la deletion

- [x] 5.1 `sync::plan_sync` lit `metadata.retire_capabilities` depuis
      le change context et le passe à `merge_into_existing`.
- [x] 5.2 Si `merge_plan.should_delete_spec`, ajouter `main_spec_path`
      à `plan.deletions` **et** à un nouveau champ `deleted:
      Vec<PathBuf>` du `SyncPlan`. Ne pas écrire dans `updates` — la
      capacité est retirée, pas mise à jour.
- [x] 5.3 `SyncOutcome` gagne `deleted: Vec<PathBuf>`. Idem
      `ArchiveOutcome` (qui l'obtient via sync).
- [x] 5.4 Tests intégration : sync sur un change qui retire une capa +
      flag → fichier absent du disque, `outcome.deleted` non vide,
      `outcome.updated/unchanged` ne le contiennent pas.

## 6. Contrat JSON

- [x] 6.1 `SyncReportV1` gagne `deleted: Vec<String>` (camelCase),
      toujours présent, vide dans le cas courant.
- [x] 6.2 `ArchiveReportV1` gagne `deleted: Vec<String>` (même
      règle).
- [x] 6.3 Le shape d'échec de sync et d'archive dans `main.rs` gagne
      `"deleted": []` — cohérence avec les autres champs du shape.
- [x] 6.4 Test : `codev sync <c> --json` sur un change qui retire une
      capa expose bien `deleted[0]`.

## 7. Template proposal

- [x] 7.1 `assets/schemas/spec-driven/templates/proposal.md` gagne une
      nouvelle sous-section `### Capacités retirées` sous `## Capacités`.
      Commentaire : « Une ligne par capacité retirée, chemin exact.
      Requiert `retire_capabilities: true` dans `change.yaml`. »
- [x] 7.2 `codev instructions proposal` renvoie le template mis à jour
      (via `include_str!`).

## 8. Doc + dogfooding

- [x] 8.1 `ChangeMetadata::retire_capabilities` — actualiser le
      commentaire pour retirer « à venir » et ajouter un exemple :
      « Voir F5 pour le comportement. »
- [x] 8.2 `cargo test --workspace` reste vert, +15 tests minimum.
- [x] 8.3 `cargo clippy --workspace --all-targets` sans avertissement.
- [x] 8.4 `codev validate --strict` sur ce dépôt reste vert.
- [x] 8.5 Test à la main : créer un change de test qui retire une capa
      factice, appliquer via `sync`, vérifier que le fichier est
      supprimé, restaurer.
