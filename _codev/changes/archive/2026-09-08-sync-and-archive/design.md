# Design : sync et archive

## Contexte

Voir `proposal.md` pour la motivation. Le parseur donne les spans à l'octet
près (B6), le validateur donne l'ensemble des règles E1–E3 comme pré-flight.
Ce design assemble ces deux briques pour produire, sans écriture, un plan de
fusion vérifié dans son intégralité.

## Objectifs / Hors objectifs

Ce design cadre :

- la fusion sémantique en tant que **fonction pure** produisant une liste
  d'edits ponctuels sur la spec principale ;
- l'extension du type [`Plan`](../../../crates/codev-core/src/plan.rs) pour
  porter aussi un déplacement ;
- l'orchestration côté engine (validate → merge → plan) et la coquille CLI
  (sync/archive avec exit code binaire).

Il ne cadre pas la retraite de capacité (F5, lot 2) ni l'archive en lot (F7,
lot 4). Un `REMOVED` qui viderait la spec est refusé ici ; c'est F5 qui
autorisera plus tard le geste.

## Décisions

### Décision : la fusion produit des `Edit { byte_range, replacement }`

Plutôt qu'une reconstruction de l'AST cible puis une re-sérialisation en
markdown, la fusion produit une liste d'édits ponctuels sur la source de la
spec principale :

- `MODIFIED` → `Edit { byte_range: req.span, replacement: <bloc rendu> }`
- `REMOVED`  → `Edit { byte_range: <req.span étendu à l'espacement>, replacement: "" }`
- `RENAMED`  → `Edit { byte_range: <ligne d'en-tête seulement>, replacement: <nouvel en-tête> }`
- `ADDED`    → `Edit { byte_range: <fin_de_requirements..fin_de_requirements>, replacement: <bloc rendu> }`

Les edits sont ensuite triés par `byte_range.end` **décroissant**, puis
appliqués : chaque application préserve les offsets des edits restants.

Rationale : cette approche satisfait mécaniquement la préservation du contenu
non mentionné. Une reconstruction complète perdrait tout ce que l'AST ne
capture pas — commentaires libres entre exigences, sections libres après
`## Requirements`, espacements variables. Le round-trip du parseur est déjà
testé (`spans_reproduisent_le_source_au_caractere_pres`) ; on s'appuie sur cet
invariant plutôt que de le refaire.

**Alternatives considérées** :

- **Reconstruction complète** depuis l'AST cible. Simple à écrire, catastrophique
  à l'usage : tout ce qui n'est pas modélisé se perd.
- **Écritures successives sur le fichier**, une par section du delta. Les
  offsets bougent entre deux écritures, obligeant à re-parser à chaque étape.
  Coûte en performance et introduit des états intermédiaires que l'atomicité
  interdit.

### Décision : `PlanOp::Move { from, to }` étend le type `Plan`

Le déplacement du change vers l'archive rejoint le `Plan` existant, comme
troisième catégorie à côté de `dirs` et `writes` :

```rust
pub struct Plan {
    pub dirs: Vec<PathBuf>,
    pub writes: Vec<FileWrite>,
    pub moves: Vec<Move>,
}

pub struct Move { pub from: PathBuf, pub to: PathBuf }
```

Cela garde la propriété centrale de l'architecture : un seul plan traverse la
coquille, un seul appel à `apply::execute` peut décider `--dry-run` ou
prévisualiser en JSON. L'ordre d'exécution dans `apply::execute` reste :
`dirs` → `writes` → `moves`, parce qu'un déplacement suppose que ce qu'il
déplace a déjà été écrit et que sa destination existe.

Rationale : cité par la décision
[0001](../../decisions/0001-coeur-fonctionnel-coquille-imperative.md) — le
principe « décider n'est pas exécuter » impose que **toute** modification du
disque passe par un plan. Traiter le déplacement à côté serait rouvrir la
porte à un état incohérent (spec principale écrite, dossier non déplacé) que
tout ce projet a été conçu pour éliminer.

### Décision : `archive` refuse d'agir si `validate` a la moindre erreur

`codev archive` appelle en interne `codev_engine::validate::validate_change`
et refuse si `has_errors()`. Aucune fusion, aucune écriture, aucun
déplacement. Le seul dialogue avec l'utilisateur est un renvoi vers
`codev validate <change>` pour voir le détail.

**Rationale** : le validateur (change précédent) livre déjà tous les codes
utiles. Dupliquer les messages dans `archive` créerait deux formulations
d'une même erreur, à maintenir en parallèle. Un renvoi est plus court à
écrire, plus facile à lire, et impossible à contredire.

`codev sync`, en revanche, se contente des invariants **strictement
nécessaires à la fusion** : cible `MODIFIED` présente, cible `REMOVED`
présente, `Purpose` requis si nouvelle capacité, dernière exigence non
retirée. Un delta qui a un `requirement_no_shall` peut être synchronisé — le
défaut sera là aussi dans la spec principale, où le prochain `validate --specs`
le trouvera. C'est cohérent avec la sémantique d'un sync : « rends visible
tout de suite », par opposition à archive qui clôt.

### Décision : `sync` sépare `changed` de `already_up_to_date` dans son rapport

Un delta déjà fusionné (par exemple : `sync` puis relance de `sync` sans
modification du change) ne doit pas apparaître comme un changement — même
raisonnement que `apply::execute` sur les fichiers identiques. Le rapport
distingue donc `updated` (fichier réellement modifié) de `unchanged`
(fichier identique après merge). Utile en pre-commit, et rend `codev sync`
idempotent au sens strict.

### Décision : format des nouveaux fichiers de spec principale

Une nouvelle spec principale est rendue selon un canon strict :

```text
# <Capacité en Title Case> Specification

## Purpose

<texte du Purpose du delta>

## Requirements

<blocs des ADDED, un par ligne blanche>
```

Le titre `# <...>` est dérivé du chemin de la capacité (`identity/user-auth` →
`User Auth`). Ce n'est ni parseur-critique ni utilisateur-critique — un humain
pourra le corriger à la main plus tard — mais c'est mieux que « # spec ».

**Alternative écartée** : imposer un template `spec.md` externe (comme pour
les artefacts de change). Ajouter un point de configuration pour un cas où
l'auteur écrit lui-même la spec **une fois créée** serait de la cérémonie ; le
canon suffit à faire démarrer.

### Décision : rapports séparés `SyncReport` et `ArchiveReport`

Deux types distincts dans `contract::v1`, plutôt qu'un `MergeReport` unique
avec un `moved_to: Option<...>` :

```
SyncReport    { root, changeName, updated: [...], created: [...], unchanged: [...], status }
ArchiveReport { root, changeName, updated: [...], created: [...], movedTo: "...", status }
```

**Rationale** : un consommateur qui appelle `sync` ne s'attend jamais à voir
un `movedTo` ; un consommateur qui appelle `archive` s'attend toujours à en
voir un. Deux types séparés font disparaître la branche conditionnelle côté
lecteur, au coût de deux structures similaires — coût mineur, gain qui vaut
le duplicat.

## Risques et compromis

- **Fin de section `## Requirements` mal identifiée**. Les `ADDED` doivent
  s'insérer à la fin de cette section, avant une éventuelle section libre
  qui suit. → **Atténuation** : le parseur expose déjà via l'AST l'ordre des
  sections `##` de premier niveau ; on ajoute un accesseur qui rend la
  position du prochain `##` après `## Requirements`, ou la fin du fichier.
  Un test dédié `sync::added_precede_une_section_libre_qui_suit` couvre le
  cas.
- **REMOVED laisse un blanc double**. Supprimer un bloc peut laisser deux
  lignes vides consécutives. → **Compromis assumé** : on étend le
  `byte_range` du REMOVED pour inclure l'espacement qui suit, façon qu'une
  suppression laisse la même densité qu'avant. Un espace supplémentaire
  autour d'une section est cosmétique et se corrige à la main.
- **RENAMED d'une exigence référencée par un MODIFIED du même delta**. Le
  validateur remonte déjà `modified_uses_old_name` ; ici on refuse en cas
  d'incohérence résiduelle. → **Rationale** : la double vérification est
  peu coûteuse et évite le drame silencieux.
- **`Move` cross-device**. Sur macOS/Linux, `rename(2)` échoue entre volumes
  différents. → **Atténuation** : fallback copy + remove dans le port
  `FileSystem`, avec un test qui simule l'échec `rename` en mémoire. Improbable
  en pratique (planning et code vivent dans le même repo), mais un test
  documente le comportement de secours.

## Plan de migration

Sans objet — nouvelle capacité, aucun consommateur existant. Les changes
`parse-specs-and-deltas` et `validate-changes-and-specs` déjà présents dans
ce dépôt deviendront les premiers **candidats à l'archive** une fois ce change
lui-même appliqué : ils passent déjà `codev validate --all`.
