## MODIFIED Requirements

### Requirement: Un tag `v*.*.*` publie une GitHub Release avec les binaires cibles

Un workflow GitHub Actions SHALL être déclenché par un tag git au
format `v<major>.<minor>.<patch>` (par exemple `v0.2.0`). Le
workflow MUST produire une GitHub Release contenant :

- Un binaire précompilé par **cible supportée** : macOS arm64, macOS
  x86_64, Linux x86_64 musl statique, **Windows x86_64 MSVC**.
- Un format d'archive **par famille d'OS** : `.tar.gz` pour macOS
  et Linux (convention Unix, `tar` universel) ; `.zip` pour Windows
  (convention Windows, `Expand-Archive` natif dès PowerShell 5.1).
- Le nom de fichier suit `codev-<version>-<target>.<ext>` — par
  exemple `codev-0.2.0-x86_64-pc-windows-msvc.zip`.
- Un fichier `SHA256SUMS` qui liste le SHA-256 de **toutes** les
  archives (les quatre), au format `sha256sum` standard.
- Le contenu de chaque archive : le binaire (`codev` sur
  macOS/Linux, `codev.exe` sur Windows), `README.md`, `LICENSE`, et
  une copie du markdown source de la doc (`docs/codev.md`).

Le workflow MUST NOT écrire dans le dépôt.

#### Scenario: Un tag `v0.2.0` déclenche la release avec les quatre binaires

- **GIVEN** un développeur pousse un tag `v0.2.0` sur `main`
- **AND** la version dans `Cargo.toml` du workspace vaut `0.2.0`
- **WHEN** le workflow `.github/workflows/release.yml` s'exécute
- **THEN** une GitHub Release nommée `v0.2.0` est créée
- **AND** elle contient exactement quatre archives :
  `codev-0.2.0-aarch64-apple-darwin.tar.gz`,
  `codev-0.2.0-x86_64-apple-darwin.tar.gz`,
  `codev-0.2.0-x86_64-unknown-linux-musl.tar.gz`,
  `codev-0.2.0-x86_64-pc-windows-msvc.zip`
- **AND** elle contient un fichier `SHA256SUMS` qui liste les quatre
  archives avec leur SHA-256

#### Scenario: L'archive Windows contient `codev.exe`

- **GIVEN** l'archive `codev-<version>-x86_64-pc-windows-msvc.zip`
  de la release
- **WHEN** un utilisateur l'extrait sur Windows
- **THEN** un fichier `codev.exe` est présent
- **AND** les fichiers `README.md`, `LICENSE`, `codev.md` (doc) sont
  aussi présents

#### Scenario: Un push sans tag ne publie rien

- **GIVEN** un développeur pousse un commit sur `main` sans tag
- **WHEN** GitHub Actions s'exécute
- **THEN** aucune GitHub Release n'est créée
- **AND** aucun binaire n'est publié

### Requirement: La documentation cite les trois voies d'installation

Note : le titre historique reste « trois voies » pour préserver la
compatibilité de nom avec la spec principale ; le contenu ci-dessous
décrit **quatre** voies après ajout de Windows. Un renommage propre
viendra dans un cycle dédié.

La section Installation de `docs/codev.md` MUST citer, dans cet ordre :

1. **Voie recommandée Unix** — `curl -sSL … | sh` pour macOS/Linux.
2. **Voie recommandée Windows** — `iwr -useb … | iex` pour Windows
   dans PowerShell.
3. **Voie manuelle** — téléchargement depuis GitHub Releases + vérif
   `sha256sum -c` (Unix) ou `Get-FileHash` (Windows).
4. **Voie contributeur** — `cargo install --path crates/codev-cli`
   depuis un clone du dépôt.

Le README du dépôt MUST mentionner au moins la première **et** la
deuxième voie (avec les one-liners exacts), avec un lien vers
`docs/codev.md` pour le détail.

#### Scenario: Section Installation à jour avec les deux voies recommandées

- **GIVEN** la documentation générée par `codev docs`
- **WHEN** on cherche la section « Installation »
- **THEN** les quatre voies sont présentes dans l'ordre attendu
- **AND** la voie Windows cite exactement `iwr -useb https://…/install.ps1 | iex`
- **AND** la voie Unix cite exactement `curl -sSL https://…/install.sh | sh`

## ADDED Requirements

### Requirement: Le script `install.ps1` installe codev en une commande sur Windows

Un script `install.ps1` à la racine du dépôt SHALL permettre à un
utilisateur Windows d'installer codev via :

```powershell
iwr -useb https://raw.githubusercontent.com/mairistem/codev/main/install.ps1 | iex
```

Le script MUST :

- détecter l'architecture via `$env:PROCESSOR_ARCHITECTURE`
  (`AMD64` → target `x86_64-pc-windows-msvc`) ;
- résoudre la version : si `$env:CODEV_VERSION` est défini,
  l'utiliser ; sinon interroger l'API GitHub Releases pour la
  dernière (via `Invoke-RestMethod`) ;
- télécharger l'archive
  `codev-<version>-x86_64-pc-windows-msvc.zip` et le fichier
  `SHA256SUMS` via `Invoke-WebRequest` ;
- **vérifier le SHA-256** via `Get-FileHash -Algorithm SHA256` et
  refuser d'installer si divergence ;
- extraire via `Expand-Archive` ;
- copier `codev.exe` dans `$env:LOCALAPPDATA\Programs\codev\codev.exe`
  (création du dossier si absent) ;
- **afficher** les instructions pour ajouter
  `$env:LOCALAPPDATA\Programs\codev\` au PATH utilisateur (via
  `setx PATH` ou l'interface Système), **sans** modifier le PATH
  automatiquement.

Le script MUST refuser (exit code non nul) si :

- l'architecture n'est pas supportée (par exemple `ARM64` en V1) ;
- le SHA-256 vérifié ne correspond pas ;
- une commande PowerShell nécessaire (`Invoke-WebRequest`,
  `Expand-Archive`, `Get-FileHash`) n'est pas disponible.

#### Scenario: Installation Windows x86_64, PATH incomplet

- **GIVEN** un utilisateur Windows sur PowerShell 5.1+ (ou Core 7+)
- **AND** `$env:LOCALAPPDATA\Programs\codev\` n'est pas dans son
  PATH
- **WHEN** il lance
  `iwr -useb https://raw.githubusercontent.com/mairistem/codev/main/install.ps1 | iex`
- **THEN** le script détecte `x86_64-pc-windows-msvc`
- **AND** télécharge, vérifie SHA-256, extrait `codev.exe`
- **AND** copie `codev.exe` dans
  `$env:LOCALAPPDATA\Programs\codev\`
- **AND** affiche la commande exacte à taper pour ajouter le dossier
  au PATH utilisateur (par exemple `setx PATH "$env:PATH;$env:LOCALAPPDATA\Programs\codev"`)
- **AND** rappelle d'ouvrir un nouveau shell pour recharger le PATH

#### Scenario: Architecture non supportée refusée

- **GIVEN** un utilisateur Windows sur ARM64
- **WHEN** il lance le script
- **THEN** le script s'arrête avec un exit code non nul
- **AND** le message d'erreur nomme l'architecture détectée et
  renvoie vers `cargo install --path` comme fallback

#### Scenario: SHA-256 corrompu → refus

- **GIVEN** un environnement où le fichier téléchargé serait altéré
- **WHEN** le script compare `Get-FileHash` au `SHA256SUMS`
- **THEN** l'installation est refusée
- **AND** aucun fichier n'est copié dans `$env:LOCALAPPDATA`
