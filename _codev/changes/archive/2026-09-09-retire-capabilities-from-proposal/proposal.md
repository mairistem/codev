# Proposal : retirer une capacité entière depuis un proposal

## Pourquoi

Aujourd'hui, un proposal peut **ajouter** (`### Nouvelles capacités`) ou
**modifier** (`### Capacités modifiées`) une capacité, mais il n'a
**aucun mécanisme complet pour en retirer une**. Le `## REMOVED
Requirements` d'un delta existe déjà — mais si toutes les exigences
d'une capacité sont retirées, l'outil **refuse le merge** avec
`would_leave_spec_without_requirement` : « le marqueur
`retire_capabilities: true` (à venir) sera nécessaire pour retirer la
capacité ».

Le marqueur est déjà réservé dans `ChangeMetadata` mais n'est lu par
personne. F5 livre la fonctionnalité qui va derrière : quand un change
retire toutes les exigences d'une spec **et** déclare
`retire_capabilities: true`, `sync`/`archive` **supprime** le fichier
`_codev/specs/<capa>/spec.md` au lieu de le laisser vide ou de refuser.

Ferme la boucle CRUD sur les specs — sans ce geste, une capacité obsolète
soit reste à traîner dans les specs principales, soit force l'utilisateur
à supprimer le fichier à la main (ce qui contourne l'atomicité et laisse
l'historique du change incomplet).

## Ce qui change

- **Nouvelle sous-section `### Capacités retirées`** dans le template
  proposal, à côté des sections `Nouvelles` et `Modifiées`. Le contenu
  documente les capacités que le change retire — utile pour la relecture
  humaine, pas une source de vérité pour l'outil.
- **La source de vérité reste `retire_capabilities: true`** dans
  `change.yaml`. Sans ce marqueur, un `## REMOVED Requirements` qui
  viderait une spec principale est **refusé** comme aujourd'hui — geste
  irréversible, donc opt-in explicite.
- **Nouveau champ `deletions: Vec<PathBuf>` sur `Plan`** (cœur), pour
  qu'une suppression de fichier soit une opération de premier ordre du
  plan atomique — même granularité que `writes` et `moves`.
- **Nouvelle méthode `FileSystem::remove_file(&Path)`** sur le port, avec
  implémentation réelle (`std::fs::remove_file`) et en mémoire.
- **`merge_into_existing` étendu** — retourne un `MergePlan` qui expose
  désormais un `should_delete_spec: bool`. Vrai quand toutes les
  exigences ont été retirées **et** que `retire_capabilities: true` est
  passé en argument. L'ancienne erreur
  `WouldLeaveSpecWithoutRequirement` est conservée pour le cas sans le
  flag.
- **`sync` propage la suppression** — `SyncPlan` gagne `deleted:
  Vec<PathBuf>` ; `SyncOutcome` aussi ; le contrat JSON aussi.
- **`archive` hérite du même comportement** (il passe par `sync`).

## Capacités

### Nouvelles capacités

Aucune.

### Capacités modifiées

- `spec-merge` — trois nouvelles exigences ADDED : suppression atomique
  d'une spec vidée, propagation dans `sync`/`archive`, mécanisme
  `deletions` du plan.

### Capacités retirées

Aucune. (Documentation-only — utile pour montrer la nouvelle section
dans le template dès sa livraison.)

## Impact

- **Code** :
  - `codev-core::plan::Plan` gagne `deletions: Vec<PathBuf>` et
    `Plan::delete()`.
  - `codev-core::merge::MergePlan` gagne `should_delete_spec: bool` et
    `merge_into_existing` prend un `retire_capabilities: bool` en argument.
  - `codev-engine::ports::FileSystem` gagne `remove_file(&Path)` (avec
    `RealFileSystem` et `MemoryFileSystem`).
  - `codev-engine::apply::execute` applique les deletions **après** les
    writes et **avant** les moves.
  - `codev-engine::sync::SyncPlan/SyncOutcome` gagnent `deleted:
    Vec<PathBuf>`.
- **Contrat JSON** — `SyncReportV1` et `ArchiveReportV1` gagnent
  `deleted: Vec<String>` (additif, toujours présent, vide dans le cas
  courant). Aucun champ retiré ni renommé.
- **Template proposal** — `assets/schemas/spec-driven/templates/proposal.md`
  gagne la section `### Capacités retirées`.
- **Documentation** — la ligne `retire_capabilities: true` de
  `ChangeMetadata` gagne un exemple d'usage dans son commentaire.
- **Migration** — aucune. Sans le flag, comportement inchangé (le
  refus historique reste).
- **Hors périmètre** :
  - **Un flag `codev new change --retire-capabilities`** — l'utilisateur
    édite `change.yaml` à la main dans ce cas rare ; pas d'ergonomie
    supplémentaire tant qu'un besoin ne se manifeste pas.
  - **Rétablir une capacité retirée par un change précédent** — pas
    d'undo. Un ADR ou un ADDED du proposal suivant fait l'affaire.
  - **Supprimer aussi les décisions liées à la capacité retirée** —
    hors périmètre ; les décisions restent (elles sont l'histoire).
