# Compagnon en deux minutes

**Planifiez avec Claude avant qu'il ne code.**

Compagnon vous aide à travailler avec Claude Code, l'assistant de
programmation d'Anthropic, sans perdre la main. Avant d'écrire du code,
Claude rédige un plan court que vous pouvez lire et corriger. Une fois que
vous êtes d'accord, Claude construit exactement cela, et le plan reste dans
votre [dépôt](glossary.md#dépôt) (le dossier qui contient les fichiers du
projet) comme trace de ce qui a été fait, et pourquoi.

Inutile de savoir comment fonctionne l'IA. Vous demandez avec vos mots —
« ajoute un mode sombre qui suit le réglage du système » — et Compagnon
guide Claude pour le reste. Votre travail est dans Jira ? Dites « Prépare
un plan pour PROJ-123 » : Claude lit le ticket, sans le modifier, et le
cite en tête du plan ([Partir d'un ticket Jira](guides/mcp.md)).

<div class="example">
<p class="example-title">Exemple</p>

```text
> Ajoute un mode sombre qui suit le réglage du système
⏺ J'écris d'abord un plan : pourquoi, ce qui change, comment, et les tâches.
⏺ Le plan est prêt dans _compagnon/changes/mode-sombre/. Lisez d'abord
  proposal.md ; rien n'est construit avant votre accord.
> Le plan me va, réalise-le
⏺ 7 tâches sur 7 faites, chacune testée.
```

Vous avez demandé avec vos mots ; Claude a planifié, attendu, puis construit.

</div>

## Qui décide

Vous. Claude propose, vous décidez, et rien n'est construit avant votre
accord :

<figure class="diagram">
<div class="scroll">
<svg viewBox="0 0 760 150" role="img" aria-label="Cinq étapes : vous demandez, Claude planifie, vous validez, Claude construit, la modification est classée.">
<g stroke-width="2" style="fill:var(--dg-box)">
<g style="stroke:var(--dg-you)"><rect x="4" y="16" width="132" height="118" rx="14"/><rect x="310" y="16" width="132" height="118" rx="14"/></g>
<g style="stroke:var(--dg-claude)"><rect x="157" y="16" width="132" height="118" rx="14"/><rect x="463" y="16" width="132" height="118" rx="14"/></g>
<rect x="616" y="16" width="140" height="118" rx="14" style="stroke:var(--dg-line)" stroke-dasharray="5 4"/>
</g>
<path d="M139,75h14m-6,-5l6,5l-6,5M292,75h14m-6,-5l6,5l-6,5M445,75h14m-6,-5l6,5l-6,5M598,75h14m-6,-5l6,5l-6,5" fill="none" stroke-width="2" style="stroke:var(--dg-line)"/>
<g class="mono" font-size="12" style="fill:var(--dg-muted)">
<text x="20" y="44">1 · VOUS</text><text x="173" y="44">2 · CLAUDE</text><text x="326" y="44">3 · VOUS</text><text x="479" y="44">4 · CLAUDE</text><text x="632" y="44">5 · CONSERVÉ</text>
</g>
<g font-size="16" font-weight="600" style="fill:var(--dg-ink)">
<text x="20" y="78">Demander</text><text x="173" y="78">Planifier</text><text x="326" y="78">Valider</text><text x="479" y="78">Construire</text><text x="632" y="78">Classer</text>
</g>
<g font-size="13" style="fill:var(--dg-muted)">
<text x="20" y="104">avec vos mots</text><text x="173" y="104">pourquoi,</text><text x="173" y="121">comment, tâches</text><text x="326" y="104">ou le corriger</text><text x="479" y="104">tâche par tâche</text><text x="632" y="104">avec le code</text>
</g>
</svg>
</div>
<figcaption>Les étapes en corail sont les vôtres, celles en bleu sont à Claude ; le plan est conservé avec le code.</figcaption>
</figure>

Chaque décision est consignée là où toute votre équipe peut la lire.

## Le nom

Compagnon vient du compagnonnage, la tradition des métiers d'art : un
maître mène l'ouvrage, les compagnons le réalisent, et chacun suit des plans
arrêtés à l'avance. Ici, vous êtes le maître, et les [agents](glossary.md#agent) de Claude
sont vos compagnons.

## Ce que vous y gagnez

- **Moins de surprises.** Vous voyez ce que Claude va faire avant qu'il le
  fasse.
- **Une mémoire partagée.** Les plans et les décisions restent dans le
  dépôt, en texte simple : chacun peut comprendre plus tard pourquoi une
  chose a été construite.
- **Des décisions qui tiennent.** Une fois qu'un choix est tranché par votre
  équipe, Claude le suit au lieu de le remettre en cause à chaque
  modification.

## Par où commencer

La documentation se lit comme un parcours en cinq étapes. Les étapes 1 à 3
suffisent pour travailler au quotidien ; les étapes 4 et 5 sont pour qui
veut aller plus loin. Chaque page se termine par « Étape suivante » ; une
page dont certains lecteurs n'ont pas besoin s'ouvre sur un bouton qui dit
qui peut la passer.

<figure class="diagram">
<div class="scroll">
<svg viewBox="0 0 760 190" role="img" aria-label="Le parcours de lecture en cinq étapes : 1, Découvrir, une page ; 2, Démarrer, deux pages ; 3, Au quotidien, cinq pages ; 4, Comprendre, cinq pages ; 5, Pour l'équipe, six pages. Les étapes 1 à 3 sont pour tous, les étapes 4 et 5 pour aller plus loin. Les annexes se lisent au besoin.">
<g stroke-width="2" style="fill:var(--dg-box)">
<rect x="4" y="40" width="136" height="96" rx="14" style="stroke:var(--dg-you)"/>
<rect x="158" y="40" width="136" height="96" rx="14" style="stroke:var(--dg-you)"/>
<rect x="312" y="40" width="136" height="96" rx="14" style="stroke:var(--dg-you)"/>
<rect x="466" y="40" width="136" height="96" rx="14" style="stroke:var(--dg-claude)"/>
<rect x="620" y="40" width="136" height="96" rx="14" style="stroke:var(--dg-claude)"/>
</g>
<path d="M142,88h12m-6,-5l6,5l-6,5M296,88h12m-6,-5l6,5l-6,5M450,88h12m-6,-5l6,5l-6,5M604,88h12m-6,-5l6,5l-6,5" fill="none" stroke-width="2" style="stroke:var(--dg-line)"/>
<g class="mono" font-size="12" style="fill:var(--dg-muted)">
<text x="4" y="24">POUR TOUS</text><text x="466" y="24">POUR ALLER PLUS LOIN</text>
</g>
<path d="M4,30H446M466,30H756" fill="none" stroke-width="2" style="stroke:var(--dg-line)"/>
<circle cx="30" cy="66" r="12" style="fill:var(--dg-you)"/><text x="30" y="71" text-anchor="middle" font-size="13" font-weight="600" style="fill:var(--dg-on-you)">1</text>
<text x="18" y="104" font-size="15" font-weight="600" style="fill:var(--dg-ink)">Découvrir</text>
<text x="18" y="124" font-size="13" style="fill:var(--dg-muted)">1 page</text>
<circle cx="184" cy="66" r="12" style="fill:var(--dg-you)"/><text x="184" y="71" text-anchor="middle" font-size="13" font-weight="600" style="fill:var(--dg-on-you)">2</text>
<text x="172" y="104" font-size="15" font-weight="600" style="fill:var(--dg-ink)">Démarrer</text>
<text x="172" y="124" font-size="13" style="fill:var(--dg-muted)">2 pages</text>
<circle cx="338" cy="66" r="12" style="fill:var(--dg-you)"/><text x="338" y="71" text-anchor="middle" font-size="13" font-weight="600" style="fill:var(--dg-on-you)">3</text>
<text x="326" y="104" font-size="15" font-weight="600" style="fill:var(--dg-ink)">Au quotidien</text>
<text x="326" y="124" font-size="13" style="fill:var(--dg-muted)">5 pages</text>
<circle cx="492" cy="66" r="12" style="fill:var(--dg-claude)"/><text x="492" y="71" text-anchor="middle" font-size="13" font-weight="600" style="fill:var(--dg-box)">4</text>
<text x="480" y="104" font-size="15" font-weight="600" style="fill:var(--dg-ink)">Comprendre</text>
<text x="480" y="124" font-size="13" style="fill:var(--dg-muted)">5 pages</text>
<circle cx="646" cy="66" r="12" style="fill:var(--dg-claude)"/><text x="646" y="71" text-anchor="middle" font-size="13" font-weight="600" style="fill:var(--dg-box)">5</text>
<text x="634" y="104" font-size="15" font-weight="600" style="fill:var(--dg-ink)">Pour l'équipe</text>
<text x="634" y="124" font-size="13" style="fill:var(--dg-muted)">6 pages</text>
<rect x="4" y="150" width="752" height="34" rx="10" fill="none" stroke-width="1.5" stroke-dasharray="5 4" style="stroke:var(--dg-line)"/>
<text x="20" y="172" font-size="13" style="fill:var(--dg-muted)">Annexes : référence, glossaire, FAQ, un exemple de plan — à ouvrir au besoin.</text>
</svg>
</div>
<figcaption>Lisez les étapes dans l'ordre ; les étapes 1 à 3 suffisent au quotidien, les étapes 4 et 5 vont plus loin.</figcaption>
</figure>

Chaque terme est expliqué, en mots simples, dans le [Glossaire](glossary.md).

D'où vient Compagnon, et pourquoi : [Origines](origins.md).

## Étape suivante

[Installation](installation.md)
