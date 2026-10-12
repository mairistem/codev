use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use codev_core::Layout;
use codev_core::config::DEFAULT_LANGUAGE;
use codev_core::detect::locale::is_valid_language_code;
use serde::Deserialize;

use crate::error::{EngineError, Result, Warning};
use crate::ports::{Env, FileSystem};

pub const DEFAULT_SCHEMA: &str = "spec-driven";

/// The top-level keys of `_codev/config.yaml` this codev knows. Any other
/// key is ignored with an `unknown_config_key` warning: it may come from a
/// newer codev, and failing on it would leave an older one unable to say
/// so. Keep it in sync with [`ProjectConfig`] —
/// `every_known_key_is_recognized` fails otherwise.
pub const PROJECT_KEYS: &[&str] = &[
    "schema",
    "language",
    "workflows",
    "context",
    "rules",
    "inherits",
    "mcp",
    "changelog",
    "codev_version",
];

/// The keys of the `mcp:` section this codev knows. The section grows one
/// tool per integration, and ignoring an unknown tool only leaves that
/// integration unconfigured: tolerant, like the top level.
pub const MCP_KEYS: &[&str] = &["jira_tool"];

/// The code of the warning (and of the validate finding, and of the doctor
/// check) for a key this codev ignores.
pub const UNKNOWN_CONFIG_KEY: &str = "unknown_config_key";

/// A key of `_codev/config.yaml` this codev does not know.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnknownKey {
    /// As written, `mcp.<key>` inside `mcp`.
    pub key: String,
    /// 1-based line of the key, 1 when it cannot be located (a flow
    /// mapping, for instance).
    pub line: usize,
}

/// The `_codev/config.yaml` as written on disk.
///
/// No `deny_unknown_fields` here nor on [`McpConfig`]: unknown keys are
/// collected into `unknown_keys` instead. `inherits` entries and
/// `changelog` stay strict — ignoring a key there would silently read
/// another source or write another `CHANGELOG.md`.
#[derive(Debug, Clone, Default, Deserialize)]
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
    /// `codev changelog` settings — project only, like `mcp`: they describe
    /// this repository's `CHANGELOG.md`.
    #[serde(default)]
    pub changelog: ChangelogConfig,
    /// The codev versions the team accepts, as a Cargo requirement
    /// (`">=0.5"`). Project only: the team writing in this repository owns
    /// it, an inherited source does not.
    #[serde(default)]
    pub codev_version: Option<String>,
    /// Top-level and `mcp` keys this codev ignored, in file order.
    #[serde(skip)]
    pub unknown_keys: Vec<UnknownKey>,
}

/// The `changelog:` block of `_codev/config.yaml`.
#[derive(Debug, Clone, Default, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ChangelogConfig {
    /// Archive folder names (`<date>-<name>`) of changes already described
    /// by hand in `CHANGELOG.md`: `codev changelog` leaves them out, so
    /// that a project adopting it does not list them twice.
    #[serde(default)]
    pub exclude: Vec<String>,
}

/// Names of the MCP tools to inject into the skills at install time.
///
/// Each field maps to a use case (Jira, later Design, Confluence,
/// GitHub…). The tool name is specific to each user's Claude Code
/// environment — which is why it is configured project by project rather
/// than hardcoded in the CATALOG.
#[derive(Debug, Clone, Default, Deserialize, PartialEq, Eq)]
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
    /// `codev_version`, already validated.
    pub codev_version: Option<String>,
    /// The project's unknown keys — inherited sources' are not reported:
    /// only their context and rules are read.
    pub unknown_keys: Vec<UnknownKey>,
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
    let mut config: ProjectConfig =
        serde_norway::from_str(&raw).map_err(|e| EngineError::Invalid {
            path: path.to_path_buf(),
            reason: with_requirement_hint(e.to_string(), &raw),
        })?;
    config.unknown_keys = unknown_keys(&raw);
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
    if let Some(requirement) = &config.codev_version
        && let Err(reason) = codev_core::update::parse_requirement(requirement)
    {
        return Err(EngineError::InvalidCodevVersion {
            path: path.to_path_buf(),
            reason,
        });
    }
    Ok(Some(config))
}

/// The top-level and `mcp` keys of `raw` that this codev does not know, in
/// file order. Read from a generic YAML mapping: the typed parse has
/// already ignored them.
fn unknown_keys(raw: &str) -> Vec<UnknownKey> {
    let Ok(serde_norway::Value::Mapping(map)) = serde_norway::from_str(raw) else {
        return Vec::new();
    };
    let mut unknown = Vec::new();
    for (key, value) in &map {
        let Some(key) = key.as_str() else { continue };
        if !PROJECT_KEYS.contains(&key) {
            unknown.push(UnknownKey {
                key: key.to_string(),
                line: key_line(raw, None, key),
            });
        } else if key == "mcp"
            && let Some(mcp) = value.as_mapping()
        {
            for sub in mcp.keys().filter_map(serde_norway::Value::as_str) {
                if !MCP_KEYS.contains(&sub) {
                    unknown.push(UnknownKey {
                        key: format!("mcp.{sub}"),
                        line: key_line(raw, Some("mcp"), sub),
                    });
                }
            }
        }
    }
    unknown
}

/// The 1-based line where `key` is written — at the top level, or in the
/// block of `parent`. A text scan, only used to point at the key: 1 when
/// not found.
fn key_line(raw: &str, parent: Option<&str>, key: &str) -> usize {
    let names = |line: &str, name: &str| {
        line.strip_prefix(name)
            .is_some_and(|rest| rest.trim_start().starts_with(':'))
    };
    let lines: Vec<&str> = raw.lines().collect();
    let Some(parent) = parent else {
        return lines
            .iter()
            .position(|l| names(l, key))
            .map_or(1, |i| i + 1);
    };
    let Some(start) = lines.iter().position(|l| names(l, parent)) else {
        return 1;
    };
    for (i, line) in lines.iter().enumerate().skip(start + 1) {
        let trimmed = line.trim_start();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if trimmed.len() == line.len() {
            break;
        }
        if names(trimmed, key) {
            return i + 1;
        }
    }
    start + 1
}

/// `codev_version`, read leniently: whatever else the file holds, as long
/// as it is a YAML mapping with a string under that key.
fn lenient_codev_version(raw: &str) -> Option<String> {
    let serde_norway::Value::Mapping(map) = serde_norway::from_str(raw).ok()? else {
        return None;
    };
    map.get("codev_version")?.as_str().map(str::to_string)
}

/// A parse error, completed with the project's requirement when it has
/// one: the key that broke the parse may come from a newer codev.
fn with_requirement_hint(reason: String, raw: &str) -> String {
    match lenient_codev_version(raw) {
        Some(requirement) => format!(
            "{reason} — this project requires codev_version: \"{requirement}\"; \
             run `codev self-update` if your codev is older"
        ),
        None => reason,
    }
}

/// The message of an `unknown_config_key` warning or finding.
pub fn unknown_key_message(key: &str, codev_version: Option<&str>) -> String {
    let requirement = codev_version
        .map(|r| format!(" (it requires codev_version: \"{r}\")"))
        .unwrap_or_default();
    format!(
        "_codev/config.yaml: `{key}` is not a key this codev knows and is ignored; \
         the project may need a newer codev{requirement} — run `codev self-update`, \
         or correct the key if it is a typo"
    )
}

/// Resolves the project's effective configuration, inherited sources included.
pub fn resolve(fs: &dyn FileSystem, env: &dyn Env, layout: &Layout) -> Result<ResolvedConfig> {
    let config_path = layout.config_file();
    let project = load(fs, &config_path)?.unwrap_or_default();

    let mut context = Vec::new();
    let mut rules: BTreeMap<String, Vec<Block>> = BTreeMap::new();
    let mut warnings: Vec<Warning> = project
        .unknown_keys
        .iter()
        .map(|u| {
            Warning::new(
                UNKNOWN_CONFIG_KEY,
                unknown_key_message(&u.key, project.codev_version.as_deref()),
            )
        })
        .collect();

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
        codev_version: project.codev_version.clone(),
        unknown_keys: project.unknown_keys.clone(),
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
pub fn expand_tilde(raw: &str, env: &dyn Env) -> PathBuf {
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

    fn load_str(yaml: &str) -> Result<Option<ProjectConfig>> {
        let fs = MemoryFileSystem::new().with_file("/p/_codev/config.yaml", yaml);
        load(&fs, Path::new("/p/_codev/config.yaml"))
    }

    #[test]
    fn unknown_top_level_keys_are_collected_with_their_line() {
        let config =
            load_str("schema: spec-driven\n# a comment\nreview_policy: strict\nshema: x\n")
                .unwrap()
                .unwrap();
        assert_eq!(config.schema.as_deref(), Some("spec-driven"));
        assert_eq!(
            config.unknown_keys,
            [
                UnknownKey {
                    key: "review_policy".into(),
                    line: 3
                },
                UnknownKey {
                    key: "shema".into(),
                    line: 4
                },
            ]
        );
    }

    #[test]
    fn unknown_mcp_keys_are_collected_with_a_dotted_name() {
        let config = load_str(
            "mcp:\n  jira_tool: mcp__a__get\n  # note\n  design_tool: mcp__b__get\nlanguage: en\n",
        )
        .unwrap()
        .unwrap();
        assert_eq!(config.mcp.jira_tool.as_deref(), Some("mcp__a__get"));
        assert_eq!(
            config.unknown_keys,
            [UnknownKey {
                key: "mcp.design_tool".into(),
                line: 4
            }]
        );
        // A flow mapping cannot be located line by line: the `mcp` line.
        let config = load_str("schema: x\nmcp: {jira_tool: a, design_tool: b}\n")
            .unwrap()
            .unwrap();
        assert_eq!(config.unknown_keys[0].key, "mcp.design_tool");
        assert_eq!(config.unknown_keys[0].line, 2);
    }

    #[test]
    fn every_known_key_is_recognized() {
        let config = load_str(
            "schema: spec-driven\nlanguage: en\nworkflows: [propose]\ncontext: x\n\
             rules:\n  specs: [r]\ninherits:\n  - path: /a\n\
             mcp:\n  jira_tool: t\nchangelog:\n  exclude: []\ncodev_version: \">=0.5\"\n",
        )
        .unwrap()
        .unwrap();
        assert!(config.unknown_keys.is_empty(), "{:?}", config.unknown_keys);
        assert_eq!(PROJECT_KEYS.len(), 9, "a new key belongs in this test too");
    }

    #[test]
    fn an_unknown_changelog_key_still_fails() {
        let err = load_str("changelog:\n  since: v1.0.0\n").unwrap_err();
        assert_eq!(err.code(), "invalid");
        assert!(err.to_string().contains("since"), "{err}");
    }

    #[test]
    fn an_unknown_inherits_key_still_fails() {
        let err = load_str("inherits:\n  - path: ~/shared\n    pin: abc\n").unwrap_err();
        assert_eq!(err.code(), "invalid");
        assert!(err.to_string().contains("pin"), "{err}");
    }

    #[test]
    fn a_strict_section_error_quotes_codev_version() {
        let err = load_str("codev_version: \">=0.9\"\nchangelog:\n  since: v1.0.0\n").unwrap_err();
        assert_eq!(err.code(), "invalid");
        let message = err.to_string();
        assert!(message.contains("since"), "{message}");
        assert!(message.contains("\">=0.9\""), "{message}");
        assert!(message.contains("codev self-update"), "{message}");
    }

    #[test]
    fn a_strict_section_error_without_codev_version_names_the_key() {
        let err = load_str("inherits:\n  - path: ~/shared\n    pin: abc\n").unwrap_err();
        let message = err.to_string();
        assert!(message.contains("pin"), "{message}");
        assert!(!message.contains("codev_version"), "{message}");
    }

    #[test]
    fn an_unknown_top_level_key_warns_and_quotes_codev_version() {
        let fs = MemoryFileSystem::new().with_file(
            "/p/_codev/config.yaml",
            "codev_version: \">=0.9\"\nreview_policy: strict\n",
        );
        let resolved = resolve(&fs, &env_with_home(), &Layout::new("/p")).unwrap();
        assert_eq!(resolved.warnings.len(), 1, "{:?}", resolved.warnings);
        let warning = &resolved.warnings[0];
        assert_eq!(warning.code, "unknown_config_key");
        assert!(
            warning.message.contains("`review_policy`"),
            "{}",
            warning.message
        );
        assert!(warning.message.contains("\">=0.9\""), "{}", warning.message);
        assert!(
            warning.message.contains("codev self-update"),
            "{}",
            warning.message
        );
        assert_eq!(resolved.unknown_keys[0].key, "review_policy");
    }

    #[test]
    fn an_unknown_top_level_key_without_requirement_warns() {
        let fs = MemoryFileSystem::new().with_file("/p/_codev/config.yaml", "shema: spec-driven\n");
        let resolved = resolve(&fs, &env_with_home(), &Layout::new("/p")).unwrap();
        assert_eq!(
            resolved.schema, DEFAULT_SCHEMA,
            "the misspelled key is ignored"
        );
        let warning = &resolved.warnings[0];
        assert_eq!(warning.code, "unknown_config_key");
        assert!(warning.message.contains("`shema`"), "{}", warning.message);
        assert!(
            warning.message.contains("newer codev"),
            "{}",
            warning.message
        );
        assert!(
            !warning.message.contains("codev_version"),
            "{}",
            warning.message
        );
    }

    #[test]
    fn an_inherited_source_unknown_key_is_ignored_silently() {
        let fs = MemoryFileSystem::new()
            .with_file(
                "/Users/me/shared/_codev/config.yaml",
                "review_policy: strict\ncontext: |\n  Shared context\n",
            )
            .with_file("/p/_codev/config.yaml", "inherits:\n  - path: ~/shared\n");
        let resolved = resolve(&fs, &env_with_home(), &Layout::new("/p")).unwrap();
        assert!(resolved.warnings.is_empty(), "{:?}", resolved.warnings);
        assert!(resolved.unknown_keys.is_empty());
        assert_eq!(resolved.context[0].text, "Shared context");
    }

    #[test]
    fn changelog_exclude_is_read() {
        let fs = MemoryFileSystem::new().with_file(
            "/p/_codev/config.yaml",
            "changelog:\n  exclude:\n    - 2026-01-01-by-hand\n",
        );
        let config = load(&fs, Path::new("/p/_codev/config.yaml"))
            .unwrap()
            .unwrap();
        assert_eq!(config.changelog.exclude, ["2026-01-01-by-hand"]);
        let fs = MemoryFileSystem::new().with_file("/p/_codev/config.yaml", "schema: x\n");
        let config = load(&fs, Path::new("/p/_codev/config.yaml"))
            .unwrap()
            .unwrap();
        assert!(config.changelog.exclude.is_empty());
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
    fn mcp_config_unknown_field_is_tolerated() {
        // Tolerated, not silent: the key is collected and reported as an
        // `unknown_config_key` warning.
        let config = load_str("schema: spec-driven\nmcp:\n  jiraTool: mcp__foo\n")
            .unwrap()
            .unwrap();
        assert!(config.mcp.jira_tool.is_none());
        assert_eq!(config.unknown_keys[0].key, "mcp.jiraTool");
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

    #[test]
    fn codev_version_is_read_from_the_project_config() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "codev_version: \">=0.5\"\n");
        let config = resolve(&fs, &env_with_home(), &Layout::new("/p")).unwrap();
        assert_eq!(config.codev_version.as_deref(), Some(">=0.5"));
        let fs =
            MemoryFileSystem::new().with_file("/p/_codev/config.yaml", "schema: spec-driven\n");
        let config = resolve(&fs, &env_with_home(), &Layout::new("/p")).unwrap();
        assert!(config.codev_version.is_none());
    }

    #[test]
    fn an_invalid_codev_version_fails_with_its_code() {
        let fs =
            MemoryFileSystem::new().with_file("/p/_codev/config.yaml", "codev_version: latest\n");
        let err = resolve(&fs, &env_with_home(), &Layout::new("/p")).unwrap_err();
        assert_eq!(err.code(), "invalid_codev_version");
        assert!(err.to_string().contains("latest"), "{err}");
    }
}
