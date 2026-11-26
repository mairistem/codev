# Compagnon en deux minutes

**Planifiez avec Claude avant qu'il ne code.**

Compagnon vous aide à travailler avec Claude Code, l'assistant de
programmation d'Anthropic, sans perdre la main. Avant d'écrire la moindre
ligne de code, Claude met par écrit ce qu'il s'apprête à construire : un
plan court que vous pouvez lire, discuter et corriger. Une fois que vous
êtes d'accord, Claude construit exactement cela, et le plan reste dans votre
[dépôt](glossary.md#dépôt) (le dossier qui contient les fichiers du projet)
comme trace de ce qui a été fait, et pourquoi.

Inutile de savoir comment fonctionne l'IA pour l'utiliser. Vous demandez
avec vos mots — « ajoute un mode sombre qui suit le réglage du système » — et
Compagnon guide Claude pour le reste.

## Qui décide

Vous. À chaque étape, Claude propose et vous décidez :

1. **Vous demandez** un changement, avec vos mots.
2. **Claude propose un plan** : pourquoi ce changement, ce qui change,
   comment, et la liste des tâches.
3. **Vous le relisez et le validez** — ou demandez des corrections. Rien
   n'est construit avant votre accord.
4. **Claude le réalise**, tâche par tâche, et vérifie son propre travail.
5. **Vous relisez le résultat** et clôturez la modification.

Rien n'est appliqué sans votre accord, et chaque décision est consignée là
où toute votre équipe peut la lire.

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
  changement.

## Par où commencer

Qu'allez-vous faire avec Compagnon ? Chaque réponse ouvre un parcours
court :

- [Je relis et je valide des plans — je ne code pas](paths/review.md)
- [Je ne code pas, mais je veux que Claude construise quelque chose pour moi](paths/build.md)
- [Je suis développeur et je fais coder Claude](paths/develop.md)
- [J'installe Compagnon pour mon équipe](paths/team.md)

Chaque terme est expliqué, en mots simples, dans le [Glossaire](glossary.md).

Jusqu'à la 0.6, Compagnon s'appelait codev : [Migrer depuis codev](guides/moving-from-codev.md)
explique comment convertir un projet créé à cette époque. D'où vient
Compagnon, et pourquoi, est raconté dans [Origines](origins.md).
