# Les skills Claude Code

codev fournit huit workflows. `codev init` et `codev update` convertissent
chacun d'eux en skill Claude Code dans
`.claude/skills/codev-<workflow>/SKILL.md`. Claude Code découvre les skills
automatiquement, et vous pouvez aussi en invoquer une directement en tapant
son nom comme commande slash, par exemple `/codev-propose`.

| Skill | Ce qu'elle fait | Ce qu'elle modifie |
|---|---|---|
| `/codev-propose` | Crée un change et rédige tous ses artefacts de planification en une passe | `_codev/changes/<name>/` uniquement |
| `/codev-explore` | Mûrit une idée, compare des approches, clarifie un besoin | Rien |
| `/codev-onboard` | Présente codev, lit l'état du projet, recommande la prochaine action | Rien |
| `/codev-configure` | Propose un `context:` plus riche et des `rules:` par artefact pour `_codev/config.yaml` | `_codev/config.yaml`, après confirmation |
| `/codev-apply` | Implémente les tâches d'un change planifié en les cochant au fur et à mesure | Code du projet et `tasks.md` |
| `/codev-update` | Révise un artefact de planification déjà rédigé et préserve la cohérence des autres | `_codev/changes/<name>/` uniquement |
| `/codev-sync` | Fusionne les deltas d'un change dans les specs principales sans l'archiver | Uniquement via `codev sync` |
| `/codev-archive` | Valide un change, le fusionne et le déplace dans l'archive | Uniquement via `codev archive` |

## Permissions des outils

Chaque skill déclare les outils qu'elle peut utiliser dans le champ
`allowed-tools` de son frontmatter. Ces permissions font partie de
l'engagement de chaque skill : une skill qui ne doit pas écrire de fichiers ne
reçoit pas les outils pour le faire.

| Skill | `allowed-tools` |
|---|---|
| `codev-propose` | `Bash(codev:*), Read, Write, Edit, Glob, Grep`, plus l'outil MCP Jira s'il est configuré |
| `codev-explore` | `Bash(codev:*), Read, Glob, Grep` |
| `codev-onboard` | `Bash(codev:*), Read, Glob` |
| `codev-configure` | `Bash(codev:*), Read, Write, Edit, Glob, Grep` |
| `codev-apply` | `Bash(codev:*), Read, Write, Edit, Glob, Grep, Bash` |
| `codev-update` | `Bash(codev:*), Read, Write, Edit, Glob, Grep` |
| `codev-sync` | `Bash(codev:*), Read` |
| `codev-archive` | `Bash(codev:*), Read` |

`Bash(codev:*)` permet à une skill de lancer des commandes `codev`, et rien
d'autre. Seule `codev-apply` obtient un accès général à `Bash`, parce qu'elle
doit lancer vos tests et vos commandes de build. `codev-sync` et
`codev-archive` ne peuvent écrire aucun fichier : toutes leurs modifications
passent par la CLI.

## Annonce des mises à jour

`/codev-propose`, `/codev-apply`, `/codev-update`, `/codev-sync` et
`/codev-archive` lisent les objets `update` et `versionRequirement` de leur
premier `codev status --json`. Lorsque le `codev_version` du projet n'est pas
satisfait, ou qu'un codev plus récent est disponible et n'est pas en
sourdine, la skill le signale en une ligne, avec le lien de la release, et
vous demande **Update now** ou **Later** avant toute autre chose :

- **Update now** lance `codev self-update --yes`, puis reprend l'action ;
- **Later** lance `codev self-update --snooze` — 7 jours de silence pour
  cette version — puis reprend l'action. Pour un `codev_version` non
  satisfait, rien n'est mis en sourdine : c'est l'équipe qui l'a demandé.

La question est posée une fois par action, jamais au milieu : `/codev-apply`
la pose avant la première tâche. `/codev-explore`, `/codev-onboard` et
`/codev-configure` ne la posent jamais ; `/codev-onboard` relaie
`codev doctor`, dont la vérification `codev_version` dit la même chose.
Aucune skill n'a besoin de plus que `Bash(codev:*)` pour cela.

## Les skills en détail

### Propose (`/codev-propose`)

Transforme une demande en plan complet : proposal, deltas de specs, design et
tâches. La skill lit le code concerné avant d'écrire, fonde le périmètre sur
ce qu'elle y trouve, et signale les contradictions avec les specs existantes
au lieu de les trancher seule. Elle rédige la prose dans la
[langue des artefacts](guides/artifact-language.md) du projet.

La demande n'autorise que la planification, même si elle dit « construis » ou
« corrige » : la skill s'arrête une fois les artefacts rédigés et attend votre
retour.

Chaque artefact a un rôle explicite, tiré des instructions du schéma —
product owner pour la proposal, analyste QA pour les specs, architecte pour
le design, tech lead pour les tâches — avec des critères « Done when » que
l'agent vérifie avant de passer à la suite. Les critères du tech lead exigent
que la documentation concernée soit mise à jour dans une tâche, et que chaque
scénario soit couvert par un test automatisé qui suit les conventions de test
de votre projet. Le product owner rédige la note de version de la proposal
pour les utilisateurs, classée par catégorie, ou `None` pour un change
interne. Un [schéma personnalisé](guides/custom-schemas.md) définit les
siens.

Avant de présenter le plan, la skill le relit en deux étapes :

1. **Traçabilité** — chaque capacité a son fichier de spec, chaque exigence
   un scénario nominal et un scénario d'erreur ou de cas limite, chaque
   scénario une tâche dont la vérification le couvre par un test
   automatisé, une tâche met à jour la documentation lorsque la section
   Impact touche un comportement documenté, et la
   [note de version](reference/file-formats.md#release-note) de la proposal
   concorde avec le plan — chaque élément incompatible de What Changes, et
   chaque exigence supprimée ou modifiée de façon incompatible, a son élément
   **BREAKING**. Les manques sont corrigés dans les artefacts.
2. **Relecture contradictoire** — la skill relit le plan en relecteur
   sceptique : besoin, périmètre, specs, décisions, hypothèses, tâches.
   Sécurité et confidentialité, compatibilité et migration, exploitabilité,
   performance, accessibilité et UX ne s'ajoutent que lorsque la section
   Impact de la proposal les touche, de même que les
   [`rules`](reference/configuration.md#rules) de votre projet. Elle corrige
   ce qui ne fait pas de doute, consigne ce qui demande votre décision dans
   les Open Questions du design, et n'élargit jamais le périmètre.

Le résumé se termine par les **points à challenger** : au plus cinq points
d'une ligne, classés par impact — quoi, pourquoi c'est important, quel
artefact — ou la mention qu'aucun point n'a été relevé. Commencez votre
relecture par là.

Si la demande mentionne un identifiant de ticket correspondant à
`[A-Z]{2,}-\d+` et qu'un outil MCP Jira est configuré, la skill récupère le
premier ticket — en lecture seule, en un seul appel — et le cite en tête de la
proposal. Voir [Jira et autres serveurs MCP](guides/mcp.md).

Rôles : analyste métier, enquêteur de code, product owner, analyste QA,
architecte, tech lead, contrôleur de traçabilité et relecteur sceptique — voir
[Planifier un change](roles.md#planifier-un-change-codev-propose).

### Explore (`/codev-explore`)

Un partenaire de réflexion pour les demandes floues, les investigations et les
comparaisons d'approches. La skill lit le code et les specs, mais ne crée ni
change, ni artefact, ni fichier. Une fois l'idée claire, elle suggère
`/codev-propose`.

Rôle : [partenaire de réflexion](roles.md#partenaire-de-réflexion-codev-explore).

### Onboard (`/codev-onboard`)

Une visite guidée en lecture seule pour qui découvre codev : ce que fait
l'outil, ce que contient le projet actuel (specs, changes actifs, décisions),
et l'action à mener ensuite — par exemple `/codev-configure` sur un projet
fraîchement initialisé. Elle lance aussi `codev doctor --json` et liste
jusqu'à cinq éléments manquants de l'installation avec leur correction, sans
en exécuter aucune.

Rôle : [guide](roles.md#guide-codev-onboard).

### Configure (`/codev-configure`)

Lit le README, le guide de contribution, la documentation et un échantillon du
code source, puis rédige un bloc `context:` et des `rules:` par artefact pour
`_codev/config.yaml`. La skill affiche le diff et n'écrit qu'après votre
confirmation. Elle ne touche jamais à `schema`, `language`, `workflows`, `mcp`
ni `inherits`, et préserve les commentaires existants.

Ses règles `tasks` comprennent des règles de test pour votre langage :
framework, emplacement des tests, nommage, commandes — tirées de vos tests
existants. C'est seulement lorsque le projet n'a encore aucun test qu'elle
se rabat sur les conventions courantes de Rust, TypeScript/JavaScript,
Python, Go ou Java.

Tant que la configuration ne comporte aucune `rules:`, `codev init` et
`codev status` recommandent de la lancer.

Rôle : [analyste de configuration](roles.md#analyste-de-configuration-codev-configure).

### Apply (`/codev-apply`)

Implémente un change planifié. La skill vérifie que la planification est
complète, puis traite dans l'ordre chaque tâche non cochée de `tasks.md`, la
vérifie et ne la coche qu'une fois la vérification réussie. Elle écrit les
tests à la manière de votre suite de tests existante, puis selon les
conventions de votre langage et vos `rules`. Elle prend la commande de test
dans votre configuration, votre manifeste ou votre CI, et vous la demande si
elle n'en trouve aucune. Une fois toutes les tâches cochées, elle relit le
code touché par le change comme le ferait un nouvel arrivant : elle simplifie
ce qui ne fait pas de doute, sans changer le comportement ni le périmètre, et
signale au plus trois notes de lisibilité. Elle lance ensuite une fois la
suite de tests complète. Elle s'arrête au premier blocage ou à la première
ambiguïté. Elle ne modifie jamais un autre change, ne synchronise jamais et
n'archive jamais.

Rôles : [développeur](roles.md#développeur-codev-apply),
[relecteur nouvel arrivant](roles.md#relecteur-nouvel-arrivant-codev-apply).

### Update (`/codev-update`)

Révise un artefact existant — par exemple une exigence d'un delta de spec
après relecture — puis recherche dans les autres artefacts les répercussions
éventuelles et les signale. La skill ne crée pas les artefacts manquants
(c'est le rôle de `/codev-propose`), ne modifie pas le code (celui de
`/codev-apply`) et ne touche pas aux changes archivés. Elle se termine par
`codev validate`.

Rôle : [éditeur](roles.md#éditeur-codev-update).

### Sync (`/codev-sync`)

Lance `codev sync` pour un change et indique quelles specs principales ont été
créées, mises à jour ou laissées inchangées.

Rôle : [mainteneur des specs vivantes](roles.md#mainteneur-des-specs-vivantes-codev-sync-codev-archive).

### Archive (`/codev-archive`)

Lance `codev archive` pour un change. Si la validation signale des erreurs, la
skill s'arrête et vous renvoie vers `codev validate` au lieu de chercher à les
contourner.

Rôle : [mainteneur des specs vivantes](roles.md#mainteneur-des-specs-vivantes-codev-sync-codev-archive).

## Choisir les skills à installer

Par défaut, les huit skills sont installées. Pour en installer moins,
choisissez le préréglage **minimal** à l'initialisation — `propose`,
`explore`, `onboard` et `configure` — ou listez les workflows souhaités dans
`_codev/config.yaml` :

```yaml
workflows:
  - propose
  - explore
  - apply
  - archive
```

Lancez ensuite `codev update`. Un nom de workflow inconnu produit un
avertissement et est ignoré. `codev update` ne supprime pas les skills qui ne
sont plus listées : supprimez vous-même leurs dossiers dans `.claude/skills/`.

## Modifier une skill générée

Chaque `SKILL.md` généré enregistre la version de codev qui l'a produit. Si
vous modifiez une skill à la main, `codev update` détecte la modification et
laisse le fichier en place :

```text
  Left in place because edited by hand — rerun with --force to overwrite:
    /home/you/acme-app/.claude/skills/codev-apply/SKILL.md
```

`codev update --force` l'écrase avec la version générée. Les skills issues
d'une version antérieure de codev sont toujours régénérées.
