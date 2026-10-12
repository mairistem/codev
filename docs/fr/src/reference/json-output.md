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
- **Des chemins portables.** Tous les chemins, absolus ou relatifs, utilisent
  le séparateur `/` sur tous les systèmes — sous Windows, `C:/work/acme-app`.
  Les chemins figurant dans les constats sont relatifs à la racine du projet.
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
| `invalid_change_id` | Un nom de change qui n'est pas en kebab-case, ou le nom réservé `archive` ; le message nomme la règle enfreinte |
| `change_exists` | `new change` avec un nom déjà pris |
| `schema_not_found` | Nom de schéma inconnu |
| `schema_invalid`, `schema_unreadable` | Un fichier de schéma que codev refuse : un défaut de structure, ou un YAML invalide ou une clé inconnue |
| `unknown_artifact` | `instructions` avec un artefact que le schéma du change n'a pas |
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
  "integration": [
    {
      "step": "claude_md",
      "action": "appended",
      "path": "CLAUDE.md",
      "reason": null
    },
    {
      "step": "ci",
      "action": "skipped",
      "path": ".github/workflows/codev.yml",
      "reason": ".github/workflows/ci.yml already runs codev validate"
    },
    {
      "step": "pr_template",
      "action": "created",
      "path": ".github/PULL_REQUEST_TEMPLATE.md",
      "reason": null
    }
  ],
  "status": []
}
```

`preserved` liste les skills laissées en place parce qu'elles ont été
modifiées à la main. `integration` compte une entrée par étape d'intégration
au dépôt : `step` vaut `claude_md`, `ci` ou `pr_template` ; `action` vaut
`created`, `appended`, `refreshed`, `unchanged` ou `skipped` ; `path` est
relatif au projet, ou `null` ; `reason` explique une étape écartée, ou vaut
`null`. `codev update` ne liste que les fichiers gérés qu'il a trouvés.
L'indication `codev doctor` et le job GitLab n'apparaissent que dans la
sortie humaine.

Avertissements propres à `init` : `inherit_not_written` (`--inherit` alors
que `config.yaml` existe), `inherit_pin_failed` (une source git n'a pas pu
être épinglée) et `managed_section_malformed` (marqueurs codev déséquilibrés ;
le fichier reste intact). Une valeur `--inherit` invalide échoue avec
`invalid_inherit`.

### doctor

`codev doctor --json` :

```json
{
  "root": "/home/you/acme-app",
  "checks": [
    {
      "id": "config.rules",
      "group": "configuration",
      "status": "missing",
      "code": "rules_missing",
      "message": "no rules: in _codev/config.yaml",
      "fix": "/codev-configure"
    },
    {
      "id": "config.language",
      "group": "configuration",
      "status": "ok",
      "code": "ok",
      "message": "language: is set",
      "fix": null
    },
    "…"
  ],
  "summary": {
    "total": 14,
    "missing": 2,
    "warnings": 1
  },
  "status": []
}
```

Chaque vérification a un `id` stable, son `group`, un `status` (`ok`,
`missing` ou `warning`), un `code` stable (`ok` lorsque la vérification
réussit), un `message` et un `fix` — la commande ou la skill à lancer — ou
`null` lorsque le statut est `ok`. Les vérifications suivent l'ordre du
tableau ci-dessous ; `pr_template` n'apparaît que pour un dépôt GitHub.
`summary.missing` détermine le code de sortie de `--strict`.

| `id` de la vérification | `group` | Codes lorsque le statut n'est pas `ok` |
|---|---|---|
| `config.rules` | `configuration` | `rules_missing` |
| `config.language` | `configuration` | `language_missing` |
| `config.keys` | `configuration` | `unknown_config_key` (avertissement) |
| `config.roles` | `configuration` | `roles_invalid` (avertissement) ; lorsqu'il est `ok`, `message` liste les rôles actifs, les rôles actifs selon le domaine avec leurs domaines, et les rôles désactivés |
| `skills` | `configuration` | `skills_missing`, `skills_outdated` |
| `sources` | `configuration` | `source_unpinned`, `lock_stale` (avertissement) |
| `codev_version` | `configuration` | `codev_version_unsatisfied`, `update_available` (avertissement) |
| `specs` | `planning` | `no_main_spec` |
| `decisions` | `planning` | `no_accepted_decision` (avertissement) |
| `validate` | `planning` | `validate_errors`, `validate_warnings` (avertissement) |
| `claude_md` | `repository` | `claude_md_section_missing` |
| `ci` | `repository` | `ci_validate_missing` |
| `pr_template` | `repository` | `pr_template_section_missing` |
| `changelog` | `repository` | `changelog_missing` (avertissement), `changelog_outdated` (avertissement) |

### changelog

`codev changelog --format json` (le `--json` de cette commande) :

```json
{
  "root": "/home/you/acme-app",
  "since": "v1.3.0",
  "until": null,
  "release": null,
  "entries": [
    {
      "category": "Added",
      "text": "`--format json` pour `codev export`.",
      "breaking": false,
      "change": "2026-10-02-export-json",
      "archivePath": "_codev/changes/archive/2026-10-02-export-json",
      "source": "release_note"
    }
  ],
  "markdown": "## [Unreleased]\n\n### Added\n\n- `--format json` pour `codev export`.\n",
  "written": null,
  "status": []
}
```

- `since` — la référence après laquelle commence l'intervalle, explicite ou
  le tag par défaut ; `null` lorsque tous les changes archivés en font
  partie. `until` — `null` pour l'arbre de travail.
- `release` — `{ "version", "date" }` avec `--release`, sinon `null`.
- `entries` — dans l'ordre d'affichage. `text` est l'élément tel qu'écrit,
  marqueur `**BREAKING**` compris. `source` vaut `release_note`, ou
  `what_changes` pour un change archivé sans note de version.
- `markdown` — les éléments mis en forme au format Keep a Changelog.
- `written` — avec `--write`, `{ "path", "action" }`, `action` valant
  `created`, `updated` ou `unchanged` ; sinon `null`.

Avertissements dans `status` : `release_note_missing`,
`release_note_invalid`, `release_note_inconsistent`, `not_a_git_repository`,
`archived_change_unreadable`. Codes d'erreur : `unknown_ref`,
`not_a_git_repository` (avec `--since` ou `--until` hors de git),
`invalid_release_version`, `invalid_date`, `invalid_argument`,
`changelog_malformed`.

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
  "update": {
    "current": "0.5.0",
    "latest": "0.6.0",
    "url": "https://github.com/mairistem/codev/releases/tag/v0.6.0",
    "available": true,
    "snoozed": false
  },
  "versionRequirement": {
    "requirement": ">=0.5",
    "satisfied": true
  },
  "status": []
}
```

Le `status` d'un artefact vaut `done`, `ready`, `blocked` ou `skipped`.
`planningHome` est la racine du projet ; les skills s'en servent plutôt que de
supposer le répertoire courant.

`update` est présent lorsque la dernière release de codev est connue :
`current` est la version en cours, `latest` la dernière release, `url` sa
page, `available` indique si `latest` est plus récente, et `snoozed` si
`codev self-update --snooze` l'a mise en sourdine. Il est absent lorsque la
vérification est désactivée (`CI`, `CODEV_NO_UPDATE_CHECK=1`) ou n'a encore
jamais réussi. `versionRequirement` est présent lorsque `_codev/config.yaml`
définit `codev_version` : `requirement` est sa valeur, `satisfied` indique si
la version en cours la respecte. Ces deux objets s'ajoutent sans rien
changer : un consommateur qui les ignore lit le document comme avant.

### self-update

`codev self-update --json`, qui implique `--yes` :

```json
{
  "current": "0.5.0",
  "latest": "0.6.0",
  "url": "https://github.com/mairistem/codev/releases/tag/v0.6.0",
  "action": "updated",
  "installMethod": "standalone",
  "path": "/home/you/.local/bin/codev",
  "hint": null,
  "projectUpdated": true,
  "status": []
}
```

`action` vaut `updated`, `up_to_date`, `available` (avec `--check`),
`snoozed`, `nothing_to_snooze` ou `managed_elsewhere`. `installMethod` vaut
`standalone`, `cargo`, `build` ou `package_manager` ; pour les trois
derniers, `hint` contient la commande qui met codev à jour. `latest` et `url`
valent `null` lorsqu'ils sont inconnus. Les échecs gardent cette forme et
portent leur code dans `status` — voir les [codes](cli.md#codev-self-update).

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
  "role": {
    "id": "architect",
    "name": "Architect",
    "kind": "artifact",
    "at": "plan.write",
    "steps": ["plan.write"],
    "artifact": "design",
    "domains": [],
    "question": "How will it be built?",
    "when": "always",
    "whenOrigin": "default",
    "activatedBy": "always",
    "disableable": false,
    "knowledge": null,
    "doneWhen": [
      { "origin": "schema:spec-driven", "text": "each decision names the alternative it rejects and why" },
      "…"
    ]
  },
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
| `role` | Le rôle qui a la charge de l'artefact, sous la forme de [`codev roles`](#roles) ; `null` lorsque le schéma n'en déclare pas |

Un échec conserve cette forme, avec `language` toujours renseigné — la langue
configurée, ou `en` lorsque la configuration ne peut pas être lue — et les
autres champs à `null` ou vides.

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

`kind` vaut `change`, `spec`, `decisions` ou `config` — ce dernier seulement
lorsque `_codev/config.yaml` contient des clés que ce codev ne connaît pas
(voir [Clés inconnues](configuration.md#clés-inconnues)). Les constats sont rapportés dans
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

### roles

`codev roles --at plan.challenge --change add-dark-mode --json` :

```json
{
  "changeName": "add-dark-mode",
  "schemaName": "spec-driven",
  "at": "plan.challenge",
  "language": "en",
  "roles": [
    {
      "id": "skeptic",
      "name": "Skeptic",
      "kind": "step",
      "at": "plan.challenge",
      "steps": ["plan.challenge"],
      "artifact": null,
      "domains": [],
      "question": "Would the plan survive a reviewer who did not write it?",
      "when": "always",
      "whenOrigin": "project",
      "activatedBy": "project",
      "disableable": true,
      "knowledge": "## Signals\n\nPlanning smells — each one is a question to raise, not a verdict.\n…",
      "doneWhen": [
        { "origin": "built-in", "text": "**Need** — does the change solve the Why? Is there a simpler solution, or one without code?" },
        "…",
        { "origin": "project", "text": "The rollback plan is stated" }
      ]
    },
    {
      "id": "accessibility",
      "name": "Accessibility",
      "kind": "step",
      "at": "plan.challenge",
      "steps": ["plan.challenge", "code.review"],
      "artifact": null,
      "domains": ["user-interface"],
      "question": "Can everyone use what this change adds, with a keyboard or assistive technology?",
      "when": "on_domain",
      "whenOrigin": "default",
      "activatedBy": "domain:user-interface",
      "disableable": true,
      "knowledge": "## Scope\n\nLook at the user interface the change adds or modifies…",
      "doneWhen": [
        { "origin": "built-in", "text": "Every interactive element is reachable and operable with the keyboard alone, in a logical focus order, with a visible focus" },
        "…"
      ]
    }
  ],
  "status": []
}
```

| Champ | Signification |
|---|---|
| `changeName` | Le change donné avec `--change` ; `null` sans lui |
| `schemaName` | Le schéma d'où viennent les rôles d'artefact |
| `at` | L'étape donnée avec `--at` ; `null` liste tous les rôles, actifs ou non |
| `roles[].name` | Le nom affiché dans la `language` configurée, en anglais lorsque le rôle n'en a pas dans cette langue |
| `roles[].kind` | `step`, ou `artifact` avec l'identifiant de l'artefact dans `artifact` |
| `roles[].at` | L'étape donnée avec `--at`, sinon la première étape du rôle ; `plan.write` pour tous les rôles d'artefact |
| `roles[].steps` | Toutes les étapes que sert le rôle, dans l'ordre du workflow : un expert sert `plan.challenge` et `code.review` |
| `roles[].domains` | Les domaines qui activent un expert, comme `["personal-data"]` ; vide pour les autres rôles |
| `roles[].when`, `roles[].whenOrigin` | `always`, `never` ou `on_domain`, fixé par `default`, `project` ou `change` |
| `roles[].activatedBy` | Pourquoi le rôle est actif : `always`, `project`, `change`, ou `domain:<nom>` pour un expert dont la proposal coche le domaine ; `null` lorsqu'il est inactif |
| `roles[].disableable` | `false` pour les rôles d'artefact, l'`analyst` et le `developer` |
| `roles[].knowledge` | Les connaissances du rôle, en markdown dans les sections Scope, Signals, By stack, References et Reporting, pour un rôle actif qui en a ; `null` sinon |
| `roles[].doneWhen` | Les critères du rôle, puis ceux du projet, chacun avec son `origin` : `built-in`, `schema:<name>` ou `project` |

Avec `--at`, `roles` ne contient que les rôles actifs, éventuellement
aucun : l'étape est alors sautée. Sans `--change`, aucune proposal n'est
lue : les experts sont donc inactifs. Un échec — `unknown_step`, `unknown_role`,
`invalid_role_activation`, `role_not_disableable` — conserve cette forme,
avec `roles` vide et `language` renseigné comme pour `instructions`.

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

`origin` vaut `project` ou `built-in`. Un schéma du projet inutilisable est
écarté de `schemas`, avec un avertissement `schema_unusable` dans `status`.
Un schéma du projet dont le `name:` diffère de son dossier est listé sous le
nom de son dossier, avec un avertissement `schema_name_mismatch`.

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
`origin` vaut `project`, `path:<path>` ou `git:<url>`. `path` est relatif au
projet pour une décision qui s'y trouve, absolu sinon.

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
