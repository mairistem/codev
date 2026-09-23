# Proposal : permettre la dérive locale d'une décision héritée

## Pourquoi

Aujourd'hui, un projet qui hérite d'une source (`path:` ou `git:`)
consomme ses ADR **tels quels**. Si l'équipe consommatrice comprend une
décision héritée et choisit délibérément de faire autrement — parce qu'un
contexte local le justifie —, elle n'a **aucun moyen propre** de le
consigner :

- `codev decision supersede path:~/partage/0100 "…"` est déjà refusé (code
  `cannot_supersede_inherited`) — une décision héritée reste en lecture
  seule côté consommateur, c'est acquis depuis
  [0005](../../decisions/0005-sources-heritees-en-lecture-seule.md).
- Écrire un ADR local sans référence à l'héritée laisse l'agent face aux
  deux décisions en même temps dans les instructions de `design` : il ne
  sait pas laquelle prime.

Résultat aujourd'hui : la seule voie propre, c'est une note dans le
`design.md` du change en cours, invisible pour tous les changes suivants.
K6 comble ce trou en donnant un **geste explicite** : « on comprend la
décision héritée, on choisit une alternative locale, la trace est là ».

## Ce qui change

- **Nouveau champ frontmatter `deviates_from`** — liste d'identifiants
  qualifiés (`path:~/partage/0100`, `git:git@github.com:acme/shared.git/0100`).
  Additif : un ADR sans ce champ conserve sa sémantique actuelle.
- **Nouvelle commande `codev decision deviate <qualified-id> <titre>`** —
  crée un ADR local `accepted` avec `deviates_from: ["<qualified>"]`,
  scellé comme n'importe quel autre ADR local (K3).
- **L'index prend en compte les dérives** — une décision héritée
  référencée par un `deviates_from` local est marquée `deviated_by:
  <qualified-local>` dans l'index. Elle reste visible dans `codev
  decision list` (transparence), mais **disparaît** des instructions
  injectées à `design` — l'agent voit la dérive, pas la décision
  qu'elle remplace.
- **Nouveau finding stable `decision_dangling_deviation`** (warning) —
  quand un `deviates_from` cible un `qualified-id` qui n'existe plus
  (source déplacée, SHA changé, dossier retiré).
- **Nouveau finding stable `decision_conflicting_deviations`** (erreur) —
  quand deux ADR locaux dévient de la même cible héritée. La règle est :
  « une cible, une dérive ».
- **Contrat JSON** — `DecisionV1` gagne un `deviatesFrom: Vec<String>`
  (additif, jamais rempli pour un ADR sans le champ). Les entrées
  d'index héritées gagnent un `deviatedBy: Option<String>` calculé.
- **Refus explicites** :
  - Dévier d'une décision **locale** est refusé (code
    `cannot_deviate_from_local`) — la voie propre pour ça, c'est
    `codev decision supersede`.
  - Dévier d'un id inconnu est refusé (code `unknown_decision_id`, code
    déjà existant).

## Capacités

### Nouvelles capacités

Aucune.

### Capacités modifiées

- `decisions` — quatre nouvelles exigences ADDED : format et sémantique
  de `deviates_from`, comportement de la commande `decision deviate`,
  effet sur l'index et l'injection dans design, findings de validate sur
  les cas dégénérés.

## Impact

- **Code** : extension du parseur ADR (`codev-core::decisions::parser`)
  pour lire le champ `deviates_from` ; extension de l'index
  (`codev-engine::decisions`) pour calculer `deviated_by` et retirer les
  héritées déviées du calcul `in_effect` côté consommateur ; extension
  de `plan_new_decision` / création d'un `plan_deviate` dans
  `codev-engine::decisions_actions` ; extension de `validate_decisions`
  pour les deux nouveaux findings ; nouvelle sous-commande CLI.
- **Contrat JSON** : deux champs additifs sur `DecisionV1`
  (`deviatesFrom`, `deviatedBy`) et un `DecisionDeviatedV1` proche de
  `DecisionCreatedV1`. Aucun champ retiré ni renommé.
- **Fichier écrit** : rien de nouveau — l'ADR local est écrit dans
  `_codev/decisions/NNNN-<slug>.md`, comme n'importe quel autre. Le
  sceau est ajouté à `seal.yaml` par le même plan (cohérent avec K3).
- **Injection dans instructions design** : une décision héritée déviée
  disparaît du tableau `decisions[]` **et** de la section humaine
  « Décisions en vigueur ». Un consommateur du contrat ne voit que la
  dérive.
- **Migration** : aucune. Les 6 ADR existants n'ont pas de
  `deviates_from`, ce champ est optionnel, l'index calcule
  `deviatedBy` à vide.
- **Hors périmètre** :
  - **Dériver d'une décision locale** — refusé par ce change. Un `codev
    decision revise <id>` (édition contrôlée avec re-sceau) est un autre
    geste, à discuter séparément si besoin.
  - **Dériver d'une décision héritée déjà déviée par la source elle-même**
    — traité comme une dérive normale sur la nouvelle décision. Pas de
    logique de résolution en cascade dans ce lot.
  - **Interface interactive pour choisir quoi dévier** — la commande
    prend un `qualified-id` en argument, l'utilisateur le connaît via
    `codev decision list`. Pas d'assistant.
