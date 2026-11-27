# Réaliser et ranger

Une fois le plan approuvé, Claude le réalise tâche par tâche, vérifie
chacune, relit ce qu'il a écrit, puis range le plan pour que les
spécifications du projet disent ce qui est désormais vrai.

<figure class="diagram">
<div class="scroll">
<svg viewBox="0 0 760 140" role="img" aria-label="Réaliser et ranger en quatre étapes : 1, Claude réalise tâche par tâche, chacune testée ; 2, Claude relit son code d'un œil neuf ; 3, vous vérifiez le résumé et le code ; 4, le plan est rangé et rejoint les spécifications du projet.">
<g stroke-width="2" style="fill:var(--dg-box)">
<rect x="4" y="16" width="168" height="118" rx="14" style="stroke:var(--dg-claude)"/>
<rect x="198" y="16" width="168" height="118" rx="14" style="stroke:var(--dg-claude)"/>
<rect x="392" y="16" width="168" height="118" rx="14" style="stroke:var(--dg-you)"/>
<rect x="586" y="16" width="168" height="118" rx="14" style="stroke:var(--dg-line)" stroke-dasharray="5 4"/>
</g>
<path d="M175,75h18m-6,-5l6,5l-6,5M369,75h18m-6,-5l6,5l-6,5M563,75h18m-6,-5l6,5l-6,5" fill="none" stroke-width="2" style="stroke:var(--dg-line)"/>
<g class="mono" font-size="12" style="fill:var(--dg-muted)">
<text x="20" y="44">1 · CLAUDE</text><text x="214" y="44">2 · CLAUDE</text><text x="408" y="44">3 · VOUS</text><text x="602" y="44">4 · RANGÉ</text>
</g>
<g font-size="16" font-weight="600" style="fill:var(--dg-ink)">
<text x="20" y="78">Réalise</text><text x="214" y="78">Relit</text><text x="408" y="78">Vérifiez</text><text x="602" y="78">Rangé</text>
</g>
<g font-size="13" style="fill:var(--dg-muted)">
<text x="20" y="102">tâche par tâche,</text><text x="20" y="120">chacune testée</text><text x="214" y="102">son code</text><text x="214" y="120">d'un œil neuf</text><text x="408" y="102">le résumé</text><text x="408" y="120">et le code</text><text x="602" y="102">le plan rejoint</text><text x="602" y="120">les spécifications</text>
</g>
</svg>
</div>
<figcaption>Claude réalise et relit ; vous vérifiez ; ranger garde ce qui est désormais vrai.</figcaption>
</figure>

## Réaliser

Quand le plan vous convient, dites-le :

> Le plan me va, réalise la modification.

Claude prend les tâches une à une, dans l'ordre. Après chacune, il lance
le [test](../glossary.md#test) que la tâche indique, et ne coche la tâche
que si le test réussit. Si quelque chose n'est pas clair, il s'arrête et
vous demande plutôt que de deviner. Une fois toutes les tâches faites, il
relit son propre code d'un œil neuf, corrige ce qui ne fait pas de doute,
et vous donne un résumé court.

<div class="example">
<p class="example-title">Exemple</p>

```text
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

Deux phrases : Claude réalise et relit, puis range le plan.

</div>

<details>
<summary>Sous le capot : comment Claude réalise, étape par étape</summary>

La commande raccourcie est `/compagnon-apply <name>`.

`/compagnon-apply <name>` construit ce que décrit le plan :

1. La compétence vérifie que le plan est complet.
2. Elle prend dans l'ordre chaque tâche non cochée de `tasks.md`
   (`- [ ]`), la réalise, lance la vérification que la tâche indique, et ne
   coche la case qu'une fois cette vérification réussie.
3. Elle écrit les tests à la manière de vos tests existants, puis selon
   les habitudes de votre langage et vos
   [`rules`](../reference/configuration.md#rules), et vous demande comment les
   lancer quand votre projet ne le dit pas.
4. Une fois toutes les tâches cochées, Claude relit le code qu'il a
   touché, comme le ferait un nouvel arrivant sur le projet, rejoint par
   les [experts](../roles.md#experts-et-domaines) des domaines que coche la
   proposition. Il simplifie ce qui ne fait pas de doute, sans élargir le
   périmètre, et liste au plus cinq notes de relecture. Un constat grave de
   sécurité ou de données personnelles est toujours signalé.
5. Enfin, elle lance une fois tous les tests du projet.

Chaque constat va dans le compte rendu de relecture, avec la réponse de
l'auteur et la vérification du relecteur. Le résumé se termine par ce que
vous seul pouvez trancher : par exemple, le code touche désormais des
données personnelles que le plan ne listait pas.

Claude s'arrête au premier blocage ou au premier doute pour vous
consulter, plutôt que de deviner. Il ne travaille que sur la modification
désignée, et ne synchronise ni n'archive jamais : c'est à vous de demander
ces étapes. La réalisation est la seule compétence autorisée à lancer
n'importe quelle commande dans le terminal, parce qu'elle doit lancer vos
tests.

<details>
<summary>Sous le capot : les corrections des experts et le mode isolé</summary>

Claude trouve comment lancer vos tests dans la configuration du projet,
son manifeste ou son [intégration continue](../glossary.md#intégration-continue).
Un expert corrige un défaut manifeste dans le périmètre de la modification,
avec un test. Tout autre changement de comportement, il le signale au lieu
de le faire. Les critères et les connaissances de ces deux étapes viennent
de `compagnon roles` ; lorsqu'aucun rôle n'est actif pour la relecture,
celle-ci est sautée et le résumé le signale. En mode isolé, chaque
relecteur travaille d'abord seul comme [sous-agent](../glossary.md#sous-agent)
(une copie distincte de Claude qui ne voit pas votre conversation), et la
compétence fusionne leurs constats.

</details>

</details>

## Vérifier le résultat

Lisez le résumé : les tâches faites, les notes de relecture, et ce que vous
seul pouvez trancher. Essayez le résultat, ou demandez à Claude comment le
faire. Si quelque chose ne va pas, dites-le avec vos mots ; Claude le
corrige avant que vous ne rangiez quoi que ce soit.

## Archiver

Quand le résultat vous convient :

> La modification est terminée, archive-la.

Claude recopie ce que le plan a changé dans les spécifications du projet
et déplace le plan dans les archives : le projet se souvient désormais
pourquoi la modification existe. Enregistrez ensuite le résultat dans
l'historique du projet ([enregistrement](../glossary.md#enregistrement)),
ou demandez-le à Claude.

<details>
<summary>Sous le capot : ce que fait l'archivage</summary>

La commande raccourcie est `/compagnon-archive <name>`.

```bash
compagnon archive --change <name>
```

`/compagnon-archive` clôt une modification terminée, en quatre temps :

1. Elle valide la modification, et **refuse** de continuer sur une erreur.
2. Elle vérifie le [compte rendu de relecture](review-record.md),
   et **refuse** de continuer si une relecture était attendue mais n'a pas
   été consignée, ou si un constat grave n'est pas encore réglé
   (`review_blocked`). `compagnon review --change <name>` montre ce qui
   reste. Claude ne consigne jamais une décision à votre place.
3. Elle recopie les deltas dans les spécifications du projet, comme la
   synchronisation.
4. Elle déplace le dossier de la modification dans
   `_compagnon/changes/archive/<YYYY-MM-DD>-<name>/`.

Rien ne change sur le disque tant que toute la mise à jour n'a pas été
vérifiée. Ensuite, les spécifications du projet décrivent ce que vous
avez livré, et le dossier archivé garde le pourquoi et le comment.
Enregistrez les deux dans [Git](../glossary.md#git) : c'est un
[enregistrement](../glossary.md#enregistrement) (commit). Si la modification
a une note de version, `compagnon changelog --write` l'ajoute à
`CHANGELOG.md` (voir [Notes de version et CHANGELOG](release-notes.md)).

<details>
<summary>Sous le capot : quand l'archivage refuse à cause du compte rendu</summary>

L'archivage signale `review_blocked` quand le compte rendu manque pour une
étape où des rôles de relecture étaient actifs, ou quand un constat de
gravité `high` est encore `open` ou `disputed`. La synchronisation, comme
l'archivage, valide d'abord : sur une erreur, elle n'écrit rien et renvoie
vers `compagnon validate <name>`.

</details>

Ce que vérifie `compagnon validate`, et comment `--change` est choisi
quand plusieurs modifications sont en cours, se trouvent dans la
référence de l'[Interface en ligne de commande](../reference/cli.md#compagnon-validate)
et ses [conventions](../reference/cli.md#conventions).

</details>

<details>
<summary>Sous le capot : synchroniser sans archiver</summary>

```bash
compagnon sync --change <name>
```

`/compagnon-sync` recopie les deltas de la modification dans les
spécifications du projet, sous `_compagnon/specs/`, **sans** déplacer la
modification. Elle sert quand une autre modification en cours a besoin de
voir une nouvelle capacité, ou pour vérifier le résultat avant
l'archivage. Elle vous dit quelles spécifications ont été créées, mises à
jour ou laissées inchangées. En général, vous sautez cette étape :
l'archivage synchronise pour vous.

</details>

## Étape suivante

[Comment ça marche](../workflow.md) Ce qui se passe derrière chaque étape.
