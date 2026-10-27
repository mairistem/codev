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

## Modifier un rôle

Les rôles intégrés (`assets/roles/`) sont des prompts : leur effet se
mesure, il ne se devine pas. Une pull request qui modifie un fichier de
`assets/roles/` exécute les évaluations des rôles concernés et colle dans
sa description la sortie de
`cargo run -p codev-evals -- compare --quality-only` face à la ligne de
base versionnée, `evals/baseline/` : le rappel, les faux positifs et la
validité des preuves de chaque rôle doivent rester dans l'écart couvert
par les répétitions de la ligne de base. Une pull request destinée à
réduire la consommation des skills colle la sortie de
`cargo run -p codev-evals -- compare`, sans `--quality-only` : la qualité
doit tenir et le coût passer sous le plancher de la ligne de base, chaque
cas à son exécution la moins chère. Toute autre pull request qui change
ce que fait une skill — un fichier de `assets/workflows/`, ou le JSON
qu'une skill lit, `crates/codev-cli/src/contract.rs` — colle la sortie de
`cargo run -p codev-evals -- compare --quality-only`, qui rapporte
toujours ce que ses exécutions consomment. Lancez l'une et l'autre sur
les cas des étapes que la pull request touche, avec `--step`
(répétable) : `review` pour les étapes de revue, `plan.write` pour la
planification de `/codev-propose`, `plan.update` pour les révisions de
`/codev-update`, `code.implement` pour l'implémentation de
`/codev-apply`. La décision acceptée 0008 en fait la
règle pour tout changement du workflow. Enregistrez le candidat avec
`--repeat 3` au moins, et avec le `--jobs` de la ligne de base (4), pour
que les deux exécutions profitent du cache de prompt de la même façon.

La suite demande [Claude Code](https://claude.com/claude-code) installé et
connecté (ou `ANTHROPIC_API_KEY` défini). Elle appelle un modèle : une
exécution des 40 cas de revue coûte une quinzaine de dollars américains,
chaque cas étant plafonné à 3 par défaut ; un cas de planification coûte
environ 1,65 dollar et prend plusieurs minutes, plafonné par défaut à
6 dollars et 1 800 secondes, tout comme un cas de révision ; un cas
d'implémentation écrit, construit et relit un change entier, plafonné par
défaut à 30 dollars et 5 400 secondes.

```bash
cargo build --bin codev
cargo run -p codev-evals -- list                       # les cas
cargo run -p codev-evals -- run --role security --repeat 3 --jobs 4 --out target/evals/mine
cargo run -p codev-evals -- run --step plan.write --repeat 3 --jobs 4 --out target/evals/plan
cargo run -p codev-evals -- run --step plan.update --repeat 3 --jobs 4 --out target/evals/update
cargo run -p codev-evals -- run --step code.implement --repeat 3 --jobs 2 --out target/evals/implement
cargo run -p codev-evals -- calibrate implement-decision-accept-command   # éprouver un script, sans modèle
cargo run -p codev-evals -- score target/evals/mine    # renoter, sans modèle
cargo run -p codev-evals -- compare evals/baseline target/evals/mine --quality-only
```

Pour mesurer un autre binaire que `target/debug/codev`, ou en figer un
pendant que vous continuez à travailler, copiez-le dans un dossier à lui
et passez-le avec `--codev <dossier>/codev` : le fichier garde le nom
`codev`. Le runner place ce dossier en tête du chemin de recherche, et
l'agent appelle `codev` par son nom ; sous un autre nom, l'agent
lancerait n'importe quel autre `codev` installé. `codev-evals run`
refuse un tel binaire.

L'agent tourne dans le bac à sable (sandbox) de Claude Code, comme le
ferait un utilisateur qui s'en sert : les commandes shell que Claude Code
connaît, et `codev`, s'exécutent sans demande, mais elles ne peuvent
écrire que dans le répertoire du cas et dans le répertoire temporaire du
système — qui contient aussi ceux des autres cas —, ne joindre que l'API
d'Anthropic, et ne pas lire la clé d'API ; les outils de fichiers, hors du
bac à sable, ne modifient qu'à l'intérieur du répertoire du cas. Tout autre
appel est refusé et compté dans le rapport, et une exécution qui modifie
le `.git/config` de son dépôt est enregistrée en erreur. Le bac à sable
borne un agent, il ne l'isole pas : lancez la suite sur une machine ou
dans un conteneur jetable. Sous macOS, le bac à sable est intégré. Sous Linux, il demande
`bubblewrap` et `socat` (`sudo apt-get install bubblewrap socat`) et, sous
Ubuntu 24.04, un profil AppArmor qui autorise `bwrap` à créer des espaces
de noms utilisateur, comme l'installe `.github/workflows/evals.yml` ; sans
eux, chaque cas s'arrête en erreur plutôt que de tourner sans
confinement.

`run` écrit `report.md` et `report.json` dans son répertoire de sortie et
se termine avec le code 1 quand un rôle passe sous un seuil
(`--min-recall`, `--max-false-positives`, `--min-evidence`) ou qu'une
exécution s'est arrêtée. `--repeat N` exécute chaque cas N fois ; le
rapport donne la médiane de chaque mesure sur les exécutions, avec son
écart, et ce qu'elles ont consommé (coût, durées, tours, tokens, refus de
permission, appels au modèle avec leur entrée et leur sortie par appel),
et où vont les tokens : chaque source — la lecture de la skill, chaque
sous-commande `codev`, les fichiers, les recherches, la sortie du modèle
lui-même — avec sa part du coût. La sortie ne décide jamais d'une
comparaison : elle porte le raisonnement du modèle, que gardent les
mesures de qualité. Un répertoire qui contient déjà des observations est repris :
seules les exécutions manquantes, et celles en erreur, sont refaites. `CODEV_EVAL_MODEL` choisit le modèle (par défaut celui de Claude
Code), `CODEV_EVAL_BUDGET_USD` et `CODEV_EVAL_TIMEOUT_SECS` bornent chaque
cas, quelle que soit son étape. Comparez avec le même modèle que la ligne de base. Les mainteneurs
peuvent aussi lancer à la main le workflow **Role evaluations** ; il
demande le secret de dépôt `ANTHROPIC_API_KEY` et s'arrête sans rien faire
sinon.

Les cas de revue tournent par défaut dans le mode `inline`.
`--review-mode isolated` les joue dans le mode `isolated` : le
`_codev/config.yaml` de chaque cas reçoit `review: { mode: isolated }`
avant le rendu des skills, l'outil de sous-agent (`Agent`) est accordé,
et un cas dont le rôle n'a pas tourné seul — pas de ligne
`Mode: isolated.`, une ligne `Fallback:`, ou aucun sous-agent lancé —
s'arrête en erreur. Les tokens que les totaux de la session comptent
au-delà de ses propres appels vont à la source `sub-agents` ; une
exécution dont les totaux sont inférieurs à ses propres appels s'arrête
en erreur. `--fixture-config <fichier>` ajoute un mapping YAML au
`_codev/config.yaml` de chaque cas de revue, pour un réglage dont une
mesure a besoin, et à celui de chaque cas d'implémentation — avec
`roles: { developer: { model: sonnet } }`, par exemple, pour mesurer
l'implémentation sur le modèle du développeur ; il ne peut pas définir
`review:`, que seule `--review-mode` donne. Les cas de planification et
de révision ignorent ces deux options ; les cas d'implémentation ignorent
`--review-mode` et gardent le mode de leur propre arbre. `run.json` les enregistre, et une exécution reprise avec
d'autres valeurs s'arrête. `compare` refuse deux exécutions dont les cas
de revue ont tourné dans des modes différents, et nomme les
configurations ajoutées des deux exécutions, en signalant quand elles
diffèrent. Pour mesurer un changement de modèle, passez
`--across-models` : `compare` compare alors deux exécutions dont les
modèles diffèrent et nomme les deux en tête de son rapport. Enregistrez
les deux exécutions le même jour, dans le même mode :

```bash
cargo run -p codev-evals -- run --step review --review-mode isolated --repeat 3 --jobs 4 --out target/evals/opus
CODEV_EVAL_MODEL=claude-sonnet-5-5 cargo run -p codev-evals -- run --step review --review-mode isolated --repeat 3 --jobs 4 --out target/evals/sonnet
cargo run -p codev-evals -- compare target/evals/opus target/evals/sonnet --across-models --quality-only
```

Le [chapitre sur la fiabilité](docs/fr/src/reliability.md) de la
documentation présente la ligne de base aux lecteurs. Il est généré,
jamais modifié à la main : chaque fois que `evals/baseline/` change —
enregistrée ou notée de nouveau —, lancez
`cargo run -p codev-evals -- docs`, qui réécrit
`docs/en/src/reliability.md` et `docs/fr/src/reliability.md`. Les tests
échouent tant que les pages diffèrent de la ligne de base.

### Écrire un cas

Un cas est un répertoire de `evals/cases/` : `case.yaml`, le projet sur sa
branche par défaut (`base/`) et les fichiers que le change ajoute ou
modifie, artefacts de planification compris (`change/`). Chaque constat
attendu donne sa gravité (`severity`), ses emplacements (`locations`) et
une catégorie (`category`) tirée de `evals/categories.yaml`, le
vocabulaire commun dont les termes reconnaissent une catégorie dans le
titre ou la recommandation d'un constat ; `alternatives` liste d'autres
endroits où un relecteur peut à bon droit citer le même défaut. Deux
catégories attendues dans un même cas ne partagent aucun terme.

Un cas est refusé quand son plan cite, entre accents graves, un fichier ou
un test que son projet ne contient pas : un relecteur signalerait à juste
titre la lacune, et le rôle se verrait compter un faux positif. Une étape
de plan peut nommer ce qu'elle va créer sous `planned` ; un cas
`code.review`, dont les tâches sont faites, ne planifie rien.
`cargo run -p codev-evals -- list` charge tous les cas et nomme ce qui ne
va pas.

### Écrire un cas de planification

Un cas de planification rejoue une demande sur ce dépôt tel qu'il était
avant un change archivé : son répertoire ne contient que `case.yaml`,
avec `step: plan.write`, `kind: seeded`, `ref` (le commit sur lequel la
demande est planifiée — le parent du commit de fusion du change),
`request` (les mots de l'utilisateur, dans n'importe quelle langue,
jamais copiés de la proposition archivée : un test refuse plus de douze
mots d'affilée de celle-ci), `summary`, et `expected` — `modified`, les
exigences sur lesquelles porte la demande, `collateral`, celles que le
vrai change a modifiées au passage, chacune sous la forme
`<capacité>::<nom de l'exigence>`, et `decisions`, chacune avec son `id`
et `expect` (`follow` ou `supersede`). Le lanceur reconstruit le projet à
partir de `git archive <ref>` et y installe les skills du codev testé :
la suite demande donc l'historique complet. Un clone superficiel ne
charge pas les cas et nomme le commit (`git fetch --unshallow`). Le
chargement refuse aussi une exigence ou une décision qui n'existait pas
à `ref`, et un `ref` dont l'arbre contient déjà le cas.

### Écrire un cas de révision

Un cas de révision rejoue la demande d'un auteur qui veut réviser un
artefact d'un change actif avec `/codev-update`. Aucun commit de `main` ne
contient de change actif — les pull requests sont fusionnées en un seul
commit avec leur archivage —, aussi le cas apporte-t-il le change : son
`change/` contient les fichiers d'un change archivé, copiés tels que
l'archive les garde, que le lanceur écrit dans `_codev/changes/<change>/`
sur l'arbre de `ref`, le parent du commit qui a amené le change sur
`main`. `case.yaml` donne `step: plan.update`, `kind: seeded`, `ref`,
`change` (le nom du change), `artifact` (`proposal`, `specs`, `design` ou
`tasks`), `request` (les mots de l'auteur, dans n'importe quelle langue),
`summary`, et `expected`, dont les chemins sont relatifs au dossier du
change : `revised`, les fichiers que la révision doit modifier ; `kept`,
ceux qu'elle doit laisser identiques à l'octet près ; `ripple`, ceux dont
l'agent doit nommer la conséquence sans les modifier ; `contains`, un texte
que les fichiers révisés doivent contenir ; et `refuse`, pour une demande
que la skill doit refuser (le cas n'attend alors ni chemin `revised` ni
`contains`). Le chargement refuse un chemin que `change/` ne contient pas,
et un `ref` dont l'arbre contient déjà le change ou le cas.

Avant l'exécution, le lanceur enregistre `codev validate <change> --json`
sur le projet ; le prompt nomme la skill, le change, l'artefact et la
demande, et dit de ne poser aucune question, de signaler une répercussion
sans l'appliquer et de ne rien modifier hors du dossier du change. Une
exécution est notée sans modèle, par des vérifications qu'un lecteur peut
refaire à la main : chaque fichier `revised` diffère de `change/` ; chaque
fichier `kept` et `ripple` est inchangé ; chaque chemin `ripple` est nommé
dans les messages de l'agent — tous les messages qu'il a écrits, appels
d'outils et leurs sorties exclus ; chaque texte `contains` figure dans un
fichier révisé, sans tenir compte de la casse ni des suites d'espaces ;
`codev validate` ne signale aucune erreur que le projet n'avait pas déjà ;
et un refus laisse le dossier du change tel quel et nomme `/codev-propose`.
Une exécution réussit quand toutes ses vérifications réussissent. La
section `plan.update` du rapport donne, par répétition, la part des
exécutions qui réussissent, la part des vérifications qui réussissent et
les exécutions qui modifient un fichier hors du dossier du change, ainsi
que les vérifications de chaque exécution ; `compare` échoue quand la part
des exécutions qui réussissent passe sous le minimum de la ligne de base,
ou quand les exécutions qui modifient un fichier hors de l'étape dépassent
son maximum.

### Écrire un cas d'implémentation

Un cas d'implémentation rejoue l'implémentation d'un change archivé avec
`/codev-apply`. `case.yaml` donne `step: code.implement`, `kind: seeded`,
`ref` (le parent du commit qui a apporté le change sur `main`), `merged`
(ce commit), `change`, `summary` et `expected` : `refuse`, pour un plan que
la skill doit refuser d'implémenter ; `docs`, quand l'exécution doit
toucher `docs/en/` et `docs/fr/` ; et `acceptance`, les noms des assertions
d'acceptation. Son `change/` contient les fichiers de planification du
change archivé, toutes les tâches décochées et sans `review.md` ; une
tâche dont la vérification demande un outil absent du bac à sable, comme
`mdbook`, est reformulée en une vérification que le bac à sable peut
lancer, et `summary` le dit. Un cas de refus n'a ni `tasks.md` ni script.

À côté de `case.yaml`, `acceptance.sh` est la vérification cachée : écrit
d'après les scénarios des specs delta du change, il est lancé par `bash`
dans un dossier vide, `CODEV` nommant le binaire construit, en 300 secondes
au plus, et affiche une ligne par assertion, `ok <nom>` ou `not ok <nom>` ;
une assertion pour laquelle il n'affiche pas `ok` échoue. Avant d'utiliser
un cas, `cargo run -p codev-evals -- calibrate <cas>` construit `merged` et
`ref` et doit sortir avec 0 : toutes les assertions `ok` sur `merged`, son
format, son lint et ses tests passant avec la chaîne d'outils actuelle, et
au moins une assertion en échec sur `ref`.

Le runner pose `change/` sur l'arbre de `ref`, installe les skills,
télécharge et construit les dépendances dans un Cargo home à l'intérieur du
projet, enregistre `codev validate <change> --json`, puis lance Claude Code
hors ligne dans son bac à sable, avec les outils qu'accorde la skill
`codev-apply` installée. Après l'exécution, il formate, analyse, teste et
construit lui-même le projet, puis lance le script. Ces commandes exécutent
hors du bac à sable du code écrit par l'agent : chacune démarre avec un
environnement vidé — le chemin de recherche, le dossier personnel, les
variables propres à la chaîne d'outils Rust, le dossier temporaire et la
locale, jamais la clé d'API ni un jeton — et un délai de 1 200 secondes.
Ne lancez cette étape que sur vos propres cas, sur une machine où c'est
acceptable. Une exécution réussit quand toutes les tâches sont cochées, que
les quatre commandes passent, que toutes les assertions sont `ok`, que le
compte rendu de revue contient `code.review`, que `codev validate`
n'ajoute aucune erreur, que rien n'a changé sous `_codev/` hors du change,
que le change n'est pas archivé, que la documentation est touchée quand
c'est attendu, et que l'agent n'a rien lu du dépôt d'où viennent les cas —
ses lectures et recherches sont relevées dans ses appels d'outils. La
section `code.implement` du rapport met les cas de refus à part ;
`compare` échoue quand la part des exécutions qui réussissent, celle des
assertions d'acceptation ou celle des refus qui réussissent passe sous le
minimum de la ligne de base.

Changer un terme du vocabulaire change les scores : renotez la ligne de
base (`cargo run -p codev-evals -- score evals/baseline`) et dites quels
constats il reclasse.

N'ajustez pas un rôle et ses cas dans la même pull request : un cas dit
ce qu'un relecteur devrait trouver, indépendamment de la formulation du
rôle. Quand les règles de notation ou les cas changent, enregistrez de
nouveau la ligne de base, en local, avec la version de Claude Code
qu'épingle `.github/workflows/evals.yml`
(`npm install -g @anthropic-ai/claude-code@<cette version>` ; un test
vérifie `evals/baseline/run.json` face à l'épinglage) — supprimez
`evals/baseline/`, puis
`cargo run -p codev-evals -- run --out evals/baseline --repeat 5 --jobs 4`
(environ 270 dollars et plusieurs heures, cas de planification compris) —,
ce que les tests vérifient : cinq exécutions de chaque cas, aucune en
erreur. Les cas d'une nouvelle étape — les cas de révision à l'arrivée de
`plan.update`, les cas d'implémentation avec `code.implement` — peuvent
être enregistrés seuls, dans la ligne de base
existante, quand les autres cas et la notation n'ont pas changé : avec le
Claude Code épinglé et le codev que nomme la ligne de base,
`cargo run -p codev-evals -- run --step plan.update --out evals/baseline --repeat 5 --jobs 4`
la reprend en n'y ajoutant que les exécutions des nouveaux cas (une reprise
refuse une autre version de Claude Code), puis lancez
`cargo run -p codev-evals -- score evals/baseline` et
`cargo run -p codev-evals -- docs`. Un cas modifié ou une notation modifiée
demande toujours d'enregistrer de nouveau toute la ligne de base. La sortie brute de Claude Code de chaque exécution
(`claude.jsonl`, `claude.stderr`) reste hors du contrôle de version.

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

Les encadrés sont des citations qui commencent par un titre en gras :
`> **Note**` ou `> **Warning**` en anglais, `> **Remarque**` ou
`> **Attention**` en français. Le site et `codev docs` affichent les remarques
avec un filet vert et les avertissements avec un filet corail.

Pour prévisualiser le site, installez [mdBook](https://rust-lang.github.io/mdBook/)
et lancez `mdbook serve docs/fr`.

Les changements visibles par les utilisateurs sont décrits dans la section
`## Release note` de leur proposition. Une fois le changement archivé,
`codev changelog --write` les ajoute à la section `## [Unreleased]` de
[CHANGELOG.md](CHANGELOG.md) ; `codev doctor` signale quand elle est en retard.

## Processus de release

Pour les mainteneurs :

1. Vérifiez que `main` est au vert et que `codev doctor` indique `changelog`
   ok.
2. Incrémentez `version` dans la section `[workspace.package]` du
   `Cargo.toml` racine, puis lancez `cargo build` pour mettre à jour
   `Cargo.lock`.
3. Lancez `codev changelog --release X.Y.Z --write` : les éléments générés de
   `[Unreleased]` deviennent la section `## [X.Y.Z] - YYYY-MM-DD` de
   `CHANGELOG.md`, sous une nouvelle section `## [Unreleased]` vide.
   Relisez-la, et mettez à jour les liens en bas du fichier.
4. Commitez avec le message `chore(release): vX.Y.Z`.
5. Créez le tag et poussez aussitôt — tant que le tag n'existe pas,
   `codev changelog` compte encore les changements publiés comme non publiés :

   ```bash
   git tag vX.Y.Z
   git push origin main --tags
   ```

Le workflow de release compile les quatre binaires précompilés (macOS arm64 et
x86_64, Linux x86_64 musl, Windows x86_64), calcule `SHA256SUMS` et publie la
GitHub Release. Son texte est `codev changelog --until vX.Y.Z --format markdown`,
produit par le binaire compilé depuis le tag, suivi des notes générées par
GitHub. Les installateurs prennent en compte la nouvelle version
immédiatement.

**Les codev plus anciens.** Un même dépôt est lu par plusieurs versions de
codev à la fois : un change qui ajoute une clé, une valeur ou un format à un
fichier que codev lit suit donc la
[décision 0013](_codev/decisions/0013-a-file-written-for-a-newer-codev-warns-unless-ignoring-it-would-weaken-a-check.md).
Son design indique ce qu'en fait un codev plus ancien — il l'ignore avec un
avertissement, la refuse, ou la lit de travers — et, lorsque l'ignorer
affaiblirait une vérification, sa note de version donne le `codev_version`
minimal qu'un projet doit exiger avant de l'utiliser.

## Sécurité

N'ouvrez pas d'issue publique pour signaler une vulnérabilité. Suivez plutôt
[SECURITY.md](SECURITY.md) (en anglais).

## Code de conduite

La participation à ce projet est régie par le
[code de conduite](CODE_OF_CONDUCT.md) (en anglais).
