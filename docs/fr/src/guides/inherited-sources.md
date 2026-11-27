# Sources héritées

Les équipes qui maintiennent plusieurs dépôts partagent en général des
conventions : la façon dont les services journalisent, la base de données à
utiliser, le versionnement des API. Grâce aux sources héritées, ces
conventions vivent dans un seul dépôt Compagnon, et chaque projet qui en hérite les
reçoit — sans rien copier.

<figure class="diagram">
<div class="scroll">
<svg viewBox="0 0 760 270" role="img" aria-label="Sources héritées : un dépôt ou un dossier partagé, acme-standards, contient un contexte, des règles et des décisions, en lecture seule. Deux projets, acme-api et acme-web, le lisent chacun à la version épinglée dans leur propre _compagnon/compagnon.lock. Seul compagnon sources update déplace un épinglage.">
<defs><marker id="dg-inherited-sources-arrow" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" style="fill:var(--dg-line)"/></marker></defs>
<rect x="8" y="40" width="236" height="176" rx="12" style="fill:var(--dg-box);stroke:var(--dg-line)" stroke-width="2"/>
<text x="24" y="68" font-size="14" font-weight="600" style="fill:var(--dg-ink)">Dépôt ou dossier partagé</text>
<text x="24" y="90" class="mono" font-size="13" style="fill:var(--dg-muted)">acme-standards</text>
<rect x="24" y="102" width="204" height="24" rx="6" style="fill:var(--dg-file);stroke:var(--dg-line)" stroke-width="1"/>
<text x="36" y="119" class="mono" font-size="13" style="fill:var(--dg-ink)">context:</text>
<rect x="24" y="132" width="204" height="24" rx="6" style="fill:var(--dg-file);stroke:var(--dg-line)" stroke-width="1"/>
<text x="36" y="149" class="mono" font-size="13" style="fill:var(--dg-ink)">rules:</text>
<rect x="24" y="162" width="204" height="24" rx="6" style="fill:var(--dg-file);stroke:var(--dg-line)" stroke-width="1"/>
<text x="36" y="179" class="mono" font-size="13" style="fill:var(--dg-ink)">decisions/</text>
<text x="24" y="204" font-size="12" style="fill:var(--dg-muted)">en lecture seule</text>
<rect x="470" y="20" width="282" height="100" rx="12" style="fill:var(--dg-box);stroke:var(--dg-claude)" stroke-width="2"/>
<text x="486" y="46" font-size="14" font-weight="600" style="fill:var(--dg-ink)">Projet acme-api</text>
<rect x="486" y="58" width="250" height="48" rx="8" style="fill:var(--dg-file);stroke:var(--dg-line)" stroke-width="1"/>
<text x="498" y="77" class="mono" font-size="12" style="fill:var(--dg-ink)">_compagnon/compagnon.lock</text>
<text x="498" y="96" class="mono" font-size="12" style="fill:var(--dg-muted)">commit = "12958274…"</text>
<rect x="470" y="144" width="282" height="100" rx="12" style="fill:var(--dg-box);stroke:var(--dg-claude)" stroke-width="2"/>
<text x="486" y="170" font-size="14" font-weight="600" style="fill:var(--dg-ink)">Projet acme-web</text>
<rect x="486" y="182" width="250" height="48" rx="8" style="fill:var(--dg-file);stroke:var(--dg-line)" stroke-width="1"/>
<text x="498" y="201" class="mono" font-size="12" style="fill:var(--dg-ink)">_compagnon/compagnon.lock</text>
<text x="498" y="220" class="mono" font-size="12" style="fill:var(--dg-muted)">commit = "12958274…"</text>
<line x1="246" y1="104" x2="466" y2="70" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-inherited-sources-arrow)"/>
<line x1="246" y1="152" x2="466" y2="194" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-inherited-sources-arrow)"/>
<text x="356" y="76" text-anchor="middle" font-size="12" style="fill:var(--dg-muted)">lit</text>
<text x="356" y="196" text-anchor="middle" font-size="12" style="fill:var(--dg-muted)">à la version épinglée</text>
<text x="8" y="258" font-size="13" style="fill:var(--dg-muted)">seul compagnon sources update déplace un épinglage</text>
</svg>
</div>
<figcaption>Sources héritées : chaque projet lit les conventions partagées à la version que fixe son <code>compagnon.lock</code>, sans rien copier.</figcaption>
</figure>

## Ce qu'apporte une source

Une source héritée est un autre projet Compagnon, doté de son propre
`_compagnon/config.yaml`. Votre projet y lit trois choses :

- son **`context:`**, placé avant votre propre contexte dans les
  instructions de chaque artefact ;
- ses **`rules:`**, placées avant vos propres règles pour le même artefact ;
- ses **décisions**, issues de son dossier `_compagnon/decisions/`, qui sont
  transmises à l'agent avec les vôtres lorsqu'il rédige un `design.md`.

Les sources sont en lecture seule. Compagnon n'y écrit jamais, et n'y lit que des
fichiers Markdown et YAML : rien d'exécutable n'est hérité.

Lorsque l'agent reçoit le contexte et les règles, chaque bloc indique sa
provenance, du plus général (les sources, dans l'ordre de déclaration) au plus
spécifique (votre projet). Lorsque deux blocs se contredisent, le dernier
l'emporte, et les compétences ont pour consigne de signaler la contradiction plutôt
que de la trancher en silence.

L'héritage n'est pas transitif : si une source déclare elle-même
`inherits:`, ces sources sont ignorées et Compagnon vous en avertit. Déclarez
directement dans votre projet toutes les sources dont vous avez besoin.

## Déclarer les sources à l'initialisation

Le plus rapide pour démarrer un projet sur des conventions partagées est de
nommer les sources dès l'initialisation, une par `--inherit` :

```bash
compagnon init --yes --inherit /home/you/acme-standards.git@main
```

```text
compagnon initialized in /home/you/acme-api

  Structure   _compagnon/ — specs, decisions, changes, schemas
  Skills      compagnon-propose, compagnon-explore, compagnon-onboard, compagnon-apply, compagnon-sync, compagnon-archive, compagnon-update, compagnon-configure → .claude/skills/
  Inherits    /home/you/acme-standards.git @main → pinned 8dc995327621
…
```

`compagnon init` les écrit sous `inherits:` dans le nouveau `_compagnon/config.yaml` :

```yaml
# set with compagnon init --inherit
inherits:
  - git: /home/you/acme-standards.git
    ref: main
```

Une valeur qui commence par `git@`, `https://`, `http://`, `ssh://` ou
`file://`, ou qui se termine par `.git`, est une source git ; `@<ref>` choisit
la branche ou le tag, et `main` s'applique en son absence. Toute autre valeur
est un dossier local, qui doit exister ; un dossier relatif est écrit en chemin
absolu, avec des barres obliques `/` sur toutes les plateformes, pour que le
fichier soit identique quelle que soit la machine qui a lancé `compagnon init`. Une valeur manifestement fausse — vide,
un dossier absent, une URL git terminée par `@` — est refusée avec
`invalid_inherit` avant toute écriture.

Les sources git sont épinglées aussitôt, exactement comme le ferait
`compagnon sources update`. Si cela échoue — pas de réseau, identifiants
manquants, `ref` inexistante —, le projet est tout de même initialisé et la
source déclarée, mais elle reste non épinglée :

```text
warning: git source `https://git.example.com/acme/compagnon-standards.git` has no locked SHA — run `compagnon sources update` to resolve it
warning: the inherited git source(s) could not be pinned: https://git.example.com/acme/compagnon-standards.git is invalid: ls-remote for `https://git.example.com/acme/compagnon-standards.git` failed: fatal: unable to access 'https://git.example.com/acme/compagnon-standards.git/': Could not resolve host: git.example.com
help: run compagnon sources update
```

Corrigez la cause, puis lancez `compagnon sources update`. `compagnon init` ne déplace
jamais un épinglage existant et ne modifie jamais un `config.yaml` existant :
sur un projet déjà initialisé, ajoutez les sources sous `inherits:` à la main.

## Sources locales

Une source `path:` est un dossier de votre machine — en général un clone du
dépôt partagé de votre équipe :

```yaml
# _compagnon/config.yaml
inherits:
  - path: ~/src/acme-standards
```

Le chemin peut être absolu ou commencer par `~`. Il est lu directement : les
modifications apportées à ce dossier sont donc visibles immédiatement. Les
sources `path:` sont pratiques pendant que vous rédigez des conventions
partagées ; pour tous les autres, préférez une source git épinglée.

## Sources git

Une source `git:` est un dépôt distant, épinglé sur un enregistrement :

```yaml
inherits:
  - git: git@github.com:acme/compagnon-standards.git
    ref: main
```

`ref` est obligatoire : une branche ou un tag. Utilisez `subpath` lorsque le
projet Compagnon se trouve dans un sous-dossier du dépôt :

```yaml
inherits:
  - git: git@github.com:acme/platform.git
    ref: v2
    subpath: standards
```

Une source git n'est utilisée qu'une fois verrouillée. Résolvez-la et
téléchargez-la avec :

```bash
compagnon sources update
```

```text
Changes:
  + git@github.com:acme/compagnon-standards.git @main  → 12958274cbed0eb66985f57d3272c6d3134b672d

compagnon.lock updated.
```

Cette commande résout chaque `ref` en enregistrement avec `git ls-remote`,
récupère cet enregistrement dans un cache local et l'enregistre dans `_compagnon/compagnon.lock` :

```toml
version = 1

[[source]]
git = "git@github.com:acme/compagnon-standards.git"
ref = "main"
commit = "12958274cbed0eb66985f57d3272c6d3134b672d"
resolved_at = "2026-09-30"
```

**Enregistrez `compagnon.lock`.** Toutes les personnes qui travaillent sur le projet
lisent alors exactement le même enregistrement (commit), et un changement d'épinglage apparaît
en relecture comme n'importe quel autre diff.

`compagnon sources update` et `compagnon init --inherit` sont les seules commandes qui
accèdent au réseau, et seule `compagnon sources update` déplace un épinglage.
Toutes les autres lisent l'enregistrement verrouillé depuis le cache, hors ligne. Relancez
`compagnon sources update` pour déplacer les épinglages vers l'enregistrement actuel de
chaque `ref` ; la commande indique pour chaque source si elle est ajoutée
(`+`), déplacée (`~`, avec l'ancien et le nouvel enregistrement) ou inchangée (`=`).

Le cache se trouve dans `$XDG_CACHE_HOME/compagnon/`, ou dans `~/.cache/compagnon/`
lorsque `XDG_CACHE_HOME` n'est pas défini. Les sources git nécessitent `git`
dans votre `PATH`, avec un accès au dépôt : Compagnon utilise vos identifiants git
existants.

## Inspecter les sources

```bash
compagnon sources list
```

```text
Inherited sources:
  git git@github.com:acme/compagnon-standards.git  [locked]
      commit: 12958274cbed0eb66985f57d3272c6d3134b672d
      path:   /home/you/.cache/compagnon/content/12958274cbed0eb66985f57d3272c6d3134b672d
```

Chaque source se trouve dans l'un des états suivants :

| État | Signification |
|---|---|
| `resolved` | Source `path:` trouvée, avec un `_compagnon/config.yaml` |
| `unreadable` | Source `path:` introuvable, ou dépourvue de `_compagnon/config.yaml` |
| `unlocked` | Source `git:` pas encore présente dans `compagnon.lock` — lancez `compagnon sources update` |
| `locked` | Source `git:` épinglée et présente dans le cache |
| `needs_update` | Source `git:` épinglée mais absente du cache — lancez `compagnon sources update` |

`compagnon sources show <TARGET>` affiche une source, désignée par son chemin ou
son URL, et liste les fichiers qu'elle expose.

## Décisions héritées

Les décisions héritées apparaissent dans `compagnon decision list` à côté des
vôtres. Leur identifiant qualifié indique la source dont elles proviennent,
telle que déclarée dans votre configuration :

```text
path:~/src/acme-standards/0100
git:git@github.com:acme/compagnon-standards.git/0100
```

Si une décision locale porte le même identifiant court qu'une décision
héritée, la décision locale l'emporte et Compagnon émet l'avertissement
`decision_id_collision`.

Vous ne pouvez ni modifier ni remplacer une décision héritée depuis votre
projet. Lorsque votre projet doit s'en écarter, consignez un écart :

```bash
compagnon decision deviate path:~/src/acme-standards/0100 "Services log in logfmt"
```

La nouvelle décision locale référence la décision héritée dans
`deviates_from`. Elle est créée `proposed`, non scellée : rédigez pourquoi
votre projet s'écarte, puis acceptez-la, ce qui la scelle :

```bash
compagnon decision accept 0003
```

L'écart n'entre en vigueur qu'une fois accepté. D'ici là, la décision héritée
reste en vigueur et figure toujours dans les instructions de l'artefact
`design`. Une fois l'écart accepté, la décision héritée reste listée, ce qui
rend l'écart visible, mais elle n'est plus en vigueur pour votre projet. Voir
[Contournée par un écart](decisions.md#contournée-par-un-écart).

## Ce qui n'est pas hérité

- **Les spécifications.** Chaque projet possède ses propres spécifications de
  comportement.
- **`schema`, `language`, `workflows` et `mcp`.** Ces clés relèvent du projet
  et des personnes qui y travaillent.
- **Les sources de la source**, comme expliqué plus haut.

## Pour aller plus loin

- [Décisions](decisions.md)
- [Configuration](../reference/configuration.md)
- [Interface en ligne de commande](../reference/cli.md)
