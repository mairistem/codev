# Design : commandes CLI pour les décisions

## Contexte

Voir `proposal.md` pour la motivation. Le module
`codev-engine::decisions` livré par le change précédent fournit déjà
l'index avec `in_effect` et la résolution qualifiée. Ce change en fait
la brique de résolution centrale : chaque commande (`new`, `list`,
`show`, `supersede`) part d'un index à jour et calcule un `Plan` à
exécuter.

## Objectifs / Hors objectifs

Ce design cadre :

- où vivent les fonctions de calcul de plan (cœur pur vs coquille) ;
- la forme des nouveaux types du contrat public ;
- l'atomicité de `supersede`, qui réécrit deux fichiers.

Il ne cadre **pas** l'immuabilité (K3), la déviation (K6), la promotion
depuis design (K7), ni l'édition interactive dans `$EDITOR`.

## Décisions

### Décision : les fonctions `plan_new` et `plan_supersede` vivent dans `codev-engine`

Elles n'écrivent pas — elles produisent un `Plan`, comme
`plan_init` ou `plan_new_change`. Suit directement la décision
[0001](../../decisions/0001-coeur-fonctionnel-coquille-imperative.md) :
décider n'est pas exécuter. La coquille CLI applique le plan via
`apply::execute`, ce qui rentabilise le mécanisme d'idempotence et de
`--dry-run` (ce dernier arrivera quand un consommateur le demandera).

**Alternative écartée** : mettre le calcul dans `codev-core::decisions`.
Bloqué : la génération du prochain `id` exige de connaître les décisions
déjà présentes, donc le résultat de l'index — qui est côté engine
(lecture disque). Impossible de rester pur sans traverser le port.

### Décision : le squelette d'ADR est embarqué, pas configurable

Le fichier `assets/templates/decision.md` est inclus dans le binaire via
`include_str!` et interpolé avec l'`id`, le titre, le statut et la date au
moment de la création. C'est aligné avec la façon dont les templates
d'artefacts fonctionnent déjà — cf. la décision
[0002](../../decisions/0002-graphe-de-crates-comme-regle-de-dependance.md)
qui préfère les données figées à des points d'extension prématurés.

**Coût accepté** : un projet qui voudrait un squelette de décision
différent devra l'écrire à la main. Templater le fichier via
`_codev/templates/decision.md` sera un futur change au premier besoin
réel.

### Décision : `supersede` réécrit le frontmatter du prédécesseur, jamais son corps

L'ancien ADR est réécrit avec un nouveau frontmatter — `status:
superseded` — mais tout ce qui suit le second `---` reste au caractère
près. Le parseur porte déjà `frontmatter_span` sur `Decision`, donc
l'opération est un remplacement ponctuel de la plage `[0..frontmatter_span.end)`
par le nouveau frontmatter. Même mécanique que `merge::apply_edits` —
même invariant testable par golden.

**Rationale** : le corps est ce qu'a écrit l'auteur. L'outil réécrit un
statut, pas une décision.

### Décision : `supersede` refuse de toucher à une décision héritée

Une décision héritée est **en lecture seule** — décision
[0005](../../decisions/0005-sources-heritees-en-lecture-seule.md). La
commande refuse avec le code `cannot_supersede_inherited` et suggère la
déviation (K6, à venir). C'est la première fois qu'on distingue lecture
et écriture selon l'origine ; les futures commandes `deviate` et
`promote` s'appuieront sur la même règle.

### Décision : le squelette d'ADR ne contient que les sections utiles

Quatre sections : **Contexte**, **Décision**, **Conséquences**,
**Alternatives écartées**. Ce sont celles qu'utilisent les six ADR
existants du dépôt. Un ADR est court par nature — imposer plus est du
bruit.

### Décision : les types du contrat sont trois structs distincts

- `DecisionV1` — la forme complète, utilisée par `list` (chaque entrée du
  tableau) et par `show` (au niveau racine).
- `DecisionCreatedV1` — rendu par `new`, contient `decision:
  DecisionV1` plus `path: String`.
- `DecisionSupersededV1` — rendu par `supersede`, contient `newDecision`
  et `oldId` avec `oldPath`.

Trois structs plutôt qu'une seule flexible : un consommateur qui appelle
`new` n'a rien à faire du champ `oldId`, et l'inverse. Le compilateur
attrape la mauvaise commande côté agent avant que la mauvaise clé
n'entre en circulation.

## Risques et compromis

- **Génération d'id concurrentielle**. Deux `codev decision new` lancés
  en même temps calculeraient le même `id` et l'un écraserait l'autre. →
  **Atténuation** : `plan_new` produit un `WriteMode::CreateOnly` sur le
  fichier de sortie. Le second exec échoue avec `already_exists`. Rare en
  pratique (usage humain, non batch) et signalé plutôt que masqué.
- **Titre à caractères Unicode inhabituels** (émoji, ponctuation étrangère).
  Le slug perd ces caractères, le nom de fichier peut devenir court ou
  vide. → **Compromis assumé** : si le slug est vide après nettoyage,
  utiliser `decision` comme fallback (`0007-decision.md`). Documenté dans
  le rendu.
- **Corps du prédécesseur avec `\r\n`**. La lecture puis réécriture peut
  normaliser en `\n` par inadvertance. → **Atténuation** : le contenu
  après `frontmatter_span.end` est repris tel quel, byte à byte — même
  logique que `merge::apply_edits`.

## Plan de migration

Sans objet — nouvelles commandes. Les six ADR existants du dépôt
serviront de premier vrai test au moment de créer une septième décision
par la commande neuve — vérification directe que la numérotation « + 1 »
tombe bien sur `0007`.
