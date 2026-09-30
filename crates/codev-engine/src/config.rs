use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use codev_core::Layout;
use codev_core::config::DEFAULT_LANGUAGE;
use codev_core::detect::locale::is_valid_language_code;
use serde::Deserialize;

use crate::error::{EngineError, Result, Warning};
use crate::ports::{Env, FileSystem};

pub const DEFAULT_SCHEMA: &str = "spec-driven";

/// The `_codev/config.yaml` as written on disk.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectConfig {
    #[serde(default)]
    pub schema: Option<String>,
    /// Language of the prose skills write in artifacts, as an ISO 639 code
    /// (`en`, `fr`, `pt-BR`…). Absent means English.
    #[serde(default)]
    pub language: Option<String>,
    /// The workflows to install. `None` lets `codev-agents` pick its default
    /// catalog: it is the one that knows what exists.
    #[serde(default)]
    pub workflows: Option<Vec<String>>,
    #[serde(default)]
    pub context: Option<String>,
    /// Rules per artifact id.
    #[serde(default)]
    pub rules: BTreeMap<String, Vec<String>>,
    #[serde(default)]
    pub inherits: Vec<InheritSource>,
    /// Configuration of the MCP servers used by the skills. Optional field:
    /// a project with no MCP connected has nothing to declare, and the skills
    /// render with an informative fallback ("Jira MCP not configured").
    #[serde(default)]
    pub mcp: McpConfig,
}

/// Names of the MCP tools to inject into the skills at install time.
///
/// Each field maps to a use case (Jira, later Design, Confluence,
/// GitHub…). The tool name is specific to each user's Claude Code
/// environment — which is why it is configured project by project rather
/// than hardcoded in the CATALOG.
#[derive(Debug, Clone, Default, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct McpConfig {
    /// The MCP tool to call to fetch a Jira ticket mentioned in the prompt
    /// of a `/codev-propose`. Examples:
    /// `mcp__claude_ai_Atlassian_Rovo__getJiraIssue`,
    /// `mcp__claude_ai_Atlassian__getJiraIssue`.
    #[serde(default)]
    pub jira_tool: Option<String>,
}

/// A read-only inherited source.
///
/// A single type with optional fields, rather than an `untagged` enum: serde
/// errors on an untagged enum boil down to "data did not match any variant",
/// which helps no one facing hand-written YAML. Here the validation is ours,
/// and so is the message.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InheritSource {
    #[serde(default)]
    pub path: Option<String>,
    #[serde(default)]
    pub git: Option<String>,
    #[serde(default, rename = "ref")]
    pub git_ref: Option<String>,
    #[serde(default)]
    pub subpath: Option<String>,
}

/// A context or rule block, with its origin.
///
/// The origin is not decorative: the agent must be able to say "this rule
/// comes from codev-decisions", and flag a contradiction between two sources
/// instead of silently settling it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    pub origin: String,
    pub text: String,
}

/// The effective configuration, with inherited sources merged in.
#[derive(Debug, Clone)]
pub struct ResolvedConfig {
    pub schema: String,
    /// Artifact prose language — project only, like `mcp`: it belongs to
    /// the team writing in this repository, not to an inherited source.
    pub language: String,
    pub workflows: Option<Vec<String>>,
    /// From most general to most specific. On a contradiction, the last block
    /// wins — and the agent is instructed to flag the contradiction.
    pub context: Vec<Block>,
    pub rules: BTreeMap<String, Vec<Block>>,
    /// MCP configuration — project only, no inheritance: the tool names
    /// depend on the user's Claude Code configuration, not on the source
    /// project.
    pub mcp: McpConfig,
    pub warnings: Vec<Warning>,
}

impl ResolvedConfig {
    pub fn rules_for(&self, artifact_id: &str) -> &[Block] {
        self.rules
            .get(artifact_id)
            .map(Vec::as_slice)
            .unwrap_or_default()
    }
}

/// Reads a `config.yaml`. A missing file is not an error: a project can live
/// without configuration.
pub fn load(fs: &dyn FileSystem, path: &Path) -> Result<Option<ProjectConfig>> {
    if !fs.exists(path) {
        return Ok(None);
    }
    let raw = fs
        .read_to_string(path)
        .map_err(|e| EngineError::Unreadable {
            path: path.to_path_buf(),
            reason: e.to_string(),
        })?;
    let config: ProjectConfig = serde_norway::from_str(&raw).map_err(|e| EngineError::Invalid {
        path: path.to_path_buf(),
        reason: e.to_string(),
    })?;
    if let Some(code) = &config.language
        && !is_valid_language_code(code)
    {
        return Err(EngineError::Invalid {
            path: path.to_path_buf(),
            reason: format!(
                "`language: {code}` is not a language code; use an ISO 639 code such as `en`, `fr` or `pt-BR`"
            ),
        });
    }
    Ok(Some(config))
}

/// Resolves the project's effective configuration, inherited sources included.
pub fn resolve(fs: &dyn FileSystem, env: &dyn Env, layout: &Layout) -> Result<ResolvedConfig> {
    let config_path = layout.config_file();
    let project = load(fs, &config_path)?.unwrap_or_default();

    let mut context = Vec::new();
    let mut rules: BTreeMap<String, Vec<Block>> = BTreeMap::new();
    let mut warnings = Vec::new();

    // Sources first, in declared order: they are more general than the
    // project, which must be able to refine them last.
    for source in &project.inherits {
        match classify(source, &config_path)? {
            Source::Path(raw) => {
                let root = expand_tilde(&raw, env);
                let origin = format!("path:{raw}");
                let source_layout = Layout::new(&root);
                let source_config_path = source_layout.config_file();

                match load(fs, &source_config_path) {
                    Ok(Some(inherited)) => {
                        if !inherited.inherits.is_empty() {
                            warnings.push(Warning::new(
                                "inherit_not_transitive",
                                format!(
                                    "source `{raw}` itself inherits from other sources; \
                                     inheritance is not transitive, so those sources are ignored. \
                                     Declare them directly in {}",
                                    config_path.display()
                                ),
                            ));
                        }
                        push_blocks(&inherited, &origin, &mut context, &mut rules);
                    }
                    Ok(None) => warnings.push(Warning::new(
                        "inherit_unresolved",
                        format!(
                            "inherited source `{raw}` has no {} — check the path, \
                             or initialize it with `codev init`",
                            source_config_path.display()
                        ),
                    )),
                    Err(err) => warnings.push(Warning::new(
                        "inherit_unreadable",
                        format!("inherited source `{raw}` is unreadable: {err}"),
                    )),
                }
            }
            Source::Git {
                url,
                git_ref,
                subpath,
            } => {
                let git_content_dir = resolve_git_source_dir(fs, env, layout, &url, &git_ref);
                match git_content_dir {
                    GitResolution::Ok(content_root) => {
                        let source_config_path = Layout::new(crate::sources::project_root(
                            &content_root,
                            subpath.as_deref(),
                        ))
                        .config_file();
                        let origin = format!("git:{url}");
                        if let Ok(Some(inherited)) = load(fs, &source_config_path) {
                            if !inherited.inherits.is_empty() {
                                warnings.push(Warning::new(
                                    "inherit_not_transitive",
                                    format!(
                                        "source `{url}` itself inherits from other sources; \
                                         inheritance is not transitive, so those sources are ignored"
                                    ),
                                ));
                            }
                            push_blocks(&inherited, &origin, &mut context, &mut rules);
                        }
                    }
                    GitResolution::Unlocked => warnings.push(Warning::new(
                        "git_source_unlocked",
                        format!(
                            "git source `{url}` has no locked SHA — \
                             run `codev sources update` to resolve it"
                        ),
                    )),
                    GitResolution::CacheMissing => warnings.push(Warning::new(
                        "git_source_needs_update",
                        format!(
                            "git source `{url}` is locked but missing from the local cache — \
                             run `codev sources update`"
                        ),
                    )),
                }
            }
        }
    }

    push_blocks(&project, "project", &mut context, &mut rules);

    Ok(ResolvedConfig {
        schema: project
            .schema
            .clone()
            .unwrap_or_else(|| DEFAULT_SCHEMA.to_string()),
        language: project
            .language
            .clone()
            .unwrap_or_else(|| DEFAULT_LANGUAGE.to_string()),
        workflows: project.workflows.clone(),
        context,
        rules,
        // MCP is project-only: no merging with inherited sources. The tool
        // names depend on the user's Claude Code configuration, not on a
        // shared repository.
        mcp: project.mcp.clone(),
        warnings,
    })
}

fn push_blocks(
    config: &ProjectConfig,
    origin: &str,
    context: &mut Vec<Block>,
    rules: &mut BTreeMap<String, Vec<Block>>,
) {
    if let Some(text) = config.context.as_ref().filter(|t| !t.trim().is_empty()) {
        context.push(Block {
            origin: origin.to_string(),
            text: text.trim().to_string(),
        });
    }
    for (artifact, entries) in &config.rules {
        for text in entries.iter().filter(|t| !t.trim().is_empty()) {
            rules.entry(artifact.clone()).or_default().push(Block {
                origin: origin.to_string(),
                text: text.trim().to_string(),
            });
        }
    }
}

enum Source {
    Path(String),
    Git {
        url: String,
        git_ref: String,
        subpath: Option<String>,
    },
}

/// Resolution state of a `git:` source, looked up in the lock.
enum GitResolution {
    /// Locked SHA, content present in the cache — path returned.
    Ok(std::path::PathBuf),
    /// No `codev.lock`, or no entry for this source.
    Unlocked,
    /// Locked SHA, but the `content/<sha>/` directory is missing from the cache.
    CacheMissing,
}

fn resolve_git_source_dir(
    fs: &dyn FileSystem,
    env: &dyn Env,
    layout: &Layout,
    url: &str,
    git_ref: &str,
) -> GitResolution {
    let lock_path = layout.planning_dir().join("codev.lock");
    let lock = match crate::sources::lockfile::load(fs, &lock_path) {
        Ok(Some(lock)) => lock,
        _ => return GitResolution::Unlocked,
    };
    let Some(entry) = crate::sources::LockEntry::find_matching(Some(&lock), url, git_ref) else {
        return GitResolution::Unlocked;
    };
    let cache_root = crate::sources::cache::root(env);
    let content_dir = crate::sources::cache::content_dir(&cache_root, &entry.commit);
    if fs.exists(&content_dir) {
        GitResolution::Ok(content_dir)
    } else {
        GitResolution::CacheMissing
    }
}

fn classify(source: &InheritSource, config_path: &Path) -> Result<Source> {
    match (&source.path, &source.git) {
        (Some(_), Some(_)) => Err(EngineError::Invalid {
            path: config_path.to_path_buf(),
            reason: "an inherited source declares both `path` and `git`; choose one of the two"
                .into(),
        }),
        (None, None) => Err(EngineError::Invalid {
            path: config_path.to_path_buf(),
            reason: "an inherited source declares neither `path` nor `git`".into(),
        }),
        (Some(path), None) => {
            if source.git_ref.is_some() || source.subpath.is_some() {
                return Err(EngineError::Invalid {
                    path: config_path.to_path_buf(),
                    reason: format!(
                        "source `{path}` is local: `ref` and `subpath` only apply \
                         to a `git` source"
                    ),
                });
            }
            Ok(Source::Path(path.clone()))
        }
        (None, Some(url)) => {
            let git_ref = source.git_ref.clone().ok_or_else(|| EngineError::Invalid {
                path: config_path.to_path_buf(),
                reason: format!("git source `{url}` has no `ref` — specify the branch or tag"),
            })?;
            Ok(Source::Git {
                url: url.clone(),
                git_ref,
                subpath: source.subpath.clone(),
            })
        }
    }
}

/// Expands a leading `~` in a path.
///
/// Only at the start, and only when followed by the end of the string or a
/// separator — `~something` is not a user's home directory here.
pub(crate) fn expand_tilde(raw: &str, env: &dyn Env) -> PathBuf {
    let rest = match raw.strip_prefix('~') {
        Some("") => "",
        Some(rest) if rest.starts_with('/') => &rest[1..],
        _ => return PathBuf::from(raw),
    };
    match env.home_dir() {
        Some(home) if rest.is_empty() => home,
        Some(home) => home.join(rest),
        None => PathBuf::from(raw),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ports::{FixedEnv, MemoryFileSystem};

    fn env_with_home() -> FixedEnv {
        let mut env = FixedEnv::at("/p");
        env.vars.insert("HOME".into(), "/Users/me".into());
        env
    }

    #[test]
    fn a_project_without_config_uses_defaults() {
        let fs = MemoryFileSystem::new().with_file("/p/_codev/specs/.gitkeep", "");
        let resolved = resolve(&fs, &env_with_home(), &Layout::new("/p")).unwrap();
        assert_eq!(resolved.schema, DEFAULT_SCHEMA);
        assert!(resolved.workflows.is_none());
        assert!(resolved.context.is_empty());
        assert!(resolved.warnings.is_empty());
    }

    #[test]
    fn the_project_comes_after_its_sources() {
        let fs = MemoryFileSystem::new()
            .with_file(
                "/Users/me/shared/_codev/config.yaml",
                "context: |\n  House convention\nrules:\n  specs:\n    - Shared rule\n",
            )
            .with_file(
                "/p/_codev/config.yaml",
                "inherits:\n  - path: ~/shared\ncontext: |\n  Project-specific\nrules:\n  specs:\n    - Local rule\n",
            );

        let resolved = resolve(&fs, &env_with_home(), &Layout::new("/p")).unwrap();

        assert_eq!(
            resolved
                .context
                .iter()
                .map(|b| (b.origin.as_str(), b.text.as_str()))
                .collect::<Vec<_>>(),
            [
                ("path:~/shared", "House convention"),
                ("project", "Project-specific"),
            ],
            "order goes from most general to most specific"
        );
        assert_eq!(
            resolved
                .rules_for("specs")
                .iter()
                .map(|b| b.text.as_str())
                .collect::<Vec<_>>(),
            ["Shared rule", "Local rule"]
        );
        assert!(resolved.warnings.is_empty());
    }

    #[test]
    fn a_missing_source_warns_without_failing() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "inherits:\n  - path: /nowhere\n");
        let resolved = resolve(&fs, &env_with_home(), &Layout::new("/p")).unwrap();
        assert_eq!(resolved.schema, DEFAULT_SCHEMA);
        assert_eq!(resolved.warnings.len(), 1);
        assert_eq!(resolved.warnings[0].code, "inherit_unresolved");
    }

    #[test]
    fn an_unlocked_git_source_is_reported() {
        // The legacy `inherit_git_unsupported` warning is replaced by
        // `git_source_unlocked` — the source is recognized, the user is just
        // asked to lock it.
        let fs = MemoryFileSystem::new().with_file(
            "/p/_codev/config.yaml",
            "inherits:\n  - git: git@github.com:o/r.git\n    ref: main\n",
        );
        let resolved = resolve(&fs, &env_with_home(), &Layout::new("/p")).unwrap();
        assert!(
            resolved
                .warnings
                .iter()
                .any(|w| w.code == "git_source_unlocked"),
            "{:?}",
            resolved.warnings
        );
        assert!(
            !resolved
                .warnings
                .iter()
                .any(|w| w.code == "inherit_git_unsupported"),
            "the legacy warning must no longer be emitted"
        );
    }

    #[test]
    fn a_locked_git_source_is_resolved() {
        let lock = "version = 1\n\n[[source]]\ngit = \"url\"\nref = \"main\"\ncommit = \"deadbeef\"\nresolved_at = \"2026-09-08\"\n";
        let fs = MemoryFileSystem::new()
            .with_file(
                "/p/_codev/config.yaml",
                "inherits:\n  - git: url\n    ref: main\n",
            )
            .with_file("/p/_codev/codev.lock", lock)
            // Cache content: a well-formed inherited config.
            .with_file(
                "/Users/me/.cache/codev/content/deadbeef/_codev/config.yaml",
                "context: |\n  Context inherited via git\n",
            );
        let resolved = resolve(&fs, &env_with_home(), &Layout::new("/p")).unwrap();
        assert!(resolved.warnings.is_empty(), "{:?}", resolved.warnings);
        assert!(
            resolved
                .context
                .iter()
                .any(|b| b.origin == "git:url" && b.text.contains("inherited via git"))
        );
    }

    #[test]
    fn a_locked_git_source_without_cache_asks_for_update() {
        let lock = "version = 1\n\n[[source]]\ngit = \"url\"\nref = \"main\"\ncommit = \"deadbeef\"\nresolved_at = \"2026-09-08\"\n";
        let fs = MemoryFileSystem::new()
            .with_file(
                "/p/_codev/config.yaml",
                "inherits:\n  - git: url\n    ref: main\n",
            )
            .with_file("/p/_codev/codev.lock", lock);
        // No content in the cache.
        let resolved = resolve(&fs, &env_with_home(), &Layout::new("/p")).unwrap();
        assert!(
            resolved
                .warnings
                .iter()
                .any(|w| w.code == "git_source_needs_update")
        );
    }

    #[test]
    fn a_git_source_without_ref_is_rejected() {
        let fs =
            MemoryFileSystem::new().with_file("/p/_codev/config.yaml", "inherits:\n  - git: url\n");
        let err = resolve(&fs, &env_with_home(), &Layout::new("/p")).unwrap_err();
        assert_eq!(err.code(), "invalid");
        assert!(err.to_string().contains("`ref`"), "{err}");
    }

    #[test]
    fn inheritance_is_not_transitive_and_says_so() {
        let fs = MemoryFileSystem::new()
            .with_file(
                "/Users/me/a/_codev/config.yaml",
                "inherits:\n  - path: ~/b\ncontext: |\n  From A\n",
            )
            .with_file("/Users/me/b/_codev/config.yaml", "context: |\n  From B\n")
            .with_file("/p/_codev/config.yaml", "inherits:\n  - path: ~/a\n");

        let resolved = resolve(&fs, &env_with_home(), &Layout::new("/p")).unwrap();

        assert_eq!(resolved.context.len(), 1, "only A is inherited");
        assert_eq!(resolved.context[0].text, "From A");
        assert_eq!(resolved.warnings[0].code, "inherit_not_transitive");
    }

    #[test]
    fn rejects_an_ambiguous_source() {
        let fs = MemoryFileSystem::new().with_file(
            "/p/_codev/config.yaml",
            "inherits:\n  - path: /a\n    git: git@github.com:o/r.git\n",
        );
        let err = resolve(&fs, &env_with_home(), &Layout::new("/p")).unwrap_err();
        assert_eq!(err.code(), "invalid");
        assert!(err.to_string().contains("choose one of the two"), "{err}");
    }

    #[test]
    fn rejects_ref_on_a_local_source() {
        let fs = MemoryFileSystem::new().with_file(
            "/p/_codev/config.yaml",
            "inherits:\n  - path: /a\n    ref: main\n",
        );
        let err = resolve(&fs, &env_with_home(), &Layout::new("/p")).unwrap_err();
        assert!(err.to_string().contains("`ref` and `subpath`"), "{err}");
    }

    #[test]
    fn reports_an_unknown_key_in_the_config() {
        let fs = MemoryFileSystem::new().with_file("/p/_codev/config.yaml", "shema: spec-driven\n");
        let err = resolve(&fs, &env_with_home(), &Layout::new("/p")).unwrap_err();
        assert_eq!(err.code(), "invalid");
        assert!(err.to_string().contains("shema"), "{err}");
    }

    #[test]
    fn expands_the_tilde() {
        let env = env_with_home();
        assert_eq!(expand_tilde("~/x", &env), PathBuf::from("/Users/me/x"));
        assert_eq!(expand_tilde("~", &env), PathBuf::from("/Users/me"));
        assert_eq!(expand_tilde("/abs", &env), PathBuf::from("/abs"));
        assert_eq!(
            expand_tilde("~something", &env),
            PathBuf::from("~something")
        );
    }

    // ─────────────── MCP configuration ───────────────

    #[test]
    fn missing_mcp_config_yields_empty_default() {
        let yaml = "schema: spec-driven\n";
        let cfg: ProjectConfig = serde_norway::from_str(yaml).unwrap();
        assert_eq!(cfg.mcp, McpConfig::default());
        assert!(cfg.mcp.jira_tool.is_none());
    }

    #[test]
    fn mcp_config_jira_tool_is_read() {
        let yaml = "schema: spec-driven\nmcp:\n  jira_tool: mcp__foo__getJiraIssue\n";
        let cfg: ProjectConfig = serde_norway::from_str(yaml).unwrap();
        assert_eq!(cfg.mcp.jira_tool.as_deref(), Some("mcp__foo__getJiraIssue"));
    }

    #[test]
    fn mcp_config_unknown_field_is_rejected() {
        // deny_unknown_fields locks down McpConfig — a typo does not slip
        // through silently.
        let yaml = "schema: spec-driven\nmcp:\n  jiraTool: mcp__foo\n";
        let err = serde_norway::from_str::<ProjectConfig>(yaml).unwrap_err();
        assert!(err.to_string().contains("jiraTool"), "{err}");
    }

    #[test]
    fn resolve_propagates_the_project_mcp() {
        let fs = MemoryFileSystem::new().with_file(
            "/p/_codev/config.yaml",
            "mcp:\n  jira_tool: mcp__bar__get\n",
        );
        let resolved = resolve(&fs, &env_with_home(), &Layout::new("/p")).unwrap();
        assert_eq!(resolved.mcp.jira_tool.as_deref(), Some("mcp__bar__get"));
    }

    #[test]
    fn language_defaults_to_english() {
        let fs =
            MemoryFileSystem::new().with_file("/p/_codev/config.yaml", "schema: spec-driven\n");
        let config = resolve(&fs, &env_with_home(), &Layout::new("/p")).unwrap();
        assert_eq!(config.language, "en");
    }

    #[test]
    fn language_is_read_from_the_project() {
        let fs = MemoryFileSystem::new().with_file("/p/_codev/config.yaml", "language: fr\n");
        let config = resolve(&fs, &env_with_home(), &Layout::new("/p")).unwrap();
        assert_eq!(config.language, "fr");
    }

    #[test]
    fn an_invalid_language_is_rejected_with_a_hint() {
        let fs = MemoryFileSystem::new().with_file("/p/_codev/config.yaml", "language: French\n");
        let err = resolve(&fs, &env_with_home(), &Layout::new("/p"))
            .unwrap_err()
            .to_string();
        assert!(err.contains("`language: French`"), "{err}");
        assert!(err.contains("ISO 639"), "{err}");
    }
}
