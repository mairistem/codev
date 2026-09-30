# codev

**Le développement piloté par les specs pour Claude Code.**

codev ajoute une fine couche de spécifications à un dépôt, pour que vous et
votre agent de code vous accordiez sur ce qu'il faut construire avant d'écrire
la moindre ligne — et pour que les décisions d'architecture, une fois prises,
cessent d'être remises en débat à chaque change.

L'outil se compose de deux moitiés :

| Où | Quoi | Exemples |
|---|---|---|
| Votre terminal | Le binaire `codev` : le moteur | `codev init`, `codev status`, `codev archive` |
| Le chat Claude Code | Les skills générées : le volant | `/codev-propose`, `/codev-apply` |

`codev init` installe les skills dans `.claude/skills/`. Dès lors, vous
travaillez surtout dans le chat, et les skills pilotent la CLI à votre place.

codev n'appelle jamais de modèle de langage. Il gère des fichiers Markdown, le
graphe de dépendances entre les artefacts de planification, la validation et la
fusion des modifications de specs. Votre agent rédige ; codev lui indique quoi
écrire, où, et sous quelles contraintes.

## Pourquoi codev

Dans un dépôt sans specs, l'intention n'existe que dans la tête des gens. Le
code s'écrit d'un côté, se relit de l'autre, et six mois plus tard quelqu'un
fouille l'historique des commits pour retrouver pourquoi un comportement existe.
codev inverse la démarche : pour chaque change significatif, vous commencez par
écrire ce que vous voulez, vous le relisez, puis vous l'implémentez.

Ce que codev vous apporte :

- **Un cycle explicite.** Chaque change passe par quatre étapes : propose,
  apply, sync et archive. Chaque étape a son artefact et sa skill Claude Code
  dédiée. Voir [Le workflow](workflow.md).
- **Des deltas de specs, fusionnés avec précision.** Un change ne réécrit
  jamais une spec en entier. Il déclare des exigences `ADDED`, `MODIFIED`,
  `REMOVED` ou `RENAMED`, que codev fusionne dans les specs principales à
  l'archivage du change, sans toucher au reste du fichier. C'est ce qui rend
  codev utilisable sur du code existant, et pas seulement sur des projets
  neufs.
- **Des décisions d'architecture immuables.** Les décisions acceptées (ADR)
  sont scellées par une empreinte de leur contenu. `codev validate` signale
  toute réécriture silencieuse. On ne modifie pas une décision : on la
  remplace par une nouvelle.
- **Des conventions partagées entre dépôts.** Un projet peut hériter du
  contexte, des règles et des décisions d'un autre dépôt, en lecture seule,
  épinglés sur un commit git. Voir [Sources héritées](guides/inherited-sources.md).
- **La langue de votre équipe.** Les artefacts sont rédigés dans la langue
  définie dans la configuration, tandis que leur structure reste lisible par la
  machine. Voir [Langue des artefacts](guides/artifact-language.md).
- **Un binaire natif unique.** Aucun environnement d'exécution à installer.
  Des binaires précompilés pour macOS, Linux et Windows, vérifiés par SHA-256
  à l'installation.

## Origines

codev s'inspire d'[OpenSpec](https://github.com/Fission-AI/OpenSpec) (MIT),
l'outil TypeScript qui a popularisé un cycle de développement piloté par les
specs pour les agents de code. codev en reprend l'ossature, la reconstruit en
Rust et fait ses propres choix par-dessus.

**Ce que codev doit à OpenSpec :**

- Le cycle propose → apply → archive, et l'idée que le *change* est l'unité de
  travail.
- Les deltas de specs et leurs quatre opérations lisibles — `ADDED`,
  `MODIFIED`, `REMOVED`, `RENAMED`.
- La notion de *capacité*, qui regroupe un comportement observable plutôt que
  des fichiers ou des modules.
- Des workflows et des templates traités comme des données, et non comme du
  code.

**Ce que codev fait autrement :**

- **Un binaire Rust plutôt qu'un paquet Node.js.** Un exécutable précompilé,
  sans environnement d'exécution.
- **Un dossier `_codev/` visible.** Les artefacts de planification sont des
  fichiers sources à part entière : vous les lisez, les comparez et les
  relisez. Le tiret bas initial laisse le dossier visible pour des outils comme
  ripgrep et fd, qui ignorent par défaut les répertoires cachés.
- **Un cœur pur et une coquille impérative.** La bibliothèque centrale
  n'effectue aucune entrée-sortie ; chacune de ses décisions se teste sans
  disque. Voir [Architecture](architecture.md).
- **Une seule cible, bien servie.** codev cible uniquement Claude Code, là où
  OpenSpec prend en charge de nombreux outils.

**Ce qui est propre à codev :**

- Des décisions d'architecture scellées, avec remplacement, écarts locaux par
  rapport aux décisions héritées, et promotion d'une décision de design en ADR.
- Des sources héritées en lecture seule, épinglées par commit git dans
  `_codev/codev.lock`.
- Une intégration MCP côté skills : `/codev-propose` détecte un identifiant de
  ticket comme `PROJ-123` et récupère le ticket via le serveur MCP Jira
  configuré pour le projet.
- Un contrat JSON versionné sur chaque commande, pour que les skills puissent
  s'appuyer sur un format de sortie stable.
- Une documentation embarquée dans le binaire (`codev docs`), consultable hors
  ligne.

## Pour aller plus loin

- Vous découvrez codev ? Commencez par l'[Installation](installation.md), puis
  le [Démarrage rapide](quickstart.md).
- Vous voulez comprendre le modèle ? Lisez [Le workflow](workflow.md) et
  [Concepts](concepts.md).
- Vous cherchez une option ou une clé ? Consultez la
  [référence de la CLI](reference/cli.md) et la
  [référence de la configuration](reference/configuration.md).
