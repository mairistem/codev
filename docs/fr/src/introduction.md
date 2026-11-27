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

La documentation se lit comme un parcours en cinq étapes : découvrir,
démarrer, au quotidien, comprendre, pour l'équipe. Commencez par
[Installation](installation.md), puis suivez « Étape suivante » à la fin
de chaque page.

Chaque terme est expliqué, en mots simples, dans le [Glossaire](glossary.md).

D'où vient Compagnon, et pourquoi : [Origines](origins.md).
