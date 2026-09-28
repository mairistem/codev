## Purpose

Décrit la skill `/codev-configure` — la seule qui laisse Claude
enrichir `_codev/config.yaml` à partir d'une exploration en lecture
du projet. Elle intervient après `codev init` : la sonde du CLI a
rempli ce qu'elle pouvait (stack, MCPs), la skill couvre le reste
(conventions, style, choix structurants) que seul un LLM peut lire.

## ADDED Requirements

### Requirement: Skill `configure` enrichit `_codev/config.yaml` en analysant le projet

Le catalogue de codev SHALL exposer un workflow `configure` — installé
sous `.claude/skills/codev-configure/SKILL.md`, invocable
`/codev-configure` — dont le rôle est d'enrichir le `_codev/config.yaml`
en analysant le projet, sans modifier ni les workflows, ni les MCPs
détectés, ni le schéma.

Le fonctionnement MUST être :

1. **Lecture du fichier existant** — `_codev/config.yaml` du projet.
2. **Exploration en lecture pure** — `README.md` racine,
   `CONTRIBUTING.md` s'il existe, contenu de `docs/`, échantillon
   des fichiers source les plus édités (via `git log --pretty=format: --name-only | sort | uniq -c | sort -rn | head`),
   structure des dossiers principaux.
3. **Proposition de patch** — nouvelle valeur pour `context:` (2 à 5
   lignes ciblées sur ce qui compte pour piloter les skills — stack
   au-delà du langage, conventions d'API, style d'erreur, ton des
   commentaires, choix de bibliothèques structurants) et pour
   `rules:` par artefact (`specs:`, `design:`, `tasks:` — 1 à 2
   règles chacune, ancrées dans ce que le projet fait).
4. **Affichage du diff** en sortie, sans écriture.
5. **Écriture sur confirmation** de l'utilisateur uniquement.

La skill MUST **préserver** les champs qu'elle ne touche pas — `schema`,
`workflows`, `mcp`, `inherits`, ainsi que les commentaires de
provenance existants. Elle ne réécrit pas la clé `mcp:` détectée par
`codev init`.

La skill MUST **refuser d'agir** si `_codev/config.yaml` est absent —
elle renvoie l'utilisateur vers `codev init`.

#### Scenario: Rôle documenté dans le catalogue

- **GIVEN** le catalogue de workflows codev
- **WHEN** on résout le workflow `configure`
- **THEN** son entrée existe (`find("configure").is_some()`)
- **AND** son `body` cite les cinq étapes (lecture, exploration,
  proposition, diff, confirmation)
- **AND** son `body` liste explicitement les champs préservés
  (`schema`, `workflows`, `mcp`, `inherits`)

#### Scenario: Confirmation obligatoire avant écriture

- **GIVEN** un projet avec `_codev/config.yaml` au contexte thin
- **WHEN** l'utilisateur lance `/codev-configure`
- **THEN** un diff de la modification est affiché
- **AND** le fichier n'est écrit qu'après confirmation explicite de
  l'utilisateur

#### Scenario: Refus si config absente

- **GIVEN** un projet sans `_codev/config.yaml`
- **WHEN** l'utilisateur lance `/codev-configure`
- **THEN** la skill refuse d'agir
- **AND** renvoie vers `codev init`
