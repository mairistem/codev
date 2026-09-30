# Interface en ligne de commande

Ce chapitre documente chaque commande `codev`. L'aide de chaque commande est
reproduite telle que l'affiche `codev <command> --help` ; lancez cette commande
pour consulter l'aide de la version que vous avez installée.

```bash
codev --help
```

```text
codev adds a thin layer of specs to a repository so that you and your agent agree on what must be built before a single line of code is written.

The commands below run in your terminal. The workflows are invoked in the Claude Code chat: /codev-propose, /codev-explore.

Usage: codev <COMMAND>

Commands:
  init          Initialize codev in a project and install the Claude Code skills
  update        Regenerate the skills after a codev upgrade
  new           Create a new item
  list          List active changes, or specified capabilities with --specs
  status        Show the state of a change's artifacts
  instructions  Print everything needed to write an artifact
  schemas       List the available workflow schemas
  decision      Create, inspect and supersede architecture decisions
  sources       Manage inherited sources (local paths and remote git repositories)
  sync          Merge a change's deltas into the main specs without archiving
  archive       Merge, then move a change to the dated archive
  docs          Open the codev documentation in the browser
  completions   Generate a shell completion script for local installation
  validate      Check changes and specs for structural errors and consistency
  help          Print this message or the help of the given subcommand(s)

Options:
  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## Conventions

Ces règles valent pour toutes les commandes, sauf `docs` et `completions`,
qui ne lisent pas le projet :

- **Découverte du projet.** codev cherche `_codev/` dans le répertoire
  courant, puis dans chacun des répertoires parents. Lancez les commandes
  depuis n'importe quel endroit du projet. En dehors d'un projet, les
  commandes échouent avec `no_codev_root`.
- **`--json`.** Toute commande qui lit ou écrit le projet accepte `--json`. La
  sortie standard contient alors exactement un document JSON — y compris en
  cas d'échec — et les messages destinés aux humains ne sont pas affichés.
  Voir [Sortie JSON](json-output.md).
- **Code de sortie.** `0` en cas de succès, `1` en cas d'échec. Pour
  `codev validate`, voir [la section dédiée](#codev-validate).
- **Erreurs.** En mode lisible, les erreurs sont écrites sur la sortie
  d'erreur sous la forme `error: …`, souvent suivies d'une ligne `help: …`
  qui indique la correction.
- **Choix du change.** Les commandes qui acceptent `--change <CHANGE>` le
  déduisent lorsque le projet compte exactement un change actif, et échouent
  avec `ambiguous_change` lorsqu'il y en a plusieurs.

## Projet

### codev init

```text
Initialize codev in a project and install the Claude Code skills

Usage: codev init [OPTIONS] [PATH]

Arguments:
  [PATH]
          Project folder (default: the current folder)

Options:
      --force
          Rewrite skills even if they were edited by hand

  -y, --yes
          Accept all defaults without prompting (defaults + detected values)

      --no-detect
          Disable the environment probe (useful for reproducible tests)

      --preset <PRESET>
          Preselect the answer to the workflows question

          Possible values:
          - full:    All 8 workflows
          - minimal: propose, explore, onboard, configure
          - custom:  Pick the workflows one by one

      --language <CODE>
          Language of the prose skills write in artifacts, as an ISO 639 code (`en`, `fr`, `pt-BR`…). Default: detected from the locale, otherwise `en`

      --json


  -h, --help
          Print help (see a summary with '-h')
```

Initialise codev dans `PATH` (par défaut, le dossier courant) : crée
l'arborescence `_codev/`, écrit `_codev/config.yaml` et installe les skills
dans `.claude/skills/`. Voir
[Ce que fait `codev init`](../quickstart.md#ce-que-fait-codev-init) pour la
détection et les questions.

- `--yes` saute toutes les questions et applique les valeurs par défaut et les
  valeurs détectées. Lorsque l'entrée standard n'est pas un terminal (CI,
  pipes), `--yes` est implicite.
- `--preset` répond à la question des workflows : `full` installe les huit
  workflows, `minimal` installe `propose`, `explore`, `onboard` et
  `configure`, et `custom` vous laisse les choisir un par un (avec un repli
  sur `full` en l'absence de terminal).
- `--language` définit la clé `language:`. Sans cette option, la langue est
  déduite de la locale, à défaut `en`. Voir
  [Langue des artefacts](../guides/artifact-language.md).
- `--no-detect` désactive la détection des manifestes, de la licence, de la
  CI, de la locale et des serveurs MCP.

Si `_codev/config.yaml` existe déjà, il reste intact : `codev init` se
contente de créer les dossiers manquants et d'installer ou de rafraîchir les
skills. Les skills que vous avez modifiées à la main sont préservées, sauf si
vous passez `--force`.

### codev update

```text
Regenerate the skills after a codev upgrade

Usage: codev update [OPTIONS]

Options:
      --force  Rewrite skills even if they were edited by hand
      --json
  -h, --help   Print help
```

Régénère les skills listées par `workflows:` dans la configuration (les huit
en l'absence de la clé), par exemple après une mise à jour de codev ou une
modification de la configuration `mcp:`. Une skill dont le contenu diffère de
ce que génère cette version, mais qui porte la marque de cette version, a été
modifiée à la main : elle est laissée en place et signalée, sauf si vous
passez `--force`. Les skills retirées de `workflows:` ne sont pas
supprimées.

### codev docs

```text
Open the codev documentation in the browser

Three forms:

- default: writes `codev-docs-<version>.html` to the system temp folder and opens it in the browser; - `--write <PATH>`: writes the HTML to the given path, opens nothing (for sharing — email, Confluence, shared drive); - `--print`: prints the markdown source to stdout (to pipe into `less`, `bat` or an LLM).

Usage: codev docs [OPTIONS]

Options:
      --print
          Print the markdown source to stdout (does not open anything)

      --write <PATH>
          Write the HTML to the given path (does not open anything)

      --lang <LANG>
          Language of the documentation

          Possible values:
          - en: English
          - fr: French

          [default: en]

  -h, --help
          Print help (see a summary with '-h')
```

Produit cette documentation sous la forme d'une page HTML autonome — CSS
intégré, aucun accès réseau — et l'ouvre dans votre navigateur. La page est
écrite dans `codev-docs-<version>.html`, dans le dossier temporaire du
système. `--write` l'écrit ailleurs sans l'ouvrir, pour la partager ;
`--print` affiche la source Markdown. `--lang` choisit l'édition anglaise
(`en`, par défaut) ou française (`fr`). `codev docs` fonctionne partout, y
compris en dehors d'un projet codev.

```bash
codev docs --lang fr --write ~/Downloads/codev-manual-fr.html
```

### codev completions

```text
Generate a shell completion script for local installation

The output goes to stdout — redirect it to your shell's completions folder. Typical setups:

- bash: `codev completions bash > ~/.local/share/bash-completion/completions/codev` - zsh: `codev completions zsh > "${fpath[1]}/_codev"`, then `compinit` - fish: `codev completions fish > ~/.config/fish/completions/codev.fish` - powershell: `codev completions powershell | Out-String | Invoke-Expression`

The command touches no file; it does not read `_codev/` either and works in any directory.

Usage: codev completions <SHELL>

Arguments:
  <SHELL>
          Target shell: bash, zsh, fish, powershell, elvish

          [possible values: bash, elvish, fish, powershell, zsh]

Options:
  -h, --help
          Print help (see a summary with '-h')
```

Voir [Complétion du shell](../installation.md#complétion-du-shell).

## Changes

### codev new change

```text
Create a change

Usage: codev new change [OPTIONS] <NAME>

Arguments:
  <NAME>  Name in kebab-case (`add-user-auth`)

Options:
      --schema <SCHEMA>  Workflow schema to use
      --goal <GOAL>      Goal, kept in the change's metadata
      --json
  -h, --help             Print help
```

Crée `_codev/changes/<NAME>/` avec un `change.yaml` qui enregistre le schéma,
la date de création et l'objectif. Le nom doit être en kebab-case : lettres
minuscules, chiffres et tirets simples. Par défaut, le schéma est celui de la
clé `schema:` de la configuration, à défaut `spec-driven`. Échoue avec
`change_exists` si le change existe déjà.

```bash
codev new change add-dark-mode --goal "Let users switch the UI to a dark theme"
```

### codev list

```text
List active changes, or specified capabilities with --specs

Usage: codev list [OPTIONS]

Options:
      --specs
      --json
  -h, --help   Print help
```

Liste les changes actifs ou, avec `--specs`, les capacités qui possèdent une
spec principale sous `_codev/specs/`.

### codev status

```text
Show the state of a change's artifacts

Usage: codev status [OPTIONS]

Options:
      --change <CHANGE>  Change name (inferred when there is only one)
      --json
  -h, --help             Print help
```

Affiche l'état de chaque artefact d'un change — terminé `[x]`, prêt `[ ]`,
bloqué `[-]` ou ignoré `[~]` — et indique si la planification est complète.
Échoue avec `no_active_change` lorsque le projet n'a aucun change actif.

### codev instructions

```text
Print everything needed to write an artifact

Usage: codev instructions [OPTIONS] [ARTIFACT]

Arguments:
  [ARTIFACT]  Artifact identifier (default: the next one to write)

Options:
      --change <CHANGE>
      --json
  -h, --help             Print help
```

Affiche tout ce dont un agent a besoin pour rédiger un artefact : où l'écrire,
l'instruction du schéma, le template, la langue des artefacts, le contexte et
les règles du projet avec leur provenance, les dépendances à lire au
préalable, et ce que l'artefact débloque. Pour l'artefact `design`, la
commande liste aussi les décisions d'architecture en vigueur. Sans
`ARTIFACT`, elle choisit le premier artefact prêt à être rédigé.

Les skills l'appellent avec `--json` ; voir
[`codev instructions`](json-output.md#instructions).

### codev sync

```text
Merge a change's deltas into the main specs without archiving

Usage: codev sync [OPTIONS]

Options:
      --change <CHANGE>  Change name (inferred when there is only one)
      --json
  -h, --help             Print help
```

Valide le change, refuse de continuer si la validation signale une erreur,
puis fusionne ses deltas dans les specs principales, sans déplacer le change.
Relancée, la commande indique que les specs sont inchangées. Voir
[Sync](../workflow.md#sync).

### codev archive

```text
Merge, then move a change to the dated archive

Usage: codev archive [OPTIONS]

Options:
      --change <CHANGE>  Change name (inferred when there is only one)
      --json
  -h, --help             Print help
```

Valide le change, refuse de continuer si la validation signale une erreur,
fusionne ses deltas dans les specs principales, puis le déplace dans
`_codev/changes/archive/<YYYY-MM-DD>-<name>/`. Voir
[Archive](../workflow.md#archive).

### codev validate

```text
Check changes and specs for structural errors and consistency

Usage: codev validate [OPTIONS] [ITEM]

Arguments:
  [ITEM]  Name of a specific change or spec capability

Options:
      --all      Validate all active changes and all main specs
      --changes  Validate all active changes
      --specs    Validate all main specs
      --strict   Treat any finding (warnings included) as a reason for a non-zero exit code. Useful for CI and automation
      --json
  -h, --help     Print help
```

Sans `ITEM` ni option, valide tout : chaque change actif, chaque spec
principale et les sceaux des décisions. `ITEM` est le nom d'un change actif ou
le chemin d'une capacité (`ui/theme`).

Code de sortie :

| Résultat | Sans `--strict` | Avec `--strict` |
|---|---|---|
| Aucun constat | 0 | 0 |
| Avertissements uniquement | 0 | 1 |
| Au moins une erreur | 1 | 1 |

```text
change add-dark-mode — _codev/changes/add-dark-mode
  error   _codev/changes/add-dark-mode/change.yaml:1: zero_delta_without_marker — no delta file under `specs/` and `skip_specs: true` is not declared; add a delta or set `skip_specs: true` in change.yaml

decisions decisions — _codev/decisions
  ✓ no issues

2 item(s) validated, 1 finding(s) including 1 error(s).
```

Chaque constat porte un code stable. Voir
[Codes de validation](file-formats.md#codes-de-validation).

### codev schemas

```text
List the available workflow schemas

Usage: codev schemas [OPTIONS]

Options:
      --json
  -h, --help  Print help
```

Liste les schémas du projet, issus de `_codev/schemas/`, ainsi que le schéma
intégré `spec-driven`, avec l'ordre de leurs artefacts. Voir
[Schémas personnalisés](../guides/custom-schemas.md).

## Décisions

### codev decision

```text
Create, inspect and supersede architecture decisions

Usage: codev decision <COMMAND>

Commands:
  list       List local and inherited decisions
  show       Show a specific decision
  new        Create a new local decision
  supersede  Supersede a decision: mark it `superseded` and create a new one
  seal       Add or rewrite the seal of a local decision
  deviate    Record a local deviation from an inherited decision
  promote    Promote a `### Decision: <title>` block of a `design.md` to an ADR
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

Les décisions sont désignées par leur identifiant court (`0007`) ou leur
identifiant qualifié (`project/0007`, `path:~/shared/0100`,
`git:git@github.com:acme/shared.git/0100`). Voir
[Décision](../concepts.md#décision).

### codev decision list

```text
List local and inherited decisions

Usage: codev decision list [OPTIONS]

Options:
      --json
  -h, --help  Print help
```

Liste les décisions locales et héritées avec leur statut. Les décisions en
vigueur sont marquées `•`, les autres `–` ; une décision remplacée indique
celle qui l'a remplacée.

### codev decision show

```text
Show a specific decision

Usage: codev decision show [OPTIONS] <ID>

Arguments:
  <ID>  Short (`0007`) or qualified (`path:~/shared/0100`) identifier

Options:
      --json
  -h, --help  Print help
```

### codev decision new

```text
Create a new local decision

Usage: codev decision new [OPTIONS] <TITLE>

Arguments:
  <TITLE>  Free-form title — slugified for the file name

Options:
      --status <STATUS>  Initial status [default: accepted]
      --json
  -h, --help             Print help
```

Crée `_codev/decisions/NNNN-<slug>.md` à partir du template d'ADR, avec le
prochain numéro libre. Avec le statut par défaut `accepted`, la décision est
scellée immédiatement ; utilisez `--status proposed` pour la rédiger d'abord,
puis `codev decision seal` une fois qu'elle est acceptée.

### codev decision supersede

```text
Supersede a decision: mark it `superseded` and create a new one

Usage: codev decision supersede [OPTIONS] <OLD_ID> <NEW_TITLE>

Arguments:
  <OLD_ID>     Identifier of the decision to supersede (short or qualified)
  <NEW_TITLE>  Title of the new decision

Options:
      --json
  -h, --help  Print help
```

Crée une nouvelle décision acceptée dont le champ `supersedes` pointe vers
`OLD_ID`, et passe le statut de l'ancienne décision à `superseded`. Le contenu
de l'ancienne décision n'est pas modifié : son sceau reste donc valide. Pour
vous écarter d'une décision héritée, utilisez `codev decision deviate`.

### codev decision seal

```text
Add or rewrite the seal of a local decision

Without `--force`: refuses if a seal exists and the body has changed. With `--force`: rewrites the seal (use after a deliberate edit of the body).

Usage: codev decision seal [OPTIONS] <ID>

Arguments:
  <ID>
          Short identifier (`0007`) — inherited ones (`path:` / `git:`) are refused

Options:
      --force
          Rewrite an existing seal even if the body has changed

      --json


  -h, --help
          Print help (see a summary with '-h')
```

Enregistre l'empreinte SHA-256 du contenu de la décision dans
`_codev/decisions/seal.yaml`. Sceller une décision déjà scellée et inchangée
n'a aucun effet.

### codev decision deviate

```text
Record a local deviation from an inherited decision

Creates a local `accepted` ADR that explicitly references the inherited decision being departed from. The inherited decision stays visible in `codev decision list`, but disappears from the instructions injected into the `design` artifact. To deviate from a local decision, use `codev decision supersede`.

Usage: codev decision deviate [OPTIONS] <TARGET> <NEW_TITLE>

Arguments:
  <TARGET>
          Qualified identifier of the inherited decision to set aside

          Example: `path:~/shared/0100` or `git:git@github.com:acme/shared.git/0100`.

  <NEW_TITLE>
          Free-form title of the local deviation — slugified

Options:
      --json


  -h, --help
          Print help (see a summary with '-h')
```

### codev decision promote

```text
Promote a `### Decision: <title>` block of a `design.md` to an ADR

Extracts the block's content, creates a sealed local ADR, and replaces the block's body with a textual reference to the new ADR. Refuses an archived change.

Usage: codev decision promote [OPTIONS] <CHANGE> <TITLE>

Arguments:
  <CHANGE>
          Name of the active change whose `design.md` holds the block

  <TITLE>
          Exact title of the block to promote (what follows `Decision: `)

Options:
      --json


  -h, --help
          Print help (see a summary with '-h')
```

```bash
codev decision promote add-audit-log "Append-only audit table"
```

## Héritage

### codev sources

```text
Manage inherited sources (local paths and remote git repositories)

Usage: codev sources <COMMAND>

Commands:
  list    List all declared sources with their state
  update  Resolve refs, download, and update `_codev/codev.lock`
  show    Show the details of a specific source
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

Voir [Sources héritées](../guides/inherited-sources.md).

### codev sources list

```text
List all declared sources with their state

Usage: codev sources list [OPTIONS]

Options:
      --json
  -h, --help  Print help
```

### codev sources show

```text
Show the details of a specific source

Usage: codev sources show [OPTIONS] <TARGET>

Arguments:
  <TARGET>  URL (`git:` source) or path (`path:` source)

Options:
      --json
  -h, --help  Print help
```

### codev sources update

```text
Resolve refs, download, and update `_codev/codev.lock`

Usage: codev sources update [OPTIONS]

Options:
      --json
  -h, --help  Print help
```

La seule commande qui accède au réseau. Nécessite `git`.
