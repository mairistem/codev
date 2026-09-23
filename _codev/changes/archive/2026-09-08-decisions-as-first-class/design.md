# Design : décisions comme objet de première classe

## Contexte

Voir `proposal.md` pour la motivation. Le parseur de specs et son AST à
spans (livrés par `parse-specs-and-deltas`) sont réutilisables tels quels
pour lire les sections d'un ADR — inutile d'écrire un second parseur
markdown. Le frontmatter YAML est traité par `serde_norway`, déjà présent
comme choisi par la décision
[0006](../../decisions/0006-serde-norway-pour-yaml.md).

## Objectifs / Hors objectifs

Ce design cadre :

- la répartition cœur / engine — parseur pur dans `codev-core::decisions`,
  index et fusion des sources dans `codev-engine::decisions` ;
- la forme publique du contrat JSON pour les décisions ;
- la stratégie d'injection dans `instructions design`, sans casser
  l'existant.

Il ne cadre **pas** les commandes CLI `codev decision …` (K5), la détection
d'immuabilité (K3), la déviation (K6), la promotion depuis design (K7). Ces
quatre points forment un lot cohérent à traiter ensuite.

## Décisions

### Décision : parseur pur dans `codev-core::decisions`

Comme `parser`, `validate`, `merge` — un module de plus dans `codev-core`,
sans I/O. Il expose `parse_decision(source: &str) -> Parsed<Decision>` qui
rend un `Decision { id, title, status, date, tags, supersedes, sections,
span }` avec la liste des `Finding` structurels rencontrés.

Le parseur du frontmatter emprunte la mécanique déjà éprouvée : `serde_norway`
sur le bloc entre les deux `---`. Le corps markdown est parsé en sections
`##` de premier niveau via les briques de `parser::shared` — sans réécrire
la reconnaissance des zones littérales. Suit directement la décision
[0001](../../decisions/0001-coeur-fonctionnel-coquille-imperative.md) — cœur
pur — et le graphe de crates de la décision
[0002](../../decisions/0002-graphe-de-crates-comme-regle-de-dependance.md).

**Alternative écartée** : parseur ADR dédié dans un nouveau crate. Le
volume est trop faible pour justifier une frontière de plus, et
`codev-core` reste sans nouvelle dépendance.

### Décision : index et supersession dans `codev-engine::decisions`

L'engine coordonne la lecture du disque et la fusion avec les sources
héritées. Il expose `pub fn index(fs, layout, config) -> DecisionIndex` qui
rend :

```rust
pub struct DecisionIndex {
    pub entries: Vec<IndexEntry>,     // toutes les décisions parsées
    pub in_effect: Vec<QualifiedId>,  // qualifié source+id
    pub findings: Vec<Finding>,       // conflits d'id, supersedes fantômes...
}
```

Le calcul de `in_effect` est un simple parcours de graphe : une décision
`accepted` non supersedée par une autre décision `accepted` est en vigueur.
L'implémentation est un `BTreeMap` de qualifieurs plus un balayage linéaire
— la taille (dizaines d'ADR max en pratique) ne justifie rien de plus.

**Rationale** : la coordination touche au disque, donc elle vit dans
l'engine. Suit la décision
[0001](../../decisions/0001-coeur-fonctionnel-coquille-imperative.md).

### Décision : identifiants qualifiés `origin/id`

Un `QualifiedId` est un `(origin, id)` — `origin` étant `"projet"` ou
`"path:<chemin déclaré>"`. C'est ce qui permet à deux sources de coexister
sans collision. La sérialisation JSON produit un champ `id` court (le
numérique local) et un champ `qualifiedId` (`"projet/0007"` ou
`"path:~/partage/0100"`) — pour que les consommateurs simples puissent se
contenter du premier et les consommateurs multi-sources aient le second.

**Alternative écartée** : namespacer directement l'`id` (`"partage-0100"`).
Change le format des ADR à la lecture, obligerait un projet qui migre à
renommer ses fichiers. Le qualifieur externe préserve les fichiers tels
qu'ils sont.

### Décision : la version du projet gagne en cas de collision d'id

Quand un même `id` apparaît dans le projet et dans une source héritée, on
émet un finding `decision_id_collision` mais **la version du projet est
retenue comme en vigueur** — la version héritée devient « masquée ». Le
finding suffit à faire remonter le problème sans bloquer le rendu.

**Rationale** : la version du projet est plus proche de l'auteur et
sûrement plus récente. Refuser plutôt que masquer arrêterait la commande
sur un cas récupérable.

### Décision : champ `decisions[]` ajouté à `InstructionsV1`, pas fusionné avec `context`

Un nouveau champ dédié au niveau racine :

```json
{
  "changeName": "…",
  "artifact": "design",
  "context": [ … ],
  "rules": [ … ],
  "decisions": [
    { "id": "0007", "qualifiedId": "projet/0007",
      "title": "…", "status": "accepted", "tags": ["architecture"],
      "path": "_codev/decisions/0007-….md", "origin": "projet" }
  ],
  …
}
```

Rétrocompatible : ajout de champ, pas de rupture. Le champ est présent et
vide dans les réponses des artefacts non-`design`.

**Rationale** : fusionner dans `context` obligerait à passer par
`origin: "decisions"` et à sérialiser tout le contenu — perdant la
structure exploitable (id, statut, tags). Un champ typé est plus clair
côté agent : « lis ces décisions avant de rédiger » sans confusion avec du
contexte projet libre.

### Décision : le rendu humain ajoute une section « Décisions en vigueur »

Une seule section, une ligne par décision : `- <id> <title>`. Ni le
contenu, ni le path — l'agent qui produit le design va lire les fichiers.
Le rendu humain sert au coup d'œil, la richesse est côté JSON.

**Rationale** : reste compact, aligné sur la façon dont `context` et
`rules` sont rendus. Un consommateur qui veut plus a `--json`.

## Risques et compromis

- **Un projet avec 100 décisions** injecterait 100 entrées dans chaque
  appel `instructions design`. → **Compromis assumé** : la liste est
  filtrée aux `in_effect`, en pratique < 30 dans les cas connus. Un jour où
  ce sera trop, on ajoutera un filtre par `tags` — hors périmètre de ce
  change.
- **Le parseur du frontmatter tolère les clés inconnues par défaut avec
  `serde_norway`**. → **Choix retenu** : `deny_unknown_fields` sur la
  struct de frontmatter, cohérent avec le reste du projet (`Schema`,
  `ChangeMetadata`, `ProjectConfig`). Une clé fautive remonte
  immédiatement.
- **Chaîne de supersession circulaire** (`A supersedes B` et `B supersedes
  A`). → **Atténuation** : détectée à la construction de l'index,
  finding `decision_supersession_cycle`, chaînes ignorées pour le calcul
  d'`in_effect` (aucune des deux n'y entre) — force l'auteur à corriger.
- **Un ADR au frontmatter valide mais dont l'id n'est pas kebab-numérique**
  (par exemple `id: my-decision`). → **Choix retenu** : accepter n'importe
  quel `id` non vide, sans imposer de format. Les 6 ADR existants utilisent
  `0001…0006` par convention documentaire, mais l'outil ne l'impose pas.

## Plan de migration

Sans objet — nouvelle capacité. Les 6 ADR déjà présents dans le dépôt
seront indexés par le premier appel `codev instructions design` après
compilation, sans intervention manuelle. C'est le premier test réel du
change.
