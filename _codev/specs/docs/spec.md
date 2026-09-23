# Docs Specification

## Purpose

Livrer une documentation complète de codev sous une forme **unique et
diffusable** : un fichier HTML autonome (CSS embarqué, pas
d'assets externes) que le binaire `codev` peut générer et ouvrir
lui-même, pour compléter l'aide CLI et servir de point de partage
avec des tiers.

## Requirements

### Requirement: `codev docs` génère un HTML autonome et l'ouvre dans le navigateur

Le binaire `codev` SHALL exposer une sous-commande `docs` qui, sans
argument, génère un fichier HTML autonome et l'ouvre dans le
navigateur système par défaut.

Le HTML MUST :

- porter tout son CSS **inline** dans un `<style>` de l'en-tête —
  aucune référence à un CDN, aucune image externe, aucun script
  runtime obligatoire ;
- rendre correctement les six éléments markdown courants — titres,
  paragraphes, listes, tableaux, blocs de code, citations ;
- porter en tête le nom `codev` et la version du binaire qui l'a
  généré ;
- s'ouvrir dans n'importe quel navigateur récent, y compris **hors
  ligne**.

Le chemin de sortie par défaut MUST être dans le répertoire système
temporaire (`std::env::temp_dir()`) et porter la version dans son
nom : `codev-docs-<version>.html`.

#### Scenario: `codev docs` sans argument ouvre le HTML

- **GIVEN** un binaire `codev` de version 0.1.0
- **WHEN** l'utilisateur lance `codev docs`
- **THEN** un fichier `codev-docs-0.1.0.html` est écrit dans
  `std::env::temp_dir()`
- **AND** un appel système ouvre ce fichier dans le navigateur par
  défaut (macOS `open`, Linux `xdg-open`, Windows `start`)
- **AND** le code de retour est 0
- **AND** le HTML porte le nom `codev` et la version `0.1.0` en tête

#### Scenario: Le HTML est autonome

- **GIVEN** le fichier généré à l'étape précédente
- **WHEN** on l'ouvre dans un navigateur sans connexion internet
- **THEN** la page rend correctement (CSS inline, pas de requête
  externe)
- **AND** aucun élément `<link>` ne pointe vers un `http(s):`

### Requirement: `codev docs --print` imprime le markdown source sur stdout

`codev docs --print` MUST imprimer le **markdown source** de la
documentation sur stdout, sans conversion. La sortie est utile pour
pipeliner vers `less`, `bat`, ou un outil qui consomme du markdown
(un LLM, par exemple).

Cette forme MUST NOT ouvrir de navigateur, MUST NOT écrire de
fichier, et MUST retourner exit code 0.

#### Scenario: `--print` sort du markdown non vide

- **GIVEN** le binaire courant
- **WHEN** l'utilisateur lance `codev docs --print`
- **THEN** stdout porte un contenu non vide (> 1000 octets — la doc
  fait plusieurs sections)
- **AND** le contenu commence par un titre de niveau 1
  markdown (`# codev`)
- **AND** aucun fichier n'est écrit

### Requirement: `codev docs --write <PATH>` écrit sans ouvrir

`codev docs --write <PATH>` MUST écrire le HTML autonome au chemin
donné et **ne rien ouvrir**. C'est le mode « diffusion ciblée » —
placer la doc dans un share drive, un dossier Confluence, un dépôt.

Le chemin est utilisé tel quel : la commande MUST créer les dossiers
parents manquants **uniquement** si le chemin donné a un parent qui
existe déjà (la commande n'invente pas d'arborescence — elle refuse
un chemin dont plusieurs niveaux manquent).

#### Scenario: `--write` produit le fichier au chemin donné

- **GIVEN** le binaire courant
- **AND** un répertoire de travail où `./out/` existe
- **WHEN** l'utilisateur lance `codev docs --write ./out/manuel.html`
- **THEN** le fichier `./out/manuel.html` existe et porte le HTML
  autonome
- **AND** aucun navigateur n'est ouvert
- **AND** le code de retour est 0

### Requirement: `codev docs` est indépendant d'un dépôt initialisé

`codev docs` MUST NOT lire `_codev/config.yaml` ni tout autre état
projet. La commande fonctionne dans n'importe quel répertoire, même
sans dépôt codev, même sans `.git`.

#### Scenario: Fonctionne hors d'un dépôt codev

- **GIVEN** un utilisateur dans un répertoire qui n'a pas de `_codev/`
- **WHEN** il lance `codev docs --print`
- **THEN** le markdown est imprimé normalement
- **AND** aucun message d'erreur ne mentionne `_codev`
