# Glossaire

Chaque terme employé par la documentation, en mots simples, par ordre
alphabétique. Quand l'outil affiche un mot anglais, il est donné entre
parenthèses.

## Agent

Un programme qui accomplit une tâche de lui-même, pas à pas, en décidant de
la suite : lire des fichiers, lancer des commandes, écrire du code. Claude,
quand il travaille dans Claude Code, est un agent. Plusieurs agents peuvent
travailler côte à côte, chacun sur sa part du travail.

## Archive

La dernière étape d'une modification. Une fois le travail fini et vérifié,
l'archivage reporte ses spécifications dans celles du projet et range son
dossier, daté, dans `_compagnon/changes/archive/`. Rien n'est perdu : le
plan reste lisible, comme trace.

## Artefact

L'un des fichiers qui forment un plan : la proposition, les spécifications,
la conception ou les tâches. Compagnon les appelle des artefacts ; vous
pouvez simplement y voir les quatre parties du plan.

## Capacité

Un domaine de ce que fait le produit, comme « connexion » ou « factures ».
Les spécifications du projet sont rangées par capacité, chacune dans son
dossier sous `_compagnon/specs/`.

## Claude Code

L'assistant de programmation d'Anthropic. Il fonctionne dans votre
terminal, votre éditeur de code ou les applications Claude, lit votre dépôt
et agit sur lui à votre demande. Compagnon lui ajoute ses compétences.

## Compétence

Un ensemble d'instructions que Claude Code suit pour un type de tâche
(en anglais : skill). Compagnon en installe huit, comme planifier une
modification ou la réaliser. Claude lance la bonne quand votre demande y
correspond ; voir [Comment demander](skills.md#comment-demander).

## Conception

La partie d'un plan qui dit comment la modification sera construite : les
choix techniques, les solutions écartées, les risques. Rédigée dans
`design.md`.

## Décision

Un choix fait une fois par l'équipe, pour toutes les modifications, comme
« les prix sont toujours affichés TTC ». Les décisions sont rangées dans
`_compagnon/decisions/` ; Claude les suit au lieu de les remettre en cause.

## Delta

La partie des spécifications d'une modification qui dit ce qui change dans
celles du projet : exigences ajoutées, modifiées ou retirées. À
l'archivage, le delta est reporté dans les spécifications du projet.

## Demande de fusion

Une demande pour ajouter un travail au projet partagé, sur GitHub ou
GitLab (en anglais : pull request). L'équipe la lit, la commente, puis
l'approuve ou demande des corrections ; le plan et le code y arrivent
souvent ensemble.

## Dépôt

Le dossier qui contient les fichiers d'un projet et tout leur historique,
le plus souvent partagé sur GitHub ou GitLab. Compagnon y range ses
plans, dans le dossier `_compagnon/`.

## Déroulé

La suite d'étapes que suit chaque modification : demander, planifier,
relire, réaliser, archiver (en anglais : workflow). Chaque étape a sa
compétence. Dans la configuration, `workflows:` liste les compétences
qu'un projet installe.

## Domaine

Un sujet sensible qu'une modification peut toucher : sécurité, données
personnelles, interface, performances, exploitation, migration. La
proposition liste les domaines touchés, et chacun fait venir son expert
pour relire le plan et le code.

## Enregistrement

Une étape sauvegardée de l'historique du projet, avec un court message qui
dit ce qui a changé (en anglais : commit). Claude peut le faire pour
vous.

## Exigence

Une règle que le produit doit respecter, écrite pour pouvoir être vérifiée :
« l'application DOIT suivre le jeu de couleurs du système ». Dans les
fichiers, le mot-clé peut apparaître en anglais : SHALL ou MUST veulent
dire « doit ». Chaque exigence est accompagnée de scénarios.

## Frontmatter

Les quelques lignes tout en haut d'un fichier, entre deux lignes `---`, qui
décrivent le fichier au lieu d'en faire partie : son nom, sa version.
Chaque compétence commence par cet en-tête.

## Git

L'outil qui garde l'historique des fichiers d'un projet : qui a changé
quoi, quand, et pourquoi. Les plans de Compagnon vivent dans cet
historique, à côté du code.

## Intégration continue

Des vérifications qu'un service comme GitHub lance tout seul chaque fois
qu'on propose un changement au projet, par exemple les tests et
`compagnon validate` (en anglais : CI). Une vérification en rouge
signale quelque chose à corriger.

## Kebab-case

Une façon d'écrire un nom en mots minuscules reliés par des tirets, sans
espace : `add-dark-mode`. Claude nomme ainsi chaque modification, pour que
le nom serve de nom de dossier.

## MCP

Model Context Protocol : une façon standard de relier Claude à d'autres
outils, comme Jira. Avec une connexion Jira, Claude peut lire le ticket que
vous citez quand il rédige un plan.

## Modification

Un travail, de la demande jusqu'à son archivage : une fonctionnalité, une
correction, une refonte (en anglais : change). Chaque modification a son
propre dossier sous `_compagnon/changes/`, qui contient son plan.

## Plan

Ce que Claude rédige avant de construire quoi que ce soit : la proposition,
les spécifications, la conception et les tâches. Vous le lisez, le
corrigez et le validez avant que la moindre ligne de code soit écrite.

## Proposition

La première partie d'un plan, dans `proposal.md` : pourquoi la
modification est nécessaire, ce qui va changer, et ce qu'elle touche.

## Relecture

Une relecture du plan ou du code par des rôles qui ne l'ont pas écrit (en
anglais : review). Ses constats, et les réponses qui leur sont faites,
sont conservés dans le fichier `review.md` de la modification.

## Rôle

Un point de vue que Claude adopte à une étape, avec sa propre liste de
vérifications : le sceptique qui cherche les failles d'un plan, le nouveau
venu qui vérifie que le code se lit facilement, l'expert sécurité. Voir
[Rôles](roles.md).

## Sceau

Une empreinte du texte d'une décision acceptée. Si quelqu'un modifie la
décision ensuite, l'empreinte ne correspond plus et `compagnon validate`
le signale.

## Scénario

Un exemple concret d'exigence, en trois lignes : étant donné une situation,
quand quelque chose se produit, alors le résultat attendu. Chaque scénario
devient un test.

## Schéma

La liste des parties que contient un plan, et l'ordre dans lequel elles
s'écrivent. Le schéma habituel de Compagnon demande une proposition, des
spécifications, une conception et des tâches ; une équipe peut définir le
sien.

## Sous-agent

Une copie séparée de Claude, lancée pour une seule tâche, qui ne voit pas
votre conversation. Compagnon s'en sert pour qu'un relecteur juge un plan
ou du code sans connaître les intentions de son auteur.

## Spécification

Une description de ce que fait le produit, sous forme d'exigences et de
scénarios (en anglais : spec). Les spécifications du projet, dans
`_compagnon/specs/`, décrivent le comportement actuel et font référence.

## Tâches

La dernière partie d'un plan, dans `tasks.md` : la liste des étapes pour
réaliser la modification, chacune avec le test qui prouve qu'elle est
faite. Claude les coche au fur et à mesure.

## Terminal

La fenêtre où l'on tape des commandes, comme `compagnon init`. Sur macOS,
elle s'appelle Terminal ; sur Windows, PowerShell. Claude Code peut aussi
y fonctionner.

## Test

Un petit programme qui vérifie que le produit fonctionne comme prévu, par
exemple « en mode sombre, la page est sombre ». Chaque scénario d'un plan
devient un test, et Claude les lance pour vérifier son propre travail.
