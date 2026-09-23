//! `codev docs` : la documentation embarquée, rendue en HTML autonome.
//!
//! Trois formes :
//! - défaut : écrit dans `temp_dir()/codev-docs-<version>.html` et
//!   ouvre dans le navigateur système ;
//! - `--write <PATH>` : écrit au chemin donné, n'ouvre pas ;
//! - `--print` : imprime le markdown source sur stdout.
//!
//! Le HTML est **autonome** — CSS inline, pas de dépendance à un CDN,
//! pas de JS obligatoire, ouvrable hors ligne. Alignement avec la
//! philosophie codev : le binaire embarque ce qu'il faut pour
//! fonctionner sans supposer un accès réseau à l'exécution.

use std::io;
use std::path::{Path, PathBuf};

/// Le markdown source de la doc. Embarqué au build via `include_str!` :
/// la doc suit toujours la version du binaire, aucune dépendance à un
/// fichier externe à l'exécution.
pub const MARKDOWN_SOURCE: &str = include_str!("../../../docs/codev.md");

/// Le CSS embarqué, injecté inline dans chaque HTML rendu.
const CSS: &str = include_str!("../assets/docs.css");

/// Convertit le markdown source en HTML autonome, entoure du template
/// HTML5 (doctype, `<head>`, CSS inline, `<header>` version).
pub fn render_html(md: &str, version: &str) -> String {
    use pulldown_cmark::{html, Options, Parser};

    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);

    let parser = Parser::new_ext(md, options);
    let mut body_html = String::new();
    html::push_html(&mut body_html, parser);

    format!(
        "<!doctype html>\n\
         <html lang=\"fr\">\n\
         <head>\n\
         <meta charset=\"utf-8\">\n\
         <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n\
         <title>codev — Documentation</title>\n\
         <style>\n{css}\n</style>\n\
         </head>\n\
         <body>\n\
         <header>\n\
         <span class=\"title\">codev</span>\n\
         <span class=\"version\">v{version}</span>\n\
         </header>\n\
         <main>\n\
         {body_html}\n\
         </main>\n\
         </body>\n\
         </html>\n",
        css = CSS,
        version = version,
        body_html = body_html,
    )
}

/// Chemin par défaut où écrire le HTML — dans le répertoire système
/// temporaire, avec la version dans le nom pour distinguer plusieurs
/// binaires installés côte à côte.
pub fn default_output_path(version: &str) -> PathBuf {
    std::env::temp_dir().join(format!("codev-docs-{version}.html"))
}

/// Écrit le HTML au chemin donné. Le parent du chemin doit exister —
/// on ne crée pas d'arborescence profonde à la place de l'utilisateur.
pub fn write_to(path: &Path, version: &str) -> io::Result<()> {
    let html = render_html(MARKDOWN_SOURCE, version);
    std::fs::write(path, html)
}

/// Écrit dans `default_output_path(version)` puis délègue à
/// `open::that(...)` pour ouvrir dans le navigateur système.
///
/// Retourne le chemin écrit — le CLI l'affiche à l'utilisateur pour
/// qu'il sache où le trouver s'il veut le re-partager.
pub fn open_default(version: &str) -> io::Result<PathBuf> {
    let path = default_output_path(version);
    write_to(&path, version)?;
    open::that(&path)?;
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn markdown_source_est_non_vide() {
        // Traceur d'un include_str! qui pointerait sur un fichier vide.
        assert!(
            MARKDOWN_SOURCE.len() > 1000,
            "MARKDOWN_SOURCE ne fait que {} octets — la doc semble vide",
            MARKDOWN_SOURCE.len()
        );
        assert!(
            MARKDOWN_SOURCE.starts_with("# codev"),
            "MARKDOWN_SOURCE doit commencer par le titre"
        );
    }

    #[test]
    fn render_html_produit_un_html5_autonome() {
        let out = render_html("# Titre\n\nParagraphe.", "9.9.9");
        assert!(out.starts_with("<!doctype html>"));
        assert!(out.contains("codev"));
        assert!(out.contains("v9.9.9"));
        assert!(out.contains("<h1>Titre</h1>"));
        assert!(out.contains("<p>Paragraphe.</p>"));
        // Autonomie : aucune référence à un URL externe (http://, https://).
        // On tolère quand même les URL dans le corps de la doc rendu — le
        // test cible spécifiquement les balises `<link>` et `<script src>`.
        assert!(
            !out.contains("<link href=\"http"),
            "aucun <link> externe attendu (autonomie du HTML)"
        );
        assert!(
            !out.contains("<script src=\"http"),
            "aucun <script src> externe attendu"
        );
    }

    #[test]
    fn render_html_gere_tables_et_code() {
        let md = "\
| A | B |\n\
|---|---|\n\
| 1 | 2 |\n\
\n\
```\n\
let x = 1;\n\
```\n";
        let out = render_html(md, "0.1.0");
        assert!(out.contains("<table>"));
        assert!(out.contains("<code>"));
    }

    #[test]
    fn write_to_ecrit_le_fichier_html() {
        let dir = std::env::temp_dir().join("codev-docs-test-write");
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("docs.html");
        write_to(&path, "0.1.0").unwrap();
        let content = std::fs::read_to_string(&path).unwrap();
        assert!(content.starts_with("<!doctype html>"));
        assert!(content.contains("codev"));
        // Nettoyage.
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn default_output_path_porte_la_version_dans_le_nom() {
        let p = default_output_path("1.2.3");
        let name = p.file_name().unwrap().to_string_lossy();
        assert_eq!(name, "codev-docs-1.2.3.html");
    }
}
