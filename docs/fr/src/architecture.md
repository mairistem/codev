# Architecture

Ce chapitre s'adresse aux contributeurs : comment le code est organisé, et
pourquoi. Les décisions qui le sous-tendent, avec leur contexte et les
alternatives envisagées, sont consignées sous forme d'ADR dans
[`_codev/decisions/`](https://github.com/mairistem/codev/tree/main/_codev/decisions)
— codev est développé avec codev. Ce chapitre les résume et les relie entre
elles.

## Un cœur pur, une coquille impérative

codev suit un principe : **décider n'est pas exécuter**.

- Tout ce qui est pur vit dans `codev-core` : le modèle du domaine, les
  schémas, le graphe des artefacts, les analyseurs, les règles de validation,
  la fusion et le calcul des plans. Ni `std::fs`, ni `std::env`, ni horloge,
  ni réseau.
- Tout ce qui a besoin du monde extérieur vit au-dessus, derrière un port.

La règle de dépendance n'est pas une convention de dossiers : c'est le graphe
des crates, et Cargo refuse de compiler un cycle. Le compilateur la fait
respecter, sans interfaces de façade ni conteneur d'injection de dépendances.

## Le graphe des crates

```text
codev-cli ──▶ codev-agents ──▶ codev-engine ──▶ codev-core
    │                               ▲                ▲
    ├───────────────────────────────┘                │
    └────────────────────────────────────────────────┘
```

| Crate | Rôle | Entrées-sorties |
|---|---|---|
| `codev-core` | Modèle, schémas, graphe des artefacts, analyseurs, règles de validation, fusion, plans | Aucune |
| `codev-engine` | Organisation sur disque, configuration, résolution des schémas, index des décisions, sources héritées, exécution des plans | Via des ports |
| `codev-agents` | Le trait `AgentTarget` et son implémentation pour Claude Code ; le catalogue des workflows | Via des ports |
| `codev-cli` | Analyse des arguments, rendu lisible, contrat JSON versionné, questions interactives, documentation embarquée | Oui — c'est la coquille |

`codev-agents` dépend aussi de `codev-core`, et `codev-cli` de toutes les
crates.

Un module ne devient une crate que lorsqu'il l'a mérité — une frontière de
test nette, ou un consommateur externe. Découper plus tôt ne produit que des
refactorisations inutiles.

## Des plans, pas des effets de bord

Le cœur ne touche jamais au disque. Il renvoie un **plan** d'effets — fichiers
à créer ou à écraser, fichiers à supprimer, dossiers à déplacer — que la
coquille exécute. `init`, `new change`, `sync`, `archive`, l'installation des
skills et les commandes de décision fonctionnent tous ainsi.

On y gagne plusieurs choses à la fois : le plan complet est validé avant la
première écriture, les fusions se testent sans fichiers temporaires, et les
exécutions à blanc ou les aperçus viennent sans effort supplémentaire.

## Les ports

Il existe quatre ports, réservés aux effets :

| Port | Pourquoi |
|---|---|
| `FileSystem` | Rend `init`, `sync` et `archive` testables en mémoire |
| `Clock` | Les dates d'archivage et de décision doivent être déterministes dans les tests |
| `Env` | Répertoire personnel, `XDG_CACHE_HOME`, variables de locale |
| `ProcessRunner` | Pilotage de `git` pour les sources git héritées |

Ce que codev s'interdit : pas de trait de dépôt masquant `std::fs`, pas de
structures `*UseCase` à méthode unique, pas d'`Arc<dyn Trait>` par défaut. Les
traits servent les frontières réellement ouvertes (`AgentTarget`, `Rule`), les
génériques servent le reste, et les enums modélisent les ensembles fermés.

## Les deux risques dont l'architecture protège

1. **Une réécriture destructrice du Markdown.** Un delta `MODIFIED` qui
   reformate le fichier ou perd du contenu qu'il ne mentionne pas. Les
   analyseurs conservent donc les plages d'octets d'origine de chaque bloc,
   afin que la fusion réécrive un bloc sans toucher au reste.
2. **La dérive du contrat JSON.** La sortie JSON est une API publique,
   consommée par des skills déjà installées sur les machines des
   utilisateurs. Les types du domaine ne dérivent donc pas `Serialize` :
   `codev-cli` dispose d'un module `contract` dédié, dont les types traduisent
   le domaine, avec des tests qui figent les noms des champs. C'est la seule
   frontière DTO du projet, et elle a une raison d'être.

## Points d'extension

1. **`AgentTarget`** — une implémentation par outil d'agent. Claude Code
   aujourd'hui ; un autre outil peut être ajouté sans toucher au cœur.
2. **`Rule`** — une règle de validation par implémentation, enregistrée dans
   une liste.
3. **Schémas, templates et workflows sous forme de données** —
   `schema.yaml`, les templates et le corps des skills vivent dans `assets/`
   et sont embarqués à la compilation. Améliorer une instruction ne demande
   jamais de toucher au code Rust.

## Où se trouvent les choses

| Chemin | Contenu |
|---|---|
| `assets/schemas/spec-driven/` | Le schéma intégré et ses templates |
| `assets/templates/decision.md` | Le template d'ADR |
| `assets/workflows/*.md` | Le corps de chaque skill |
| `crates/codev-agents/src/workflows.rs` | Le catalogue des workflows : descriptions, `allowed-tools`, valeurs par défaut |
| `crates/codev-cli/src/contract.rs` | Le contrat JSON, version 1 |
| `crates/codev-cli/build.rs` | Embarque les chapitres de documentation listés dans `docs/<lang>/src/SUMMARY.md` |
| `_codev/` | Les specs, décisions et changes de codev lui-même |

## Décisions d'architecture

| ADR | Décision |
|---|---|
| 0001 | Cœur fonctionnel, coquille impérative : le cœur renvoie des plans, la coquille les exécute |
| 0002 | Le graphe des crates fait respecter la règle de dépendance ; quatre ports pour les effets |
| 0003 | La racine de planification est `_codev/`, un dossier visible |
| 0004 | Une identité unique pour une skill et sa commande slash ; pas de fichiers de commande séparés |
| 0005 | Les sources héritées sont en lecture seule et épinglées par commit, et non des dépôts partagés accessibles en écriture |
| 0006 | `serde_norway` pour le YAML, le fork maintenu de `serde_yaml` |

Les ADR rédigés avant la version 0.4 sont en français. Ce sont des
enregistrements scellés, conservés tels quels.

## Conventions de code

- **Erreurs.** `thiserror` et des erreurs typées dans les crates de
  bibliothèque ; `anyhow` uniquement dans `codev-cli`, où les erreurs sont
  traduites dans le tableau `status` du contrat JSON.
- **Commentaires.** Ils sont rédigés en anglais et expliquent le *pourquoi*.
  Un commentaire qui paraphrase le code n'est que du bruit.
- **Tests.** Le cœur est testé par des tests unitaires purs ; le moteur, avec
  un `FileSystem` en mémoire et des `Clock` et `Env` figés ; le contrat JSON,
  par des tests sur les noms et la forme de ses champs.
