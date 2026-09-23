# Shell Completions Specification

## Purpose

Livrer un mécanisme pour que l'utilisateur active la complétion des
commandes `codev` dans son shell, sans avoir à écrire ni maintenir de
script à la main — l'outil génère le script à partir de la déclaration
`clap` existante.

## Requirements

### Requirement: `codev completions <shell>` imprime un script de complétion sur stdout

Le binaire `codev` SHALL exposer une sous-commande `completions
<shell>` où `<shell>` est l'une des cinq valeurs standard de
`clap_complete` : `bash`, `zsh`, `fish`, `powershell`, `elvish`.

La commande MUST :

- écrire le script sur **stdout uniquement** — aucun fichier système
  n'est touché ;
- retourner exit code **0** en cas de succès ;
- refuser avec exit code non nul et un message clair si `<shell>`
  n'est pas l'une des cinq valeurs reconnues.

Le script MUST correspondre à la structure courante des commandes
codev — chaque nouvelle sous-commande ajoutée à `Cli` (par exemple
`decision promote`, `decision deviate`) apparaît dans le script sans
intervention manuelle.

#### Scenario: Génération zsh non vide et cite codev

- **GIVEN** un binaire `codev` de la version courante
- **WHEN** l'utilisateur lance `codev completions zsh`
- **THEN** stdout porte une sortie non vide (≥ 500 caractères)
- **AND** cette sortie contient au moins une fois le nom `codev`
- **AND** le code de retour est 0

#### Scenario: Support des cinq shells de clap_complete

- **GIVEN** un binaire `codev` de la version courante
- **WHEN** l'utilisateur lance successivement `codev completions bash`,
  `codev completions zsh`, `codev completions fish`,
  `codev completions powershell`, `codev completions elvish`
- **THEN** chaque appel produit une sortie non vide sur stdout
- **AND** chaque appel retourne exit code 0

#### Scenario: Shell inconnu refusé

- **GIVEN** un binaire `codev` de la version courante
- **WHEN** l'utilisateur lance `codev completions nushell`
- **THEN** aucun script n'est imprimé sur stdout
- **AND** un message d'erreur nomme `nushell` et rappelle la liste
  des shells reconnus
- **AND** le code de retour est non nul

### Requirement: La commande `completions` est purement lecture

`codev completions <shell>` MUST NOT écrire dans le système de
fichiers, ni contacter le réseau, ni lire `_codev/config.yaml` ou
tout autre état du projet. Elle est indépendante d'un dépôt
initialisé : elle fonctionne dans n'importe quel répertoire courant,
y compris hors de toute racine codev.

#### Scenario: Fonctionne hors d'un dépôt codev

- **GIVEN** un utilisateur dans un répertoire qui n'a pas de `_codev/`
- **WHEN** il lance `codev completions bash`
- **THEN** le script est imprimé normalement
- **AND** aucun message d'erreur ne mentionne `_codev`
