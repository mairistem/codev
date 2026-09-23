# Proposal : distribuer codev sans exiger Rust

## Pourquoi

Aujourd'hui, la seule voie d'installation documentée est
`cargo install --path crates/codev-cli`. Ça marche pour Ludovic et
tout développeur qui a déjà la toolchain Rust — mais **la majorité des
utilisateurs JVS visés n'ont pas Rust**. Ils ont git, Claude Code,
souvent Node ou Python, jamais rustc.

Le binaire, lui, est **autonome** une fois compilé : ~4 Mo, aucune
dépendance runtime Rust, quelques dépendances système standard.
L'obstacle est purement à l'étape d'install.

Ce lot livre deux voies d'install additionnelles, sans casser la voie
`cargo install` qui reste utile aux contributeurs :

- **GitHub Releases** avec binaires précompilés pour macOS (arm64 et
  x86_64) et Linux (x86_64 musl statique) — l'utilisateur télécharge,
  extrait, met sur son PATH.
- **Script `install.sh`** pipeable via `curl … | sh` — détecte
  OS/arch, télécharge le bon binaire, vérifie son SHA-256, installe
  dans `~/.local/bin/`. Un utilisateur qui découvre codev tape
  **une seule ligne** pour l'avoir.

## Ce qui change

- **Nouveau workflow GitHub Actions** `.github/workflows/release.yml`
  déclenché par un tag `v*.*.*` :
  - Trois jobs matrix, un par target : `aarch64-apple-darwin`,
    `x86_64-apple-darwin`, `x86_64-unknown-linux-musl`.
  - Chaque job `cargo build --release --bin codev --target <target>`.
  - Empaquetage : `codev-<version>-<target>.tar.gz` contenant le
    binaire, `README.md`, `LICENSE`, et le fichier
    `docs/codev.md` (source de la doc).
  - Un job final agrège les checksums SHA-256 dans un fichier
    `SHA256SUMS` et publie la GitHub Release avec les artefacts.
- **Nouveau script racine `install.sh`** :
  - Détecte l'OS via `uname` (macOS/Linux) et l'arch via `uname -m`
    (arm64/x86_64).
  - Résout la version demandée (`$CODEV_VERSION` env var ou dernière
    release via l'API GitHub).
  - Télécharge le tar.gz cible et le `SHA256SUMS`.
  - Vérifie le SHA-256 du fichier téléchargé contre celui du
    `SHA256SUMS`.
  - Extrait et copie `codev` dans `~/.local/bin/`. Crée le dossier
    si absent.
  - Vérifie que `~/.local/bin/` est dans le `$PATH` — sinon, imprime
    la ligne à ajouter à son `.zshrc`/`.bashrc`.
- **Section « Installation » de `docs/codev.md`** — refondue avec
  **trois voies** : `curl … | sh` (recommandé), téléchargement
  manuel depuis GitHub Releases, `cargo install` (contributeurs).
- **README.md du dépôt** — la première voie citée devient
  `curl -sSL https://raw.githubusercontent.com/mairistem/codev/main/install.sh | sh`.

## Capacités

### Nouvelles capacités

- `distribution` — décrit le contrat de distribution : quelles cibles
  sont supportées, quel format d'artefact, quel script d'install,
  quelle vérification d'intégrité.

### Capacités modifiées

Aucune.

### Capacités retirées

Aucune.

## Impact

- **Code** : rien dans les crates Rust. Le change touche exclusivement :
  - `.github/workflows/release.yml` (nouveau)
  - `install.sh` (nouveau)
  - `docs/codev.md` (section Installation)
  - `README.md` (mention)
- **Contrat JSON** : rien. La distribution vit hors du binaire.
- **Fichier écrit** : le workflow GH Actions écrit des artefacts sur
  GitHub Releases ; `install.sh` écrit `~/.local/bin/codev` chez
  l'utilisateur.
- **Migration** : aucune. La voie `cargo install` reste documentée
  et fonctionne.
- **Hors périmètre** :
  - **Windows** — les utilisateurs JVS sont majoritairement sur
    macOS/Linux. Un utilisateur Windows qui se manifeste plus tard
    déclenchera un change dédié.
  - **Linux ARM64** — bonus reportable. Cargo install fonctionne en
    attendant.
  - **Homebrew tap** — plus lourd à maintenir (repo dédié pour le
    tap, formule Ruby, versioning). Reportable jusqu'à demande
    explicite.
  - **Auto-update du binaire** — pattern à la `rustup update`. Pas
    nécessaire pour la V1 ; `codev-<version>` réinstalle facilement.
  - **Signature GPG / cosign** — la vérification SHA-256 depuis un
    fichier publié par le même workflow suffit pour la V1. Une
    vraie signature demandera une clef à gérer et à publier.
  - **Publication sur crates.io** — nécessite cargo côté user, ne
    résout pas le problème initial.
