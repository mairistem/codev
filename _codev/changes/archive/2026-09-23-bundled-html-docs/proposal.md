# Proposal : documentation codev en HTML autonome, ouvrable via `codev docs`

## Pourquoi

L'aide CLI (`codev --help`, `codev <cmd> --help`) donne la surface —
chaque sous-commande, chaque flag. Elle **ne donne pas** :

- une vue d'ensemble du cycle **propose → apply → sync → archive** ;
- les concepts (capacités, décisions, delta operations, sceau K3,
  dérive K6, promotion K7) ;
- les procédures d'installation (cargo install, completions shell,
  configuration MCP) ;
- une doc à partager par mail, Slack ou Confluence pour convaincre
  ou onboarder un collègue.

Il manque **un document unique et diffusable** que codev peut ouvrir
lui-même. C'est utile aux futurs utilisateurs JVS qui arriveront, et
tout de suite utile à Ludovic pour socialiser l'outil.

## Ce qui change

- **Nouveau fichier `docs/codev.md`** — source de vérité de la
  documentation, versionné dans le dépôt. Structure à ~10 sections,
  couvre du « qu'est-ce que codev » aux « décisions d'architecture »
  et « extensions MCP ». Rédaction manuelle, ton aligné sur les
  proposals/designs qu'on écrit depuis 15 changes.
- **Nouvelle dépendance `pulldown-cmark = "0.10"`** dans
  `crates/codev-cli/Cargo.toml` — convertisseur markdown → HTML
  standard de facto en Rust, pas de JS runtime, pas d'assets externes.
- **Nouvelle dépendance `open = "5"`** — mini-crate qui délègue à
  `open` (macOS), `xdg-open` (Linux), `start` (Windows) pour ouvrir
  un fichier dans l'app par défaut.
- **Embed du markdown** dans le binaire via
  `include_str!("../../docs/codev.md")` — la doc est **toujours
  synchronisée** avec la version du binaire, sans dépendance à un
  fichier externe.
- **Nouvelle sous-commande `codev docs`** avec trois formes :
  - `codev docs` (défaut) — écrit un fichier
    `codev-docs-<version>.html` dans le répertoire système temporaire
    (`std::env::temp_dir()`) et l'ouvre dans le navigateur système.
    Le HTML embarque son CSS et est autonome.
  - `codev docs --write <PATH>` — écrit le HTML au chemin donné et
    n'ouvre rien. Utile pour diffusion ciblée (`--write ./docs.html`
    dans un share drive).
  - `codev docs --print` — imprime le **markdown** brut sur stdout
    (pour piper vers `less`, `bat`, ou un LLM qui préfère du texte).
- **Un CSS minimal maison** de ~120 lignes — style techdoc lisible
  (Times/Georgia pour le body, sans-serif pour headings, code en
  monospace, sommaire à gauche sur desktop). Pas de framework, pas de
  CDN, autonomie stricte.

## Capacités

### Nouvelles capacités

- `docs` — décrit le contrat de la sous-commande `codev docs` :
  formats de sortie, autonomie du HTML, chemins de sortie, absence
  de dépendance à un dépôt initialisé.

### Capacités modifiées

Aucune.

### Capacités retirées

Aucune.

## Impact

- **Code** :
  - Nouveau fichier `docs/codev.md` (versionné).
  - Nouveau module `codev-cli::docs` qui expose :
    - `MARKDOWN_SOURCE: &str` (l'embed).
    - `fn render_html(md: &str, version: &str) -> String` : convertit
      via `pulldown-cmark`, entoure d'un template HTML avec CSS
      inline.
    - `fn open_default(version: &str) -> io::Result<PathBuf>` :
      écrit dans `temp_dir()` + `open::that()`.
    - `fn write_to(path: &Path, version: &str) -> io::Result<()>`.
  - Nouvelle variante `Command::Docs { print: bool, write:
    Option<PathBuf> }` dans `main.rs`.
- **Contrat JSON** : rien. La sous-commande sort du contrat global
  (comme `completions`) — le HTML n'est pas du JSON structuré, et un
  `--json` n'aurait aucun sens.
- **Fichier écrit** : uniquement le HTML au chemin choisi
  (`temp_dir()` par défaut, ou `--write <PATH>`). Aucune écriture
  dans `_codev/`, aucune écriture système. Idempotent : ré-appeler
  écrase le fichier.
- **Migration** : aucune. Feature purement additive.
- **Hors périmètre** :
  - **PDF** — deux formats à maintenir alors qu'un seul suffit pour
    la V1. Reportable si un vrai besoin apparaît (impression
    formelle).
  - **Recherche full-text JS** — Ctrl+F du navigateur est déjà là.
    Reportable si la doc devient longue.
  - **Traduction / i18n** — un seul français pour la V1.
  - **Extraction auto depuis clap** pour la section CLI — la
    différence rédactionnelle vaut la maintenance manuelle. Reportable.
  - **Génération au build via `build.rs`** — la conversion runtime
    coûte quelques millisecondes, indolore. Le build reste simple.
