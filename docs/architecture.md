# Architecture

Ce document dit **comment le code est organisé et pourquoi**. Les décisions
elles-mêmes, avec leur contexte et leurs alternatives écartées, vivent dans
[`_codev/decisions/`](../_codev/decisions/) — ce document les résume et les
relie.

## Le parti pris central

Ni couches à la Clean Architecture, ni fourre-tout : **cœur fonctionnel,
coquille impérative**.

- Tout ce qui est **pur** vit dans `codev-core` : modèle de domaine, schémas,
  graphe d'artefacts, règles de validation, calcul des plans. Aucun
  `std::fs`, aucun `std::env`, aucune horloge, aucun réseau.
- Tout ce qui a besoin du **monde extérieur** vit au-dessus, derrière un port.

La règle de dépendance n'est pas une convention de dossiers : c'est le graphe de
crates, et **Cargo refuse de compiler un cycle**. Elle est donc vérifiée par le
compilateur, sans interfaces de façade ni container d'injection.

## Le graphe de crates

```
codev-cli  ──►  codev-agents  ──►  codev-engine  ──►  codev-core
   │                                                      ▲
   └──────────────────────────────────────────────────────┘
```

| Crate | Rôle | I/O |
|---|---|---|
| `codev-core` | Modèle, schémas, graphe d'artefacts, règles, plans | **aucune** |
| `codev-engine` | Disposition disque, config, résolution de schémas, exécution des plans | via ports |
| `codev-agents` | `trait AgentTarget` + implémentation Claude Code | via ports |
| `codev-cli` | clap, rendu humain, contrat JSON versionné | oui — c'est la coquille |

**Règle de promotion.** Un module ne devient un crate que lorsqu'il l'a mérité :
un cycle de tests propre, ou un consommateur externe. `validation` et `archive`
restent des modules de `codev-engine` jusque-là. Découper trop tôt ne produit
que du churn de refactor.

## Décider n'est pas exécuter

Le cœur ne touche jamais au disque. Il retourne un **plan d'effets** que la
coquille exécute :

```rust
// pur, testable, sérialisable
pub fn plan_archive(change: &Change, specs: &SpecIndex, cfg: &Config)
    -> Result<ArchivePlan, ArchiveError>;

// bête, transactionnel, seul endroit qui écrit
pub fn execute(plan: &ArchivePlan, fs: &impl FileSystem) -> Result<(), IoError>;
```

Une décision, quatre bénéfices : `--dry-run` gratuit, prévisualisation `--json`
gratuite, atomicité (le plan est validé **entièrement** avant la première
écriture), et des tests de fusion sans aucun fichier temporaire.

Le même schéma s'applique à `init` (`ScaffoldPlan`), `new change`, `sync` et
`archive`.

## Les ports

Quatre, pas plus, et uniquement pour des effets :

| Port | Pourquoi |
|---|---|
| `FileSystem` | rend `init`/`archive` testables en mémoire, sans tmpdir |
| `Clock` | les dates d'archive et de décision doivent être déterministes en test |
| `Env` | résolution des chemins, variables `CODEV_*` |
| `ProcessRunner` | pilotage de `git` pour les sources héritées distantes |

Ce qu'on **ne** fait pas : pas de `trait SpecRepository` masquant `std::fs`, pas
de structs `*UseCase` à une méthode, pas de `Arc<dyn Trait>` par défaut. Les
traits servent aux frontières réellement ouvertes (`AgentTarget`, `Rule`), les
génériques ailleurs, et les `enum` pour les ensembles fermés.

## Les deux risques que l'architecture protège

Aucune couche ne protège de ceci ; un cœur pur et des golden tests, si.

1. **Une réécriture markdown destructrice.** Un `MODIFIED` qui reformate le
   fichier ou perd du contenu non mentionné. D'où : un AST de blocs qui conserve
   les spans d'origine, pour réécrire un bloc sans toucher au reste.
2. **Une dérive du contrat JSON.** Le JSON de sortie est une API publique,
   consommée par des skills déjà installées chez les utilisateurs. Il ne
   `derive(Serialize)` donc **pas** sur les types du domaine : `codev-cli`
   porte un module `contract::v1` figé, testé par snapshots. C'est la seule
   frontière de DTO du projet, et elle a une vraie raison d'être.

## Les points d'extension

Trois, qui portent l'évolutivité annoncée :

1. **`trait AgentTarget`** — un outil = une implémentation. Claude Code
   aujourd'hui ; Cursor ou une cible générique `.agents/` sans toucher au cœur.
2. **`trait Rule`** — une règle de validation = une implémentation, enregistrée
   dans un registre.
3. **Schémas et templates externes** — les workflows et les prompts restent des
   données, jamais du code. C'est la leçon d'OPSX contre le workflow historique
   d'OpenSpec : quand les instructions sont enfouies dans le binaire, personne
   ne peut les améliorer sans une release.

## Conventions

- **Erreurs** : `thiserror` et des erreurs typées dans les crates de
  bibliothèque ; `anyhow` uniquement dans `codev-cli`, où elles sont traduites
  vers le tableau `status[]` du contrat JSON.
- **Commentaires** : en français, comme le reste du projet, et ils expliquent le
  *pourquoi*. Un commentaire qui paraphrase le code est du bruit.
- **Tests** : le cœur se teste en unitaire pur ; l'engine avec un `FileSystem`
  en mémoire ; le contrat JSON par snapshots.
