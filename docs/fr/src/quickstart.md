# Votre premier change

Cette page vous mène d'un dépôt vide à votre premier change terminé, en une
dizaine de minutes. Vous parlez à Claude avec vos mots : chaque étape donne
la phrase à dire, puis la commande raccourcie, qui fait la même chose.

Il vous faut [Claude Code](glossary.md#claude-code) et un dépôt où
travailler. L'exemple ajoute un mode sombre à une application ; prenez votre
propre idée.

## 1. Installer Compagnon

Sur macOS ou Linux :

```bash
curl -sSL https://raw.githubusercontent.com/mairistem/compagnon/main/install.sh | sh
```

Sur Windows, dans PowerShell :

```powershell
iwr -useb https://raw.githubusercontent.com/mairistem/compagnon/main/install.ps1 | iex
```

Les autres façons d'installer sont dans [Installation](installation.md).

## 2. Préparer votre dépôt

À la racine de votre dépôt, lancez une fois :

```bash
compagnon init
```

Il peut poser deux questions — quels workflows installer, et quelques mots
sur votre projet — puis ajoute un dossier `_compagnon/` et apprend à Claude
Code les skills de Compagnon. Redémarrez Claude Code pour qu'il les voie.

<details>
<summary>Sous le capot : ce qu'écrit init</summary>

```bash
compagnon init --yes --language fr
```

```text
compagnon initialized in /home/you/acme-app

  Structure   _compagnon/ — specs, decisions, changes, schemas
  Skills      compagnon-propose, compagnon-explore, compagnon-onboard, compagnon-apply, compagnon-sync, compagnon-archive, compagnon-update, compagnon-configure → .claude/skills/

  Repository integration
    + CLAUDE.md                          created
    + .github/workflows/compagnon.yml        created
    + .github/PULL_REQUEST_TEMPLATE.md   created

  17 file(s) created, 0 updated

→ Recommended next step: in Claude Code, run /compagnon-configure.
```

La détection, les questions et chaque fichier écrit sont décrits dans
[Ce que fait `compagnon init`](reference/cli.md#ce-que-fait-compagnon-init).
`compagnon doctor` liste ensuite ce qui manque encore à l'installation, avec
la correction de chaque point.

</details>

## 3. Demander un change (`/compagnon-propose`)

Dans Claude Code, dites ce que vous voulez :

> Prépare un plan pour un mode sombre qui suit le réglage du système.

Claude rédige un plan et s'arrête : il ne modifie encore aucun code. Le
raccourci fait la même chose : `/compagnon-propose ajoute un mode sombre qui
suit le réglage du système`.

<details>
<summary>Sous le capot : comment le plan est rédigé</summary>

La skill nomme le change (`add-dark-mode`), crée son dossier, puis demande à
Compagnon, pour chaque partie du plan, quoi écrire et dans quelle langue :

```bash
compagnon new change add-dark-mode
compagnon instructions proposal --change add-dark-mode
```

```text
Artifact: proposal (change `add-dark-mode`, schema spec-driven)
Write to: /home/you/acme-app/_compagnon/changes/add-dark-mode/proposal.md
Language: fr — prose and structural keywords in fr
```

</details>

## 4. Relire le plan

Le plan se trouve dans `_compagnon/changes/add-dark-mode/`, en quatre
fichiers courts : la **proposition** (pourquoi), les **specs** (ce qui sera
vrai ensuite), la **conception** (comment) et les **tâches** (les étapes).
Lisez-les comme la proposition d'un collègue. Pour changer quelque chose,
dites-le simplement :

> Dans le plan, garde le choix de l'utilisateur quand il choisit un thème à la main.

[Relire un plan](guides/review-a-plan.md) vous dit quoi regarder dans chaque
fichier, sans terminal.

<details>
<summary>Sous le capot : vérifier le plan</summary>

```bash
compagnon status --change add-dark-mode
compagnon validate add-dark-mode
```

`status` montre les quatre parties comme rédigées ; `validate` vérifie leur
format. Un change sans spec échoue volontairement avec
`zero_delta_without_marker`.

</details>

## 5. Laisser Claude le réaliser (`/compagnon-apply`)

Une fois le plan à votre goût :

> Le plan me va, implémente-le.

Claude traite les tâches une à une, vérifie chacune avec son test, et
s'arrête pour vous demander si quelque chose n'est pas clair. Raccourci :
`/compagnon-apply add-dark-mode`.

## 6. Le classer (`/compagnon-archive`)

Vérifiez le résultat, puis :

> Le change est terminé, archive-le.

Claude fusionne les specs du plan dans celles du projet et range le change
dans l'archive : le projet se souvient désormais de la raison d'être du mode
sombre. Raccourci : `/compagnon-archive add-dark-mode`. Commitez le
résultat.

<details>
<summary>Sous le capot : ce que fait l'archivage</summary>

```bash
compagnon archive --change add-dark-mode
```

```text
Archived change `add-dark-mode`

Created:
  /home/you/acme-app/_compagnon/specs/ui/theme/spec.md

Moved to: /home/you/acme-app/_compagnon/changes/archive/2026-09-30-add-dark-mode
```

</details>

## Et ensuite

- [Le workflow](workflow.md) détaille chaque étape.
- [Les skills Claude Code](skills.md#comment-demander) donne, pour chaque
  skill, des façons de la demander.
- Dites « fais-moi visiter Compagnon » pour une visite guidée de votre
  projet.
