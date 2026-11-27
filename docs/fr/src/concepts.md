# Concepts

Compagnon conserve trois types d'enregistrements dans le
[dépôt](glossary.md#dépôt) de votre projet, à côté du code :

- les **spécifications** disent ce que fait le système ;
- les **décisions** expliquent pourquoi il est construit ainsi ;
- les **modifications** sont le travail en cours. Chaque modification
  porte un [delta](glossary.md#delta) : les retouches qu'elle apportera
  aux spécifications.

<figure class="diagram">
<div class="scroll">
<svg viewBox="0 0 760 300" role="img" aria-label="Ce que garde le projet : les spécifications disent ce que fait le système ; les décisions disent pourquoi il est construit ainsi ; les modifications sont le travail en cours. Une modification contient un delta, avec des exigences ADDED (ajoutées), MODIFIED (modifiées) et REMOVED (supprimées), qui est fusionné dans les spécifications quand la modification est synchronisée ou archivée.">
<defs><marker id="dg-concepts-arrow" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" style="fill:var(--dg-line)"/></marker></defs>
<rect x="8" y="20" width="228" height="112" rx="12" style="fill:var(--dg-box);stroke:var(--dg-line)" stroke-width="2"/>
<circle cx="30" cy="44" r="5" fill="#00c39b"/>
<text x="44" y="49" class="mono" font-size="13" style="fill:var(--dg-muted)">_compagnon/specs/</text>
<text x="24" y="84" font-size="18" font-weight="600" style="fill:var(--dg-ink)">Spécifications</text>
<text x="24" y="110" font-size="14" style="fill:var(--dg-muted)">Ce que fait le système</text>
<rect x="266" y="20" width="228" height="112" rx="12" style="fill:var(--dg-box);stroke:var(--dg-line)" stroke-width="2"/>
<circle cx="288" cy="44" r="5" fill="#ffab03"/>
<text x="302" y="49" class="mono" font-size="13" style="fill:var(--dg-muted)">_compagnon/decisions/</text>
<text x="282" y="84" font-size="18" font-weight="600" style="fill:var(--dg-ink)">Décisions</text>
<text x="282" y="110" font-size="14" style="fill:var(--dg-muted)">Le pourquoi des choix</text>
<rect x="524" y="20" width="228" height="112" rx="12" style="fill:var(--dg-box);stroke:var(--dg-claude)" stroke-width="2"/>
<circle cx="546" cy="44" r="5" fill="#ff5331"/>
<text x="560" y="49" class="mono" font-size="13" style="fill:var(--dg-muted)">_compagnon/changes/</text>
<text x="540" y="84" font-size="18" font-weight="600" style="fill:var(--dg-ink)">Modifications</text>
<text x="540" y="110" font-size="14" style="fill:var(--dg-muted)">Le travail en cours</text>
<line x1="638" y1="134" x2="638" y2="158" style="stroke:var(--dg-line)" stroke-width="2" stroke-dasharray="4 4"/>
<text x="648" y="151" font-size="12" style="fill:var(--dg-muted)">contient</text>
<rect x="524" y="160" width="228" height="128" rx="12" style="fill:var(--dg-file);stroke:var(--dg-claude)" stroke-width="2"/>
<text x="540" y="186" font-size="15" font-weight="600" style="fill:var(--dg-ink)">Un delta</text>
<text x="540" y="214" class="mono" font-size="13" style="fill:var(--dg-ink)">## ADDED</text>
<text x="650" y="214" font-size="12" style="fill:var(--dg-muted)">règles ajoutées</text>
<text x="540" y="240" class="mono" font-size="13" style="fill:var(--dg-ink)">## MODIFIED</text>
<text x="650" y="240" font-size="12" style="fill:var(--dg-muted)">règles réécrites</text>
<text x="540" y="266" class="mono" font-size="13" style="fill:var(--dg-ink)">## REMOVED</text>
<text x="650" y="266" font-size="12" style="fill:var(--dg-muted)">règles retirées</text>
<path d="M520 224 H122 V138" fill="none" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-concepts-arrow)"/>
<text x="320" y="212" text-anchor="middle" font-size="13" style="fill:var(--dg-muted)">fusionné dans les spécifications</text>
</svg>
</div>
<figcaption>Ce que garde le projet : les spécifications disent quoi, les décisions disent pourquoi, et le delta d'une modification est fusionné dans les spécifications une fois le travail fait.</figcaption>
</figure>

Voici où se trouve chacun d'eux :

```text
_compagnon/
├── config.yaml     # configuration du projet
├── specs/          # contrats de comportement — CE QUE fait le système
├── decisions/      # décisions d'architecture — POURQUOI il est construit ainsi
├── changes/        # travail en cours, qui modifie les deux dossiers ci-dessus
│   ├── <name>/     #   proposal.md, design.md, tasks.md, specs/** (deltas)
│   └── archive/    #   changes terminés, datés
├── schemas/        # schémas de workflow personnalisés (facultatif)
└── compagnon.lock      # sources git héritées et épinglées (le cas échéant)
```

Vous pouvez lancer chaque commande Compagnon depuis n'importe quel dossier du
projet. Compagnon remonte de dossier en dossier jusqu'à trouver `_compagnon/`.

## Spécification

Les spécifications sont rangées par [capacité](glossary.md#capacité) : une
chose que fait le système et que l'on peut constater de l'extérieur, comme
la connexion des utilisateurs ou le réglage du thème. Chaque capacité porte
un nom fait de parties séparées par des barres obliques, comme `ui/theme`,
chaque partie en [kebab-case](glossary.md#kebab-case). Regroupez vos
capacités selon l'organisation de votre métier, et non selon celle de votre
code.

Chaque capacité a une spécification, dans
`_compagnon/specs/<capability>/spec.md`. C'est la référence de ce que fait
cette capacité aujourd'hui. Elle comporte deux sections : `## Purpose`, une
ou deux phrases sur sa raison d'être, et `## Requirements`, les règles
qu'elle suit.

```markdown
# Theme Specification

## Purpose

Permettre aux utilisateurs de choisir le thème de couleurs de l'interface.

## Requirements

### Requirement: Le thème suit par défaut la préférence du système

L'application SHALL s'afficher avec le jeu de couleurs du système d'exploitation tant que l'utilisateur n'a pas choisi de thème.

#### Scenario: Aucun thème encore choisi

- **GIVEN** un utilisateur qui n'a jamais modifié le réglage du thème
- **WHEN** son système d'exploitation est en mode sombre
- **THEN** l'application s'affiche avec le thème sombre
```

Chaque exigence est une règle ferme : elle emploie `SHALL` ou `MUST` (`DOIT`
ou `DOIVENT` dans une spécification rédigée en français). Chacune comporte
au moins un scénario : un court exemple de ce que l'on peut voir se
produire.

Une spécification décrit ce que fait le système, jamais la façon dont le code
le fait. Un test simple : si le code peut évoluer sans que le comportement
évolue, la spécification ne doit pas évoluer non plus.

Vous ne modifiez jamais directement les spécifications du projet. Elles
n'évoluent que lorsqu'une modification est synchronisée ou archivée. Ainsi,
chaque retouche d'une spécification arrive avec la modification qui
l'explique.

## Modification

Une modification est un travail sur le système, rangé dans son propre
dossier `_compagnon/changes/<name>/`. Elle contient le plan : une
proposition (pourquoi et quoi), ses deltas (les retouches des
spécifications), une conception (comment) et une liste de tâches, plus un
fichier `change.yaml` qui contient ses informations. Ces parties sont les
[artefacts](glossary.md#artefact) de la modification ;
[Comment ça marche](workflow.md) dit qui rédige chacun et à quel moment. Le
[schéma](glossary.md#schéma), `spec-driven` par défaut, fixe les parties
que contient un plan et le [déroulé](glossary.md#déroulé) que suit une
modification ; un projet peut définir le sien, voir
[Schémas personnalisés](guides/custom-schemas.md).

Une modification est **active** tant que son dossier se trouve dans
`_compagnon/changes/<name>/`. L'archivage la déplace dans
`_compagnon/changes/archive/<YYYY-MM-DD>-<name>/`, où elle reste comme la
trace du pourquoi et du comment de l'évolution des spécifications.

## Delta

Un delta est la partie d'une modification qui retouche une spécification. Il
se trouve dans `_compagnon/changes/<name>/specs/<capability>/spec.md`, sous
le même nom de capacité que la spécification qu'il retouche. Il contient au
plus quatre sortes de retouches, chacune dans sa propre section :

| Section | Ce qu'elle fait lors de la fusion dans la spécification du projet |
|---|---|
| `## ADDED Requirements` | Ajoute les exigences à la fin de la spécification |
| `## MODIFIED Requirements` | Remplace sur place l'exigence de même nom |
| `## REMOVED Requirements` | Supprime l'exigence, en donnant une **Reason** (raison) et une **Migration** (que faire à la place) |
| `## RENAMED Requirements` | Change le titre d'une exigence sans toucher à son texte |

Voici un delta qui modifie une exigence et en renomme une autre :

```markdown
## MODIFIED Requirements

### Requirement: Le thème suit par défaut la préférence du système

L'application SHALL s'afficher avec le jeu de couleurs du système d'exploitation tant que l'utilisateur n'a pas choisi de thème, et SHALL basculer en direct lorsque cette préférence change.

#### Scenario: Aucun thème encore choisi

- **GIVEN** un utilisateur qui n'a jamais modifié le réglage du thème
- **WHEN** son système d'exploitation est en mode sombre
- **THEN** l'application s'affiche avec le thème sombre

## RENAMED Requirements

- FROM: Le thème choisi est mémorisé
- TO: Le thème choisi suit l'utilisateur
```

Attention avec `MODIFIED` : ce bloc remplace l'exigence tout entière. Copiez
d'abord l'exigence complète depuis la spécification du projet, scénarios
compris, puis modifiez la copie. Tout ce que vous omettez est perdu lors de
la fusion. Si vous ajoutez seulement un comportement, utilisez `ADDED`.

La fusion ne touche que les exigences nommées dans le delta. Tout le reste de
la spécification du projet reste tel quel.

Le delta d'une **nouvelle** capacité commence par sa propre section
`## Purpose` ; sa fusion crée la spécification de la capacité. Pour
supprimer entièrement une capacité, rédigez un delta `REMOVED` qui vide sa
spécification et ajoutez `retire_capabilities: true` dans `change.yaml` :
seul [git](glossary.md#git), l'historique des versions du projet, peut
annuler cette suppression.

La syntaxe exacte figure dans [Formats de fichiers](reference/file-formats.md).

## Décision

Une décision, ou ADR (*architecture decision record*), consigne un choix
sur la façon dont le système est construit : ce qui y a mené, le choix
lui-même, ses conséquences et les autres options envisagées. Chacune est un
fichier rangé dans `_compagnon/decisions/`, nommé d'un numéro et d'un titre
court (`NNNN-<nom-court>.md`).

Claude reçoit les décisions en vigueur chaque fois qu'il rédige un
`design.md` : les conceptions les respectent au lieu de les remettre en
débat. Une décision acceptée est [scellée](glossary.md#sceau) : pour la
changer, on la remplace par une nouvelle. Certaines décisions viennent d'un
autre dépôt, par une [source héritée](guides/inherited-sources.md).

Comment une décision est proposée, acceptée, remplacée, contournée ou
promue depuis une conception : voir [Décisions](guides/decisions.md).

## Pour aller plus loin

- [Comment ça marche](workflow.md) : les étapes d'une modification et qui
  rédige chaque partie de son plan.
- [Décisions](guides/decisions.md) : la vie d'une décision.
- [Formats de fichiers](reference/file-formats.md) : la syntaxe exacte des
  spécifications, des deltas et des décisions.
- [Sources héritées](guides/inherited-sources.md) : des règles et des
  décisions partagées entre dépôts.
- [Glossaire](glossary.md) : chaque terme en quelques mots.
