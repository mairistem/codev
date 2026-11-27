# Votre première modification

<div class="skip-step">

[Vous ne codez pas ? Passez à Parler à Claude](skills.md)

</div>

Cette page vous mène d'un projet préparé à votre première modification
terminée, en une dizaine de minutes. Vous parlez à Claude avec vos mots :
chaque étape donne la phrase à dire, puis la commande raccourcie, qui fait
la même chose.

Il vous faut Compagnon installé et `compagnon init` lancé dans votre projet
([Installation](installation.md)), et Claude Code ouvert dessus. L'exemple
ajoute un mode sombre à une application ; prenez votre propre idée.

<div class="example">
<p class="example-title">Exemple</p>

```text
> Ajoute un mode sombre qui suit le réglage du système
⏺ Bash(compagnon new change add-dark-mode)
  ⎿ Change `add-dark-mode` created
⏺ Le plan est prêt dans _compagnon/changes/add-dark-mode/. Lisez d'abord proposal.md.
> Le plan me va, réalise-le
⏺ Update(src/theme.ts)
  ⎿ Updated src/theme.ts with 18 additions and 2 removals
⏺ Bash(npm test)
  ⎿ ✓ 42 tests passed
⏺ 7 tâches sur 7 faites · 2 remarques de relecture, rien à trancher.
> La modification est terminée, archive-la
⏺ Bash(compagnon archive --change add-dark-mode)
  ⎿ ✓ Archived · moved to _compagnon/changes/archive/2026-11-27-add-dark-mode
```

Trois phrases de votre part ; les étapes ci-dessous les prennent une à une.

</div>

## 1. Demander une modification (`/compagnon-propose`)

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

## 2. Relire le plan

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

## 3. Laisser Claude la réaliser (`/compagnon-apply`)

Une fois le plan à votre goût :

> Le plan me va, réalise la modification.

Claude traite les tâches une à une, vérifie chacune avec son
[test](glossary.md#test) (un petit programme qui prouve que la tâche
fonctionne), et s'arrête pour vous demander si quelque chose n'est pas clair. Raccourci :
`/compagnon-apply add-dark-mode`.

## 4. La classer (`/compagnon-archive`)

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
<path d="M312,110h60M372,110V60h42m-6,-5l6,5l-6,5M372,110V160h42m-6,-5l6,5l-6,5" fill="none" stroke-width="2" style="stroke:var(--dg-line)"/>
<g class="mono" font-size="13" style="fill:var(--dg-ink)">
<text x="24" y="48" font-weight="600">_compagnon/changes/&lt;nom&gt;/</text><text x="40" y="84">proposal.md</text><text x="40" y="112">specs/…/spec.md</text><text x="40" y="140">design.md</text><text x="40" y="168">tasks.md</text>
<text x="436" y="48" font-weight="600">_compagnon/changes/archive/</text><text x="436" y="70">&lt;date&gt;-&lt;nom&gt;/</text><text x="436" y="148" font-weight="600">_compagnon/specs/…/spec.md</text>
</g>
<g font-size="13" style="fill:var(--dg-muted)">
<text x="316" y="102" font-size="12">archiver</text><text x="436" y="90">conservé tel quel</text><text x="436" y="176">mis à jour par le plan</text>
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

## Étape suivante

[Parler à Claude](skills.md)
