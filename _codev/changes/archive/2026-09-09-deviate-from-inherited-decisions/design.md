# Design : dérives locales d'une décision héritée

## Contexte

Voir `proposal.md`. K3 vient de fixer l'immutabilité des ADR locaux ; ce
change (K6) complète le cadre côté sources héritées : le consommateur ne
peut pas modifier ce qu'il hérite, mais doit pouvoir **enregistrer sa
divergence** de manière lisible pour l'outil.

## Objectifs / Hors objectifs

Ce design cadre le nouveau champ, la commande CLI, l'effet sur l'index,
l'injection dans `design`, et les deux nouveaux findings de validate. Il
ne cadre ni la promotion depuis `design.md` (K7), ni les dérives
transitives (dériver d'une dérive), ni un mode « annuler ma dérive »
(retirer l'ADR local suffit — pas de geste dédié).

## Décisions

### Décision : nouveau champ `deviates_from`, distinct de `supersedes`

Deux options :

| Option | Pro | Contre |
|---|---|---|
| **A. Réutiliser `supersedes`** avec des identifiants qualifiés | Un seul mécanisme à connaître | Sémantique déjà chargée — « supersede » veut dire « remplace ». Chez le projet source, la décision n'a rien été remplacée du tout : c'est faux de le dire |
| **B. Nouveau champ `deviates_from`** | Sémantique propre à ce cas — « on la comprend, on choisit autrement » | Un champ frontmatter de plus |

**Choisi : B.** Un ADR est un document historique ; y écrire « supersedes
path:~/partage/0100 » ferait croire au lecteur que la source elle-même a
retiré cette décision. La dérive est un geste **local** ; le champ doit
en refléter la portée. Alignement avec la décision
[0005](../../decisions/0005-sources-heritees-en-lecture-seule.md) : les
sources héritées sont **lecture seule**, même la sémantique de leur
statut ne se pilote pas depuis le consommateur.

### Décision : la dérive ne s'applique qu'à des cibles héritées

`codev decision deviate projet/0003 …` est refusé (code
`cannot_deviate_from_local`), avec un renvoi vers `codev decision
supersede`. Deux gestes distincts, chacun avec sa sémantique :

- `supersede` — « on remplace notre propre décision par une nouvelle » ;
  le fichier de l'ancienne bascule à `status: superseded`.
- `deviate` — « on comprend la décision héritée, on l'écarte
  localement » ; **rien** n'est écrit du côté de la source, l'ADR local
  porte la trace.

Confondre les deux ferait perdre la nuance qui les distingue.

### Décision : occultation dans les instructions `design`, mais visibilité dans `decision list`

L'index calcule un attribut `deviated_by: <qualified-local>` sur chaque
entrée héritée référencée par un ADR local `accepted`. Deux effets :

- **Instructions de `design`** — l'entrée déviée n'apparaît **plus** dans
  le tableau `decisions[]` ni dans la section humaine « Décisions en
  vigueur ». L'agent qui rédige un `design.md` voit la dérive, pas la
  décision remplacée — sinon il proposerait de la respecter, faussement.
- **`codev decision list`** — l'entrée déviée reste listée, avec son
  `deviatedBy` visible. La transparence prime : l'utilisateur doit
  pouvoir voir tout ce qui existe dans l'index, y compris les héritées
  écartées.

**Alternative écartée** : masquer la déviée partout. Rend le suivi
impossible — l'utilisateur ne saurait plus qu'une décision existe côté
source tant que quelqu'un ne lui dit pas.

### Décision : une cible, une dérive — enforcée par validate

Deux ADR locaux qui dévient de la même cible → conflit non résoluble :
lequel des deux prime ? On refuse de trancher silencieusement et on
émet `decision_conflicting_deviations` en **erreur**. L'utilisateur
choisit — soit il retire un des deux ADR, soit il en supersede un par
l'autre.

**Alternative écartée** : garder la plus récente. Un `date:` de
frontmatter est un champ libre, un utilisateur pourrait le mentir. On
refuse de résoudre par heuristique.

### Décision : une cible qui disparaît → warning, pas erreur

Une source déplacée (`git:` retirée du config, SHA changé qui masque le
fichier, `path:` renommé) casse la cible d'un `deviates_from`. Deux
scénarios raisonnables :

1. Le projet a évolué et la source aussi ; la dérive n'a plus de sens →
   le retirer.
2. La source est temporairement inaccessible ; la dérive est toujours
   pertinente → attendre.

`decision_dangling_deviation` en **warning** couvre les deux sans
bloquer les flows (`sync`, `archive`). L'utilisateur voit et décide.

**Alignement** avec la décision de K3 : `decision_unsealed` est aussi
warning (migration ne bloque pas) ; `decision_seal_mismatch` est erreur
(l'index n'est plus fiable). Ici, une dérive orpheline ne compromet pas
l'index — la cible est juste absente.

### Décision : l'ADR local de dérive est un ADR normal, scellé par K3

Rien de spécial côté seal : `plan_deviate` produit un plan qui écrit
l'ADR **et** l'entrée de sceau, exactement comme `plan_new`. La règle
« accepted → scellé » reste vraie.

**Corollaire** : `codev decision deviate <cible> <titre>` refuse si la
cible est ambiguë (deux sources exposent le même `<qualified-id>`) ; le
code stable est `ambiguous_decision_id` (déjà existant).

## Risques et compromis

- **Une source qui, à son tour, dévie de la même cible** que le projet
  consommateur — l'index côté source ne connaît pas la notion de
  dérive du consommateur, donc pas de conflit détecté chez la source.
  → **Compromis assumé** : ce lot ne fait pas de résolution en cascade
  (hors périmètre du proposal). L'utilisateur verra les deux dérives
  côte à côte dans `codev decision list`, il tranchera.
- **Un utilisateur pourrait dévier « pour tester »** puis oublier de
  retirer l'ADR local, laissant une entrée `deviatesFrom` qui n'a plus
  de sens. → **Atténuation** : le workflow de propose/apply/archive
  laisse une trace propre — un ADR de dérive est un ADR complet, avec
  contexte et décision, pas un tag jetable.
- **`deviatedBy` calculé, pas persisté** — l'index le recalcule à
  chaque appel. Coût négligeable (une passe de plus sur les entrées
  déjà chargées), et évite le problème du « comment garder à jour un
  attribut dérivé quand le fichier change ».

## Plan de migration

Aucune. Le champ `deviates_from` est optionnel ; les ADR existants n'en
ont pas ; l'index calcule `deviatedBy` à vide pour toutes les entrées.
