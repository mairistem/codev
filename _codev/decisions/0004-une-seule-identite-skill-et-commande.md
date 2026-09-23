---
id: 0004
title: Une seule identité pour la skill et la slash command ; pas de fichiers de commandes
status: accepted
date: 2026-09-08
tags: [skills, claude-code]
---

## Contexte

OpenSpec livre deux formes pour chaque workflow : des skills nommées
`openspec-propose` et des fichiers de commandes invoqués `/opsx:propose`. Ce
double nommage est un accident historique, et il lui coûte une table de
correspondance dans chaque page de documentation, plus un sous-système entier de
génération de commandes avec un adaptateur par outil.

Dans Claude Code, une skill déposée dans `.claude/skills/<nom>/SKILL.md` est
déjà invocable par l'utilisateur en tapant `/<nom>`.

## Décision

Un seul artefact par workflow : `.claude/skills/codev-<workflow>/SKILL.md`,
invoqué `/codev-propose`. Aucun fichier de commande n'est généré.

L'axe `delivery` d'OpenSpec (`skills` / `commands` / `both`) n'existe pas.

## Conséquences

- Tout le sous-système de génération de commandes disparaît : registre,
  adaptateurs par outil, génération YAML, formes d'invocation. Plusieurs
  milliers de lignes en amont.
- Un seul nom à documenter, à taper et à chercher.
- Si un jour un outil ne sait lire que des fichiers de commandes, c'est une
  implémentation de `AgentTarget` de plus, pas une reprise du modèle.
- Coût accepté : on ne peut pas offrir un namespace `/codev:<verbe>` distinct
  des noms de skills. `codev-propose` reste lisible et se complète au clavier.

## Alternatives écartées

- **Générer les deux formes**, par symétrie avec OpenSpec. Redondant pour la
  seule cible retenue, et deux fichiers à garder cohérents pour un même
  workflow.
