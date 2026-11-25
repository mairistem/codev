# Glossaire

Chaque terme employé par la documentation, en mots simples, par ordre
alphabétique.

## Agent

Un programme qui accomplit une tâche de lui-même, pas à pas, en décidant de
la suite : lire des fichiers, lancer des commandes, écrire du code. Claude,
quand il travaille dans Claude Code, est un agent. Plusieurs agents peuvent
travailler côte à côte, chacun sur sa part du travail.

## Archive

La dernière étape d'un change. Une fois le travail fini et vérifié,
l'archivage fusionne les specs du change dans celles du projet et range le
dossier du change, daté, dans `_compagnon/changes/archive/`. Rien n'est
perdu : le plan reste lisible, comme trace.

## Capacité

Un domaine de ce que fait le produit, comme « connexion » ou « factures ».
Les specs du projet sont rangées par capacité, chacune dans son dossier sous
`_compagnon/specs/`.

## Change

Un travail, de la demande jusqu'à son archivage : une fonctionnalité, une
correction, une refonte. Chaque change a son propre dossier sous
`_compagnon/changes/`, qui contient son plan.

## Claude Code

L'assistant de programmation d'Anthropic. Il fonctionne dans votre
terminal, votre éditeur de code ou les applications Claude, lit votre dépôt
et agit sur lui à votre demande. Compagnon ajoute ses skills à Claude Code.

## Conception

La partie d'un plan qui dit comment le change sera construit : les choix
techniques, les alternatives envisagées, les risques. Rédigée dans
`design.md`.

## Décision

Un choix fait une fois par l'équipe, pour tous les changes, comme « les
dates sont stockées en UTC ». Les décisions sont rangées dans
`_compagnon/decisions/` ; Claude les suit au lieu de les remettre en cause.

## Delta

La partie des specs d'un change qui dit ce qui change dans les specs du
projet : exigences ajoutées, modifiées ou retirées. À l'archivage, le delta
est fusionné dans les specs du projet.

## Exigence

Une règle que le produit doit respecter, écrite pour pouvoir être vérifiée :
« l'application DOIT suivre le jeu de couleurs du système ». Chaque
exigence est accompagnée de scénarios.

## MCP

Model Context Protocol : une façon standard de relier Claude à d'autres
outils, comme Jira. Avec une connexion Jira, Claude peut lire le ticket que
vous citez quand il rédige un plan.

## Plan

Ce que Claude rédige avant de construire quoi que ce soit : la proposition,
les specs, la conception et les tâches. Vous le lisez, le corrigez et le
validez avant que la moindre ligne de code soit écrite.

## Proposition

La première partie d'un plan, dans `proposal.md` : pourquoi le change est
nécessaire, ce qui va changer, et ce qu'il touche.

## Revue

Une relecture du plan ou du code par des rôles qui ne l'ont pas écrit. Ses
constats, et les réponses qui leur sont faites, sont conservés dans le
`review.md` du change.

## Rôle

Un point de vue que Claude adopte à une étape, avec sa propre liste de
vérifications : le sceptique qui cherche les failles d'un plan, le nouveau
venu qui vérifie que le code se lit facilement, l'expert sécurité. Voir
[Rôles](roles.md).

## Scénario

Un exemple concret d'exigence, en trois lignes : étant donné une situation,
quand quelque chose se produit, alors le résultat attendu. Chaque scénario
devient un test.

## Skill

Un ensemble d'instructions que Claude Code suit pour un type de tâche.
Compagnon en installe huit, comme planifier un change ou le réaliser. Claude
lance la bonne quand votre demande y correspond ; voir
[Comment demander](skills.md#comment-demander).

## Spec

Une description de ce que fait le produit, sous forme d'exigences et de
scénarios. Les specs du projet, dans `_compagnon/specs/`, décrivent le
comportement actuel et font référence.

## Tâches

La dernière partie d'un plan, dans `tasks.md` : la liste des étapes pour
construire le change, chacune avec le test qui prouve qu'elle est faite.
Claude les coche au fur et à mesure.
