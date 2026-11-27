# Décisions

Une décision d'architecture consigne un choix que tout le projet suit — une
bibliothèque, une façon d'organiser le code — et la raison de ce choix. Ce
chapitre explique comment une décision est proposée, acceptée, remplacée et,
si besoin, contournée.

Une [décision](../glossary.md#décision), ou ADR (*architecture decision
record*), est un fichier rangé dans `_compagnon/decisions/`, nommé d'un
numéro et d'un titre court (`NNNN-<nom-court>.md`). Elle contient le
contexte qui a mené au choix, le choix lui-même, ses conséquences et les
autres options envisagées. Claude reçoit les décisions **en vigueur**
chaque fois qu'il rédige un `design.md` : les conceptions les respectent au
lieu de les remettre en débat.

<figure class="diagram">
<div class="scroll">
<svg viewBox="0 0 760 280" role="img" aria-label="La vie d'une décision : elle est proposée, et vous pouvez la reprendre librement ; l'accepter la scelle et la met en vigueur ; quand une remplaçante est acceptée, elle est remplacée et gardée pour l'histoire. Branches latérales : un choix rédigé dans une conception est promu en décision proposée ; une fois un écart local accepté, une décision héritée est contournée et ne s'applique plus dans ce projet.">
<defs><marker id="dg-decisions-arrow" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" style="fill:var(--dg-line)"/></marker></defs>
<rect x="8" y="40" width="200" height="80" rx="14" style="fill:var(--dg-box);stroke:var(--dg-claude)" stroke-width="2"/>
<text x="108" y="74" text-anchor="middle" font-size="17" font-weight="600" style="fill:var(--dg-ink)">Proposée</text>
<text x="108" y="98" text-anchor="middle" font-size="13" style="fill:var(--dg-muted)">à reprendre librement</text>
<line x1="210" y1="80" x2="276" y2="80" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-decisions-arrow)"/>
<text x="243" y="68" text-anchor="middle" class="mono" font-size="12" style="fill:var(--dg-muted)">accept</text>
<rect x="280" y="40" width="200" height="80" rx="14" style="fill:var(--dg-box);stroke:var(--dg-you)" stroke-width="2"/>
<circle cx="302" cy="58" r="5" style="fill:var(--dg-you)"/>
<text x="380" y="74" text-anchor="middle" font-size="17" font-weight="600" style="fill:var(--dg-ink)">Acceptée</text>
<text x="380" y="98" text-anchor="middle" font-size="13" style="fill:var(--dg-muted)">scellée, en vigueur</text>
<line x1="482" y1="80" x2="548" y2="80" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-decisions-arrow)"/>
<text x="515" y="68" text-anchor="middle" font-size="12" style="fill:var(--dg-muted)">remplacée</text>
<rect x="552" y="40" width="200" height="80" rx="14" style="fill:var(--dg-box);stroke:var(--dg-line)" stroke-width="2"/>
<text x="652" y="74" text-anchor="middle" font-size="17" font-weight="600" style="fill:var(--dg-ink)">Remplacée</text>
<text x="652" y="98" text-anchor="middle" font-size="13" style="fill:var(--dg-muted)">gardée pour l'histoire</text>
<rect x="8" y="190" width="200" height="72" rx="12" style="fill:var(--dg-file);stroke:var(--dg-line)" stroke-width="2"/>
<text x="108" y="220" text-anchor="middle" font-size="15" font-weight="600" style="fill:var(--dg-ink)">Promue</text>
<text x="108" y="244" text-anchor="middle" font-size="12" style="fill:var(--dg-muted)">depuis une conception</text>
<line x1="108" y1="188" x2="108" y2="124" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-decisions-arrow)"/>
<text x="118" y="160" class="mono" font-size="12" style="fill:var(--dg-muted)">promote</text>
<path d="M380 122 V226 H548" fill="none" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-decisions-arrow)"/>
<text x="392" y="176" font-size="12" style="fill:var(--dg-muted)">votre écart accepté</text>
<rect x="552" y="190" width="200" height="72" rx="12" style="fill:var(--dg-file);stroke:var(--dg-line)" stroke-width="2"/>
<text x="652" y="220" text-anchor="middle" font-size="15" font-weight="600" style="fill:var(--dg-ink)">Contournée</text>
<text x="652" y="244" text-anchor="middle" font-size="12" style="fill:var(--dg-muted)">héritée, écartée ici</text>
</svg>
</div>
<figcaption>La vie d'une décision : rédigée tant qu'elle est proposée, scellée une fois acceptée, remplacée plutôt que modifiée ; une conception peut en promouvoir une, et un écart local met de côté une décision héritée.</figcaption>
</figure>

Une décision a l'un des cinq statuts suivants : `proposed` (proposée),
`accepted` (acceptée), `superseded` (remplacée), `deprecated` (obsolète) ou
`rejected` (rejetée). Chacune possède un identifiant court, comme `0007`, et
un identifiant complet qui dit aussi d'où elle vient : de votre projet, ou
d'un autre dépôt dont il hérite.

<details>
<summary>Sous le capot : l'identifiant complet d'une décision</summary>

L'identifiant qualifié indique la provenance de la décision : `project/0007`
pour une décision du projet lui-même, `path:~/shared/0100` ou
`git:git@github.com:acme/shared.git/0100` pour une décision héritée d'un
autre dépôt.

</details>

## Proposée

Toute nouvelle décision commence `proposed`, quelle que soit sa façon de
naître : à partir de rien, pour en remplacer une autre, comme écart ou
promue depuis une conception. Tant qu'elle est `proposed`, vous pouvez la
rédiger et la reprendre librement. Elle ne remplace encore rien et ne met
encore rien de côté.

<details>
<summary>Sous le capot : créer une décision</summary>

```bash
compagnon decision new "Use PostgreSQL for persistence"
```

`compagnon decision supersede`, `deviate` et `promote`, plus bas, créent
aussi une décision `proposed`. `compagnon decision list` affiche chaque
décision et son statut.

</details>

## Acceptée et scellée

Une fois son texte définitif, vous acceptez la décision. L'acceptation est
la seule étape qui fait entrer une décision en vigueur. Une décision est
**en vigueur** lorsqu'elle est `accepted` et qu'aucune autre ne la
remplace.

L'acceptation la scelle aussi. Le [sceau](../glossary.md#sceau) est une
empreinte du texte de la décision : si quelqu'un modifie le texte ensuite,
`compagnon validate` voit que l'empreinte ne correspond plus et le signale.
Il avertit aussi lorsqu'une décision acceptée n'a pas de sceau. Une décision
`proposed` n'est pas scellée.

Le sceau ne couvre pas le [frontmatter](../glossary.md#frontmatter) de la
décision, le court en-tête en haut du fichier qui contient son statut. Le
statut peut donc passer de `accepted` à `superseded` sans briser le sceau.

> **Remarque**
> Pour changer ce que dit une décision acceptée,
> [remplacez-la](#remplacée) plutôt que de la modifier. Si vous avez modifié
> volontairement une décision scellée, scellez-la à nouveau ;
> [Codes d'erreur et solutions](../reference/errors.md#decision_seal_mismatch)
> explique comment.

<details>
<summary>Sous le capot : accepter, et comment le sceau est enregistré</summary>

```bash
compagnon decision accept 0001
compagnon decision seal 0001 --force   # enregistre le nouveau texte d'une décision modifiée
```

Lorsqu'une décision devient acceptée par une commande `compagnon decision` —
`accept`, ou `new --status accepted` — Compagnon enregistre une empreinte
SHA-256 du contenu de la décision dans `_compagnon/decisions/seal.yaml`.
`compagnon decision seal` fait de même pour une décision que vous avez
acceptée à la main. `compagnon validate` compare le contenu de chaque
décision locale `accepted` ou `superseded` à son empreinte, et signale une
modification par l'erreur `decision_seal_mismatch` et un sceau manquant par
l'avertissement `decision_unsealed`. Une décision créée avec
`new --status accepted` est scellée immédiatement, avec le texte d'exemple
du modèle : préférez `new`, puis `accept`.

</details>

## Remplacée

Pour changer une décision, ne la modifiez pas : remplacez-la par une
nouvelle. La commande s'appelle `supersede` (remplacer).

La nouvelle décision commence `proposed`, et nomme celle qu'elle remplace.
L'ancienne reste `accepted`, et en vigueur, pendant que vous rédigez la
nouvelle. Quand vous acceptez la nouvelle décision, l'ancienne est marquée
`superseded` dans la même étape. Son texte reste strictement identique : son
sceau reste donc valide, et l'historique du raisonnement est préservé.

Seule une décision `accepted` peut être remplacée. Si une autre décision l'a
remplacée entre-temps, l'acceptation de la vôtre est refusée et rien n'est
écrit : voir
[`predecessor_not_accepted`](../reference/errors.md#predecessor_not_accepted).

<details>
<summary>Sous le capot : remplacer une décision</summary>

```bash
compagnon decision supersede 0001 "Use SQLite for persistence"
compagnon decision accept 0002
```

La première commande crée `0002` avec `supersedes: ["0001"]` dans son
frontmatter, sans toucher à `0001`. La seconde accepte `0002` et marque
`0001` `superseded`.

</details>

## Contournée par un écart

Certaines décisions viennent d'un autre dépôt : elles sont
[héritées](inherited-sources.md). Vous ne pouvez pas en remplacer une
depuis votre projet, puisque vous ne pouvez pas écrire dans sa source.
Consignez plutôt une exception locale, appelée écart.

L'écart est une décision `proposed` de votre projet qui pointe vers la
décision héritée. Il entre en vigueur lorsque vous l'acceptez. Dès lors, la
décision héritée apparaît toujours dans la liste des décisions, mais elle
ne s'applique plus à votre projet, et Claude ne la reçoit plus lorsqu'il
rédige des conceptions. Tant que vous n'avez pas accepté l'écart, la
décision héritée reste en vigueur.

<details>
<summary>Sous le capot : s'écarter d'une décision héritée</summary>

```bash
compagnon decision deviate path:~/shared/0100 "Services log in logfmt"
compagnon decision accept 0003
```

Le champ `deviates_from` de la nouvelle décision contient l'identifiant
complet de la décision héritée.

</details>

## Promue depuis une conception

Pendant qu'on prépare une modification, sa conception fait souvent un choix
qui mérite de lui survivre. Rédigez ce choix dans `design.md` sous la forme
d'un bloc `### Decision: <title>`, puis promouvez-le en décision à part
entière.

Compagnon crée une décision `proposed` dont la section `## Decision`
reprend le texte du bloc, mot pour mot. Dans `design.md`, le texte du bloc
est remplacé par une référence à la nouvelle décision. Relisez-la et
répartissez le texte promu dans ses sections (contexte, décision,
conséquences). Acceptez-la ensuite, ce qui la scelle — idéalement avant
d'archiver la modification.

<details>
<summary>Sous le capot : promouvoir une décision de conception</summary>

```bash
compagnon decision promote add-audit-log "Append-only audit table"
```

Le premier argument est le nom de la modification, le second le titre du
bloc `### Decision:` de son `design.md`.

</details>

## Pour aller plus loin

- [Concepts](../concepts.md) : spécifications, décisions et modifications,
  et comment elles s'articulent.
- [Sources héritées](inherited-sources.md) : des décisions partagées depuis
  un autre dépôt.
- [Interface en ligne de commande](../reference/cli.md) : chaque commande
  `compagnon decision` et ses options.
- [Codes d'erreur et solutions](../reference/errors.md) : que faire quand
  une décision est signalée.
