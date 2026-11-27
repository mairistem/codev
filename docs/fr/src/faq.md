# FAQ

## Questions générales

### Compagnon envoie-t-il mon code à un modèle ?

Non. Le programme `compagnon` ne parle jamais à un modèle d'IA. Il ne va sur
internet que pour deux choses : télécharger des règles partagées depuis un
autre [dépôt](glossary.md#dépôt), quand vous le demandez
(`compagnon sources update`), et voir si une version plus récente de
Compagnon existe. C'est votre [agent](glossary.md#agent), Claude Code, qui
rédige ; il peut lire et faire exactement ce qu'il peut faire d'habitude
sur votre ordinateur.

### Compagnon vérifie-t-il les mises à jour ?

Oui, et c'est tout ce qu'il vérifie. Au plus une fois par jour,
`compagnon status` demande à GitHub le numéro de la dernière version de
Compagnon, et garde la réponse sur votre ordinateur.

- Rien sur vous ni sur votre projet n'est envoyé. Il n'y a aucun suivi
  d'aucune sorte.
- Si la demande échoue, rien ne s'affiche, et Compagnon ne réessaie pas
  avant le lendemain.
- La vérification ne s'exécute jamais pendant les vérifications
  automatiques du projet ([intégration continue](glossary.md#intégration-continue),
  CI) : leurs résultats restent ainsi identiques d'une exécution à l'autre.
- Pour la désactiver sur votre ordinateur, définissez
  `COMPAGNON_NO_UPDATE_CHECK=1`.
- Rien n'est jamais installé sans votre accord. Pour mettre à jour, vous
  lancez `compagnon self-update`, ou vous répondez « Update now » quand une
  compétence vous le propose.

<details>
<summary>Sous le capot : la requête de mise à jour</summary>

La requête est un unique appel HTTPS anonyme à l'API publique de GitHub :
`https://api.github.com/repos/mairistem/compagnon/releases/latest`. Elle lit
le numéro de la dernière version publiée et le garde en cache dans
`~/.cache/compagnon/update-check.json`, pendant 24 heures. Elle est sautée
dès que la variable d'environnement `CI` est définie.

</details>

### Puis-je utiliser Compagnon sans Claude Code ?

Oui, en partie. La commande `compagnon` fonctionne seule : vous pouvez
créer une modification, rédiger vous-même les documents de son plan (les
[artefacts](glossary.md#artefact)) en suivant `compagnon instructions`, puis
la vérifier, la synchroniser et l'archiver. Ce que vous perdez, ce sont les
compétences (skills) : elles rédigent à votre place, et ne fonctionnent que
dans Claude Code.

### Faut-il enregistrer `_compagnon/` et `.claude/skills/` dans git ?

Oui, les deux. Un [enregistrement](glossary.md#enregistrement) (commit)
garde une version des fichiers du projet dans son historique, tenu par
[git](glossary.md#git). `_compagnon/` est la mémoire de vos
spécifications, de vos décisions et de vos modifications : enregistrez-le
en entier, y compris `_compagnon/decisions/seal.yaml` et
`_compagnon/compagnon.lock`. Quand `.claude/skills/` est enregistré, toute
l'équipe a les mêmes compétences, sans avoir à lancer `compagnon init`.

### Une petite correction exige-t-elle une modification complète ?

Chaque modification suit les mêmes étapes, mais son plan peut être très
court : une proposition d'une ligne, une seule exigence dans son
[delta](glossary.md#delta), et deux tâches. Si la modification ne change
pas le comportement du logiciel, définissez `skip_specs: true` dans son
`change.yaml` : elle n'a alors besoin d'aucune spécification.

### Puis-je tenir Compagnon à l'écart de `CLAUDE.md`, de la CI ou du modèle de demande de fusion ?

Oui. Lors de la mise en place du projet, `compagnon init --no-claude-md`,
`--no-ci` et `--no-pr-template` laissent chacun d'eux tranquille, et
`--no-integration` fait les trois ; `compagnon update` ne les ajoute jamais
par la suite. Vous pouvez aussi les reprendre en main après coup. Voir
[Intégration au dépôt et compagnon doctor](guides/repository-integration.md).

### Claude peut-il partir d'un ticket Jira ?

Oui. Dites « Prépare un plan pour PROJ-123 » : Claude lit le ticket une
fois, sans rien modifier dans Jira, et le cite en tête du plan. Voir
[Partir d'un ticket Jira](guides/mcp.md).

### Mes relecteurs doivent-ils travailler isolément ?

Par défaut, la compétence qui a écrit le plan ou le code le relit aussi,
en jouant chaque [rôle](glossary.md#rôle) de relecteur tour à tour, dans
la même conversation. En mode isolé, chaque relecteur travaille seul, comme
un [sous-agent](glossary.md#sous-agent) séparé qui ne voit jamais votre
conversation. Cela coûte plus cher et le bénéfice n'est pas encore mesuré :
commencez par les modifications où un regard neuf compte le plus. Voir
[Relecture en ligne ou isolée](roles.md#relecture-en-ligne-ou-isolée).

### Comment partager cette documentation ?

Enregistrez-la sous la forme d'une seule page web, qui ne charge rien
depuis internet : vous pouvez l'envoyer par e-mail ou la déposer dans un
wiki.

```bash
compagnon docs --lang fr --write ~/Downloads/compagnon-manual-fr.html
```

### Quelque chose ne va pas : où chercher ?

Quand une commande signale un problème, elle le nomme par un code, comme
`decision_seal_mismatch`. [Codes d'erreur et solutions](reference/errors.md)
liste chaque code, avec ce qu'il signifie et comment le corriger, ainsi
que les problèmes courants qui n'ont pas de code, comme des compétences
qui n'apparaissent pas dans Claude Code.

## Pour aller plus loin

- [Codes d'erreur et solutions](reference/errors.md)
- [Installation](installation.md)
- [Parler à Claude](skills.md)
- [Partir d'un ticket Jira](guides/mcp.md)
- [Rôles](roles.md)
- [Glossaire](glossary.md)
