# Contribuer à codev

Merci de vouloir améliorer codev.

## Prérequis

- **Rust stable + Cargo** — dernière stable recommandée (édition 2024
  active, MSRV `1.89`).
- **git**.
- **GitHub CLI (`gh`)** — facultatif, mais utile pour ouvrir issues
  et PR sans quitter le terminal.

## Le cycle — contribuer à codev, c'est utiliser codev

codev se contribue **avec codev**. La démonstration vivante que le
cycle marche, c'est qu'on l'applique à codev lui-même.

1. **Fork + clone.**
   ```bash
   gh repo fork mairistem/codev --clone
   cd codev
   ```
2. **Construire depuis la source.**
   ```bash
   cargo install --path crates/codev-cli
   ```
3. **Initialiser si nécessaire** — le dépôt est déjà initialisé, mais
   sur un fork frais on vérifie :
   ```bash
   codev status
   ```
4. **Créer un change.** Dans Claude Code :
   ```
   /codev-propose <mon-idée>
   ```
   Ou en direct :
   ```bash
   codev new change <mon-idée>
   ```
5. **Implémenter, tester.**
   ```bash
   cargo test --workspace
   cargo clippy --workspace --all-targets
   codev validate --strict
   ```
6. **Archiver.**
   ```bash
   codev archive <mon-change>
   ```
7. **Push et Pull Request.**
   ```bash
   git push -u origin ma-branche
   gh pr create
   ```

## Style

- Commits en **français ou anglais** — indifférent, mais cohérent
  dans une même PR.
- Ton neutre, formulations directes, pas d'auto-promotion dans les
  messages de commit.
- Si un agent (Claude Code notamment) a co-rédigé, ajoute une ligne
  `Co-Authored-By:` en pied de commit — c'est la convention GitHub
  standard.

## Release

Le processus de release, pour le mainteneur :

1. Bump de version dans `Cargo.toml` (workspace + crates si SemVer
   change).
2. Nouvelle entrée en tête de `CHANGELOG.md` avec la date et les
   changes archivés depuis la précédente version.
3. Commit `chore(release): vX.Y.Z`.
4. Tag `git tag vX.Y.Z`, `git push origin main --tags`.
5. GitHub Actions publie automatiquement les binaires précompilés
   (macOS arm64, macOS Intel, Linux musl, Windows) sur la Release.

Les notes détaillées de la Release sont auto-générées par GitHub
(`generate_release_notes: true` dans le workflow). Le `CHANGELOG.md`
sert de vue résumée par version.

## Signaler une faille

Voir [SECURITY.md](SECURITY.md). Ne pas ouvrir d'issue publique
pour une faille.

## Code de conduite

Voir [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md). Participation au projet
implique respect de la charte.
