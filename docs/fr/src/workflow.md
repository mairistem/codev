# Le déroulé

Dans un projet Compagnon, chaque modification passe par les quatre mêmes
étapes, dans le même ordre. Cette suite d'étapes est le
[déroulé](glossary.md#déroulé) de Compagnon. À chaque étape, vous formulez
votre demande à Claude avec vos mots, et Claude suit une compétence (skill) :
un ensemble d'instructions écrit pour cette étape. Trois des étapes ont
aussi une commande à taper dans un terminal (la CLI), qui effectue le
travail de fond. La réalisation n'a pas de commande : c'est le travail de
l'agent.

```text
┌───────────┐    ┌───────────┐    ┌───────────┐    ┌───────────┐
│  propose  │───▶│   apply   │───▶│   sync    │───▶│  archive  │
│ planifier │    │ construire│    │ fusionner │    │  classer  │
│           │    │           │    │ les deltas│    │           │
└───────────┘    └───────────┘    └───────────┘    └───────────┘
```

| Étape | Compétence | Commande | Ce qu'elle écrit |
|---|---|---|---|
| Proposer | `/compagnon-propose` | `compagnon new change`, `compagnon instructions` | `_compagnon/changes/<name>/` |
| Réaliser | `/compagnon-apply` | — | Code du projet, cases de `tasks.md` |
| Synchroniser (facultatif) | `/compagnon-sync` | `compagnon sync` | `_compagnon/specs/` |
| Archiver | `/compagnon-archive` | `compagnon archive` | `_compagnon/specs/`, `_compagnon/changes/archive/` |

Deux autres compétences vous aident en chemin. `/compagnon-explore` vous
permet de mûrir une idée avant de proposer quoi que ce soit.
`/compagnon-update` révise un plan déjà rédigé. Voir
[Les compétences de Claude Code](skills.md).

À chaque étape, Claude tient un rôle défini : responsable produit (product
owner), analyste QA, architecte, référent technique (tech lead), relecteur
sceptique ou développeur. Chaque rôle a des critères qu'il vérifie avant de
passer à la suite. Claude s'arrête aux moments où c'est à vous de décider.
Voir [Rôles](roles.md).

## Proposer

Proposer, c'est planifier la modification avant d'écrire la moindre ligne
de code. Claude crée un dossier pour la modification et y rédige le plan.
Le plan se compose de plusieurs fichiers, que Compagnon appelle des
[artefacts](glossary.md#artefact). Un [schéma](glossary.md#schéma) fixe les
fichiers que contient un plan. Avec le schéma par défaut, `spec-driven`, un
plan en compte quatre :

| Artefact | Fichier | Demande d'abord | Ce qu'il dit |
|---|---|---|---|
| `proposal` | `proposal.md` | — | Pourquoi la modification est nécessaire, ce qui change, et quelles [capacités](glossary.md#capacité) (domaines du produit) elle touche |
| `specs` | `specs/**/*.md` | `proposal` | Les changements apportés aux spécifications, appelés [deltas](glossary.md#delta) : le comportement ajouté, modifié, supprimé ou renommé |
| `design` | `design.md` | `proposal` | Comment la construire : décisions, solutions écartées, risques |
| `tasks` | `tasks.md` | `specs`, `design` | La liste des tâches à réaliser |

Les fichiers dépendent les uns des autres. Les spécifications et la
conception peuvent être rédigées dès que la proposition existe. Les tâches
viennent une fois les deux terminées. Compagnon déduit où en est un plan
uniquement des fichiers présents : il ne tient aucun registre à part.

<details>
<summary>Sous le capot : comment `compagnon status` présente chaque artefact</summary>

`compagnon status` présente chaque artefact comme terminé (`[x]`), prêt à
être rédigé (`[ ]`), bloqué par une dépendance (`[-]`) ou ignoré (`[~]`). Cet
état se déduit uniquement des fichiers présents : Compagnon ne tient aucun
fichier d'état.

</details>

Dans Claude Code, `/compagnon-propose` rédige tout le plan d'une traite :

1. Si votre demande cite un numéro de ticket comme `PROJ-123`, et qu'une
   connexion à Jira (un serveur MCP) est configurée, Claude lit d'abord le
   ticket. Voir [Jira et autres serveurs MCP](guides/mcp.md).
2. Il crée le dossier de la modification.
3. Pour chaque fichier prêt à être rédigé, il récupère les instructions de
   ce fichier, étudie le code concerné et rédige le fichier à partir de son
   modèle.
4. Il vérifie que la proposition, les spécifications et les tâches
   concordent. Puis il relit le plan en relecteur sceptique. Il écrit chaque
   constat dans le [compte rendu de revue](guides/review-record.md) de la
   modification, corrige ce qui ne fait pas de doute et liste jusqu'à cinq
   points à examiner avec vous. Voir
   [Proposer](skills.md#propose-compagnon-propose).
5. Il s'arrête une fois le plan complet et vous le présente. Il ne commence
   jamais la réalisation dans le même tour.

<details>
<summary>Sous le capot : les commandes que lance la compétence de proposition</summary>

L'étape 2 lance `compagnon new change <name>`. L'étape 3 lance, pour chaque
artefact prêt :

```bash
compagnon instructions <artifact> --change <name> --json
```

</details>

Quand il rédige `design.md`, Claude reçoit les décisions d'architecture en
vigueur. La conception les respecte alors, ou propose ouvertement d'en
remplacer une. Voir [Décisions](concepts.md#décision).

Certaines modifications ne changent pas ce que fait le produit : un
nettoyage du code, une nouvelle version d'une bibliothèque, de la
documentation. Une telle modification déclare `skip_specs: true` dans son
`change.yaml`. Son plan n'a alors pas de spécifications, et la validation
accepte qu'elle n'ait aucun delta.

## Réaliser

`/compagnon-apply <name>` construit ce que décrit le plan. Claude lit
`tasks.md` et prend chaque tâche non cochée (`- [ ]`) dans l'ordre du
fichier. Pour chacune, il lance la vérification que la tâche indique : un
test, une commande ou un comportement observable. Il ne coche la case
qu'une fois cette vérification réussie.

Les nouveaux tests suivent d'abord vos tests existants, puis les habitudes
de votre langage de programmation, puis vos
[`rules`](reference/configuration.md#rules).

Une fois toutes les tâches cochées, Claude relit le code qu'il a touché,
comme le ferait un nouvel arrivant sur le projet. Les
[experts](roles.md#experts-et-domaines) qui correspondent aux domaines cités
dans la proposition se joignent à cette relecture. Claude simplifie ce qui
ne fait pas de doute, sans élargir le périmètre, et liste au plus cinq
notes de relecture dans son résumé. Chaque constat est conservé dans le
[compte rendu de revue](guides/review-record.md) de la modification, avec la
réponse de l'auteur et la vérification du relecteur. Ce que vous seul
pouvez trancher est listé à la fin. Claude lance ensuite une fois la suite
de tests complète.

Claude s'arrête au premier blocage ou au premier point flou pour vous
consulter, plutôt que de deviner. Il ne travaille que sur la modification
que vous avez désignée. Il n'archive jamais et ne synchronise jamais : c'est
à vous de demander ces étapes.

La réalisation est la seule compétence autorisée à lancer n'importe quelle
commande dans le terminal, parce qu'elle doit lancer vos tests. Elle n'a
pas de commande à elle : construire est le travail de l'agent.

## Synchroniser

```bash
compagnon sync --change <name>
```

La synchronisation recopie les deltas de la modification dans les
spécifications du projet, sous `_compagnon/specs/`, **sans** déplacer la
modification. Elle sert dans deux cas : quand une autre modification en
cours a besoin de voir une nouvelle capacité dans les spécifications du
projet, ou quand vous voulez vérifier le résultat avant l'archivage.

La relancer ne fait aucun mal : la deuxième fois, elle indique que les
spécifications sont inchangées. En général, vous sautez cette étape, car
l'archivage synchronise pour vous.

Comme l'archivage, la synchronisation valide d'abord la modification. Si la
validation signale une erreur, elle refuse de continuer : rien n'est écrit,
et `compagnon validate <name>` montre ce qui ne va pas.

## Archiver

```bash
compagnon archive --change <name>
```

L'archivage clôt une modification, en quatre temps :

1. Il valide la modification. Si la validation signale une erreur, il
   **refuse** de continuer et vous renvoie vers `compagnon validate <name>`
   pour le détail.
2. Il vérifie le [compte rendu de revue](guides/review-record.md). Il
   **refuse** de continuer (`review_blocked`) si ce compte rendu manque pour
   une étape où des rôles de relecture étaient actifs, ou si un constat de
   gravité haute est encore ouvert ou contesté.
   `compagnon review --change <name>` montre ce qui reste.
3. Il recopie les deltas dans les spécifications du projet, exactement comme
   la synchronisation.
4. Il déplace le dossier de la modification, compte rendu compris, dans
   `_compagnon/changes/archive/<YYYY-MM-DD>-<name>/`.

Compagnon calcule et vérifie toute la fusion avant d'écrire le premier
fichier. Un problème est donc signalé avant toute modification sur le
disque : une modification invalide, ou un delta qui ne s'applique pas. Par
exemple, un delta peut modifier (`MODIFIED`) une exigence que la
spécification du projet ne contient pas.

Après l'archivage, les spécifications du projet décrivent le comportement
que vous avez livré. Le dossier archivé garde la trace du pourquoi et du
comment. Enregistrez les deux dans Git (commit). Si la modification a une
note de version, lancez `compagnon changelog --write` pour l'ajouter à
`CHANGELOG.md` (voir [Notes de version et CHANGELOG](guides/release-notes.md)).

## Choisir la modification

`compagnon status`, `compagnon instructions`, `compagnon sync` et
`compagnon archive` acceptent `--change <name>`. Quand le projet compte
exactement une modification en cours, vous pouvez l'omettre. S'il y en a
plusieurs, Compagnon vous demande de choisir :

```text
error: several active changes: add-audit-log, rework — specify which one
help: `--change add-audit-log`
```

Les noms des modifications s'écrivent en [kebab-case](glossary.md#kebab-case) :
lettres minuscules, chiffres et tirets, par exemple `add-user-auth`.

## Valider

```bash
compagnon validate
```

Sans rien d'autre, `compagnon validate` vérifie chaque modification en
cours, chaque spécification du projet et les sceaux qui protègent les
décisions. Pour ne vérifier qu'un élément, donnez le nom d'une modification
ou d'une capacité. Pour ne vérifier qu'une famille, ajoutez `--changes` ou
`--specs`.

Une erreur fait échouer la commande (code de sortie 1). Avec `--strict`,
les avertissements aussi. Utilisez `--strict` en intégration continue (CI),
les vérifications lancées automatiquement à chaque envoi de code :

```bash
compagnon validate --strict
```

La validation vérifie :

- la forme des spécifications et des deltas : chaque exigence emploie
  `SHALL` ou `MUST` (ou `DOIT` ou `DOIVENT`) et comporte au moins un
  `#### Scenario:` ;
- qu'un delta ne se contredit pas : aucune exigence à la fois ajoutée et
  modifiée, aucun renommage en conflit avec un ajout ;
- la règle du delta vide : une modification sans aucun delta doit déclarer
  `skip_specs: true` ;
- que les décisions scellées n'ont pas été altérées.

Voir [Formats de fichiers](reference/file-formats.md) pour les règles que
chaque fichier doit respecter.
