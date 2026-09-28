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
   - Projet initialisé, aucun change, **et `_codev/config.yaml`
     thin** (contexte < 200 caractères et `rules:` vides) →
     `/codev-configure` en premier, avec une phrase qui explique le
     bénéfice (« Claude enrichira ta config à partir du projet »),
     puis `/codev-propose <idée>` en second.
   - Projet initialisé, aucun change, config non-thin → **inviter à
     lire `README.md` pour prendre le pouls du projet**, puis
     `/codev-propose <idée>` ; `/codev-explore <sujet>` reste
     mentionné comme alternative.
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

#### Scenario: Recommandation configure quand la config est thin

- **GIVEN** un projet initialisé sans change actif, dont le
  `_codev/config.yaml` a un `context:` inférieur à 200 caractères et
  des `rules:` vides
- **WHEN** l'utilisateur lance `/codev-onboard`
- **THEN** le bloc « la suite » cite `/codev-configure` en premier,
  avec une phrase sur le bénéfice attendu
- **AND** mentionne `/codev-propose <idée>` en second

#### Scenario: Recommandation par défaut cite README.md quand la config n'est pas thin

- **GIVEN** un projet initialisé sans change actif dont le
  `_codev/config.yaml` a un `context:` ≥ 200 caractères OU des `rules:`
  non vides
- **WHEN** l'utilisateur lance `/codev-onboard`
- **THEN** le bloc « la suite » invite à lire `README.md` avant de
  créer un change
- **AND** cite en actionable `/codev-propose <une-idée>` et mentionne
  `/codev-explore <sujet>` comme alternative

### Requirement: `onboard` fait partie du catalogue par défaut

Le tableau `DEFAULT_WORKFLOWS` de `codev-agents::workflows` MUST
contenir la **liste complète des 8 workflows** de codev : `propose`,
`explore`, `onboard`, `apply`, `sync`, `archive`, `update` et
`configure`. Un utilisateur qui lance `codev init --yes` (ou depuis un
pipe non interactif) sur un projet neuf, sans clé `workflows:` dans
son `config.yaml`, obtient donc toutes les skills disponibles
immédiatement — y compris `configure`, la porte d'entrée
recommandée après l'init.

Un projet qui veut restreindre le catalogue MUST déclarer une clé
`workflows:` explicite avec un sous-ensemble choisi — c'est la voie
opt-out, plutôt que l'ancienne voie opt-in.

Cette bascule règle un problème de découverte : sous l'ancien défaut
(3 workflows), un utilisateur qui tapait `/codev-apply` après
`/codev-propose` ne trouvait pas la skill et croyait qu'elle
n'existait pas.

#### Scenario: Catalogue par défaut inclut les 8 workflows

- **GIVEN** un projet dont le `config.yaml` n'a pas de clé
  `workflows:`
- **WHEN** `select(None)` est appelé sur le catalogue
- **THEN** la liste des `id` retournés est exactement
  `["propose", "explore", "onboard", "apply", "sync", "archive", "update", "configure"]`
- **AND** aucun warning n'est émis

#### Scenario: Restriction opt-out via workflows explicite

- **GIVEN** un projet dont le `config.yaml` contient
  `workflows: [propose, explore, onboard]`
- **WHEN** `select` est appelé avec cette liste
- **THEN** seules ces trois skills sont retournées
- **AND** `apply`, `sync`, `archive`, `update`, `configure` ne sont
  **pas** installés
