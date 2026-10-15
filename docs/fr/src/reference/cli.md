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
  update        Regenerate the skills, and refresh codev's managed sections, after a codev upgrade
  self-update   Download and install the latest codev release, verified
  doctor        Check what the project's codev setup still lacks, and how to fix it
  new           Create a new item
  list          List active changes, or specified capabilities with --specs
  search        Search the requirements and decisions that match a query
  show          Print one requirement, or the requirement names of a capability
  status        Show the state of a change's artifacts
  instructions  Print everything needed to write an artifact
  roles         List the roles active at a workflow step, with their criteria
  schemas       List the available workflow schemas
  decision      Create, accept, inspect and supersede architecture decisions
  sources       Manage inherited sources (local paths and remote git repositories)
  sync          Merge a change's deltas into the main specs without archiving
  review        Show a change's review record and what is left to decide
  archive       Merge, then move a change to the dated archive
  changelog     Generate release notes from the archived changes
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

      --inherit <SOURCE>
          Inherit conventions from a source, written under `inherits:` (repeatable). A git URL (`git@…`, `https://…`, `ssh://…`, `file://…`, or ending in `.git`), optionally suffixed with `@<ref>` (default `main`), or a local folder. Git sources are pinned in `_codev/codev.lock` right away

      --no-claude-md
          Do not write codev's section in `CLAUDE.md`

      --no-ci
          Do not write `.github/workflows/codev.yml`

      --no-pr-template
          Do not write codev's section in the pull request template

      --no-integration
          Skip all repository integration: same as `--no-claude-md --no-ci --no-pr-template`

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
- `--inherit <SOURCE>` déclare une source héritée sous `inherits:`, une fois
  par source. Une valeur qui commence par `git@`, `https://`, `http://`,
  `ssh://` ou `file://`, ou qui se termine par `.git`, est une source git ; un
  suffixe `@<ref>` choisit la branche ou le tag (`…/conventions.git@v2`),
  `main` à défaut. Toute autre valeur est un dossier local, qui doit exister ;
  un chemin relatif est écrit sous forme absolue. Les sources git sont
  aussitôt épinglées dans `_codev/codev.lock`, comme le ferait
  `codev sources update` ; si cela échoue (réseau, identifiants, ref
  inconnue), init réussit quand même, avertit avec `inherit_pin_failed` et
  affiche `help: run codev sources update`. Une valeur invalide échoue avec
  `invalid_inherit` avant toute écriture. Voir
  [Sources héritées](../guides/inherited-sources.md).
- `--no-claude-md`, `--no-ci` et `--no-pr-template` écartent chacun un
  fichier d'intégration ; `--no-integration` écarte les trois. Voir
  [Intégration au dépôt et codev doctor](../guides/repository-integration.md).

Sauf refus de votre part, `codev init` écrit aussi la section de codev dans
`CLAUDE.md` et, sur GitHub, `.github/workflows/codev.yml` et une section dans
le modèle de pull request. Le récapitulatif indique pour chaque fichier s'il a
été créé, complété, rafraîchi, laissé à jour ou écarté, avec la raison ; pour
un dépôt GitLab, il affiche un job à coller dans `.gitlab-ci.yml`, que codev
ne modifie jamais. La dernière ligne suggère `codev doctor` lorsqu'il manque
encore quelque chose.

Si `_codev/config.yaml` existe déjà, il reste intact : `codev init` se
contente de créer les dossiers manquants, d'installer ou de rafraîchir les
skills, et d'ajouter ou de rafraîchir les fichiers d'intégration. Les sources
`--inherit` ne sont alors pas écrites ; l'avertissement `inherit_not_written`
les nomme pour que vous les ajoutiez à la main. Les skills que vous avez
modifiées à la main sont préservées, sauf si vous passez `--force`.

### codev update

```text
Regenerate the skills, and refresh codev's managed sections, after a codev upgrade

Usage: codev update [OPTIONS]

Options:
      --force  Rewrite skills even if they were edited by hand
      --json
  -h, --help   Print help
```

Régénère les skills listées par `workflows:` dans la configuration (les huit
en l'absence de la clé), par exemple après une mise à jour de codev ou une
modification de la configuration `mcp:`. `codev update` réécrit une skill
marquée d'une autre version ; parmi les skills marquées de sa propre version,
il réécrit celle dont le contenu correspond encore à son `checksum` — un rendu
de codev, d'une compilation antérieure ou pour l'autre mode de revue — et
préserve celle qui n'y correspond plus, modifiée à la main, sauf si vous
passez `--force`. Une skill sans checksum, rendue avant leur existence, est
préservée quand elle porte la marque de cette version et diffère du rendu par
plus que le mode de revue : lancez une fois `codev update --force` pour la
doter d'un checksum — il écrase aussi une skill que vous avez réellement
modifiée : vérifiez d'abord `git diff`, puis réappliquez vos modifications.
Une skill préservée est signalée. Les skills retirées de `workflows:` ne sont
pas supprimées. Un nom de workflow inconnu est ignoré avec un avertissement
`unknown_workflow`. Les dossiers manquants de `_codev/` et un
`_codev/config.yaml` manquant sont recréés ; aucun fichier existant n'est
jamais écrasé.

`codev update` rafraîchit aussi les sections gérées par codev dans `CLAUDE.md`
et dans le modèle de pull request, ainsi que `.github/workflows/codev.yml`
lorsque sa première ligne est `# codev:managed` — ce qui aligne sa version
épinglée sur celle qui s'exécute. Il ne rafraîchit que ce qui existe : il ne
crée jamais ces fichiers et n'ajoute aucune section, si bien qu'un projet
initialisé avec `--no-integration` reste tel quel.

### codev self-update

```text
Download and install the latest codev release, verified

Downloads the release archive for this platform from GitHub, verifies its SHA-256 against the release's `SHA256SUMS`, replaces this binary atomically, then runs the new binary's `codev update` in the current project, if any. Never downgrades unless `--version` asks for it. A binary installed by cargo, a cargo build or a package manager is left alone: the command that upgrades it is printed instead.

Usage: codev self-update [OPTIONS]

Options:
      --version <X.Y.Z>
          Install this version (`X.Y.Z`) instead of the latest; allows a downgrade

      --check
          Only report the running and latest versions (exit code 0 whether or not an update is available)

      --snooze
          Silence the update notice for the latest known version, for 7 days

  -y, --yes
          Do not ask for confirmation

      --json
          JSON output; implies `--yes`

  -h, --help
          Print help (see a summary with '-h')
```

Installe un codev plus récent à la place de celui qui s'exécute, en une
commande et jamais dans votre dos :

1. détermine la version cible — `--version`, ou la release la plus récente,
   demandée à GitHub sur le moment, quels que soient le cache, `CI` ou
   `CODEV_NO_UPDATE_CHECK` ;
2. dans un terminal, demande confirmation, sauf avec `--yes` ;
3. télécharge `codev-<version>-<target>.tar.gz` (`.zip` sous Windows) et
   `SHA256SUMS` depuis la release GitHub, puis vérifie le SHA-256 de
   l'archive ;
4. écrit le nouveau binaire à côté de celui qui s'exécute, puis le renomme à
   sa place, si bien que le chemin du binaire contient toujours un binaire
   complet ;
5. dans un projet codev, lance `codev update` avec le nouveau binaire.

Un binaire installé par `cargo install`, compilé dans un dossier `target/` de
cargo ou installé par un gestionnaire de paquets (Homebrew, Nix, Scoop,
`/usr/bin`) n'est pas touché : la commande qui le met à jour est affichée et
le code de sortie vaut `0`. Une release plus ancienne que le binaire en cours
est refusée (`update_downgrade_refused`), sauf si vous la demandez avec
`--version`.

`--check` affiche la version installée et la plus récente, rafraîchit le
cache de vérification et sort avec `0`, qu'une mise à jour soit disponible ou
non. `--snooze` fait taire, pendant 7 jours, l'annonce de `codev status` et
des skills pour la dernière version connue ; une release plus récente est de
nouveau annoncée.

| Code | Signification |
|---|---|
| `update_check_failed` | L'API des releases est injoignable, ou ne renvoie aucune release |
| `update_unsupported_platform` | Aucun binaire publié pour cette plateforme ; compilez depuis les sources |
| `update_download_failed` | L'archive ou `SHA256SUMS` ne peut pas être téléchargé |
| `update_checksum_mismatch` | `SHA256SUMS` n'a pas d'entrée pour l'archive, ou son SHA-256 diffère |
| `update_extract_failed` | L'archive ne peut pas être extraite, ou ne contient aucun binaire codev |
| `update_not_writable` | Le dossier du binaire n'est pas accessible en écriture |
| `update_downgrade_refused` | La cible est plus ancienne que la version en cours, sans `--version` |
| `invalid_version` | `--version` n'est pas de la forme `X.Y.Z` |

Tout échec laisse le binaire installé intact. Lorsque `codev update` échoue
dans le projet après une mise à jour réussie, la commande avertit
`update_project_refresh_failed` et sort tout de même avec `0` : lancez
`codev update` dans ce projet. Pour `--json`, voir
[Sortie JSON](json-output.md#self-update).

### codev doctor

```text
Check what the project's codev setup still lacks, and how to fix it

Read-only and offline: configuration, skills, inherited sources, the running codev version against `codev_version` and the cached update check, specs, decisions, `CLAUDE.md`, CI and pull request template, `codev validate`, and whether `CHANGELOG.md` matches the archived changes. Each check is ok, missing or a warning, with the command or skill that fixes it.

Usage: codev doctor [OPTIONS]

Options:
      --strict
          Exit with code 1 when a check is missing (warnings never fail)

      --json


  -h, --help
          Print help (see a summary with '-h')
```

Liste ce qui manque encore à l'installation de codev dans le projet, regroupé
sous Configuration, Planning et Repository, avec la commande ou la skill qui
corrige chaque point. La commande n'écrit rien et n'a pas besoin du réseau ;
la vérification `changelog` lance des requêtes `git` locales, en lecture
seule.

| Vérification | Réussie lorsque | Correction |
|---|---|---|
| `config.rules` | `rules:` a au moins une entrée, héritées comprises | `/codev-configure` |
| `config.language` | `language:` est défini | ajouter `language: <code>` |
| `config.keys` | `_codev/config.yaml` ne contient aucune clé que ce codev ignore — simple avertissement | corriger ou retirer la clé, ou `codev self-update` |
| `config.roles` | chaque entrée de `roles:` est valide — informatif, il liste les rôles actifs, les rôles actifs selon le domaine avec leurs domaines, et les rôles désactivés ; sinon un avertissement `roles_invalid` | `codev validate` |
| `skills` | chaque skill configurée est installée et générée par cette version de codev | `codev update` |
| `sources` | chaque source git est épinglée dans `codev.lock` et présente dans le cache | `codev sources update` |
| `codev_version` | le codev en cours satisfait `codev_version`, et le cache de vérification ne connaît aucune release plus récente — simple avertissement dans ce second cas | `codev self-update`, ou `codev self-update --version <X.Y.Z>` lorsque le codev en cours dépasse l'exigence |
| `specs` | au moins une spec principale existe | `/codev-propose` |
| `decisions` | au moins une décision est acceptée — simple avertissement | `codev decision new` |
| `validate` | `codev validate` ne signale rien | `codev validate` |
| `claude_md` | `CLAUDE.md` contient la section de codev | `codev init` |
| `ci` | un workflow GitHub ou `.gitlab-ci.yml` exécute `codev validate` | `codev init` sur GitHub |
| `pr_template` | le modèle de pull request contient la section de codev — GitHub uniquement | `codev init` |
| `changelog` | `CHANGELOG.md` correspond à ce que génère `codev changelog` — simple avertissement | `codev changelog --write` |

La vérification `codev_version` lit le cache que tient `codev status` ;
elle ne contacte jamais le réseau.

Chaque ligne affiche `✓` (ok), `✗` (manquant) ou `!` (avertissement) ; la
dernière ligne compte les vérifications et celles qui restent à corriger. Le
code de sortie vaut `0`, sauf avec `--strict` lorsqu'une vérification manque :
les avertissements ne font jamais échouer la commande. Voir
[Intégration au dépôt et codev doctor](../guides/repository-integration.md)
et, pour `--json`, [Sortie JSON](json-output.md#doctor).

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

`--print` est fait pour être redirigé : lorsque le lecteur s'arrête avant la
fin, comme dans `codev docs --print | head`, la commande s'arrête sans message
et renvoie le code 0. Dans la page, remarques et avertissements gardent leur
sens : une remarque porte un filet vert, un avertissement un filet corail.

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
la date de création et l'objectif, s'il est fourni. Le nom doit être en
kebab-case : lettres minuscules, chiffres et tirets simples, ni en tête ni en
fin ; il peut commencer par un chiffre. `archive` est réservé au dossier
des changes archivés et refusé. Par défaut, le schéma est celui de la
clé `schema:` de la configuration, à défaut `spec-driven` ; `change.yaml`
enregistre le nom sous lequel le schéma a été trouvé — le nom du dossier pour
un schéma du projet. Échoue sans rien
écrire avec `invalid_change_id` — en nommant la règle que le nom enfreint —,
avec `change_exists` si le change existe déjà, et avec `schema_not_found` si
le schéma n'existe pas.

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
spec principale : chaque dossier de `_codev/specs/` contenant un fichier
nommé exactement `spec.md`, dossiers imbriqués compris. Les autres fichiers,
comme un `README.md` ou `notes/myspec.md`, ne sont pas des specs et sont
ignorés, par `codev validate` également.

### codev search

```text
Search the requirements and decisions that match a query

Usage: codev search [OPTIONS] <QUERY>...

Arguments:
  <QUERY>...  Words naming the behavior, in the language the specs are written in

Options:
      --limit <LIMIT>  Results to print, from 1 to 50 [default: 10]
      --archive        Also list the matching archived changes, in a section of their own
      --json
  -h, --help           Print help
```

Classe les exigences des specs principales — chacune avec sa description et
ses scénarios — et les décisions, héritées comprises, selon leur
correspondance avec la requête, et affiche les meilleures : le type,
l'identifiant (`<capacité>::<exigence>`, ou l'identifiant et le titre de la
décision avec son statut et si elle est en vigueur) et un extrait. La
casse, les accents et les mots vides sont ignorés, et les mots se
rapprochent par leur racine dans la langue des specs : écrivez la requête
dans cette langue, ainsi `decisions` trouve `decision`. La sortie contient
10 résultats, sauf si `--limit` en demande de 1 à 50, quelle que soit la
taille du projet. `--archive` ajoute les changes archivés qui
correspondent, dans une section à part, jamais classés avec les exigences.
Une requête qui ne trouve rien le dit et se termine avec le code 0 ; une
requête faite seulement de mots vides échoue avec `search_query_empty`, et
un `--limit` hors limites avec `search_limit_invalid`. La commande ne fait
que lire.

```bash
codev search create a decision from the command line
codev search "decision seal" --archive
```

### codev show

```text
Print one requirement, or the requirement names of a capability

Usage: codev show [OPTIONS] <TARGET>

Arguments:
  <TARGET>  `<capability>::<requirement name>`, or `<capability>` for its index

Options:
      --json
  -h, --help  Print help
```

`codev show <capacité>::<nom de l'exigence>` affiche cette exigence telle que
sa spec principale la contient — titre, description et scénarios, à
l'identique — et rien d'autre de la spec ; le nom est comparé sans tenir
compte de la casse ni des espaces. `codev show <capacité>` affiche
l'objectif de la capacité et les noms de ses exigences, sous la forme
`<capacité>::<nom>` que la commande accepte. La capacité doit être l'une de
celles que nomme `codev list --specs` : toute autre valeur échoue avec
`unknown_capability`, et une exigence que la capacité ne contient pas avec
`unknown_requirement`, qui liste celles qu'elle contient.

```bash
codev show decisions
codev show "decisions::Creating a decision through the CLI"
```

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
bloqué `[-]` avec les artefacts qu'il attend, ou ignoré `[~]` — puis
`Planning: <n>/<total> artifacts` et le prochain artefact à rédiger, ou
`Planning is complete.`. Échoue avec `no_active_change` lorsque le projet n'a
aucun change actif. Dans les deux cas, lorsque `codev doctor` signalerait une
vérification manquante, une dernière ligne `hint:` sur stderr suggère de le
lancer. Un `--change` qui nomme un change inexistant échoue avec
`unknown_change` ; `--change archive` échoue avec `invalid_change_id`, car ce
nom est réservé aux changes archivés.

Lorsqu'une release plus récente de codev est connue et que vous ne l'avez pas
mise en sourdine, stderr porte aussi
``codev X.Y.Z is available (you have A.B.C) — run `codev self-update` ``. Pour
le savoir, `codev status` demande à GitHub la dernière release au plus une
fois toutes les 24 heures, avec un délai de 2 secondes, et garde la réponse
en cache dans `~/.cache/codev/update-check.json`. Il ne vérifie jamais
lorsque `CI` est défini ou avec `CODEV_NO_UPDATE_CHECK=1`, et une
vérification qui échoue reste silencieuse. Aucune autre commande ne vérifie.
Avec `--json`, la même information figure dans l'objet `update` — voir
[Sortie JSON](json-output.md#status).

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
commande liste aussi les décisions d'architecture en vigueur, et le
[rôle](../roles.md) de l'artefact avec ses critères. Sans
`ARTIFACT`, elle choisit le premier artefact prêt à être rédigé. Elle n'écrit
aucun fichier.

Pour un artefact désactivé par `skip_specs`, elle indique qu'il ne doit pas
être créé, avec un avertissement `artifact_skipped`. Elle échoue avec
`unknown_artifact` lorsque `ARTIFACT` n'est pas un artefact du schéma du
change, avec `no_artifact_ready` lorsque `ARTIFACT` est omis et qu'aucun
artefact n'est prêt, et avec `template_not_found` lorsqu'un schéma du projet
déclare un fichier de template qu'il ne contient pas, et avec `unknown_role`,
`invalid_role_activation` ou `role_not_disableable` lorsqu'un réglage de
rôle est invalide — comme `codev status`.

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

### codev review

```text
Show a change's review record and what is left to decide

Reads `_codev/changes/<name>/review.md`: per step and per role, the findings with their severity, status and location, and who must act next (author, reviewer or human). Lists the record's defects, as `codev validate` does, and whether the review lets the change be archived. Exits with 1 when the record has an error.

Usage: codev review [OPTIONS]

Options:
      --change <CHANGE>
          Change name (inferred when there is only one)

      --json


  -h, --help
          Print help (see a summary with '-h')
```

Lit le [compte rendu de revue](../guides/review-record.md) du change et
l'affiche par étape et par rôle, avec, pour chaque constat, qui doit agir,
les défauts du compte rendu (les codes `review_*` de
[`codev validate`](#codev-validate)) et si la revue permet d'archiver le
change. Sans compte rendu, la commande le dit et réussit. Le verrou
d'archivage ne change pas le code de sortie : c'est `codev archive` qui
l'applique. JSON : [`review`](json-output.md#review).

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
puis vérifie son [compte rendu de revue](../guides/review-record.md) et
refuse avec `review_blocked` s'il manque pour une étape où des rôles de
relecture étaient actifs ou si un constat de gravité haute est encore ouvert
ou contesté ; fusionne ensuite ses deltas dans les specs principales et le
déplace, compte rendu compris, dans
`_codev/changes/archive/<YYYY-MM-DD>-<name>/`. Voir
[Archive](../workflow.md#archive).

### codev changelog

```text
Generate release notes from the archived changes

Lists the release-note items of the changes archived after `--since` (default: the highest `v*` tag) and up to `--until` (default: the working tree), by category, breaking items first. A change archived without a release note falls back on its What Changes bullets, with a warning. `--write` keeps the `[Unreleased]` section of `CHANGELOG.md` up to date, in a managed section; with `--release`, it becomes a dated version block.

Usage: codev changelog [OPTIONS]

Options:
      --since <REF>
          Git ref the range starts after (default: the highest `v*` tag below `--until`)

      --until <REF>
          Git ref the range ends at (default: the working tree)

      --format <FORMAT>
          Output format

          Possible values:
          - keep-a-changelog: `## [Unreleased]` (or the release) and `### <Category>` sections
          - markdown:         The same without the version heading, for a release body
          - json:             The JSON contract v1

          [default: keep-a-changelog]

      --release <VERSION>
          Title the block `## [<VERSION>] - <DATE>` instead of `## [Unreleased]`

      --date <DATE>
          Release date, `YYYY-MM-DD` (default: today)

      --write
          Update `CHANGELOG.md` (created if missing); text outside codev's managed section is kept

  -h, --help
          Print help (see a summary with '-h')
```

Liste les éléments de note de version des changes archivés dans un
intervalle git. Un change en fait partie lorsque son dossier se trouve dans
`_codev/changes/archive/` à `--until` et ne s'y trouvait pas à `--since` :
c'est git qui en décide, pas les dates. Par défaut, `--since` est le tag `v*`
le plus élevé dans l'ordre des versions — inférieur à `--until` lorsque
`--until` est un tag de version — et, sans aucun tag, tous les changes
archivés sont listés. Hors d'un dépôt git, tous les changes archivés sont
listés, avec un avertissement.

Les éléments sont regroupés par catégorie dans l'ordre de Keep a Changelog,
les éléments incompatibles en premier, puis dans l'ordre d'archivage. Un
change dont la note vaut `None` n'ajoute rien. Un change archivé sans note de
version apporte les puces de sa section What Changes sous `Changed`, avec un
avertissement `release_note_missing` ; un change dont les deltas suppriment
des exigences sans élément `Removed` ni élément incompatible reçoit un
avertissement `release_note_inconsistent`. Les avertissements vont sur
stderr et ne changent jamais le code de sortie. Les changes listés sous
[`changelog.exclude`](configuration.md#changelog) sont laissés de côté.

```bash
codev changelog --since v1.3.0 --format markdown
```

```text
### Added

- `--format json` pour `codev export`.

### Fixed

- `codev export` ne perd plus la dernière ligne.
```

Avec `--write`, la commande met à jour `CHANGELOG.md` au lieu d'afficher le
texte : le bloc `[Unreleased]` vit dans une section gérée, juste après
l'en-tête du fichier, et le reste du fichier n'est jamais réécrit. Avec
`--release X.Y.Z`, le bloc devient `## [X.Y.Z] - <date>` sous une nouvelle
section `[Unreleased]` vide. `--write` est refusé avec `--format markdown`.
Voir [Notes de version et CHANGELOG](../guides/release-notes.md).

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

### codev roles

```text
List the roles active at a workflow step, with their criteria

With `--at`, the roles active at that step and their "done when" criteria, each with its origin (`built-in`, `schema:<name>`, `project`). Without it, every role with its activation. The roles come from codev's built-in catalog and the schema in use, with the project's `roles:` applied, and, with `--change`, the change's `change.yaml` and the domains its proposal ticks, which activate the experts. With `--change`, also the change's diff since the default branch and the domains its experts' triggers suggest. The `By stack` part of the knowledge keeps the languages detected in the project.

Usage: codev roles [OPTIONS]

Options:
      --at <STEP>
          The workflow step: plan.understand, plan.investigate, plan.write, plan.trace, plan.challenge, code.implement or code.review

      --change <CHANGE>
          Apply this change's schema, `change.yaml` and domains

      --knowledge
          Also print each role's knowledge (always in the JSON, for active roles)

      --all-stacks
          Serve the knowledge's `By stack` part for every language, not only the languages detected in the project

      --json


  -h, --help
          Print help (see a summary with '-h')
```

Liste les [rôles](../roles.md) qu'une skill applique à une étape du
workflow : avec `--at`, les rôles actifs à cette étape, chacun avec sa
question et ses critères « Done when » précédés de leur origine ; sans
`--at`, une ligne par rôle avec ses étapes ou son artefact et son
activation — `never (project)` lorsque votre configuration le désactive,
`on_domain: <domaine>` pour un expert, `cannot be disabled` pour les rôles
d'artefact, l'`analyst` et le `developer`. `--knowledge` affiche aussi les
connaissances de chaque rôle après ses critères, précédées d'une ligne qui
nomme les langages pour lesquels leur partie `By stack` est filtrée. Une
étape dont tous les rôles sont inactifs affiche
`No role is active at <step>: skip this step.` Elle n'écrit aucun fichier.

Avec `--change`, elle affiche d'abord le diff du change — la branche par
défaut dont il part, les fichiers modifiés et lus, les fichiers écartés par
catégorie — ou la raison de son absence, puis une ligne par
[domaine suggéré](../roles.md#les-déclencheurs-et-les-domaines-suggérés) :
son état (`ticked`, `justified`, `unticked`), son premier déclencheur avec
sa source, et le fichier concerné. La lecture du diff n'exécute que des
commandes git locales ; hors de git, elle le signale et poursuit.

La partie `By stack` des connaissances garde les langages détectés à partir
des manifestes de dépendances du projet ; `--all-stacks` sert tous les
langages. Sans aucun manifeste, tout est servi.

Sans `--change`, elle applique le schéma configuré du projet et son
`roles:` ; contrairement aux autres commandes, elle ne déduit pas de change.
Avec `--change`, elle applique le schéma du change, son `change.yaml` et les
domaines que coche sa proposal, qui activent les
[experts](../roles.md#experts-et-domaines) ; sans lui, les experts restent
inactifs. Elle échoue avec `unknown_step` lorsque `--at` n'est pas une des étapes, et avec
`unknown_role`, `invalid_role_activation` ou `role_not_disableable`
lorsqu'un réglage de rôle est invalide — voir
[Configurer les rôles](../roles.md#configurer-les-rôles). Avec `--at`,
une dernière ligne nomme les rôles de l'étape que la configuration a
désactivés, chacun avec le fichier dont le réglage l'a fait (`project` ou
`change`). Les skills l'appellent avec `--json` ; voir
[`codev roles`](json-output.md#roles).

### codev schemas

```text
List the available workflow schemas

Usage: codev schemas [OPTIONS]

Options:
      --json
  -h, --help  Print help
```

Liste les schémas du projet, issus de `_codev/schemas/`, ainsi que le schéma
intégré `spec-driven`, avec l'ordre de leurs artefacts. Un schéma du projet
nommé `spec-driven` remplace le schéma intégré. Un schéma du projet
inutilisable est écarté de la liste avec un avertissement `schema_unusable`
qui en donne la raison ; les autres restent listés. Un schéma du projet
porte le nom de son dossier : celui dont le `name:` indique autre chose est
listé sous le nom de son dossier, avec un avertissement
`schema_name_mismatch`. Voir
[Schémas personnalisés](../guides/custom-schemas.md).

## Décisions

### codev decision

```text
Create, accept, inspect and supersede architecture decisions

Usage: codev decision <COMMAND>

Commands:
  list       List local and inherited decisions
  show       Show a specific decision
  new        Create a new local decision
  accept     Accept a proposed decision: set its status to `accepted` and seal it
  supersede  Create a proposed decision that will supersede an accepted one
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

Affiche un en-tête — l'identifiant court et qualifié, le titre, le statut, la
date, puis les tags, `Supersedes` et `Superseded by` lorsqu'ils sont
renseignés, et le chemin du fichier, relatif au projet ou absolu pour une
source héritée située en dehors — suivi du contenu du fichier. Échoue avec
`unknown_decision_id` lorsqu'aucune décision ne correspond, et avec
`ambiguous_decision_id`, en listant les candidates, lorsqu'un identifiant
court en désigne plusieurs : utilisez alors l'identifiant qualifié.

### codev decision new

```text
Create a new local decision

The decision is created `proposed` and unsealed, so its body can be written freely; `codev decision accept` then accepts and seals it.

Usage: codev decision new [OPTIONS] <TITLE>

Arguments:
  <TITLE>
          Free-form title — slugified for the file name

Options:
      --status <STATUS>
          Initial status; `accepted` seals the body right away, `proposed` leaves it editable until `codev decision accept`

          [default: proposed]

      --json


  -h, --help
          Print help (see a summary with '-h')
```

Crée `_codev/decisions/NNNN-<slug>.md` à partir du template d'ADR, avec le
prochain numéro libre. Avec le statut par défaut `proposed`, la décision n'est
pas scellée : rédigez son contenu, puis lancez `codev decision accept` pour
l'accepter et la sceller. `--status accepted` la scelle immédiatement, pour une
décision dont le texte est déjà définitif.

### codev decision accept

```text
Accept a proposed decision: set its status to `accepted` and seal it

Rewrites the frontmatter status of a local `proposed` decision and records the hash of its body in `seal.yaml`, in the same plan — all writes happen or none does. The body is left untouched. The decisions listed in its `supersedes` are marked `superseded` in the same plan; each must still be `accepted`. This is how a decision created by `new`, `supersede`, `deviate` or `promote` takes effect. Inherited decisions and decisions that are not `proposed` are refused.

Usage: codev decision accept [OPTIONS] <ID>

Arguments:
  <ID>
          Short identifier (`0007`) — inherited ones (`path:` / `git:`) are refused

Options:
      --json


  -h, --help
          Print help (see a summary with '-h')
```

Passe le statut d'une décision locale `proposed` à `accepted` et enregistre
l'empreinte SHA-256 de son contenu dans `_codev/decisions/seal.yaml`, dans le
même plan : les deux sont écrits, ou aucun. Seul le frontmatter change. Une
décision héritée est refusée avec `cannot_accept_inherited`, une décision qui
n'est pas `proposed` avec `decision_not_proposed` ; pour remplacer une décision
acceptée, utilisez `codev decision supersede`.

C'est la seule commande qui fait entrer une décision en vigueur, quelle que soit
la commande qui l'a créée. Quand la décision liste des prédécesseurs dans
`supersedes` — comme celle que crée `codev decision supersede` —, le même plan
passe chacun d'eux à `superseded` ; leur contenu, et donc leur sceau, n'est pas
modifié. Un prédécesseur qui n'est plus `accepted`, par exemple parce qu'une
autre décision l'a remplacé entre-temps, est refusé avec
`predecessor_not_accepted`, et rien n'est écrit.

```bash
codev decision accept 0002
```

```text
✓ Accepted 0002 — sealed (sha256:c8873b83c178dbf8434045f43a722911b9d9913ee9f13915ccda893539b788c1)
  File: /home/you/acme-app/_codev/decisions/0002-use-sqlite.md
  Superseded: project/0001 (status: superseded)
```

### codev decision supersede

```text
Create a proposed decision that will supersede an accepted one

The new decision is created `proposed` and unsealed, with the old one in its `supersedes`. The old decision is not modified and stays in effect until `codev decision accept` accepts the new one, which marks the old one `superseded` in the same step.

Usage: codev decision supersede [OPTIONS] <OLD_ID> <NEW_TITLE>

Arguments:
  <OLD_ID>
          Identifier of the decision to supersede (short or qualified)

  <NEW_TITLE>
          Title of the new decision

Options:
      --json


  -h, --help
          Print help (see a summary with '-h')
```

Crée une nouvelle décision `proposed`, non scellée, dont le champ `supersedes`
pointe vers `OLD_ID`. L'ancienne décision n'est pas modifiée : elle reste
`accepted`, et en vigueur, jusqu'à ce que vous acceptiez la nouvelle avec
`codev decision accept`, qui la passe alors à `superseded` dans la même étape.
`OLD_ID` doit être une décision locale `accepted` ; sinon, la commande refuse
avec `predecessor_not_accepted`. Pour vous écarter d'une décision héritée,
utilisez `codev decision deviate`.

```bash
codev decision supersede 0001 "Use SQLite"
```

```text
✓ Decision `0002 Use SQLite` created (proposed), to supersede `project/0001`
  New ADR:     /home/you/acme-app/_codev/decisions/0002-use-sqlite.md
  Old ADR:     /home/you/acme-app/_codev/decisions/0001-use-postgresql.md (unchanged, still in effect)

Open the new file to write the Context, Decision and Consequences sections,
then run `codev decision accept 0002` to accept and seal it.
Accepting it marks `project/0001` superseded; until then it stays in effect.
```

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

Creates a local `proposed` ADR, unsealed, that explicitly references the inherited decision being departed from. The deviation takes effect once `codev decision accept` accepts it: the inherited decision then stays visible in `codev decision list`, but disappears from the instructions injected into the `design` artifact. To deviate from a local decision, use `codev decision supersede`.

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

L'écart est créé `proposed` : la décision héritée reste en vigueur jusqu'à ce
que vous acceptiez l'écart avec `codev decision accept`. Voir
[Décisions héritées](../guides/inherited-sources.md#décisions-héritées).

### codev decision promote

```text
Promote a `### Decision: <title>` block of a `design.md` to an ADR

Extracts the block's content into a `proposed` local ADR, unsealed, and replaces the block's body with a textual reference to the new ADR. Review and rework the ADR, then accept and seal it with `codev decision accept`. Refuses an archived change.

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

L'ADR est créé `proposed`, avec le texte du bloc repris tel quel sous
`## Decision` et des emplacements à compléter pour les autres sections.
Retravaillez-le, puis acceptez-le et scellez-le avec `codev decision accept` :

```bash
codev decision promote add-audit-log "Append-only audit table"
```

```text
✓ Decision `Append-only audit table` promoted to proposed ADR 0004
  File:    /home/you/acme-app/_codev/decisions/0004-append-only-audit-table.md
  Source:  /home/you/acme-app/_codev/changes/add-audit-log/design.md

Review the ADR promoted from change `add-audit-log`: split its body into Context,
Decision, Consequences and Alternatives considered,
then run `codev decision accept 0004` to accept and seal it.
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
