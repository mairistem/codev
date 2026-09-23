# Proposal : parseur de specs et de deltas

## Pourquoi

Aujourd'hui codev peut créer et lister des changes, mais rien ne sait lire une
spec ou un delta écrit à la main. Les trois commandes du lot 1 encore
manquantes — `validate`, `sync`, `archive` — en dépendent toutes.

## Ce qui change

- **Nouveau parseur de spec principale** : `## Purpose`, `## Requirements`,
  `### Requirement: <nom>`, `#### Scenario: <nom>` avec ses lignes
  **WHEN** / **THEN** / **AND**.
- **Nouveau parseur de delta** : `## ADDED Requirements`,
  `## MODIFIED Requirements`, `## REMOVED Requirements` (avec `**Raison**` /
  `**Migration**`), `## RENAMED Requirements` (`FROM:` / `TO:`).
- **Masquage des code fences et des commentaires HTML** partagé par les deux :
  un ` ``` `-block contenant `### Requirement:` ne doit pas produire une fausse
  exigence — le piège classique et silencieux.
- **AST à spans** : chaque bloc conserve son intervalle `[début, fin)` dans le
  texte source. C'est la brique qui rendra la fusion `MODIFIED` non
  destructive : réécrire un bloc sans reformater le reste du fichier.
- API pure dans `codev-core` : la fonction prend une `&str`, rend un AST
  typé, ne touche pas au disque.

## Capacités

### Nouvelles capacités

- `spec-parsing`

### Capacités modifiées

Aucune — le projet n'a encore aucune spec principale.

## Impact

- **Code** : nouveau module `codev-core::parser` (`spec.rs`, `delta.rs`,
  `fence.rs`, `ast.rs`). Aucune modification de `codev-engine`, `codev-agents`
  ou `codev-cli` dans ce change — les consommateurs (`validate`, `sync`,
  `archive`) feront l'objet de changes séparés.
- **Dépendances** : le design tranchera entre un parseur maison ligne à ligne
  (l'approche d'OpenSpec, ~1 200 lignes pour l'ensemble) et une dépendance
  markdown existante. Rien n'est engagé ici.
- **Hors périmètre** : parseur de `tasks.md` (activé quand `validate --archived`
  sera implémenté) et parseur de `proposal.md` (activé quand la validation de
  proposal en aura besoin). Ces deux artefacts servent aujourd'hui à l'humain
  et à l'agent, pas à codev, et leur parsing n'est demandé par aucune commande
  du lot 1.
