---
id: 0002
title: Le graphe de crates applique la règle de dépendance ; quatre ports pour les effets
status: accepted
date: 2026-09-08
tags: [architecture]
---

## Contexte

On veut la garantie que le domaine ne dépend pas de l'infrastructure. Dans les
écosystèmes où la Clean Architecture s'est popularisée (Java, C#), les couches
sont une convention de dossiers que rien n'applique : d'où les interfaces de
façade, les DTO, les mappers et le container d'injection, qui existent surtout
pour rendre la convention visible.

## Décision

La couche d'un module est **sa position dans le graphe de crates**, et Cargo
refuse de compiler un cycle. La règle de dépendance est donc vérifiée par le
compilateur :

```
codev-cli → codev-agents → codev-engine → codev-core
```

L'inversion de dépendance ne s'applique qu'aux **effets**, par quatre ports :
`FileSystem`, `Clock`, `Env`, `ProcessRunner`.

Les traits ne servent qu'aux frontières réellement ouvertes — `AgentTarget`
(un outil = une implémentation) et `Rule` (une règle de validation = une
implémentation). Ailleurs : des génériques, et des `enum` pour les ensembles
fermés (opérations de delta, sévérités, statuts).

Règle de promotion : un module ne devient un crate que lorsqu'il a son propre
cycle de tests lourd ou un consommateur externe.

## Conséquences

- Pas de dossiers `domain/application/infrastructure/` : ils réimplémenteraient
  à la main ce que le graphe de crates donne gratuitement.
- `init`, `update` et `archive` se testent avec un `FileSystem` en mémoire,
  sans répertoire temporaire ni tests sérialisés.
- Le nombre de crates démarre bas (quatre). Un découpage plus fin serait du
  churn de refactor tant que les frontières ne sont pas éprouvées.
- Coût accepté : les ports se propagent en paramètres génériques dans l'engine.

## Alternatives écartées

- **`trait SpecRepository` masquant le système de fichiers.** N'achète que de la
  testabilité, que les ports d'effets fournissent déjà, en ajoutant une
  abstraction de persistance là où il n'y a que des fichiers markdown.
- **`Arc<dyn Trait>` par défaut** pour tous les collaborateurs. Coût de
  dispatch dynamique et d'allocation pour de la souplesse dont on n'a besoin
  qu'à deux endroits.
