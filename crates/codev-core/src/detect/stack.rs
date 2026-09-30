//! Stack identification from the usual manifests.
//!
//! Each function takes the raw bytes of a manifest and returns
//! `Some(Stack)` if it could extract something concrete enough.
//! The functions are independent — the orchestrator decides
//! which manifest to try first.

use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stack {
    /// Primary language: "Rust", "JavaScript/TypeScript", "Python", "Go", "Java".
    pub language: String,
    /// Rust edition, language version, etc. — free-form text. `None` if not extracted.
    pub edition_or_version: Option<String>,
    /// Number of crates in a Cargo workspace. `None` for a single-crate project
    /// or for a language without workspaces.
    pub workspace_crate_count: Option<usize>,
    /// Short overview of the main dependencies — names, without versions.
    /// Capped at 10 entries.
    pub dependencies_summary: Vec<String>,
}

// ─────────────────────────────── Cargo.toml ───────────────────────────────

#[allow(clippy::question_mark)] // else-if let Some — more readable than a ? that would hide the workspace branch
pub fn from_cargo_toml(bytes: &[u8]) -> Option<Stack> {
    let text = std::str::from_utf8(bytes).ok()?;
    let doc: toml::Value = toml::from_str(text).ok()?;
    let table = doc.as_table()?;

    // Two shapes: `[workspace]` (workspace root) or `[package]` (single crate).
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

    // Tells TypeScript from JavaScript by the presence of `typescript` in the
    // deps or of `tsconfig.json` (the latter is checked by the orchestrator).
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
    // The `module <path>` line must be present; otherwise this is not a go.mod.
    text.lines().find(|l| l.trim().starts_with("module "))?;

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
    // Minimal detection without an XML dependency: look for <project> as a
    // marker, and extract the Java version if the tag is present.
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
name = "my-project"
edition = "2021"

[dependencies]
serde = "1"
"#;

    #[test]
    fn cargo_workspace_detects_workspace_edition_and_deps() {
        let stack = from_cargo_toml(CARGO_WORKSPACE.as_bytes()).unwrap();
        assert_eq!(stack.language, "Rust");
        assert_eq!(stack.edition_or_version.as_deref(), Some("2024"));
        assert_eq!(stack.workspace_crate_count, Some(3));
        assert!(stack.dependencies_summary.contains(&"anyhow".to_string()));
    }

    #[test]
    fn cargo_package_detects_single_crate_project() {
        let stack = from_cargo_toml(CARGO_PACKAGE.as_bytes()).unwrap();
        assert_eq!(stack.language, "Rust");
        assert_eq!(stack.edition_or_version.as_deref(), Some("2021"));
        assert_eq!(stack.workspace_crate_count, None);
        assert_eq!(
            project_name_from_cargo_toml(CARGO_PACKAGE.as_bytes()).as_deref(),
            Some("my-project")
        );
    }

    #[test]
    fn unreadable_cargo_returns_none() {
        assert!(from_cargo_toml(b"not valid toml {{{").is_none());
    }

    const PACKAGE_TYPESCRIPT: &str = r#"{
        "name": "my-app",
        "dependencies": {
            "react": "^18",
            "typescript": "^5"
        },
        "engines": { "node": ">=20" }
    }"#;

    const PACKAGE_PLAIN_JS: &str = r#"{
        "name": "old-app",
        "dependencies": { "lodash": "^4" }
    }"#;

    #[test]
    fn package_json_tells_ts_from_js() {
        let ts = from_package_json(PACKAGE_TYPESCRIPT.as_bytes()).unwrap();
        assert_eq!(ts.language, "TypeScript");
        assert_eq!(ts.edition_or_version.as_deref(), Some("Node >=20"));
        assert!(ts.dependencies_summary.contains(&"react".to_string()));

        let js = from_package_json(PACKAGE_PLAIN_JS.as_bytes()).unwrap();
        assert_eq!(js.language, "JavaScript");
        assert_eq!(js.edition_or_version, None);

        assert_eq!(
            project_name_from_package_json(PACKAGE_TYPESCRIPT.as_bytes()).as_deref(),
            Some("my-app")
        );
    }

    const PYPROJECT: &str = r#"
[project]
name = "myproj"
requires-python = ">=3.11"
dependencies = ["fastapi>=0.100", "pydantic~=2.0"]
"#;

    #[test]
    fn pyproject_detects_python_and_deps() {
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

    const GO_MOD: &str = r#"module github.com/user/myrepo

go 1.22

require (
    github.com/spf13/cobra v1.8.0
)
"#;

    #[test]
    fn go_mod_detects_version_and_name() {
        let s = from_go_mod(GO_MOD.as_bytes()).unwrap();
        assert_eq!(s.language, "Go");
        assert_eq!(s.edition_or_version.as_deref(), Some("Go 1.22"));
        assert_eq!(
            project_name_from_go_mod(GO_MOD.as_bytes()).as_deref(),
            Some("myrepo")
        );
    }

    const POM_XML: &str = r#"<?xml version="1.0"?>
<project>
  <artifactId>my-service</artifactId>
  <properties>
    <maven.compiler.source>17</maven.compiler.source>
  </properties>
</project>
"#;

    #[test]
    fn pom_xml_detects_java_and_artifact_id() {
        let s = from_pom_xml(POM_XML.as_bytes()).unwrap();
        assert_eq!(s.language, "Java");
        assert_eq!(s.edition_or_version.as_deref(), Some("Java 17"));
        assert_eq!(
            project_name_from_pom_xml(POM_XML.as_bytes()).as_deref(),
            Some("my-service")
        );
    }
}
