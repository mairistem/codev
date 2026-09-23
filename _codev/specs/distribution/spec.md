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

- Un binaire précompilé par **cible supportée** (V1 : macOS arm64,
  macOS x86_64, Linux x86_64 musl statique), empaqueté dans un
  archive `.tar.gz` nommé `codev-<version>-<target>.tar.gz`.
- Un fichier `SHA256SUMS` qui liste le SHA-256 de chaque archive,
  au format `sha256sum` standard (`<hex>  <nom>`).
- Le contenu de chaque archive : le binaire `codev`, `README.md`,
  `LICENSE`, et une copie du markdown source de la doc
  (`docs/codev.md`).

Le workflow MUST NOT écrire dans le dépôt (pas de commit,
pas de push) — il produit uniquement des artefacts de release.

#### Scenario: Un tag `v0.2.0` déclenche la release

- **GIVEN** un développeur pousse un tag `v0.2.0` sur `main`
- **AND** la version dans `Cargo.toml` du workspace vaut `0.2.0`
- **WHEN** le workflow `.github/workflows/release.yml` s'exécute
- **THEN** une GitHub Release nommée `v0.2.0` est créée
- **AND** elle contient trois archives :
  `codev-0.2.0-aarch64-apple-darwin.tar.gz`,
  `codev-0.2.0-x86_64-apple-darwin.tar.gz`,
  `codev-0.2.0-x86_64-unknown-linux-musl.tar.gz`
- **AND** elle contient un fichier `SHA256SUMS` qui liste les trois
  archives avec leur SHA-256

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

La section Installation du fichier `docs/codev.md` MUST citer, dans
cet ordre :

1. **Recommandée pour les utilisateurs** — la commande
   `curl -sSL … | sh` (une seule ligne).
2. **Alternative** — téléchargement manuel depuis GitHub Releases,
   avec instruction pour la vérification manuelle du SHA-256.
3. **Pour les contributeurs** — `cargo install --path
   crates/codev-cli`, comme aujourd'hui.

Le README du dépôt MUST mentionner au moins la première voie, avec un
lien vers `docs/codev.md` pour le détail.

#### Scenario: Section Installation à jour

- **GIVEN** la documentation générée par `codev docs`
- **WHEN** on cherche la section « Installation »
- **THEN** les trois voies sont présentes dans l'ordre attendu
- **AND** aucune ne dépend de Rust hormis la troisième
