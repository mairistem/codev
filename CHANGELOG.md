# Changelog

Toutes les évolutions notables de codev sont listées ici.

Format : [Keep a Changelog](https://keepachangelog.com/fr/1.1.0/),
versionnage [SemVer](https://semver.org/lang/fr/).

Les notes détaillées de chaque version vivent dans la Release GitHub
correspondante — cette page en donne la vue résumée.

## [0.3.1] — 2026-09-28

### Added

- **Skill `/codev-configure`** — la 8ème skill du cycle, qui enrichit
  `_codev/config.yaml` en analysant le projet. Elle lit `README.md`,
  `CONTRIBUTING.md`, `docs/`, un échantillon des fichiers source les
  plus édités, et rédige un patch pour `context:` (2-5 lignes) et
  `rules:` par artefact. Elle **affiche un diff** et n'écrit qu'après
  confirmation. Elle préserve strictement `schema`, `workflows`,
  `mcp` et `inherits` — c'est le complément à la sonde de `codev
  init`, pas son remplaçant.
- **Nudges automatiques** — `codev init` et `codev status` (cas
  « aucun change actif ») incitent explicitement à `/codev-configure`
  quand la config est **thin** (contexte < 200 caractères, aucune
  `rules:`). Détection uniforme via
  `codev-core::config::is_config_thin`. La sortie JSON reste
  inchangée — les nudges vivent en sortie humaine seulement.
- **Onboard prend en compte la config thin** — la skill
  `/codev-onboard` recommande `/codev-configure` en premier sur un
  projet fraîchement initialisé dont la config n'a pas encore été
  enrichie.

### Changed

- **Défaut de `codev init` et `codev update`** : passe de 7 à **8
  workflows** avec l'ajout de `configure`. Un projet qui restreint
  via `workflows:` explicite continue à obtenir seulement ce qu'il
  a demandé.

Notes complètes :
https://github.com/mairistem/codev/releases/tag/v0.3.1

## [0.3.0] — 2026-09-28

### Added

- **`codev init` interactif avec auto-détection** — au premier init sur
  un projet neuf, `codev init` sonde silencieusement l'environnement
  (stack via `Cargo.toml`/`package.json`/`pyproject.toml`/`go.mod`/`pom.xml`,
  licence, CI, MCPs configurés dans `.mcp.json`/`~/.claude.json`), puis
  pose au plus **deux** questions (workflows, contexte projet), et
  génère un `_codev/config.yaml` prérempli avec des **commentaires de
  provenance** au-dessus de chaque champ détecté.
- **Détection des MCPs Jira/Atlassian** — les serveurs déclarés dans
  les fichiers Claude Code sont matchés par regex (`jira`, `atlassian`)
  et convertis en tool_id via la convention Claude Code
  (`mcp__<name_normalized>__getJiraIssue`).
- **Flags CLI** — `codev init` accepte désormais `--yes` (`-y`),
  `--no-detect`, et `--preset <complet|minimal|personnalise>`.
  Sans TTY sur stdin, `--yes` est implicite (scriptabilité en CI).

### Changed

- **Défaut de `codev init` et `codev update`** : les **7 workflows**
  sont installés par défaut (propose, explore, onboard, apply, sync,
  archive, update), plus seulement les 3 premiers. Un projet qui veut
  moins de skills déclare `workflows:` explicite avec un sous-ensemble
  choisi (voie **opt-out**). Ce changement règle un problème de
  découverte : l'ancien défaut cachait `/codev-apply` derrière un
  opt-in que les nouveaux utilisateurs ne trouvaient jamais.

Notes complètes :
https://github.com/mairistem/codev/releases/tag/v0.3.0

## [0.2.2] — 2026-09-24

### Added

- **`docs/codev.md` §3.5** — nouveau tutoriel « Ta première évolution,
  en cinq minutes » : parcours complet propose → apply → valide →
  archive sur un cas concret (`codev list --json`), avec les deux
  voies (skill Claude Code et CLI pure) à chaque étape.
- **Trois diagrammes Mermaid** dans `docs/codev.md` : machine à
  états d'un change (§3), graphe de crates (§5), cycle de vie d'un
  delta (§5). GitHub les rend nativement ; le HTML embarqué de
  `codev docs` dégrade proprement en bloc de code (l'autonomie
  hors-ligne du HTML est préservée).

Notes complètes :
https://github.com/mairistem/codev/releases/tag/v0.2.2

## [0.2.1] — 2026-09-24

### Added

- Documents standards de projet open source : `CONTRIBUTING.md`,
  `CHANGELOG.md`, `CODE_OF_CONDUCT.md` (Contributor Covenant 2.1 FR),
  `SECURITY.md`, templates GitHub d'issues et de PR.
- Crédit explicite à [OpenSpec](https://github.com/Fission-AI/OpenSpec)
  en tête de `README.md` et via une nouvelle sous-section « Origines »
  de `docs/codev.md`.
- `README.md` : 4 badges (release / latest / license / plateformes),
  sommaire, section « Le cycle » avec schéma ASCII.

### Fixed

- Spec `distribution` — l'exigence « La documentation cite les
  trois voies d'installation » avait perdu son scénario au moment
  de l'ajout du support Windows, et sa dernière phrase était
  tronquée. Réparée. `codev validate --strict` est désormais vert.

Notes complètes :
https://github.com/mairistem/codev/releases/tag/v0.2.1

## [0.2.0] — 2026-09-24

### Added

- Cible **Windows x86_64-pc-windows-msvc** dans les binaires
  précompilés, avec le script `install.ps1` pour une installation
  sans Rust.
- **Documentation embarquée** : `codev docs` ouvre un manuel HTML
  autonome dans le navigateur, sans dépendance réseau.
- **Complétions shell** : `codev completions <shell>` pour bash,
  zsh, fish, powershell, elvish.
- **`codev docs --write PATH`** pour diffusion ciblée (impression,
  hébergement statique, etc.).
Notes complètes :
https://github.com/mairistem/codev/releases/tag/v0.2.0

## [0.1.1] — 2026-09-23

### Fixed

- CI Release : cross-compilation `x86_64-apple-darwin` depuis
  `macos-14`, les runners `macos-13` gratuits n'étant plus
  fiablement disponibles.

Notes complètes :
https://github.com/mairistem/codev/releases/tag/v0.1.1

## [0.1.0] — 2026-09-23

### Added

- Import initial : cycle propose → apply → sync → archive,
  7 skills Claude Code, gouvernance des décisions
  (K3 sceau, K6 déviation, K7 promotion), `codev validate --strict`,
  intégration MCP configurable côté projet, distribution
  précompilée macOS et Linux.

Notes complètes :
https://github.com/mairistem/codev/releases/tag/v0.1.0
