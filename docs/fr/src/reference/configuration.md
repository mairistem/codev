# Configuration

Un projet codev se configure dans `_codev/config.yaml`. Toutes les clés sont
facultatives : un projet peut même fonctionner sans ce fichier. Une clé que
codev ne connaît pas est signalée, par un avertissement au premier niveau et
dans `mcp`, par une erreur ailleurs : une faute de frappe ne passe donc
jamais inaperçue. Voir [Clés inconnues](#clés-inconnues).

`codev init` génère le fichier avec les valeurs qu'il a détectées, chacune
précédée d'un commentaire indiquant sa provenance. Vous pouvez le modifier à la
main à tout moment. Après avoir modifié `workflows` ou `mcp`, lancez
`codev update` pour que les skills reflètent la modification.

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
```

## schema

Le schéma de workflow utilisé par les nouveaux changes. Valeur par défaut :
`spec-driven`, le schéma embarqué dans codev. Le nom d'un schéma défini sous
`_codev/schemas/` est également accepté. Un change enregistre son schéma dans
son propre `change.yaml` : modifier cette clé n'affecte donc pas les changes
en cours. Voir [Schémas personnalisés](../guides/custom-schemas.md).

## language

La langue dans laquelle les skills rédigent la prose des artefacts, sous la
forme d'un code ISO 639 : deux ou trois lettres minuscules, éventuellement
suivies d'une sous-étiquette (`en`, `fr`, `pt-BR`, `zh-Hant`). Valeur par
défaut : `en`. Les mots-clés structurels restent toujours en anglais. Voir
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

```yaml
roles:
  newcomer: never            # forme courte
  security: always           # l'expert sécurité relit chaque change
  skeptic:
    when: always
    done_when:
      - Le plan de retour arrière est indiqué
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
mal orthographié, fait échouer chaque commande avec `invalid` : l'ignorer
ferait disparaître des critères sans bruit.

## Clés inconnues

Une version plus récente de codev peut ajouter des clés qu'une version plus
ancienne ne connaît pas. Pour qu'un codev plus ancien continue de fonctionner
sur une configuration plus récente et vous dise ce dont il a besoin, les clés
inconnues sont traitées selon ce que produirait le fait de les ignorer :

| Emplacement | Clé inconnue | Pourquoi |
|---|---|---|
| premier niveau | avertissement `unknown_config_key`, clé ignorée | chaque clé est une fonctionnalité indépendante : le reste de la configuration s'applique |
| `mcp` | avertissement `unknown_config_key` (`mcp.<clé>`), clé ignorée | un outil inconnu laisse seulement cette intégration non configurée |
| une entrée de `inherits` | erreur `invalid` | l'ignorer ferait lire une autre source que celle que vous avez décrite |
| `changelog` | erreur `invalid` | l'ignorer ferait écrire un autre fichier à `codev changelog --write` |
| une entrée de `roles` | erreur `invalid` | l'ignorer ferait disparaître des critères ou une activation |

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

Cette tolérance commence avec la version de codev qui l'a introduite :
**codev 0.5 et les versions antérieures rejettent toute clé inconnue**, à tout
niveau. Lorsque vous ajoutez une clé que seules les versions récentes
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
