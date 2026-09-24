# Changelog

Toutes les évolutions notables de codev sont listées ici.

Format : [Keep a Changelog](https://keepachangelog.com/fr/1.1.0/),
versionnage [SemVer](https://semver.org/lang/fr/).

Les notes détaillées de chaque version vivent dans la Release GitHub
correspondante — cette page en donne la vue résumée.

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
