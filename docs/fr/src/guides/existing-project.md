# Adopter Compagnon sur un projet existant

Votre projet a déjà du code, des tests et un historique. Inutile de tout
décrire avant d'utiliser Compagnon : vous l'installez en quelques minutes,
puis les spécifications se construisent une modification après l'autre, en
commençant par les parties que vous touchez.

## Ce que compagnon init ajoute à votre projet

Lancez-le à la racine de votre [dépôt](../glossary.md#dépôt) :

```bash
compagnon init
```

Il regarde ce qui existe déjà (les manifestes du projet comme `Cargo.toml`
ou `package.json`, la licence, `.github/workflows/`, votre langue, les
serveurs MCP déclarés), pose au plus deux questions — quelles compétences
installer, et une courte description du projet — puis ajoute :

| Quoi | Où |
|---|---|
| Le dossier de Compagnon : spécifications, décisions, modifications, archive, schémas, et la configuration | `_compagnon/`, `_compagnon/config.yaml` |
| Les compétences Claude Code | `.claude/skills/compagnon-<nom>/SKILL.md` |
| Une section qui demande à Claude de planifier avant de changer un comportement | `CLAUDE.md`, créé s'il manque |
| Sur GitHub, une tâche d'[intégration continue](../glossary.md#intégration-continue) qui lance `compagnon validate --strict` à chaque envoi de modifications vers le dépôt partagé et à chaque [demande de fusion](../glossary.md#demande-de-fusion) | `.github/workflows/compagnon.yml` |
| Sur GitHub, une section Compagnon dans le modèle de demande de fusion | votre modèle existant, ou `.github/PULL_REQUEST_TEMPLATE.md` |

Votre propre texte n'est jamais remplacé. Les sections de Compagnon dans
`CLAUDE.md` et dans le modèle de demande de fusion se placent entre les
marqueurs `<!-- compagnon:begin -->` et `<!-- compagnon:end -->`, ajoutées à
la fin de votre fichier ; tout ce qui est en dehors reste à vous. Si l'un de
vos fichiers d'intégration continue lance déjà `compagnon validate`, ou si
`.github/workflows/compagnon.yml` est un fichier que vous avez écrit,
Compagnon n'y touche pas. Sur GitLab, il ne modifie jamais `.gitlab-ci.yml` :
il affiche une tâche à y coller. Pour sauter l'un de ces fichiers, passez
`--no-claude-md`, `--no-ci`, `--no-pr-template` ou `--no-integration`.
Le détail : [Ce que fait `compagnon init`](../reference/cli.md#ce-que-fait-compagnon-init)
et [Intégration au dépôt et compagnon doctor](repository-integration.md).

Si `_compagnon/config.yaml` existe déjà, `compagnon init` le laisse tel
quel : il installe ou met à jour seulement les compétences et ces sections.

## Rien à documenter d'avance

Juste après `compagnon init`, `compagnon doctor` signale, entre autres :

```text
Planning
  ✗ specs            no main spec in _compagnon/specs/
      fix: /compagnon-propose <a change> — documenting existing behavior is planned work too
```

C'est attendu, et cela ne vous bloque pas. Compagnon fonctionne sans
aucune spécification : `compagnon validate` passe, et vous pouvez planifier
votre première modification tout de suite. Le conseil affiché le dit à sa
façon : documenter un comportement existant est aussi un travail qui se
planifie.

Les spécifications grandissent ensuite modification après modification.
Chaque modification porte un [delta](../glossary.md#delta) : les exigences
qu'elle ajoute, modifie, supprime ou renomme, qui décrivent le comportement
qu'elle touche. Quand vous archivez la modification, son delta est fusionné
dans `_compagnon/specs/`. Au bout de quelques mois, les parties du produit
qui ont bougé sont décrites ; celles que personne n'a touchées ne le sont
pas, et n'ont pas besoin de l'être tant que personne n'y touche.

## Commencer par une première modification

Choisissez l'une de ces deux voies :

- **Une petite modification réelle.** Quelque chose que vous alliez faire
  de toute façon, comme une correction ou une petite fonctionnalité. Dans
  Claude Code :

  > /compagnon-propose Permettre d'exporter ses factures au format CSV

  Le plan décrit le nouveau comportement ; l'archiver crée la première
  spécification. [Votre première modification](../quickstart.md) détaille
  les quatre étapes.
- **Une modification qui documente l'existant.** Choisissez une partie du
  produit qui change souvent, ou que les nouveaux venus comprennent mal, et
  demandez un plan qui écrit comment elle se comporte aujourd'hui :

  > /compagnon-propose Documenter comment fonctionne aujourd'hui la numérotation des factures, sans la changer

  Les exigences du plan décrivent le comportement actuel ; une fois
  archivées, elles forment la spécification de cette partie, et la
  prochaine modification qui la touche part de là.

Avant l'une ou l'autre, `/compagnon-configure` peut lire votre README,
votre guide de contribution, votre documentation et un échantillon de votre
code, puis proposer un `context:` et des `rules:` pour
`_compagnon/config.yaml` : la technique employée, les conventions, ce que
chaque partie d'un plan doit couvrir. Il vous montre les différences et
n'écrit rien avant votre confirmation. Voir
[Configuration](../reference/configuration.md#context).

## Consigner les décisions déjà prises

Votre projet repose déjà sur des choix : une base de données, un cadre de développement (framework),
la façon dont les modules dépendent les uns des autres. Consignez ceux que
Claude doit respecter sous forme de [décisions](../concepts.md#décision) :

```bash
compagnon decision new "Utiliser PostgreSQL pour la persistance"
# rédigez le contexte, le choix et ses conséquences dans le nouveau fichier
compagnon decision accept 0001
```

`compagnon decision new` crée le fichier dans `_compagnon/decisions/`, à
l'état `proposed`, pour que vous puissiez le modifier librement ;
`compagnon decision accept` le met en vigueur et scelle son texte. Dès
lors, chaque conception rédigée par Claude la suit, ou propose
explicitement de la remplacer. Commencez par deux ou trois décisions qui
comptent ; `compagnon doctor` n'émet qu'un avertissement quand il n'y en a
aucune.

## Pour continuer

- [Votre première modification](../quickstart.md) : les quatre étapes, de
  la demande à l'archive.
- [Configuration](../reference/configuration.md) : chaque réglage de
  `_compagnon/config.yaml`.
- [Concepts](../concepts.md) : spécifications, modifications, deltas et
  décisions.
