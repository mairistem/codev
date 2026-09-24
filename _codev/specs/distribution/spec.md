# Distribution Specification

## Purpose

Rendre codev installable sur les postes des utilisateurs qui **n'ont
pas la toolchain Rust** — via des binaires précompilés publiés sur
GitHub Releases et un script `install.sh` qui les récupère en une
commande.

## Requirements

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
### Requirement: Le script `install.sh` installe codev en une commande, sans Rust

Un script `install.sh` à la racine du dépôt SHALL permettre à un
utilisateur d'installer codev via :

```
curl -sSL https://raw.githubusercontent.com/mairistem/codev/main/install.sh | sh
```

Le script MUST :

- détecter l'OS (`uname -s` : `Darwin` ou `Linux`) et l'architecture
  (`uname -m` : `arm64`/`aarch64` ou `x86_64`) ;
- résoudre la version à installer : si `$CODEV_VERSION` est défini,
  l'utiliser ; sinon interroger l'API GitHub Releases pour la
  dernière ;
- télécharger l'archive `codev-<version>-<target>.tar.gz`
  correspondante et le fichier `SHA256SUMS` ;
- **vérifier le SHA-256** de l'archive contre le `SHA256SUMS` et
  refuser d'installer si la vérification échoue ;
- extraire le binaire dans un dossier temporaire, puis le copier
  dans `~/.local/bin/codev` avec les permissions `755` ;
- si `~/.local/bin` n'existe pas, le créer ;
- afficher, en fin de succès, le message adapté selon que
  `~/.local/bin` est déjà dans `$PATH` ou non (ajout à `.zshrc`/
  `.bashrc` recommandé si absent).

Le script MUST refuser (exit non nul) si :

- l'OS ou l'arch ne correspond à aucune cible supportée ;
- le SHA-256 vérifié ne correspond pas ;
- `curl` ou `tar` ne sont pas disponibles.

#### Scenario: Installation macOS Apple Silicon, PATH prêt

- **GIVEN** un utilisateur sur macOS Apple Silicon dont `~/.local/bin`
  est déjà dans `$PATH`
- **WHEN** il lance
  `curl -sSL https://raw.githubusercontent.com/mairistem/codev/main/install.sh | sh`
- **THEN** le script détecte `aarch64-apple-darwin`
- **AND** télécharge, vérifie SHA-256, extrait le binaire
- **AND** copie `codev` dans `~/.local/bin/`
- **AND** le message final invite l'utilisateur à taper `codev --version`
  pour vérifier

#### Scenario: PATH incomplet — message pédagogique

- **GIVEN** le même utilisateur mais `~/.local/bin` absent du `$PATH`
- **WHEN** l'installation se termine
- **THEN** le script affiche la ligne exacte à ajouter à `~/.zshrc`
  (par exemple `export PATH="$HOME/.local/bin:$PATH"`)
- **AND** le message rappelle qu'il faut ouvrir un nouveau shell ou
  faire `source ~/.zshrc`

#### Scenario: Plateforme non supportée refusée

- **GIVEN** un utilisateur sur Windows
- **WHEN** il lance le script via WSL avec `uname` retournant un OS
  non supporté ou une arch non supportée
- **THEN** le script quitte avec un exit code non nul
- **AND** le message d'erreur nomme la plateforme détectée et pointe
  vers la voie `cargo install --path` comme fallback

#### Scenario: SHA-256 corrompu → refus

- **GIVEN** un environnement où le fichier téléchargé serait altéré
  (test manuel ou fixture)
- **WHEN** le script vérifie le SHA-256
- **THEN** l'installation est refusée
- **AND** aucun fichier n'est copié dans `~/.local/bin/`

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
deuxième voie (avec les one-liners `curl … | sh` et `iwr … | iex`).

#### Scenario: la section Installation de docs/codev.md liste les quatre voies dans l'ordre

- **GIVEN** un lecteur qui ouvre `docs/codev.md` à la section
  Installation
- **WHEN** il parcourt les sous-sections dans l'ordre
- **THEN** il rencontre successivement la voie Unix (`curl | sh`),
  la voie Windows (`iwr | iex`), la voie manuelle (téléchargement
  depuis GitHub Releases avec vérif SHA-256), et la voie contributeur
  (`cargo install --path`)

#### Scenario: le README pointe au moins les deux voies « sans Rust » dans son Démarrage

- **GIVEN** un lecteur qui ouvre `README.md` à la racine du dépôt
- **WHEN** il parcourt la section « Démarrage »
- **THEN** il voit l'exemple `curl -sSL … | sh` pour macOS/Linux
- **AND** il voit l'exemple `iwr -useb … | iex` pour Windows dans
  PowerShell
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
-liners exacts), avec un lien vers
`docs/codev.md` pour le détail.

#### Scenario: Section Installation à jour avec les deux voies recommandées

- **GIVEN** la documentation générée par `codev docs`
- **WHEN** on cherche la section « Installation »
- **THEN** les quatre voies sont présentes dans l'ordre attendu
- **AND** la voie Windows cite exactement `iwr -useb https://…/install.ps1 | iex`
- **AND** la voie Unix cite exactement `curl -sSL https://…/install.sh | sh`
