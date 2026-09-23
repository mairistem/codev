# Skills Specification

## Purpose

Décrit le contrat des skills que codev installe dans Claude Code : leur nom,
ce qu'elles doivent faire, ce qu'elles n'ont pas le droit de faire, et
comment leur frontmatter garantit ces promesses. Les entrées sont ajoutées
au fil des changes qui introduisent chaque workflow — un ADDED par workflow.

## Requirements

### Requirement: Skill `apply` guide l'implémentation d'un change

Le catalogue de codev SHALL exposer un workflow `apply` — installé sous
`.claude/skills/codev-apply/SKILL.md`, invocable `/codev-apply` — dont le
rôle est de traiter les tâches non cochées du `tasks.md` d'un change, dans
l'ordre du fichier, en cochant chaque case à mesure.

#### Scenario: Implémentation d'un change avec un seul actif

- **GIVEN** un projet avec un seul change actif dont `tasks.md` porte deux
  tâches non cochées
- **WHEN** l'utilisateur tape `/codev-apply`
- **THEN** la skill résout implicitement le change actif
- **AND** implémente la première tâche puis la coche
- **AND** implémente la seconde tâche puis la coche

#### Scenario: Reprise après interruption

- **GIVEN** un `tasks.md` où la première tâche est déjà cochée `- [x]` et la
  seconde ne l'est pas
- **WHEN** l'utilisateur tape `/codev-apply`
- **THEN** la skill ignore la tâche déjà cochée
- **AND** commence par la première tâche non cochée

#### Scenario: Ambiguïté demande un choix explicite

- **GIVEN** deux changes actifs
- **WHEN** l'utilisateur tape `/codev-apply` sans nom
- **THEN** la skill demande lequel appliquer, en listant les deux noms

### Requirement: Skill `apply` respecte les frontières du change

Le workflow `apply` MUST se cantonner à ce qui est nécessaire pour cocher
les tâches du change nommé : il MUST NOT modifier d'autres changes, MUST NOT
archiver ni sync tout seul, et MUST s'arrêter dès qu'une tâche est
ambiguë ou bloquée plutôt que de deviner.

#### Scenario: Refus d'archiver depuis apply

- **GIVEN** un change dont toutes les tâches sont cochées
- **WHEN** l'utilisateur tape `/codev-apply`
- **THEN** la skill signale que le change est prêt à être archivé
- **AND** invite explicitement à lancer `/codev-archive` ou `codev archive`
  comme prochaine étape séparée

#### Scenario: Tâche ambiguë interrompt le flux

- **GIVEN** un `tasks.md` contenant une tâche dont la formulation admet
  plusieurs interprétations qui changeraient matériellement le résultat
- **WHEN** la skill arrive à cette tâche
- **THEN** la skill demande une clarification à l'utilisateur avant
  d'implémenter
- **AND** ne coche pas la tâche tant que la clarification n'est pas obtenue

### Requirement: Contrat du frontmatter d'une skill codev

Toute skill livrée par codev MUST porter un frontmatter YAML valide dont le
`name` correspond au nom du dossier `.claude/skills/<name>/`, dont le champ
`allowed-tools` inclut au moins `Bash(codev:*)`, et dont `metadata.version`
correspond à la version du binaire qui l'a générée.

#### Scenario: Frontmatter parseur par un lecteur YAML tiers

- **GIVEN** une skill livrée par la version courante du binaire
- **WHEN** son frontmatter est extrait et passé à un parseur YAML standard
- **THEN** le parseur rend `name`, `allowed-tools` et `metadata.version`
  sans erreur
- **AND** `metadata.version` égale la version que le binaire annonce

#### Scenario: Édition à la main détectée à l'update

- **GIVEN** une skill dont un utilisateur a édité le corps à la main, sans
  changer sa version
- **WHEN** l'utilisateur relance `codev update` sans `--force`
- **THEN** la skill n'est pas écrasée
- **AND** le rapport de l'update la signale comme préservée

### Requirement: Skill `sync` merge le delta d'un change sans le déplacer

Le catalogue de codev SHALL exposer un workflow `sync` — installé sous
`.claude/skills/codev-sync/SKILL.md`, invocable `/codev-sync` — dont le rôle
est de faire entrer les deltas d'un change dans les specs principales, en
laissant le change actif à son emplacement.

#### Scenario: Sync d'un change actif unique

- **GIVEN** un projet avec un seul change actif dont la planification est
  complète et qui porte un delta ADDED sur une capacité nouvelle
- **WHEN** l'utilisateur tape `/codev-sync`
- **THEN** la skill résout implicitement le change actif
- **AND** lance `codev sync <nom>`
- **AND** résume à l'utilisateur les main specs créées ou mises à jour

#### Scenario: Deuxième sync silencieux

- **GIVEN** un change déjà synchronisé, dont aucune main spec n'a changé
  depuis
- **WHEN** l'utilisateur tape `/codev-sync` une seconde fois
- **THEN** la skill rend compte qu'il n'y a rien à faire
- **AND** ne relance pas d'écriture

#### Scenario: Sync ne déplace jamais

- **GIVEN** un change dont la fusion réussit
- **WHEN** l'utilisateur tape `/codev-sync`
- **THEN** le dossier `_codev/changes/<nom>/` existe toujours à son
  emplacement d'origine

#### Scenario: Sync invite à archiver après un changement

- **GIVEN** un change dont la fusion a modifié au moins une spec principale
  (créée ou mise à jour)
- **WHEN** l'utilisateur tape `/codev-sync`
- **THEN** le rendu final contient une ligne invitant à `/codev-archive`
  pour clore le cycle, formulée sans injonction

#### Scenario: Sync sans changement n'invite pas

- **GIVEN** un change dont la fusion est un no-op (toutes les specs
  principales sont déjà à jour)
- **WHEN** l'utilisateur tape `/codev-sync`
- **THEN** le rendu final rend compte de l'absence de changement
- **AND** ne suggère PAS d'archiver — il n'y a rien de nouveau à propager

### Requirement: Skill `archive` clôt un change avec pré-flight strict

Le catalogue SHALL exposer un workflow `archive` — installé sous
`.claude/skills/codev-archive/SKILL.md`, invocable `/codev-archive` — dont le
rôle est de fusionner le delta puis de déplacer le change vers
`_codev/changes/archive/<date>-<nom>/`. La skill MUST refuser d'agir si
`codev archive` rapporte un pré-flight de validation en échec.

#### Scenario: Archive d'un change validé

- **GIVEN** un change dont la planification est complète et qui passe
  `codev validate`
- **WHEN** l'utilisateur tape `/codev-archive`
- **THEN** la skill lance `codev archive <nom>`
- **AND** résume à l'utilisateur les main specs touchées
- **AND** nomme la destination d'archive datée

#### Scenario: Archive refusé pour erreur de validation

- **GIVEN** un change dont un delta contient une erreur remontée par
  `codev validate` (par exemple, une exigence dupliquée)
- **WHEN** l'utilisateur tape `/codev-archive`
- **THEN** la skill n'insiste pas
- **AND** invite explicitement l'utilisateur à lancer `codev validate <nom>`
  pour voir le détail
- **AND** ne tente pas de deviner ou de corriger l'erreur

### Requirement: Skills `sync` et `archive` s'appuient sur le contrat JSON

Les workflows `sync` et `archive` MUST invoquer le CLI avec `--json` et lire
la forme structurée (`SyncReportV1`, `ArchiveReportV1`) plutôt que la sortie
humaine — c'est le contrat public que codev garantit stable dans sa version
courante, et c'est ce qui rend le rendu de la skill fiable.

#### Scenario: Rendu structuré des créations et mises à jour

- **GIVEN** un change dont la fusion crée une spec principale et en met une
  autre à jour
- **WHEN** l'utilisateur tape `/codev-sync`
- **THEN** le rendu nomme distinctement les deux — le fichier créé et le
  fichier mis à jour — chacun sur sa ligne

#### Scenario: Refus d'archive détecté par code stable

- **GIVEN** un change dont `codev archive --json` refuse avec le code
  `validation_failed` dans son tableau `status`
- **WHEN** l'utilisateur tape `/codev-archive`
- **THEN** la skill détecte le code stable dans le JSON
- **AND** dit exactement : « Le change a des erreurs. Lance `codev validate
  <nom>` pour voir le détail. »
- **AND** ne parse pas le message humain (qui peut être reformulé sans
  préavis)

### Requirement: Skills `sync` et `archive` ne demandent pas le Bash général

Les workflows `sync` et `archive` MUST se limiter à `Bash(codev:*)` et à des
outils de lecture dans leur frontmatter `allowed-tools` — ils n'exécutent
aucune commande de vérification autre que celles du binaire codev, à
l'inverse de `apply` qui doit pouvoir lancer des tests projets.

#### Scenario: Le Bash général n'apparaît pas

- **GIVEN** la skill `sync` livrée par la version courante
- **WHEN** son frontmatter est inspecté
- **THEN** la chaîne `allowed-tools` ne contient pas `Bash` seul en fin de
  liste, seulement le préfixe `Bash(codev:*)`
- **AND** la même règle vaut pour `archive`

### Requirement: Skill `update` révise un artefact de planification

Le catalogue de codev SHALL exposer un workflow `update` — installé sous
`.claude/skills/codev-update/SKILL.md`, invocable `/codev-update` — dont le
rôle est de réviser un artefact de planification déjà écrit (proposal,
specs, design ou tasks) d'un change actif, à partir d'une description libre
donnée par l'utilisateur.

#### Scenario: Révision d'un design d'après une nouvelle contrainte

- **GIVEN** un change actif dont `design.md` cite une décision technique X
- **WHEN** l'utilisateur tape `/codev-update design "remplacer X par Y à
  cause de la contrainte Z"`
- **THEN** la skill lit `design.md`, applique la révision demandée, et
  écrit la nouvelle version
- **AND** relance `codev validate <change>` en fin de traitement

#### Scenario: Résolution implicite quand un seul change est actif

- **GIVEN** un projet avec un seul change actif
- **WHEN** l'utilisateur tape `/codev-update proposal "réduire le
  périmètre"`
- **THEN** la skill résout implicitement le change actif
- **AND** applique la révision au `proposal.md` de ce change

#### Scenario: Ambiguïté sur le change à réviser

- **GIVEN** deux changes actifs
- **WHEN** l'utilisateur tape `/codev-update tasks "…"` sans nommer de
  change
- **THEN** la skill demande à l'utilisateur lequel réviser, en listant les
  deux noms
- **AND** n'écrit rien avant d'avoir la réponse

### Requirement: Skill `update` annonce la ripple avant d'agir

Quand la révision demandée sur un artefact rend un autre incohérent, la
skill MUST le signaler à l'utilisateur et proposer la correction avant de
l'écrire, plutôt que de laisser la spec principale, le design ou la liste
de tâches en désaccord silencieux.

#### Scenario: Retirer une capacité du proposal ripple sur specs

- **GIVEN** un `proposal.md` déclarant deux capacités nouvelles `a` et
  `b`, et un fichier `specs/b/spec.md` déjà écrit
- **WHEN** l'utilisateur tape `/codev-update proposal "retirer la
  capacité b — hors périmètre finalement"`
- **THEN** la skill applique la révision au `proposal.md`
- **AND** signale à l'utilisateur que `specs/b/spec.md` devient orphelin
- **AND** propose de supprimer ce fichier ou d'appeler
  `/codev-update specs …` pour l'ajuster
- **AND** n'écrit pas cette seconde modification sans confirmation

#### Scenario: Une révision sans ripple s'applique sans confirmation supplémentaire

- **GIVEN** une révision qui ne touche qu'à `design.md` sans conséquence
  sur les autres artefacts
- **WHEN** l'utilisateur tape `/codev-update design "…"`
- **THEN** la skill applique la révision sans demander de confirmation
  additionnelle

### Requirement: Skill `update` reste dans la frontière planning

Le workflow `update` MUST se limiter aux fichiers sous
`_codev/changes/<nom>/` et MUST NOT :

- modifier du code du projet ;
- créer un artefact manquant (proposal, specs, design, tasks) — c'est
  `/codev-propose` qui le fait ;
- toucher à un change déjà archivé sous `changes/archive/`.

#### Scenario: Refus d'écrire un artefact manquant

- **GIVEN** un change dont `design.md` n'existe pas encore
- **WHEN** l'utilisateur tape `/codev-update design "ajouter la décision
  Z"`
- **THEN** la skill refuse
- **AND** invite explicitement à `/codev-propose` pour créer l'artefact

#### Scenario: Refus d'un change archivé

- **GIVEN** un change qui vit sous `changes/archive/2026-09-09-<nom>/`
- **WHEN** l'utilisateur tape `/codev-update proposal --change
  <archived-nom>`
- **THEN** la skill refuse
- **AND** rappelle qu'un change archivé est de l'histoire ; corriger
  demande de le dé-archiver à la main

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

### Requirement: Skill `propose` détecte les tickets Jira mentionnés et enrichit le proposal

Le workflow `propose` du catalogue SHALL, en amont de la résolution du
nom de change, scanner le prompt de l'utilisateur pour repérer un
identifiant de ticket qui matche le pattern régulier `[A-Z]{2,}-\d+`
(par exemple `JVS-1234`, `PROJ-42`).

Sur détection d'au moins un ticket, la skill MUST :

1. **Tenter d'appeler** l'outil MCP dont le nom est déclaré côté
   projet via `_codev/config.yaml.mcp.jira_tool`. Ce nom est injecté
   au moment de l'installation de la skill dans le frontmatter
   `allowed-tools` et dans le body — l'utilisateur/organisation
   choisit **son** MCP (`mcp__claude_ai_Atlassian__getJiraIssue`,
   `mcp__claude_ai_Atlassian_Rovo__getJiraIssue`, ou un autre) sans
   que ça touche au code source de codev.
2. **Un seul appel par invocation**, sur l'identifiant du ticket
   **le plus tôt mentionné dans le prompt** ; les autres sont juste
   nommés.
3. **En cas de succès** : injecter le contenu (titre, description,
   status, type) dans le contexte de rédaction, et faire figurer le
   ticket en tête du `proposal.md` sous forme d'une ligne citation
   `> Source : ticket **<ID>** — « <titre> » (<status>)`.
4. **En cas d'échec — outil MCP indisponible dans la session ou
   non configuré côté projet** : afficher un message informatif à
   l'utilisateur, puis continuer le flow habituel sans le contenu
   du ticket. Le proposal cite quand même le ticket en tête (« Source
   : ticket **<ID>** — contenu non récupéré »).
5. **En l'absence de pattern** : comportement bit-identique à
   aujourd'hui — aucun appel MCP, aucun message.

Le CATALOG des workflows MUST utiliser un placeholder textuel
`{{JIRA_MCP_TOOL}}` dans `allowed_tools` et dans le body du
workflow `propose`, en lieu et place d'un nom de MCP hardcodé. Le
placeholder est substitué au moment du rendering du frontmatter
d'installation par `ClaudeCode::render`.

Quand `_codev/config.yaml.mcp.jira_tool` est **absent** ou vide, le
rendering MUST :

- retirer proprement `{{JIRA_MCP_TOOL}}` de `allowed_tools` **et** la
  virgule qui le précède (pour ne pas laisser un `allowed-tools`
  malformé qui se terminerait par `", "`) ;
- remplacer chaque occurrence dans le body par la chaîne
  `(MCP Jira non configuré)` — la skill reste installée et
  fonctionnelle sur les autres aspects, mais n'appelle plus de MCP.

#### Scenario: Configuration `mcp.jira_tool` présente → skill fonctionnelle

- **GIVEN** un projet dont `_codev/config.yaml` déclare
  `mcp: { jira_tool: mcp__claude_ai_Atlassian_Rovo__getJiraIssue }`
- **WHEN** l'utilisateur lance `codev update`
- **THEN** le fichier `.claude/skills/codev-propose/SKILL.md` porte
  `allowed-tools: "Bash(codev:*), Read, Write, Edit, Glob, Grep, mcp__claude_ai_Atlassian_Rovo__getJiraIssue"`
- **AND** le body de la skill cite
  `mcp__claude_ai_Atlassian_Rovo__getJiraIssue` là où le CATALOG
  contient `{{JIRA_MCP_TOOL}}`
- **AND** aucun `{{…}}` ne subsiste dans le fichier installé

#### Scenario: Configuration `mcp.jira_tool` absente → fallback propre

- **GIVEN** un projet dont `_codev/config.yaml` n'a pas de bloc `mcp:`
- **WHEN** l'utilisateur lance `codev update`
- **THEN** le fichier `.claude/skills/codev-propose/SKILL.md` porte
  `allowed-tools: "Bash(codev:*), Read, Write, Edit, Glob, Grep"`
  (sans le placeholder et sans virgule finale)
- **AND** le body remplace `{{JIRA_MCP_TOOL}}` par la mention
  `(MCP Jira non configuré)`
- **AND** aucun `{{…}}` ne subsiste dans le fichier installé

#### Scenario: Ticket mentionné, MCP configuré et disponible → ticket cité en tête

- **GIVEN** un utilisateur tape `/codev-propose ajouter JWT pour JVS-1234`
- **AND** le projet a configuré `mcp.jira_tool:
  mcp__claude_ai_Atlassian_Rovo__getJiraIssue`
- **AND** ce MCP est disponible dans la session Claude Code
- **AND** le ticket `JVS-1234` existe
- **WHEN** la skill s'exécute
- **THEN** un appel `mcp__claude_ai_Atlassian_Rovo__getJiraIssue({issueIdOrKey: "JVS-1234"})`
  est émis
- **AND** le `proposal.md` du change créé contient en tête une ligne
  `> Source : ticket **JVS-1234** — « <titre> » (<status>)`

#### Scenario: Ticket mentionné, MCP absent → proposal quand même

- **GIVEN** un utilisateur tape `/codev-propose corriger JVS-1234`
- **AND** aucun MCP Jira n'est configuré (`mcp.jira_tool` absent ou
  outil déclaré indisponible)
- **WHEN** la skill s'exécute
- **THEN** la skill affiche un message informatif nommant
  `JVS-1234` et indiquant que le MCP Jira n'est pas actif
- **AND** le proposal est créé quand même, en tête duquel
  `JVS-1234` est mentionné avec « contenu non récupéré »

#### Scenario: Aucun ticket mentionné → comportement inchangé

- **GIVEN** un utilisateur tape `/codev-propose ajouter
  l'authentification`
- **WHEN** la skill s'exécute
- **THEN** aucun appel MCP n'est émis
- **AND** aucun message relatif à un ticket n'apparaît
- **AND** le proposal est rédigé exactement comme avant ce lot
