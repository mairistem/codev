//! Résolution et téléchargement des sources git — la seule commande de
//! codev qui touche à internet.

use std::path::PathBuf;

use crate::config::InheritSource;
use crate::error::EngineError;
use crate::ports::{Clock, Env, FileSystem, ProcessRunner};

use super::cache;
use super::lockfile::{self, LockEntry, Lockfile};

/// Un fetch à effectuer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FetchStep {
    pub url: String,
    pub bare_dir: PathBuf,
    pub sha: String,
    pub content_dir: PathBuf,
}

/// L'état d'une résolution — inchangé / nouveau / déplacé.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PinChange {
    Added {
        url: String,
        git_ref: String,
        to: String,
    },
    Moved {
        url: String,
        git_ref: String,
        from: String,
        to: String,
    },
    Unchanged {
        url: String,
        git_ref: String,
        sha: String,
    },
}

impl PinChange {
    pub fn is_change(&self) -> bool {
        !matches!(self, Self::Unchanged { .. })
    }
}

#[derive(Debug, Clone)]
pub struct SourcesUpdatePlan {
    pub fetches: Vec<FetchStep>,
    pub new_lock: Lockfile,
    pub diff: Vec<PinChange>,
}

/// Construit le plan à partir des sources déclarées, du lock actuel, et
/// des SHA fraîchement résolus par `git ls-remote`.
pub fn plan_sources_update(
    inherits_git: &[GitSourceInput],
    current_lock: Option<&Lockfile>,
    resolutions: &[(String, String, String)], // (url, ref, sha)
    today: &str,
) -> SourcesUpdatePlan {
    let cache_root = std::path::Path::new(""); // rempli à l'exécution
    let mut fetches = Vec::new();
    let mut new_entries = Vec::new();
    let mut diff = Vec::new();

    for src in inherits_git {
        let sha = resolutions
            .iter()
            .find(|(u, r, _)| u == &src.git && r == &src.git_ref)
            .map(|(_, _, sha)| sha.clone());
        let Some(sha) = sha else {
            continue;
        };
        // Calculer l'état par rapport au lock actuel.
        let previous = LockEntry::find_matching(current_lock, &src.git, &src.git_ref);
        match previous {
            Some(prev) if prev.commit == sha => {
                diff.push(PinChange::Unchanged {
                    url: src.git.clone(),
                    git_ref: src.git_ref.clone(),
                    sha: sha.clone(),
                });
            }
            Some(prev) => diff.push(PinChange::Moved {
                url: src.git.clone(),
                git_ref: src.git_ref.clone(),
                from: prev.commit.clone(),
                to: sha.clone(),
            }),
            None => diff.push(PinChange::Added {
                url: src.git.clone(),
                git_ref: src.git_ref.clone(),
                to: sha.clone(),
            }),
        }

        // Un fetch par entrée qui a résolu — le fetch en cache peut être
        // no-op côté git si le SHA est déjà présent, on ne le détecte pas
        // ici.
        let url_hash = cache::url_hash(&src.git);
        fetches.push(FetchStep {
            url: src.git.clone(),
            bare_dir: cache::bare_repo_dir(cache_root, &url_hash),
            sha: sha.clone(),
            content_dir: cache::content_dir(cache_root, &sha),
        });

        new_entries.push(LockEntry {
            git: src.git.clone(),
            git_ref: src.git_ref.clone(),
            subpath: src.subpath.clone(),
            commit: sha,
            resolved_at: today.to_string(),
        });
    }

    SourcesUpdatePlan {
        fetches,
        new_lock: Lockfile {
            version: 1,
            sources: new_entries,
        },
        diff,
    }
}

/// Entrée typée pour le plan — sans les Option de la déclaration YAML.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitSourceInput {
    pub git: String,
    pub git_ref: String,
    pub subpath: Option<String>,
}

impl GitSourceInput {
    /// Extrait les entrées `git:` d'une liste `inherits`. Les entrées sans
    /// URL, ou avec un URL git mais sans `ref`, sont ignorées silencieusement
    /// (le validateur les remontera ailleurs).
    pub fn from_inherits(inherits: &[InheritSource]) -> Vec<Self> {
        inherits
            .iter()
            .filter_map(|src| {
                let git = src.git.clone()?;
                let git_ref = src.git_ref.clone()?;
                Some(Self {
                    git,
                    git_ref,
                    subpath: src.subpath.clone(),
                })
            })
            .collect()
    }
}

/// Ce que `run_sources_update` a effectivement fait.
#[derive(Debug)]
pub struct UpdateOutcome {
    pub root: PathBuf,
    pub diff: Vec<PinChange>,
    pub lock_written: bool,
}

/// Exécute l'update : résolution des refs, fetches dans le cache, écriture
/// du lock.
pub fn run_sources_update(
    fs: &dyn FileSystem,
    env: &dyn Env,
    runner: &dyn ProcessRunner,
    clock: &dyn Clock,
    layout: &codev_core::Layout,
    inherits_git: &[GitSourceInput],
) -> crate::error::Result<UpdateOutcome> {
    if inherits_git.is_empty() {
        return Ok(UpdateOutcome {
            root: layout.project_root().to_path_buf(),
            diff: Vec::new(),
            lock_written: false,
        });
    }

    // 1. Vérifier que `git` est disponible.
    match runner.run("git", &["--version"], None) {
        Ok(out) if out.exit_code == 127 => {
            return Err(EngineError::Invalid {
                path: PathBuf::from("git"),
                reason: format!("git_not_found: {}", out.stderr_str().trim()),
            });
        }
        Ok(_) => {}
        Err(err) => {
            return Err(EngineError::Invalid {
                path: PathBuf::from("git"),
                reason: format!("git_not_found: {err}"),
            });
        }
    }

    // 2. `ls-remote` pour chaque source.
    let mut resolutions = Vec::new();
    for src in inherits_git {
        let out = runner
            .run("git", &["ls-remote", &src.git, &src.git_ref], None)
            .map_err(|e| EngineError::Invalid {
                path: PathBuf::from(&src.git),
                reason: format!("ls-remote a échoué : {e}"),
            })?;
        if !out.is_ok() {
            return Err(EngineError::Invalid {
                path: PathBuf::from(&src.git),
                reason: format!(
                    "ls-remote pour « {} » a échoué : {}",
                    src.git,
                    out.stderr_str().trim()
                ),
            });
        }
        let stdout = out.stdout_str();
        // Ligne du format `<sha>\t<ref>`
        let sha = stdout
            .lines()
            .next()
            .and_then(|l| l.split_whitespace().next())
            .ok_or_else(|| EngineError::Invalid {
                path: PathBuf::from(&src.git),
                reason: format!(
                    "sortie ls-remote inattendue pour « {} »",
                    src.git
                ),
            })?;
        resolutions.push((src.git.clone(), src.git_ref.clone(), sha.to_string()));
    }

    // 3. Construire le plan.
    let today = clock.today();
    let cache_root = cache::root(env);
    let lock_path = layout.planning_dir().join("codev.lock");
    let current_lock = lockfile::load(fs, &lock_path)?;
    let plan = plan_sources_update(
        inherits_git,
        current_lock.as_ref(),
        &resolutions,
        &today,
    );

    // 4. Fetches — on ne les rejoue pas si le contenu est déjà en cache.
    for fetch in &plan.fetches {
        let bare = cache_root.join("git").join(cache::url_hash(&fetch.url));
        let content = cache_root.join("content").join(&fetch.sha);

        // Init bare repo si absent.
        if !fs.exists(&bare) {
            fs.create_dir_all(&bare).map_err(|e| EngineError::Write {
                path: bare.clone(),
                source: e,
            })?;
            let out = runner
                .run("git", &["init", "--bare", bare.to_str().unwrap_or("")], None)
                .map_err(|e| EngineError::Invalid {
                    path: bare.clone(),
                    reason: format!("git init a échoué : {e}"),
                })?;
            if !out.is_ok() {
                return Err(EngineError::Invalid {
                    path: bare.clone(),
                    reason: format!("git init : {}", out.stderr_str().trim()),
                });
            }
        }

        // Fetch le SHA.
        let out = runner
            .run(
                "git",
                &[
                    "-C",
                    bare.to_str().unwrap_or(""),
                    "fetch",
                    "--depth",
                    "1",
                    "--filter=blob:none",
                    &fetch.url,
                    &fetch.sha,
                ],
                None,
            )
            .map_err(|e| EngineError::Invalid {
                path: bare.clone(),
                reason: format!("git fetch : {e}"),
            })?;
        if !out.is_ok() {
            return Err(EngineError::Invalid {
                path: bare.clone(),
                reason: format!("git fetch : {}", out.stderr_str().trim()),
            });
        }

        // Extraire le contenu du SHA vers `content/<sha>/` si absent.
        if !fs.exists(&content) {
            fs.create_dir_all(&content).map_err(|e| EngineError::Write {
                path: content.clone(),
                source: e,
            })?;
            let out = runner
                .run(
                    "git",
                    &[
                        "-C",
                        bare.to_str().unwrap_or(""),
                        "worktree",
                        "add",
                        "--detach",
                        content.to_str().unwrap_or(""),
                        &fetch.sha,
                    ],
                    None,
                )
                .map_err(|e| EngineError::Invalid {
                    path: content.clone(),
                    reason: format!("git worktree : {e}"),
                })?;
            if !out.is_ok() {
                return Err(EngineError::Invalid {
                    path: content.clone(),
                    reason: format!("git worktree : {}", out.stderr_str().trim()),
                });
            }
        }
    }

    // 5. Écriture du lock, comparaison contenu-à-contenu pour idempotence.
    let serialized = lockfile::serialize(&plan.new_lock).map_err(|reason| {
        EngineError::Invalid {
            path: lock_path.clone(),
            reason,
        }
    })?;
    let lock_written = match fs.read_to_string(&lock_path) {
        Ok(existing) if existing == serialized => false,
        _ => {
            fs.write(&lock_path, &serialized).map_err(|e| EngineError::Write {
                path: lock_path.clone(),
                source: e,
            })?;
            true
        }
    };

    Ok(UpdateOutcome {
        root: layout.project_root().to_path_buf(),
        diff: plan.diff,
        lock_written,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ports::{FixedClock, FixedEnv, MemoryFileSystem, MockProcessRunner, ProcessOutput};
    use codev_core::Layout;

    fn env() -> FixedEnv {
        let mut env = FixedEnv::at("/p");
        env.vars.insert("HOME".into(), "/home".into());
        env
    }

    fn source(git: &str, git_ref: &str) -> GitSourceInput {
        GitSourceInput {
            git: git.into(),
            git_ref: git_ref.into(),
            subpath: None,
        }
    }

    fn resolutions_ok() -> Vec<(String, String, String)> {
        vec![(
            "git@github.com:acme/shared.git".into(),
            "main".into(),
            "9f2c1ab7".into(),
        )]
    }

    #[test]
    fn plan_produit_un_fetch_par_nouvelle_source() {
        let sources = vec![source("git@github.com:acme/shared.git", "main")];
        let plan = plan_sources_update(&sources, None, &resolutions_ok(), "2026-09-08");
        assert_eq!(plan.fetches.len(), 1);
        assert_eq!(plan.new_lock.sources.len(), 1);
        assert!(matches!(plan.diff[0], PinChange::Added { .. }));
    }

    #[test]
    fn diff_distingue_add_move_unchanged() {
        let sources = vec![source("url", "main")];
        // Add.
        let p = plan_sources_update(
            &sources,
            None,
            &[("url".into(), "main".into(), "aa".into())],
            "2026-09-08",
        );
        assert!(matches!(p.diff[0], PinChange::Added { .. }));

        // Unchanged.
        let lock = Lockfile {
            version: 1,
            sources: vec![LockEntry {
                git: "url".into(),
                git_ref: "main".into(),
                subpath: None,
                commit: "aa".into(),
                resolved_at: "2026-09-08".into(),
            }],
        };
        let p = plan_sources_update(
            &sources,
            Some(&lock),
            &[("url".into(), "main".into(), "aa".into())],
            "2026-09-08",
        );
        assert!(matches!(p.diff[0], PinChange::Unchanged { .. }));

        // Moved.
        let p = plan_sources_update(
            &sources,
            Some(&lock),
            &[("url".into(), "main".into(), "bb".into())],
            "2026-09-08",
        );
        match &p.diff[0] {
            PinChange::Moved { from, to, .. } => {
                assert_eq!(from, "aa");
                assert_eq!(to, "bb");
            }
            other => panic!("attendu Moved, obtenu {other:?}"),
        }
    }

    #[test]
    fn run_avec_mock_runner_ecrit_le_lock() {
        let fs = MemoryFileSystem::new().with_file("/p/_codev/config.yaml", "");
        let runner = MockProcessRunner::new()
            .with_response(
                "git",
                &["--version"],
                ProcessOutput {
                    stdout: b"git version 2.42.0\n".to_vec(),
                    stderr: Vec::new(),
                    exit_code: 0,
                },
            )
            .with_response(
                "git",
                &["ls-remote"],
                ProcessOutput {
                    stdout: b"9f2c1ab7\trefs/heads/main\n".to_vec(),
                    stderr: Vec::new(),
                    exit_code: 0,
                },
            )
            .with_response(
                "git",
                &["init"],
                ProcessOutput {
                    stdout: Vec::new(),
                    stderr: Vec::new(),
                    exit_code: 0,
                },
            )
            .with_response(
                "git",
                &["-C"], // couvre init, fetch, worktree via le préfixe court
                ProcessOutput {
                    stdout: Vec::new(),
                    stderr: Vec::new(),
                    exit_code: 0,
                },
            );

        let sources = vec![source("git@github.com:acme/shared.git", "main")];
        let outcome = run_sources_update(
            &fs,
            &env(),
            &runner,
            &FixedClock("2026-09-08".into()),
            &Layout::new("/p"),
            &sources,
        )
        .unwrap();

        assert!(outcome.lock_written);
        let lock_str = fs.read("/p/_codev/codev.lock").expect("lock écrit");
        assert!(lock_str.contains("git = \"git@github.com:acme/shared.git\""));
        assert!(lock_str.contains("commit = \"9f2c1ab7\""));
    }

    #[test]
    fn deuxieme_update_ne_reecrit_pas_le_lock() {
        let existing_lock = "version = 1\n\n[[source]]\ngit = \"url\"\nref = \"main\"\ncommit = \"aa\"\nresolved_at = \"2026-09-08\"\n";
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file("/p/_codev/codev.lock", existing_lock);
        let runner = MockProcessRunner::new()
            .with_response(
                "git",
                &["--version"],
                ProcessOutput {
                    stdout: b"git version 2.42\n".to_vec(),
                    stderr: Vec::new(),
                    exit_code: 0,
                },
            )
            .with_response(
                "git",
                &["ls-remote"],
                ProcessOutput {
                    stdout: b"aa\trefs/heads/main\n".to_vec(),
                    stderr: Vec::new(),
                    exit_code: 0,
                },
            )
            .with_response(
                "git",
                &["-C"],
                ProcessOutput {
                    stdout: Vec::new(),
                    stderr: Vec::new(),
                    exit_code: 0,
                },
            );

        // Simule un cache déjà peuplé.
        let sources = vec![source("url", "main")];
        // Injecter le bare et le content pour éviter un `init`.
        let bare_hash = cache::url_hash("url");
        let bare_path = format!("/home/.cache/codev/git/{bare_hash}");
        let content_path = "/home/.cache/codev/content/aa";
        let fs = fs
            .with_file(format!("{bare_path}/HEAD"), "ref: refs/heads/main")
            .with_file(format!("{content_path}/README.md"), "x");

        let outcome = run_sources_update(
            &fs,
            &env(),
            &runner,
            &FixedClock("2026-09-08".into()),
            &Layout::new("/p"),
            &sources,
        )
        .unwrap();

        assert!(matches!(outcome.diff[0], PinChange::Unchanged { .. }));
        assert!(!outcome.lock_written, "lock inchangé sur un update no-op");
    }

    #[test]
    fn git_absent_est_signale() {
        let fs = MemoryFileSystem::new();
        let runner = MockProcessRunner::new().with_response(
            "git",
            &["--version"],
            ProcessOutput {
                stdout: Vec::new(),
                stderr: b"git: command not found".to_vec(),
                exit_code: 127,
            },
        );
        let sources = vec![source("url", "main")];
        let err = run_sources_update(
            &fs,
            &env(),
            &runner,
            &FixedClock("2026-09-08".into()),
            &Layout::new("/p"),
            &sources,
        )
        .unwrap_err();
        assert!(err.to_string().contains("git_not_found"), "{err}");
    }

    #[test]
    fn aucune_source_git_est_un_no_op() {
        let fs = MemoryFileSystem::new();
        let runner = MockProcessRunner::new();
        let outcome = run_sources_update(
            &fs,
            &env(),
            &runner,
            &FixedClock("2026-09-08".into()),
            &Layout::new("/p"),
            &[],
        )
        .unwrap();
        assert!(outcome.diff.is_empty());
        assert!(!outcome.lock_written);
        assert!(runner.calls().is_empty());
    }
}
