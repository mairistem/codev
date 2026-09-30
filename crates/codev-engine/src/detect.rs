//! Orchestrator for the `codev init` probe.
//!
//! Coordinates disk access (via `FileSystem`) and environment access
//! (`Env` — for `~/.claude.json`). Every field is best-effort: a missing
//! file is not an error, it produces a silent warning.
//!
//! The pure functions that parse each manifest live in
//! `codev-core::detect` — they are testable without I/O. Here we merely
//! call the right ports in the right order.

use std::path::{Path, PathBuf};

use codev_core::detect::{
    Detected, license, mcp,
    stack::{self, Stack},
};

use crate::ports::{Env, FileSystem};

/// Search order for stack manifests. The first one found sets the primary
/// stack; the others are ignored.
const MANIFESTS: &[(&str, StackKind)] = &[
    ("Cargo.toml", StackKind::Cargo),
    ("package.json", StackKind::PackageJson),
    ("pyproject.toml", StackKind::Pyproject),
    ("go.mod", StackKind::GoMod),
    ("pom.xml", StackKind::PomXml),
];

/// Search order for MCP configs. The first one wins when a server name is
/// duplicated (project config wins over global user config).
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

/// Probes the current directory and returns a `Detected` report.
///
/// - `project_root` — the root of the inspected project.
/// - `env` — to resolve `~/.claude.json` via `HOME` / `USERPROFILE`.
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

    const MCP_PROJECT: &str = r#"{
        "mcpServers": {
            "claude.ai Atlassian Rovo": { "url": "https://mcp.atlassian.com/" }
        }
    }"#;

    #[test]
    fn rust_workspace_with_atlassian_mcp_detects_everything() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/Cargo.toml", CARGO_WS)
            .with_file("/p/.mcp.json", MCP_PROJECT)
            .with_file("/p/.github/workflows/ci.yml", "name: CI")
            .with_file("/p/LICENSE", "MIT License\n\nPermission is hereby granted");
        let env = FakeEnv::with_home("/home/x");
        let d = run(&fs, &env, Path::new("/p"));

        let stack = d.stack.expect("expected a Rust stack");
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
    fn empty_directory_yields_essentially_empty_detected() {
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
    fn project_mcp_wins_over_global_mcp_with_same_name() {
        const GLOBAL: &str = r#"{"mcpServers": {"Atlassian": {"url": "https://global"}}}"#;
        const PROJECT: &str = r#"{"mcpServers": {"Atlassian": {"url": "https://project"}}}"#;
        let fs = MemoryFileSystem::new()
            .with_file("/p/.mcp.json", PROJECT)
            .with_file("/home/x/.claude.json", GLOBAL);
        let env = FakeEnv::with_home("/home/x");
        let d = run(&fs, &env, Path::new("/p"));
        assert_eq!(d.mcps.len(), 1);
        assert_eq!(d.mcps[0].url.as_deref(), Some("https://project"));
        assert_eq!(d.mcps[0].source, ".mcp.json");
    }

    #[test]
    fn first_manifest_found_wins() {
        // A directory with both Cargo.toml (Rust) and package.json (JS)
        // → Rust wins because it comes first in MANIFESTS.
        let fs = MemoryFileSystem::new()
            .with_file("/p/Cargo.toml", CARGO_WS)
            .with_file("/p/package.json", r#"{"name": "a", "dependencies": {}}"#);
        let env = FakeEnv::with_home("/home/x");
        let d = run(&fs, &env, Path::new("/p"));
        assert_eq!(d.stack.unwrap().language, "Rust");
    }
}
