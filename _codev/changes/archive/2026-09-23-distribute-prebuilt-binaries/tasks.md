# Tâches

## 1. Workflow GitHub Actions

- [x] 1.1 Créer `.github/workflows/release.yml` avec :
      - Trigger `on: push: tags: - 'v*.*.*'`.
      - Job `build` en matrix sur 3 targets :
        - `runner: macos-14`, `target: aarch64-apple-darwin`
        - `runner: macos-13`, `target: x86_64-apple-darwin`
        - `runner: ubuntu-24.04`, `target: x86_64-unknown-linux-musl`
      - Étapes par job : checkout, install Rust toolchain avec le
        target (via `dtolnay/rust-toolchain@stable`), cargo build
        release avec `--target <target>`, empaquetage
        `codev-<version>-<target>.tar.gz`, upload artifact.
- [x] 1.2 Sur le runner Linux musl : installer
      `musl-tools` (`apt install -y musl-tools`) avant le build.
- [x] 1.3 Job final `release` qui :
      - `needs: [build]` (dépend des 3 jobs matrix)
      - `download-artifact` récupère les trois tarballs
      - calcule SHA-256 de chaque tarball → agrège dans `SHA256SUMS`
      - `softprops/action-gh-release@v2` crée la Release GitHub avec
        les trois tarballs + `SHA256SUMS` en assets, `name: ${{
        github.ref_name }}`, `generate_release_notes: true`.
- [x] 1.4 Le workflow n'écrit aucun commit et n'exige aucun secret
      autre que `GITHUB_TOKEN` (accordé automatiquement par GH pour
      les writes de release).

## 2. Script `install.sh`

- [x] 2.1 Créer `install.sh` à la racine du dépôt, `#!/bin/sh`,
      shebang POSIX-compatible (pas de bashisms). En-tête avec
      description et licence.
- [x] 2.2 Fonctions internes :
      - `detect_target()` : combine `uname -s` (Darwin/Linux) et
        `uname -m` (arm64/aarch64/x86_64) pour retourner un des
        trois targets supportés, ou fail avec message.
      - `resolve_version()` : `${CODEV_VERSION:-$(curl -s
        https://api.github.com/repos/mairistem/codev/releases/latest |
        grep '"tag_name"' | head -1 | cut -d '"' -f 4)}`. Retire le
        `v` en tête.
      - `download_and_verify()` : télécharge tarball + `SHA256SUMS`,
        vérifie via `sha256sum` (Linux) ou `shasum -a 256` (macOS).
      - `install_binary()` : extrait dans un tempdir, `mkdir -p
        ~/.local/bin`, `cp <tempdir>/codev ~/.local/bin/codev`,
        `chmod 755`, nettoyage.
      - `check_path()` : test si `~/.local/bin` est dans `$PATH`.
- [x] 2.3 Corps principal en enchaînant ces fonctions, avec messages
      clairs à chaque étape (`echo "==> ..."`).
- [x] 2.4 Messages d'erreur : « OS ou arch non supporté » →
      fallback vers `cargo install --path`.
- [x] 2.5 Refus si `curl` ou `tar` manquent — messages clairs.
- [x] 2.6 Le script MUST être testable via `sh install.sh` en local
      (sans piping), pour qu'un utilisateur puisse d'abord `curl >
      install.sh`, `cat install.sh`, puis `sh install.sh`.

## 3. Section Installation de la doc

- [x] 3.1 Refondre la section 2 (« Installation ») de
      `docs/codev.md` avec les trois voies dans l'ordre :
      recommandée (curl), alternative (téléchargement manuel), et
      contributeur (cargo).
- [x] 3.2 Détailler la voie manuelle : lien vers releases, `tar xzf`,
      copie dans `~/.local/bin/`, vérification `sha256sum -c
      SHA256SUMS`.
- [x] 3.3 Ajouter un paragraphe court sur `$PATH` : comment vérifier,
      comment ajouter `~/.local/bin` s'il manque.
- [x] 3.4 Renommer la sous-section « Complétions shell » — elle
      reste utile, mais fait sens après l'installation, pas avant.

## 4. README

- [x] 4.1 Ajouter en tête du `README.md` (après une éventuelle
      description) un bloc « Installation rapide » avec la
      commande `curl -sSL https://raw.githubusercontent.com/mairistem/codev/main/install.sh | sh`.
- [x] 4.2 Renvoyer vers `docs/codev.md` (ou `codev docs`) pour le
      détail.

## 5. Validation à la main

- [x] 5.1 Lecture croisée du workflow YAML avec la documentation
      GitHub Actions pour éviter les fautes de syntaxe (indentation
      YAML capricieuse).
- [x] 5.2 Test local du script (partiellement) : lancer
      `install.sh` avec `CODEV_VERSION=0.1.0` **une fois qu'une
      release existe pour ce tag**. Refuser d'installer si le SHA
      diffère (test manuel : altérer le fichier téléchargé).
- [x] 5.3 Si aucune release n'existe encore : dry-run manuel du
      script en commentant les `curl` et en travaillant sur un
      tarball local.
- [x] 5.4 `codev validate --strict` reste vert.

## 6. Livraison

- [x] 6.1 Après merge de ce change : bump la version dans
      `Cargo.toml` (workspace) à `0.2.0`, `git tag v0.2.0`, `git
      push origin v0.2.0`. Le workflow tourne, la release apparaît.
- [x] 6.2 Vérifier la release sur GitHub : trois assets tar.gz + un
      `SHA256SUMS`.
- [x] 6.3 Test grandeur nature :
      `curl -sSL https://raw.githubusercontent.com/mairistem/codev/main/install.sh | sh`
      sur une VM Linux ou un Mac vierge (sans Rust). Vérifier :
      `codev --version` rend `0.2.0`, `codev docs` s'ouvre, `codev
      list` fonctionne dans un dépôt initialisé.
