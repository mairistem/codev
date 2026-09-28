//! Orchestrateur de la sonde `codev init`.
//!
//! Coordonne l'accès disque (via `FileSystem`) et l'accès environnement
//! (`Env` — pour `~/.claude.json`). Chaque champ est best-effort : un
//! fichier absent n'est pas une erreur, il produit un warning silencieux.
//!
//! Les fonctions pures qui parsent chaque manifeste vivent dans
//! `codev-core::detect` — elles sont testables sans I/O. Ici on se
//! contente d'appeler les bons ports dans le bon ordre.

use std::path::{Path, PathBuf};

use codev_core::detect::{
    license, mcp,
    stack::{self, Stack},
    Detected,
};

use crate::ports::{Env, FileSystem};

/// Ordre de recherche de manifestes de stack. Le premier trouvé fixe la
/// stack primaire, les autres sont ignorés.
const MANIFESTS: &[(&str, StackKind)] = &[
    ("Cargo.toml", StackKind::Cargo),
    ("package.json", StackKind::PackageJson),
    ("pyproject.toml", StackKind::Pyproject),
    ("go.mod", StackKind::GoMod),
    ("pom.xml", StackKind::PomXml),
];

/// Ordre de recherche des configs MCP. Le premier gagne en cas de doublon
/// de nom de serveur (config projet gagne sur config utilisateur globale).
fn mcp_sources(project_root: &Path, home: Option<&Path>) -> Vec<(PathBuf, String)> {
    let mut out = vec![
        (project_root.join(".mcp.json"), ".mcp.json".to_string()),
        (
            project_root.join(".claude").join("settings.json"),
            ".claude/settings.json".to_string(),
        ),
        (
            project_root.join(".claude").join("settings.local.json"),
            ".claude/settings.local.json".to_string(),
        ),
    ];
    if let Some(h) = home {
        out.push((h.join(".claude.json"), "~/.claude.json".to_string()));
    }
    out
}

enum StackKind {
    Cargo,
    PackageJson,
    Pyproject,
    GoMod,
    PomXml,
}

/// Sonde le dossier courant et retourne un rapport `Detected`.
///
/// - `project_root` — la racine du projet inspecté.
/// - `env` — pour résoudre `~/.claude.json` via `HOME` / `USERPROFILE`.
pub fn run(fs: &dyn FileSystem, env: &dyn Env, project_root: &Path) -> Detected {
    let home = env.home_dir();

    let (stack, project_name) = detect_stack_and_name(fs, project_root);
    let license = detect_license(fs, project_root);
    let has_ci = detect_ci(fs, project_root);
    let is_git_repo = fs.exists(&project_root.join(".git"));
    let mcps = detect_mcps(fs, project_root, home.as_deref());

    Detected {
        stack,
        project_name,
        license,
        has_ci,
        is_git_repo,
        mcps,
    }
}

fn detect_stack_and_name(
    fs: &dyn FileSystem,
    project_root: &Path,
) -> (Option<Stack>, Option<String>) {
    for (filename, kind) in MANIFESTS {
        let path = project_root.join(filename);
        if !fs.exists(&path) {
            continue;
        }
        let Ok(content) = fs.read_to_string(&path) else {
            continue;
        };
        let bytes = content.as_bytes();
        let stack = match kind {
            StackKind::Cargo => stack::from_cargo_toml(bytes),
            StackKind::PackageJson => stack::from_package_json(bytes),
            StackKind::Pyproject => stack::from_pyproject_toml(bytes),
            StackKind::GoMod => stack::from_go_mod(bytes),
            StackKind::PomXml => stack::from_pom_xml(bytes),
        };
        if stack.is_some() {
            let name = match kind {
                StackKind::Cargo => stack::project_name_from_cargo_toml(bytes),
                StackKind::PackageJson => stack::project_name_from_package_json(bytes),
                StackKind::Pyproject => stack::project_name_from_pyproject_toml(bytes),
                StackKind::GoMod => stack::project_name_from_go_mod(bytes),
                StackKind::PomXml => stack::project_name_from_pom_xml(bytes),
            };
            return (stack, name);
        }
    }
    (None, None)
}

fn detect_license(fs: &dyn FileSystem, project_root: &Path) -> Option<String> {
    for candidate in ["LICENSE", "LICENSE.md", "LICENSE.txt", "COPYING"] {
        let path = project_root.join(candidate);
        if let Ok(content) = fs.read_to_string(&path)
            && let Some(license) = license::identify(content.as_bytes())
        {
            return Some(license);
        }
    }
    None
}

fn detect_ci(fs: &dyn FileSystem, project_root: &Path) -> bool {
    let workflows = project_root.join(".github").join("workflows");
    fs.list_dir(&workflows)
        .map(|entries| !entries.is_empty())
        .unwrap_or(false)
}

fn detect_mcps(
    fs: &dyn FileSystem,
    project_root: &Path,
    home: Option<&Path>,
) -> Vec<mcp::DetectedMcp> {
    let mut per_source: Vec<Vec<mcp::DetectedMcp>> = Vec::new();
    for (path, label) in mcp_sources(project_root, home) {
        if let Ok(content) = fs.read_to_string(&path) {
            per_source.push(mcp::parse_mcp_config(content.as_bytes(), &label));
        }
    }
    mcp::merge_first_wins(per_source)
}

// ─────────────────────────────── tests ───────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ports::MemoryFileSystem;
    use std::collections::BTreeMap;

    struct FakeEnv {
        vars: BTreeMap<String, String>,
    }

    impl FakeEnv {
        fn with_home(home: &str) -> Self {
            let mut vars = BTreeMap::new();
            vars.insert("HOME".to_string(), home.to_string());
            Self { vars }
        }
    }

    impl Env for FakeEnv {
        fn current_dir(&self) -> std::io::Result<PathBuf> {
            Ok(PathBuf::from("/p"))
        }
        fn var(&self, key: &str) -> Option<String> {
            self.vars.get(key).cloned()
        }
    }

    const CARGO_WS: &str = r#"
[workspace]
members = ["a", "b", "c", "d"]
[workspace.package]
edition = "2024"
[workspace.dependencies]
serde = "1"
"#;

    const MCP_PROJET: &str = r#"{
        "mcpServers": {
            "claude.ai Atlassian Rovo": { "url": "https://mcp.atlassian.com/" }
        }
    }"#;

    #[test]
    fn workspace_rust_avec_mcp_atlassian_detecte_tout() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/Cargo.toml", CARGO_WS)
            .with_file("/p/.mcp.json", MCP_PROJET)
            .with_file("/p/.github/workflows/ci.yml", "name: CI")
            .with_file("/p/LICENSE", "MIT License\n\nPermission is hereby granted");
        let env = FakeEnv::with_home("/home/x");
        let d = run(&fs, &env, Path::new("/p"));

        let stack = d.stack.expect("stack Rust attendue");
        assert_eq!(stack.language, "Rust");
        assert_eq!(stack.workspace_crate_count, Some(4));
        assert_eq!(d.license.as_deref(), Some("MIT"));
        assert!(d.has_ci);
        assert!(!d.is_git_repo);
        assert_eq!(d.mcps.len(), 1);
        assert_eq!(d.mcps[0].name, "claude.ai Atlassian Rovo");
        assert!(mcp::matches_jira(&d.mcps[0]));
    }

    #[test]
    fn dossier_vide_produit_detected_essentiellement_vide() {
        let fs = MemoryFileSystem::new();
        let env = FakeEnv::with_home("/home/x");
        let d = run(&fs, &env, Path::new("/p"));
        assert!(d.stack.is_none());
        assert!(d.project_name.is_none());
        assert!(d.license.is_none());
        assert!(!d.has_ci);
        assert!(!d.is_git_repo);
        assert!(d.mcps.is_empty());
    }

    #[test]
    fn mcp_projet_gagne_sur_mcp_global_quand_meme_nom() {
        const GLOBAL: &str = r#"{"mcpServers": {"Atlassian": {"url": "https://global"}}}"#;
        const PROJET: &str = r#"{"mcpServers": {"Atlassian": {"url": "https://projet"}}}"#;
        let fs = MemoryFileSystem::new()
            .with_file("/p/.mcp.json", PROJET)
            .with_file("/home/x/.claude.json", GLOBAL);
        let env = FakeEnv::with_home("/home/x");
        let d = run(&fs, &env, Path::new("/p"));
        assert_eq!(d.mcps.len(), 1);
        assert_eq!(d.mcps[0].url.as_deref(), Some("https://projet"));
        assert_eq!(d.mcps[0].source, ".mcp.json");
    }

    #[test]
    fn premier_manifeste_trouve_gagne() {
        // Un dossier qui a à la fois Cargo.toml (Rust) et package.json (JS)
        // → Rust gagne parce qu'il vient en premier dans MANIFESTS.
        let fs = MemoryFileSystem::new()
            .with_file("/p/Cargo.toml", CARGO_WS)
            .with_file("/p/package.json", r#"{"name": "a", "dependencies": {}}"#);
        let env = FakeEnv::with_home("/home/x");
        let d = run(&fs, &env, Path::new("/p"));
        assert_eq!(d.stack.unwrap().language, "Rust");
    }
}
