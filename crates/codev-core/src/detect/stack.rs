//! Reconnaissance de la stack depuis les manifestes usuels.
//!
//! Chaque fonction prend les octets bruts d'un manifeste et retourne
//! `Some(Stack)` si elle a pu extraire quelque chose de suffisamment concret.
//! Les fonctions sont indépendantes — c'est l'orchestrateur qui choisit
//! quel manifeste tenter en premier.

use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stack {
    /// Langage primaire : "Rust", "JavaScript/TypeScript", "Python", "Go", "Java".
    pub language: String,
    /// Édition Rust, version langage, etc. — texte libre. `None` si non extrait.
    pub edition_or_version: Option<String>,
    /// Nombre de crates dans un workspace Cargo. `None` pour un projet simple
    /// ou pour un langage sans workspace.
    pub workspace_crate_count: Option<usize>,
    /// Aperçu court des dépendances principales — noms, sans versions.
    /// Coupé à 10 entrées.
    pub dependencies_summary: Vec<String>,
}

// ─────────────────────────────── Cargo.toml ───────────────────────────────

#[allow(clippy::question_mark)] // else-if let Some — plus lisible qu'un ? qui masquerait la branche workspace
pub fn from_cargo_toml(bytes: &[u8]) -> Option<Stack> {
    let text = std::str::from_utf8(bytes).ok()?;
    let doc: toml::Value = toml::from_str(text).ok()?;
    let table = doc.as_table()?;

    // Deux formes : `[workspace]` (workspace root) ou `[package]` (crate simple).
    let (edition, workspace_crate_count, dependencies) = if let Some(ws) = table.get("workspace") {
        let ws_table = ws.as_table()?;
        let edition = ws_table
            .get("package")
            .and_then(|p| p.as_table())
            .and_then(|p| p.get("edition"))
            .and_then(|e| e.as_str())
            .map(String::from);
        let members = ws_table
            .get("members")
            .and_then(|m| m.as_array())
            .map(|arr| arr.len());
        let deps = ws_table
            .get("dependencies")
            .and_then(|d| d.as_table())
            .map(dep_names)
            .unwrap_or_default();
        (edition, members, deps)
    } else if let Some(pkg) = table.get("package") {
        let edition = pkg
            .as_table()
            .and_then(|p| p.get("edition"))
            .and_then(|e| e.as_str())
            .map(String::from);
        let deps = table
            .get("dependencies")
            .and_then(|d| d.as_table())
            .map(dep_names)
            .unwrap_or_default();
        (edition, None, deps)
    } else {
        return None;
    };

    Some(Stack {
        language: "Rust".to_string(),
        edition_or_version: edition,
        workspace_crate_count,
        dependencies_summary: dependencies,
    })
}

fn dep_names(map: &toml::map::Map<String, toml::Value>) -> Vec<String> {
    map.keys().take(10).cloned().collect()
}

pub fn project_name_from_cargo_toml(bytes: &[u8]) -> Option<String> {
    let text = std::str::from_utf8(bytes).ok()?;
    let doc: toml::Value = toml::from_str(text).ok()?;
    doc.as_table()?
        .get("package")?
        .as_table()?
        .get("name")?
        .as_str()
        .map(String::from)
}

// ─────────────────────────────── package.json ───────────────────────────────

#[derive(Deserialize)]
struct PackageJson {
    name: Option<String>,
    dependencies: Option<serde_json::Map<String, serde_json::Value>>,
    #[serde(rename = "devDependencies")]
    dev_dependencies: Option<serde_json::Map<String, serde_json::Value>>,
    engines: Option<serde_json::Map<String, serde_json::Value>>,
}

pub fn from_package_json(bytes: &[u8]) -> Option<Stack> {
    let pkg: PackageJson = serde_json::from_slice(bytes).ok()?;

    // Distingue TypeScript / JavaScript par présence de `typescript` dans les
    // deps ou de `tsconfig.json` (le second est vérifié par l'orchestrateur).
    let all_deps: Vec<String> = pkg
        .dependencies
        .iter()
        .chain(pkg.dev_dependencies.iter())
        .flat_map(|m| m.keys().cloned())
        .collect();

    let language = if all_deps.iter().any(|n| n == "typescript") {
        "TypeScript"
    } else {
        "JavaScript"
    }
    .to_string();

    let node_version = pkg
        .engines
        .as_ref()
        .and_then(|e| e.get("node"))
        .and_then(|v| v.as_str())
        .map(String::from);

    Some(Stack {
        language,
        edition_or_version: node_version.map(|v| format!("Node {v}")),
        workspace_crate_count: None,
        dependencies_summary: all_deps.into_iter().take(10).collect(),
    })
}

pub fn project_name_from_package_json(bytes: &[u8]) -> Option<String> {
    serde_json::from_slice::<PackageJson>(bytes).ok()?.name
}

// ─────────────────────────────── pyproject.toml ───────────────────────────────

pub fn from_pyproject_toml(bytes: &[u8]) -> Option<Stack> {
    let text = std::str::from_utf8(bytes).ok()?;
    let doc: toml::Value = toml::from_str(text).ok()?;
    let table = doc.as_table()?;

    let project = table.get("project").and_then(|p| p.as_table());
    let python_version = project
        .and_then(|p| p.get("requires-python"))
        .and_then(|v| v.as_str())
        .map(String::from);
    let deps = project
        .and_then(|p| p.get("dependencies"))
        .and_then(|d| d.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str())
                .filter_map(|s| s.split(&['=', '<', '>', '~', '!'][..]).next())
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .take(10)
                .map(String::from)
                .collect()
        })
        .unwrap_or_default();

    Some(Stack {
        language: "Python".to_string(),
        edition_or_version: python_version.map(|v| format!("Python {v}")),
        workspace_crate_count: None,
        dependencies_summary: deps,
    })
}

pub fn project_name_from_pyproject_toml(bytes: &[u8]) -> Option<String> {
    let text = std::str::from_utf8(bytes).ok()?;
    let doc: toml::Value = toml::from_str(text).ok()?;
    doc.as_table()?
        .get("project")?
        .as_table()?
        .get("name")?
        .as_str()
        .map(String::from)
}

// ─────────────────────────────── go.mod ───────────────────────────────

pub fn from_go_mod(bytes: &[u8]) -> Option<Stack> {
    let text = std::str::from_utf8(bytes).ok()?;
    let go_version = text
        .lines()
        .find_map(|l| l.trim().strip_prefix("go "))
        .map(|v| v.trim().to_string());
    // La ligne `module <path>` est présente ; sinon ce n'est pas un go.mod.
    text.lines()
        .find(|l| l.trim().starts_with("module "))?;

    Some(Stack {
        language: "Go".to_string(),
        edition_or_version: go_version.map(|v| format!("Go {v}")),
        workspace_crate_count: None,
        dependencies_summary: Vec::new(),
    })
}

pub fn project_name_from_go_mod(bytes: &[u8]) -> Option<String> {
    let text = std::str::from_utf8(bytes).ok()?;
    text.lines()
        .find_map(|l| l.trim().strip_prefix("module "))
        .and_then(|m| m.trim().rsplit('/').next())
        .map(String::from)
}

// ─────────────────────────────── pom.xml ───────────────────────────────

pub fn from_pom_xml(bytes: &[u8]) -> Option<Stack> {
    let text = std::str::from_utf8(bytes).ok()?;
    // Détection minimale sans dépendance XML : on cherche <project> comme
    // marqueur, et on extrait la version Java si le tag est présent.
    if !text.contains("<project") {
        return None;
    }
    let java_version = extract_tag_content(text, "maven.compiler.source")
        .or_else(|| extract_tag_content(text, "java.version"));
    Some(Stack {
        language: "Java".to_string(),
        edition_or_version: java_version.map(|v| format!("Java {v}")),
        workspace_crate_count: None,
        dependencies_summary: Vec::new(),
    })
}

pub fn project_name_from_pom_xml(bytes: &[u8]) -> Option<String> {
    let text = std::str::from_utf8(bytes).ok()?;
    extract_tag_content(text, "artifactId")
}

fn extract_tag_content(text: &str, tag: &str) -> Option<String> {
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");
    let start = text.find(&open)? + open.len();
    let rel_end = text[start..].find(&close)?;
    Some(text[start..start + rel_end].trim().to_string())
}

// ─────────────────────────────── tests ───────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    const CARGO_WORKSPACE: &str = r#"
[workspace]
members = ["crates/a", "crates/b", "crates/c"]

[workspace.package]
edition = "2024"

[workspace.dependencies]
anyhow = "1"
serde = "1"
tokio = "1"
"#;

    const CARGO_PACKAGE: &str = r#"
[package]
name = "mon-projet"
edition = "2021"

[dependencies]
serde = "1"
"#;

    #[test]
    fn cargo_workspace_detecte_workspace_edition_et_deps() {
        let stack = from_cargo_toml(CARGO_WORKSPACE.as_bytes()).unwrap();
        assert_eq!(stack.language, "Rust");
        assert_eq!(stack.edition_or_version.as_deref(), Some("2024"));
        assert_eq!(stack.workspace_crate_count, Some(3));
        assert!(stack.dependencies_summary.contains(&"anyhow".to_string()));
    }

    #[test]
    fn cargo_package_detecte_projet_simple() {
        let stack = from_cargo_toml(CARGO_PACKAGE.as_bytes()).unwrap();
        assert_eq!(stack.language, "Rust");
        assert_eq!(stack.edition_or_version.as_deref(), Some("2021"));
        assert_eq!(stack.workspace_crate_count, None);
        assert_eq!(
            project_name_from_cargo_toml(CARGO_PACKAGE.as_bytes()).as_deref(),
            Some("mon-projet")
        );
    }

    #[test]
    fn cargo_illisible_retourne_none() {
        assert!(from_cargo_toml(b"pas du toml valide {{{").is_none());
    }

    const PACKAGE_TYPESCRIPT: &str = r#"{
        "name": "mon-app",
        "dependencies": {
            "react": "^18",
            "typescript": "^5"
        },
        "engines": { "node": ">=20" }
    }"#;

    const PACKAGE_JS_PUR: &str = r#"{
        "name": "old-app",
        "dependencies": { "lodash": "^4" }
    }"#;

    #[test]
    fn package_json_distingue_ts_de_js() {
        let ts = from_package_json(PACKAGE_TYPESCRIPT.as_bytes()).unwrap();
        assert_eq!(ts.language, "TypeScript");
        assert_eq!(ts.edition_or_version.as_deref(), Some("Node >=20"));
        assert!(ts.dependencies_summary.contains(&"react".to_string()));

        let js = from_package_json(PACKAGE_JS_PUR.as_bytes()).unwrap();
        assert_eq!(js.language, "JavaScript");
        assert_eq!(js.edition_or_version, None);

        assert_eq!(
            project_name_from_package_json(PACKAGE_TYPESCRIPT.as_bytes()).as_deref(),
            Some("mon-app")
        );
    }

    const PYPROJECT: &str = r#"
[project]
name = "myproj"
requires-python = ">=3.11"
dependencies = ["fastapi>=0.100", "pydantic~=2.0"]
"#;

    #[test]
    fn pyproject_detecte_python_et_deps() {
        let s = from_pyproject_toml(PYPROJECT.as_bytes()).unwrap();
        assert_eq!(s.language, "Python");
        assert_eq!(s.edition_or_version.as_deref(), Some("Python >=3.11"));
        assert!(s.dependencies_summary.contains(&"fastapi".to_string()));
        assert!(s.dependencies_summary.contains(&"pydantic".to_string()));
        assert_eq!(
            project_name_from_pyproject_toml(PYPROJECT.as_bytes()).as_deref(),
            Some("myproj")
        );
    }

    const GO_MOD: &str = r#"module github.com/user/monrepo

go 1.22

require (
    github.com/spf13/cobra v1.8.0
)
"#;

    #[test]
    fn go_mod_detecte_version_et_nom() {
        let s = from_go_mod(GO_MOD.as_bytes()).unwrap();
        assert_eq!(s.language, "Go");
        assert_eq!(s.edition_or_version.as_deref(), Some("Go 1.22"));
        assert_eq!(project_name_from_go_mod(GO_MOD.as_bytes()).as_deref(), Some("monrepo"));
    }

    const POM_XML: &str = r#"<?xml version="1.0"?>
<project>
  <artifactId>mon-service</artifactId>
  <properties>
    <maven.compiler.source>17</maven.compiler.source>
  </properties>
</project>
"#;

    #[test]
    fn pom_xml_detecte_java_et_artifact_id() {
        let s = from_pom_xml(POM_XML.as_bytes()).unwrap();
        assert_eq!(s.language, "Java");
        assert_eq!(s.edition_or_version.as_deref(), Some("Java 17"));
        assert_eq!(project_name_from_pom_xml(POM_XML.as_bytes()).as_deref(), Some("mon-service"));
    }
}
