# Tâches

## 1. Dépendances

- [x] 1.1 Ajouter `pulldown-cmark = "0.10"` aux dépendances de
      `crates/codev-cli/Cargo.toml`, avec la feature par défaut
      suffisante (tables, footnotes intégrés).
- [x] 1.2 Ajouter `open = "5"` aux dépendances de
      `crates/codev-cli/Cargo.toml`.

## 2. Source de la documentation

- [x] 2.1 Créer `docs/codev.md` — le markdown source de la doc,
      versionné.
- [x] 2.2 Rédiger les 10 sections définies dans le design :
      Introduction, Installation, Le cycle, Les 7 skills, Les
      concepts, La CLI, Configuration, Décisions d'architecture,
      Extensions MCP, FAQ. Longueur cible : 2000-3000 mots.
- [x] 2.3 Ton et style : aligné sur les proposals/designs qu'on
      écrit depuis 16 changes. Français, phrases claires, pas
      d'anglicismes gratuits.

## 3. Module `codev-cli::docs`

- [x] 3.1 Créer `crates/codev-cli/src/docs.rs` avec :
      - `const MARKDOWN_SOURCE: &str = include_str!("../../../docs/codev.md");`
      - `const CSS: &str = include_str!("../assets/docs.css");`
      - `pub fn render_html(md: &str, version: &str) -> String` :
        appelle `pulldown_cmark::Parser::new_ext(md, Options::ENABLE_TABLES | Options::ENABLE_FOOTNOTES)`,
        pousse dans `html::push_html`, entoure du template HTML5.
      - `pub fn default_output_path(version: &str) -> PathBuf` :
        `std::env::temp_dir().join(format!("codev-docs-{version}.html"))`.
      - `pub fn write_to(path: &Path, version: &str) -> io::Result<()>` :
        rend + `std::fs::write` (crée le parent si son parent
        existe, refuse sinon).
      - `pub fn open_default(version: &str) -> io::Result<PathBuf>` :
        `write_to(default_output_path, version)` + `open::that(path)`.
- [x] 3.2 Créer `crates/codev-cli/assets/docs.css` — ~120 lignes de
      CSS techdoc autonome (typographie, code monospace, tableaux
      lisibles, palette monochrome).
- [x] 3.3 Ajouter `pub mod docs;` à `crates/codev-cli/src/main.rs`.

## 4. Sous-commande CLI

- [x] 4.1 Ajouter la variante `Command::Docs { print: bool, write:
      Option<PathBuf> }` à l'enum `Command` dans `main.rs`. Les
      deux flags sont mutuellement exclusifs (`conflicts_with`).
- [x] 4.2 Doc-comment du variant : « Ouvre la documentation codev
      dans le navigateur, ou l'écrit à un chemin donné, ou imprime
      le markdown source sur stdout. »
- [x] 4.3 Ajouter le bras du match dans `run(cli)` :
      ```rust
      Command::Docs { print, write } => {
          if print {
              use std::io::Write;
              let _ = std::io::stdout().write_all(docs::MARKDOWN_SOURCE.as_bytes());
              return 0;
          }
          if let Some(path) = write {
              match docs::write_to(&path, VERSION) {
                  Ok(()) => 0,
                  Err(e) => { eprintln!("écriture impossible : {e}"); 1 }
              }
          } else {
              match docs::open_default(VERSION) {
                  Ok(p) => { eprintln!("Ouvert : {}", p.display()); 0 }
                  Err(e) => {
                      eprintln!(
                          "impossible d'ouvrir le navigateur : {e}\n\
                           essaie `codev docs --print` ou `codev docs --write <PATH>`"
                      );
                      1
                  }
              }
          }
      }
      ```

## 5. Tests

- [x] 5.1 Test unitaire `docs::render_html_produit_un_html5_complet` :
      appelle `render_html("# Titre\n\nParagraphe.", "0.1.0")`,
      vérifie que la sortie commence par `<!doctype html>`,
      contient le titre `codev`, la version, le paragraphe, et
      **aucune** URL `http` (autonomie).
- [x] 5.2 Test `docs::render_html_gere_tables_et_code` : entrée
      markdown avec un tableau et un bloc de code, vérifie que la
      sortie contient `<table>` et `<code>`.
- [x] 5.3 Test `docs::write_to_ecrit_le_fichier` : dans un
      `tempdir`, appelle `write_to`, vérifie que le fichier existe
      et contient bien du HTML.
- [x] 5.4 Test `docs::markdown_source_est_non_vide` : vérifie que
      `MARKDOWN_SOURCE.len() > 1000` (traceur d'un embed cassé).

## 6. Doc CLI

- [x] 6.1 Le `--help` de `codev docs` mentionne les trois formes,
      avec un exemple par forme.
- [x] 6.2 Le README du dépôt mentionne `codev docs` avec une
      phrase et un lien vers `docs/codev.md`.

## 7. Dogfooding et intégration workspace

- [x] 7.1 Après `cargo install --path crates/codev-cli`, lancer
      `codev docs --print | head -30` — vérifier de visu que la
      doc est cohérente.
- [x] 7.2 Lancer `codev docs --write /tmp/manuel.html`, ouvrir le
      fichier dans un navigateur (macOS `open
      /tmp/manuel.html`), vérifier le rendu (styles, sommaire si
      JS, contenu).
- [x] 7.3 Lancer `codev docs` (sans argument) et vérifier que le
      navigateur s'ouvre effectivement sur le fichier temporaire.
- [x] 7.4 `cargo test --workspace` reste vert, gagne au moins 4
      tests dédiés.
- [x] 7.5 `cargo clippy --workspace --all-targets` reste sans
      avertissement.
- [x] 7.6 `codev validate --strict` sur ce dépôt reste vert.
