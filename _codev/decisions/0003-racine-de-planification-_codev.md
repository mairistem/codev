---
id: 0003
title: La racine de planification est `_codev/`, visible
status: accepted
date: 2026-09-08
tags: [ergonomie, disposition]
---

## Contexte

La racine de planification contient les specs, les décisions et les changes. Or
c'est le seul endroit du dépôt **fait pour être lu** : par un humain en revue,
et par l'agent. Trois noms étaient en jeu : `codev/`, `.codev/`, `_codev/`.

## Décision

`_codev/` à la racine du dépôt.

## Conséquences

- Visible pour `ripgrep`, `fd`, `ls`, et donc pour les outils de recherche de
  Claude Code, qui ignorent les dossiers cachés par défaut. C'est le point
  décisif : une source de vérité que l'agent ne peut pas trouver ne sert à rien.
- Le préfixe `_` la détache visuellement du code source et l'épingle en haut de
  la plupart des listings.
- Le nom est une **constante unique** dans `codev-core`. En changer coûte une
  recompilation, pas un refactor.
- Effet de bord bénin : certains générateurs de sites statiques (Jekyll, Hugo)
  excluent les dossiers préfixés par `_` de leur sortie — ce qui est ici le
  comportement souhaitable.

## Alternatives écartées

- **`.codev/`**, le choix initial. Écarté après avoir constaté que les outils de
  recherche ignorent les dossiers cachés par défaut : cacher la source de vérité
  à l'agent contredit l'objectif de l'outil.
- **`codev/`**, à la manière d'OpenSpec. Correct, mais se mélange visuellement
  aux dossiers de code (`crates/`, `docs/`).
