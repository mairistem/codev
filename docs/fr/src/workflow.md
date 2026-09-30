# Le workflow

Dans un projet codev, chaque change passe par les quatre mêmes étapes. Chaque
étape dispose d'une skill Claude Code et — sauf l'implémentation — d'une
commande CLI qui effectue le travail sous-jacent.

```text
┌───────────┐    ┌───────────┐    ┌───────────┐    ┌───────────┐
│  propose  │───▶│   apply   │───▶│   sync    │───▶│  archive  │
│ planifier │    │ construire│    │ fusionner │    │  classer  │
│           │    │           │    │ les deltas│    │           │
└───────────┘    └───────────┘    └───────────┘    └───────────┘
```

| Étape | Skill | CLI | Écrit |
|---|---|---|---|
| Propose | `/codev-propose` | `codev new change`, `codev instructions` | `_codev/changes/<name>/` |
| Apply | `/codev-apply` | — | Code du projet, cases de `tasks.md` |
| Sync (facultative) | `/codev-sync` | `codev sync` | `_codev/specs/` |
| Archive | `/codev-archive` | `codev archive` | `_codev/specs/`, `_codev/changes/archive/` |

Deux autres skills accompagnent le cycle : `/codev-explore`, pour mûrir une
idée avant de proposer quoi que ce soit, et `/codev-update`, pour réviser un
plan déjà rédigé. Voir [Les skills Claude Code](skills.md).

## Propose

L'étape propose crée le dossier d'un change et rédige ses artefacts de
planification. Avec le schéma par défaut `spec-driven`, un change compte
quatre artefacts :

| Artefact | Fichier | Dépend de | Rôle |
|---|---|---|---|
| `proposal` | `proposal.md` | — | Pourquoi le change est nécessaire, ce qui change, quelles capacités sont concernées |
| `specs` | `specs/**/*.md` | `proposal` | Les deltas de specs : le comportement ajouté, modifié, supprimé ou renommé |
| `design` | `design.md` | `proposal` | Comment l'implémenter : décisions, alternatives, risques |
| `tasks` | `tasks.md` | `specs`, `design` | La liste des tâches d'implémentation |

Les dépendances forment un graphe : `specs` et `design` peuvent être rédigés
dès que la proposal existe, et `tasks` une fois ces deux artefacts terminés.
`codev status` présente chaque artefact comme terminé (`[x]`), prêt à être
rédigé (`[ ]`), bloqué par une dépendance (`[-]`) ou ignoré (`[~]`). Cet état
se déduit uniquement des fichiers présents : codev ne tient aucun fichier
d'état.

Dans Claude Code, `/codev-propose` fait tout en une seule passe :

1. Si votre demande mentionne un identifiant de ticket comme `PROJ-123` et
   qu'un serveur MCP Jira est configuré, la skill récupère d'abord le ticket.
   Voir [Jira et autres serveurs MCP](guides/mcp.md).
2. Elle crée le change avec `codev new change <name>`.
3. Pour chaque artefact prêt, elle lance
   `codev instructions <artifact> --change <name> --json`, étudie le code
   concerné et rédige le fichier en suivant le template.
4. Elle s'arrête lorsque la planification est complète et présente le plan.
   Elle ne commence jamais l'implémentation dans le même tour.

Lorsqu'elle rédige `design.md`, la skill reçoit les décisions d'architecture
en vigueur, afin que le design les respecte — ou propose explicitement d'en
remplacer une. Voir [Décisions](concepts.md#décision).

Un change sans impact sur le comportement — refactorisation, montée de version
d'une dépendance, documentation — déclare `skip_specs: true` dans son
`change.yaml`. L'artefact `specs` est alors ignoré, et la validation accepte
l'absence de deltas.

## Apply

`/codev-apply <name>` implémente le plan. La skill lit `tasks.md`, traite
chaque tâche `- [ ]` non cochée dans l'ordre du fichier, lance la vérification
indiquée par la tâche (un test, une commande, un comportement observable),
puis coche la case.

Elle s'arrête au premier blocage ou à la première ambiguïté pour vous
consulter plutôt que de deviner. Elle ne modifie que le change désigné,
n'archive jamais et ne synchronise jamais : ces étapes restent des demandes
explicites de votre part.

`apply` est la seule skill autorisée à exécuter des commandes shell
arbitraires, parce qu'elle doit lancer vos tests. Elle n'a pas d'équivalent en
CLI : l'implémentation est le travail de l'agent.

## Sync

```bash
codev sync --change <name>
```

La synchronisation fusionne les deltas du change dans les specs principales
sous `_codev/specs/`, **sans** déplacer le change. Utilisez-la lorsqu'un autre
change en cours a besoin de voir une nouvelle capacité dans les specs
principales, ou pour relire la fusion avant l'archivage.

La synchronisation est idempotente : la relancer indique que les specs sont
inchangées. Dans le cas courant, vous pouvez sauter cette étape : l'archivage
synchronise pour vous.

Comme l'archivage, la synchronisation valide d'abord le change et refuse de
continuer si la validation signale une erreur : rien n'est écrit, et
`codev validate <name>` affiche les constats.

## Archive

```bash
codev archive --change <name>
```

L'archivage clôt un change :

1. Il valide le change et **refuse** de continuer si la validation signale une
   erreur, en vous renvoyant vers `codev validate <name>` pour le détail.
2. Il fusionne les deltas dans les specs principales, exactement comme `sync`.
3. Il déplace le dossier du change dans
   `_codev/changes/archive/<YYYY-MM-DD>-<name>/`.

La fusion complète est calculée et vérifiée avant l'écriture du premier
fichier : un change invalide ou un delta inapplicable — par exemple une
exigence `MODIFIED` absente de la spec principale — est signalé avant toute
modification sur le disque.

Après l'archivage, les specs principales décrivent le comportement que vous
avez livré, et le dossier archivé conserve la trace du pourquoi et du comment.
Commitez les deux.

## Choisir le change

`codev status`, `codev instructions`, `codev sync` et `codev archive`
acceptent `--change <name>`. Lorsque le projet compte exactement un change
actif, vous pouvez l'omettre. S'il y en a plusieurs, codev vous demande de
choisir :

```text
error: several active changes: add-audit-log, rework — specify which one
help: `--change add-audit-log`
```

Les noms de changes sont en kebab-case : lettres minuscules, chiffres et
tirets, par exemple `add-user-auth`.

## Valider

```bash
codev validate
```

Sans argument, `codev validate` vérifie chaque change actif, chaque spec
principale et les sceaux des décisions. Passez le nom d'un change ou d'une
capacité pour ne vérifier qu'un élément, ou `--changes` / `--specs` pour ne
vérifier qu'une famille. Les erreurs font sortir la commande avec le code 1 ;
avec `--strict`, les avertissements aussi. Utilisez cette option en CI :

```bash
codev validate --strict
```

Les vérifications portent sur la structure des specs et des deltas (chaque
exigence emploie `SHALL` ou `MUST` et comporte au moins un
`#### Scenario:`), sur la cohérence interne d'un delta (aucune exigence à la
fois ajoutée et modifiée, aucun renommage en conflit avec un ajout), sur la
règle du delta vide, et sur l'intégrité des décisions scellées. Voir
[Formats de fichiers](reference/file-formats.md) pour les règles que chaque
fichier doit respecter.
