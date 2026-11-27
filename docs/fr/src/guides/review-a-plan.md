# Relire un plan

Ce guide s'adresse à vous si vous n'écrivez pas de code mais devez relire
et valider ce que Claude prévoit de construire : responsable produit
(product owner), manager, expert métier. Aucun
[terminal](../glossary.md#terminal) n'est nécessaire : un
[plan](../glossary.md#plan) tient en quatre fichiers texte courts, que vous
pouvez lire sur GitHub, dans un éditeur,
ou en demandant à Claude de vous les montrer.

<figure class="diagram">
<div class="scroll">
<svg viewBox="0 0 760 200" role="img" aria-label="Dans un plan, quatre fichiers dans l'ordre de lecture : la proposition (proposal.md) dit pourquoi et ce qui change, pour tous ; les spécifications (dossier specs/) disent ce que fera le produit, avec ses scénarios, pour les testeurs ; la conception (design.md) dit comment, avec ses choix et ses risques, pour les développeurs ; les tâches (tasks.md) listent le travail, chacune avec son test, pour Claude. La plupart des lecteurs s'arrêtent après le premier.">
<defs><marker id="dg-review-a-plan-arrow" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" style="fill:var(--dg-line)"/></marker></defs>
<rect x="4" y="20" width="168" height="130" rx="12" style="fill:var(--dg-box);stroke:var(--dg-you)" stroke-width="3"/>
<rect x="4" y="20" width="168" height="6" rx="3" style="fill:var(--dg-you)"/>
<text x="18" y="56" font-size="15" font-weight="600" style="fill:var(--dg-ink)">La proposition</text>
<text x="18" y="80" font-size="13" style="fill:var(--dg-ink)">le pourquoi et le quoi</text>
<text x="18" y="106" class="mono" font-size="12" style="fill:var(--dg-muted)">(proposal.md)</text>
<text x="18" y="134" class="mono" font-size="11" style="fill:var(--dg-muted)">pour tous</text>
<line x1="174" y1="85" x2="190" y2="85" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-review-a-plan-arrow)"/>
<rect x="194" y="20" width="168" height="130" rx="12" style="fill:var(--dg-box);stroke:var(--dg-line)" stroke-width="2"/>
<rect x="194" y="20" width="168" height="6" rx="3" style="fill:var(--dg-line)"/>
<text x="208" y="56" font-size="15" font-weight="600" style="fill:var(--dg-ink)">Les spécifications</text>
<text x="208" y="80" font-size="13" style="fill:var(--dg-ink)">ce que fera le produit</text>
<text x="208" y="106" class="mono" font-size="12" style="fill:var(--dg-muted)">(dossier specs/)</text>
<text x="208" y="134" class="mono" font-size="11" style="fill:var(--dg-muted)">pour les testeurs</text>
<line x1="364" y1="85" x2="380" y2="85" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-review-a-plan-arrow)"/>
<rect x="384" y="20" width="168" height="130" rx="12" style="fill:var(--dg-box);stroke:var(--dg-line)" stroke-width="2"/>
<rect x="384" y="20" width="168" height="6" rx="3" style="fill:var(--dg-line)"/>
<text x="398" y="56" font-size="15" font-weight="600" style="fill:var(--dg-ink)">La conception</text>
<text x="398" y="80" font-size="13" style="fill:var(--dg-ink)">le comment, les risques</text>
<text x="398" y="106" class="mono" font-size="12" style="fill:var(--dg-muted)">(design.md)</text>
<text x="398" y="134" class="mono" font-size="11" style="fill:var(--dg-muted)">pour les développeurs</text>
<line x1="554" y1="85" x2="570" y2="85" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-review-a-plan-arrow)"/>
<rect x="574" y="20" width="182" height="130" rx="12" style="fill:var(--dg-box);stroke:var(--dg-claude)" stroke-width="2"/>
<rect x="574" y="20" width="182" height="6" rx="3" style="fill:var(--dg-claude)"/>
<text x="588" y="56" font-size="15" font-weight="600" style="fill:var(--dg-ink)">Les tâches</text>
<text x="588" y="80" font-size="13" style="fill:var(--dg-ink)">chacune avec son test</text>
<text x="588" y="106" class="mono" font-size="12" style="fill:var(--dg-muted)">(tasks.md)</text>
<text x="588" y="134" class="mono" font-size="11" style="fill:var(--dg-muted)">pour Claude</text>
<text x="380" y="186" text-anchor="middle" font-size="13" style="fill:var(--dg-muted)">Lisez-les de gauche à droite — la plupart s'arrêtent après le premier.</text>
</svg>
</div>
<figcaption>Dans un plan : quatre fichiers courts, du pourquoi aux tâches. Le premier est pour tous ; plus on va à droite, plus c'est technique.</figcaption>
</figure>

Votre rôle n'est pas de vérifier le code. Il est de vérifier que le plan
**résout le bon problème, de la bonne façon pour ceux qui l'utiliseront** :
la proposition et les spécifications sont l'endroit où vous le dites.
Claude et les [rôles](../glossary.md#rôle) de relecture s'occupent du côté
technique.

## Où se trouve le plan

Chaque modification a son propre dossier, `_compagnon/changes/<nom>/`, qui
contient les quatre fichiers ci-dessus. Vous pouvez aussi simplement
demander :

> Montre-moi le plan du mode sombre, en mots simples.

Avant que vous le lisiez, les relecteurs de Claude ont déjà mis le plan à
l'épreuve : leurs remarques, et la réponse donnée à chacune, sont dans
`review.md`, dans le même dossier. Le chapitre
[Compte rendu de relecture](review-record.md) explique comment le lire.

## La proposition : pourquoi

`proposal.md` dit pourquoi la modification est nécessaire, ce qui va
changer et ce qu'elle touche. Vérifiez que :

- le problème décrit est le vrai, tel que les utilisateurs ou les clients
  le vivent ;
- rien de ce que vous attendiez ne manque, et rien d'indésirable n'a été
  ajouté ;
- ce qui est explicitement laissé de côté est acceptable pour l'instant.

Dans la proposition, une section « ## Domaines » (ou « ## Domains »)
liste les [domaines](../glossary.md#domaine) sensibles que la modification
touche, comme la sécurité ou les données personnelles.

## Les spécifications : ce qui sera vrai ensuite

Les fichiers sous `specs/` listent des **exigences** — des règles que le
produit doit respecter — chacune avec des **scénarios** : une situation,
une action, le résultat attendu. Ils se lisent ainsi :

```text
Exigence : Le thème suit par défaut le réglage du système

Scénario : Aucun thème encore choisi
- ÉTANT DONNÉ un utilisateur qui n'a jamais changé de thème
- QUAND son ordinateur est en mode sombre
- ALORS l'application affiche le thème sombre
```

Les fichiers du plan peuvent montrer quelques mots-clés en anglais, quelle
que soit leur langue : SHALL ou MUST veut dire « doit » ; GIVEN / WHEN /
THEN veulent dire « étant donné », « quand », « alors ».

C'est la partie qui compte le plus pour vous. Vérifiez que :

- chaque scénario décrit ce que vous attendez, dans vos propres termes
  métier ;
- les cas inhabituels sont couverts : une liste vide, un utilisateur sans
  droits, une erreur ;
- aucun scénario ne décrit quelque chose que vous ne voulez pas.

Chaque scénario devient un [test](../glossary.md#test) : s'il est écrit ici, il sera vérifié.

### Lire une règle modifiée

Un plan ne réécrit pas toutes les règles du projet : il ne liste que celles
qui changent — son [delta](../glossary.md#delta) — rangées sous quatre
sortes de sections. Voici les titres que vous verrez :

- **`Exigences AJOUTÉES`** (`ADDED Requirements`) : des règles nouvelles,
  qui n'existaient pas avant.
- **`Exigences MODIFIÉES`** (`MODIFIED Requirements`) : des règles qui
  existent déjà et qui changent.
- **`Exigences SUPPRIMÉES`** (`REMOVED Requirements`) : des règles qui
  disparaissent, chacune avec sa **Raison** et sa **Migration** (ce qu'il
  faut faire à la place).
- **`Exigences RENOMMÉES`** (`RENAMED Requirements`) : des règles dont le
  titre change, avec une ligne DE (l'ancien titre) et une ligne VERS (le
  nouveau) ; leur texte reste le même.

Une règle modifiée montre son nouveau texte **en entier**, scénarios
compris, sous le même titre que la règle actuelle. Ce nouveau texte
remplace entièrement l'actuel : ce qui n'y figure plus disparaîtra. Pour
voir ce qui change, comparez-le à la règle actuelle, sous le même titre
dans les spécifications du projet, dans le dossier `_compagnon/specs/` :
sur GitHub, ouvrez ce dossier depuis la page du dépôt, puis le
fichier `spec.md` du dossier qui porte le même nom que celui du plan. Ou
demandez au développeur, ou à Claude :

> Dans le plan du mode sombre, montre-moi chaque règle modifiée avant et
> après.

Vérifiez que la différence est exactement ce que vous avez demandé, et que
rien de ce que vous voulez garder n'a disparu au passage.
[Un exemple de plan commenté](example-plan.md) montre une règle modifiée
et une règle ajoutée, avec ce qu'on vérifie dans chacune.

## La conception : comment

`design.md` explique les choix techniques. Vous n'avez pas à les juger,
mais lisez ses risques et les solutions écartées : si un risque vous inquiète, ou
si un compromis touche les utilisateurs (rapidité, coût, fonctionnalité
abandonnée), dites-le.

## Les tâches : les étapes

`tasks.md` liste les étapes du travail, chacune avec le test qui prouve
qu'elle est faite. Vérifiez seulement que l'ordre vous paraît logique, et
que rien de ce que verra un utilisateur n'est laissé sans test.

## Demander des corrections

Vous ne modifiez jamais les fichiers vous-même. Dites à Claude quoi
changer, avec vos mots : il met le plan à jour et garde ses parties
cohérentes.

<div class="example">
<p class="example-title">Exemple</p>

```text
> Dans le plan, un utilisateur qui choisit un thème à la main le garde,
  même si le réglage du système change
⏺ Update(_compagnon/changes/add-dark-mode/specs/ui/theme/spec.md)
  ⎿ Updated with 1 addition: Scénario : Un thème choisi à la main est gardé
⏺ Update(_compagnon/changes/add-dark-mode/tasks.md)
  ⎿ Updated with 1 addition: 2.3 Garder le thème choisi à la main
⏺ Les spécifications et les tâches le disent désormais. Lisez le nouveau
  scénario dans specs/.
```

Une phrase de votre part ; Claude a modifié chaque partie du plan concernée.

</div>

Relisez ensuite les parties modifiées. Recommencez jusqu'à ce que le plan
dise ce que vous voulez.

## Valider

Quand le plan vous convient, dites-le : « le plan me va, réalise la
modification ».
Claude construit alors exactement ce que dit le plan, et rien d'autre. Si
votre équipe relit le travail sur GitHub, le plan arrive sous
forme de [demande de fusion](../glossary.md#demande-de-fusion) (pull
request) : une demande pour l'ajouter au projet. Approuvez-la à cet
endroit : votre accord y est consigné.

## Étape suivante

[Approuver un plan sur GitHub](approve-a-plan.md) Votre équipe relit sur GitHub ? Approuvez-y le plan.
