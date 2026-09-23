---
id: 0001
title: Cœur fonctionnel, coquille impérative — décider n'est pas exécuter
status: accepted
date: 2026-09-08
tags: [architecture]
---

## Contexte

`archive` est l'opération la plus dangereuse de l'outil : elle réécrit les specs
principales, supprime éventuellement un fichier, puis déplace un dossier. Chez
OpenSpec, `archive.ts` fait 2 129 lignes, en grande partie parce que la décision
et l'écriture sont entremêlées — d'où le code de réservation de destination, de
rollback et de copie de secours vérifiée, tous nés du fait qu'une écriture peut
échouer alors que d'autres ont déjà eu lieu.

## Décision

Le cœur ne touche jamais au disque. Toute opération se décompose en deux temps :

1. une fonction **pure** qui produit un plan d'effets complet et sérialisable
   (`ArchivePlan`, `ScaffoldPlan`, `SyncPlan`) ;
2. une fonction d'**exécution** dans la coquille, bête et transactionnelle, seule
   à écrire.

Corollaire imposé : `codev-core` ne dépend ni de `std::fs`, ni de `std::env`, ni
d'une horloge, ni du réseau. Les effets passent par les ports décrits dans la
décision [0002](0002-graphe-de-crates-comme-regle-de-dependance.md).

## Conséquences

- `--dry-run` et la prévisualisation `--json` sont gratuits : ce sont le plan,
  rendu.
- L'atomicité devient structurelle : le plan est validé **entièrement** avant la
  première écriture, il n'y a donc plus d'état intermédiaire à rattraper.
- La fusion de specs se teste sans répertoire temporaire, ce qui rend les golden
  tests praticables — et c'est la seule protection réelle contre une réécriture
  markdown destructrice.
- Coût accepté : deux fonctions au lieu d'une, et un type de plan par opération.

## Alternatives écartées

- **Clean Architecture en couches** (`domain` / `application` /
  `infrastructure`, pattern Repository sur le système de fichiers, structs
  `*UseCase`). Elle rentabilise son coût quand l'infrastructure est volatile —
  changement de base, d'ORM, d'API tierce. Ici l'infrastructure est le système
  de fichiers, et on n'en changera pas. On aurait payé la cérémonie sans
  l'acheter.
- **Écriture directe avec rollback**, l'approche d'OpenSpec. Elle fonctionne,
  mais chaque nouveau cas d'échec ajoute un chemin de récupération, et aucun de
  ces chemins n'est testable sans provoquer une vraie panne d'écriture.
