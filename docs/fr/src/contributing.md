# Contribuer à Compagnon

Compagnon est développé avec Compagnon : ses propres spécifications,
décisions et modifications sont dans le dossier `_compagnon/` de son
[dépôt](glossary.md#dépôt). Cette page résume comment contribuer ; le guide
complet est
[CONTRIBUTING.fr.md](https://github.com/mairistem/compagnon/blob/main/CONTRIBUTING.fr.md),
et [Architecture](architecture.md) explique comment le code est organisé.

## Deux façons de contribuer

- **Une petite correction : une simple
  [demande de fusion](glossary.md#demande-de-fusion).** Une coquille, la
  formulation d'un passage de la documentation, un commentaire, un
  [test](glossary.md#test) manquant : ouvrez directement la demande de
  fusion, sans modification Compagnon.
- **Tout le reste : une modification Compagnon.** Une nouvelle
  fonctionnalité, un changement de comportement, une nouvelle option ou une
  refactorisation commence par une modification sous `_compagnon/changes/`,
  relue en même temps que le code.

En cas de doute, ouvrez quand même la demande de fusion et posez la
question.

## Installer et vérifier votre travail

Il vous faut Rust (version stable ; la version minimale prise en charge est
la 1.89, l'espace de travail utilise l'édition 2024), [git](glossary.md#git)
et, pour utiliser les compétences pendant que vous contribuez, Claude Code
(facultatif).

```bash
git clone https://github.com/mairistem/compagnon.git
cd compagnon
cargo build
cargo run --bin compagnon -- --help
```

Pour utiliser votre version compilée sur d'autres projets :

```bash
cargo install --path crates/compagnon-cli
```

L'[intégration continue](glossary.md#intégration-continue) lance ces
commandes sur chaque demande de fusion ; lancez-les avant d'envoyer votre
travail :

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo run --locked --quiet --bin compagnon -- validate --strict
```

La dernière valide l'arborescence `_compagnon/` de Compagnon lui-même : ses
spécifications, ses modifications en cours et les sceaux de ses décisions
doivent rester valides. L'intégration continue lance aussi les tests sous
Linux, macOS et Windows, vérifie que l'espace de travail compile avec Rust
1.89 (`cargo check --workspace --locked`) et audite les dépendances avec
`cargo audit`. Les avertissements comptent comme des erreurs.

## La modification d'abord

Un changement de comportement commence par une modification Compagnon,
avant le code. Le fichier `CLAUDE.md` du dépôt le rappelle à Claude Code à
chaque session : « Do not modify behavior without a compagnon change. »
Les corrections triviales — coquilles, formulation de la documentation —
en sont dispensées.

1. Pour toute contribution d'envergure, ouvrez d'abord un ticket sur GitHub
   (onglet « Issues ») afin d'en discuter.
2. Dans votre clone, lancez `/compagnon-propose <votre idée>` dans Claude
   Code, ou créez la modification à la main avec
   `compagnon new change <nom>` et rédigez son plan.
3. Réalisez-la, avec `/compagnon-apply` ou à la main, en cochant les
   tâches de `tasks.md`.
4. Une fois le travail terminé, archivez-la avec
   `compagnon archive --change <nom>`, pour que `_compagnon/specs/`
   décrive le nouveau comportement.
5. Ouvrez une demande de fusion qui contient la modification archivée et
   le code.

Suivez les décisions acceptées de `_compagnon/decisions/` au lieu d'en
débattre à nouveau ; pour en changer une, une modification propose de la
remplacer.

## Évaluer une modification des compétences

Les rôles et les compétences sont des consignes données au modèle : leur
effet se mesure, il ne se devine pas. La
[décision 0008](https://github.com/mairistem/compagnon/blob/main/_compagnon/decisions/0008-every-change-to-the-workflow-is-evaluated-per-call-for-quality-and-tokens.md),
acceptée, en fait la règle : une demande de fusion qui modifie un rôle
(`assets/roles/`), le déroulé d'une compétence (`assets/workflows/`) ou le
JSON que lit une compétence (`crates/compagnon-cli/src/contract.rs`) lance
la suite d'évaluation, `compagnon-evals`, sur les étapes qu'elle touche, et
colle dans sa description la comparaison avec la ligne de base versionnée
(`evals/baseline/`) :

```bash
cargo build --bin compagnon
cargo run -p compagnon-evals -- run --step review --repeat 3 --jobs 4 --out target/evals/mine
cargo run -p compagnon-evals -- compare evals/baseline target/evals/mine --quality-only
```

La qualité doit rester dans l'écart couvert par les répétitions de la
ligne de base. Une demande de fusion destinée à réduire la consommation
retire `--quality-only` : son coût doit en plus passer sous le plancher de
la ligne de base. La suite demande Claude Code installé et connecté,
appelle un modèle et coûte de l'argent — une quinzaine de dollars pour les
40 cas de relecture — et fait travailler des agents dans le bac à sable de
Claude Code : lancez-la sur une machine ou dans un conteneur jetable.
[Fiabilité des rôles](reliability.md) montre la ligne de base actuelle ;
CONTRIBUTING.fr.md donne chaque étape, chaque option et chaque type de cas.

## Messages d'enregistrement

Les messages d'[enregistrement](glossary.md#enregistrement) (commit)
suivent la convention des enregistrements conventionnels
([Conventional Commits](https://www.conventionalcommits.org/)), en anglais :

```text
feat(cli): add --lang to compagnon docs
fix(engine): name the missing requirement when a MODIFIED delta fails
docs: explain inherited git sources
```

Les types courants sont `feat`, `fix`, `docs`, `refactor`, `test` et
`chore` ; la portée désigne le module (la crate) ou le domaine concerné (`core`,
`engine`, `agents`, `cli`, `skills`, `docs`). Gardez une ligne de sujet
courte, à l'impératif. Si un agent a coécrit un enregistrement, ajoutez une
ligne `Co-Authored-By:`.

Une demande de fusion qui change un comportement met à jour la
documentation dans les deux langues, `docs/en/` et `docs/fr/`, ou signale
que la partie française demande un suivi.

## Le guide complet

[CONTRIBUTING.fr.md](https://github.com/mairistem/compagnon/blob/main/CONTRIBUTING.fr.md)
couvre le reste : écrire des cas d'évaluation, les conventions de la
documentation, la publication d'une version, le signalement des failles de
sécurité et le code de conduite. [Architecture](architecture.md) décrit les
modules du code (les crates) et leurs dépendances.
