# Interface en ligne de commande

Ce chapitre documente chaque commande `compagnon`. L'aide de chaque commande est
reproduite telle que l'affiche `compagnon <command> --help` ; lancez cette commande
pour consulter l'aide de la version que vous avez installée.

```bash
compagnon --help
```

```text
compagnon adds a thin layer of specs to a repository so that you and your agent agree on what must be built before a single line of code is written.

The commands below run in your terminal. The workflows are invoked in the Claude Code chat: /compagnon-propose, /compagnon-explore.

Usage: compagnon <COMMAND>

Commands:
  init          Initialize compagnon in a project and install the Claude Code skills
  update        Regenerate the skills, and refresh compagnon's managed sections, after a compagnon upgrade
  self-update   Download and install the latest compagnon release, verified
  doctor        Check what the project's compagnon setup still lacks, and how to fix it
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
  docs          Open the compagnon documentation in the browser
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

- **Découverte du projet.** Compagnon cherche `_compagnon/` dans le répertoire
  courant, puis dans chacun des répertoires parents. Lancez les commandes
  depuis n'importe quel endroit du projet. En dehors d'un projet, les
  commandes échouent avec `no_compagnon_root`.
- **`--json`.** Toute commande qui lit ou écrit le projet accepte `--json`. La
  sortie standard contient alors exactement un document JSON — y compris en
  cas d'échec — et les messages destinés aux humains ne sont pas affichés.
  Voir [Sortie JSON](json-output.md).
- **Code de sortie.** `0` en cas de succès, `1` en cas d'échec. Pour
  `compagnon validate`, voir [la section dédiée](#compagnon-validate).
- **Erreurs.** En mode lisible, les erreurs sont écrites sur la sortie
  d'erreur sous la forme `error: …`, souvent suivies d'une ligne `help: …`
  qui indique la correction.
- **Choix de la modification.** Les commandes qui acceptent `--change <CHANGE>` la
  déduisent lorsque le projet compte exactement une modification active, et échouent
  avec `ambiguous_change` lorsqu'il y en a plusieurs.

## Projet

### compagnon init

```text
Initialize compagnon in a project and install the Claude Code skills

Usage: compagnon init [OPTIONS] [PATH]

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
          Language skills write artifacts in — their prose, and their structural keywords when compagnon ships the language — as an ISO 639 code (`en`, `fr`, `pt-BR`…). Default: detected from the locale, otherwise `en`

      --inherit <SOURCE>
          Inherit conventions from a source, written under `inherits:` (repeatable). A git URL (`git@…`, `https://…`, `ssh://…`, `file://…`, or ending in `.git`), optionally suffixed with `@<ref>` (default `main`), or a local folder. Git sources are pinned in `_compagnon/compagnon.lock` right away

      --no-claude-md
          Do not write compagnon's section in `CLAUDE.md`

      --no-ci
          Do not write `.github/workflows/compagnon.yml`

      --no-pr-template
          Do not write compagnon's section in the pull request template

      --no-integration
          Skip all repository integration: same as `--no-claude-md --no-ci --no-pr-template`

      --json


  -h, --help
          Print help (see a summary with '-h')
```

Initialise Compagnon dans `PATH` (par défaut, le dossier courant) : crée
l'arborescence `_compagnon/`, écrit `_compagnon/config.yaml` et installe les compétences
dans `.claude/skills/`. Voir
[Ce que fait `compagnon init`](#ce-que-fait-compagnon-init) pour la
détection et les questions.

- `--yes` saute toutes les questions et applique les valeurs par défaut et les
  valeurs détectées. Lorsque l'entrée standard n'est pas un terminal (CI,
  pipes), `--yes` est implicite.
- `--preset` répond à la question des déroulés : `full` installe les huit
  déroulés, `minimal` installe `propose`, `explore`, `onboard` et
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
  aussitôt épinglées dans `_compagnon/compagnon.lock`, comme le ferait
  `compagnon sources update` ; si cela échoue (réseau, identifiants, ref
  inconnue), init réussit quand même, avertit avec `inherit_pin_failed` et
  affiche `help: run compagnon sources update`. Une valeur invalide échoue avec
  `invalid_inherit` avant toute écriture. Voir
  [Sources héritées](../guides/inherited-sources.md).
- `--no-claude-md`, `--no-ci` et `--no-pr-template` écartent chacun un
  fichier d'intégration ; `--no-integration` écarte les trois. Voir
  [Intégration au dépôt et compagnon doctor](../guides/repository-integration.md).

Sauf refus de votre part, `compagnon init` écrit aussi la section de Compagnon dans
`CLAUDE.md` et, sur GitHub, `.github/workflows/compagnon.yml` et une section dans
le modèle de demande de fusion. Le récapitulatif indique pour chaque fichier s'il a
été créé, complété, rafraîchi, laissé à jour ou écarté, avec la raison ; pour
un dépôt GitLab, il affiche un job à coller dans `.gitlab-ci.yml`, que Compagnon
ne modifie jamais. La dernière ligne suggère `compagnon doctor` lorsqu'il manque
encore quelque chose.

Si `_compagnon/config.yaml` existe déjà, il reste intact : `compagnon init` se
contente de créer les dossiers manquants, d'installer ou de rafraîchir les
compétences, et d'ajouter ou de rafraîchir les fichiers d'intégration. Les sources
`--inherit` ne sont alors pas écrites ; l'avertissement `inherit_not_written`
les nomme pour que vous les ajoutiez à la main. Les compétences que vous avez
modifiées à la main sont préservées, sauf si vous passez `--force`.

#### Ce que fait `compagnon init`

1. **Il sonde l'environnement**, sans rien afficher : manifestes du projet
   (`Cargo.toml`, `package.json`, `pyproject.toml`, `go.mod`, `pom.xml`),
   fichier de licence, présence de `.github/workflows/`, votre locale, et les
   serveurs MCP déclarés dans `.mcp.json`, `.claude/settings.json` et
   `~/.claude.json`. `--no-detect` désactive cette détection.
2. **Il pose au plus deux questions** lorsqu'il s'exécute dans un terminal :
   quels déroulés installer (complet, minimal ou personnalisé), et un contexte
   libre pour les compétences. Une réponse vide à la question du contexte ouvre
   `$EDITOR` sur un squelette. Un serveur MCP Jira détecté est simplement
   confirmé, sans question. Avec `--yes`, ou lorsque l'entrée standard n'est
   pas un terminal (CI, pipes), aucune question n'est posée.
3. **Il crée l'arborescence `_compagnon/`** — `specs/`, `decisions/`, `changes/`,
   `changes/archive/`, `schemas/` — ainsi qu'un fichier `_compagnon/config.yaml`
   dans lequel chaque valeur détectée est accompagnée d'un commentaire
   indiquant sa provenance :

   ```yaml
   # compagnon configuration — generated by `compagnon init`.
   # Hand-editable: each non-trivial field carries its source as a comment.

   schema: spec-driven

   # Language skills write the artifacts in (proposal, design, tasks,
   # specs). Structural keywords such as `## Why` or `### Requirement:`
   # follow it when compagnon ships the language, and stay in English otherwise.
   # set with `compagnon init --language`
   language: fr

   workflows:
     - propose
     - explore
     - onboard
     - apply
     - sync
     - archive
     - update
     - configure
   ```

4. **Il installe les compétences** dans `.claude/skills/compagnon-<workflow>/SKILL.md`.
5. **Il intègre Compagnon au dépôt**, sauf si vous le refusez :
   - une section Compagnon dans `CLAUDE.md`, créé s'il n'existe pas, qui indique à
     Claude Code que les spécifications font foi et que tout changement de
     comportement passe par `/compagnon-propose` puis `/compagnon-apply` ;
   - sur GitHub, `.github/workflows/compagnon.yml`, qui exécute
     `compagnon validate --strict` à chaque push et demande de fusion ;
   - sur GitHub, une section Compagnon dans le modèle de demande de fusion.

   Le texte de Compagnon se trouve entre les marqueurs `<!-- compagnon:begin -->` et
   `<!-- compagnon:end -->` et ne remplace jamais le vôtre. `--no-claude-md`,
   `--no-ci`, `--no-pr-template` et `--no-integration` écartent ces fichiers.
   Voir [Intégration au dépôt et compagnon doctor](../guides/repository-integration.md).
6. **Il déclare les sources héritées** passées avec `--inherit`, et épingle
   aussitôt les sources git. Voir [Sources héritées](../guides/inherited-sources.md).

Si `_compagnon/config.yaml` existe déjà, `compagnon init` n'y touche pas : il se
contente d'installer ou de rafraîchir les compétences et l'intégration au dépôt.

### compagnon update

```text
Regenerate the skills, and refresh compagnon's managed sections, after a compagnon upgrade

Usage: compagnon update [OPTIONS]

Options:
      --force  Rewrite skills even if they were edited by hand
      --json
  -h, --help   Print help
```

Régénère les compétences listées par `workflows:` dans la configuration (les huit
en l'absence de la clé), par exemple après une mise à jour de Compagnon ou une
modification de la configuration `mcp:`. `compagnon update` réécrit une compétence
marquée d'une autre version ; parmi les compétences marquées de sa propre version,
il réécrit celle dont le contenu correspond encore à son `checksum` — un rendu
de Compagnon, d'une compilation antérieure ou pour l'autre mode de relecture — et
préserve celle qui n'y correspond plus, modifiée à la main, sauf si vous
passez `--force`. Une compétence sans checksum, rendue avant leur existence, est
préservée quand elle porte la marque de cette version et diffère du rendu par
plus que le mode de relecture : lancez une fois `compagnon update --force` pour la
doter d'un checksum — il écrase aussi une compétence que vous avez réellement
modifiée : vérifiez d'abord `git diff`, puis réappliquez vos modifications.
Une compétence préservée est signalée. Les compétences retirées de `workflows:` ne sont
pas supprimées. Un nom de déroulé inconnu est ignoré avec un avertissement
`unknown_workflow`. Les dossiers manquants de `_compagnon/` et un
`_compagnon/config.yaml` manquant sont recréés ; aucun fichier existant n'est
jamais écrasé.

`compagnon update` rafraîchit aussi les sections gérées par Compagnon dans `CLAUDE.md`
et dans le modèle de demande de fusion, ainsi que `.github/workflows/compagnon.yml`
lorsque sa première ligne est `# compagnon:managed` — ce qui aligne sa version
épinglée sur celle qui s'exécute. Il ne rafraîchit que ce qui existe : il ne
crée jamais ces fichiers et n'ajoute aucune section, si bien qu'un projet
initialisé avec `--no-integration` reste tel quel.

### compagnon migrate

```text
Convert a project set up before the rename to Compagnon

Usage: compagnon migrate [OPTIONS]

Options:
      --dry-run  Print the operations without performing them
      --json
  -h, --help     Print help (see more with '--help')
```

Convertit sur place un projet créé avec codev, le nom du produit avant la
0.7.0. Il trouve le projet depuis le dossier courant comme toute autre
commande, sous `_codev/` ou `_compagnon/`, puis :

- déplace `_codev/` vers `_compagnon/`, et `codev.lock` vers `compagnon.lock` ;
- renomme la clé `codev_version` en `compagnon_version` dans
  `_compagnon/config.yaml`, en conservant sa valeur, ses commentaires et tout
  le reste du fichier à l'octet près ;
- réécrit les marqueurs de format de `review.md` et `corrections.md`, ainsi
  que les marqueurs des sections gérées de `CLAUDE.md` et du modèle de demande
  de fusion ;
- remplace les compétences générées `codev-*` et `.github/workflows/codev.yml` par
  leurs versions Compagnon, puis rafraîchit le tout comme
  `compagnon update`.

Une compétence `codev-*` ou un `codev.yml` modifiés à la main sont conservés et
signalés. La prose des spécifications, des décisions et des modifications n'est jamais
réécrite. `--dry-run` affiche les opérations sans rien écrire. Une exécution
interrompue par une erreur (`migration_incomplete`) indique le chemin en
cause et ce qui était déjà fait : corrigez la cause et relancez la commande
pour terminer. Un projet déjà migré est laissé tel quel. La commande refuse,
sans rien écrire, un dossier qui contient à la fois `_codev/` et
`_compagnon/` (`migration_conflict`) et une exigence de version que le
Compagnon en cours d'exécution ne satisfait pas
(`migration_version_unsatisfied`). Lorsque le projet déclare des sources git,
elle vous rappelle de lancer `compagnon sources update`. Voir
[Migrer depuis codev](../guides/moving-from-codev.md).

### compagnon self-update

```text
Download and install the latest compagnon release, verified

Downloads the release archive for this platform from GitHub, verifies its SHA-256 against the release's `SHA256SUMS`, replaces this binary atomically, then runs the new binary's `compagnon update` in the current project, if any. Never downgrades unless `--version` asks for it. A binary installed by cargo, a cargo build or a package manager is left alone: the command that upgrades it is printed instead.

Usage: compagnon self-update [OPTIONS]

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

Installe un Compagnon plus récent à la place de celui qui s'exécute, en une
commande et jamais dans votre dos :

1. détermine la version cible — `--version`, ou la release la plus récente,
   demandée à GitHub sur le moment, quels que soient le cache, `CI` ou
   `COMPAGNON_NO_UPDATE_CHECK` ;
2. dans un terminal, demande confirmation, sauf avec `--yes` ;
3. télécharge `compagnon-<version>-<target>.tar.gz` (`.zip` sous Windows) et
   `SHA256SUMS` depuis la release GitHub, puis vérifie le SHA-256 de
   l'archive ;
4. écrit le nouveau binaire à côté de celui qui s'exécute, puis le renomme à
   sa place, si bien que le chemin du binaire contient toujours un binaire
   complet ;
5. dans un projet Compagnon, lance `compagnon update` avec le nouveau binaire.

Un binaire installé par `cargo install`, compilé dans un dossier `target/` de
cargo ou installé par un gestionnaire de paquets (Homebrew, Nix, Scoop,
`/usr/bin`) n'est pas touché : la commande qui le met à jour est affichée et
le code de sortie vaut `0`. Une release plus ancienne que le binaire en cours
est refusée (`update_downgrade_refused`), sauf si vous la demandez avec
`--version`.

`--check` affiche la version installée et la plus récente, rafraîchit le
cache de vérification et sort avec `0`, qu'une mise à jour soit disponible ou
non. `--snooze` fait taire, pendant 7 jours, l'annonce de `compagnon status` et
des compétences pour la dernière version connue ; une release plus récente est de
nouveau annoncée.

| Code | Signification |
|---|---|
| `update_check_failed` | L'API des releases est injoignable, ou ne renvoie aucune release |
| `update_unsupported_platform` | Aucun binaire publié pour cette plateforme ; compilez depuis les sources |
| `update_download_failed` | L'archive ou `SHA256SUMS` ne peut pas être téléchargé |
| `update_checksum_mismatch` | `SHA256SUMS` n'a pas d'entrée pour l'archive, ou son SHA-256 diffère |
| `update_extract_failed` | L'archive ne peut pas être extraite, ou ne contient aucun binaire Compagnon |
| `update_not_writable` | Le dossier du binaire n'est pas accessible en écriture |
| `update_downgrade_refused` | La cible est plus ancienne que la version en cours, sans `--version` |
| `invalid_version` | `--version` n'est pas de la forme `X.Y.Z` |

Tout échec laisse le binaire installé intact. Lorsque `compagnon update` échoue
dans le projet après une mise à jour réussie, la commande avertit
`update_project_refresh_failed` et sort tout de même avec `0` : lancez
`compagnon update` dans ce projet. Pour `--json`, voir
[Sortie JSON](json-output.md#self-update).

### compagnon doctor

```text
Check what the project's compagnon setup still lacks, and how to fix it

Read-only and offline: configuration, skills, inherited sources, the running compagnon version against `compagnon_version` and the cached update check, specs, decisions, `CLAUDE.md`, CI and pull request template, `compagnon validate`, and whether `CHANGELOG.md` matches the archived changes. Each check is ok, missing or a warning, with the command or skill that fixes it.

Usage: compagnon doctor [OPTIONS]

Options:
      --strict
          Exit with code 1 when a check is missing (warnings never fail)

      --json


  -h, --help
          Print help (see a summary with '-h')
```

Liste ce qui manque encore à l'installation de Compagnon dans le projet, regroupé
sous Configuration, Planning et Repository, avec la commande ou la compétence qui
corrige chaque point. La commande n'écrit rien et n'a pas besoin du réseau ;
la vérification `changelog` lance des requêtes `git` locales, en lecture
seule.

| Vérification | Réussie lorsque | Correction |
|---|---|---|
| `config.rules` | `rules:` a au moins une entrée, héritées comprises | `/compagnon-configure` |
| `config.language` | `language:` est défini | ajouter `language: <code>` |
| `config.keys` | `_compagnon/config.yaml` ne contient aucune clé que ce Compagnon ignore — simple avertissement | corriger ou retirer la clé, ou `compagnon self-update` |
| `config.roles` | chaque entrée de `roles:` est valide — informatif, il liste les rôles actifs, les rôles actifs selon le domaine avec leurs domaines, et les rôles désactivés ; sinon un avertissement `roles_invalid` | `compagnon validate` |
| `skills` | chaque compétence configurée est installée et générée par cette version de Compagnon | `compagnon update` |
| `sources` | chaque source git est épinglée dans `compagnon.lock` et présente dans le cache | `compagnon sources update` |
| `compagnon_version` | le Compagnon en cours satisfait `compagnon_version`, et le cache de vérification ne connaît aucune release plus récente — simple avertissement dans ce second cas | `compagnon self-update`, ou `compagnon self-update --version <X.Y.Z>` lorsque le Compagnon en cours dépasse l'exigence |
| `specs` | au moins une spécification principale existe | `/compagnon-propose` |
| `decisions` | au moins une décision est acceptée — simple avertissement | `compagnon decision new` |
| `validate` | `compagnon validate` ne signale rien | `compagnon validate` |
| `claude_md` | `CLAUDE.md` contient la section de Compagnon | `compagnon init` |
| `ci` | un fichier de déroulé GitHub Actions ou `.gitlab-ci.yml` exécute `compagnon validate` | `compagnon init` sur GitHub |
| `pr_template` | le modèle de demande de fusion contient la section de Compagnon — GitHub uniquement | `compagnon init` |
| `changelog` | `CHANGELOG.md` correspond à ce que génère `compagnon changelog` — simple avertissement | `compagnon changelog --write` |

La vérification `compagnon_version` lit le cache que tient `compagnon status` ;
elle ne contacte jamais le réseau.

Chaque ligne affiche `✓` (ok), `✗` (manquant) ou `!` (avertissement) ; la
dernière ligne compte les vérifications et celles qui restent à corriger. Le
code de sortie vaut `0`, sauf avec `--strict` lorsqu'une vérification manque :
les avertissements ne font jamais échouer la commande. Voir
[Intégration au dépôt et compagnon doctor](../guides/repository-integration.md)
et, pour `--json`, [Sortie JSON](json-output.md#doctor).

### compagnon docs

```text
Open the compagnon documentation in the browser

Three forms:

- default: writes `compagnon-docs-<version>.html` to the system temp folder and opens it in the browser; - `--write <PATH>`: writes the HTML to the given path, opens nothing (for sharing — email, Confluence, shared drive); - `--print`: prints the markdown source to stdout (to pipe into `less`, `bat` or an LLM).

Usage: compagnon docs [OPTIONS]

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
écrite dans `compagnon-docs-<version>.html`, dans le dossier temporaire du
système. `--write` l'écrit ailleurs sans l'ouvrir, pour la partager ;
`--print` affiche la source Markdown. `--lang` choisit l'édition anglaise
(`en`, par défaut) ou française (`fr`). `compagnon docs` fonctionne partout, y
compris en dehors d'un projet Compagnon.

`--print` est fait pour être redirigé : lorsque le lecteur s'arrête avant la
fin, comme dans `compagnon docs --print | head`, la commande s'arrête sans message
et renvoie le code 0. Dans la page, remarques et avertissements gardent leur
sens : une remarque porte un filet vert, un avertissement un filet corail.

```bash
compagnon docs --lang fr --write ~/Downloads/compagnon-manual-fr.html
```

### compagnon completions

```text
Generate a shell completion script for local installation

The output goes to stdout — redirect it to your shell's completions folder. Typical setups:

- bash: `compagnon completions bash > ~/.local/share/bash-completion/completions/compagnon` - zsh: `compagnon completions zsh > "${fpath[1]}/_compagnon"`, then `compinit` - fish: `compagnon completions fish > ~/.config/fish/completions/compagnon.fish` - powershell: `compagnon completions powershell | Out-String | Invoke-Expression`

The command touches no file; it does not read `_compagnon/` either and works in any directory.

Usage: compagnon completions <SHELL>

Arguments:
  <SHELL>
          Target shell: bash, zsh, fish, powershell, elvish

          [possible values: bash, elvish, fish, powershell, zsh]

Options:
  -h, --help
          Print help (see a summary with '-h')
```

Voir [Complétion du shell](../installation.md#complétion-du-shell).

## Modifications

### compagnon new change

```text
Create a change

Usage: compagnon new change [OPTIONS] <NAME>

Arguments:
  <NAME>  Name in kebab-case (`add-user-auth`)

Options:
      --schema <SCHEMA>  Workflow schema to use
      --goal <GOAL>      Goal, kept in the change's metadata
      --json
  -h, --help             Print help
```

Crée `_compagnon/changes/<NAME>/` avec un `change.yaml` qui enregistre le schéma,
la date de création et l'objectif, s'il est fourni. Le nom doit être en
kebab-case : lettres minuscules, chiffres et tirets simples, ni en tête ni en
fin ; il peut commencer par un chiffre. `archive` est réservé au dossier
des modifications archivées et refusé. Par défaut, le schéma est celui de la
clé `schema:` de la configuration, à défaut `spec-driven` ; `change.yaml`
enregistre le nom sous lequel le schéma a été trouvé — le nom du dossier pour
un schéma du projet. Échoue sans rien
écrire avec `invalid_change_id` — en nommant la règle que le nom enfreint —,
avec `change_exists` si la modification existe déjà, et avec `schema_not_found` si
le schéma n'existe pas.

```bash
compagnon new change add-dark-mode --goal "Let users switch the UI to a dark theme"
```

### compagnon list

```text
List active changes, or specified capabilities with --specs

Usage: compagnon list [OPTIONS]

Options:
      --specs
      --json
  -h, --help   Print help
```

Liste les modifications actives ou, avec `--specs`, les capacités qui possèdent une
spécification principale : chaque dossier de `_compagnon/specs/` contenant un fichier
nommé exactement `spec.md`, dossiers imbriqués compris. Les autres fichiers,
comme un `README.md` ou `notes/myspec.md`, ne sont pas des spécifications et sont
ignorés, par `compagnon validate` également.

### compagnon search

```text
Search the requirements and decisions that match a query

Usage: compagnon search [OPTIONS] <QUERY>...

Arguments:
  <QUERY>...  Words naming the behavior, in the language the specs are written in

Options:
      --limit <LIMIT>  Results to print, from 1 to 50 [default: 10]
      --archive        Also list the matching archived changes, in a section of their own
      --json
  -h, --help           Print help
```

Classe les exigences des spécifications principales — chacune avec sa description et
ses scénarios — et les décisions, héritées comprises, selon leur
correspondance avec la requête, et affiche les meilleures : le type,
l'identifiant (`<capacité>::<exigence>`, ou l'identifiant et le titre de la
décision avec son statut et si elle est en vigueur) et un extrait. La
casse, les accents et les mots vides sont ignorés, et les mots se
rapprochent par leur racine dans la langue des spécifications : écrivez la requête
dans cette langue, ainsi `decisions` trouve `decision`. La sortie contient
10 résultats, sauf si `--limit` en demande de 1 à 50, quelle que soit la
taille du projet. `--archive` ajoute les modifications archivées qui
correspondent, dans une section à part, jamais classées avec les exigences.
Une requête qui ne trouve rien le dit et se termine avec le code 0 ; une
requête faite seulement de mots vides échoue avec `search_query_empty`, et
un `--limit` hors limites avec `search_limit_invalid`. La commande ne fait
que lire.

```bash
compagnon search create a decision from the command line
compagnon search "decision seal" --archive
```

### compagnon show

```text
Print one requirement, or the requirement names of a capability

Usage: compagnon show [OPTIONS] <TARGET>

Arguments:
  <TARGET>  `<capability>::<requirement name>`, or `<capability>` for its index

Options:
      --json
  -h, --help  Print help
```

`compagnon show <capacité>::<nom de l'exigence>` affiche cette exigence telle que
sa spécification principale la contient — titre, description et scénarios, à
l'identique — et rien d'autre de la spécification ; le nom est comparé sans tenir
compte de la casse ni des espaces. `compagnon show <capacité>` affiche
l'objectif de la capacité et les noms de ses exigences, sous la forme
`<capacité>::<nom>` que la commande accepte. La capacité doit être l'une de
celles que nomme `compagnon list --specs` : toute autre valeur échoue avec
`unknown_capability`, et une exigence que la capacité ne contient pas avec
`unknown_requirement`, qui liste celles qu'elle contient.

```bash
compagnon show decisions
compagnon show "decisions::Creating a decision through the CLI"
```

### compagnon status

```text
Show the state of a change's artifacts

Usage: compagnon status [OPTIONS]

Options:
      --change <CHANGE>  Change name (inferred when there is only one)
      --json
  -h, --help             Print help
```

Affiche l'état de chaque artefact d'une modification — terminé `[x]`, prêt `[ ]`,
bloqué `[-]` avec les artefacts qu'il attend, ou ignoré `[~]` — puis
`Planning: <n>/<total> artifacts` et le prochain artefact à rédiger, ou
`Planning is complete.`. Échoue avec `no_active_change` lorsque le projet n'a
aucune modification active. Dans les deux cas, lorsque `compagnon doctor` signalerait une
vérification manquante, une dernière ligne `hint:` sur stderr suggère de le
lancer. Un `--change` qui nomme une modification inexistante échoue avec
`unknown_change` ; `--change archive` échoue avec `invalid_change_id`, car ce
nom est réservé aux modifications archivées.

Lorsqu'une release plus récente de Compagnon est connue et que vous ne l'avez pas
mise en sourdine, stderr porte aussi
``Compagnon X.Y.Z is available (you have A.B.C) — run `compagnon self-update` ``. Pour
le savoir, `compagnon status` demande à GitHub la dernière release au plus une
fois toutes les 24 heures, avec un délai de 2 secondes, et garde la réponse
en cache dans `~/.cache/compagnon/update-check.json`. Il ne vérifie jamais
lorsque `CI` est défini ou avec `COMPAGNON_NO_UPDATE_CHECK=1`, et une
vérification qui échoue reste silencieuse. Aucune autre commande ne vérifie.
Avec `--json`, la même information figure dans l'objet `update` — voir
[Sortie JSON](json-output.md#status).

### compagnon instructions

```text
Print everything needed to write an artifact

Usage: compagnon instructions [OPTIONS] [ARTIFACT]

Arguments:
  [ARTIFACT]  Artifact identifier (default: the next one to write)

Options:
      --change <CHANGE>
      --json
  -h, --help             Print help
```

Affiche tout ce dont un agent a besoin pour rédiger un artefact : où l'écrire,
l'instruction du schéma, le modèle, la langue des artefacts, le contexte et
les règles du projet avec leur provenance, les dépendances à lire au
préalable, et ce que l'artefact débloque. Pour l'artefact `design`, la
commande liste aussi les décisions d'architecture en vigueur, et le
[rôle](../roles.md) de l'artefact avec ses critères. Sans
`ARTIFACT`, elle choisit le premier artefact prêt à être rédigé. Elle n'écrit
aucun fichier.

Une fois la proposition de la modification écrite, les instructions des artefacts qui
la requièrent directement — `specs` et `design` — listent aussi les exigences
existantes que la modification touche probablement, classées par rapport au `Why`
et au `What Changes` de la proposition, au plus 10, chacune avec un extrait,
et indiquent qu'on en lit une avec `compagnon show <capacité>::<exigence>` et
qu'une exigence que la modification altère se modifie au lieu d'être ajoutée de
nouveau. Une spécification illisible est écartée avec un avertissement
`spec_unreadable`.

Pour un artefact désactivé par `skip_specs`, elle indique qu'il ne doit pas
être créé, avec un avertissement `artifact_skipped`. Elle échoue avec
`unknown_artifact` lorsque `ARTIFACT` n'est pas un artefact du schéma de la
modification, avec `no_artifact_ready` lorsque `ARTIFACT` est omis et qu'aucun
artefact n'est prêt, et avec `template_not_found` lorsqu'un schéma du projet
déclare un fichier de modèle qu'il ne contient pas, et avec `unknown_role`,
`invalid_role_activation` ou `role_not_disableable` lorsqu'un réglage de
rôle est invalide — comme `compagnon status`.

Les compétences l'appellent avec `--json` ; voir
[`compagnon instructions`](json-output.md#instructions).

### compagnon sync

```text
Merge a change's deltas into the main specs without archiving

Usage: compagnon sync [OPTIONS]

Options:
      --change <CHANGE>  Change name (inferred when there is only one)
      --json
  -h, --help             Print help
```

Valide la modification, refuse de continuer si la validation signale une erreur,
puis fusionne ses deltas dans les spécifications principales, sans déplacer la modification.
Relancée, la commande indique que les spécifications sont inchangées. Voir
[Synchroniser](../workflow.md#synchroniser).

### compagnon review

```text
Show a change's review record and what is left to decide

Reads `_compagnon/changes/<name>/review.md`: per step and per role, the findings with their severity, status and location, and who must act next (author, reviewer or human). Lists the record's defects, as `compagnon validate` does, and whether the review lets the change be archived. Exits with 1 when the record has an error. Also lists the author's corrections from `corrections.md`, apart from the record.

Usage: compagnon review [OPTIONS]

Options:
      --change <CHANGE>
          Change name (inferred when there is only one)

      --json


  -h, --help
          Print help (see a summary with '-h')
```

Lit le [compte rendu de relecture](../guides/review-record.md) de la modification et
l'affiche par étape et par rôle, avec, pour chaque constat, qui doit agir,
les défauts du compte rendu (les codes `review_*` de
[`compagnon validate`](#compagnon-validate)) et si la relecture permet d'archiver la
modification. Sans compte rendu, la commande le dit et réussit. Le verrou
d'archivage ne change pas le code de sortie : c'est `compagnon archive` qui
l'applique. Elle liste ensuite les [corrections](file-formats.md#correctionsmd)
de l'auteur sous `Corrections`, avec les défauts de leur fichier : elles ne
décident ni de l'archivage ni du code de sortie. JSON :
[`review`](json-output.md#review).

### compagnon archive

```text
Merge, then move a change to the dated archive

Usage: compagnon archive [OPTIONS]

Options:
      --change <CHANGE>  Change name (inferred when there is only one)
      --json
  -h, --help             Print help
```

Valide la modification, refuse de continuer si la validation signale une erreur,
puis vérifie son [compte rendu de relecture](../guides/review-record.md) et
refuse avec `review_blocked` s'il manque pour une étape où des rôles de
relecture étaient actifs ou si un constat de gravité haute est encore ouvert
ou contesté ; fusionne ensuite ses deltas dans les spécifications principales et la
déplace, compte rendu compris, dans
`_compagnon/changes/archive/<YYYY-MM-DD>-<name>/`. Voir
[Archiver](../workflow.md#archiver).

### compagnon changelog

```text
Generate release notes from the archived changes

Lists the release-note items of the changes archived after `--since` (default: the highest `v*` tag) and up to `--until` (default: the working tree), by category, breaking items first. A change archived without a release note falls back on its What Changes bullets, with a warning. `--write` keeps the `[Unreleased]` section of `CHANGELOG.md` up to date, in a managed section; with `--release`, it becomes a dated version block.

Usage: compagnon changelog [OPTIONS]

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
          Update `CHANGELOG.md` (created if missing); text outside compagnon's managed section is kept

  -h, --help
          Print help (see a summary with '-h')
```

Liste les éléments de note de version des modifications archivées dans un
intervalle git. Une modification en fait partie lorsque son dossier se trouve dans
`_compagnon/changes/archive/` à `--until` et ne s'y trouvait pas à `--since` :
c'est git qui en décide, pas les dates. Par défaut, `--since` est le tag `v*`
le plus élevé dans l'ordre des versions — inférieur à `--until` lorsque
`--until` est un tag de version — et, sans aucun tag, toutes les modifications
archivées sont listées. Hors d'un dépôt git, toutes les modifications archivées sont
listées, avec un avertissement.

Les éléments sont regroupés par catégorie dans l'ordre de Keep a Changelog,
les éléments incompatibles en premier, puis dans l'ordre d'archivage. Une
modification dont la note vaut `None` n'ajoute rien. Une modification archivée sans note de
version apporte les puces de sa section What Changes sous `Changed`, avec un
avertissement `release_note_missing` ; une modification dont les deltas suppriment
des exigences sans élément `Removed` ni élément incompatible reçoit un
avertissement `release_note_inconsistent`. Les avertissements vont sur
stderr et ne changent jamais le code de sortie. Les modifications listées sous
[`changelog.exclude`](configuration.md#changelog) sont laissées de côté.

```bash
compagnon changelog --since v1.3.0 --format markdown
```

```text
### Added

- `--format json` pour `compagnon export`.

### Fixed

- `compagnon export` ne perd plus la dernière ligne.
```

Avec `--write`, la commande met à jour `CHANGELOG.md` au lieu d'afficher le
texte : le bloc `[Unreleased]` vit dans une section gérée, juste après
l'en-tête du fichier, et le reste du fichier n'est jamais réécrit. Avec
`--release X.Y.Z`, le bloc devient `## [X.Y.Z] - <date>` sous une nouvelle
section `[Unreleased]` vide. `--write` est refusé avec `--format markdown`.
Voir [Notes de version et CHANGELOG](../guides/release-notes.md).

### compagnon validate

```text
Check changes and specs for structural errors and consistency

Usage: compagnon validate [OPTIONS] [ITEM]

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

Sans `ITEM` ni option, valide tout : chaque modification active, chaque spécification
principale et les sceaux des décisions. `ITEM` est le nom d'une modification active ou
le chemin d'une capacité (`ui/theme`).

Lorsque le Compagnon en cours ne satisfait pas le
[`compagnon_version`](configuration.md#compagnon_version) du projet,
`compagnon validate`, `compagnon sync` et `compagnon archive` refusent de s'exécuter :
elles ne vérifient et n'écrivent rien, et échouent avec
`compagnon_version_unsatisfied`, en nommant l'exigence, la version en cours et
la correction que donne `compagnon doctor`. Relevez un épinglage de CI
(`COMPAGNON_VERSION=`) en même temps que `compagnon_version`.

Code de sortie :

| Résultat | Sans `--strict` | Avec `--strict` |
|---|---|---|
| Aucun constat | 0 | 0 |
| Avertissements uniquement | 0 | 1 |
| Au moins une erreur | 1 | 1 |

```text
change add-dark-mode — _compagnon/changes/add-dark-mode
  error   _compagnon/changes/add-dark-mode/change.yaml:1: zero_delta_without_marker — no delta file under `specs/` and `skip_specs: true` is not declared; add a delta or set `skip_specs: true` in change.yaml

decisions decisions — _compagnon/decisions
  ✓ no issues

2 item(s) validated, 1 finding(s) including 1 error(s).
```

Chaque constat porte un code stable. Voir
[Codes de validation](file-formats.md#codes-de-validation).

### compagnon roles

```text
List the roles active at a workflow step, with their criteria

With `--at`, the roles active at that step and their "done when" criteria, each with its origin (`built-in`, `schema:<name>`, `project`). Without it, every role with its activation. The roles come from compagnon's built-in catalog and the schema in use, with the project's `roles:` applied, and, with `--change`, the change's `change.yaml` and the domains its proposal ticks, which activate the experts. With `--change`, also the change's diff since the default branch and the domains its experts' triggers suggest. The `By stack` part of the knowledge keeps the languages detected in the project.

Usage: compagnon roles [OPTIONS]

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

Liste les [rôles](../roles.md) qu'une compétence applique à une étape du
déroulé : avec `--at`, les rôles actifs à cette étape, chacun avec sa
question et ses critères « Done when » précédés de leur origine ; sans
`--at`, une ligne par rôle avec ses étapes ou son artefact et son
activation — `never (project)` lorsque votre configuration le désactive,
`on_domain: <domaine>` pour un expert, `cannot be disabled` pour les rôles
d'artefact, l'`analyst` et le `developer`. `--knowledge` affiche aussi les
connaissances de chaque rôle après ses critères, précédées d'une ligne qui
nomme les langages pour lesquels leur partie `By stack` est filtrée. Une
étape dont tous les rôles sont inactifs affiche
`No role is active at <step>: skip this step.` Elle n'écrit aucun fichier.

Avec `--change`, elle affiche d'abord le diff de la modification — la branche par
défaut dont elle part, les fichiers modifiés et lus, les fichiers écartés par
catégorie — ou la raison de son absence, puis une ligne par
[domaine suggéré](../roles.md#les-déclencheurs-et-les-domaines-suggérés) :
son état (`ticked`, `justified`, `unticked`), son premier déclencheur avec
sa source, et le fichier concerné. La lecture du diff n'exécute que des
commandes git locales ; hors de git, elle le signale et poursuit.

La partie `By stack` des connaissances garde les langages détectés à partir
des manifestes de dépendances du projet ; `--all-stacks` sert tous les
langages. Sans aucun manifeste, tout est servi.

Sans `--change`, elle applique le schéma configuré du projet et son
`roles:` ; contrairement aux autres commandes, elle ne déduit pas de modification.
Avec `--change`, elle applique le schéma de la modification, son `change.yaml` et les
domaines que coche sa proposition, qui activent les
[experts](../roles.md#experts-et-domaines) ; sans lui, les experts restent
inactifs. Elle échoue avec `unknown_step` lorsque `--at` n'est pas une des étapes, et avec
`unknown_role`, `invalid_role_activation` ou `role_not_disableable`
lorsqu'un réglage de rôle est invalide — voir
[Configurer les rôles](../roles.md#configurer-les-rôles). Avec `--at`,
une dernière ligne nomme les rôles de l'étape que la configuration a
désactivés, chacun avec le fichier dont le réglage l'a fait (`project` ou
`change`). Les compétences l'appellent avec `--json` ; voir
[`compagnon roles`](json-output.md#roles).

### compagnon schemas

```text
List the available workflow schemas

Usage: compagnon schemas [OPTIONS]

Options:
      --json
  -h, --help  Print help
```

Liste les schémas du projet, issus de `_compagnon/schemas/`, ainsi que le schéma
intégré `spec-driven`, avec l'ordre de leurs artefacts. Un schéma du projet
nommé `spec-driven` remplace le schéma intégré. Un schéma du projet
inutilisable est écarté de la liste avec un avertissement `schema_unusable`
qui en donne la raison ; les autres restent listés. Un schéma du projet
porte le nom de son dossier : celui dont le `name:` indique autre chose est
listé sous le nom de son dossier, avec un avertissement
`schema_name_mismatch`. Voir
[Schémas personnalisés](../guides/custom-schemas.md).

## Décisions

### compagnon decision

```text
Create, accept, inspect and supersede architecture decisions

Usage: compagnon decision <COMMAND>

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

### compagnon decision list

```text
List local and inherited decisions

Usage: compagnon decision list [OPTIONS]

Options:
      --json
  -h, --help  Print help
```

Liste les décisions locales et héritées avec leur statut. Les décisions en
vigueur sont marquées `•`, les autres `–` ; une décision remplacée indique
celle qui l'a remplacée.

### compagnon decision show

```text
Show a specific decision

Usage: compagnon decision show [OPTIONS] <ID>

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

### compagnon decision new

```text
Create a new local decision

The decision is created `proposed` and unsealed, so its body can be written freely; `compagnon decision accept` then accepts and seals it.

Usage: compagnon decision new [OPTIONS] <TITLE>

Arguments:
  <TITLE>
          Free-form title — slugified for the file name

Options:
      --status <STATUS>
          Initial status; `accepted` seals the body right away, `proposed` leaves it editable until `compagnon decision accept`

          [default: proposed]

      --json


  -h, --help
          Print help (see a summary with '-h')
```

Crée `_compagnon/decisions/NNNN-<slug>.md` à partir du modèle d'ADR, avec le
prochain numéro libre. Avec le statut par défaut `proposed`, la décision n'est
pas scellée : rédigez son contenu, puis lancez `compagnon decision accept` pour
l'accepter et la sceller. `--status accepted` la scelle immédiatement, pour une
décision dont le texte est déjà définitif.

### compagnon decision accept

```text
Accept a proposed decision: set its status to `accepted` and seal it

Rewrites the frontmatter status of a local `proposed` decision and records the hash of its body in `seal.yaml`, in the same plan — all writes happen or none does. The body is left untouched. The decisions listed in its `supersedes` are marked `superseded` in the same plan; each must still be `accepted`. This is how a decision created by `new`, `supersede`, `deviate` or `promote` takes effect. Inherited decisions and decisions that are not `proposed` are refused.

Usage: compagnon decision accept [OPTIONS] <ID>

Arguments:
  <ID>
          Short identifier (`0007`) — inherited ones (`path:` / `git:`) are refused

Options:
      --json


  -h, --help
          Print help (see a summary with '-h')
```

Passe le statut d'une décision locale `proposed` à `accepted` et enregistre
l'empreinte SHA-256 de son contenu dans `_compagnon/decisions/seal.yaml`, dans le
même plan : les deux sont écrits, ou aucun. Seul le frontmatter change. Une
décision héritée est refusée avec `cannot_accept_inherited`, une décision qui
n'est pas `proposed` avec `decision_not_proposed` ; pour remplacer une décision
acceptée, utilisez `compagnon decision supersede`.

C'est la seule commande qui fait entrer une décision en vigueur, quelle que soit
la commande qui l'a créée. Quand la décision liste des prédécesseurs dans
`supersedes` — comme celle que crée `compagnon decision supersede` —, le même plan
passe chacun d'eux à `superseded` ; leur contenu, et donc leur sceau, n'est pas
modifié. Un prédécesseur qui n'est plus `accepted`, par exemple parce qu'une
autre décision l'a remplacé entre-temps, est refusé avec
`predecessor_not_accepted`, et rien n'est écrit.

```bash
compagnon decision accept 0002
```

```text
✓ Accepted 0002 — sealed (sha256:c8873b83c178dbf8434045f43a722911b9d9913ee9f13915ccda893539b788c1)
  File: /home/you/acme-app/_compagnon/decisions/0002-use-sqlite.md
  Superseded: project/0001 (status: superseded)
```

### compagnon decision supersede

```text
Create a proposed decision that will supersede an accepted one

The new decision is created `proposed` and unsealed, with the old one in its `supersedes`. The old decision is not modified and stays in effect until `compagnon decision accept` accepts the new one, which marks the old one `superseded` in the same step.

Usage: compagnon decision supersede [OPTIONS] <OLD_ID> <NEW_TITLE>

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
`compagnon decision accept`, qui la passe alors à `superseded` dans la même étape.
`OLD_ID` doit être une décision locale `accepted` ; sinon, la commande refuse
avec `predecessor_not_accepted`. Pour vous écarter d'une décision héritée,
utilisez `compagnon decision deviate`.

```bash
compagnon decision supersede 0001 "Use SQLite"
```

```text
✓ Decision `0002 Use SQLite` created (proposed), to supersede `project/0001`
  New ADR:     /home/you/acme-app/_compagnon/decisions/0002-use-sqlite.md
  Old ADR:     /home/you/acme-app/_compagnon/decisions/0001-use-postgresql.md (unchanged, still in effect)

Open the new file to write the Context, Decision and Consequences sections,
then run `compagnon decision accept 0002` to accept and seal it.
Accepting it marks `project/0001` superseded; until then it stays in effect.
```

### compagnon decision seal

```text
Add or rewrite the seal of a local decision

Without `--force`: refuses if a seal exists and the body has changed. With `--force`: rewrites the seal (use after a deliberate edit of the body).

Usage: compagnon decision seal [OPTIONS] <ID>

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
`_compagnon/decisions/seal.yaml`. Sceller une décision déjà scellée et inchangée
n'a aucun effet.

### compagnon decision deviate

```text
Record a local deviation from an inherited decision

Creates a local `proposed` ADR, unsealed, that explicitly references the inherited decision being departed from. The deviation takes effect once `compagnon decision accept` accepts it: the inherited decision then stays visible in `compagnon decision list`, but disappears from the instructions injected into the `design` artifact. To deviate from a local decision, use `compagnon decision supersede`.

Usage: compagnon decision deviate [OPTIONS] <TARGET> <NEW_TITLE>

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
que vous acceptiez l'écart avec `compagnon decision accept`. Voir
[Décisions héritées](../guides/inherited-sources.md#décisions-héritées).

### compagnon decision promote

```text
Promote a `### Decision: <title>` block of a `design.md` to an ADR

Extracts the block's content into a `proposed` local ADR, unsealed, and replaces the block's body with a textual reference to the new ADR. Review and rework the ADR, then accept and seal it with `compagnon decision accept`. Refuses an archived change.

Usage: compagnon decision promote [OPTIONS] <CHANGE> <TITLE>

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
Retravaillez-le, puis acceptez-le et scellez-le avec `compagnon decision accept` :

```bash
compagnon decision promote add-audit-log "Append-only audit table"
```

```text
✓ Decision `Append-only audit table` promoted to proposed ADR 0004
  File:    /home/you/acme-app/_compagnon/decisions/0004-append-only-audit-table.md
  Source:  /home/you/acme-app/_compagnon/changes/add-audit-log/design.md

Review the ADR promoted from change `add-audit-log`: split its body into Context,
Decision, Consequences and Alternatives considered,
then run `compagnon decision accept 0004` to accept and seal it.
```

## Héritage

### compagnon sources

```text
Manage inherited sources (local paths and remote git repositories)

Usage: compagnon sources <COMMAND>

Commands:
  list    List all declared sources with their state
  update  Resolve refs, download, and update `_compagnon/compagnon.lock`
  show    Show the details of a specific source
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

Voir [Sources héritées](../guides/inherited-sources.md).

### compagnon sources list

```text
List all declared sources with their state

Usage: compagnon sources list [OPTIONS]

Options:
      --json
  -h, --help  Print help
```

### compagnon sources show

```text
Show the details of a specific source

Usage: compagnon sources show [OPTIONS] <TARGET>

Arguments:
  <TARGET>  URL (`git:` source) or path (`path:` source)

Options:
      --json
  -h, --help  Print help
```

### compagnon sources update

```text
Resolve refs, download, and update `_compagnon/compagnon.lock`

Usage: compagnon sources update [OPTIONS]

Options:
      --json
  -h, --help  Print help
```

La seule commande qui accède au réseau. Nécessite `git`.
