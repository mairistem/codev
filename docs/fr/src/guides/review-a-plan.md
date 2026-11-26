# Relire un plan

Ce guide s'adresse à vous si vous n'écrivez pas de code mais devez relire
et valider ce que Claude prévoit de construire : responsable produit
(product owner), manager, expert métier. Aucun
[terminal](../glossary.md#terminal) n'est nécessaire : un plan tient en quatre
fichiers texte courts, que vous pouvez lire sur votre hébergeur de code
(GitHub, GitLab), dans un éditeur, ou en demandant à Claude de vous les
montrer.

Votre rôle n'est pas de vérifier le code. Il est de vérifier que le plan
**résout le bon problème, de la bonne façon pour ceux qui l'utiliseront**.
Claude et les [rôles](../glossary.md#rôle) de relecture s'occupent du côté technique.

## Où se trouve le plan

Chaque modification a son propre dossier, `_compagnon/changes/<nom>/`. Vous
pouvez aussi simplement demander :

> Montre-moi le plan du mode sombre, en mots simples.

## La proposition : pourquoi

`proposal.md` dit pourquoi la modification est nécessaire, ce qui va
changer et ce qu'elle touche. Vérifiez que :

- le problème décrit est le vrai, tel que les utilisateurs ou les clients
  le vivent ;
- rien de ce que vous attendiez ne manque, et rien d'indésirable n'a été
  ajouté ;
- ce qui est explicitement laissé de côté est acceptable pour l'instant.

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
THEN veulent dire « étant donné », « quand », « alors ». Dans la
proposition, une section « ## Domaines » (ou « ## Domains ») liste les
[domaines](../glossary.md#domaine) sensibles que la modification touche,
comme la sécurité ou les données personnelles.

C'est la partie qui compte le plus pour vous. Vérifiez que :

- chaque scénario décrit ce que vous attendez, dans vos propres termes
  métier ;
- les cas inhabituels sont couverts : une liste vide, un utilisateur sans
  droits, une erreur ;
- aucun scénario ne décrit quelque chose que vous ne voulez pas.

Chaque scénario devient un [test](../glossary.md#test) : s'il est écrit ici, il sera vérifié.

### Lire une règle modifiée

Un plan ne réécrit pas toutes les règles du projet : il ne liste que celles
qui changent, rangées sous quatre sortes de sections. Voici les titres que
vous verrez :

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
sur GitHub ou GitLab, ouvrez ce dossier depuis la page du dépôt, puis le
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

> Dans le plan, un utilisateur qui choisit un thème à la main le garde,
> même si le réglage du système change.

> Retire l'export PDF de cette modification : on le fera plus tard.

Relisez ensuite les parties modifiées. Recommencez jusqu'à ce que le plan
dise ce que vous voulez.

## Valider

Quand le plan vous convient, dites-le : « le plan me va, réalise la
modification ».
Claude construit alors exactement ce que dit le plan, et rien d'autre. Si
votre équipe relit le travail sur GitHub ou GitLab, le plan arrive sous
forme de [demande de fusion](../glossary.md#demande-de-fusion) (pull
request) : une demande pour l'ajouter au projet. Approuvez-la à cet
endroit : votre accord y est consigné.
