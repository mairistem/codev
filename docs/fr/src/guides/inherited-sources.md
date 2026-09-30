# Sources héritées

Les équipes qui maintiennent plusieurs dépôts partagent en général des
conventions : la façon dont les services journalisent, la base de données à
utiliser, le versionnement des API. Grâce aux sources héritées, ces
conventions vivent dans un seul dépôt codev, et chaque projet qui en hérite les
reçoit — sans rien copier.

## Ce qu'apporte une source

Une source héritée est un autre projet codev, doté de son propre
`_codev/config.yaml`. Votre projet y lit trois choses :

- son **`context:`**, placé avant votre propre contexte dans les
  instructions de chaque artefact ;
- ses **`rules:`**, placées avant vos propres règles pour le même artefact ;
- ses **décisions**, issues de son dossier `_codev/decisions/`, qui sont
  transmises à l'agent avec les vôtres lorsqu'il rédige un `design.md`.

Les sources sont en lecture seule. codev n'y écrit jamais, et n'y lit que des
fichiers Markdown et YAML : rien d'exécutable n'est hérité.

Lorsque l'agent reçoit le contexte et les règles, chaque bloc indique sa
provenance, du plus général (les sources, dans l'ordre de déclaration) au plus
spécifique (votre projet). Lorsque deux blocs se contredisent, le dernier
l'emporte, et les skills ont pour consigne de signaler la contradiction plutôt
que de la trancher en silence.

L'héritage n'est pas transitif : si une source déclare elle-même
`inherits:`, ces sources sont ignorées et codev vous en avertit. Déclarez
directement dans votre projet toutes les sources dont vous avez besoin.

## Sources locales

Une source `path:` est un dossier de votre machine — en général un clone du
dépôt partagé de votre équipe :

```yaml
# _codev/config.yaml
inherits:
  - path: ~/src/acme-standards
```

Le chemin peut être absolu ou commencer par `~`. Il est lu directement : les
modifications apportées à ce dossier sont donc visibles immédiatement. Les
sources `path:` sont pratiques pendant que vous rédigez des conventions
partagées ; pour tous les autres, préférez une source git épinglée.

## Sources git

Une source `git:` est un dépôt distant, épinglé sur un commit :

```yaml
inherits:
  - git: git@github.com:acme/codev-standards.git
    ref: main
```

`ref` est obligatoire : une branche ou un tag. Utilisez `subpath` lorsque le
projet codev se trouve dans un sous-dossier du dépôt :

```yaml
inherits:
  - git: git@github.com:acme/platform.git
    ref: v2
    subpath: standards
```

Une source git n'est utilisée qu'une fois verrouillée. Résolvez-la et
téléchargez-la avec :

```bash
codev sources update
```

```text
Changes:
  + git@github.com:acme/codev-standards.git @main  → 12958274cbed0eb66985f57d3272c6d3134b672d

codev.lock updated.
```

Cette commande résout chaque `ref` en commit avec `git ls-remote`, récupère ce
commit dans un cache local et l'enregistre dans `_codev/codev.lock` :

```toml
version = 1

[[source]]
git = "git@github.com:acme/codev-standards.git"
ref = "main"
commit = "12958274cbed0eb66985f57d3272c6d3134b672d"
resolved_at = "2026-09-30"
```

**Commitez `codev.lock`.** Toutes les personnes qui travaillent sur le projet
lisent alors exactement le même commit, et un changement d'épinglage apparaît
en revue comme n'importe quel autre diff.

`codev sources update` est la seule commande qui accède au réseau. Toutes les
autres lisent le commit verrouillé depuis le cache, hors ligne. Relancez
`codev sources update` pour déplacer les épinglages vers le commit actuel de
chaque `ref` ; la commande indique pour chaque source si elle est ajoutée
(`+`), déplacée (`~`, avec l'ancien et le nouveau commit) ou inchangée (`=`).

Le cache se trouve dans `$XDG_CACHE_HOME/codev/`, ou dans `~/.cache/codev/`
lorsque `XDG_CACHE_HOME` n'est pas défini. Les sources git nécessitent `git`
dans votre `PATH`, avec un accès au dépôt : codev utilise vos identifiants git
existants.

## Inspecter les sources

```bash
codev sources list
```

```text
Inherited sources:
  git git@github.com:acme/codev-standards.git  [locked]
      commit: 12958274cbed0eb66985f57d3272c6d3134b672d
      path:   /home/you/.cache/codev/content/12958274cbed0eb66985f57d3272c6d3134b672d
```

Chaque source se trouve dans l'un des états suivants :

| État | Signification |
|---|---|
| `resolved` | Source `path:` trouvée, avec un `_codev/config.yaml` |
| `unreadable` | Source `path:` introuvable, ou dépourvue de `_codev/config.yaml` |
| `unlocked` | Source `git:` pas encore présente dans `codev.lock` — lancez `codev sources update` |
| `locked` | Source `git:` épinglée et présente dans le cache |
| `needs_update` | Source `git:` épinglée mais absente du cache — lancez `codev sources update` |

`codev sources show <TARGET>` affiche une source, désignée par son chemin ou
son URL, et liste les fichiers qu'elle expose.

## Décisions héritées

Les décisions héritées apparaissent dans `codev decision list` à côté des
vôtres. Leur identifiant qualifié indique la source dont elles proviennent,
telle que déclarée dans votre configuration :

```text
path:~/src/acme-standards/0100
git:git@github.com:acme/codev-standards.git/0100
```

Si une décision locale porte le même identifiant court qu'une décision
héritée, la décision locale l'emporte et codev émet l'avertissement
`decision_id_collision`.

Vous ne pouvez ni modifier ni remplacer une décision héritée depuis votre
projet. Lorsque votre projet doit s'en écarter, consignez un écart :

```bash
codev decision deviate path:~/src/acme-standards/0100 "Services log in logfmt"
```

La nouvelle décision locale référence la décision héritée dans
`deviates_from`. Elle est créée `proposed`, non scellée : rédigez pourquoi
votre projet s'écarte, puis acceptez-la, ce qui la scelle :

```bash
codev decision accept 0003
```

L'écart n'entre en vigueur qu'une fois accepté. D'ici là, la décision héritée
reste en vigueur et figure toujours dans les instructions de l'artefact
`design`. Une fois l'écart accepté, la décision héritée reste listée, ce qui
rend l'écart visible, mais elle n'est plus en vigueur pour votre projet. Voir
[Écart](../concepts.md#écart).

## Ce qui n'est pas hérité

- **Les specs.** Chaque projet possède ses propres specs de comportement.
- **`schema`, `language`, `workflows` et `mcp`.** Ces clés relèvent du projet
  et des personnes qui y travaillent.
- **Les sources de la source**, comme expliqué plus haut.
