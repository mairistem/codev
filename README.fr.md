<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="docs/assets/compagnon-logo-dark.svg">
    <img src="docs/assets/compagnon-logo.svg" alt="Compagnon" width="420">
  </picture>
</p>

# Compagnon

**Le développement piloté par les specs pour Claude Code.**

[![CI](https://github.com/mairistem/compagnon/actions/workflows/ci.yml/badge.svg)](https://github.com/mairistem/compagnon/actions/workflows/ci.yml)
[![Latest release](https://img.shields.io/github/v/release/mairistem/compagnon)](https://github.com/mairistem/compagnon/releases/latest)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Platforms](https://img.shields.io/badge/platforms-macOS%20%7C%20Linux%20%7C%20Windows-lightgrey)](https://github.com/mairistem/compagnon/releases/latest)

[English](README.md) · Français

Compagnon ajoute une fine couche de spécifications à votre dépôt, pour que vous et
Claude Code vous accordiez sur ce qu'il faut construire avant d'écrire la
moindre ligne de code — et pour que les décisions d'architecture, une fois
prises, cessent d'être remises en débat à chaque change.

<p align="center">
  <img src="docs/assets/demo.gif" alt="Enregistrement de terminal : compagnon init, un change dont le plan est validé puis archivé, et la spec vivante qui en résulte" width="800">
</p>

## Pourquoi

Sans specs, l'intention se perd dans l'historique des conversations et dans la
tête des gens. Avec Compagnon, chaque change commence par un petit plan dans
`_compagnon/changes/` : le pourquoi, le quoi, le comment, et un **delta de spec**
qui énonce le comportement ajouté ou modifié.

Un change ajoute une exigence :

```markdown
## ADDED Requirements

### Requirement: Le thème choisi est mémorisé

L'application SHALL restaurer le dernier thème choisi par l'utilisateur, sur chacun des appareils depuis lesquels il se connecte.

#### Scenario: Utilisateur de retour

- **GIVEN** un utilisateur qui a choisi le thème sombre sur son ordinateur portable
- **WHEN** il se connecte depuis son téléphone
- **THEN** l'application s'affiche avec le thème sombre
```

À l'archivage du change, Compagnon fusionne le delta dans la spec vivante,
`_compagnon/specs/ui/theme/spec.md`, sans toucher au reste de son contenu :

```markdown
# Theme Specification

## Purpose

Permettre aux utilisateurs de choisir le thème de couleurs de l'interface.

## Requirements

### Requirement: Le thème suit par défaut la préférence du système

L'application SHALL s'afficher avec le jeu de couleurs du système d'exploitation tant que l'utilisateur n'a pas choisi de thème.

#### Scenario: Aucun thème encore choisi
…

### Requirement: Le thème choisi est mémorisé

L'application SHALL restaurer le dernier thème choisi par l'utilisateur, sur chacun des appareils depuis lesquels il se connecte.

#### Scenario: Utilisateur de retour
…
```

La spec décrit toujours ce que fait le système aujourd'hui, et chaque change
archivé garde la trace de ce qui l'a fait évoluer. Compagnon n'appelle jamais
lui-même de modèle : Claude Code rédige, et Compagnon lui indique quoi écrire, où,
et sous quelles contraintes.

## Premiers pas

Installez Compagnon — aucune chaîne d'outils Rust n'est nécessaire.

macOS et Linux :

```bash
curl -sSL https://raw.githubusercontent.com/mairistem/compagnon/main/install.sh | sh
```

Windows (PowerShell) :

```powershell
iwr -useb https://raw.githubusercontent.com/mairistem/compagnon/main/install.ps1 | iex
```

Les deux scripts vérifient l'empreinte SHA-256 du binaire qu'ils téléchargent.
Vous pouvez aussi télécharger un binaire depuis la
[page des releases](https://github.com/mairistem/compagnon/releases/latest), ou
compiler depuis les sources avec `cargo install --path crates/compagnon-cli` ;
voir le [guide d'installation](https://mairistem.github.io/compagnon/fr/installation.html).

**Prérequis :** macOS (Apple Silicon ou Intel), Linux x86_64 ou Windows
x86_64, et [Claude Code](https://claude.com/claude-code) pour utiliser les
skills.

### Démarrage rapide

```bash
cd your-project
compagnon init
```

Puis, dans Claude Code :

```text
/compagnon-propose add a dark mode that follows the system preference
/compagnon-apply add-dark-mode
/compagnon-archive add-dark-mode
```

Le [démarrage rapide](https://mairistem.github.io/compagnon/fr/quickstart.html)
détaille chaque étape.

## Fonctionnement

Chaque change suit le même cycle, avec une skill Claude Code par étape :

```text
┌───────────┐    ┌───────────┐    ┌───────────┐    ┌───────────┐
│  propose  │───▶│   apply   │───▶│   sync    │───▶│  archive  │
│ planifier │    │ construire│    │ fusionner │    │  classer  │
│           │    │           │    │ les deltas│    │           │
└───────────┘    └───────────┘    └───────────┘    └───────────┘
```

- **propose** — rédige le plan : proposal, deltas de specs, design, tâches.
  Aucun code.
- **apply** — implémente les tâches, lance leur vérification et les coche.
- **sync** — fusionne les deltas dans les specs principales (facultatif :
  l'archivage s'en charge).
- **archive** — valide le change, le fusionne et le déplace dans une archive
  datée.

`/compagnon-explore`, `/compagnon-update`, `/compagnon-onboard` et `/compagnon-configure`
complètent l'ensemble. Derrière les skills, la CLI `compagnon` effectue le travail
unitaire — état, instructions, validation, fusion — avec une sortie JSON
stable.

Tout se trouve dans un dossier visible à la racine de votre dépôt :

```text
_compagnon/
├── specs/        # contrats de comportement — ce que fait le système
├── decisions/    # décisions d'architecture — pourquoi il est construit ainsi
├── changes/      # travail en cours, et l'archive datée
└── config.yaml   # contexte, règles, langue, sources héritées
```

- **Les changes sont des deltas.** Un change déclare des exigences `ADDED`,
  `MODIFIED`, `REMOVED` ou `RENAMED` au lieu de réécrire les specs : Compagnon
  fonctionne ainsi sur du code existant, et pas seulement sur des projets
  neufs.
- **Les décisions sont immuables.** Les ADR acceptés sont scellés par une
  empreinte ; pour en changer un, on le remplace, et `compagnon validate` détecte
  les modifications silencieuses.
- **Les conventions se partagent.** Un projet peut hériter du contexte, des
  règles et des décisions d'un autre dépôt, en lecture seule et épinglés sur un
  commit git.
- **Votre langue.** Les artefacts sont rédigés dans la langue choisie par votre
  équipe ; leur structure reste lisible par la machine.

## Développé avec Compagnon

Compagnon est développé avec Compagnon. Chaque évolution de ce dépôt — y compris les
rôles, les experts et les comptes rendus de revue décrits plus haut — a été
planifiée, relue et archivée avec l'outil lui-même, et chaque pull request
passe `compagnon validate --strict`. Parcourez-le comme un exemple réel :

- [`_compagnon/specs/`](_compagnon/specs/) — les specs vivantes, qui décrivent ce que
  fait Compagnon aujourd'hui ;
- [`_compagnon/changes/archive/`](_compagnon/changes/archive/) — chaque change, avec
  son proposal, ses deltas de spec, son design et ses tâches ;
- [un compte rendu de revue](_compagnon/changes/archive/2026-10-13-review-record/review.md) — les remarques avec leurs preuves, les
  réponses de l'auteur et la façon dont chacune a été tranchée ;
- [`_compagnon/decisions/`](_compagnon/decisions/) — les décisions d'architecture
  scellées.

## Comparaison

Compagnon fait partie des nombreux outils de développement piloté par les specs
destinés aux agents de code. Voici une synthèse des fonctionnalités
documentées, en septembre 2026 :

| | Compagnon | [OpenSpec](https://github.com/Fission-AI/OpenSpec) | [Spec Kit](https://github.com/github/spec-kit) |
|---|---|---|---|
| Distribution | Binaire natif unique | Paquet npm (Node.js) | CLI Python (installée avec uv) |
| Agents pris en charge | Claude Code | Plus de 30 outils | De nombreux agents, dont GitHub Copilot |
| Deltas de specs fusionnés dans des specs vivantes | Oui | Oui | Non — des artefacts par fonctionnalité |
| Décisions d'architecture (ADR) | Oui, scellées contre les modifications | Non | Non — une constitution de projet à la place |
| Partage entre dépôts | Sources en lecture seule, épinglées par commit git | Stores (bêta) | — |
| Licence | MIT | MIT | MIT |

Compagnon s'appuie directement sur les idées d'OpenSpec : le cycle
propose → apply → archive, le change comme unité de travail, et les opérations
de delta `ADDED` / `MODIFIED` / `REMOVED` / `RENAMED` viennent toutes
d'OpenSpec. Si vous avez besoin d'un autre agent que Claude Code, OpenSpec et
Spec Kit vous conviendront mieux. Si une information de ce tableau n'est plus à
jour, merci d'ouvrir une issue.

## Documentation

La documentation complète se trouve sur
**[mairistem.github.io/compagnon](https://mairistem.github.io/compagnon/fr/)** —
workflow, concepts, guides, et références de la CLI, de la configuration et
des formats de fichiers. Elle est aussi embarquée dans le binaire et
consultable hors ligne :

```bash
compagnon docs --lang fr
```

## Contribuer

Les contributions sont les bienvenues. Compagnon est développé avec Compagnon : les
petites corrections passent par une pull request classique, et tout le reste
commence par un change Compagnon. Consultez [CONTRIBUTING.fr.md](CONTRIBUTING.fr.md),
ainsi que la [feuille de route](ROADMAP.md) (en anglais) pour ce qui est prévu.

Merci de signaler les problèmes de sécurité en privé, comme indiqué dans
[SECURITY.md](SECURITY.md) (en anglais). Ce projet applique un
[code de conduite](CODE_OF_CONDUCT.md) (en anglais).

## Licence

Compagnon est distribué sous [licence MIT](LICENSE).

Compagnon s'inspire d'[OpenSpec](https://github.com/Fission-AI/OpenSpec), de
Fission AI, également distribué sous licence MIT.
