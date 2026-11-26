# FAQ et dépannage

## Questions générales

### Compagnon envoie-t-il mon code à un modèle ?

Non. Le programme `compagnon` ne parle jamais à un modèle d'IA. Il ne va sur
internet que pour deux choses :

- télécharger des règles partagées depuis un autre
  [dépôt](glossary.md#dépôt), quand vous le demandez (`compagnon sources update`) ;
- voir si une version plus récente de Compagnon existe (`compagnon status`,
  `compagnon self-update`).

C'est votre [agent](glossary.md#agent), Claude Code, qui rédige. Il peut
lire et faire exactement ce qu'il peut faire d'habitude sur votre ordinateur.

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
le numéro de la dernière release et le garde en cache dans
`~/.cache/compagnon/update-check.json`, pendant 24 heures. Elle est sautée
dès que la variable d'environnement `CI` est définie.

</details>

### Puis-je utiliser Compagnon sans Claude Code ?

Oui, en partie. La commande `compagnon` fonctionne seule. Vous pouvez créer
une modification et rédiger vous-même les documents de son plan (les
[artefacts](glossary.md#artefact)), en suivant ce que vous indique
`compagnon instructions`. Vous pouvez ensuite vérifier, synchroniser et
archiver la modification. Ce que vous perdez, ce sont les compétences
(skills) : elles rédigent à votre place, et ne fonctionnent que dans Claude
Code.

### Faut-il enregistrer `_compagnon/` et `.claude/skills/` dans git ?

Oui, les deux. Un [enregistrement](glossary.md#enregistrement) (commit)
garde une version des fichiers du projet dans son historique, tenu par
[git](glossary.md#git).

- `_compagnon/` est la mémoire de vos spécifications, de vos décisions et de
  vos modifications. Enregistrez-le en entier, y compris
  `_compagnon/decisions/seal.yaml` et `_compagnon/compagnon.lock`.
- Quand `.claude/skills/` est enregistré, toute l'équipe a les mêmes
  compétences, sans avoir à lancer `compagnon init`.

### Une petite correction exige-t-elle une modification complète ?

Chaque modification suit les mêmes étapes, mais son plan peut être très
court. Par exemple : une proposition d'une ligne, une seule exigence dans
son [delta](glossary.md#delta) (la partie de la spécification qu'elle ajoute
ou change), et une liste de deux tâches. Si la modification ne change pas le
comportement du logiciel, définissez `skip_specs: true` dans son
`change.yaml` : elle n'a alors besoin d'aucune spécification.

### Puis-je tenir Compagnon à l'écart de `CLAUDE.md`, de la CI ou du modèle de demande de fusion ?

Oui. Lors de la mise en place du projet, dites à `compagnon init` ce qu'il
doit laisser de côté :

- `--no-claude-md` laisse tranquille `CLAUDE.md` (le fichier de consignes
  que Claude lit dans chaque projet) ;
- `--no-ci` n'ajoute aucune vérification automatique (CI) ;
- `--no-pr-template` laisse tranquille le modèle de
  [demande de fusion](glossary.md#demande-de-fusion) (pull request) ;
- `--no-integration` fait les trois.

`compagnon update` ne les ajoute jamais par la suite.

Vous pouvez aussi les reprendre en main après coup :

- Pour garder la vérification automatique tout en gérant vous-même son
  fichier, supprimez sa première ligne, `# compagnon:managed`. Compagnon
  laisse alors le fichier tranquille.
- Pour retirer le texte de Compagnon de `CLAUDE.md` ou du modèle, supprimez
  tout ce qui se trouve entre les marqueurs `compagnon:begin` et
  `compagnon:end`.

Voir [Intégration au dépôt et compagnon doctor](guides/repository-integration.md).

### Mes relecteurs doivent-ils travailler isolément ?

Par défaut, la compétence qui a écrit le plan ou le code le relit aussi.
Elle joue chaque [rôle](glossary.md#rôle) de relecteur tour à tour, dans la
même conversation. C'est le mode en ligne (`inline`).

En mode isolé (`isolated`), chaque rôle de relecteur travaille seul, comme
un assistant séparé (un [sous-agent](glossary.md#sous-agent) de Claude Code) qui ne voit jamais votre
conversation. La compétence rassemble ensuite ce qu'ils ont trouvé.

La relecture isolée coûte plus cher, car chaque relecteur relit tout. Son
bénéfice n'est pas encore mesuré. Commencez donc petit : utilisez-la pour
les modifications où un regard neuf compte le plus. Activez-la pour tout le
projet une fois la différence constatée. Voir
[Relecture en ligne ou isolée](roles.md#relecture-en-ligne-ou-isolée).

<details>
<summary>Sous le capot : activer la relecture isolée</summary>

Pour une modification, ajoutez `review: { mode: isolated }` à son
`change.yaml`. Pour tout le projet, ajoutez-le à `_compagnon/config.yaml`,
puis lancez `compagnon update` : les compétences sont régénérées pour le
nouveau mode.

</details>

### Comment partager cette documentation ?

Enregistrez-la sous la forme d'une seule page web :

```bash
compagnon docs --lang fr --write ~/Downloads/compagnon-manual-fr.html
```

Le fichier contient tout ce dont il a besoin : il ne charge rien depuis
internet. Vous pouvez l'envoyer par e-mail ou le déposer dans un wiki.

## Dépannage

### `no compagnon project found`

```text
error: no compagnon project found from /home/you — run `compagnon init` at the project root
```

Compagnon cherche un dossier `_compagnon/` dans le dossier courant, puis dans
chaque dossier au-dessus. Il n'en a trouvé aucun. Lancez la commande depuis
votre projet, ou mettez le projet en place avec `compagnon init`.

### Les compétences n'apparaissent pas dans Claude Code

Claude Code ne cherche les compétences (skills) qu'au démarrage d'une
session. Après `compagnon init` ou `compagnon update`, redémarrez Claude
Code ou ouvrez une nouvelle session. Vérifiez aussi que les fichiers
`.claude/skills/compagnon-*/SKILL.md` existent à la racine du projet que
vous avez ouvert.

### `compagnon update` laisse une compétence en place

```text
  Left in place because edited by hand — rerun with --force to overwrite:
```

Quelqu'un a modifié cette compétence à la main, et Compagnon ne jette pas
votre travail. Vous avez deux choix :

- garder votre retouche, et ignorer le message ;
- ou la remplacer par une copie neuve : lancez `compagnon update --force`.
  Vérifiez d'abord ce que vous perdriez avec `git diff`.

<details>
<summary>Sous le capot : comment Compagnon repère une retouche à la main</summary>

Compagnon garde une empreinte de chaque compétence qu'il écrit. Une
compétence dont le contenu ne correspond plus à son empreinte est considérée
comme modifiée à la main. Une compétence générée avant les empreintes est
préservée de la même façon, jusqu'à ce qu'un `compagnon update --force` lui
en donne une.

</details>

### Le sous-agent d'un relecteur échoue

Cela n'arrive qu'en relecture isolée. Quand l'assistant d'un relecteur
(sous-agent) échoue ou est arrêté, Compagnon le relance une fois. S'il
échoue encore, ce rôle relit dans la conversation principale, et le compte
rendu de relecture dit pourquoi, par une ligne `Fallback:` sous le rôle.
Rien n'est perdu : la relecture est faite, simplement pas isolément pour ce
rôle.

Si tous les rôles se rabattent sur la conversation principale, vérifiez
deux choses :

- que Claude Code peut lancer des sous-agents (l'outil `Agent`) ;
- que `compagnon doctor` indique des compétences à jour pour le mode de
  relecture. S'il signale `skills_review_mode`, le mode a changé mais
  `compagnon update` n'a pas été lancé depuis.

### `compagnon validate` signale `zero_delta_without_marker`

La modification ne dit pas en quoi le comportement du logiciel change : son
dossier `specs/` ne contient aucun delta. Faites l'un ou l'autre :

- rédigez le delta : le comportement que la modification ajoute ou
  modifie ;
- ou, si la modification ne change vraiment pas le comportement, ajoutez
  `skip_specs: true` à son `change.yaml`.

### `compagnon validate` signale `decision_seal_mismatch`

Quelqu'un a modifié le texte d'une décision acceptée, à la main ou avec un
outil de mise en forme. Les décisions acceptées sont
[scellées](glossary.md#sceau) : elles ne doivent pas changer. Vous pouvez :

- **restaurer** le texte d'origine (`git diff` montre ce qui a changé) ; le
  sceau redevient valide ;
- ou, si la retouche était volontaire, **la sceller à nouveau** :
  `compagnon decision seal <ID> --force`.

Pour changer ce que dit une décision, la bonne façon est de la remplacer
par une nouvelle : `compagnon decision supersede <ID> "<new title>"`,
rédigez la nouvelle décision, puis acceptez-la.

Pour tenir les outils de mise en forme à l'écart des décisions, excluez
`_compagnon/decisions/` dans leurs réglages.

<details>
<summary>Sous le capot : quand une décision est scellée</summary>

Une décision est scellée quand elle est acceptée. Pour rédiger une décision
avant de la sceller, laissez-la `proposed` — comme la créent
`compagnon decision new`, `supersede`, `deviate` et `promote` — et lancez
`compagnon decision accept <ID>` une fois son texte définitif. Une décision
créée avec `compagnon decision new --status accepted` est scellée
immédiatement, avec le texte d'exemple du modèle.

</details>

### `compagnon decision accept` signale `predecessor_not_accepted`

La décision que vous acceptez dit en remplacer une autre (dans
`supersedes`). Mais cette autre décision n'est plus `accepted` : le plus
souvent, une troisième décision l'a remplacée après la création de la
vôtre. Rien n'a été écrit.

Lancez `compagnon decision list` pour voir quelle décision est en vigueur
aujourd'hui. Ensuite, soit faites pointer `supersedes` vers elle, dans
l'en-tête ([frontmatter](glossary.md#frontmatter)) de la vôtre, soit
abandonnez la vôtre si elle ne sert plus. `compagnon decision supersede`
refuse avec le même code une décision qui n'est pas `accepted`.

### `compagnon validate` signale `decision_missing_frontmatter`

Compagnon lit chaque fichier `.md` de `_compagnon/decisions/` comme une
décision. Sortez de ce dossier les autres fichiers Markdown, un README par
exemple.

### Une spécification archivée avec un Compagnon antérieur contient une exigence coupée

C'est un défaut de Compagnon 0.6.1 et des versions antérieures (les
copies installées avant le correctif). Il se produisait en archivant une
modification qui faisait deux choses à la fois : elle modifiait la dernière
exigence d'une spécification, et elle ajoutait de nouvelles exigences.

L'exigence modifiée pouvait alors être coupée en deux, au milieu d'une
ligne, avec les exigences ajoutées collées au milieu.

`compagnon validate` ne le détecte pas, et synchroniser ou archiver à
nouveau la modification ne le répare pas.

- **Pour vérifier :** regardez ce que chacun de ces archivages a changé
  dans la spécification (`git diff`).
- **Pour réparer :** remettez l'exigence entière, à la main, ou par une
  nouvelle modification dont le delta `MODIFIED` en donne le texte complet.

### La synchronisation ou l'archivage refuse une modification

```text
error: change `add-dark-mode` has errors; run `compagnon validate add-dark-mode` for details
```

La modification contient des erreurs : Compagnon refuse donc de la fusionner
dans les spécifications du projet. Lancez la commande `compagnon validate`
suggérée, corrigez ce qu'elle signale, puis synchronisez ou archivez à
nouveau. Avec `--json`, le code d'erreur est `validation_failed`.

Un cas fréquent : l'archivage est refusé parce qu'une exigence `MODIFIED`
n'existe pas dans la spécification du projet. Son nom diffère de celui de la
spécification du projet. Corrigez le nom, ou utilisez `ADDED` si l'exigence
est nouvelle.

### `compagnon validate` signale `rename_source_missing`

Vous renommez une exigence, mais l'ancien nom (après `FROM:` dans
`## RENAMED Requirements`) ne correspond à aucun titre `### Requirement:` de
la spécification du projet. Il doit correspondre exactement. Vous pouvez
écrire le nom seul, ou le titre complet entre accents graves :

```markdown
- FROM: `### Requirement: Le thème choisi est mémorisé`
- TO: `### Requirement: Le thème choisi suit l'utilisateur`
```

### `compagnon validate` signale `delta_unexpected_heading`

Un titre `###` d'un delta n'est pas un titre d'exigence. Par exemple
`### Notes`, ou un mot-clé dans une langue que Compagnon ne connaît pas.
Compagnon lit `### Requirement:` et son équivalent français
`### Exigence :` ; voir [Langue des artefacts](guides/artifact-language.md).

### `/compagnon-propose` ignore mon ticket

Il y a deux causes probables.

1. L'outil Jira n'était pas défini lors de la génération des compétences.
   Dans `.claude/skills/compagnon-propose/SKILL.md`, vous lisez alors
   « (Jira MCP not configured) » à la place du nom de l'outil. Ajoutez la
   clé `mcp.jira_tool` à `_compagnon/config.yaml`, puis lancez
   `compagnon update --force`.
2. L'outil est défini, mais indisponible dans la session Claude Code en
   cours : nom erroné, serveur non connecté ou connexion expirée. La
   compétence vous le signale, et rédige la proposition à partir de votre
   seule demande.

Voir [Jira et autres serveurs MCP](guides/mcp.md).

### `compagnon init` signale `inherit_pin_failed`

Vous avez demandé à hériter des règles d'un autre dépôt git (avec
`--inherit`). Compagnon l'a noté, mais n'a pas pu le fixer sur une version
précise. La cause est l'une de celles-ci :

- `git` n'est pas installé ;
- le réseau ou vos identifiants ont échoué ;
- la branche ou l'étiquette (`ref`) n'existe pas. La valeur par défaut est
  `main` : un dépôt dont la branche par défaut est `master` demande
  `…/repo.git@master`.

Corrigez la cause, ou la `ref:` dans `_compagnon/config.yaml`, puis lancez
`compagnon sources update`.

### `compagnon init` signale `managed_section_malformed`

`CLAUDE.md` ou le modèle de demande de fusion contient une ligne
`<!-- compagnon:begin` sans `<!-- compagnon:end -->` correspondant, ou en
contient plusieurs. Compagnon n'a pas touché au fichier. Gardez exactement
une paire de marqueurs, ou supprimez-les tous pour que `compagnon init`
ajoute une section neuve.

### Une source git est `unlocked` ou `needs_update`

Lancez `compagnon sources update`.

- `unlocked` signifie que la source n'a jamais été récupérée.
- `needs_update` signifie qu'elle est notée dans `compagnon.lock`, mais
  absente de votre ordinateur. C'est typique sur une machine neuve.

### Le Compagnon d'un collègue échoue sur la configuration

Le Compagnon de votre collègue est plus ancien que votre
`_compagnon/config.yaml` : le fichier contient un réglage que sa version ne
connaît pas.

- **Compagnon 0.5 et les versions antérieures** rejettent toute clé
  inconnue : chaque commande échoue avec `invalid`, en nommant la clé. Votre
  collègue doit mettre à jour : avec `compagnon self-update` depuis une
  version récente, sinon avec l'installateur décrit dans
  [Installation](installation.md).
- **Compagnon 0.6.1 et les versions antérieures** rejettent aussi une clé
  inconnue à quelques autres endroits : dans un rôle sous `roles:`, dans le
  `change.yaml` d'une modification, dans un `schema.yaml` de projet et dans
  l'en-tête d'un fichier de décision.
- **Les versions plus récentes** affichent un avertissement à la place, et
  continuent de fonctionner. L'avertissement cite votre `compagnon_version`
  et renvoie à `compagnon self-update`.

Pour que le message soit clair pour tous, relevez `compagnon_version` chaque
fois que vous ajoutez une clé que seules les versions récentes connaissent.
Voir [Clés inconnues](reference/configuration.md#clés-inconnues).

<details>
<summary>Sous le capot : les codes d'avertissement des clés inconnues</summary>

Selon le fichier, l'avertissement est `unknown_config_key`,
`change_unknown_key`, `schema_unknown_key` ou `decision_unknown_field`. Une
clé inconnue dans une entrée de `inherits` ou dans `changelog` échoue
toujours avec `invalid`, en citant `compagnon_version`.

</details>

### `unknown_config_key`

`_compagnon/config.yaml` contient une clé que ce Compagnon ne connaît pas.
Cette clé est ignorée ; le reste de la configuration s'applique. Il y a deux
raisons possibles :

- la clé vient d'une version plus récente de Compagnon : lancez
  `compagnon self-update`, ou installez la version que `compagnon doctor`
  indique dans sa vérification `compagnon_version` ;
- c'est une faute de frappe, comme `shema` pour `schema` : corrigez-la.

D'ici là, `compagnon validate --strict` échoue.

Lorsque `compagnon doctor` propose `compagnon self-update --version <X.Y.Z>`,
votre Compagnon est plus récent que ce qu'autorise le `compagnon_version` du
projet. Cette commande installe une version qui convient.

### `change_unknown_key`

Le `change.yaml` d'une modification contient une clé que ce Compagnon ne
connaît pas, au premier niveau ou sous `review:` (`review.<clé>`). Cette clé
est ignorée ; le reste du fichier s'applique. Soit :

- la modification a été écrite avec une version plus récente de Compagnon :
  lancez `compagnon self-update` ;
- soit la clé est mal orthographiée, comme `skip_spec` pour `skip_specs` :
  corrigez-la, sinon le réglage qu'elle désigne ne s'applique pas.

D'ici là, `compagnon validate --strict` échoue.

### `schema_unknown_key`

Un [schéma](glossary.md#schéma) de votre projet,
`_compagnon/schemas/<nom>/schema.yaml`, contient une clé que ce Compagnon ne
connaît pas. L'avertissement nomme la clé et sa ligne. La clé est ignorée,
et le schéma reste utilisable.

- Si le schéma a été écrit pour une version plus récente de Compagnon,
  lancez `compagnon self-update`.
- Sinon, corrigez la clé. Par exemple, un `require` mal orthographié laisse
  un artefact sans le document dont il dépend.

`compagnon validate --strict` la signale sur l'élément `config`.

### `decision_unknown_field`

L'en-tête d'un fichier de décision (un ADR) contient un champ que ce
Compagnon ne connaît pas. La décision est lue à partir des champs qu'il
connaît, et `compagnon decision accept` conserve le champ inconnu lorsqu'il
réécrit l'en-tête. Lancez `compagnon self-update` si une version plus
récente de Compagnon l'a ajouté ; sinon, corrigez-le ou supprimez-le. Les
champs d'une décision héritée d'un autre dépôt ne sont pas signalés : vous
ne pouvez pas modifier sa source.

### `compagnon validate`, `sync` ou `archive` signale `compagnon_version_unsatisfied`

Votre Compagnon n'est pas une version qu'autorise le projet (son réglage
`compagnon_version`). Certaines commandes rendent un verdict sur le projet :
elles l'acceptent ou le refusent. Ces commandes refusent de s'exécuter avec
une version que le projet n'autorise pas. Sinon, un Compagnon plus ancien
pourrait accepter ce que celui d'un collègue refuserait.

Appliquez la correction que nomme le message :

- `compagnon self-update` ;
- ou `compagnon self-update --version <X.Y.Z>`, lorsque votre Compagnon est
  plus récent que la version la plus haute qu'autorise le projet. Vous
  pouvez aussi élargir `compagnon_version`, si l'équipe passe à la version
  plus récente.

Dans les vérifications automatiques du projet (CI), relevez la version de
Compagnon qu'elles installent chaque fois que `compagnon_version` est
relevé. Toutes les autres commandes, `compagnon status` et
`compagnon doctor` compris, continuent de fonctionner.

<details>
<summary>Sous le capot : limites hautes et épinglage en CI</summary>

Une limite haute est un `compagnon_version` comme `^0.6`, qui autorise les
0.6.x mais pas la 0.7. En CI, la version installée est épinglée par
`COMPAGNON_VERSION=` : relevez cet épinglage chaque fois que
`compagnon_version` est relevé.

</details>

### `compagnon self-update` signale `update_checksum_mismatch`

Le fichier téléchargé ne correspond pas à l'empreinte publiée avec la
version (sa ligne dans `SHA256SUMS`), ou cette ligne manque. Rien n'a été
installé. Réessayez plus tard : votre réseau a pu altérer le
téléchargement. Si le problème persiste, signalez-le. Cette
vérification est ce qui vous protège d'un fichier falsifié.

### `compagnon self-update` signale `update_not_writable`

Compagnon ne peut pas écrire dans le dossier où il est installé, par
exemple `/usr/local/bin`, qui appartient à l'administrateur (root).
Réinstallez Compagnon dans un dossier qui vous appartient (`install.sh`
utilise `~/.local/bin`), ou lancez la commande avec les droits qu'exige ce
dossier.

### `compagnon self-update` indique que Compagnon est géré ailleurs

Votre copie de Compagnon a été installée autrement : avec `cargo install`,
compilée depuis un clone de son code, ou par un gestionnaire de paquets.
Compagnon laisse cet outil gérer les mises à jour, et affiche la commande à
lancer, par exemple `cargo install --path crates/compagnon-cli` depuis un
clone à jour.

### `compagnon: command not found`

Votre [terminal](glossary.md#terminal) ne sait pas où Compagnon est installé : son dossier n'est pas
dans votre `PATH`. Voir
[Vérifier l'installation](installation.md#vérifier-linstallation).
