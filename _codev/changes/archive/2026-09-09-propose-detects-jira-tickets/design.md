# Design : `codev-propose` détecte et enrichit un ticket Jira

## Contexte

Voir `proposal.md`. Premier vrai wire d'un MCP dans une skill codev.
Le design cristallise les six choix pris pendant l'exploration.

## Objectifs / Hors objectifs

Ce design cadre : la place de la détection dans le flow, le format de
la citation en tête du proposal, la stratégie de fallback sans MCP, et
le test d'invariant qui verrouille l'`allowed-tools`. Il ne cadre pas
d'autres MCP (Design, GitHub…), ni un mode multi-tickets, ni la
configuration projet du pattern.

## Décisions

### Décision : la détection vit dans le body markdown, pas dans le CLI

Alignement direct avec la stratégie MCP notée en mémoire : le CLI
`codev` reste **agnostique aux MCP**. Il ne connaît ni Jira ni
Atlassian ; il gère `_codev/`, le graphe d'artefacts, le sceau, la
fusion, l'archive. Les intégrations externes vivent au niveau skill
Claude Code — c'est l'agent qui lit le body, repère le pattern, appelle
le MCP, et rédige le proposal.

**Conséquence** : rien à changer dans les crates Rust côté runtime.
Seuls le body markdown de la skill et l'`allowed-tools` du CATALOG
bougent. Une intégration future d'un autre MCP suivra le même
gabarit.

**Alternative écartée** : un flag `codev new change --from-ticket
JVS-1234` qui, côté CLI, appellerait le MCP via un port dédié. Rejeté
parce qu'il obligerait le core à connaître les MCP et casserait
l'agnosticisme.

### Décision : nom exact du MCP hardcodé — `mcp__claude_ai_Atlassian__getJiraIssue`

Le MCP officiel Claude Atlassian expose une trentaine d'outils ; on ne
déclare que celui strictement nécessaire à la lecture d'un ticket. La
skill n'écrit jamais dans Jira, ne fait pas de recherche JQL, ne
manipule ni Confluence ni Compass. La restriction est un garde-fou :
si un jour un mauvais prompt tente d'utiliser la skill pour écrire, le
tool call échouera au niveau harness plutôt que de silencieusement
transiger.

**Alternative écartée** : `mcp__claude_ai_Atlassian__*` (glob). Plus
permissif mais laisse la porte ouverte à des appels non prévus.
Refusé au titre du principe « minimum viable » de la sécurité.

### Décision : citation en tête du proposal, pas dans le fichier `change.yaml`

Le ticket est **information humaine** — l'endroit naturel est le
proposal, que le relecteur lit en premier. Le mettre dans
`change.yaml` (par exemple `source_ticket: JVS-1234`) aurait obligé à
étendre `ChangeMetadata` (nouveau champ), à le parser, à le rendre au
statut… du code pour un cas qui vit très bien en markdown.

**Format retenu** — première ligne du proposal sous le titre :

```
# Proposal : <titre>

> Source : ticket **JVS-1234** — « Authentifier les utilisateurs par JWT » (In Progress)

## Pourquoi
[…]
```

Si le MCP a échoué : `> Source : ticket **JVS-1234** — contenu non
récupéré`. La forme reste stable, seul le suffixe change.

### Décision : le pattern est générique `[A-Z]{2,}-\d+`, pas configurable

Toute organisation Atlassian utilise un préfixe majuscules de 2+
lettres suivi d'un tiret et d'un numéro. Un `A-1` seul serait
ambigu (référence à une cellule Excel ?) — la contrainte 2+ lettres
évite ce faux positif. Ajouter la configuration projet
(`ticket_pattern: "JVS-\\d+"`) coûterait un nouveau champ
`ChangeMetadata` ou `ProjectConfig` pour un bénéfice marginal — les
faux positifs sur `[A-Z]{2,}-\d+` en pratique sont rares.

**Reportable** : si un projet remonte régulièrement des faux positifs,
on ajoutera `_codev/config.yaml.ticket_pattern`.

### Décision : sans MCP, message informatif — pas silencieux

Un utilisateur qui mentionne `JVS-1234` s'attend à ce que ça compte
pour quelque chose. Rester silencieux le laisserait dans le flou.
Le message coûte une phrase, éclaire le comportement de la skill, et
n'empêche pas l'exécution.

**Alignement** avec la philosophie codev : « signaler, jamais bloquer
silencieusement » — pattern déjà appliqué dans `sync` (invite à
archiver), `validate` (émet des warnings), `deviate` (annonce la
ripple).

### Décision : un ticket récupéré, les autres juste nommés

Traiter chaque ticket mentionné multiplierait les appels MCP et
alourdirait le proposal. La règle « premier détecté = source
principale » est simple et prévisible. Les autres tickets restent
mentionnés en tête (`autre(s) ticket(s) mentionné(s) : JVS-5678`) pour
la traçabilité — un lecteur du proposal peut aller les consulter à
la main.

**Reportable** : un mode `--all-tickets` ou un multi-appel MCP si le
pattern devient récurrent.

## Risques et compromis

- **L'utilisateur ne s'attend pas à ce que la skill appelle un MCP.**
  Sur un environnement où plusieurs MCP sont branchés, un appel
  inattendu peut être surprenant. → **Atténuation** : le message
  informatif quand le MCP réussit (« Ticket JVS-1234 récupéré via
  MCP Atlassian, injecté dans le contexte du proposal ») rend l'appel
  visible.
- **Faux positif de pattern** — un identifiant qui ressemble à un
  ticket sans en être un (par exemple `TODO-42` dans un commentaire du
  prompt). → **Compromis assumé** : le MCP renverra probablement
  `not_found`, la skill se rabat sur « ticket mentionné, contenu non
  récupéré ». Le pire cas est un warning inutile.
- **Le contenu du ticket est trop volumineux** — une description Jira
  peut être longue. → **Compromis assumé** : le MCP retourne un JSON
  structuré ; la skill résume ce qu'elle en injecte dans le prompt
  (titre, status, description tronquée si besoin). C'est de la
  responsabilité rédactionnelle de la skill, pas d'une logique de
  code.
- **Le MCP retourne une erreur d'authentification** (token expiré,
  espace privé). → **Comportement** : traité comme « MCP
  indisponible », message informatif spécifique
  (« authentification requise »).

## Plan de migration

Aucune. Les utilisateurs sans MCP branché ne voient rien changer
tant qu'ils ne mentionnent pas de ticket. À la première mention de
ticket sans MCP, un message informatif apparaît — mais le proposal
sort quand même.

Pour activer la nouvelle capacité, il suffit de brancher le MCP
Atlassian dans la config Claude Code de l'utilisateur (indépendant
de codev) — puis de relancer `codev update --force` pour que la
nouvelle version de la skill soit installée avec son `allowed-tools`
étendu.
