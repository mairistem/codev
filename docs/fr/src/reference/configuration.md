# Configuration

Un projet codev se configure dans `_codev/config.yaml`. Toutes les clés sont
facultatives : un projet peut même fonctionner sans ce fichier. Une clé que
codev ne connaît pas est signalée, par un avertissement au premier niveau,
dans `mcp`, dans `review` et dans la table d'un rôle, par une erreur dans une
entrée de `inherits` et dans `changelog` : une faute de frappe ne passe donc
jamais inaperçue. Voir [Clés inconnues](#clés-inconnues).

`codev init` génère le fichier avec les valeurs qu'il a détectées, chacune
précédée d'un commentaire indiquant sa provenance. Vous pouvez le modifier à la
main à tout moment. Après avoir modifié `workflows`, `mcp` ou `review`,
lancez `codev update` pour que les skills reflètent la modification.

## Exemple complet

```yaml
schema: spec-driven

language: en

workflows:
  - propose
  - explore
  - onboard
  - apply
  - sync
  - archive
  - update
  - configure

context: |
  TypeScript monorepo: a Next.js web app and a Node.js API.
  Errors are typed; never throw raw strings.

rules:
  specs:
    - Describe observable behavior, never implementation.
  design:
    - Cite the decisions in _codev/decisions/ that constrain the approach.
  tasks:
    - "Each task states how to verify it: a named test, a command, or an observable behavior."

inherits:
  - path: ~/src/acme-standards
  - git: git@github.com:acme/codev-standards.git
    ref: main
    subpath: standards

mcp:
  jira_tool: mcp__atlassian__getJiraIssue

changelog:
  exclude:
    - 2026-03-02-legacy-import

codev_version: ">=0.5"

roles:
  newcomer: never
  skeptic:
    done_when:
      - Le plan de retour arrière est indiqué

triggers:
  - domain: personal-data
    paths: ["src/billing/**"]

review:
  mode: isolated
```

## schema

Le schéma de workflow utilisé par les nouveaux changes. Valeur par défaut :
`spec-driven`, le schéma embarqué dans codev. Le nom d'un schéma défini sous
`_codev/schemas/` est également accepté. Un change enregistre son schéma dans
son propre `change.yaml` : modifier cette clé n'affecte donc pas les changes
en cours. Voir [Schémas personnalisés](../guides/custom-schemas.md).

## language

La langue dans laquelle les skills rédigent les artefacts, sous la
forme d'un code ISO 639 : deux ou trois lettres minuscules, éventuellement
suivies d'une sous-étiquette (`en`, `fr`, `pt-BR`, `zh-Hant`). Valeur par
défaut : `en`. Il fixe la prose, et les mots-clés structurels quand codev
fournit la langue (anglais et français) ; une autre langue garde les
mots-clés anglais. codev lit les mots-clés en anglais comme en français,
quel que soit ce réglage. Voir
[Langue des artefacts](../guides/artifact-language.md).

## workflows

Les workflows à installer comme skills Claude Code, désignés par leur
identifiant : `propose`, `explore`, `onboard`, `configure`, `apply`, `sync`,
`archive`, `update`. En l'absence de la clé, les huit sont installés. Un
identifiant inconnu produit l'avertissement `unknown_workflow` et est ignoré.
Voir [Les skills Claude Code](../skills.md).

## context

Texte libre transmis à l'agent avec les instructions de **chaque** artefact :
votre stack, vos conventions, vos contraintes — tout ce qui ne peut pas se
déduire du code. C'est une contrainte imposée à l'agent, jamais un contenu à
recopier dans les fichiers. `/codev-configure` peut le rédiger pour vous.

## rules

Les règles propres à un artefact, sous la forme d'une table associant un
identifiant d'artefact à une liste de chaînes. Avec le schéma `spec-driven`,
les identifiants sont `proposal`, `specs`, `design` et `tasks`. Chaque règle
n'est transmise à l'agent que lorsqu'il rédige l'artefact correspondant.

Dans un scalaire simple, YAML interprète `key: value` comme une table :
mettez donc entre guillemets toute règle contenant un deux-points suivi d'une
espace, comme la règle `tasks` ci-dessus.

`/codev-propose` applique les règles une seconde fois lorsqu'elle relit le plan
terminé : `rules` est donc aussi le moyen d'ajouter un angle de relecture
propre à votre projet :

```yaml
rules:
  design:
    - "Évaluer l'impact sur l'accessibilité (RGAA) de toute modification de l'interface utilisateur."
```

Les règles de test se placent sous `tasks` : `/codev-propose` rédige le plan
en les respectant, et `/codev-apply` les suit lorsqu'elle écrit les tests.
Décrivez ce que font vos tests existants — `/codev-configure` en rédige la
règle à partir d'eux :

```yaml
rules:
  tasks:
    # Une règle pour votre pile, par exemple :
    # Rust
    - "Tests unitaires dans un #[cfg(test)] mod tests à côté du code, tests d'intégration dans tests/ ; cargo test, cargo clippy -- -D warnings, cargo fmt --check."
    # TypeScript
    - "Vitest, *.test.ts à côté du code, Testing Library pour les composants ; tsc --noEmit et le linter en CI."
    # Python
    - "pytest, tests/ reproduisant l'arborescence du paquet, fixtures et parametrize ; ruff check."
    # Go
    - "Tests pilotés par table dans des fichiers _test.go du même paquet ; go test ./..., go vet."
    # Java
    - "JUnit 5 sous src/test/java, à l'image du paquet principal, Mockito seulement aux frontières ; mvn verify."
```

Tant que `rules` est vide, `codev init` et `codev status` suggèrent de lancer
`/codev-configure`.

## inherits

Les sources en lecture seule dont hériter le contexte, les règles et les
décisions, dans l'ordre du plus général au plus spécifique. Chaque entrée est
soit un dossier local, soit un dépôt git :

| Clé | S'applique à | Signification |
|---|---|---|
| `path` | local | Dossier contenant un projet codev ; chemin absolu ou commençant par `~` |
| `git` | git | URL du dépôt, telle que `git` la comprend |
| `ref` | git | Branche ou tag à suivre — obligatoire |
| `subpath` | git | Dossier du dépôt qui contient le projet codev |

Une entrée déclare exactement une clé parmi `path` et `git` ; `ref` et
`subpath` sont rejetées sur une entrée `path`. Les sources git sont épinglées
dans `_codev/codev.lock` par `codev sources update`. Voir
[Sources héritées](../guides/inherited-sources.md).

`codev init --inherit <SOURCE>` écrit cette clé pour vous lorsqu'il crée la
configuration, une entrée par option, sous un commentaire
`# set with codev init --inherit`, et épingle aussitôt les sources git :

```bash
codev init --yes --inherit git@github.com:acme/codev-standards.git@v2 --inherit ~/src/acme-standards
```

```yaml
# set with codev init --inherit
inherits:
  - git: git@github.com:acme/codev-standards.git
    ref: v2
  - path: ~/src/acme-standards
```

## mcp

Les noms des outils MCP que les skills peuvent appeler. Ils sont propres à
chaque installation de Claude Code et ne sont donc jamais hérités.

| Clé | Signification |
|---|---|
| `jira_tool` | Outil qui récupère un ticket Jira, utilisé par `/codev-propose` lorsqu'un identifiant de ticket est mentionné |

Voir [Jira et autres serveurs MCP](../guides/mcp.md).

## changelog

Les réglages de [`codev changelog`](cli.md#codev-changelog). Propres au
projet, jamais hérités.

| Clé | Signification |
|---|---|
| `exclude` | Noms de dossiers d'archive (`<date>-<nom>`) des changes déjà décrits à la main dans `CHANGELOG.md`. `codev changelog` les laisse de côté, sans avertissement |

Utilisez-la pour adopter `codev changelog` dans un projet dont le
`CHANGELOG.md` décrit déjà des changes archivés sans note de version, afin
qu'ils ne soient pas listés deux fois. Voir
[Notes de version et CHANGELOG](../guides/release-notes.md).

## codev_version

Les versions de codev que l'équipe accepte, sous forme d'exigence de version
sémantique dans la syntaxe de Cargo : `">=0.5"`, `"^0.6"`, `">=0.5, <0.8"`.
Mettez la valeur entre guillemets, car YAML interprète `>` à sa façon. Propre
au projet, jamais héritée.

Lorsque le codev en cours ne la satisfait pas, `codev doctor` signale la
vérification `codev_version` comme manquante et les skills proposent de mettre
à jour avant d'agir. La correction est `codev self-update` quand le codev en
cours est trop ancien. Quand il est trop récent — au-delà d'une borne
supérieure comme `<0.7` — un simple `codev self-update` refuserait de
rétrograder : la correction nomme donc une version,
`codev self-update --version 0.6.4`, prise dans la dernière version publiée
que connaît le cache de mise à jour si elle satisfait l'exigence, sinon dans
l'exigence elle-même (`<=0.6.2` donne `0.6.2`, `^0.5` donne `0.5.0`). Une
valeur qui n'est pas une exigence fait échouer toutes les commandes avec
`invalid_codev_version`.

`codev validate`, `codev sync` et `codev archive` — les commandes dont la
réussite est un verdict sur le projet — **refusent de s'exécuter** lorsque le
codev en cours ne satisfait pas `codev_version` : elles ne vérifient et
n'écrivent rien, et échouent avec `codev_version_unsatisfied`, dont le
message nomme l'exigence, la version en cours et la même correction que
`codev doctor`. Sans cela, un codev plus ancien pourrait ignorer une clé qui
resserre une vérification et accepter ce que le codev d'un collègue refuse
(voir [Clés inconnues](#clés-inconnues)). Toutes les autres commandes
continuent de fonctionner, pour que `codev status`, `codev doctor` et les
skills puissent encore vous dire quoi installer. Deux conséquences à
anticiper :

- un job de CI épinglé avec `CODEV_VERSION=` (comme l'écrit `codev init`)
  se met à échouer dès que `codev_version` dépasse cette version : relevez
  l'épinglage dans le même commit ;
- une exigence avec une borne supérieure (`^0.6`) refuse aussi un codev plus
  récent : élargissez `codev_version` avant de passer au-delà, ou installez
  la version que nomme `codev doctor`.

codev 0.5 et les versions antérieures ne connaissent pas cette clé et
rejettent le fichier : ajoutez-la une fois que tout le monde utilise une
version qui la connaît.

## roles

Les [rôles](../roles.md) qui s'exécutent, et les critères que votre projet
leur ajoute. Propre au projet, jamais hérité. Chaque clé est un identifiant
de rôle ; sa valeur est soit une activation, soit une table :

| Clé | Signification |
|---|---|
| `when` | `always`, `never` ou `on_domain` ; la valeur par défaut est `on_domain` pour les experts, `always` pour tous les autres rôles. `on_domain` n'active un rôle que pour un change dont la proposal coche l'un de ses domaines. Une étape dont tous les rôles sont inactifs est sautée, et la skill le signale |
| `done_when` | Critères ajoutés après ceux du rôle, avec l'origine `project` ; les entrées vides sont ignorées |
| `model` | Le modèle sur lequel tourne le sous-agent du rôle dans le [mode de revue](#review) `isolated` : `opus`, `sonnet` ou `haiku`, ou `inherit` pour le modèle de la session. Aucun rôle n'en fixe par défaut — voir [Les modèles des relecteurs](../roles.md#les-modèles-des-relecteurs). En mode `inline`, chaque rôle tourne sur le modèle de la session |

```yaml
roles:
  newcomer: never            # forme courte
  security: always           # l'expert sécurité relit chaque change
  skeptic:
    when: always
    done_when:
      - Le plan de retour arrière est indiqué
  privacy:
    model: sonnet            # le sous-agent de l'expert vie privée tourne sur Sonnet
  migration:
    model: inherit           # retour au modèle de la session
```

Les identifiants sont ceux des rôles d'étape intégrés (`analyst`,
`investigator`, `tracer`, `skeptic`, `developer`, `newcomer`), ceux des
experts (`security`, `privacy`, `accessibility`, `performance`, `ops`,
`migration` — voir [Experts et domaines](../roles.md#experts-et-domaines))
et ceux des rôles d'artefact du schéma utilisé (`product`, `qa`,
`architect`, `lead` avec `spec-driven`). Les rôles d'artefact, l'`analyst`
et le `developer` ne peuvent pas être désactivés ; leurs critères peuvent
être complétés. Le `change.yaml`
d'un change peut changer l'activation pour ce change seulement — voir
[change.yaml](file-formats.md#changeyaml).

`codev validate` signale un identifiant inconnu par `unknown_role`, une
activation autre que `always`, `never` ou `on_domain` — ou `on_domain` sur
un rôle lié à aucun domaine — par `invalid_role_activation`, et `never` sur
un rôle qui ne peut pas être désactivé par `role_not_disableable` — des erreurs de l'élément `config`, à la ligne du
rôle. `codev status`, `codev instructions` et `codev roles` échouent avec
le même code. Toute autre clé dans la table d'un rôle, comme un `done-when`
mal orthographié, est ignorée avec un avertissement `unknown_config_key`
nommé `roles.<id>.<clé>`, à la ligne de la clé : le reste de la table du
rôle s'applique, et `codev validate --strict` échoue dessus, si bien qu'une
faute de frappe reste détectée. Un `model` autre que `opus`,
`sonnet`, `haiku` ou `inherit` est une erreur `invalid_role_model`,
signalée et bloquante de la même façon. Un `model` sur un rôle qui ne sert
aucune étape de relecture (`plan.trace`, `plan.challenge`, `code.review`)
et qui n'est pas le `developer` ne s'applique jamais : c'est un
avertissement `role_model_unused`. Le modèle du `developer` implémente les
tâches de `/codev-apply` dans un sous-agent, quel que soit le mode de
revue (décision 0012).

**`model` et les versions plus anciennes de codev.** Un codev antérieur à
la version qui a introduit `model` ne connaît pas la clé : codev 0.6.1 et
les versions antérieures la refusent, comme toute clé inconnue dans un
rôle, et chaque commande échoue avec `invalid`. Relevez
[`codev_version`](#codev_version) à cette version avant de l'indiquer,
pour que chaque codev qui lit le dépôt, CI comprise, la comprenne. Une
source partagée par [`inherits`](#inherits) ne devrait pas l'indiquer tant
que chaque projet qui en hérite n'utilise pas cette version : à partir
d'elle, le `roles:` d'une source est ignoré sans être lu, mais un codev
plus ancien le lit encore et perdrait le contexte et les règles de la
source.

## triggers

Les [déclencheurs](../roles.md#les-déclencheurs-et-les-domaines-suggérés)
propres à votre projet : les chemins et les motifs que seule votre équipe
sait sensibles. Propre au projet, jamais hérité. Une liste de groupes, qui
nomment chacun le domaine qu'ils suggèrent et contiennent au moins l'une
des clés `paths`, `manifests` ou `patterns` :

| Clé | Signification |
|---|---|
| `domain` | `security`, `personal-data`, `user-interface`, `performance`, `operations` ou `migration` |
| `paths` | Globs sur les fichiers modifiés, relatifs à la racine du projet, sans tenir compte de la casse ; `*` reste dans un dossier, `**` traverse les dossiers |
| `manifests` | Noms de fichiers de manifestes de dépendances, reconnus à toute profondeur |
| `patterns` | Expressions régulières sur les lignes que le diff ajoute |

```yaml
triggers:
  - domain: personal-data
    paths: ["src/billing/**", "src/crm/**"]
    patterns: ['\bcustomer_id\b']
  - domain: migration
    paths: [api/public.yaml]
```

Mettez entre guillemets un glob qui commence par `*` : en YAML, `*` ouvre
un alias. Leurs correspondances ont la source `project`, et ils sont
évalués quels que soient vos réglages `roles:`. Les expressions s'exécutent
en temps linéaire, quelle que soit leur forme, et leur taille compilée est
plafonnée.

`codev validate` signale, à la ligne de `triggers:`, une erreur
`invalid_trigger` pour un groupe sans `domain` connu, sans aucun
déclencheur, ou avec un glob ou une expression qui ne compile pas — le
groupe est alors ignoré par toutes les commandes, qui le signalent dans
leur `status` —, et un avertissement `invalid_trigger` pour une clé
qu'il ne connaît pas, ignorée tandis que le reste du groupe s'applique.

codev 0.6 et les versions antérieures ne connaissent pas cette clé : elles
émettent l'avertissement `unknown_config_key`, qui fait échouer leur
`codev validate --strict`. Lorsque vous l'ajoutez, relevez
`codev_version` jusqu'à la version qui l'a introduite.

## review

La manière dont les rôles relecteurs travaillent. Propre au projet, jamais
héritée.

| Clé | Signification |
|---|---|
| `mode` | `inline` (par défaut) : la skill relit dans sa propre conversation, un rôle après l'autre. `isolated` : chaque rôle relecteur travaille seul, comme sous-agent de Claude Code qui ne voit jamais la conversation, puis la skill fusionne leurs constats |

```yaml
review:
  mode: isolated
```

Un change peut la remplacer dans son `change.yaml`, avec la même clé :
`review: { mode: inline }` pour corriger une coquille dans un projet
isolé, ou `isolated` pour un change sensible dans un projet en ligne.
`codev roles --json` donne le mode en vigueur et son origine
(`review.mode`, `review.origin`). Toute autre valeur fait échouer chaque
commande avec `invalid_review_mode`, qui nomme `inline` et `isolated` ;
une clé inconnue sous `review` est un avertissement `unknown_config_key`
(`review.<clé>`).

**Changer de mode.** Définissez la clé, puis lancez `codev update` : en
mode isolé, les skills `/codev-propose` et `/codev-apply` demandent
l'outil `Agent` de Claude Code, en mode en ligne elles ne le demandent
pas, et `codev doctor` signale `skills_review_mode` tant qu'elles n'ont
pas été régénérées. Une skill que vous avez modifiée à la main est
conservée ; `codev update --force` la réécrit.

**Pourquoi `inline` par défaut.** La revue isolée multiplie la lecture de
la matière par le nombre de rôles, et son bénéfice n'est pas encore
mesuré : les évaluations des rôles de codev font travailler un rôle à la
fois, sans conversation d'écriture, et ne peuvent donc pas comparer les
deux modes. Voir
[Revue en ligne ou isolée](../roles.md#revue-en-ligne-ou-isolée).

codev 0.6 et les versions antérieures ne connaissent pas cette clé :
elles émettent l'avertissement `unknown_config_key` — qui fait échouer
leur `codev validate --strict` — et refusent un `change.yaml` qui la
définit. Lorsque vous la définissez, relevez `codev_version` jusqu'à la
version qui l'a introduite, pour que chaque codev qui lit le dépôt, CI
comprise, la comprenne.

## Clés inconnues

Une version plus récente de codev peut ajouter des clés qu'une version plus
ancienne ne connaît pas. Pour qu'un codev plus ancien continue de fonctionner
sur une configuration plus récente et vous dise ce dont il a besoin, les clés
inconnues sont traitées selon ce que produirait le fait de les ignorer :

| Emplacement | Clé inconnue | Pourquoi |
|---|---|---|
| premier niveau | avertissement `unknown_config_key`, clé ignorée | chaque clé est une fonctionnalité indépendante : le reste de la configuration s'applique |
| `mcp` | avertissement `unknown_config_key` (`mcp.<clé>`), clé ignorée | un outil inconnu laisse seulement cette intégration non configurée |
| `review` | avertissement `unknown_config_key` (`review.<clé>`), clé ignorée | un réglage de revue inconnu laisse la revue telle qu'elle était |
| une entrée de `inherits` | erreur `invalid` | l'ignorer ferait lire une autre source que celle que vous avez décrite |
| `changelog` | erreur `invalid` | l'ignorer ferait écrire un autre fichier à `codev changelog --write` |
| une entrée de `roles` | avertissement `unknown_config_key` (`roles.<id>.<clé>`), clé ignorée | le reste de la table du rôle s'applique ; une clé dont l'oubli ferait disparaître une relecture s'accompagne d'un `codev_version` relevé |
| un groupe de `triggers` | avertissement `invalid_trigger` de `codev validate`, clé ignorée | le reste du groupe suggère toujours son domaine |

L'avertissement nomme la clé, indique que le projet demande peut-être une
version plus récente de codev, cite `codev_version` quand le fichier la
définit et renvoie à `codev self-update` ; l'erreur fait de même quand
`codev_version` est définie. L'avertissement apparaît :

- dans la sortie de `codev status`, `codev new change`, `codev init` et
  `codev update`, et dans le tableau `status` de leur sortie `--json` ;
- dans `codev validate`, comme constat `warning` d'un élément `config`, à la
  ligne de la clé : `codev validate --strict` — le cas de la CI — échoue
  donc, tandis que `codev validate` seul réussit ;
- dans `codev doctor`, comme vérification `config.keys`.

Les clés de la configuration d'une source héritée ne sont pas signalées :
seuls son `context` et ses `rules` sont lus.

Les autres fichiers que vous écrivez à la main suivent la même règle, chacun
avec son propre code, pour que vous sachiez quel fichier ouvrir :

| Fichier | Clé inconnue | Signalée par |
|---|---|---|
| le `change.yaml` d'un change, au premier niveau ou sous `review:` | avertissement `change_unknown_key` (`review.<clé>` sous `review`), clé ignorée | les commandes qui lisent le change (`status`, `instructions`, `roles`, `review`), et `codev validate` sur l'élément du change |
| un `schema.yaml` de projet, à tout niveau, `role:` d'un artefact compris | avertissement `schema_unknown_key`, clé ignorée | les commandes qui utilisent le schéma, et `codev validate` sur l'élément `config`, localisé dans le fichier du schéma |
| le frontmatter d'un ADR | avertissement `decision_unknown_field`, champ ignoré et conservé par `codev decision accept` | `codev validate` sur l'élément `decisions`, à la ligne du champ ; ceux d'une décision héritée ne sont pas signalés |

Une *valeur* que codev ne connaît pas — un `review.mode`, une activation de
rôle, un `status` d'ADR — reste une erreur : on ne peut pas l'ignorer sans
deviner ce que vous vouliez dire. Les fichiers que codev écrit lui-même
portent leur format : un `_codev/decisions/seal.yaml` d'une `version` plus
récente est refusé avec `seal_version_unsupported`, qui renvoie à
`codev self-update`. Cette politique est la décision 0013 du dépôt de codev.

Cette tolérance commence avec la version de codev qui l'a introduite :
**codev 0.5 et les versions antérieures rejettent toute clé inconnue**, à tout
niveau, et **codev 0.6.1 et les versions antérieures rejettent une clé
inconnue dans un rôle, dans `change.yaml`, dans un schéma de projet et dans
le frontmatter d'un ADR**. Lorsque vous ajoutez une clé que seules les versions récentes
connaissent, relevez aussi `codev_version` : les membres de l'équipe restés
sur un codev plus ancien apprendront par `codev doctor` et les skills quelle
version installer.

## Variables d'environnement

| Variable | Utilisée par | Effet |
|---|---|---|
| `LC_ALL`, `LC_MESSAGES`, `LANG` | `codev init` | Valeur par défaut de `language:` lorsque `--language` n'est pas fourni |
| `XDG_CACHE_HOME` | sources git, vérification des mises à jour | Emplacement du cache : `$XDG_CACHE_HOME/codev/`, sinon `~/.cache/codev/` |
| `CI` | `codev status` | N'importe quelle valeur désactive la vérification des mises à jour |
| `CODEV_NO_UPDATE_CHECK` | `codev status` | `1` désactive la vérification des mises à jour |
| `VISUAL`, `EDITOR` | `codev init` | Éditeur ouvert lorsque la question du contexte reste sans réponse |
| `CODEV_VERSION` | `install.sh`, `install.ps1` | Version à installer à la place de la plus récente |
