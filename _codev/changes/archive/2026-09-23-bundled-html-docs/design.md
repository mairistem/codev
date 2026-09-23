# Design : `codev docs` et son bundle HTML autonome

## Contexte

Voir `proposal.md`. Doc bundle qui répond aux trois questions
tranchées par défaut :

- **Génération** : runtime, pas build-time (build reste simple).
- **Design visuel** : maison minimal, aucun CDN.
- **Contenu** : manuel, rédigé avec le même ton que les artefacts
  codev.

## Objectifs / Hors objectifs

Ce design cadre la conversion markdown → HTML, l'ouverture système,
les trois formes de la sous-commande, l'embed du markdown, et le
squelette CSS. Il ne cadre pas la génération PDF, la recherche
full-text JS, ni la génération de la section CLI depuis clap.

## Décisions

### Décision : conversion **runtime** via `pulldown-cmark`

Deux options :

| Option | Pro | Contre |
|---|---|---|
| **A. Build-time via `build.rs`** — le HTML est embedded prêt à l'usage | Coût zéro à l'exécution, HTML précalculé | Build cache invalidé à chaque édition de la doc, `build.rs` supplémentaire à maintenir |
| **B. Runtime — le markdown est embedded, conversion au moment de l'appel** | Build reste simple, cache des sources markdown identique à celui des autres `include_str!` | ~5-10 ms de conversion à chaque `codev docs` |

**Choisi : B.** Les 5-10 ms sont indolores devant l'ouverture du
navigateur (~500 ms). Le build reste homogène — pas de `build.rs`
qui devient un endroit spécial pour une seule chose.

### Décision : `pulldown-cmark` version 0.10, pas de renderer custom

`pulldown-cmark` est le converter markdown → HTML mainstream en
Rust : utilisé par mdBook, docs.rs, GitBook alternatives. Son
`html::push_html` gère les six éléments dont on a besoin (titres,
paragraphes, listes, tableaux, code, blockquotes) avec les
extensions **tables** et **footnotes** activées.

**Alternative écartée** : `comrak` (compatible CommonMark strict).
Plus lourd, moins nécessaire.

### Décision : template HTML minimal maison, pas de framework

Un template de 150 lignes environ, avec :

- Doctype HTML5.
- `<meta charset="utf-8">` et viewport mobile.
- `<style>` inline avec ~120 lignes de CSS.
- `<header>` : nom + version.
- `<nav>` : sommaire auto-généré à partir des `h2`/`h3` (via JS de 30
  lignes ; sans JS, le sommaire n'apparaît pas mais la doc reste
  navigable via Ctrl+F).
- `<main>` : le contenu.
- **Aucun** `<link>` externe.

**CSS style** : typographie techdoc, `system-ui` pour les headings,
`Georgia`/`serif` pour le body (lecture confortable sur écran),
`ui-monospace` pour code. Palette monochrome (contrastes suffisants,
imprimable).

**Alternative écartée** : Tailwind Play CDN. Casse l'autonomie
(première ouverture demande internet).

### Décision : trois formes exclusives — défaut, `--print`, `--write`

Résolution ordonnée par précédence :

1. `--print` seul → markdown sur stdout, rien d'autre.
2. `--write <PATH>` → HTML au chemin donné, pas d'ouverture.
3. Défaut (aucun flag) → HTML dans `temp_dir()` + ouverture.

Les combinaisons `--print --write` sont refusées par clap
(`conflicts_with`).

### Décision : chemin par défaut = `temp_dir()/codev-docs-<version>.html`

Ré-appeler `codev docs` écrase le fichier de la session précédente.
Idempotent. Un nom de fichier qui porte la version distingue les
versions installées côte à côte (rare mais possible).

**Alternative écartée** : `~/.local/share/codev/docs.html`. Persistant
mais pollue le `$HOME`, qui n'est pas le rôle d'une commande docs.

### Décision : ouverture système via la crate `open`

`open = "5"` fait exactement une chose : `open::that(path)` → délègue
à `open` / `xdg-open` / `start`. 200 lignes de code source, zéro
dépendance runtime.

**Alternative écartée** : `std::process::Command::new("open").arg(…)`
maison. Marche sur macOS mais casse sur Linux/Windows. La crate
absorbe ces différences pour trois fois rien.

### Décision : contenu manuel, dix sections

Plan de la doc, à rédiger dans `docs/codev.md` :

1. **Introduction** — qu'est-ce que codev, pourquoi ce cycle.
2. **Installation** — cargo install, PATH, `~/.cargo/bin`, completions.
3. **Le cycle** — propose → apply → sync → archive, schéma ASCII.
4. **Les 7 skills Claude Code** — propose, explore, apply, sync,
   archive, update, onboard. Chacune : rôle, `allowed-tools`,
   quand l'utiliser.
5. **Les concepts** — capacité, décision, change, delta operation
   (`ADDED`, `MODIFIED`, `REMOVED`, `RENAMED`).
6. **La CLI** — chaque commande avec un exemple, groupée par famille
   (projet, changes, décisions, sources, validation, docs).
7. **Configuration** — `_codev/config.yaml` : schéma, workflows,
   context, rules, inherits, mcp.
8. **Décisions d'architecture** — les 6 ADR racines, K3/K6/K7.
9. **Extensions MCP** — Jira/Atlassian aujourd'hui, comment ajouter
   d'autres MCP.
10. **FAQ / troubleshooting** — trous mémoire courants, cas
    d'échec.

Un ton unique aligné sur les artefacts qu'on écrit depuis 16 changes.

## Risques et compromis

- **Un utilisateur sur un système où `open`/`xdg-open`/`start`
  n'existe pas** — cas de bord (serveurs headless). La crate `open`
  remonte l'erreur ; la commande imprime « impossible d'ouvrir le
  navigateur — utilise `--print` ou `--write`. »
- **Le HTML fait 100-300 Ko une fois embarqué avec le CSS et la
  doc** — mesurable au premier build, acceptable devant un binaire
  qui pèse déjà ~4 Mo.
- **Les futures évolutions de la doc casseront les tests golden**
  éventuels — c'est **le point**. Un test golden serait remplacé par
  des tests structurels : « le HTML commence par `<!doctype html>` »,
  « il ne cite aucun URL externe », « il contient le nom `codev` ».

## Plan de migration

Aucune. Feature additive, opt-in.
