use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use codev_core::Layout;
use serde::Deserialize;

use crate::error::{EngineError, Result, Warning};
use crate::ports::{Env, FileSystem};

pub const DEFAULT_SCHEMA: &str = "spec-driven";

/// Le `_codev/config.yaml` tel qu'il est écrit sur le disque.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectConfig {
    #[serde(default)]
    pub schema: Option<String>,
    /// Les workflows à installer. `None` laisse `codev-agents` choisir son
    /// catalogue par défaut : c'est lui qui sait ce qui existe.
    #[serde(default)]
    pub workflows: Option<Vec<String>>,
    #[serde(default)]
    pub context: Option<String>,
    /// Règles par identifiant d'artefact.
    #[serde(default)]
    pub rules: BTreeMap<String, Vec<String>>,
    #[serde(default)]
    pub inherits: Vec<InheritSource>,
    /// Configuration des MCP utilisés par les skills. Champ optionnel :
    /// un projet sans MCP branché n'a rien à déclarer, et les skills se
    /// rendent avec un fallback informatif (« MCP Jira non configuré »).
    #[serde(default)]
    pub mcp: McpConfig,
}

/// Noms des outils MCP à injecter dans les skills à l'installation.
///
/// Chaque champ correspond à un usage (Jira, plus tard Design, Confluence,
/// GitHub…). Le nom du tool est spécifique à l'environnement Claude Code
/// de chaque utilisateur — c'est pour ça qu'on le configure projet par
/// projet plutôt que de le hardcoder dans le CATALOG.
#[derive(Debug, Clone, Default, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct McpConfig {
    /// Le tool MCP à appeler pour récupérer un ticket Jira mentionné dans
    /// le prompt d'un `/codev-propose`. Exemples :
    /// `mcp__claude_ai_Atlassian_Rovo__getJiraIssue`,
    /// `mcp__claude_ai_Atlassian__getJiraIssue`.
    #[serde(default)]
    pub jira_tool: Option<String>,
}

/// Une source héritée en lecture seule.
///
/// Un seul type avec des champs optionnels, plutôt qu'une énumération
/// `untagged` : les erreurs de serde sur une énumération untagged se réduisent
/// à « aucune variante ne correspond », ce qui n'aide personne face à un YAML
/// écrit à la main. Ici la validation est la nôtre, donc le message aussi.
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

/// Un bloc de contexte ou de règle, avec sa provenance.
///
/// La provenance n'est pas décorative : l'agent doit pouvoir dire « cette règle
/// vient de codev-decisions », et signaler une contradiction entre deux sources
/// au lieu de la trancher en silence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    pub origin: String,
    pub text: String,
}

/// La configuration effective, sources héritées fusionnées.
#[derive(Debug, Clone)]
pub struct ResolvedConfig {
    pub schema: String,
    pub workflows: Option<Vec<String>>,
    /// Du plus général au plus spécifique. En cas de contradiction, le dernier
    /// bloc prime — et l'agent a pour consigne de signaler la contradiction.
    pub context: Vec<Block>,
    pub rules: BTreeMap<String, Vec<Block>>,
    /// Configuration MCP — projet uniquement, pas d'héritage : le nom
    /// des tools dépend de la config Claude Code de l'utilisateur, pas
    /// du projet source.
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

/// Lit un `config.yaml`. Un fichier absent n'est pas une erreur : un projet
/// peut vivre sans configuration.
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
    let config: ProjectConfig =
        serde_norway::from_str(&raw).map_err(|e| EngineError::Invalid {
            path: path.to_path_buf(),
            reason: e.to_string(),
        })?;
    Ok(Some(config))
}

/// Résout la configuration effective du projet, sources héritées comprises.
pub fn resolve(fs: &dyn FileSystem, env: &dyn Env, layout: &Layout) -> Result<ResolvedConfig> {
    let config_path = layout.config_file();
    let project = load(fs, &config_path)?.unwrap_or_default();

    let mut context = Vec::new();
    let mut rules: BTreeMap<String, Vec<Block>> = BTreeMap::new();
    let mut warnings = Vec::new();

    // Les sources d'abord, dans l'ordre déclaré : elles sont plus générales que
    // le projet, qui doit pouvoir les affiner en dernier.
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
                                    "la source « {raw} » hérite elle-même d'autres sources ; \
                                     l'héritage n'est pas transitif, ces sources sont ignorées. \
                                     Déclare-les directement dans {}",
                                    config_path.display()
                                ),
                            ));
                        }
                        push_blocks(&inherited, &origin, &mut context, &mut rules);
                    }
                    Ok(None) => warnings.push(Warning::new(
                        "inherit_unresolved",
                        format!(
                            "la source héritée « {raw} » n'a pas de {} — vérifie le chemin, \
                             ou initialise-la avec `codev init`",
                            source_config_path.display()
                        ),
                    )),
                    Err(err) => warnings.push(Warning::new(
                        "inherit_unreadable",
                        format!("la source héritée « {raw} » est illisible : {err}"),
                    )),
                }
            }
            Source::Git { url, git_ref, subpath } => {
                let git_content_dir = resolve_git_source_dir(fs, env, layout, &url, &git_ref);
                match git_content_dir {
                    GitResolution::Ok(content_root) => {
                        let source_layout = Layout::new(&content_root);
                        let source_config_path = if let Some(sub) = &subpath {
                            content_root.join(sub).join("_codev").join("config.yaml")
                        } else {
                            source_layout.config_file()
                        };
                        let origin = format!("git:{url}");
                        if let Ok(Some(inherited)) = load(fs, &source_config_path) {
                            if !inherited.inherits.is_empty() {
                                warnings.push(Warning::new(
                                    "inherit_not_transitive",
                                    format!(
                                        "la source « {url} » hérite elle-même d'autres sources ; \
                                         l'héritage n'est pas transitif, ces sources sont ignorées"
                                    ),
                                ));
                            }
                            push_blocks(&inherited, &origin, &mut context, &mut rules);
                        }
                    }
                    GitResolution::Unlocked => warnings.push(Warning::new(
                        "git_source_unlocked",
                        format!(
                            "la source git « {url} » n'a pas de SHA verrouillé — \
                             lance `codev sources update` pour résoudre"
                        ),
                    )),
                    GitResolution::CacheMissing => warnings.push(Warning::new(
                        "git_source_needs_update",
                        format!(
                            "la source git « {url} » est verrouillée mais absente du cache local — \
                             lance `codev sources update`"
                        ),
                    )),
                }
            }
        }
    }

    push_blocks(&project, "projet", &mut context, &mut rules);

    Ok(ResolvedConfig {
        schema: project
            .schema
            .clone()
            .unwrap_or_else(|| DEFAULT_SCHEMA.to_string()),
        workflows: project.workflows.clone(),
        context,
        rules,
        // MCP est projet-uniquement : pas de fusion avec les sources
        // héritées. Le nom des tools dépend de la config Claude Code de
        // l'utilisateur, pas d'un dépôt partagé.
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

/// État de résolution d'une source `git:` par consultation du lock.
enum GitResolution {
    /// SHA verrouillé, contenu présent dans le cache — chemin retourné.
    Ok(std::path::PathBuf),
    /// Aucun `codev.lock` ou aucune entrée pour cette source.
    Unlocked,
    /// SHA verrouillé mais dossier `content/<sha>/` absent du cache.
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
            reason: "une source héritée déclare à la fois `path` et `git` ; choisis l'un des deux"
                .into(),
        }),
        (None, None) => Err(EngineError::Invalid {
            path: config_path.to_path_buf(),
            reason: "une source héritée ne déclare ni `path` ni `git`".into(),
        }),
        (Some(path), None) => {
            if source.git_ref.is_some() || source.subpath.is_some() {
                return Err(EngineError::Invalid {
                    path: config_path.to_path_buf(),
                    reason: format!(
                        "la source « {path} » est locale : `ref` et `subpath` ne s'appliquent \
                         qu'à une source `git`"
                    ),
                });
            }
            Ok(Source::Path(path.clone()))
        }
        (None, Some(url)) => {
            let git_ref = source.git_ref.clone().ok_or_else(|| EngineError::Invalid {
                path: config_path.to_path_buf(),
                reason: format!(
                    "la source git « {url} » n'a pas de `ref` — précise la branche ou le tag"
                ),
            })?;
            Ok(Source::Git {
                url: url.clone(),
                git_ref,
                subpath: source.subpath.clone(),
            })
        }
    }
}

/// Développe un `~` en tête de chemin.
///
/// Seulement en tête, et seulement suivi d'une fin de chaîne ou d'un séparateur
/// — `~machin` n'est pas un dossier personnel d'utilisateur ici.
pub(crate) fn expand_tilde(raw: &str, env: &dyn Env) -> PathBuf {
    let reste = match raw.strip_prefix('~') {
        Some("") => "",
        Some(reste) if reste.starts_with('/') => &reste[1..],
        _ => return PathBuf::from(raw),
    };
    match env.home_dir() {
        Some(home) if reste.is_empty() => home,
        Some(home) => home.join(reste),
        None => PathBuf::from(raw),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ports::{FixedEnv, MemoryFileSystem};

    fn env_avec_home() -> FixedEnv {
        let mut env = FixedEnv::at("/p");
        env.vars.insert("HOME".into(), "/Users/moi".into());
        env
    }

    #[test]
    fn un_projet_sans_config_prend_les_valeurs_par_defaut() {
        let fs = MemoryFileSystem::new().with_file("/p/_codev/specs/.gitkeep", "");
        let resolved = resolve(&fs, &env_avec_home(), &Layout::new("/p")).unwrap();
        assert_eq!(resolved.schema, DEFAULT_SCHEMA);
        assert!(resolved.workflows.is_none());
        assert!(resolved.context.is_empty());
        assert!(resolved.warnings.is_empty());
    }

    #[test]
    fn le_projet_passe_apres_ses_sources() {
        let fs = MemoryFileSystem::new()
            .with_file(
                "/Users/moi/partage/_codev/config.yaml",
                "context: |\n  Convention maison\nrules:\n  specs:\n    - Règle partagée\n",
            )
            .with_file(
                "/p/_codev/config.yaml",
                "inherits:\n  - path: ~/partage\ncontext: |\n  Propre au projet\nrules:\n  specs:\n    - Règle locale\n",
            );

        let resolved = resolve(&fs, &env_avec_home(), &Layout::new("/p")).unwrap();

        assert_eq!(
            resolved
                .context
                .iter()
                .map(|b| (b.origin.as_str(), b.text.as_str()))
                .collect::<Vec<_>>(),
            [
                ("path:~/partage", "Convention maison"),
                ("projet", "Propre au projet"),
            ],
            "l'ordre va du plus général au plus spécifique"
        );
        assert_eq!(
            resolved
                .rules_for("specs")
                .iter()
                .map(|b| b.text.as_str())
                .collect::<Vec<_>>(),
            ["Règle partagée", "Règle locale"]
        );
        assert!(resolved.warnings.is_empty());
    }

    #[test]
    fn une_source_introuvable_avertit_sans_faire_echouer() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "inherits:\n  - path: /nulle/part\n");
        let resolved = resolve(&fs, &env_avec_home(), &Layout::new("/p")).unwrap();
        assert_eq!(resolved.schema, DEFAULT_SCHEMA);
        assert_eq!(resolved.warnings.len(), 1);
        assert_eq!(resolved.warnings[0].code, "inherit_unresolved");
    }

    #[test]
    fn une_source_git_sans_lock_est_signalee() {
        // Le warning historique `inherit_git_unsupported` est remplacé par
        // `git_source_unlocked` — la source est reconnue, on demande juste à
        // l'utilisateur de la verrouiller.
        let fs = MemoryFileSystem::new().with_file(
            "/p/_codev/config.yaml",
            "inherits:\n  - git: git@github.com:o/r.git\n    ref: main\n",
        );
        let resolved = resolve(&fs, &env_avec_home(), &Layout::new("/p")).unwrap();
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
            "l'ancien warning ne doit plus être émis"
        );
    }

    #[test]
    fn une_source_git_verrouillee_est_resolue() {
        let lock = "version = 1\n\n[[source]]\ngit = \"url\"\nref = \"main\"\ncommit = \"deadbeef\"\nresolved_at = \"2026-09-08\"\n";
        let fs = MemoryFileSystem::new()
            .with_file(
                "/p/_codev/config.yaml",
                "inherits:\n  - git: url\n    ref: main\n",
            )
            .with_file("/p/_codev/codev.lock", lock)
            // Contenu du cache : une config héritée bien formée.
            .with_file(
                "/Users/moi/.cache/codev/content/deadbeef/_codev/config.yaml",
                "context: |\n  Contexte hérité par git\n",
            );
        let resolved = resolve(&fs, &env_avec_home(), &Layout::new("/p")).unwrap();
        assert!(resolved.warnings.is_empty(), "{:?}", resolved.warnings);
        assert!(resolved
            .context
            .iter()
            .any(|b| b.origin == "git:url" && b.text.contains("hérité par git")));
    }

    #[test]
    fn une_source_git_verrouillee_sans_cache_demande_update() {
        let lock = "version = 1\n\n[[source]]\ngit = \"url\"\nref = \"main\"\ncommit = \"deadbeef\"\nresolved_at = \"2026-09-08\"\n";
        let fs = MemoryFileSystem::new()
            .with_file(
                "/p/_codev/config.yaml",
                "inherits:\n  - git: url\n    ref: main\n",
            )
            .with_file("/p/_codev/codev.lock", lock);
        // Aucun contenu dans le cache.
        let resolved = resolve(&fs, &env_avec_home(), &Layout::new("/p")).unwrap();
        assert!(resolved
            .warnings
            .iter()
            .any(|w| w.code == "git_source_needs_update"));
    }

    #[test]
    fn une_source_git_sans_ref_est_refusee() {
        let fs = MemoryFileSystem::new().with_file(
            "/p/_codev/config.yaml",
            "inherits:\n  - git: url\n",
        );
        let err = resolve(&fs, &env_avec_home(), &Layout::new("/p")).unwrap_err();
        assert_eq!(err.code(), "invalid");
        assert!(err.to_string().contains("`ref`"), "{err}");
    }

    #[test]
    fn lheritage_nest_pas_transitif_et_le_dit() {
        let fs = MemoryFileSystem::new()
            .with_file(
                "/Users/moi/a/_codev/config.yaml",
                "inherits:\n  - path: ~/b\ncontext: |\n  Depuis A\n",
            )
            .with_file("/Users/moi/b/_codev/config.yaml", "context: |\n  Depuis B\n")
            .with_file("/p/_codev/config.yaml", "inherits:\n  - path: ~/a\n");

        let resolved = resolve(&fs, &env_avec_home(), &Layout::new("/p")).unwrap();

        assert_eq!(resolved.context.len(), 1, "seul A est hérité");
        assert_eq!(resolved.context[0].text, "Depuis A");
        assert_eq!(resolved.warnings[0].code, "inherit_not_transitive");
    }

    #[test]
    fn refuse_une_source_ambigue() {
        let fs = MemoryFileSystem::new().with_file(
            "/p/_codev/config.yaml",
            "inherits:\n  - path: /a\n    git: git@github.com:o/r.git\n",
        );
        let err = resolve(&fs, &env_avec_home(), &Layout::new("/p")).unwrap_err();
        assert_eq!(err.code(), "invalid");
        assert!(err.to_string().contains("choisis l'un des deux"), "{err}");
    }

    #[test]
    fn refuse_ref_sur_une_source_locale() {
        let fs = MemoryFileSystem::new().with_file(
            "/p/_codev/config.yaml",
            "inherits:\n  - path: /a\n    ref: main\n",
        );
        let err = resolve(&fs, &env_avec_home(), &Layout::new("/p")).unwrap_err();
        assert!(err.to_string().contains("`ref` et `subpath`"), "{err}");
    }

    #[test]
    fn signale_une_cle_inconnue_dans_la_config() {
        let fs = MemoryFileSystem::new().with_file("/p/_codev/config.yaml", "shema: spec-driven\n");
        let err = resolve(&fs, &env_avec_home(), &Layout::new("/p")).unwrap_err();
        assert_eq!(err.code(), "invalid");
        assert!(err.to_string().contains("shema"), "{err}");
    }

    #[test]
    fn developpe_le_tilde() {
        let env = env_avec_home();
        assert_eq!(expand_tilde("~/x", &env), PathBuf::from("/Users/moi/x"));
        assert_eq!(expand_tilde("~", &env), PathBuf::from("/Users/moi"));
        assert_eq!(expand_tilde("/abs", &env), PathBuf::from("/abs"));
        assert_eq!(expand_tilde("~machin", &env), PathBuf::from("~machin"));
    }

    // ─────────────── configuration MCP ───────────────

    #[test]
    fn config_mcp_absent_donne_default_vide() {
        let yaml = "schema: spec-driven\n";
        let cfg: ProjectConfig = serde_norway::from_str(yaml).unwrap();
        assert_eq!(cfg.mcp, McpConfig::default());
        assert!(cfg.mcp.jira_tool.is_none());
    }

    #[test]
    fn config_mcp_jira_tool_est_lu() {
        let yaml = "schema: spec-driven\nmcp:\n  jira_tool: mcp__foo__getJiraIssue\n";
        let cfg: ProjectConfig = serde_norway::from_str(yaml).unwrap();
        assert_eq!(
            cfg.mcp.jira_tool.as_deref(),
            Some("mcp__foo__getJiraIssue")
        );
    }

    #[test]
    fn config_mcp_champ_inconnu_est_refuse() {
        // deny_unknown_fields verrouille McpConfig — une faute de frappe
        // ne passe pas silencieusement.
        let yaml = "schema: spec-driven\nmcp:\n  jiraTool: mcp__foo\n";
        let err = serde_norway::from_str::<ProjectConfig>(yaml).unwrap_err();
        assert!(err.to_string().contains("jiraTool"), "{err}");
    }

    #[test]
    fn resolve_propage_mcp_du_projet() {
        let fs = MemoryFileSystem::new().with_file(
            "/p/_codev/config.yaml",
            "mcp:\n  jira_tool: mcp__bar__get\n",
        );
        let resolved = resolve(&fs, &env_avec_home(), &Layout::new("/p")).unwrap();
        assert_eq!(resolved.mcp.jira_tool.as_deref(), Some("mcp__bar__get"));
    }
}
