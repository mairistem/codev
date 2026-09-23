# Proposal : ajouter Windows au pipeline de distribution

## Pourquoi

La release `v0.1.1` publie trois binaires pour macOS (arm64 + x86_64)
et Linux (musl x86_64). Les utilisateurs Windows — encore majoritaires
en entreprise, y compris chez JVS — n'ont **aucune voie d'install**
sans passer par WSL ou par la toolchain Rust. C'est un trou incompatible
avec l'objectif « installable en une commande, sans Rust ».

Ajouter Windows, c'est :

- une **cible de plus** dans la matrix du workflow release
  (`x86_64-pc-windows-msvc`) ;
- un **format d'archive** différent (`.zip` — convention Windows,
  extraction native via `Expand-Archive`) ;
- un **script d'install PowerShell** (`install.ps1`) qui suit le
  pattern rustup / Scoop / winget :
  `iwr <url> -useb | iex`.

Les 3 cibles existantes (macOS/Linux) et le script `install.sh` ne
sont **pas touchés**. Ce lot est purement additif.

## Ce qui change

- **Workflow `.github/workflows/release.yml`** — nouvelle ligne de
  matrix :
  ```yaml
  - runner: windows-latest
    target: x86_64-pc-windows-msvc
  ```
  Un pas d'empaquetage spécifique Windows produit
  `codev-<version>-x86_64-pc-windows-msvc.zip` (au lieu de `.tar.gz`)
  contenant `codev.exe`, `README.md`, `LICENSE`, `docs/codev.md`.
- **Nouveau script `install.ps1`** à la racine du dépôt, pipeable :
  ```powershell
  iwr -useb https://raw.githubusercontent.com/mairistem/codev/main/install.ps1 | iex
  ```
  Le script :
  - détecte l'architecture (`$env:PROCESSOR_ARCHITECTURE`) ;
  - résout la version (`$env:CODEV_VERSION` ou dernière via l'API
    GitHub) ;
  - télécharge le `.zip` cible et le `SHA256SUMS` via
    `Invoke-WebRequest` ;
  - vérifie le SHA-256 via `Get-FileHash -Algorithm SHA256` ;
  - extrait via `Expand-Archive` ;
  - copie `codev.exe` dans `$env:LOCALAPPDATA\Programs\codev\` avec
    création du dossier si absent ;
  - **affiche** les instructions pour ajouter le dossier au PATH
    utilisateur (ne modifie pas la variable d'environnement — même
    parti pris que `install.sh` qui ne touche pas à `.zshrc`).
- **Section Installation de `docs/codev.md`** — la « voie recommandée »
  se dédouble : `curl … | sh` pour macOS/Linux et `iwr … | iex` pour
  Windows. Un tableau ou deux sous-blocs, à choisir au rendu.
- **README** — ajout du one-liner Windows en complément.
- **`SHA256SUMS`** — inclut désormais le `.zip` Windows en plus des
  trois `.tar.gz`.

## Capacités

### Nouvelles capacités

Aucune.

### Capacités modifiées

- `distribution` — deux exigences MODIFIED :
  - « Un tag `v*.*.*` publie une GitHub Release » — la liste des
    cibles passe de 3 à 4 ; le format d'archive est `.tar.gz` pour
    macOS/Linux et `.zip` pour Windows.
  - « La documentation cite les voies d'installation » — deux voies
    recommandées (Unix + Windows).
- `distribution` — une exigence ADDED : le script `install.ps1` et
  son contrat (détection arch, vérif SHA-256, install dans
  `%LOCALAPPDATA%\Programs\codev\`).

### Capacités retirées

Aucune.

## Impact

- **Code** : rien dans les crates Rust — encore une fois, le change
  touche uniquement l'infra CI et les scripts d'install.
  - `.github/workflows/release.yml` : nouvelle ligne matrix +
    branche d'empaquetage `.zip`.
  - `install.ps1` : nouveau fichier racine.
  - `docs/codev.md` : refonte de la section Installation.
  - `README.md` : ajout du one-liner Windows.
- **Contrat JSON** : rien.
- **Fichier écrit** : côté release, un asset de plus (le `.zip`
  Windows) + le `SHA256SUMS` mis à jour. Côté user Windows, le
  binaire dans `%LOCALAPPDATA%\Programs\codev\codev.exe`.
- **Migration** : aucune. Ce lot est additif — les binaires
  macOS/Linux et `install.sh` continuent d'exister sans changement.
- **Hors périmètre** :
  - **Windows ARM64 (`aarch64-pc-windows-msvc`)** — reportable
    jusqu'à demande concrète.
  - **Package manager Windows** (Scoop, Chocolatey, winget) — plus
    lourd à maintenir, un utilisateur peut passer par
    l'installation directe en attendant.
  - **Modification automatique du PATH utilisateur** — le script
    affiche les instructions, ne modifie pas `[Environment]`.
    Symétrie avec `install.sh`.
  - **Complétions PowerShell installées automatiquement** —
    l'utilisateur peut lancer `codev completions powershell |
    Out-String | iex` s'il veut.
