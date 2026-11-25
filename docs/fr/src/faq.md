# FAQ et dépannage

## Questions générales

### Compagnon envoie-t-il mon code à un modèle ?

Non. Le binaire `compagnon` n'appelle jamais de modèle de langage. Il n'accède
au réseau que pour récupérer les sources git héritées (`compagnon sources update`)
et pour chercher une release plus récente de Compagnon (`compagnon status`,
`compagnon self-update`). C'est votre agent — Claude Code — qui rédige, avec les mêmes accès
que dans n'importe quelle autre session.

### Compagnon vérifie-t-il les mises à jour ?

Oui, et seulement cela. Au plus une fois toutes les 24 heures,
`compagnon status` envoie une requête HTTPS anonyme à l'API publique de GitHub
(`https://api.github.com/repos/mairistem/compagnon/releases/latest`) pour lire le
numéro de la dernière release, et le garde en cache dans
`~/.cache/compagnon/update-check.json`. La requête ne contient rien sur vous ni
sur votre projet ; il n'y a aucune télémétrie. Une requête qui échoue reste
silencieuse et n'est pas retentée avant le lendemain.

La vérification ne s'exécute jamais lorsque `CI` est défini, afin que les
pipelines restent reproductibles. Pour la désactiver sur votre machine,
définissez `COMPAGNON_NO_UPDATE_CHECK=1`. Rien n'est jamais installé sans votre
accord : la mise à jour passe par `compagnon self-update`, ou par votre réponse
« Update now » dans une skill.

### Puis-je utiliser Compagnon sans Claude Code ?

La CLI fonctionne seule : vous pouvez créer des changes, rédiger vous-même les
artefacts en suivant `compagnon instructions`, valider, synchroniser et archiver.
Les skills, qui rédigent à votre place, ciblent uniquement Claude Code.

### Faut-il commiter `_compagnon/` et `.claude/skills/` ?

Oui. `_compagnon/` est la mémoire de vos specs, de vos décisions et de vos
changes : commitez-le en entier, y compris `_compagnon/decisions/seal.yaml` et
`_compagnon/compagnon.lock`. Commiter `.claude/skills/` donne à toute l'équipe les
mêmes skills, sans avoir à lancer `compagnon init`.

### Une petite correction exige-t-elle un change complet ?

Chaque change suit le même cycle, mais le plan peut rester modeste : une
proposal d'une ligne, un delta d'une seule exigence, une liste de deux tâches.
Pour un change sans impact sur le comportement, définissez
`skip_specs: true` dans son `change.yaml`.

### Puis-je tenir Compagnon à l'écart de `CLAUDE.md`, de la CI ou du modèle de pull request ?

Oui. Passez `--no-claude-md`, `--no-ci` ou `--no-pr-template` à `compagnon init`,
ou `--no-integration` pour les trois. `compagnon update` ne les ajoute jamais par
la suite. Pour garder le workflow tout en en reprenant la main, supprimez sa
première ligne, `# compagnon:managed` : Compagnon laisse alors le fichier tranquille.
Pour retirer le texte de Compagnon de `CLAUDE.md` ou du modèle, supprimez le bloc
compris entre les marqueurs `compagnon:begin` et `compagnon:end`. Voir
[Intégration au dépôt et compagnon doctor](guides/repository-integration.md).

### Mes relecteurs doivent-ils travailler isolément ?

Par défaut (`inline`), la skill qui a écrit le plan ou le code le relit
aussi, un rôle après l'autre, dans la même conversation. Avec
`review: { mode: isolated }` dans `_compagnon/config.yaml`, chaque rôle
relecteur travaille seul, comme sous-agent de Claude Code qui ne voit
jamais la conversation, puis la skill fusionne ce qu'ils ont trouvé. Cela
coûte plus cher — chaque relecteur lit la matière — et le bénéfice n'est
pas encore mesuré : commencez par les changes où un regard neuf compte le
plus, en définissant `review: { mode: isolated }` dans leur
`change.yaml`, ou pour tout le projet une fois la différence constatée.
Lancez `compagnon update` après avoir changé le mode du projet. Voir
[Revue en ligne ou isolée](roles.md#revue-en-ligne-ou-isolée).

### Comment partager cette documentation ?

```bash
compagnon docs --lang fr --write ~/Downloads/compagnon-manual-fr.html
```

Le fichier est autonome — CSS intégré, aucune ressource externe — et peut être
envoyé par e-mail ou déposé dans un wiki.

## Dépannage

### `no compagnon project found`

```text
error: no compagnon project found from /home/you — run `compagnon init` at the project root
```

Compagnon cherche `_compagnon/` dans le répertoire courant et dans ses parents. Lancez
la commande depuis votre projet, ou initialisez-le avec `compagnon init`.

### Les skills n'apparaissent pas dans Claude Code

Claude Code découvre les skills au démarrage d'une session. Après
`compagnon init` ou `compagnon update`, redémarrez Claude Code ou ouvrez une nouvelle
session. Vérifiez que les fichiers `.claude/skills/compagnon-*/SKILL.md` existent à
la racine du projet que vous avez ouvert.

### `compagnon update` laisse une skill en place

```text
  Left in place because edited by hand — rerun with --force to overwrite:
```

Cette skill a été modifiée à la main — son contenu ne correspond plus à son
empreinte —, et Compagnon ne supprime pas votre travail. Conservez la
modification et ignorez le message, ou lancez `compagnon update --force` pour la
régénérer (vérifiez d'abord `git diff`). Une skill générée avant les
empreintes est préservée de la même façon, jusqu'à ce qu'un
`compagnon update --force` lui en donne une.

### Le sous-agent d'un relecteur échoue

Lors d'une revue isolée, un sous-agent qui échoue ou qui est arrêté est
relancé une fois ; s'il échoue encore, ce rôle relit en ligne et le
compte rendu dit pourquoi, par une ligne `Fallback:` sous le rôle. Rien
n'est perdu : la revue est faite, simplement pas isolément pour ce rôle.
Si tous les rôles se rabattent sur la revue en ligne, vérifiez que Claude
Code peut lancer des sous-agents (l'outil `Agent`) et que `compagnon doctor`
indique des skills à jour pour le mode de revue : `skills_review_mode`
signifie que `compagnon update` n'a pas été lancé depuis le changement de
mode.

### `compagnon validate` signale `zero_delta_without_marker`

Le change ne comporte aucun delta de spec sous `specs/`. Rédigez le delta — le
comportement que le change ajoute ou modifie — ou, si le change n'a vraiment
aucun impact sur le comportement, ajoutez `skip_specs: true` à son
`change.yaml`.

### `compagnon validate` signale `decision_seal_mismatch`

Le contenu d'une décision scellée a été modifié — à la main, ou par un outil
de formatage. Les décisions acceptées sont immuables :

- **Restaurez** le contenu d'origine (`git diff` montre ce qui a changé) ; le
  sceau redevient valide.
- Ou, si la modification était volontaire, **scellez-la à nouveau** :
  `compagnon decision seal <ID> --force`.

Pour rédiger une décision avant de la sceller, laissez-la `proposed` — comme
la créent `compagnon decision new`, `supersede`, `deviate` et `promote` — et lancez
`compagnon decision accept <ID>` une fois son texte définitif. Une décision créée
avec `compagnon decision new --status accepted` est scellée immédiatement, avec le
texte d'exemple du template.

Pour changer ce que dit une décision, remplacez-la plutôt avec
`compagnon decision supersede <ID> "<new title>"`, rédigez la nouvelle décision,
puis acceptez-la.

Pour tenir les outils de formatage à l'écart des décisions, excluez
`_compagnon/decisions/` dans leur configuration.

### `compagnon decision accept` signale `predecessor_not_accepted`

La décision que vous acceptez liste, dans `supersedes`, une décision qui n'est
plus `accepted` — le plus souvent parce qu'une autre décision l'a remplacée
après la création de la vôtre. Rien n'a été écrit. Lancez
`compagnon decision list` pour voir quelle décision est désormais en vigueur, puis
faites pointer `supersedes` vers elle dans le frontmatter de la vôtre, ou
abandonnez la vôtre si elle est devenue inutile. `compagnon decision supersede`
refuse avec le même code une décision qui n'est pas `accepted`.

### `compagnon validate` signale `decision_missing_frontmatter`

Chaque fichier `.md` de `_compagnon/decisions/` est analysé comme une décision.
Sortez de ce dossier les autres fichiers Markdown, un README par exemple.

### Une spec archivée avec un Compagnon antérieur contient une exigence coupée

Avec Compagnon 0.6.1 et les versions antérieures — les compilations faites avant
ce correctif —, archiver un change qui modifiait la dernière exigence d'une
spec et ajoutait des exigences pouvait écrire les ajouts au milieu d'une ligne
de l'exigence modifiée : la spec contient alors le début de cette exigence,
les ajouts, puis sa fin collée derrière eux. `compagnon validate` ne le détecte
pas, et synchroniser ou archiver à nouveau le change ne le répare pas. Pour
vérifier, regardez le diff de chaque archivage d'un change dont le delta
modifiait la dernière exigence d'une spec et ajoutait des exigences ; pour
réparer, remettez l'exigence entière à la main, ou par un nouveau change dont
le delta `MODIFIED` en donne le texte complet.

### La synchronisation ou l'archivage refuse un change

```text
error: change `add-dark-mode` has errors; run `compagnon validate add-dark-mode` for details
```

Avec `--json`, le code d'erreur est `validation_failed`. Lancez la commande
`compagnon validate` suggérée, corrigez les constats, puis synchronisez ou
archivez à nouveau. Si l'archivage est refusé parce qu'une exigence
`MODIFIED` n'existe pas dans la spec principale, c'est que son nom diffère de
celui de la spec principale : corrigez le nom, ou utilisez `ADDED` si
l'exigence est nouvelle.

### `compagnon validate` signale `rename_source_missing`

Le nom qui suit `FROM:` dans `## RENAMED Requirements` ne correspond à aucun
titre `### Requirement:` de la spec principale. Il doit correspondre
exactement, que vous écriviez le nom seul ou le titre complet entre accents
graves :

```markdown
- FROM: `### Requirement: Le thème choisi est mémorisé`
- TO: `### Requirement: Le thème choisi suit l'utilisateur`
```

### `compagnon validate` signale `delta_unexpected_heading`

Un titre `###` d'une section de delta n'est pas un titre d'exigence — par
exemple `### Notes`, ou un mot-clé dans une langue que Compagnon ne connaît pas.
Compagnon lit `### Requirement:` et son équivalent français `### Exigence :` ;
voir [Langue des artefacts](guides/artifact-language.md).

### `/compagnon-propose` ignore mon ticket

1. `mcp.jira_tool` n'était pas défini dans `_compagnon/config.yaml` lors de la
   génération des skills : `.claude/skills/compagnon-propose/SKILL.md` affiche
   alors « (Jira MCP not configured) » à la place du nom de l'outil. Ajoutez la
   clé, puis lancez `compagnon update --force`.
2. L'outil est configuré mais indisponible dans la session Claude Code en
   cours — nom erroné, serveur non connecté, session expirée. La skill vous le
   signale et rédige la proposal à partir de votre seule demande.

Voir [Jira et autres serveurs MCP](guides/mcp.md).

### `compagnon init` signale `inherit_pin_failed`

Une source git passée avec `--inherit` a été déclarée mais n'a pas pu être
épinglée : `git` est absent, le réseau ou vos identifiants ont échoué, ou la
`ref` n'existe pas (`main` par défaut ; un dépôt dont la branche par défaut
est `master` demande `…/repo.git@master`). Corrigez la cause — ou la `ref:`
dans `_compagnon/config.yaml` —, puis lancez `compagnon sources update`.

### `compagnon init` signale `managed_section_malformed`

`CLAUDE.md` ou le modèle de pull request contient une ligne
`<!-- compagnon:begin` sans `<!-- compagnon:end -->` correspondant, ou plusieurs.
Compagnon n'a pas touché au fichier. Gardez exactement une paire de marqueurs, ou
supprimez-les tous pour que `compagnon init` ajoute une section neuve.

### Une source git est `unlocked` ou `needs_update`

Lancez `compagnon sources update`. `unlocked` signifie que la source n'a jamais
été résolue ; `needs_update` signifie qu'elle est épinglée dans `compagnon.lock`
mais absente de votre cache local, typiquement sur une machine neuve.

### Le Compagnon d'un collègue échoue sur la configuration

Son Compagnon est plus ancien que votre `_compagnon/config.yaml` : le fichier contient
une clé que sa version ne connaît pas.

- **Compagnon 0.5 et les versions antérieures** rejettent toute clé inconnue :
  chaque commande échoue avec `invalid` en nommant la clé. Il doit mettre à
  jour : `compagnon self-update` depuis une version récente, sinon l'installateur
  décrit dans [Installation](installation.md).
- **Compagnon 0.6.1 et les versions antérieures** rejettent aussi une clé
  inconnue dans la table d'un rôle sous `roles:`, dans le `change.yaml` d'un
  change, dans un `schema.yaml` de projet et dans le frontmatter d'un ADR.
- **Les versions plus récentes** émettent à la place un avertissement —
  `unknown_config_key`, `change_unknown_key`, `schema_unknown_key` ou
  `decision_unknown_field`, selon le fichier — et continuent de
  fonctionner ; l'avertissement cite votre `compagnon_version` et renvoie à
  `compagnon self-update`. Une clé inconnue dans une entrée de `inherits` ou dans
  `changelog` échoue toujours avec `invalid`, en citant `compagnon_version`.

Pour que le message soit explicite pour tous, relevez `compagnon_version` chaque
fois que vous ajoutez une clé que seules les versions récentes connaissent.
Voir [Clés inconnues](reference/configuration.md#clés-inconnues).

### `unknown_config_key`

`_compagnon/config.yaml` contient une clé que ce Compagnon ne connaît pas : elle est
ignorée et le reste de la configuration s'applique. Soit la clé vient d'une
version plus récente de Compagnon (lancez `compagnon self-update`, ou la version que
`compagnon doctor` indique dans sa vérification `compagnon_version`), soit c'est une
faute de frappe, comme `shema` pour `schema` : corrigez-la.
`compagnon validate --strict` échoue d'ici là. Lorsque `compagnon doctor` propose
`compagnon self-update --version <X.Y.Z>`, votre Compagnon est plus récent que ce
qu'autorise le `compagnon_version` du projet, et cette commande installe une
version qui le satisfait.

### `change_unknown_key`

Le `change.yaml` d'un change contient une clé que ce Compagnon ne connaît pas, au
premier niveau ou sous `review:` (`review.<clé>`) : elle est ignorée et le
reste du fichier s'applique. Le change a été écrit avec une version plus
récente de Compagnon — lancez `compagnon self-update` — ou la clé est mal
orthographiée, comme `skip_spec` pour `skip_specs` : corrigez-la, sinon le
marqueur qu'elle désigne ne s'applique pas. `compagnon validate --strict` échoue
d'ici là.

### `schema_unknown_key`

Un schéma de projet, `_compagnon/schemas/<nom>/schema.yaml`, contient une clé que
ce Compagnon ne connaît pas : l'avertissement nomme la clé et sa ligne, la clé
est ignorée et le schéma reste utilisable. Lancez `compagnon self-update` si le
schéma a été écrit pour une version plus récente de Compagnon, sinon corrigez la
clé — un `require` mal orthographié laisse l'artefact sans sa dépendance.
`compagnon validate --strict` la signale sur l'élément `config`.

### `decision_unknown_field`

Le frontmatter d'un ADR contient un champ que ce Compagnon ne connaît pas : la
décision est lue à partir des champs qu'il connaît, et
`compagnon decision accept` conserve le champ lorsqu'il réécrit le frontmatter.
Lancez `compagnon self-update` si une version plus récente de Compagnon l'a ajouté,
sinon corrigez-le ou supprimez-le. Les champs d'une décision héritée ne sont
pas signalés : vous ne pouvez pas modifier sa source.

### `compagnon validate`, `sync` ou `archive` signale `compagnon_version_unsatisfied`

Le Compagnon en cours ne satisfait pas le `compagnon_version` du projet, et les
commandes dont la réussite est un verdict sur le projet refusent d'en donner
un avec lui : un Compagnon plus ancien pourrait accepter ce que celui d'un
collègue refuse. Appliquez la correction que nomme le message —
`compagnon self-update`, ou `compagnon self-update --version <X.Y.Z>` lorsque votre
Compagnon dépasse une borne supérieure comme `^0.6` (ou élargissez
`compagnon_version` si l'équipe passe à la version plus récente). En CI,
relevez l'épinglage `COMPAGNON_VERSION=` chaque fois que `compagnon_version` est
relevé. Toutes les autres commandes, `compagnon status` et `compagnon doctor`
compris, continuent de fonctionner.

### `compagnon self-update` signale `update_checksum_mismatch`

L'archive téléchargée ne correspond pas à sa ligne dans le `SHA256SUMS` de la
release, ou cette ligne manque. Rien n'a été installé. Réessayez plus tard —
un proxy a pu altérer le téléchargement — et, si le problème persiste,
signalez-le : c'est la vérification qui vous protège d'une archive
falsifiée.

### `compagnon self-update` signale `update_not_writable`

Compagnon ne peut pas écrire dans le dossier de son binaire, par exemple
`/usr/local/bin` appartenant à root. Réinstallez Compagnon dans un dossier qui
vous appartient (`install.sh` utilise `~/.local/bin`), ou lancez la commande
avec les droits qu'exige ce dossier.

### `compagnon self-update` indique que Compagnon est géré ailleurs

Votre binaire a été installé par `cargo install`, compilé depuis un clone ou
installé par un gestionnaire de paquets : Compagnon laisse cet outil s'en charger
et affiche la commande à lancer, par exemple
`cargo install --path crates/compagnon-cli` depuis un clone à jour.

### `compagnon: command not found`

Le dossier d'installation n'est pas dans votre `PATH`. Voir
[Vérifier l'installation](installation.md#vérifier-linstallation).
