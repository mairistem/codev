# Architecture

Ce chapitre s'adresse aux contributeurs : comment le code est organisé, et
pourquoi. Les décisions qui le sous-tendent, avec leur contexte et les
alternatives envisagées, sont consignées sous forme de décisions d'architecture (ADR) dans
[`_compagnon/decisions/`](https://github.com/mairistem/compagnon/tree/main/_compagnon/decisions)
— Compagnon est développé avec Compagnon. Ce chapitre les résume et les relie entre
elles.

## Un cœur pur, une coquille impérative

Compagnon suit un principe : **décider n'est pas exécuter**.

- Tout ce qui est pur vit dans `compagnon-core` : le modèle du domaine, les
  schémas, le graphe des artefacts, les analyseurs, les règles de validation,
  la fusion et le calcul des plans. Ni `std::fs`, ni `std::env`, ni horloge,
  ni réseau.
- Tout ce qui a besoin du monde extérieur vit au-dessus, derrière un port.

La règle de dépendance n'est pas une convention de dossiers : c'est le graphe
des modules (crates), et Cargo refuse de compiler un cycle. Le compilateur la fait
respecter, sans interfaces de façade ni conteneur d'injection de dépendances.

## Le graphe des modules

<figure class="diagram">
<div class="scroll">
<svg viewBox="0 0 760 330" role="img" aria-label="Le graphe des modules, une flèche signifiant dépend de : compagnon-cli, la coquille, dépend de compagnon-agents, compagnon-engine et compagnon-core ; compagnon-agents, la cible Claude Code, dépend de compagnon-engine et compagnon-core ; compagnon-engine, le disque via des ports, dépend de compagnon-core ; compagnon-core, pur, ne dépend d'aucun autre module ; compagnon-evals, l'évaluation des rôles, membre de l'espace de travail hors du binaire, dépend de compagnon-core seulement.">
<defs><marker id="dg-architecture-arrow" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" style="fill:var(--dg-line)"/></marker></defs>
<text x="8" y="22" font-size="13" style="fill:var(--dg-muted)">Une flèche signifie « dépend de »</text>
<rect x="8" y="120" width="150" height="58" rx="10" style="fill:var(--dg-box);stroke:var(--dg-line)" stroke-width="2"/>
<text x="83" y="144" text-anchor="middle" class="mono" font-size="14" font-weight="600" style="fill:var(--dg-ink)">compagnon-cli</text>
<text x="83" y="166" text-anchor="middle" font-size="12" style="fill:var(--dg-muted)">la coquille</text>
<rect x="206" y="120" width="150" height="58" rx="10" style="fill:var(--dg-box);stroke:var(--dg-line)" stroke-width="2"/>
<text x="281" y="144" text-anchor="middle" class="mono" font-size="14" font-weight="600" style="fill:var(--dg-ink)">compagnon-agents</text>
<text x="281" y="166" text-anchor="middle" font-size="12" style="fill:var(--dg-muted)">cible Claude Code</text>
<rect x="404" y="120" width="150" height="58" rx="10" style="fill:var(--dg-box);stroke:var(--dg-line)" stroke-width="2"/>
<text x="479" y="144" text-anchor="middle" class="mono" font-size="14" font-weight="600" style="fill:var(--dg-ink)">compagnon-engine</text>
<text x="479" y="166" text-anchor="middle" font-size="12" style="fill:var(--dg-muted)">disque, via des ports</text>
<rect x="602" y="120" width="150" height="58" rx="10" style="fill:var(--dg-box);stroke:var(--dg-line)" stroke-width="2"/>
<text x="677" y="144" text-anchor="middle" class="mono" font-size="14" font-weight="600" style="fill:var(--dg-ink)">compagnon-core</text>
<text x="677" y="166" text-anchor="middle" font-size="12" style="fill:var(--dg-muted)">pur, sans E/S</text>
<line x1="160" y1="149" x2="204" y2="149" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-architecture-arrow)"/>
<line x1="358" y1="149" x2="402" y2="149" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-architecture-arrow)"/>
<line x1="556" y1="149" x2="600" y2="149" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-architecture-arrow)"/>
<path d="M50,118 V44 H710 V116" fill="none" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-architecture-arrow)"/>
<path d="M110,118 V80 H479 V116" fill="none" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-architecture-arrow)"/>
<path d="M281,180 V216 H660 V182" fill="none" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-architecture-arrow)"/>
<rect x="602" y="256" width="150" height="58" rx="10" style="fill:var(--dg-file);stroke:var(--dg-line)" stroke-width="2" stroke-dasharray="6 4"/>
<text x="677" y="280" text-anchor="middle" class="mono" font-size="14" font-weight="600" style="fill:var(--dg-ink)">compagnon-evals</text>
<text x="677" y="302" text-anchor="middle" font-size="12" style="fill:var(--dg-muted)">évaluation des rôles</text>
<line x1="720" y1="254" x2="720" y2="182" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-architecture-arrow)"/>
<text x="588" y="280" text-anchor="end" font-size="12" style="fill:var(--dg-muted)">Membre de l'espace de travail,</text>
<text x="588" y="298" text-anchor="end" font-size="12" style="fill:var(--dg-muted)">jamais inclus dans le binaire</text>
</svg>
</div>
<figcaption>Le graphe des modules : toutes les flèches vont vers compagnon-core, qui ne dépend d'aucun autre module, et Cargo refuse de compiler un cycle.</figcaption>
</figure>

| Module | Rôle | Entrées-sorties |
|---|---|---|
| `compagnon-core` | Modèle, schémas, graphe des artefacts, analyseurs, règles de validation, fusion, plans | Aucune |
| `compagnon-engine` | Organisation sur disque, configuration, résolution des schémas, index des décisions, sources héritées, exécution des plans | Via des ports |
| `compagnon-agents` | L'interface (trait) `AgentTarget` et son implémentation pour Claude Code ; le catalogue des déroulés | Via des ports |
| `compagnon-cli` | Analyse des arguments, rendu lisible, contrat JSON versionné, questions interactives, documentation embarquée | Oui — c'est la coquille |

Le cinquième membre de l'espace de travail, `compagnon-evals`, note les rôles
de relecture (voir [Fiabilité des rôles](reliability.md)). Il ne dépend que de
`compagnon-core` et n'est jamais inclus dans le binaire publié.

Du code ne devient un module à part entière que lorsqu'il l'a mérité — une frontière de
test nette, ou un consommateur externe. Découper plus tôt ne produit que des
refactorisations inutiles.

## Des plans, pas des effets de bord

Le cœur ne touche jamais au disque. Il renvoie un **plan** d'effets — fichiers
à créer ou à écraser, fichiers à supprimer, dossiers à déplacer — que la
coquille exécute. `init`, `new change`, `sync`, `archive`, l'installation des
compétences et les commandes de décision fonctionnent tous ainsi.

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

Ce que Compagnon s'interdit : pas d'interface de dépôt masquant `std::fs`, pas de
structures `*UseCase` à méthode unique, pas d'`Arc<dyn Trait>` par défaut. Les
interfaces servent les frontières réellement ouvertes (`AgentTarget`, `Rule`), les
génériques servent le reste, et les énumérations (enums) modélisent les ensembles fermés.

## Les deux risques dont l'architecture protège

1. **Une réécriture destructrice du Markdown.** Un delta `MODIFIED` qui
   reformate le fichier ou perd du contenu qu'il ne mentionne pas. Les
   analyseurs conservent donc les plages d'octets d'origine de chaque bloc,
   afin que la fusion réécrive un bloc sans toucher au reste.
2. **La dérive du contrat JSON.** La sortie JSON est une API publique,
   consommée par des compétences déjà installées sur les machines des
   utilisateurs. Les types du domaine ne dérivent donc pas `Serialize` :
   `compagnon-cli` dispose d'un sous-module `contract` dédié, dont les types traduisent
   le domaine, avec des tests qui figent les noms des champs. C'est la seule
   frontière d'objets de transfert (DTO) du projet, et elle a une raison d'être.

## Points d'extension

1. **`AgentTarget`** — une implémentation par outil d'agent. Claude Code
   aujourd'hui ; un autre outil peut être ajouté sans toucher au cœur.
2. **`Rule`** — une règle de validation par implémentation, enregistrée dans
   une liste.
3. **Schémas, modèles et déroulés sous forme de données** —
   `schema.yaml`, les modèles et le corps des compétences vivent dans `assets/`
   et sont embarqués à la compilation. Améliorer une instruction ne demande
   jamais de toucher au code Rust.

## Où se trouvent les choses

| Chemin | Contenu |
|---|---|
| `assets/schemas/spec-driven/` | Le schéma intégré et ses modèles |
| `assets/templates/decision.md` | Le modèle d'ADR |
| `assets/workflows/*.md` | Le corps de chaque compétence |
| `crates/compagnon-agents/src/workflows.rs` | Le catalogue des déroulés : descriptions, `allowed-tools`, valeurs par défaut |
| `crates/compagnon-cli/src/contract.rs` | Le contrat JSON, version 1 |
| `crates/compagnon-cli/build.rs` | Embarque les chapitres de documentation listés dans `docs/<lang>/src/SUMMARY.md` |
| `_compagnon/` | Les spécifications, décisions et modifications de Compagnon lui-même |

## Décisions d'architecture

| Décision d'architecture | Décision |
|---|---|
| 0001 | Cœur fonctionnel, coquille impérative : le cœur renvoie des plans, la coquille les exécute |
| 0002 | Le graphe des modules fait respecter la règle de dépendance ; quatre ports pour les effets |
| 0003 | La racine de planification est `_compagnon/`, un dossier visible |
| 0004 | Une identité unique pour une compétence et sa commande `/…` ; pas de fichiers de commande séparés |
| 0005 | Les sources héritées sont en lecture seule et épinglées par enregistrement (commit), et non des dépôts partagés accessibles en écriture |
| 0006 | `serde_norway` pour le YAML, la copie dérivée (fork) maintenue de `serde_yaml` |

## Conventions de code

- **Erreurs.** `thiserror` et des erreurs typées dans les modules de
  bibliothèque ; `anyhow` uniquement dans `compagnon-cli`, où les erreurs sont
  traduites dans le tableau `status` du contrat JSON.
- **Commentaires.** Ils sont rédigés en anglais et expliquent le *pourquoi*.
  Un commentaire qui paraphrase le code n'est que du bruit.
- **Tests.** Le cœur est testé par des tests unitaires purs ; le moteur, avec
  un `FileSystem` en mémoire et des `Clock` et `Env` figés ; le contrat JSON,
  par des tests sur les noms et la forme de ses champs.
