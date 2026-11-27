# Déployer Compagnon pour une équipe

Cette page s'adresse à la personne qui installe Compagnon pour une équipe :
tout le monde installe la même version, le [dépôt](../glossary.md#dépôt)
contient ce que l'équipe partage, une vérification bloque une
[demande de fusion](../glossary.md#demande-de-fusion) dont le plan n'est
pas valide, et chacun sait qui approuve quoi.

## L'installer sur chaque poste

Chaque développeur qui fait coder Claude a besoin de la commande
`compagnon` et de Claude Code. [Installation](../installation.md) donne les
commandes d'installation en une ligne pour macOS, Linux et Windows, et la
façon d'installer une version précise (`COMPAGNON_VERSION=0.6.1`).

Les personnes qui ne font que relire les plans n'installent rien : elles
lisent et approuvent le plan dans la demande de fusion, sur GitHub —
voir [Approuver un plan sur GitHub](approve-a-plan.md).

## Une seule version pour tous

Indiquez les versions que l'équipe accepte dans `_compagnon/config.yaml` :

```yaml
compagnon_version: ">=0.6, <0.8"
```

Pour exiger une seule version précise, écrivez `"=0.6.1"`. Mettez la
valeur entre guillemets : YAML lit `>` de façon particulière. Avec une
version hors de cet intervalle, les compétences proposent de mettre à jour
avant de commencer, et `compagnon validate`, `compagnon sync` et
`compagnon archive` refusent de s'exécuter tant que le développeur n'a pas
mis à jour — voir
[`compagnon_version`](../reference/configuration.md#compagnon_version).

La tâche d'[intégration continue](../glossary.md#intégration-continue) que
`compagnon init` écrit installe une version fixée (`COMPAGNON_VERSION=`
dans `.github/workflows/compagnon.yml`). Quand vous relevez
`compagnon_version` au-dessus, relevez cette version dans le même
[enregistrement](../glossary.md#enregistrement). Compagnon 0.5 et les
versions antérieures ne connaissent pas cette clé et refusent le fichier :
ajoutez-la une fois que tout le monde utilise une version plus récente.
Voir [Annonce des mises à jour](../skills.md#annonce-des-mises-à-jour).

## Ce qu'il faut enregistrer

Lancez `compagnon init` une fois, sur un seul poste, et enregistrez ce
qu'il a écrit :

| Chemin | Pourquoi |
|---|---|
| `_compagnon/` | les spécifications, les décisions, les modifications et la configuration, y compris `_compagnon/decisions/seal.yaml` et `_compagnon/compagnon.lock` |
| `.claude/skills/` | chacun reçoit les mêmes compétences sans lancer `compagnon init` |
| `CLAUDE.md` | demande à Claude Code, à chaque session, de planifier avant de changer un comportement |
| `.github/workflows/compagnon.yml` | la vérification à chaque envoi et à chaque demande de fusion (GitHub) |
| le modèle de demande de fusion (`.github/PULL_REQUEST_TEMPLATE.md` ou le vôtre) | la ligne où le développeur nomme la modification (GitHub) |

Après une mise à jour de Compagnon, `compagnon update` rafraîchit les
compétences, les sections gérées et la version fixée dans l'intégration
continue : enregistrez aussi le résultat. Voir
[Faut-il enregistrer `_compagnon/` et `.claude/skills/` dans
git ?](../faq.md#faut-il-enregistrer-_compagnon-et-claudeskills-dans-git-) dans la FAQ.

## La vérification qui bloque une demande de fusion

La vérification que met en place `compagnon init` fait échouer un envoi ou
une demande de fusion dont une spécification ou une modification est
invalide, ou qui retouche une décision acceptée  ; ce
qu'il écrit est décrit plus bas, dans
[Ce qu'écrit compagnon init](#ce-quécrit-compagnon-init).

Une vérification en échec ne bloque la fusion qu'une fois rendue
obligatoire, dans les réglages de votre dépôt sur GitHub :

- **GitHub :** une règle de protection de la branche principale (ou un
  jeu de règles) qui exige que les vérifications passent (« Require status
  checks to pass before merging »), avec la vérification
  `compagnon validate`.

Vous pouvez aussi lancer `compagnon doctor --strict` dans l'intégration
continue : il échoue quand une vérification de l'installation manque — une
compétence pas à jour, la section de `CLAUDE.md` retirée — et jamais sur un
avertissement. Un nouveau projet commence sans spécification, ce que
`doctor` signale comme manquant : ajoutez cette étape une fois que
`compagnon doctor` ne signale plus rien.

## Qui approuve quoi

Compagnon s'arrête aux moments où une personne décide ; il ne sait pas qui,
dans votre équipe, doit être cette personne. Une règle qui fonctionne :

| Quoi | Qui approuve | Où |
|---|---|---|
| Le plan : proposition, spécifications, conception, tâches | le responsable produit ou l'expert du domaine, avec le développeur | dans la demande de fusion, avant que Claude construise — voir [Approuver un plan sur GitHub](approve-a-plan.md) |
| Le code | les développeurs, comme pour toute demande de fusion | dans la demande de fusion, une fois les tâches faites |
| Une [décision](../concepts.md#décision) | l'équipe | acceptée avec `compagnon decision accept` dans une demande de fusion que relisent ceux qu'elle engage |
| Un risque que la relecture a laissé ouvert | la personne qui en répond | la compétence pose la question et note la réponse avec le nom de cette personne |

C'est une suggestion, pas une règle que Compagnon impose : ce sont les
règles de votre site — approbations obligatoires, un fichier `CODEOWNERS`
qui désigne qui doit approuver `_compagnon/changes/` ou
`_compagnon/decisions/` — qui la rendent contraignante. La page
[Rôles](../roles.md#là-où-vous-décidez) liste chaque moment où Compagnon
passe la main.

## Les rôles de relecture

Avant qu'une personne lise le plan ou le code, les rôles de relecture de
Compagnon l'ont mis à l'épreuve, et leurs constats sont dans le fichier
`review.md` de la modification. Quels rôles s'exécutent, avec quels
critères et dans quel mode, se règle pour toute l'équipe dans
`_compagnon/config.yaml` — voir [Rôles](../roles.md).

## Ce qu'écrit compagnon init

### CLAUDE.md

Claude Code lit `CLAUDE.md` au début de chaque session. `compagnon init` le crée,
ou ajoute une section au vôtre, pour que Claude ne modifie pas un comportement
sans modification Compagnon :

```markdown
<!-- compagnon:begin — managed by compagnon; `compagnon update` rewrites this block, edit outside it -->
## Spec-driven development with compagnon

This repository follows spec-driven development with compagnon.

- `_compagnon/specs/` describes the current behavior and is authoritative.
- `_compagnon/decisions/` holds accepted architecture decisions: follow them, do not re-debate them.
- For any change of behavior, start with `/compagnon-propose`, then implement it with `/compagnon-apply`. Do not modify behavior without a compagnon change.
- Trivial fixes (typos, documentation wording) are exempt.
- Run `compagnon validate` before committing.

Installed skills: `/compagnon-propose`, `/compagnon-explore`, `/compagnon-onboard`, `/compagnon-apply`, `/compagnon-sync`, `/compagnon-archive`, `/compagnon-update`, `/compagnon-configure`.
<!-- compagnon:end -->
```

La section est en anglais quelle que soit la `language:` du projet : elle
s'adresse à l'agent. Lorsque `/compagnon-apply` n'est pas installée, elle demande
de mettre en œuvre en suivant le `tasks.md` de la modification.

### Tâche d'intégration continue

Sur un dépôt GitHub — doté d'un dossier `.github/`, ou d'un remote git sur
`github.com` —, `compagnon init` crée `.github/workflows/compagnon.yml` :

```yaml
# compagnon:managed — generated by `compagnon init`, refreshed by `compagnon update`.
# Delete this first line to take the file over: compagnon will then leave it alone.
name: compagnon

on:
  push:
  pull_request:

jobs:
  validate:
    name: compagnon validate
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v7
      - name: Install compagnon 0.6.1
        run: |
          curl -sSL https://raw.githubusercontent.com/mairistem/compagnon/main/install.sh | COMPAGNON_VERSION=0.6.1 sh
          echo "$HOME/.local/bin" >> "$GITHUB_PATH"
      - name: Validate specs and changes
        run: compagnon validate --strict
```

L'installateur est épinglé sur la version de Compagnon qui a écrit le fichier :
la CI valide avec les mêmes règles que votre machine. Compagnon n'écrit pas ce
fichier lorsque :

- un autre fichier de déroulé de `.github/workflows/` exécute déjà `compagnon validate` —
  toute ligne qui nomme `compagnon` puis `validate` compte, comme
  `cargo run --bin compagnon -- validate` ;
- `.github/workflows/compagnon.yml` existe et n'a pas été écrit par Compagnon (sa
  première ligne n'est pas `# compagnon:managed`).

Compagnon ne modifie jamais aucun autre fichier de déroulé.

### Modèle de demande de fusion

Sur GitHub, `compagnon init` ajoute une section à votre modèle de demande de
fusion (pull request template) —
le premier `pull_request_template.md` trouvé dans `.github/`, à la racine du
projet ou dans `docs/`, quelle que soit la casse — ou crée
`.github/PULL_REQUEST_TEMPLATE.md` :

```markdown
<!-- compagnon:begin — managed by compagnon; `compagnon update` rewrites this block, edit outside it -->
## compagnon

- compagnon change: <!-- folder under `_compagnon/changes/`, or "trivial fix, no change" -->
- [ ] `compagnon validate --strict` passes.
<!-- compagnon:end -->
```

## Sections gérées

Le texte de Compagnon dans `CLAUDE.md` et dans le modèle de demande de fusion se trouve
entre deux commentaires HTML, invisibles une fois le markdown affiché :

```markdown
<!-- compagnon:begin — managed by compagnon; `compagnon update` rewrites this block, edit outside it -->
…
<!-- compagnon:end -->
```

- Tout ce qui se trouve hors des marqueurs vous appartient et n'est jamais
  modifié. Une section ajoutée à un fichier existant se place à la fin, après
  une ligne vide ; vous pouvez déplacer le bloc entier où vous voulez.
- `compagnon update` réécrit ce qui se trouve entre les marqueurs, et rien
  d'autre : vos modifications à l'intérieur du bloc sont perdues à la
  prochaine mise à jour.
- Lorsque les marqueurs sont déséquilibrés ou répétés, le fichier est laissé
  tel quel et l'avertissement `managed_section_malformed` le nomme.

Le fichier de déroulé appartient entièrement à Compagnon. `compagnon update` le
réécrit, ce qui aligne sa version épinglée après une mise à jour, tant que sa
première ligne est `# compagnon:managed`. Supprimez cette ligne pour reprendre la
main sur le fichier : Compagnon n'y touche plus.

`compagnon update` ne rafraîchit que ce qui existe. Il ne crée jamais `CLAUDE.md`,
le fichier de déroulé ou le modèle, et n'ajoute jamais de section à un fichier qui n'en
a pas.

## Refuser l'intégration

| Option | Écarte |
|---|---|
| `--no-claude-md` | la section dans `CLAUDE.md` |
| `--no-ci` | `.github/workflows/compagnon.yml` |
| `--no-pr-template` | la section dans le modèle de demande de fusion |
| `--no-integration` | les trois |

Ces options valent pour une exécution de `compagnon init`. Un `compagnon init`
ultérieur sans elles écrit les fichiers ; `compagnon update`, jamais.

## compagnon doctor

`compagnon doctor` vérifie l'ensemble de l'installation et donne, pour chaque
manque, la commande ou la compétence qui le comble. La commande est en lecture
seule et fonctionne hors ligne ; pour la vérification `changelog`, elle
demande au dépôt git local quelles modifications ont été archivées depuis le dernier
tag :

```bash
compagnon doctor
```

```text
compagnon doctor — /home/you/acme-app

Configuration
  ✗ config.rules     no rules: in _compagnon/config.yaml
      fix: /compagnon-configure
  ✓ config.language  language: is set
  ✓ config.keys      no unknown key in _compagnon/config.yaml
  ✓ config.roles     active roles: analyst, investigator, product, qa, architect, lead, tracer, skeptic, developer, newcomer; on domain: security (security), privacy (personal-data), accessibility (user-interface), performance (performance), ops (operations), migration (migration); disabled: none
  ✓ skills           8 skill(s) installed and current
  ✓ sources          no inherited git source
  ✓ compagnon_version    no newer compagnon release known

Planning
  ✗ specs            no main spec in _compagnon/specs/
      fix: /compagnon-propose <a change> — documenting existing behavior is planned work too
  ! decisions        no accepted decision (not every project needs one)
      fix: compagnon decision new "<title>"
  ✓ validate         compagnon validate reports no issue

Repository
  ✓ claude_md        CLAUDE.md has the compagnon section
  ✓ ci               .github/workflows/compagnon.yml runs compagnon validate
  ✓ pr_template      the pull request template has the compagnon section
  ✓ changelog        CHANGELOG.md matches the archived changes

14 check(s), 2 to fix, 1 warning(s)
```

`✗` signale un élément manquant, `!` un avertissement — un projet peut très
bien n'avoir aucune décision acceptée. La vérification `pr_template`
n'apparaît que pour un dépôt GitHub. La vérification `changelog`, simple
avertissement, compare la section `[Unreleased]` de `CHANGELOG.md` avec ce
que génère `compagnon changelog` ; la correction est `compagnon changelog --write` —
voir [Notes de version et CHANGELOG](release-notes.md). La vérification `compagnon_version` est manquante
lorsque le Compagnon en cours ne satisfait pas le
[`compagnon_version`](../reference/configuration.md#compagnon_version) du projet, et
devient un avertissement lorsque le cache de vérification connaît une version publiée
plus récente ; la correction est `compagnon self-update`, ou
`compagnon self-update --version <X.Y.Z>` lorsque le Compagnon en cours est plus récent
que ce que l'exigence autorise. La vérification `config.keys` signale les clés
de `_compagnon/config.yaml` que ce Compagnon ne connaît pas — voir
[Clés inconnues](../reference/configuration.md#clés-inconnues). La liste complète des
vérifications se trouve dans la
[référence de la CLI](../reference/cli.md#compagnon-doctor).

`compagnon init` et `compagnon status` se terminent par une ligne d'indication
lorsqu'il manque quelque chose, et `/compagnon-onboard` liste les éléments
manquants avec leur correction.

En CI, `compagnon doctor --strict` sort avec le code `1` lorsqu'une vérification
manque ; les avertissements ne font jamais échouer la commande.
`compagnon doctor --json` fournit le même rapport aux scripts, avec des
identifiants et des codes stables — voir [Sortie JSON](../reference/json-output.md#doctor).

## Pour aller plus loin

- [Rôles](../roles.md)
- [Configuration](../reference/configuration.md)
- [Sources héritées](inherited-sources.md)
- [Approuver un plan sur GitHub](approve-a-plan.md)

## Étape suivante

[Sources héritées](inherited-sources.md)
