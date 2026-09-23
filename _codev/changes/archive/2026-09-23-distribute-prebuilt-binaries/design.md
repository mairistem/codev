# Design : distribution de codev par binaires précompilés

## Contexte

Voir `proposal.md`. Change **hors des crates Rust** — il touche
uniquement au workflow GitHub Actions, au script `install.sh`, et à
la documentation. Aucune ligne de Rust ne bouge.

## Objectifs / Hors objectifs

Ce design cadre : les cibles supportées, le format d'artefact, la
mécanique du workflow, le comportement du script `install.sh`, la
vérification d'intégrité. Il ne cadre pas Windows, Linux ARM64,
Homebrew, auto-update, ni signature GPG.

## Décisions

### Décision : trois cibles pour la V1 — macOS arm64/x86_64, Linux x86_64 musl

Le public visé (équipes JVS + contributeurs open source curieux) est
à ≥ 95 % sur ces trois cibles. Aller plus loin ajoute de la matrice
CI (build lent, cache à gérer) sans bénéfice mesuré.

**Alternative écartée** : ajouter `aarch64-unknown-linux-musl` et
`x86_64-pc-windows-msvc` d'entrée. Rejeté au titre de la V1 —
extensions faciles quand un vrai utilisateur se manifestera.

### Décision : musl statique pour Linux, pas glibc

Un binaire compilé contre musl est **statiquement lié** — il marche
sur n'importe quelle distribution Linux, quelle que soit la version
de glibc. C'est le pattern adopté par ripgrep, fd, bat, sccache, la
plupart des CLI Rust d'infrastructure.

Un binaire glibc dynamique risquerait de casser sur des systèmes
plus anciens (« GLIBC_2.34 not found »). musl échange 10-15 % de
taille de binaire contre une portabilité totale — trade-off à
prendre.

### Décision : format `.tar.gz`, pas `.zip`

`.tar.gz` est natif sur macOS et Linux — tar est présent partout,
l'utilisateur n'a rien à installer pour extraire. `.zip` demanderait
un `unzip` séparé sur Linux minimal.

Windows utilisera `.zip` **quand** Windows sera supporté. Pour V1,
un seul format simplifie le script d'install.

### Décision : SHA-256 dans un `SHA256SUMS`, pas de signature GPG

Deux niveaux de garantie possibles :

| Option | Pro | Contre |
|---|---|---|
| **A. Rien** | Simple | Un attaquant qui compromet le CDN peut servir un binaire modifié |
| **B. SHA-256 dans un `SHA256SUMS`** publié par le même workflow | Défend contre les altérations en transit ; format standard `sha256sum -c` | Ne défend pas contre une compromission du workflow GH Actions lui-même |
| **C. Signature GPG / sigstore** | Défense forte contre tout attaquant hors des mainteneurs | Clef à gérer, publier, faire tourner. Pour un outil interne c'est disproportionné en V1. |

**Choisi : B.** Pattern des CLI Rust mainstream (ripgrep, fd,
starship). Marge de progression vers C si le contexte se durcit
(publication publique, secteur régulé, etc.).

### Décision : `install.sh` télécharge le SHA256SUMS séparément et vérifie

Le script ne fait pas confiance au serveur — il télécharge le
`SHA256SUMS` publié dans la release, calcule le SHA-256 du fichier
qu'il a téléchargé, compare, refuse si divergence. Pratique standard.

**Alternative écartée** : télécharger seulement l'archive, sans
vérifier. Rejeté — le coût est nul (un `curl` de plus, un `sha256sum`
en local).

### Décision : destination `~/.local/bin/`, pas `/usr/local/bin/`

`~/.local/bin/` est le dossier XDG standard pour un binaire installé
par l'utilisateur, sans droits root. Compatible macOS/Linux, ne
demande pas de `sudo`.

**Alternative écartée** : `/usr/local/bin/`. Demande `sudo` sur
beaucoup de systèmes ; conflit possible avec Homebrew.

### Décision : version dans le nom de l'archive

`codev-0.2.0-aarch64-apple-darwin.tar.gz` — la version est visible
sans avoir à décompresser. Utile pour un utilisateur qui garde
plusieurs versions côte à côte, et pour le script `install.sh` qui
construit le nom d'après la version résolue.

### Décision : le workflow GH Actions ne pousse rien dans le dépôt

Aucun commit auto, aucun push automatique. Le workflow lit le repo,
produit des artefacts, publie sur GitHub Releases. C'est tout.

**Rationale** : un workflow qui pousserait des commits (par exemple
pour bump la version) crée un cycle push → CI → push potentiellement
récursif. On préfère un modèle simple où le développeur bump la
version localement, tag, push le tag, le workflow publie la release.

## Risques et compromis

- **Un utilisateur pipe `curl … | sh` d'une source compromise.**
  → **Atténuation** : le script vérifie le SHA-256 depuis le même
  serveur, ce qui défend contre la compromission d'un CDN mais pas
  contre celle du repo lui-même. Pour une garantie plus forte, l'user
  peut télécharger le script, le lire, puis l'exécuter (`sh
  install.sh`). Documenté dans la doc.
- **`curl -sSL … | sh` reste une pratique discutée** — mais standard
  chez rustup, oh-my-zsh, Homebrew, starship. Le compromis
  ergonomie/sécurité est accepté par la communauté.
- **La matrice de trois cibles double le temps de CI** — la V1 tourne
  probablement à 5-10 min end-to-end. Pas bloquant.
- **La version dans `Cargo.toml` doit correspondre au tag** — sinon
  le `install.sh` ne trouvera pas l'archive. Un petit script `xtask
  release <version>` ou une note dans `docs/codev.md` peut aider. Pas
  de mécanique auto pour la V1 — un check-list humain suffit.

## Plan de migration

Pour un utilisateur qui a **déjà** installé codev via `cargo install`
et veut migrer vers la voie curl :

1. `rm ~/.cargo/bin/codev` (optionnel, remplacé au prochain
   `cargo install`)
2. `curl -sSL https://…/install.sh | sh`
3. Vérifier que `~/.local/bin/codev` est utilisé
   (`which codev` doit rendre ce chemin, à condition que le PATH
   ordonne `~/.local/bin` avant `~/.cargo/bin`)

Pour un utilisateur **nouveau** : suivre la section Installation de
`codev docs` (recommandée : `curl … | sh`).
