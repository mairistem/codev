# Configuration

Un projet codev se configure dans `_codev/config.yaml`. Toutes les clés sont
facultatives : un projet peut même fonctionner sans ce fichier. Les clés
inconnues sont rejetées avec une erreur qui liste les clés valides : une faute
de frappe ne passe donc jamais inaperçue.

`codev init` génère le fichier avec les valeurs qu'il a détectées, chacune
précédée d'un commentaire indiquant sa provenance. Vous pouvez le modifier à la
main à tout moment. Après avoir modifié `workflows` ou `mcp`, lancez
`codev update` pour que les skills reflètent la modification.

## Exemple complet

```yaml
schema: spec-driven

language: en

workflows:
  - propose
  - explore
  - onboard
  - apply
  - sync
  - archive
  - update
  - configure

context: |
  TypeScript monorepo: a Next.js web app and a Node.js API.
  Errors are typed; never throw raw strings.

rules:
  specs:
    - Describe observable behavior, never implementation.
  design:
    - Cite the decisions in _codev/decisions/ that constrain the approach.
  tasks:
    - "Each task states how to verify it: a named test, a command, or an observable behavior."

inherits:
  - path: ~/src/acme-standards
  - git: git@github.com:acme/codev-standards.git
    ref: main
    subpath: standards

mcp:
  jira_tool: mcp__atlassian__getJiraIssue
```

## schema

Le schéma de workflow utilisé par les nouveaux changes. Valeur par défaut :
`spec-driven`, le schéma embarqué dans codev. Le nom d'un schéma défini sous
`_codev/schemas/` est également accepté. Un change enregistre son schéma dans
son propre `change.yaml` : modifier cette clé n'affecte donc pas les changes
en cours. Voir [Schémas personnalisés](../guides/custom-schemas.md).

## language

La langue dans laquelle les skills rédigent la prose des artefacts, sous la
forme d'un code ISO 639 : deux ou trois lettres minuscules, éventuellement
suivies d'une sous-étiquette (`en`, `fr`, `pt-BR`, `zh-Hant`). Valeur par
défaut : `en`. Les mots-clés structurels restent toujours en anglais. Voir
[Langue des artefacts](../guides/artifact-language.md).

## workflows

Les workflows à installer comme skills Claude Code, désignés par leur
identifiant : `propose`, `explore`, `onboard`, `configure`, `apply`, `sync`,
`archive`, `update`. En l'absence de la clé, les huit sont installés. Un
identifiant inconnu produit l'avertissement `unknown_workflow` et est ignoré.
Voir [Les skills Claude Code](../skills.md).

## context

Texte libre transmis à l'agent avec les instructions de **chaque** artefact :
votre stack, vos conventions, vos contraintes — tout ce qui ne peut pas se
déduire du code. C'est une contrainte imposée à l'agent, jamais un contenu à
recopier dans les fichiers. `/codev-configure` peut le rédiger pour vous.

## rules

Les règles propres à un artefact, sous la forme d'une table associant un
identifiant d'artefact à une liste de chaînes. Avec le schéma `spec-driven`,
les identifiants sont `proposal`, `specs`, `design` et `tasks`. Chaque règle
n'est transmise à l'agent que lorsqu'il rédige l'artefact correspondant.

Dans un scalaire simple, YAML interprète `key: value` comme une table :
mettez donc entre guillemets toute règle contenant un deux-points suivi d'une
espace, comme la règle `tasks` ci-dessus.

Tant que `rules` est vide, `codev init` et `codev status` suggèrent de lancer
`/codev-configure`.

## inherits

Les sources en lecture seule dont hériter le contexte, les règles et les
décisions, dans l'ordre du plus général au plus spécifique. Chaque entrée est
soit un dossier local, soit un dépôt git :

| Clé | S'applique à | Signification |
|---|---|---|
| `path` | local | Dossier contenant un projet codev ; chemin absolu ou commençant par `~` |
| `git` | git | URL du dépôt, telle que `git` la comprend |
| `ref` | git | Branche ou tag à suivre — obligatoire |
| `subpath` | git | Dossier du dépôt qui contient le projet codev |

Une entrée déclare exactement une clé parmi `path` et `git` ; `ref` et
`subpath` sont rejetées sur une entrée `path`. Les sources git sont épinglées
dans `_codev/codev.lock` par `codev sources update`. Voir
[Sources héritées](../guides/inherited-sources.md).

## mcp

Les noms des outils MCP que les skills peuvent appeler. Ils sont propres à
chaque installation de Claude Code et ne sont donc jamais hérités.

| Clé | Signification |
|---|---|
| `jira_tool` | Outil qui récupère un ticket Jira, utilisé par `/codev-propose` lorsqu'un identifiant de ticket est mentionné |

Voir [Jira et autres serveurs MCP](../guides/mcp.md).

## Variables d'environnement

| Variable | Utilisée par | Effet |
|---|---|---|
| `LC_ALL`, `LC_MESSAGES`, `LANG` | `codev init` | Valeur par défaut de `language:` lorsque `--language` n'est pas fourni |
| `XDG_CACHE_HOME` | sources git | Emplacement du cache : `$XDG_CACHE_HOME/codev/`, sinon `~/.cache/codev/` |
| `VISUAL`, `EDITOR` | `codev init` | Éditeur ouvert lorsque la question du contexte reste sans réponse |
| `CODEV_VERSION` | `install.sh`, `install.ps1` | Version à installer à la place de la plus récente |
