---
id: 0005
title: Sources héritées en lecture seule, épinglées par SHA — et non des stores
status: accepted
date: 2026-09-08
tags: [partage, securite]
---

## Contexte

Plusieurs projets doivent partager des conventions, des specs et des décisions
d'architecture. OpenSpec répond à ce besoin par des *stores* : un dépôt de
planification autonome, enregistré à la main sur la machine, accessible en
lecture **et** en écriture via `--store <id>` sur chaque commande. Trois défauts
pour notre usage : il faut maintenir un checkout, l'outil ne synchronise jamais
(on peut donc lire du périmé sans le savoir), et le flag contamine la résolution
de racine de toutes les commandes.

## Décision

Un mécanisme distinct, **en lecture seule** : les sources héritées.

```yaml
inherits:
  - path: ~/codev/shared/back
  - git: git@github.com:mon-orga/codev-decisions.git
    ref: main
    subpath: shared/
```

- Précédence de fusion : `git` → `poste` → `projet`, le projet gagnant toujours.
- Le `ref` est résolu en SHA et **épinglé** dans `_codev/codev.lock`, versionné
  avec le projet. Rien n'est jamais lu depuis une branche flottante à
  l'exécution.
- Transport git : un dépôt *bare* en cache, alimenté par le binaire `git` via le
  port `ProcessRunner` (`ls-remote`, puis `fetch --depth 1
  --filter=blob:none`, puis `archive`). Jamais de copie de travail.
- La provenance de chaque bloc hérité est exposée dans
  `codev instructions --json`.

Trois garde-fous, parce que ce contenu **est injecté dans le prompt de
l'agent** — qui contrôle le dépôt partagé contrôle une partie des instructions
de Claude Code sur tous les projets héritants :

1. épinglage par SHA obligatoire ;
2. `codev sources update` est la seule commande qui déplace un pin, et elle
   affiche le diff du contenu hérité avant d'écrire ;
3. aucun contenu exécutable hérité — markdown et YAML déclaratif uniquement,
   jamais de hook ni de script.

## Conséquences

- `--store` n'existe sur aucune commande, et la résolution de racine reste « je
  remonte jusqu'à `_codev/` ».
- Lecture hors ligne dès que le SHA est en cache.
- Piloter le binaire `git` réutilise l'authentification existante de
  l'utilisateur (clé ssh, credential helper), donc GitHub privé, GitLab
  auto-hébergé et Azure DevOps fonctionnent sans gestion de token par forge.
- Coût accepté : dépendance au binaire `git` pour les sources distantes. C'est
  une implémentation derrière un port, remplaçable par `gix` plus tard.

## Alternatives écartées

- **Les stores d'OpenSpec.** Écrire dans un dépôt partagé depuis un projet
  brouille la responsabilité du contenu partagé, et le checkout à maintenir est
  précisément la corvée qu'on veut supprimer.
- **`git archive --remote` seul**, sans cache. GitHub désactive `upload-archive`
  côté serveur, donc ça ne marche pas là où on en a besoin.
- **Les API de forge** (`/repos/.../contents`). Vendor-specific, et il faudrait
  implémenter une gestion de jeton par forge alors que git le fait déjà.
