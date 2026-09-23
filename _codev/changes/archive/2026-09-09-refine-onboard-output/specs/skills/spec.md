## MODIFIED Requirements

### Requirement: Skill `onboard` présente codev et recommande la prochaine action

Le catalogue de codev SHALL exposer un workflow `onboard` — installé
sous `.claude/skills/codev-onboard/SKILL.md`, invocable
`/codev-onboard` — dont le rôle est de présenter codev à un utilisateur
qui le découvre, en trois blocs :

1. Une description courte de codev (deux ou trois phrases).
2. L'état courant du projet — dépôt initialisé ou non, nombre de specs
   principales, nombre de décisions locales indexées, changes actifs
   listés par nom, et **nombre de changes archivés** (affiché
   uniquement s'il est non nul, pour ne pas polluer la sortie sur un
   projet neuf).
3. La prochaine action recommandée, adaptée à l'état :
   - `_codev/` absent → `codev init`.
   - Projet initialisé, aucun change → **inviter à lire `README.md`
     pour prendre le pouls du projet**, puis `/codev-propose <idée>` ;
     `/codev-explore <sujet>` reste mentionné comme alternative si
     l'utilisateur a une question mais pas encore d'idée d'action.
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

#### Scenario: Bloc « ici, tu as » mentionne les archivés quand il y en a

- **GIVEN** un projet contenant au moins un change dans
  `_codev/changes/archive/`
- **WHEN** l'utilisateur lance `/codev-onboard`
- **THEN** le bloc « ici, tu as » contient une ligne indiquant le
  nombre de changes archivés
- **AND** ce nombre correspond au nombre de dossiers de la forme
  `<date>-<nom>/` sous `_codev/changes/archive/`

#### Scenario: Bloc « ici, tu as » n'ajoute pas de ligne archivée sur projet neuf

- **GIVEN** un projet fraîchement initialisé, sans aucun change
  archivé
- **WHEN** l'utilisateur lance `/codev-onboard`
- **THEN** le bloc « ici, tu as » **n'affiche pas** de ligne
  « changes archivés » — la sortie reste courte et non polluée

#### Scenario: Recommandation par défaut cite README.md

- **GIVEN** un projet initialisé sans aucun change actif
- **WHEN** l'utilisateur lance `/codev-onboard`
- **THEN** le bloc « la suite » invite à lire `README.md` avant de
  créer un change
- **AND** cite en actionable `/codev-propose <une-idée>` et mentionne
  `/codev-explore <sujet>` comme alternative
