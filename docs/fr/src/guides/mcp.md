# Partir d'un ticket Jira

<div class="skip-step">

[Pas de Jira ? Passez à Relire un plan](review-a-plan.md)

</div>


Votre équipe décrit déjà le travail dans Jira. Nommez le ticket dans votre
demande et Claude part de lui pour écrire le [plan](../glossary.md#plan) :
pas de copier-coller, et le plan dit d'où il vient.

1. **Vous dites** : « Prépare un plan pour PROJ-123 ».
2. **Claude lit le ticket** — son titre, sa description, son statut et son
   type — une seule fois, sans rien changer dans Jira.
3. **Vous obtenez un plan qui le cite** dès sa première ligne : un
   relecteur voit à quel ticket il répond.

<figure class="diagram">
<div class="scroll">
<svg viewBox="0 0 760 210" role="img" aria-label="D'un ticket Jira à un plan : vous nommez le ticket PROJ-123 dans votre demande ; Claude lit le ticket une seule fois, en lecture seule ; Claude écrit proposal.md, dont la première ligne cite le ticket.">
<defs><marker id="dg-mcp-arrow" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" style="fill:var(--dg-line)"/></marker></defs>
<rect x="8" y="36" width="210" height="138" rx="12" style="fill:var(--dg-box);stroke:var(--dg-you)" stroke-width="2"/>
<text x="26" y="64" class="mono" font-size="13" style="fill:var(--dg-muted)">Ticket Jira</text>
<text x="26" y="92" class="mono" font-size="18" font-weight="600" style="fill:var(--dg-ink)">PROJ-123</text>
<text x="26" y="118" font-size="15" style="fill:var(--dg-ink)">Mode sombre pour le</text>
<text x="26" y="138" font-size="15" style="fill:var(--dg-ink)">tableau de bord</text>
<circle cx="32" cy="158" r="5" style="fill:var(--dg-you)"/>
<text x="44" y="163" font-size="13" style="fill:var(--dg-muted)">En cours</text>
<line x1="222" y1="105" x2="282" y2="105" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-mcp-arrow)"/>
<text x="252" y="92" text-anchor="middle" font-size="12" style="fill:var(--dg-muted)">nommé</text>
<rect x="288" y="56" width="180" height="98" rx="49" style="fill:var(--dg-box);stroke:var(--dg-claude)" stroke-width="2"/>
<circle cx="318" cy="88" r="5" fill="#ff5331"/><circle cx="332" cy="88" r="5" fill="#ffab03"/><circle cx="346" cy="88" r="5" fill="#00c39b"/>
<text x="378" y="94" font-size="17" font-weight="600" style="fill:var(--dg-ink)">Claude</text>
<text x="378" y="118" text-anchor="middle" font-size="14" style="fill:var(--dg-ink)">le lit une fois</text>
<text x="378" y="138" text-anchor="middle" font-size="12" style="fill:var(--dg-muted)">en lecture seule</text>
<line x1="472" y1="105" x2="532" y2="105" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-mcp-arrow)"/>
<text x="502" y="92" text-anchor="middle" font-size="12" style="fill:var(--dg-muted)">écrit</text>
<rect x="538" y="24" width="214" height="162" rx="12" style="fill:var(--dg-file);stroke:var(--dg-claude)" stroke-width="2"/>
<rect x="538" y="24" width="214" height="30" rx="12" style="fill:var(--dg-box)"/>
<text x="554" y="44" class="mono" font-size="13" style="fill:var(--dg-ink)">proposal.md</text>
<text x="554" y="80" class="mono" font-size="12" font-weight="600" style="fill:var(--dg-ink)"># Proposition : Mode sombre</text>
<rect x="550" y="94" width="190" height="44" rx="6" style="fill:var(--dg-box)"/>
<rect x="550" y="94" width="4" height="44" style="fill:var(--dg-you)"/>
<text x="562" y="112" class="mono" font-size="12" style="fill:var(--dg-ink)">Source : ticket PROJ-123</text>
<text x="562" y="129" class="mono" font-size="12" style="fill:var(--dg-muted)">« Mode sombre pour… »</text>
<text x="554" y="162" class="mono" font-size="13" font-weight="600" style="fill:var(--dg-ink)">## Pourquoi</text>
</svg>
</div>
<figcaption>D'un ticket Jira à un plan : vous nommez le ticket, Claude le lit une fois sans le modifier, et le plan le cite dès sa première ligne.</figcaption>
</figure>

<div class="example">
<p class="example-title">Exemple</p>

```text
> Prépare un plan pour PROJ-123
⏺ atlassian - getJiraIssue (MCP)(issueIdOrKey: "PROJ-123")
  ⎿ Mode sombre pour le tableau de bord · Story · In Progress
⏺ Write(_compagnon/changes/add-dark-mode/proposal.md)
  ⎿ Wrote 41 lines to _compagnon/changes/add-dark-mode/proposal.md
⏺ Le plan est prêt et cite PROJ-123 dès sa première ligne. Rien n'a
  changé dans Jira.
```

Le haut de la [proposition](../glossary.md#proposition) se lit alors ainsi :

```markdown
# Proposal: Ajouter un mode sombre

> Source: ticket **PROJ-123** — "Mode sombre pour le tableau de bord" (In Progress)

## Why
```

</div>

Toute demande à `/compagnon-propose` convient, pourvu qu'elle nomme un
ticket : « Planifie PROJ-123, et garde l'ancien thème en option » ajoute
vos propres mots à ceux du ticket.

## Mettre en place

Quelqu'un le met en place une fois par ordinateur, et `compagnon init` le
fait en général seul : il trouve votre connecteur Jira et écrit son nom
dans la configuration du projet. Si un nom de ticket est traité comme du
texte ordinaire, demandez à la personne qui a installé Compagnon de
regarder ci-dessous.

<details>
<summary>Sous le capot : la mise en place</summary>

Jira parvient à Claude Code par un serveur [MCP](../glossary.md#mcp) — un
connecteur vers un autre outil. La commande `compagnon` elle-même ne parle
jamais à Jira : c'est Claude qui appelle l'outil pendant votre session.
Compagnon a seulement besoin du nom de l'outil qui lit les tickets, dans la
configuration du projet :

```yaml
# _compagnon/config.yaml
mcp:
  jira_tool: mcp__atlassian__getJiraIssue
```

`compagnon init` le renseigne pour vous lorsqu'il trouve, dans `.mcp.json`,
`.claude/settings.json` ou `~/.claude.json`, un serveur dont le nom, la
commande ou l'URL mentionne Jira ou Atlassian :

```text
  ✓ Jira MCP detected: mcp__atlassian__getJiraIssue
    (source: .mcp.json → server "atlassian")
```

Le nom de l'outil suit la convention de Claude Code,
`mcp__<server name>__getJiraIssue`, où les espaces et les points du nom du
serveur sont remplacés par des tirets bas — par exemple, un connecteur nommé
`claude.ai Atlassian Rovo` donne
`mcp__claude_ai_Atlassian_Rovo__getJiraIssue`. Si le vôtre diffère, cherchez
le nom exact dans la liste des outils d'une session Claude Code et
renseignez-le à la main.

À l'installation, Compagnon ajoute l'outil aux `allowed-tools` de
`compagnon-propose` : la compétence peut ainsi l'appeler, et rien d'autre sur ce
serveur. Après avoir modifié `mcp:`, régénérez les compétences :

```bash
compagnon update --force
```

Sans `mcp.jira_tool`, un nom de ticket dans votre demande reste du texte
ordinaire : la compétence se comporte exactement comme dans un projet sans
intégration MCP.

> **Remarque**
> `mcp:` n'est jamais repris d'une [source héritée](inherited-sources.md) :
> les noms d'outils dépendent de l'installation de Claude Code de chacun, et
> non d'un dépôt partagé.

</details>

## Ce que Claude peut faire avec Jira

Un ticket, c'est toute chaîne correspondant à `[A-Z]{2,}-\d+`, comme
`PROJ-123`. Quand votre demande en nomme un et que l'outil est configuré,
`/compagnon-propose` s'en tient à ces règles :

- **Un seul appel**, à l'unique outil configuré, pour le premier ticket
  nommé.
- **Lecture seule** : jamais de recherche, de commentaire ni de changement
  de statut.
- **Le premier ticket seulement** : les autres tickets nommés sont listés
  sous la citation, pour que vous les consultiez.
- **Pas de connexion, pas de supposition** : si le serveur MCP n'est pas
  disponible dans la session, Claude le dit, rédige la proposition à
  partir de votre seule demande et cite le ticket avec la mention
  « contenu non récupéré ».

## Autres serveurs MCP

Jira est aujourd'hui la seule intégration. Lire une page Confluence ou un
outil de conception graphique n'est pas un réglage que vous pouvez
ajouter : chaque intégration fait partie de Compagnon lui-même, et en
ajouter une relève de la contribution. Consultez
[CONTRIBUTING.fr.md](https://github.com/mairistem/compagnon/blob/main/CONTRIBUTING.fr.md)
pour en proposer une.

<details>
<summary>Sous le capot : ajouter une autre intégration MCP</summary>

Chaque intégration consiste en une clé de configuration et un emplacement
réservé dans un [déroulé](../glossary.md#déroulé). Le principe est le même
que pour Jira :

1. Ajoutez une clé sous `mcp:` dans la configuration du projet
   (`crates/compagnon-engine/src/config.rs`).
2. Transmettez-la au contexte de rendu des compétences et substituez un
   emplacement réservé comme `{{JIRA_MCP_TOOL}}` à la fois dans les
   `allowed-tools` et dans le corps du déroulé qui l'utilise
   (`crates/compagnon-agents/src/claude.rs`).
3. Décrivez la détection et l'appel dans le fichier du déroulé sous
   `assets/workflows/`, avec un repli propre lorsque la clé est absente.

On enrichit la compétence existante plutôt que de la dupliquer : il n'existe
qu'une seule `/compagnon-propose`, quelle que soit l'origine de la demande.

</details>

## Étape suivante

[Relire un plan](review-a-plan.md) Lisez le plan qu'a écrit Claude avant que rien ne soit construit.
