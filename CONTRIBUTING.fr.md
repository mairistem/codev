# Contribuer à codev

[English](CONTRIBUTING.md) · Français

Merci de contribuer à l'amélioration de codev. Ce guide explique comment
proposer une modification, mettre en place un environnement de développement
et faire fusionner une pull request.

## Deux façons de contribuer

**Les petites corrections : une pull request classique.** Une coquille, la
formulation d'un passage de la documentation, un commentaire, un test
manquant : ouvrez directement une pull request. Aucun change codev n'est
nécessaire.

**Tout le reste : un change codev.** codev est développé avec codev. Une
nouvelle fonctionnalité, une modification de comportement, une nouvelle
option ou une refactorisation commence par un change sous `_codev/changes/`,
relu en même temps que le code :

1. Pour toute contribution d'envergure, ouvrez d'abord une issue afin d'en
   discuter.
2. Dans votre clone, lancez `/codev-propose <your idea>` dans Claude Code — ou
   créez le change à la main avec `codev new change <name>` et rédigez ses
   artefacts.
3. Implémentez-le, avec `/codev-apply` ou à la main, en cochant les tâches de
   `tasks.md`.
4. Une fois le travail terminé, archivez le change avec
   `codev archive --change <name>`, pour que les specs principales de
   `_codev/specs/` décrivent le nouveau comportement.
5. Ouvrez une pull request contenant le change archivé et le code.

Si vous ne savez pas quelle voie s'applique, ouvrez tout de même la pull
request et posez la question : nous préférons vous aider plutôt que de refuser
une contribution.

## Environnement de développement

Il vous faut :

- **Rust**, version stable. La version minimale prise en charge est la 1.89 ;
  le workspace utilise l'édition 2024.
- **git**.
- **Claude Code**, pour utiliser les skills pendant que vous contribuez
  (facultatif).

```bash
git clone https://github.com/mairistem/codev.git
cd codev
cargo build
cargo run --bin codev -- --help
```

Pour utiliser votre build sur d'autres projets, installez-le :

```bash
cargo install --path crates/codev-cli
```

Le [chapitre Architecture](docs/fr/src/architecture.md) explique comment le
code est organisé.

## Vérifications

La CI exécute les commandes suivantes sur chaque pull request. Lancez-les en
local avant de pousser :

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo run --locked --quiet --bin codev -- validate --strict
```

La CI lance aussi les tests sous Linux, macOS et Windows, vérifie que le
workspace compile avec Rust 1.89 (`cargo check --workspace --locked`) et
audite les dépendances avec `cargo audit`. Les avertissements sont traités
comme des erreurs.

La dernière commande valide l'arborescence `_codev/` de codev lui-même : les
specs, les changes actifs et les sceaux des décisions doivent rester valides.

## Commits

Les messages de commit suivent la convention
[Conventional Commits](https://www.conventionalcommits.org/), en anglais :

```text
feat(cli): add --lang to codev docs
fix(engine): name the missing requirement when a MODIFIED delta fails
docs: explain inherited git sources
```

Les types courants sont `feat`, `fix`, `docs`, `refactor`, `test` et
`chore` ; la portée désigne la crate ou le domaine concerné (`core`,
`engine`, `agents`, `cli`, `skills`, `docs`). Gardez une ligne de sujet
courte, à l'impératif.

Si un agent a coécrit un commit, ajoutez une ligne `Co-Authored-By:`.

## Documentation

La documentation existe en anglais (`docs/en/`) et en français (`docs/fr/`),
avec les mêmes fichiers et la même structure. Lorsqu'une pull request modifie
un comportement, mettez à jour les deux langues — ou indiquez dans la pull
request que la version française reste à mettre à jour, pour qu'un mainteneur
puisse s'en charger.

Chaque chapitre commence par un unique titre `# `, et les chapitres sont
listés dans `docs/<lang>/src/SUMMARY.md`. `codev docs` embarque les chapitres
de cette liste, dans l'ordre, en une seule page : les liens entre chapitres
doivent donc être relatifs (`concepts.md#delta`) et pointer vers des titres
uniques dans tout le livre.

Pour prévisualiser le site, installez [mdBook](https://rust-lang.github.io/mdBook/)
et lancez `mdbook serve docs/fr`.

Les changements visibles par les utilisateurs font l'objet d'une entrée sous
`## [Unreleased]` dans [CHANGELOG.md](CHANGELOG.md).

## Processus de release

Pour les mainteneurs :

1. Vérifiez que `main` est au vert et que `CHANGELOG.md` est à jour.
2. Incrémentez `version` dans la section `[workspace.package]` du
   `Cargo.toml` racine, puis lancez `cargo build` pour mettre à jour
   `Cargo.lock`.
3. Dans `CHANGELOG.md`, renommez `## [Unreleased]` en
   `## [X.Y.Z] - YYYY-MM-DD`, ajoutez au-dessus une nouvelle section
   `## [Unreleased]` vide, et mettez à jour les liens en bas du fichier.
4. Commitez avec le message `chore(release): vX.Y.Z`.
5. Créez le tag et poussez :

   ```bash
   git tag vX.Y.Z
   git push origin main --tags
   ```

Le workflow de release compile les quatre binaires précompilés (macOS arm64 et
x86_64, Linux x86_64 musl, Windows x86_64), calcule `SHA256SUMS` et publie la
GitHub Release avec des notes générées. Les installateurs prennent en compte la
nouvelle version immédiatement.

## Sécurité

N'ouvrez pas d'issue publique pour signaler une vulnérabilité. Suivez plutôt
[SECURITY.md](SECURITY.md) (en anglais).

## Code de conduite

La participation à ce projet est régie par le
[code de conduite](CODE_OF_CONDUCT.md) (en anglais).
