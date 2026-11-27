# Origines

Pourquoi Compagnon existe, ce qu'il doit aux outils qui l'ont précédé, et où le voir à l'œuvre.

## Pourquoi Compagnon

Dans un dépôt sans spécifications, l'intention n'existe que dans la tête des gens. Le
code s'écrit d'un côté, se relit de l'autre, et six mois plus tard quelqu'un
fouille l'historique des enregistrements pour retrouver pourquoi un comportement existe.
Compagnon inverse la démarche : pour chaque modification significative, vous commencez par
écrire ce que vous voulez, vous le relisez, puis vous l'implémentez.

Ce que vous y gagnez au quotidien est expliqué dans
[Compagnon en deux minutes](introduction.md) et [Comment ça marche](workflow.md) ;
ce qui le distingue des outils qui l'ont précédé est décrit ci-dessous.

## Origines

Compagnon s'inspire d'[OpenSpec](https://github.com/Fission-AI/OpenSpec) (MIT),
l'outil TypeScript qui a popularisé un cycle de développement piloté par les
spécifications pour les agents de code. Compagnon en reprend l'ossature, la reconstruit en
Rust et fait ses propres choix par-dessus.

**Ce que Compagnon doit à OpenSpec :**

- Le cycle proposer → réaliser → archiver, et l'idée que la *modification* (change) est
  l'unité de travail.
- Les deltas de spécifications et leurs quatre opérations lisibles — `ADDED`,
  `MODIFIED`, `REMOVED`, `RENAMED`.
- La notion de *capacité*, qui regroupe un comportement observable plutôt que
  des fichiers ou des modules.
- Des déroulés et des modèles traités comme des données, et non comme du
  code.

**Ce que Compagnon fait autrement :**

- **Un binaire Rust plutôt qu'un paquet Node.js.** Un exécutable précompilé,
  sans environnement d'exécution.
- **Un dossier `_compagnon/` visible.** Les artefacts de planification sont des
  fichiers sources à part entière : vous les lisez, les comparez et les
  relisez. Le tiret bas initial laisse le dossier visible pour des outils comme
  ripgrep et fd, qui ignorent par défaut les répertoires cachés.
- **Un cœur pur et une coquille impérative.** La bibliothèque centrale
  n'effectue aucune entrée-sortie ; chacune de ses décisions se teste sans
  disque. Voir [Architecture](architecture.md).
- **Une seule cible, bien servie.** Compagnon cible uniquement Claude Code, là où
  OpenSpec prend en charge de nombreux outils.

**Ce qui est propre à Compagnon :**

- Des décisions d'architecture scellées, avec remplacement, écarts locaux par
  rapport aux décisions héritées, et promotion d'une décision de conception en décision
  d'architecture.
- Des sources héritées en lecture seule, épinglées par enregistrement git dans
  `_compagnon/compagnon.lock`.
- Une intégration MCP côté compétences : `/compagnon-propose` détecte un identifiant de
  ticket comme `PROJ-123` et récupère le ticket via le serveur MCP Jira
  configuré pour le projet.
- Un contrat JSON versionné sur chaque commande, pour que les compétences puissent
  s'appuyer sur un format de sortie stable.
- Une documentation embarquée dans le binaire (`compagnon docs`), consultable hors
  ligne.

**Le nom.** Compagnon vient du compagnonnage : un maître mène l'ouvrage, les
compagnons le réalisent, et chacun suit des plans arrêtés à l'avance. Ici,
vous êtes le maître et les agents Claude sont vos compagnons. Jusqu'à la 0.6,
le produit s'appelait codev ; [Migrer depuis codev](guides/moving-from-codev.md)
explique comment convertir un projet créé à cette époque.

## Développé avec Compagnon

Compagnon est développé avec Compagnon : chaque évolution de son dépôt est planifiée,
relue et archivée avec l'outil lui-même. Son dossier `_compagnon/` est un exemple
réel que vous pouvez parcourir :

- [les spécifications vivantes](https://github.com/mairistem/compagnon/tree/main/_compagnon/specs), qui décrivent ce que fait Compagnon
  aujourd'hui ;
- [les modifications archivées](https://github.com/mairistem/compagnon/tree/main/_compagnon/changes/archive), chacune avec sa proposition,
  ses deltas de spécification, sa conception et ses tâches ;
- [un compte rendu de relecture](https://github.com/mairistem/compagnon/blob/main/_compagnon/changes/archive/2026-10-13-review-record/review.md), avec les remarques, leurs preuves, les
  réponses de l'auteur et la façon dont chacune a été tranchée ;
- [les décisions d'architecture](https://github.com/mairistem/compagnon/tree/main/_compagnon/decisions), scellées.

## Contribuer

1. Lisez l'[Architecture](architecture.md) : les modules (crates), le cœur pur, les
   conventions d'erreurs et de tests.
2. Lisez [Contribuer à Compagnon](contributing.md) : compiler, lancer les
   tests, et proposer une modification Compagnon avant d'ouvrir une demande
   de fusion.
