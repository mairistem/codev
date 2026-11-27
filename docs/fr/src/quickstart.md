# Votre première modification

Cette page vous mène d'un
[dépôt](glossary.md#dépôt) vide à votre première modification terminée, en une
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

Ouvrez un [terminal](glossary.md#terminal) (la fenêtre où l'on tape des
commandes) dans le dossier principal de votre projet, puis tapez une fois :

```bash
compagnon init
```

Il peut vous demander quelles [compétences](glossary.md#compétence)
installer, et quelques mots sur votre projet. Puis il ajoute un dossier
`_compagnon/` et donne les compétences à Claude Code. Redémarrez Claude Code
pour qu'il les voie.

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

→ Recommended next step: in Claude Code, ask
  "configure Compagnon for this project" (shortcut: /compagnon-configure).
```

La détection, les questions et chaque fichier écrit sont décrits dans
[Ce que fait `compagnon init`](reference/cli.md#ce-que-fait-compagnon-init).
`compagnon doctor` liste ensuite ce qui manque encore à l'installation, avec
la correction de chaque point.

</details>

## 3. Demander une modification (`/compagnon-propose`)

Dans Claude Code, dites ce que vous voulez :

> Prépare un plan pour un mode sombre qui suit le réglage du système.

Claude rédige un plan et s'arrête : il ne modifie encore aucun code. Le
raccourci fait la même chose : `/compagnon-propose ajoute un mode sombre qui
suit le réglage du système`.

**Vous avez un ticket Jira ?** Nommez-le :

> Prépare un plan pour PROJ-123.

Claude le lit, sans le modifier, et le cite en tête du plan
([Partir d'un ticket Jira](guides/mcp.md)).

<details>
<summary>Sous le capot : comment le plan est rédigé</summary>

La compétence nomme la modification (`add-dark-mode`), crée son dossier, puis demande à
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
fichiers courts : la **proposition** (pourquoi), les **spécifications** (ce qui sera
vrai ensuite), la **conception** (comment) et les **tâches** (les étapes).
Lisez-les comme la proposition d'un collègue. Pour changer quelque chose,
dites-le simplement :

> Dans le plan, garde le choix de l'utilisateur quand il choisit un thème à la main.

<details>
<summary>Sous le capot : vérifier le plan</summary>

```bash
compagnon status --change add-dark-mode
compagnon validate add-dark-mode
```

`status` montre les quatre parties comme rédigées ; `validate` vérifie leur
format. Une modification sans spécification échoue volontairement avec
`zero_delta_without_marker`.

</details>

## 5. Laisser Claude la réaliser (`/compagnon-apply`)

Une fois le plan à votre goût :

> Le plan me va, réalise la modification.

Claude traite les tâches une à une, vérifie chacune avec son
[test](glossary.md#test) (un petit programme qui prouve que la tâche
fonctionne), et s'arrête pour vous demander si quelque chose n'est pas clair. Raccourci :
`/compagnon-apply add-dark-mode`.

## 6. La classer (`/compagnon-archive`)

Vérifiez le résultat, puis :

> La modification est terminée, archive-la.

Claude reporte les spécifications du plan dans celles du projet et range
la modification dans l'archive. Le projet se souvient désormais de la
raison d'être du mode sombre. Raccourci : `/compagnon-archive
add-dark-mode`. Enregistrez ensuite le résultat dans l'historique du projet
([Git](glossary.md#git)), ou demandez-le à Claude.

<figure class="diagram">
<div class="scroll">
<svg viewBox="0 0 760 220" role="img" aria-label="Le dossier d'une modification : _compagnon/changes/&lt;nom&gt;/ contient la proposition, les spécifications, la conception et les tâches ; l'archivage le déplace vers _compagnon/changes/archive/&lt;date&gt;-&lt;nom&gt;/ et met à jour les spécifications du projet.">
<g stroke-width="2" style="fill:var(--dg-file)">
<rect x="8" y="20" width="300" height="180" rx="12" style="stroke:var(--dg-claude)"/><rect x="420" y="20" width="332" height="80" rx="12" style="stroke:var(--dg-line)" stroke-dasharray="5 4"/><rect x="420" y="120" width="332" height="80" rx="12" style="stroke:var(--dg-line)"/>
</g>
<path d="M312,110h40M352,110V60h62m-6,-5l6,5l-6,5M352,110V160h62m-6,-5l6,5l-6,5" fill="none" stroke-width="2" style="stroke:var(--dg-line)"/>
<g class="mono" font-size="13" style="fill:var(--dg-ink)">
<text x="24" y="48" font-weight="600">_compagnon/changes/&lt;nom&gt;/</text><text x="40" y="84">proposal.md</text><text x="40" y="112">specs/…/spec.md</text><text x="40" y="140">design.md</text><text x="40" y="168">tasks.md</text>
<text x="436" y="48" font-weight="600">_compagnon/changes/archive/</text><text x="436" y="70">&lt;date&gt;-&lt;nom&gt;/</text><text x="436" y="148" font-weight="600">_compagnon/specs/…/spec.md</text>
</g>
<g font-size="13" style="fill:var(--dg-muted)">
<text x="316" y="100">archiver</text><text x="436" y="90">conservé tel quel</text><text x="436" y="176">mis à jour par le plan</text>
</g>
</svg>
</div>
<figcaption>Du début à l'archive : le plan est conservé, et les spécifications du projet disent ce qu'il a changé.</figcaption>
</figure>

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

## Pour aller plus loin

- [Comment ça marche](workflow.md) : chaque étape en détail.
- [Parler à Claude](skills.md#comment-demander) : d'autres façons de
  demander.
- [Partir d'un ticket Jira](guides/mcp.md)
- [Relire un plan](guides/review-a-plan.md) : quoi vérifier dans un plan.
