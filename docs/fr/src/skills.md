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
| `codev-propose` | `Bash(codev:*), Read, Write, Edit, Glob, Grep`, plus l'outil MCP Jira s'il est configuré, plus `Agent` en mode de revue isolé |
| `codev-explore` | `Bash(codev:*), Read, Glob, Grep` |
| `codev-onboard` | `Bash(codev:*), Read, Glob` |
| `codev-configure` | `Bash(codev:*), Read, Write, Edit, Glob, Grep` |
| `codev-apply` | `Bash(codev:*), Read, Write, Edit, Glob, Grep, Bash`, plus `Agent` en mode de revue isolé |
| `codev-update` | `Bash(codev:*), Read, Write, Edit, Glob, Grep` |
| `codev-sync` | `Bash(codev:*), Read` |
| `codev-archive` | `Bash(codev:*), Read` |

`Bash(codev:*)` permet à une skill de lancer des commandes `codev`, et rien
d'autre. Seule `codev-apply` obtient un accès général à `Bash`, parce qu'elle
doit lancer vos tests et vos commandes de build. `codev-sync` et
`codev-archive` ne peuvent écrire aucun fichier : toutes leurs modifications
passent par la CLI.

`Agent` est l'outil de sous-agent de Claude Code. `codev-propose` et
`codev-apply` ne le demandent que lorsque `_codev/config.yaml` définit
`review: { mode: isolated }`
([Revue en ligne ou isolée](roles.md#revue-en-ligne-ou-isolée)) : chaque
rôle relecteur travaille alors comme sous-agent. Aucune autre skill ne le
demande jamais. Après avoir changé de mode, lancez `codev update` ;
`codev doctor` signale `skills_review_mode` tant que vous ne l'avez pas
fait. Les deux skills décrivent la revue isolée dans les deux modes, car
le `change.yaml` d'un change peut remplacer le mode du projet : sans
l'outil pré-autorisé, Claude Code demande avant de lancer un sous-agent,
et s'il ne peut pas en lancer, les rôles relisent en ligne.

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

## Les corrections de l'auteur

Quand vous corrigez l'agent pendant `/codev-propose`, `/codev-apply` ou
`/codev-update` sur la manière de faire de votre projet — une convention,
un choix technique, une attente sur un artefact —, il note la correction
une fois dans le [`corrections.md`](reference/file-formats.md#correctionsmd)
du change : où, ce qu'il avait fait, ce que vous avez demandé, une ligne
chacun. Il vous cite ensuite l'entrée, pour que vous puissiez la faire
modifier ou retirer avant le commit. `codev review` liste les corrections,
et `codev validate` vérifie leur fichier.

Seule une consigne qui vaudrait aussi pour le change suivant est notée. Une
préférence ponctuelle sur ce change, une faute de frappe, un changement de
périmètre ou un malentendu sur la demande ne le sont pas. Une consigne du
projet va dans `corrections.md`, que l'équipe partage par git, et non dans
la mémoire personnelle de Claude Code. Une préférence sur la façon dont
l'agent travaille avec vous reste dans cette mémoire personnelle. Une
entrée ne nomme personne et ne cite ni donnée personnelle ni secret.

## Les skills en détail

### Propose (`/codev-propose`)

Transforme une demande en plan complet : proposal, deltas de specs, design et
tâches. La skill lit le code concerné avant d'écrire, fonde le périmètre sur
ce qu'elle y trouve, et signale les contradictions avec les specs existantes
au lieu de les trancher seule. Elle rédige la prose dans la
[langue des artefacts](guides/artifact-language.md) du projet.

Une fois la proposition écrite, les instructions qu'elle lit listent les
exigences existantes que le change touche probablement. La skill les lit et
modifie celles que le change altère au lieu d'en ajouter de nouvelles à
côté ; la liste est un point de départ, pas une liste complète.

La demande n'autorise que la planification, même si elle dit « construis » ou
« corrige » : la skill s'arrête une fois les artefacts rédigés et attend votre
retour.

Chaque artefact a un rôle explicite, déclaré par le schéma et renvoyé par
`codev instructions` —
product owner pour la proposal, analyste QA pour les specs, architecte pour
le design, tech lead pour les tâches — avec des critères « Done when » que
l'agent vérifie avant de passer à la suite. Les critères du tech lead exigent
que la documentation concernée soit mise à jour dans une tâche, et que chaque
scénario soit couvert par un test automatisé qui suit les conventions de test
de votre projet. Le product owner rédige la note de version de la proposal
pour les utilisateurs, classée par catégorie, ou `None` pour un change
interne. Un [schéma personnalisé](guides/custom-schemas.md) définit les
siens.

À chaque étape, elle demande `codev roles --at <step>` les rôles de l'étape
et applique leurs critères, y compris les critères ajoutés par votre projet ;
une étape dont votre configuration désactive le rôle est sautée, et le
résumé le signale (voir [Configurer les rôles](roles.md#configurer-les-rôles)).

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
   sceptique : besoin, périmètre, specs, décisions, hypothèses, tâches,
   et si `## Domains` coche chaque domaine que le plan touche, ainsi que
   les [`rules`](reference/configuration.md#rules) de votre projet. Le
   skeptic ne relit lui-même aucun domaine : sécurité, données
   personnelles, migration, exploitation, performance et accessibilité
   reviennent à leurs experts. Les
   [experts](roles.md#experts-et-domaines) dont la proposal coche le
   domaine dans `## Domains` — sécurité, données personnelles,
   accessibilité, performance, exploitation, migration — rejoignent cette
   relecture avec leurs propres connaissances. Avant d'écrire
   `## Domains`, la skill lit les
   [domaines que suggère le diff du change](roles.md#les-déclencheurs-et-les-domaines-suggérés)
   et coche chacun ou le garde non coché avec une raison ; le skeptic
   consigne un constat pour chacun resté `unticked`. Chaque constat est écrit
   dans le [compte rendu de revue](guides/review-record.md) du change, avec
   sa gravité et sa preuve citée ; l'auteur le corrige ou le conteste, le
   relecteur vérifie chaque réponse une fois, et la relecture n'élargit
   jamais le périmètre. En [mode isolé](roles.md#revue-en-ligne-ou-isolée),
   le contrôleur de traçabilité et chaque relecteur travaillent comme des
   sous-agents distincts qui ne voient jamais la conversation, et la skill
   fusionne leurs constats avant de les consigner.

Le résumé se termine par les **points à challenger** : au plus cinq points
d'une ligne, tous rôles confondus, classés par gravité et marqués du rôle
qui les a relevés — identifiant du constat, statut, quoi, pourquoi c'est
important, quel artefact — ou la mention qu'aucun point n'a été relevé,
puis par **À décider** : les contestations que le relecteur a maintenues et
les constats de gravité haute encore ouverts. Commencez votre relecture par
là.

Si la demande mentionne un identifiant de ticket correspondant à
`[A-Z]{2,}-\d+` et qu'un outil MCP Jira est configuré, la skill récupère le
premier ticket — en lecture seule, en un seul appel — et le cite en tête de la
proposal. Voir [Jira et autres serveurs MCP](guides/mcp.md).

Rôles : analyste, enquêteur, product owner, analyste QA, architecte, tech
lead, contrôleur de traçabilité et contradicteur — voir
[Rôles intégrés](roles.md#rôles-intégrés).

### Explore (`/codev-explore`)

Un partenaire de réflexion pour les demandes floues, les investigations et les
comparaisons d'approches. La skill lit le code et les specs, mais ne crée ni
change, ni artefact, ni fichier. Une fois l'idée claire, elle suggère
`/codev-propose`.

Posture : [partenaire de réflexion](roles.md#partenaire-de-réflexion-codev-explore).

### Onboard (`/codev-onboard`)

Une visite guidée en lecture seule pour qui découvre codev : ce que fait
l'outil, ce que contient le projet actuel (specs, changes actifs, décisions),
et l'action à mener ensuite — par exemple `/codev-configure` sur un projet
fraîchement initialisé. Elle lance aussi `codev doctor --json` et liste
jusqu'à cinq éléments manquants de l'installation avec leur correction, sans
en exécuter aucune.

Posture : [guide](roles.md#guide-codev-onboard).

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

Posture : [analyste de configuration](roles.md#analyste-de-configuration-codev-configure).

### Apply (`/codev-apply`)

Implémente un change planifié. La skill vérifie que la planification est
complète, puis traite dans l'ordre chaque tâche non cochée de `tasks.md`, la
vérifie et ne la coche qu'une fois la vérification réussie. Elle écrit les
tests à la manière de votre suite de tests existante, puis selon les
conventions de votre langage et vos `rules`. Elle prend la commande de test
dans votre configuration, votre manifeste ou votre CI, et vous la demande si
elle n'en trouve aucune. Une fois toutes les tâches cochées, elle relit le
code touché par le change comme le ferait un nouvel arrivant, rejointe par
les experts qu'activent les domaines de la proposal : elle simplifie ce qui
ne fait pas de doute, sans élargir le périmètre, et signale au plus cinq
notes de relecture, chacune marquée de son rôle — un constat de sécurité ou
de données personnelles de gravité haute est toujours signalé. Chaque
constat va dans le [compte rendu de revue](guides/review-record.md) et suit
la même boucle de réponse que ceux du plan — en mode isolé, après que
chaque relecteur a travaillé seul comme sous-agent et que la skill a
fusionné leurs constats —, et le résumé liste ce qu'il vous
revient de décider — y compris tout domaine que le code suggère désormais et
que la proposal ne coche ni ne justifie : cochez-le et son expert relit le
code, ou justifiez-le. Un expert
corrige un défaut manifeste dans le périmètre du change, avec un test, et
signale tout autre changement de comportement au lieu de le faire. Elle
tire les critères et les connaissances de ces deux étapes de
`codev roles` ; lorsqu'aucun rôle n'est actif à la relecture, celle-ci est
sautée et le résumé le signale. Elle lance ensuite une fois la suite de tests complète. Elle s'arrête au premier blocage ou à la première
ambiguïté. Elle ne modifie jamais un autre change, ne synchronise jamais et
n'archive jamais.

Rôles : [développeur](roles.md#développeur-codev-apply),
[nouvel arrivant et experts](roles.md#nouvel-arrivant-et-experts-codev-apply).

### Update (`/codev-update`)

Révise un artefact existant — par exemple une exigence d'un delta de spec
après relecture — puis recherche dans les autres artefacts les répercussions
éventuelles et les signale. La skill ne crée pas les artefacts manquants
(c'est le rôle de `/codev-propose`), ne modifie pas le code (celui de
`/codev-apply`) et ne touche pas aux changes archivés. Elle se termine par
`codev validate`.

Posture : [éditeur](roles.md#éditeur-codev-update).

### Sync (`/codev-sync`)

Lance `codev sync` pour un change et indique quelles specs principales ont été
créées, mises à jour ou laissées inchangées.

Posture : [mainteneur des specs vivantes](roles.md#mainteneur-des-specs-vivantes-codev-sync-codev-archive).

### Archive (`/codev-archive`)

Lance `codev archive` pour un change. Si la validation signale des erreurs, la
skill s'arrête et vous renvoie vers `codev validate` au lieu de chercher à les
contourner. Si le compte rendu de revue bloque l'archivage
(`review_blocked`), elle vous renvoie vers `codev review` et ne consigne
jamais une décision à votre place.

Posture : [mainteneur des specs vivantes](roles.md#mainteneur-des-specs-vivantes-codev-sync-codev-archive).

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

Chaque `SKILL.md` généré enregistre la version de codev qui l'a produit et
une empreinte (`checksum`) de son propre contenu. Si vous modifiez une skill
à la main, son contenu ne correspond plus à son empreinte : `codev update`
détecte la modification et laisse le fichier en place :

```text
  Left in place because edited by hand — rerun with --force to overwrite:
    /home/you/acme-app/.claude/skills/codev-apply/SKILL.md
```

`codev update --force` l'écrase avec la version générée. Les skills issues
d'une autre version de codev sont toujours régénérées, tout comme les skills
de cette version que codev a rendues lui-même — une compilation antérieure, ou
l'autre mode de revue —, puisqu'elles correspondent encore à leur empreinte.
Une skill de cette version générée avant les empreintes est préservée comme
une modification : lancez une fois `codev update --force` pour lui donner son
empreinte — vérifiez d'abord `git diff`, car il écrase aussi vos vraies
modifications, puis réappliquez-les.
