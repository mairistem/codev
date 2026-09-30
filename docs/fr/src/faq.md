# FAQ et dépannage

## Questions générales

### codev envoie-t-il mon code à un modèle ?

Non. Le binaire `codev` n'appelle jamais de modèle de langage et, hormis
`codev sources update` qui récupère les sources git héritées, n'accède jamais
au réseau. C'est votre agent — Claude Code — qui rédige, avec les mêmes accès
que dans n'importe quelle autre session.

### Puis-je utiliser codev sans Claude Code ?

La CLI fonctionne seule : vous pouvez créer des changes, rédiger vous-même les
artefacts en suivant `codev instructions`, valider, synchroniser et archiver.
Les skills, qui rédigent à votre place, ciblent uniquement Claude Code.

### Faut-il commiter `_codev/` et `.claude/skills/` ?

Oui. `_codev/` est la mémoire de vos specs, de vos décisions et de vos
changes : commitez-le en entier, y compris `_codev/decisions/seal.yaml` et
`_codev/codev.lock`. Commiter `.claude/skills/` donne à toute l'équipe les
mêmes skills, sans avoir à lancer `codev init`.

### Une petite correction exige-t-elle un change complet ?

Chaque change suit le même cycle, mais le plan peut rester modeste : une
proposal d'une ligne, un delta d'une seule exigence, une liste de deux tâches.
Pour un change sans impact sur le comportement, définissez
`skip_specs: true` dans son `change.yaml`.

### Comment partager cette documentation ?

```bash
codev docs --lang fr --write ~/Downloads/codev-manual-fr.html
```

Le fichier est autonome — CSS intégré, aucune ressource externe — et peut être
envoyé par e-mail ou déposé dans un wiki.

## Dépannage

### `no codev project found`

```text
error: no codev project found from /home/you — run `codev init` at the project root
```

codev cherche `_codev/` dans le répertoire courant et dans ses parents. Lancez
la commande depuis votre projet, ou initialisez-le avec `codev init`.

### Les skills n'apparaissent pas dans Claude Code

Claude Code découvre les skills au démarrage d'une session. Après
`codev init` ou `codev update`, redémarrez Claude Code ou ouvrez une nouvelle
session. Vérifiez que les fichiers `.claude/skills/codev-*/SKILL.md` existent à
la racine du projet que vous avez ouvert.

### `codev update` laisse une skill en place

```text
  Left in place because edited by hand — rerun with --force to overwrite:
```

Cette skill a été modifiée à la main, et codev ne supprime pas votre travail.
Conservez la modification et ignorez le message, ou lancez
`codev update --force` pour la régénérer (vérifiez d'abord `git diff`).

### `codev validate` signale `zero_delta_without_marker`

Le change ne comporte aucun delta de spec sous `specs/`. Rédigez le delta — le
comportement que le change ajoute ou modifie — ou, si le change n'a vraiment
aucun impact sur le comportement, ajoutez `skip_specs: true` à son
`change.yaml`.

### `codev validate` signale `decision_seal_mismatch`

Le contenu d'une décision scellée a été modifié — à la main, ou par un outil
de formatage. Les décisions acceptées sont immuables :

- **Restaurez** le contenu d'origine (`git diff` montre ce qui a changé) ; le
  sceau redevient valide.
- Ou, si la modification était volontaire, **scellez-la à nouveau** :
  `codev decision seal <ID> --force`.

Pour rédiger une décision avant de la sceller, laissez-la `proposed` — comme
la créent `codev decision new`, `supersede`, `deviate` et `promote` — et lancez
`codev decision accept <ID>` une fois son texte définitif. Une décision créée
avec `codev decision new --status accepted` est scellée immédiatement, avec le
texte d'exemple du template.

Pour changer ce que dit une décision, remplacez-la plutôt avec
`codev decision supersede <ID> "<new title>"`, rédigez la nouvelle décision,
puis acceptez-la.

Pour tenir les outils de formatage à l'écart des décisions, excluez
`_codev/decisions/` dans leur configuration.

### `codev decision accept` signale `predecessor_not_accepted`

La décision que vous acceptez liste, dans `supersedes`, une décision qui n'est
plus `accepted` — le plus souvent parce qu'une autre décision l'a remplacée
après la création de la vôtre. Rien n'a été écrit. Lancez
`codev decision list` pour voir quelle décision est désormais en vigueur, puis
faites pointer `supersedes` vers elle dans le frontmatter de la vôtre, ou
abandonnez la vôtre si elle est devenue inutile. `codev decision supersede`
refuse avec le même code une décision qui n'est pas `accepted`.

### `codev validate` signale `decision_missing_frontmatter`

Chaque fichier `.md` de `_codev/decisions/` est analysé comme une décision.
Sortez de ce dossier les autres fichiers Markdown, un README par exemple.

### La synchronisation ou l'archivage refuse un change

```text
error: change `add-dark-mode` has errors; run `codev validate add-dark-mode` for details
```

Avec `--json`, le code d'erreur est `validation_failed`. Lancez la commande
`codev validate` suggérée, corrigez les constats, puis synchronisez ou
archivez à nouveau. Si l'archivage est refusé parce qu'une exigence
`MODIFIED` n'existe pas dans la spec principale, c'est que son nom diffère de
celui de la spec principale : corrigez le nom, ou utilisez `ADDED` si
l'exigence est nouvelle.

### `codev validate` signale `rename_source_missing`

Le nom qui suit `FROM:` dans `## RENAMED Requirements` ne correspond à aucun
titre `### Requirement:` de la spec principale. Il doit correspondre
exactement, que vous écriviez le nom seul ou le titre complet entre accents
graves :

```markdown
- FROM: `### Requirement: Le thème choisi est mémorisé`
- TO: `### Requirement: Le thème choisi suit l'utilisateur`
```

### `codev validate` signale `delta_unexpected_heading`

Un titre `###` d'une section de delta n'est pas un `### Requirement:` — le
plus souvent, un mot-clé traduit comme `### Exigence :`. Les mots-clés restent
en anglais quelle que soit la langue des artefacts ; voir
[Langue des artefacts](guides/artifact-language.md).

### `/codev-propose` ignore mon ticket

1. `mcp.jira_tool` n'était pas défini dans `_codev/config.yaml` lors de la
   génération des skills : `.claude/skills/codev-propose/SKILL.md` affiche
   alors « (Jira MCP not configured) » à la place du nom de l'outil. Ajoutez la
   clé, puis lancez `codev update --force`.
2. L'outil est configuré mais indisponible dans la session Claude Code en
   cours — nom erroné, serveur non connecté, session expirée. La skill vous le
   signale et rédige la proposal à partir de votre seule demande.

Voir [Jira et autres serveurs MCP](guides/mcp.md).

### Une source git est `unlocked` ou `needs_update`

Lancez `codev sources update`. `unlocked` signifie que la source n'a jamais
été résolue ; `needs_update` signifie qu'elle est épinglée dans `codev.lock`
mais absente de votre cache local, typiquement sur une machine neuve.

### `codev: command not found`

Le dossier d'installation n'est pas dans votre `PATH`. Voir
[Vérifier l'installation](installation.md#vérifier-linstallation).
