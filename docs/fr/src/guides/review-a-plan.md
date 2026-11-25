# Relire un plan

Ce guide s'adresse à vous si vous n'écrivez pas de code mais devez relire
et valider ce que Claude prévoit de construire : product owner, manager,
expert métier. Aucun terminal n'est nécessaire : un plan tient en quatre
fichiers texte courts, que vous pouvez lire sur votre hébergeur de code
(GitHub, GitLab), dans un éditeur, ou en demandant à Claude de vous les
montrer.

Votre rôle n'est pas de vérifier le code. Il est de vérifier que le plan
**résout le bon problème, de la bonne façon pour ceux qui l'utiliseront**.
Claude et les rôles de relecture s'occupent du côté technique.

## Où se trouve le plan

Chaque change a son propre dossier, `_compagnon/changes/<nom>/`. Vous
pouvez aussi simplement demander :

> Montre-moi le plan du mode sombre, en mots simples.

## La proposition : pourquoi

`proposal.md` dit pourquoi le change est nécessaire, ce qui va changer et
ce qu'il touche. Vérifiez que :

- le problème décrit est le vrai, tel que les utilisateurs ou les clients
  le vivent ;
- rien de ce que vous attendiez ne manque, et rien d'indésirable n'a été
  ajouté ;
- ce qui est explicitement laissé de côté est acceptable pour l'instant.

## Les specs : ce qui sera vrai ensuite

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

C'est la partie qui compte le plus pour vous. Vérifiez que :

- chaque scénario décrit ce que vous attendez, dans vos propres termes
  métier ;
- les cas inhabituels sont couverts : une liste vide, un utilisateur sans
  droits, une erreur ;
- aucun scénario ne décrit quelque chose que vous ne voulez pas.

Chaque scénario devient un test : s'il est écrit ici, il sera vérifié.

## La conception : comment

`design.md` explique les choix techniques. Vous n'avez pas à les juger,
mais lisez ses risques et ses alternatives : si un risque vous inquiète, ou
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

> Retire l'export PDF de ce change : on le fera plus tard.

Relisez ensuite les parties modifiées. Recommencez jusqu'à ce que le plan
dise ce que vous voulez.

## Valider

Quand le plan vous convient, dites-le : « le plan me va, implémente-le ».
Claude construit alors exactement ce que dit le plan, et rien d'autre. Si
votre équipe utilise des pull requests, approuvez celle qui contient le
plan : votre accord y est consigné.
