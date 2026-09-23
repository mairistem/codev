# Proposal : boucler le cycle avec sync et archive

## Pourquoi

Un change de codev peut aujourd'hui être créé, planifié, implémenté et
validé — mais il reste **actif indéfiniment**. Rien ne le fait entrer dans les
specs principales, rien ne le retire des changes en cours. C'est le maillon qui
manque pour que le dépôt fasse le cycle complet, et donc pour que codev soit
utile au-delà de la planification.

## Ce qui change

- **Nouvelle commande `codev sync [change]`** : fusionne les deltas d'un
  change dans `_codev/specs/`, sans le déplacer. Le change reste actif — on
  s'en sert quand une nouvelle capacité doit apparaître dans les specs avant
  qu'un autre change ne s'y appuie.
- **Nouvelle commande `codev archive [change]`** : fusionne (comme sync) puis
  déplace le dossier du change vers `_codev/changes/archive/AAAA-MM-JJ-<nom>/`.
  Refuse d'agir si `codev validate <change>` remonte la moindre erreur — le
  pré-flight de validation est le seul rempart entre un delta cohérent et une
  spec principale.
- **Fusion sémantique par opération** : `ADDED` ajoute à la fin de
  `## Requirements`, `MODIFIED` remplace le bloc de l'exigence homonyme au
  caractère près via son span, `REMOVED` supprime le bloc entier, `RENAMED`
  retitre l'en-tête et rien d'autre. Le contenu du fichier que le delta ne
  mentionne pas — commentaires, ordre, espacements, section libre après
  `## Requirements` — reste **strictement inchangé**.
- **Création d'une spec principale pour une nouvelle capacité** : lorsque le
  delta cible une capacité qui n'existe pas encore sous `_codev/specs/`,
  `sync` crée le fichier à partir du `## Purpose` du delta et des exigences
  `ADDED`. Un delta de nouvelle capacité qui ne porterait pas de `## Purpose`
  serait refusé — le pré-flight le vérifie déjà, ce change ne fait qu'y
  ajouter la conséquence.
- **Atomicité par plan d'effets** : la fusion produit d'abord un plan
  complet — main specs à réécrire, main specs à créer, dossier de change à
  déplacer — vérifié dans son intégralité avant qu'une seule écriture ne
  touche le disque. Si un `MODIFIED` ne trouve pas son exigence cible, aucune
  écriture n'a lieu ailleurs. C'est la mise en œuvre concrète de la décision
  [0001](../../decisions/0001-coeur-fonctionnel-coquille-imperative.md) pour
  l'opération la plus délicate de l'outil.
- **Extension du type `Plan`** : nouvelle variante `Move { from, to }`, pour
  que le déplacement du change fasse partie du même plan que les écritures.
  Un `codev archive --dry-run` (à venir) montrera donc à la fois les specs
  qui bougeraient et le dossier qui se déplacerait, en une seule vue.

## Capacités

### Nouvelles capacités

- `spec-merge`

### Capacités modifiées

Aucune. `validation` et `spec-parsing` sont utilisées mais non modifiées : les
règles E1–E3 servent de pré-flight, les spans des blocs (B6) servent aux
réécritures ciblées. Aucun contrat public existant n'est touché.

## Impact

- **Code** : nouveau module `codev-core::merge` (calcul pur des edits à partir
  d'un `Delta` et d'un `Spec`), nouveau module `codev-engine::sync` et
  `codev-engine::archive`, nouvelle variante `Plan::Move`, deux
  sous-commandes CLI, deux formes `contract::v1` (`SyncReport`,
  `ArchiveReport`).
- **Dépendances** : aucune nouvelle.
- **Hors périmètre** :
  - **`retire_capabilities: true`** (F5, lot 2) — un `REMOVED` qui viderait la
    spec principale est **refusé** ici, faute de marqueur permettant à
    l'auteur d'assumer la suppression du fichier.
  - **Archive en lot** (F7, lot 4) — `codev archive` prend un change à la
    fois. Deux changes qui touchent la même spec doivent être archivés l'un
    après l'autre, l'utilisateur choisit l'ordre.
  - **Pré-flight croisé `MODIFIED` contre la main spec** (E6, lot 2) — le
    parseur ne compare pas encore le nom `MODIFIED` à ce que la spec
    principale contient. Ce change le fait **au moment de la fusion**
    (obligatoire pour construire le plan), et remonte une erreur avant écriture
    ; il ne l'ajoute pas au validateur autonome.
  - **Détection de conflits inter-changes** — deux changes actifs qui
    touchent la même exigence n'entrent pas en conflit ici puisqu'un seul est
    archivé à la fois. Le premier archivé gagne ; le second échoue s'il devient
    incohérent, avec un message pointant vers son `MODIFIED` orphelin.
