# Design : promotion d'une décision depuis un `design.md`

## Contexte

Voir `proposal.md`. K7 est le dernier trou du cycle de vie des décisions :
K3 verrouille l'immutabilité, K6 permet la dérive sur héritées, K7 fait
naître un ADR à partir d'une réflexion vécue dans un change.

## Objectifs / Hors objectifs

Ce design cadre : la sélection du bloc, la génération du corps de l'ADR,
la réécriture du design, la commande CLI, le contrat JSON, les refus.
Il ne cadre pas : la promotion multiple en une commande, la
structuration auto en Contexte/Décision/Conséquences (impossible sans
ambiguïté), ni un `unpromote`.

## Décisions

### Décision : parser léger en interne, pas de dépendance markdown

Le format des blocs à extraire est très contraint :

```
## Décisions
### Décision : <titre>
<corps>
### Décision : <autre titre>
<corps>
```

Un scan ligne à ligne suffit : trouver `## Décisions`, puis boucler sur
`### Décision : ...`, et récupérer les octets entre deux `###` (ou
jusqu'au prochain `## `). Aligné avec la décision
[0001](../../decisions/0001-coeur-fonctionnel-coquille-imperative.md) —
le parseur est pur et vit dans un nouveau module `codev-engine::design`
(la fonction pure prend le source du design en argument, la coquille
fait la lecture disque).

**Alternative écartée** : réutiliser un parseur markdown existant
(pulldown-cmark, comrak). Coût dépendance et parsing riche pour un
besoin ultra-cadré. Le codev a une tradition de parseurs à la main dans
`codev-core::parser` — on la suit.

### Décision : le corps est reproduit verbatim

Le corps d'un bloc `### Décision : <titre>` va **byte pour byte** dans
la section `## Décision` du nouvel ADR — pas de trim, pas de
reformatage, pas de dédentation. C'est ce qui permet à l'auteur de
retrouver son texte à l'identique et de le ventiler ensuite en
Contexte / Décision / Conséquences / Alternatives écartées sans avoir
à deviner ce qu'on aurait modifié.

**Alignement** avec la décision de K3 : le corps d'un ADR est traité
byte pour byte pour le hash de sceau. Cohérent d'un bout à l'autre.

### Décision : la référence dans le design est du texte, pas un lien

`> Promue en ADR **NNNN** — voir `` `_codev/decisions/NNNN-<slug>.md` ``.

Un lien `[..](../../decisions/NNNN-slug.md)` fonctionnerait tant que le
change est actif (`_codev/changes/<nom>/design.md` → `../../decisions/`
résout à `_codev/decisions/`), mais casserait après archive
(`_codev/changes/archive/<date>-<nom>/design.md` → `../../decisions/`
résout à `_codev/changes/decisions/`, inexistant). Le chemin sous forme
de texte reste **compréhensible** depuis n'importe quelle profondeur ;
l'humain trouve.

**Alternative écartée** : lien absolu `/_codev/decisions/...`. Marche
sur un site web servi depuis la racine du repo, mais pas dans un simple
`less design.md`. Trop de suppositions sur l'environnement de lecture.

### Décision : le titre du bloc reste, seul le corps est remplacé

Deux options :

| Option | Pro | Contre |
|---|---|---|
| **A. Retirer le bloc entier** | Design plus court après plusieurs promotions | L'historique de « on a discuté X » disparaît du design ; le lecteur ne sait plus pourquoi l'ADR existe |
| **B. Garder `### Décision : <titre>` + citation** | Traçabilité maintenue, le sujet reste lisible dans le design | Fichier légèrement plus long |

**Choisi : B.** Le design reste une trace de la discussion ; un lecteur
qui parcourt `design.md` après coup voit toujours quels arbitrages ont
été faits — et voit qu'ils ont été *hissés* au rang d'ADR. C'est
précisément la valeur qu'ajoute la promotion.

### Décision : `plan_promote` réutilise `plan_new_decision` de K3

Pas de nouvelle fonction pure de création d'ADR — on réutilise
`decisions_actions::plan_new_decision` en lui passant le titre extrait
et un corps « prérempli » (via une variante du rendu qui prend le corps
en argument). Le sceau est ajouté au plan, comme pour toute nouvelle
décision `accepted`. Le plan gagne en plus **un write** pour la
substitution du bloc dans `design.md` — d'où un plan atomique à 3
écritures : ADR + seal.yaml + design.md.

### Décision : refus des archivés est explicite via `codev list`

Comme dans K3 (`decision seal`) et K6 (`decision deviate`), la commande
refuse si le nom du change ne figure pas dans `codev list` (qui ne montre
que les actifs). Code stable dédié `cannot_promote_from_archived` pour
que l'agent puisse distinguer d'un `unknown_change` (par exemple pour
proposer à l'utilisateur d'ouvrir manuellement l'archive s'il insiste).

## Risques et compromis

- **Le corps verbatim peut inclure du markdown incohérent** — un `###`
  imbriqué dans le corps (peu probable mais possible) tromperait le
  scan. → **Compromis assumé** : la spec dit « jusqu'au prochain
  `###` », c'est le contrat ; l'auteur qui écrit un `###` dans un bloc
  de décision aura un ADR tronqué. Documenté. Un warning validate
  pourrait venir plus tard.
- **Un design édité entre le calcul du plan et son application** — un
  auteur qui édite `design.md` pendant que la commande tourne verrait
  sa modification écrasée. → **Compromis assumé** : atomicité à
  l'échelle du processus seulement, comme pour toutes les commandes.
- **Deux titres ambigus** — l'auteur peut avoir dupliqué un titre en
  itérant sur sa formulation. → **Traité** par le refus
  `ambiguous_decision_heading` qui nomme les deux positions dans le
  fichier (numéros de ligne).

## Plan de migration

Aucune. Les designs existants ne changent pas ; la commande est opt-in.
