# Design : fix de la détection thin

## Contexte

Voir `proposal.md`. Bug observé sur un vrai projet TypeScript (mira) :
la nudge n'apparaît pas alors qu'aucune règle n'a été écrite, parce
que la sonde a rempli un `context:` de 280 caractères.

## Décisions

### Décision : `is_config_thin(rules_empty: bool)` — signature réduite à un seul paramètre

Le paramètre `context: Option<&str>` disparaît. La fonction retourne
strictement `rules_empty`.

C'est un affaiblissement délibéré : on abandonne la finesse « long
contexte compte comme configuré » parce qu'elle produit des faux
négatifs sur les projets à stack riche (TypeScript, Java Maven, tout
projet avec plus de 5-6 dépendances déclarées).

En pratique, l'affaiblissement est neutre :

- Un utilisateur qui a le temps d'écrire un `context:` détaillé
  écrira aussi presque toujours au moins une règle — le cas
  « context long, rules vides » est rare et souvent involontaire.
- Un utilisateur qui n'a écrit ni contexte ni règles ne perd rien à
  recevoir la nudge une deuxième fois — il l'ignore d'un coup d'œil.

**Alternative écartée A** : bumper le seuil de 200 à 1000 caractères.
Rejeté — c'est repousser le problème, pas le résoudre. Un projet
polyglotte peut dépasser 1000 chars de deps détectées.

**Alternative écartée B** : distinguer contexte auto-détecté et
contexte utilisateur via un marqueur (`# rédigé par
/codev-configure`). Plus précis, mais introduit une complexité qui
n'apporte pas grand-chose : la détection sur `rules_empty` couvre le
99e centile.

### Décision : le body de `configure` ne change pas de contrat

Le body `assets/workflows/configure.md` ne mentionne actuellement
aucun seuil sur `context`. La refonte de `is_config_thin` ne l'affecte
pas — le body décrit ce que la skill fait quand elle est invoquée,
pas quand elle est **recommandée**.

Le seul body à mettre à jour est `onboard.md`, qui décrivait comment
calculer la nudge (Read sur le YAML). La formule devient : « lis le
YAML, regarde s'il porte au moins une entrée dans `rules:` — sinon,
c'est thin ».

### Décision : les tests unitaires deviennent binaires

Les 3 tests `is_config_thin_*` sont réécrits en 2 :

- `is_config_thin_vrai_quand_rules_vides` — assertion : la fonction
  retourne `true` pour `true`.
- `is_config_thin_faux_quand_rules_presentes` — assertion : la
  fonction retourne `false` pour `false`.

Suppression du test sur le seuil de 200 caractères — il ne
correspond plus à rien.

## Risques et compromis

- **Nudge « permanent » sur un utilisateur qui refuse `configure`**
  — quelqu'un qui ne veut pas de la skill verra la nudge à chaque
  `codev init` / `codev status`. → **Atténuation** : deux voies pour
  la faire disparaître — soit lancer `configure` une fois (rules
  écrites), soit ajouter à la main dans le YAML une entrée `rules:
  {}` de type « je sais ce que je fais ». La deuxième option n'est
  pas documentée mais fonctionne mécaniquement.
- **Divergence entre l'ancien seuil documenté et le nouveau
  comportement** — les personnes qui ont lu `docs/codev.md`
  s'attendent à un seuil de 200. → **Atténuation** : la
  documentation est à mettre à jour en même temps.

## Plan de migration

Aucune. Le fix change juste le comportement observable de la nudge
(elle apparaît plus souvent). Aucun format de fichier ne change.
