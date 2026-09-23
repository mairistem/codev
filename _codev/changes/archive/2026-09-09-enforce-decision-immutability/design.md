# Design : verrouiller l'immutabilité des décisions

## Contexte

Voir `proposal.md`. L'enjeu : rendre l'immutabilité **mécaniquement**
vérifiable — pas seulement énoncée dans une convention. Sans ça, K6
(dérives permises des décisions héritées) n'a pas de base solide sur
laquelle raisonner.

## Objectifs / Hors objectifs

Ce design cadre l'emplacement du sceau, ce sur quoi le hash porte, la
séparation cœur/coquille, et les commandes CLI touchées. Il ne cadre
pas la signature cryptographique (GPG et cie), ni la promotion des
décisions depuis `design.md` (c'est K7).

## Décisions

### Décision : sceau dans un fichier séparé `_codev/decisions/seal.yaml`

Deux options considérées :

| Option | Pro | Contre |
|---|---|---|
| **A. Champ `bodySha256` dans le frontmatter de chaque ADR** | Self-contained ; `git diff` révèle direct la falsification | Ajoute un champ « machine » qui pollue la lecture humaine d'un ADR ; oblige à répondre à la question « et le hash lui-même, il est dans le hash ? » (non — mais c'est déroutant à première lecture) |
| **B. Fichier séparé `seal.yaml` à côté des ADR** | ADR reste propre pour l'œil humain ; aligné avec le pattern déjà en place (`codev.lock` pour les sources) | Un fichier de plus à tenir cohérent |

**Choisi : B.** Le motif dépasse ce cas — codev a déjà un fichier de
vérité tenu par le CLI (`codev.lock` pour les sources), on répète le
motif au lieu d'inventer un mode nouveau. Le fichier `seal.yaml` vit
sous `_codev/decisions/` pour rester au plus près de son sujet, et
`serde_norway` le parse comme le reste du YAML — cohérent avec la
décision [0006](../../decisions/0006-serde-norway-pour-yaml.md).

### Décision : le hash porte sur le **corps**, pas sur le fichier entier

Le frontmatter d'un ADR est conçu pour évoluer légitimement — un ADR
`accepted` devient `superseded` par une écriture explicite de
`codev decision supersede`. Hasher le fichier entier obligerait à
re-sceller à chaque transition, ce qui priverait l'exigence
« supersede ne touche pas au corps » (déjà dans la spec) de son
attribut vérifiable.

**Corollaire** : le calcul du hash coupe au premier `\n---\n` (ou
`\n---\r\n` pour Windows) qui suit la ligne `---` d'ouverture, et hashe
tout ce qui vient après, byte pour byte, sans normalisation. Une
normalisation implicite (trim, LF↔CRLF) casserait la promesse
« identique au caractère près ».

### Décision : `plan_new_decision` et `plan_supersede` retournent un plan
qui inclut l'écriture du sceau

Alignement direct avec la décision
[0001](../../decisions/0001-coeur-fonctionnel-coquille-imperative.md) :
le cœur produit un `Plan { writes, moves, … }` complet ; la coquille
l'exécute d'un bloc. Ajouter l'écriture de `seal.yaml` au plan préserve
l'atomicité — soit l'ADR et le sceau sont écrits, soit rien ne l'est,
sans avoir à inventer une compensation.

**Corollaire** : la fonction pure qui produit le plan a besoin de lire le
`seal.yaml` **actuel** pour le fusionner avec la nouvelle entrée. Elle
reçoit son contenu en argument (le port `FileSystem` de la coquille l'a
lu au préalable), elle ne le lit pas elle-même — décision
[0002](../../decisions/0002-graphe-de-crates-comme-regle-de-dependance.md).

### Décision : `validate` remonte des findings, pas des exceptions

`decision_unsealed` est un **warning** (code sortie 0), pas une erreur :
sur un projet existant, tous les ADR sont d'abord unsealed — un errno
non nul empêcherait tous les autres flows (`codev status`, `codev sync`,
`codev archive`) de tourner jusqu'à ce que le sceau soit fait. Le
warning attire l'attention sans bloquer.

`decision_seal_mismatch` est une **erreur** (code sortie non nul) : un
corps qui ne correspond plus au sceau, c'est de la falsification (ou une
édition volontaire non ré-approuvée) — l'index de décisions ne peut
plus être considéré fiable tant que ce n'est pas résolu.

`decision_orphan_seal` est un **warning** : un ADR peut avoir été
supprimé volontairement (peu probable mais possible) ; l'orphan lock
seul ne compromet rien de gravement.

**Alternative écartée** : tout aligner en erreur. Rend la migration
impraticable — première `codev validate` post-livraison échoue sur les 6
ADR existants, cassant `sync`, `archive` et le reste. Coûte trop cher
pour ce qu'on gagne.

### Décision : commande CLI `codev decision seal <id>` unique, avec `--force`

Une seule commande, deux modes selon l'état :

- ADR non scellé → ajoute l'entrée sans discussion (cas de migration).
- ADR scellé et le corps a changé → **refuse** sans `--force`, avec le
  code stable `seal_conflict` ; avec `--force`, réécrit le
  `bodySha256` et rafraîchit `sealedAt`.
- ADR scellé et le corps est inchangé → no-op silencieux (le sceau est
  déjà correct).

Une commande `codev decision seal --all` (bulk) pour la migration
initiale est reportée : `for id in $(codev decision list --json | jq
-r …); do codev decision seal "$id"; done` fait le travail sur les 6
ADR de ce dépôt sans nécessiter un chemin dédié dans le CLI. Si le
pattern devient récurrent, on l'ajoutera plus tard.

### Décision : les décisions héritées ne sont **pas** scellées par le consommateur

Un projet consommateur ne peut pas apposer un sceau sur un ADR qu'il n'a
pas écrit — ce serait usurper le geste d'acceptation du projet source.
Le sceau vit dans le projet source ; le consommateur, quand il indexera
les décisions héritées (déjà en place), pourra optionnellement vérifier
leur `seal.yaml` distant s'il y en a un (reportable, pas dans ce
change).

**Alignement** avec la décision
[0005](../../decisions/0005-sources-heritees-en-lecture-seule.md) : les
sources héritées sont en lecture seule. `codev decision seal
path:~/partage/0100` renvoie donc `cannot_seal_inherited`.

## Risques et compromis

- **Migration silencieuse manquée.** Si l'utilisateur ne voit pas les
  warnings `decision_unsealed` (par exemple parce qu'il ne lance jamais
  `validate` manuellement — il passe par `sync` ou `archive` qui font
  un pré-flight `validate`), il pourrait laisser ses ADR sans sceau
  longtemps. → **Atténuation** : `codev status` (qui n'a pas encore de
  pré-flight validate) gagnera le comptage des warnings dans son résumé
  humain, dans un change futur. Pour l'instant, la mention dans le
  résumé de `sync`/`archive` est suffisante.
- **Faux positif sur les fins de ligne.** Le hash byte-pour-byte
  attrapera un `LF → CRLF` malencontreux (git config `core.autocrlf`,
  éditeur qui reformate). → **Compromis assumé** : `codev decision seal
  --force` est la voie officielle. Documenter dans le message d'erreur.
- **Le sceau ne protège pas contre l'auteur qui édite en connaissance
  de cause.** Un développeur peut lancer `--force` sans réfléchir. →
  **Compromis assumé** : le sceau est un garde-fou technique, pas un
  contrôle d'accès. `git blame` reste l'ultime trace.

## Plan de migration

Après livraison :

1. `codev validate` remonte 6 warnings `decision_unsealed` sur ce dépôt.
2. Boucle courte : `codev decision list --json | jq -r '.decisions[] |
   select(.origin == "projet") | .id' | while read id; do codev
   decision seal "$id"; done`. Un commit unique porte les 6 sceaux
   ajoutés dans `_codev/decisions/seal.yaml`.
3. `codev validate` redevient à 0 warning côté décisions.
