# Sortie JSON

Toute commande qui agit sur un projet accepte `--json`. La sortie constitue un
contrat public et versionné — la version 1 — sur lequel s'appuient les skills,
et que vous pouvez utiliser dans vos scripts et votre CI.

## Garanties

- **Un seul document.** La sortie standard contient exactement un document
  JSON, indenté, y compris lorsque la commande échoue. Rien d'autre n'est
  écrit sur la sortie standard.
- **Un tableau `status`.** Chaque document comporte un tableau `status`, vide
  lorsque tout s'est bien passé, et qui liste sinon les avertissements et les
  erreurs.
- **La même forme en cas d'échec.** Une commande en échec renvoie la forme de
  sa sortie nominale, avec des champs vides ou `null`, plus l'erreur dans
  `status`. Un consommateur analyse les deux cas avec le même code.
- **Des noms stables.** Les noms de champs sont en camelCase. Les codes sont
  stables et faits pour être testés ; les messages sont faits pour être lus et
  peuvent être reformulés.
- **Des chemins portables.** Les chemins utilisent le séparateur `/` sur tous
  les systèmes. Les chemins figurant dans les constats sont relatifs à la
  racine du projet.
- **Une évolution par ajout.** Au sein de la version 1, des champs peuvent être
  ajoutés, jamais renommés ni supprimés. Certains champs facultatifs sont omis
  lorsqu'ils sont vides, comme indiqué ci-dessous.

Le code de sortie suit les mêmes règles qu'en mode lisible : `0` en cas de
succès, `1` en cas d'échec et, pour `codev validate`, `1` en présence
d'erreurs (ou de tout constat avec `--strict`).

## Le tableau status

```json
{
  "status": [
    {
      "level": "error",
      "code": "ambiguous_change",
      "message": "several active changes: add-audit-log, rework — specify which one"
    }
  ]
}
```

| Champ | Valeurs |
|---|---|
| `level` | `error` ou `warning` |
| `code` | Un identifiant stable, comme `no_codev_root` |
| `message` | Texte lisible par un humain |

Codes d'erreur courants :

| Code | Signification |
|---|---|
| `no_codev_root` | Aucun `_codev/` dans le répertoire courant ni dans ses parents |
| `no_active_change` | Le projet n'a aucun change actif |
| `ambiguous_change` | Plusieurs changes actifs et aucun `--change` |
| `unknown_change` | Le change désigné n'existe pas |
| `change_exists` | `new change` avec un nom déjà pris |
| `schema_not_found` | Nom de schéma inconnu |
| `no_artifact_ready` | `instructions` sans artefact, alors qu'aucun n'est prêt |
| `template_not_found` | Un schéma fait référence à un template manquant |
| `validation_failed` | `sync` ou `archive` a refusé un change dont la validation signale des erreurs |
| `invalid` | Un fichier invalide, ou un delta qui ne peut pas être fusionné dans sa spec principale |
| `unreadable` | Un fichier n'a pas pu être lu |
| `write_failed` | Un fichier n'a pas pu être écrit |

Les avertissements reprennent le code de la situation qu'ils signalent, par
exemple `inherit_unresolved`, `git_source_unlocked`, `unknown_workflow` ou
`artifact_skipped`.

## Documents par commande

### init et update

`codev init --json` et `codev update --json` :

```json
{
  "root": "/home/you/acme-app",
  "created": ["…"],
  "updated": ["…"],
  "untouched": ["…"],
  "preserved": ["/home/you/acme-app/.claude/skills/codev-apply/SKILL.md"],
  "skills": ["codev-propose", "codev-explore", "…"],
  "status": []
}
```

`preserved` liste les skills laissées en place parce qu'elles ont été
modifiées à la main.

### new change

```json
{
  "changeName": "add-audit-log",
  "schemaName": "spec-driven",
  "changeRoot": "/home/you/acme-app/_codev/changes/add-audit-log",
  "created": [
    "/home/you/acme-app/_codev/changes/add-audit-log/change.yaml"
  ],
  "status": []
}
```

### list

```json
{
  "changes": ["add-audit-log"],
  "root": "/home/you/acme-app",
  "status": []
}
```

Avec `--specs`, le tableau s'appelle `specs` et liste des chemins de
capacités.

### status

```json
{
  "changeName": "add-audit-log",
  "schemaName": "spec-driven",
  "planningHome": "/home/you/acme-app",
  "changeRoot": "/home/you/acme-app/_codev/changes/add-audit-log",
  "applyRequires": ["tasks"],
  "isPlanningComplete": false,
  "artifacts": [
    {
      "id": "proposal",
      "outputPath": "proposal.md",
      "status": "ready",
      "requires": [],
      "missingDeps": []
    },
    {
      "id": "specs",
      "outputPath": "specs/**/*.md",
      "status": "blocked",
      "requires": ["proposal"],
      "missingDeps": ["proposal"]
    }
  ],
  "status": []
}
```

Le `status` d'un artefact vaut `done`, `ready`, `blocked` ou `skipped`.
`planningHome` est la racine du projet ; les skills s'en servent plutôt que de
supposer le répertoire courant.

### instructions

```json
{
  "changeName": "add-dark-mode",
  "schemaName": "spec-driven",
  "artifact": "design",
  "description": "Technical approach and decisions",
  "resolvedOutputPath": "/home/you/acme-app/_codev/changes/add-dark-mode/design.md",
  "instruction": "Write the document that explains HOW to implement the change.\n…",
  "template": "# Design: <change title>\n…",
  "language": "en",
  "context": [
    { "origin": "path:~/src/acme-standards", "text": "All services log in JSON." },
    { "origin": "project", "text": "TypeScript monorepo." }
  ],
  "rules": [
    { "origin": "project", "text": "Cite the decisions that constrain the approach." }
  ],
  "dependencies": [
    { "id": "proposal", "path": "/home/you/acme-app/_codev/changes/add-dark-mode/proposal.md", "done": true }
  ],
  "unlocks": ["tasks"],
  "decisions": [
    {
      "id": "0003",
      "qualifiedId": "project/0003",
      "title": "Use SQLite for persistence",
      "status": "accepted",
      "tags": [],
      "path": "_codev/decisions/0003-use-sqlite-for-persistence.md",
      "origin": "project"
    }
  ],
  "skipped": false,
  "status": []
}
```

| Champ | Signification |
|---|---|
| `resolvedOutputPath` | Où écrire ; peut être un motif glob comme `…/specs/**/*.md` |
| `instruction`, `template` | Issus du schéma ; `null` lorsque le schéma n'en définit pas |
| `language` | La langue des artefacts, `en` par défaut |
| `context`, `rules` | Blocs ordonnés de la source la plus générale jusqu'au projet, chacun avec son `origin` (`project`, `path:<path>` ou `git:<url>`) |
| `dependencies` | Les artefacts dont celui-ci dépend, avec leur chemin et leur état d'achèvement |
| `unlocks` | Les artefacts qui deviennent rédigeables dès que celui-ci existe |
| `decisions` | Les décisions en vigueur — renseigné pour `design`, vide pour les autres artefacts |
| `skipped` | `true` lorsque `skip_specs` désactive cet artefact : il ne doit pas être rédigé |

### validate

```json
{
  "root": "/home/you/acme-app",
  "items": [
    {
      "kind": "change",
      "name": "add-audit-log",
      "path": "_codev/changes/add-audit-log",
      "findings": [
        {
          "path": "_codev/changes/add-audit-log/change.yaml",
          "line": 1,
          "severity": "error",
          "code": "zero_delta_without_marker",
          "message": "no delta file under `specs/` and `skip_specs: true` is not declared; add a delta or set `skip_specs: true` in change.yaml"
        }
      ]
    }
  ],
  "hasWarnings": false,
  "status": []
}
```

`kind` vaut `change`, `spec` ou `decisions`. Les constats sont rapportés dans
`items`, et non dans `status` : `status` ne signale qu'un échec de la commande
elle-même. Voir [Codes de validation](file-formats.md#codes-de-validation).

### sync et archive

```json
{
  "changeName": "add-dark-mode",
  "root": "/home/you/acme-app",
  "updated": [],
  "created": ["/home/you/acme-app/_codev/specs/ui/theme/spec.md"],
  "unchanged": [],
  "deleted": [],
  "movedTo": "/home/you/acme-app/_codev/changes/archive/2026-09-30-add-dark-mode",
  "status": []
}
```

`codev sync --json` renvoie le même document, sans `movedTo`. `deleted` liste
les specs principales supprimées par un change qui retire une capacité.

### schemas

```json
{
  "schemas": [
    { "name": "spec-driven", "origin": "built-in", "flow": ["proposal", "specs", "design", "tasks"] }
  ],
  "root": "/home/you/acme-app",
  "status": []
}
```

`origin` vaut `project` ou `built-in`.

### decision

`codev decision list --json` renvoie `{ "root", "decisions": [...], "status" }`,
et `codev decision show --json` renvoie `{ "root", "decision", "content",
"status" }`, où `content` est le Markdown complet du fichier. Une décision se
présente ainsi :

```json
{
  "id": "0001",
  "qualifiedId": "project/0001",
  "title": "Use PostgreSQL for persistence",
  "status": "superseded",
  "date": "2026-09-30",
  "tags": [],
  "supersedes": [],
  "deviatesFrom": [],
  "path": "…/_codev/decisions/0001-use-postgresql-for-persistence.md",
  "origin": "project",
  "inEffect": false,
  "supersededBy": "project/0003"
}
```

`deviatedBy` est ajouté pour une décision héritée écartée par un écart local.
`origin` vaut `project`, `path:<path>` ou `git:<url>`.

Les commandes qui créent ou scellent des décisions renvoient :

| Commande | Champs en plus de `root` et `status` |
|---|---|
| `decision new` | `decision`, `path`, `bodySha256` (seulement avec `--status accepted`) |
| `decision accept` | `decision`, `path`, `bodySha256`, `superseded` |
| `decision supersede` | `newDecision`, `newPath`, `oldId`, `oldQualifiedId`, `oldPath` |
| `decision seal` | `seal` (`id`, `bodySha256`, `sealedAt`), `wasNoop` |
| `decision deviate` | `decision`, `path`, `targetQualifiedId` |
| `decision promote` | `decision`, `path`, `sourceChange`, `designPath` |

`bodySha256` n'est présent que si la commande a scellé la décision :
`decision supersede`, `decision deviate` et `decision promote` créent une
décision `proposed` et ne scellent rien. Pour `decision supersede`, `oldId`,
`oldQualifiedId` et `oldPath` désignent la décision que l'acceptation de la
nouvelle remplacera ; elle n'est pas modifiée. `superseded` liste les
prédécesseurs que `decision accept` a passés à `superseded` — chacun avec son
`id`, son `qualifiedId` et son `path` — et reste vide quand la décision ne
remplace rien.

Par exemple, `codev decision accept 0002 --json` pour une décision créée par
`codev decision supersede 0001 "Use SQLite"` :

```json
{
  "root": "/home/you/acme-app",
  "decision": {
    "id": "0002",
    "qualifiedId": "project/0002",
    "title": "Use SQLite",
    "status": "accepted",
    "date": "2026-09-30",
    "tags": [],
    "supersedes": [
      "0001"
    ],
    "deviatesFrom": [],
    "path": "_codev/decisions/0002-use-sqlite.md",
    "origin": "project",
    "inEffect": true,
    "supersededBy": null
  },
  "path": "/home/you/acme-app/_codev/decisions/0002-use-sqlite.md",
  "bodySha256": "sha256:c8873b83c178dbf8434045f43a722911b9d9913ee9f13915ccda893539b788c1",
  "superseded": [
    {
      "id": "0001",
      "qualifiedId": "project/0001",
      "path": "/home/you/acme-app/_codev/decisions/0001-use-postgresql.md"
    }
  ],
  "status": []
}
```

Une acceptation refusée garde la même forme, avec `decision` et `path` à
`null`, `superseded` vide et le code dans `status` : `unknown_decision_id`,
`cannot_accept_inherited`, `decision_not_proposed`,
`cannot_supersede_inherited` ou `predecessor_not_accepted`.

### sources

`codev sources list --json` renvoie `{ "root", "sources": [...], "status" }`,
où une source se présente ainsi :

```json
{
  "type": "git",
  "address": "git@github.com:acme/codev-standards.git",
  "state": "locked",
  "gitRef": "main",
  "sha": "12958274cbed0eb66985f57d3272c6d3134b672d",
  "resolvedPath": "/home/you/.cache/codev/content/12958274cbed0eb66985f57d3272c6d3134b672d"
}
```

`type` vaut `path` ou `git` ; `state` prend l'une des valeurs `resolved`,
`unreadable`, `unlocked`, `locked`, `needs_update`. `gitRef`, `subpath`, `sha`
et `resolvedPath` sont omis lorsqu'ils ne s'appliquent pas.

`codev sources show --json` renvoie `{ "source", "filesExposed", "root",
"status" }`. `codev sources update --json` renvoie `{ "root", "changes",
"lockWritten", "status" }`, où chaque modification comporte un `kind`
(`added`, `moved` ou `unchanged`), `url`, `gitRef`, `from` (pour `moved`) et
`to`.
