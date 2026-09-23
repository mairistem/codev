# Tâches

## 1. Workflow release — ajouter Windows

- [x] 1.1 Éditer `.github/workflows/release.yml`. Dans
      `strategy.matrix.include`, ajouter :
      ```yaml
      - runner: windows-latest
        target: x86_64-pc-windows-msvc
      ```
- [x] 1.2 Adapter le pas « Package tarball » pour brancher sur l'OS :
      - Sur macOS/Linux, garde le comportement actuel : `tar czf …
        .tar.gz`.
      - Sur Windows, produire un `.zip` à la place. Deux options :
        - **A. Un pas dédié `if: runner.os == 'Windows'`** utilisant
          `Compress-Archive` PowerShell.
        - **B. Un pas unique qui `if` sur `runner.os`**, avec `run:`
          en `pwsh` ou `bash` selon le cas.
      - Option **A** préférée pour lisibilité.
- [x] 1.3 Adapter le pas « Upload artifact » pour que le pattern
      matche `.zip` ou `.tar.gz` selon le job. Aujourd'hui :
      `path: codev-${{ steps.version.outputs.version }}-${{
      matrix.target }}.tar.gz`. Le rendre paramétrique via
      `ext: tar.gz` / `zip` dans la matrix, ou deux uploads
      conditionnels.
- [x] 1.4 Adapter le pas « Compute SHA-256 sums » du job `release` :
      `shasum -a 256 *.tar.gz > SHA256SUMS` →
      `shasum -a 256 *.tar.gz *.zip > SHA256SUMS`.
- [x] 1.5 Le nom du binaire sur Windows est `codev.exe`. Le pas
      d'empaquetage doit copier `target/<target>/release/codev.exe`
      (avec le `.exe`) — sinon le fichier n'existera pas.

## 2. Nouveau script `install.ps1`

- [x] 2.1 Créer `install.ps1` à la racine du dépôt. Encoding
      UTF-8 (BOM interdit), fin de ligne CRLF ou LF (PowerShell
      accepte les deux, LF plus portable git).
- [x] 2.2 Sections du script, dans l'ordre :
      - En-tête avec description, usage
        (`iwr -useb … | iex`, `CODEV_VERSION` env var).
      - `$ErrorActionPreference = 'Stop'` pour arrêter dur sur toute
        exception.
      - Détection architecture : `$env:PROCESSOR_ARCHITECTURE` →
        `AMD64` = `x86_64-pc-windows-msvc`. Autre valeur = refus.
      - Résolution version : `$env:CODEV_VERSION` ou API GitHub
        (`Invoke-RestMethod https://api.github.com/repos/mairistem/codev/releases/latest`).
      - Téléchargement de l'archive et du `SHA256SUMS` via
        `Invoke-WebRequest`.
      - Vérification SHA-256 : `(Get-FileHash <archive> -Algorithm
        SHA256).Hash` comparé (case insensitive) au sha listé dans
        `SHA256SUMS`.
      - Extraction via `Expand-Archive`.
      - Copie de `codev.exe` dans
        `$env:LOCALAPPDATA\Programs\codev\`, avec `New-Item -ItemType
        Directory -Force` si absent.
      - Vérification PATH : `$env:PATH -split ';' -contains
        '$env:LOCALAPPDATA\Programs\codev'`.
      - Message final : instructions pour ajouter au PATH via
        `setx PATH "$env:PATH;$env:LOCALAPPDATA\Programs\codev"` (à
        exécuter dans un shell séparé), ou UI Système → Variables
        d'environnement.
- [x] 2.3 Aucun `Set-ExecutionPolicy` — un script piped depuis
      stdin s'exécute sans policy.

## 3. Documentation

- [x] 3.1 Refondre la section « Installation » de `docs/codev.md`
      avec **4 voies** dans l'ordre :
      1. **Recommandée Unix** — `curl … | sh` (déjà là).
      2. **Recommandée Windows** — `iwr -useb … | iex` (nouveau).
      3. **Voie manuelle** — téléchargement + vérif SHA-256
        (parties Unix ET Windows).
      4. **Voie contributeur** — `cargo install` (déjà là).
- [x] 3.2 Compléter la voie manuelle pour Windows :
      `Expand-Archive`, `Get-FileHash`, copie dans
      `%LOCALAPPDATA%\Programs\codev\`.
- [x] 3.3 Ajouter un paragraphe court « PATH sur Windows » : comment
      vérifier, comment ajouter `%LOCALAPPDATA%\Programs\codev\`
      (via `setx`, via UI Système).

## 4. README

- [x] 4.1 Ajouter le one-liner Windows dans le bloc « Démarrage »
      juste après le curl Unix :
      ```powershell
      # Windows
      iwr -useb https://raw.githubusercontent.com/mairistem/codev/main/install.ps1 | iex
      ```
- [x] 4.2 Éventuellement renommer le commentaire du curl Unix en
      « # macOS ou Linux » pour clarifier la distinction.

## 5. Validation

- [x] 5.1 `codev validate --strict` reste vert.
- [x] 5.2 `cargo test --workspace` reste vert (rien de Rust n'a
      changé, mais on vérifie).
- [x] 5.3 Lint YAML manuel du workflow (indentation matrix,
      `if:` PowerShell, quote des variables `${{ … }}`).

## 6. Livraison

- [x] 6.1 Après merge : bump `Cargo.toml` à `0.2.0`, `git tag
      v0.2.0`, `git push origin main v0.2.0`.
- [x] 6.2 Vérifier la release sur GitHub : **quatre** assets
      (`.tar.gz` × 3 + `.zip`) + un `SHA256SUMS` qui référence les
      quatre.
- [x] 6.3 Test grandeur nature :
      - Sur macOS/Linux : `curl … | sh` doit toujours marcher (les
        3 anciennes cibles).
      - Sur Windows (VM ou poste physique) : `iwr … | iex` doit
        installer, la commande PATH proposée doit fonctionner, puis
        `codev --version` doit rendre `0.2.0`.
