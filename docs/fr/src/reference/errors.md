# Codes d'erreur et solutions

Quand `compagnon validate`, `compagnon doctor` ou une autre commande signale
un problème, elle le nomme par un code stable. Cherchez le code ci-dessous :
chaque entrée dit ce qu'il signifie et comment le corriger.

La liste complète des codes que peut renvoyer `--json` figure dans
[Sortie JSON](json-output.md) ; les vérifications de `compagnon validate`
figurent dans [Formats de fichiers](file-formats.md).

## Installer et trouver le projet

### `compagnon: command not found`

Votre [terminal](../glossary.md#terminal) ne sait pas où Compagnon est
installé : son dossier n'est pas dans votre `PATH`. Voir
[Vérifier l'installation](../installation.md#vérifier-linstallation).

### `no compagnon project found`

```text
error: no compagnon project found from /home/you — run `compagnon init` at the project root
```

Compagnon cherche un dossier `_compagnon/` dans le dossier courant, puis dans
chaque dossier au-dessus. Il n'en a trouvé aucun. Lancez la commande depuis
votre projet, ou mettez le projet en place avec `compagnon init`. Avec
`--json`, le code est `no_compagnon_root`.

## Modifications et deltas

### `zero_delta_without_marker`

`compagnon validate` signale que la modification ne dit pas en quoi le
comportement du logiciel change : son dossier `specs/` ne contient aucun
[delta](../glossary.md#delta). Faites l'un ou l'autre :

- rédigez le delta : le comportement que la modification ajoute ou
  modifie ;
- ou, si la modification ne change vraiment pas le comportement, ajoutez
  `skip_specs: true` à son `change.yaml`.

### `validation_failed`

```text
error: change `add-dark-mode` has errors; run `compagnon validate add-dark-mode` for details
```

La synchronisation ou l'archivage a refusé la modification : elle contient
des erreurs, et Compagnon refuse donc de la fusionner dans les
spécifications du projet. Lancez la commande `compagnon validate`
suggérée, corrigez ce qu'elle signale, puis synchronisez ou archivez à
nouveau.

Un cas fréquent : l'archivage est refusé parce qu'une exigence `MODIFIED`
n'existe pas dans la spécification du projet. Son nom diffère de celui de la
spécification du projet. Corrigez le nom, ou utilisez `ADDED` si l'exigence
est nouvelle.

### `rename_source_missing`

`compagnon validate` signale que vous renommez une exigence, mais que
l'ancien nom (après `FROM:` dans `## RENAMED Requirements`) ne correspond à
aucun titre `### Requirement:` de la spécification du projet. Il doit
correspondre exactement. Vous pouvez écrire le nom seul, ou le titre complet
entre accents graves :

```markdown
- FROM: `### Requirement: Le thème choisi est mémorisé`
- TO: `### Requirement: Le thème choisi suit l'utilisateur`
```

### `delta_unexpected_heading`

Un titre `###` d'un delta n'est pas un titre d'exigence. Par exemple
`### Notes`, ou un mot-clé dans une langue que Compagnon ne connaît pas.
Compagnon lit `### Requirement:` et son équivalent français
`### Exigence :` ; voir [Langue des artefacts](../guides/artifact-language.md).

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

## Décisions

### `decision_seal_mismatch`

`compagnon validate` signale que quelqu'un a modifié le texte d'une
décision acceptée, à la main ou avec un outil de mise en forme. Les
décisions acceptées sont [scellées](../glossary.md#sceau) : elles ne
doivent pas changer. Vous pouvez :

- **restaurer** le texte d'origine (`git diff` montre ce qui a changé) ; le
  sceau redevient valide ;
- ou, si la retouche était volontaire, **la sceller à nouveau** :
  `compagnon decision seal <ID> --force`.

Pour changer ce que dit une décision, la bonne façon est de la remplacer
par une nouvelle : `compagnon decision supersede <ID> "<new title>"`,
rédigez la nouvelle décision, puis acceptez-la. Voir
[Décisions](../guides/decisions.md#remplacée).

Pour tenir les outils de mise en forme à l'écart des décisions, excluez
`_compagnon/decisions/` dans leurs réglages.

### `decision_unsealed`

Un avertissement de `compagnon validate` : une décision locale acceptée ou
remplacée n'a pas de sceau, le plus souvent parce qu'elle a été acceptée à
la main, en modifiant son statut. Une fois son texte définitif, lancez
`compagnon decision seal <ID>` pour l'enregistrer. Voir
[Acceptée et scellée](../guides/decisions.md#acceptée-et-scellée).

### `predecessor_not_accepted`

`compagnon decision accept` signale que la décision que vous acceptez dit
en remplacer une autre (dans `supersedes`), mais que cette autre décision
n'est plus `accepted` : le plus souvent, une troisième décision l'a
remplacée après la création de la vôtre. Rien n'a été écrit.

Lancez `compagnon decision list` pour voir quelle décision est en vigueur
aujourd'hui. Ensuite, soit faites pointer `supersedes` vers elle, dans
l'en-tête ([frontmatter](../glossary.md#frontmatter)) de la vôtre, soit
abandonnez la vôtre si elle ne sert plus. `compagnon decision supersede`
refuse avec le même code une décision qui n'est pas `accepted`.

### `decision_missing_frontmatter`

Compagnon lit chaque fichier `.md` de `_compagnon/decisions/` comme une
décision. Sortez de ce dossier les autres fichiers Markdown, un README par
exemple.

## Compétences et relecture

### Les compétences n'apparaissent pas dans Claude Code

Claude Code ne cherche les compétences qu'au démarrage d'une session. Après
`compagnon init` ou `compagnon update`, redémarrez Claude Code ou ouvrez
une nouvelle session. Vérifiez aussi que les fichiers
`.claude/skills/compagnon-*/SKILL.md` existent à la racine du projet que
vous avez ouvert.

### Une mise à jour laisse une compétence en place

```text
  Left in place because edited by hand — rerun with --force to overwrite:
```

`compagnon update` a trouvé une compétence modifiée à la main, et ne jette
pas ce travail. Vous avez deux choix :

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

### `skills_review_mode`

`compagnon doctor` signale que le mode de relecture a changé
(`review.mode`, `inline` ou `isolated`) mais que `compagnon update` n'a pas
été lancé depuis : les compétences suivent encore l'ancien mode. Lancez
`compagnon update`.

La même vérification aide quand le sous-agent d'un relecteur échoue. Cela
n'arrive qu'en relecture isolée : quand l'assistant d'un relecteur
([sous-agent](../glossary.md#sous-agent)) échoue ou est arrêté, Compagnon
le relance une fois. S'il échoue encore, ce rôle relit dans la
conversation principale, et le compte rendu de relecture dit pourquoi, par
une ligne `Fallback:` sous le rôle. Rien n'est perdu : la relecture est
faite, simplement pas isolément pour ce rôle. Si tous les rôles se
rabattent sur la conversation principale, vérifiez que Claude Code peut
lancer des sous-agents (l'outil `Agent`), et que `compagnon doctor` ne
signale pas `skills_review_mode`.

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

Voir [Partir d'un ticket Jira](../guides/mcp.md).

## Mise en place et sources héritées

### `inherit_pin_failed`

Un avertissement de `compagnon init` : vous avez demandé à hériter des
règles d'un autre dépôt git (avec `--inherit`). Compagnon l'a noté, mais
n'a pas pu le fixer sur une version précise. La cause est l'une de
celles-ci :

- `git` n'est pas installé ;
- le réseau ou vos identifiants ont échoué ;
- la branche ou l'étiquette (`ref`) n'existe pas. La valeur par défaut est
  `main` : un dépôt dont la branche par défaut est `master` demande
  `…/repo.git@master`.

Corrigez la cause, ou la `ref:` dans `_compagnon/config.yaml`, puis lancez
`compagnon sources update`.

### `managed_section_malformed`

Un avertissement de `compagnon init` : `CLAUDE.md` ou le modèle de demande
de fusion contient une ligne `<!-- compagnon:begin` sans
`<!-- compagnon:end -->` correspondant, ou en contient plusieurs. Compagnon
n'a pas touché au fichier. Gardez exactement une paire de marqueurs, ou
supprimez-les tous pour que `compagnon init` ajoute une section neuve.

### `unlocked`

Une source git n'a jamais été récupérée. Lancez `compagnon sources update`.
Voir [Sources héritées](../guides/inherited-sources.md).

### `needs_update`

Une source git est notée dans `compagnon.lock`, mais absente de votre
ordinateur. C'est typique sur une machine neuve. Lancez
`compagnon sources update`.

## Clés inconnues et versions

### `invalid`

Un fichier que Compagnon refuse de lire. Le cas le plus fréquent : le
Compagnon d'un collègue échoue sur votre configuration, parce qu'il est plus
ancien que votre `_compagnon/config.yaml` et que le fichier contient un
réglage que sa version ne connaît pas.

- **Compagnon 0.5 et les versions antérieures** rejettent toute clé
  inconnue : chaque commande échoue avec `invalid`, en nommant la clé. Votre
  collègue doit mettre à jour : avec `compagnon self-update` depuis une
  version récente, sinon avec l'installateur décrit dans
  [Installation](../installation.md).
- **Compagnon 0.6.1 et les versions antérieures** rejettent aussi une clé
  inconnue à quelques autres endroits : dans un rôle sous `roles:`, dans le
  `change.yaml` d'une modification, dans un `schema.yaml` de projet et dans
  l'en-tête d'un fichier de décision.
- **Les versions plus récentes** affichent un avertissement à la place, et
  continuent de fonctionner : l'un des quatre codes ci-dessous.
  L'avertissement cite votre `compagnon_version` et renvoie à
  `compagnon self-update`.

Pour que le message soit clair pour tous, relevez `compagnon_version` chaque
fois que vous ajoutez une clé que seules les versions récentes connaissent.
Voir [Clés inconnues](configuration.md#clés-inconnues). Une clé inconnue
dans une entrée de `inherits` ou dans `changelog` échoue toujours avec
`invalid`, en citant `compagnon_version`.

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

Un [schéma](../glossary.md#schéma) de votre projet,
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

### `compagnon_version_unsatisfied`

`compagnon validate`, `sync` ou `archive` a refusé de s'exécuter : votre
Compagnon n'est pas une version qu'autorise le projet (son réglage
`compagnon_version`). Ces commandes rendent un verdict sur le projet : elles
refusent donc de s'exécuter avec une version que le projet n'autorise pas.
Sinon, un Compagnon plus ancien pourrait accepter ce que celui d'un
collègue refuserait.

Appliquez la correction que nomme le message :

- `compagnon self-update` ;
- ou `compagnon self-update --version <X.Y.Z>`, lorsque votre Compagnon est
  plus récent que la version la plus haute qu'autorise le projet. Vous
  pouvez aussi élargir `compagnon_version`, si l'équipe passe à la version
  plus récente.

Dans les vérifications automatiques du projet
([intégration continue](../glossary.md#intégration-continue), CI), relevez
la version de Compagnon qu'elles installent chaque fois que
`compagnon_version` est relevé. Toutes les autres commandes,
`compagnon status` et `compagnon doctor` compris, continuent de fonctionner.

<details>
<summary>Sous le capot : limites hautes et épinglage en CI</summary>

Une limite haute est un `compagnon_version` comme `^0.6`, qui autorise les
0.6.x mais pas la 0.7. En CI, la version installée est épinglée par
`COMPAGNON_VERSION=` : relevez cet épinglage chaque fois que
`compagnon_version` est relevé.

</details>

## Mettre à jour Compagnon

### `update_checksum_mismatch`

`compagnon self-update` signale que le fichier téléchargé ne correspond pas
à l'empreinte publiée avec la version (sa ligne dans `SHA256SUMS`), ou que
cette ligne manque. Rien n'a été installé. Réessayez plus tard : votre
réseau a pu altérer le téléchargement. Si le problème persiste,
signalez-le. Cette vérification est ce qui vous protège d'un fichier
falsifié.

### `update_not_writable`

`compagnon self-update` ne peut pas écrire dans le dossier où Compagnon est
installé, par exemple `/usr/local/bin`, qui appartient à l'administrateur
(root). Réinstallez Compagnon dans un dossier qui vous appartient
(`install.sh` utilise `~/.local/bin`), ou lancez la commande avec les droits
qu'exige ce dossier.

### Compagnon est géré ailleurs

`compagnon self-update` indique que votre copie de Compagnon a été
installée autrement : avec `cargo install`, compilée depuis un clone de son
code, ou par un gestionnaire de paquets. Compagnon laisse cet outil gérer
les mises à jour, et affiche la commande à lancer, par exemple
`cargo install --path crates/compagnon-cli` depuis un clone à jour.
