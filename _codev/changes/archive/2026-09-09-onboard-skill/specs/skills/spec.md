## ADDED Requirements

### Requirement: Skill `onboard` présente codev et recommande la prochaine action

Le catalogue de codev SHALL exposer un workflow `onboard` — installé
sous `.claude/skills/codev-onboard/SKILL.md`, invocable
`/codev-onboard` — dont le rôle est de présenter codev à un utilisateur
qui le découvre, en trois blocs :

1. Une description courte de codev (deux ou trois phrases).
2. L'état courant du projet — dépôt initialisé ou non, nombre de specs
   principales, nombre de décisions locales indexées, changes actifs
   listés par nom.
3. La prochaine action recommandée, adaptée à l'état :
   - `_codev/` absent → `codev init`.
   - Projet initialisé, aucun change → `/codev-propose <idée>`.
   - Un change actif dont la planification est incomplète →
     `/codev-propose <ce-change>` pour le poursuivre.
   - Un change actif dont la planification est complète →
     `/codev-apply <ce-change>`.
   - Plusieurs changes actifs → les lister et laisser l'utilisateur
     choisir.

La skill MUST être **strictement en lecture** : `allowed-tools` limité
à `Bash(codev:*), Read, Glob`. Ni `Write`, ni `Edit`, ni `Bash`
général.

#### Scenario: Rôle documenté dans le catalogue

- **GIVEN** le catalogue de workflows codev
- **WHEN** on résout le workflow `onboard`
- **THEN** son entrée existe (`find("onboard").is_some()`)
- **AND** son `allowed_tools` vaut exactement
  `"Bash(codev:*), Read, Glob"`
- **AND** son `allowed_tools` ne contient PAS `Bash` général (règle
  invariante : seule `apply` en dispose)
- **AND** son `body` cite les trois blocs (description, état, action
  recommandée)

#### Scenario: Skill installée par un `codev update`

- **GIVEN** un projet dont le `config.yaml` a `workflows: [propose,
  explore, apply, sync, archive, update, onboard]`
- **WHEN** l'utilisateur lance `codev update`
- **THEN** le fichier `.claude/skills/codev-onboard/SKILL.md` est
  créé
- **AND** son frontmatter YAML est valide et porte la description
  attendue

### Requirement: `onboard` fait partie du catalogue par défaut

Le tableau `DEFAULT_WORKFLOWS` de `codev-agents::workflows` MUST
contenir `onboard`, aux côtés de `propose` et `explore`. Un
utilisateur qui lance `codev init` sur un projet neuf, sans clef
`workflows:` dans son `config.yaml`, obtient donc `/codev-onboard`
disponible immédiatement.

Les autres workflows opt-in (`apply`, `sync`, `archive`, `update`)
restent hors du catalogue par défaut — leur inclusion demande une
déclaration explicite.

#### Scenario: Catalogue par défaut inclut onboard

- **GIVEN** un projet dont le `config.yaml` n'a pas de clef
  `workflows:`
- **WHEN** `select(None)` est appelé sur le catalogue
- **THEN** la liste des `id` retournés est exactement
  `["propose", "explore", "onboard"]`
- **AND** aucun warning n'est émis

#### Scenario: Autres opt-in restent opt-in

- **GIVEN** le même contexte
- **WHEN** on inspecte le catalogue par défaut
- **THEN** aucun de `["apply", "sync", "archive", "update"]` ne
  figure — leur inclusion demande toujours une déclaration explicite
