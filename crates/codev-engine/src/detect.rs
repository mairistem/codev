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
    Detected, license, locale, mcp,
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
/// - `env` — to resolve `~/.claude.json` via `HOME` / `USERPROFILE`, and
///   to read the locale variables that suggest the artifact language.
pub fn run(fs: &dyn FileSystem, env: &dyn Env, project_root: &Path) -> Detected {
    let home = env.home_dir();

    let (stack, project_name) = detect_stack_and_name(fs, project_root);
    let license = detect_license(fs, project_root);
    let has_ci = detect_ci(fs, project_root);
    let is_git_repo = fs.exists(&project_root.join(".git"));
    let mcps = detect_mcps(fs, project_root, home.as_deref());
    let locale = locale::detect(|var| env.var(var));

    Detected {
        stack,
        project_name,
        license,
        has_ci,
        is_git_repo,
        mcps,
        locale,
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

/// How deep [`stacks`] looks for manifests: the root and three levels of
/// directories below it (`apps/<name>/`, `packages/<scope>/<name>/`).
const STACK_DEPTH: usize = 3;
/// The most directories [`stacks`] lists: a bounded cost on any repository.
const STACK_MAX_DIRS: usize = 2000;
/// Directories whose manifests are someone else's, or build output.
const STACK_SKIPPED_DIRS: &[&str] = &[
    "node_modules",
    "vendor",
    "third_party",
    "target",
    "dist",
    "build",
];

/// The languages of the project rooted at `root`, from the names of the
/// dependency manifests at its root and in its directories up to
/// [`STACK_DEPTH`] levels below, breadth first, skipping hidden and
/// [`STACK_SKIPPED_DIRS`] directories and listing at most
/// [`STACK_MAX_DIRS`]. Every stack of a monorepo is found; the order is
/// `codev_core::detect::stack::LANGUAGES`'.
pub fn stacks(fs: &dyn FileSystem, root: &Path) -> Vec<&'static str> {
    let mut names: Vec<String> = Vec::new();
    let mut queue: std::collections::VecDeque<(PathBuf, usize)> = [(root.to_path_buf(), 0)].into();
    let mut listed = 0;
    while let Some((dir, depth)) = queue.pop_front() {
        if listed >= STACK_MAX_DIRS {
            break;
        }
        listed += 1;
        // A file lists as empty: no need to ask which entries are folders.
        for name in fs.list_dir(&dir).unwrap_or_default() {
            if !stack::manifest_languages(&name).is_empty() {
                names.push(name);
            } else if depth < STACK_DEPTH
                && !name.starts_with('.')
                && !STACK_SKIPPED_DIRS.contains(&name.as_str())
            {
                queue.push_back((dir.join(&name), depth + 1));
            }
        }
    }
    stack::languages(names.iter().map(String::as_str))
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
    fn stacks_of_a_monorepo() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/services/api/go.mod", "module x\n")
            .with_file("/p/web/package.json", "{}")
            .with_file("/p/README.md", "x");
        assert_eq!(
            stacks(&fs, Path::new("/p")),
            ["TypeScript", "JavaScript", "Go"]
        );
        assert!(stacks(&MemoryFileSystem::new(), Path::new("/p")).is_empty());
    }

    #[test]
    fn manifests_in_skipped_directories_do_not_count() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/Cargo.toml", "[package]\nname = \"x\"\n")
            .with_file("/p/node_modules/left-pad/package.json", "{}")
            .with_file("/p/.cache/pyproject.toml", "")
            .with_file("/p/target/debug/build/x/go.mod", "")
            .with_file("/p/crates/a/vendor/b/pom.xml", "");
        assert_eq!(stacks(&fs, Path::new("/p")), ["Rust"]);
    }

    #[test]
    fn the_walk_stops_at_three_levels() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/a/b/c/package.json", "{}")
            .with_file("/p/a/b/c/d/go.mod", "");
        assert_eq!(stacks(&fs, Path::new("/p")), ["TypeScript", "JavaScript"]);
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
