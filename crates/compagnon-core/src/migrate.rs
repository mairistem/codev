//! `compagnon migrate`: what converting a project set up under the
//! product's former name takes — every decision, none of the effects.
//!
//! The engine gathers [`Facts`] from the disk; [`plan`] returns the ordered
//! operations; the engine executes them, then refreshes the skills and the
//! managed sections with the code `compagnon update` uses, so that a
//! migrated project is a current one. See
//! `_compagnon/decisions/0001-functional-core-imperative-shell.md`.
//!
//! The plan is computed from **what is still present**, never from the
//! roots alone: a run interrupted after renaming the root leaves a lock, a
//! key or a skill under the former name, and the next run completes it.
//!
//! The prose of the planning files is the user's: only the format markers
//! of `review.md` and `corrections.md` are rewritten.

use std::path::{Path, PathBuf};

use crate::layout::{CONFIG_FILE, LOCK_FILE, PLANNING_DIR};
use crate::{legacy, managed};

/// The current file name of the generated CI workflow, relative to the
/// project root.
pub const WORKFLOW: &str = ".github/workflows/compagnon.yml";

/// The configuration key holding the version requirement.
pub const VERSION_KEY: &str = "compagnon_version";

/// A skill installed under the former prefix.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormerSkill {
    /// Its folder, absolute: `<root>/.claude/skills/<former>-propose`.
    pub dir: PathBuf,
    /// Whether its `SKILL.md` is still the generated rendering — it then
    /// goes; a skill edited by hand is kept.
    pub generated: bool,
}

/// What the engine read from the disk.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Facts {
    pub project_root: PathBuf,
    /// The former planning root exists.
    pub former_root: bool,
    /// The current planning root exists.
    pub current_root: bool,
    /// The planning root holds the lock under its former name.
    pub former_lock: bool,
    /// The configuration file's text, from whichever root exists.
    pub config: Option<String>,
    /// The running version, to check the project's requirement against.
    pub running_version: String,
    /// `review.md` and `corrections.md` files, relative to the planning
    /// root, with their text.
    pub records: Vec<(PathBuf, String)>,
    /// Markdown files that may hold a managed section — `CLAUDE.md`, the
    /// pull request template — relative to the project root, with their
    /// text.
    pub managed_markdown: Vec<(PathBuf, String)>,
    /// The former generated workflow's text, when the file exists.
    pub former_workflow: Option<String>,
    /// The current workflow exists already.
    pub current_workflow: bool,
    pub former_skills: Vec<FormerSkill>,
    /// The project declares git sources: they are pinned to commits that
    /// may predate their own migration.
    pub has_git_sources: bool,
    /// A configured skill is missing or out of date under the current
    /// prefix: the refresh still has work to do.
    pub skills_need_refresh: bool,
}

/// One operation, with absolute paths.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Op {
    /// Moves a file or a directory.
    Rename {
        from: PathBuf,
        to: PathBuf,
    },
    /// Writes a file, replacing it.
    Write {
        path: PathBuf,
        contents: String,
    },
    Remove {
        path: PathBuf,
    },
    /// Removes a directory left empty by a `Remove`.
    RemoveDir {
        path: PathBuf,
    },
    /// Leaves a file under its former name, edited by hand: listed in the
    /// outcome's `kept`, as "kept <path> (edited by hand)".
    Keep {
        path: PathBuf,
    },
}

/// What `migrate` does.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
    /// No planning root at all.
    NoRoot,
    /// Both roots exist: which one holds the truth is the user's call.
    Conflict,
    /// The project's version requirement excludes the running version.
    VersionUnsatisfied { requirement: String },
    /// Nothing is left under the former names.
    AlreadyMigrated,
    /// The operations to run, then the refresh of skills and managed
    /// sections. `sources_update` tells the user to re-pin git sources.
    Migrate { ops: Vec<Op>, sources_update: bool },
}

/// Decides the migration.
pub fn plan(facts: &Facts) -> Outcome {
    match (facts.former_root, facts.current_root) {
        (false, false) => return Outcome::NoRoot,
        (true, true) => return Outcome::Conflict,
        _ => {}
    }
    if let Some(requirement) = facts.config.as_deref().and_then(former_requirement)
        && crate::update::satisfies(&requirement, &facts.running_version) == Some(false)
    {
        return Outcome::VersionUnsatisfied { requirement };
    }

    let root = &facts.project_root;
    let planning = root.join(PLANNING_DIR);
    let mut ops = Vec::new();

    if facts.former_root {
        ops.push(Op::Rename {
            from: root.join(legacy::PLANNING_DIR),
            to: planning.clone(),
        });
    }
    if facts.former_lock {
        ops.push(Op::Rename {
            from: planning.join(legacy::LOCK_FILE),
            to: planning.join(LOCK_FILE),
        });
    }
    if let Some(config) = &facts.config
        && let Some(renamed) = rename_version_key(config)
    {
        ops.push(Op::Write {
            path: planning.join(CONFIG_FILE),
            contents: renamed,
        });
    }
    for (relative, text) in &facts.records {
        if let Some(rewritten) = rewrite_record_marker(text) {
            ops.push(Op::Write {
                path: planning.join(relative),
                contents: rewritten,
            });
        }
    }
    for (relative, text) in &facts.managed_markdown {
        if let Some(rewritten) = rewrite_managed_markers(text) {
            ops.push(Op::Write {
                path: root.join(relative),
                contents: rewritten,
            });
        }
    }
    if let Some(workflow) = &facts.former_workflow {
        let former = root.join(legacy::WORKFLOW);
        match workflow.strip_prefix(legacy::MANAGED_YAML_MARKER) {
            // Generated: it moves to its current name, which the refresh
            // then rewrites; an existing current workflow wins.
            Some(rest) => {
                if !facts.current_workflow {
                    ops.push(Op::Write {
                        path: root.join(WORKFLOW),
                        contents: format!("{}{rest}", managed::MANAGED_YAML_MARKER),
                    });
                }
                ops.push(Op::Remove { path: former });
            }
            None => ops.push(Op::Keep { path: former }),
        }
    }
    for skill in &facts.former_skills {
        if skill.generated {
            ops.push(Op::Remove {
                path: skill.dir.join("SKILL.md"),
            });
            ops.push(Op::RemoveDir {
                path: skill.dir.clone(),
            });
        } else {
            ops.push(Op::Keep {
                path: skill.dir.join("SKILL.md"),
            });
        }
    }

    let work_left = ops.iter().any(|op| !matches!(op, Op::Keep { .. }));
    if !work_left && !facts.skills_need_refresh {
        return Outcome::AlreadyMigrated;
    }
    Outcome::Migrate {
        ops,
        sources_update: facts.has_git_sources,
    }
}

/// The value of the former version key, when the configuration holds one.
fn former_requirement(config: &str) -> Option<String> {
    config.lines().find_map(|line| {
        let value = line.strip_prefix(legacy::VERSION_KEY)?.strip_prefix(':')?;
        let value = value.split(" #").next().unwrap_or(value).trim();
        Some(value.trim_matches(|c| c == '"' || c == '\'').to_string())
    })
}

/// `config` with the former version key renamed on its own line — every
/// other byte, comments and value included, untouched. `None` when the key
/// is absent.
pub fn rename_version_key(config: &str) -> Option<String> {
    let mut changed = false;
    let renamed: String = config
        .split_inclusive('\n')
        .map(|line| match line.strip_prefix(legacy::VERSION_KEY) {
            Some(rest) if rest.starts_with(':') => {
                changed = true;
                format!("{VERSION_KEY}{rest}")
            }
            _ => line.to_string(),
        })
        .collect();
    changed.then_some(renamed)
}

/// `text` with the format marker of a review record or a corrections file
/// renamed. Only a line holding nothing but the marker is touched.
pub fn rewrite_record_marker(text: &str) -> Option<String> {
    let mut changed = false;
    let rewritten: String = text
        .split_inclusive('\n')
        .map(|line| {
            let Some(inner) = line
                .trim_end()
                .strip_prefix("<!-- ")
                .and_then(|l| l.strip_suffix(" -->"))
            else {
                return line.to_string();
            };
            let current = [
                (legacy::REVIEW_MARKER, "compagnon-review:"),
                (legacy::CORRECTIONS_MARKER, "compagnon-corrections:"),
            ]
            .iter()
            .find_map(|(former, current)| {
                inner
                    .strip_prefix(former)
                    .map(|number| format!("<!-- {current}{number} -->"))
            });
            match current {
                Some(marker) => {
                    changed = true;
                    let ending = &line[line.trim_end().len()..];
                    format!("{marker}{ending}")
                }
                None => line.to_string(),
            }
        })
        .collect();
    changed.then_some(rewritten)
}

/// `text` with the former opening and closing lines of a managed section
/// replaced by the current ones; the section's content is left for the
/// refresh to rewrite. `None` when the text holds no former marker.
pub fn rewrite_managed_markers(text: &str) -> Option<String> {
    let mut changed = false;
    let rewritten: String = text
        .split_inclusive('\n')
        .map(|line| {
            let bare = line.trim_end();
            let ending = &line[bare.len()..];
            if bare.starts_with(legacy::BEGIN_PREFIX) {
                changed = true;
                format!("{}{ending}", managed::BEGIN_LINE)
            } else if bare == legacy::END_LINE {
                changed = true;
                format!("{}{ending}", managed::END_LINE)
            } else {
                line.to_string()
            }
        })
        .collect();
    changed.then_some(rewritten)
}

/// The path of an operation, relative to `root` when it lies under it.
pub fn relative<'a>(root: &Path, path: &'a Path) -> &'a Path {
    path.strip_prefix(root).unwrap_or(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn root() -> PathBuf {
        PathBuf::from("/p")
    }

    fn former(path: &str) -> PathBuf {
        root().join(legacy::PLANNING_DIR).join(path)
    }

    fn facts() -> Facts {
        Facts {
            project_root: root(),
            former_root: true,
            running_version: "0.7.0".into(),
            ..Facts::default()
        }
    }

    fn ops(outcome: Outcome) -> Vec<Op> {
        match outcome {
            Outcome::Migrate { ops, .. } => ops,
            other => panic!("expected a migration, got {other:?}"),
        }
    }

    #[test]
    fn a_codev_project_moves_its_root_lock_and_key() {
        let config = format!(
            "# Our setup.\nschema: spec-driven\n{}: \">=0.6\" # keep aligned\n",
            legacy::VERSION_KEY
        );
        let outcome = plan(&Facts {
            former_lock: true,
            config: Some(config),
            ..facts()
        });
        assert_eq!(
            ops(outcome),
            [
                Op::Rename {
                    from: former(""),
                    to: root().join("_compagnon"),
                },
                Op::Rename {
                    from: root().join("_compagnon").join(legacy::LOCK_FILE),
                    to: root().join("_compagnon/compagnon.lock"),
                },
                Op::Write {
                    path: root().join("_compagnon/config.yaml"),
                    contents: "# Our setup.\nschema: spec-driven\ncompagnon_version: \">=0.6\" # keep aligned\n"
                        .into(),
                },
            ]
        );
    }

    #[test]
    fn no_root_is_reported() {
        let outcome = plan(&Facts {
            former_root: false,
            ..facts()
        });
        assert_eq!(outcome, Outcome::NoRoot);
    }

    #[test]
    fn both_roots_are_a_conflict() {
        let outcome = plan(&Facts {
            current_root: true,
            ..facts()
        });
        assert_eq!(outcome, Outcome::Conflict);
    }

    #[test]
    fn a_migrated_project_is_already_migrated() {
        let outcome = plan(&Facts {
            former_root: false,
            current_root: true,
            config: Some("compagnon_version: \">=0.7\"\n".into()),
            ..facts()
        });
        assert_eq!(outcome, Outcome::AlreadyMigrated);
    }

    #[test]
    fn an_interrupted_migration_is_completed() {
        // The root was renamed, then the run stopped: the lock, the key and
        // a generated skill are still under their former names.
        let skill = root()
            .join(".claude/skills")
            .join(format!("{}-apply", legacy::SKILL_PREFIX));
        let outcome = plan(&Facts {
            former_root: false,
            current_root: true,
            former_lock: true,
            config: Some(format!("{}: \">=0.6\"\n", legacy::VERSION_KEY)),
            former_skills: vec![FormerSkill {
                dir: skill.clone(),
                generated: true,
            }],
            ..facts()
        });
        let ops = ops(outcome);
        assert!(
            !ops.iter()
                .any(|op| matches!(op, Op::Rename { from, .. } if from == &former("")))
        );
        assert!(ops.contains(&Op::Remove {
            path: skill.join("SKILL.md")
        }));
        assert!(ops.contains(&Op::RemoveDir { path: skill }));
        assert_eq!(ops.len(), 4, "{ops:?}");
    }

    #[test]
    fn missing_current_skills_still_need_the_refresh() {
        let outcome = plan(&Facts {
            former_root: false,
            current_root: true,
            skills_need_refresh: true,
            ..facts()
        });
        assert_eq!(
            outcome,
            Outcome::Migrate {
                ops: vec![],
                sources_update: false
            }
        );
    }

    #[test]
    fn an_unsatisfiable_requirement_is_refused_up_front() {
        let outcome = plan(&Facts {
            config: Some(format!("{}: \"^0.6\"\n", legacy::VERSION_KEY)),
            ..facts()
        });
        assert_eq!(
            outcome,
            Outcome::VersionUnsatisfied {
                requirement: "^0.6".into()
            }
        );
    }

    #[test]
    fn a_hand_edited_skill_is_kept() {
        let skill = root()
            .join(".claude/skills")
            .join(format!("{}-apply", legacy::SKILL_PREFIX));
        let ops = ops(plan(&Facts {
            former_skills: vec![FormerSkill {
                dir: skill.clone(),
                generated: false,
            }],
            ..facts()
        }));
        assert!(ops.contains(&Op::Keep {
            path: skill.join("SKILL.md")
        }));
        assert!(!ops.iter().any(|op| matches!(op, Op::Remove { .. })));
    }

    #[test]
    fn the_generated_workflow_moves_and_a_hand_written_one_stays() {
        let generated = format!("{}\nname: x\n", legacy::MANAGED_YAML_MARKER);
        let ops_generated = ops(plan(&Facts {
            former_workflow: Some(generated),
            ..facts()
        }));
        assert!(ops_generated.contains(&Op::Write {
            path: root().join(WORKFLOW),
            contents: format!("{}\nname: x\n", managed::MANAGED_YAML_MARKER),
        }));
        assert!(ops_generated.contains(&Op::Remove {
            path: root().join(legacy::WORKFLOW)
        }));

        let ops_hand = ops(plan(&Facts {
            former_workflow: Some("name: ours\n".into()),
            ..facts()
        }));
        assert!(ops_hand.contains(&Op::Keep {
            path: root().join(legacy::WORKFLOW)
        }));
    }

    #[test]
    fn managed_markers_are_renamed_and_the_rest_kept() {
        let text = format!(
            "# Ours\n\n{} — managed\nold body\n{}\n\nHand-written.\n",
            legacy::BEGIN_PREFIX,
            legacy::END_LINE
        );
        let rewritten = rewrite_managed_markers(&text).unwrap();
        assert_eq!(
            rewritten,
            format!(
                "# Ours\n\n{}\nold body\n{}\n\nHand-written.\n",
                managed::BEGIN_LINE,
                managed::END_LINE
            )
        );
        assert_eq!(rewrite_managed_markers("# Nothing managed\n"), None);
    }

    #[test]
    fn the_prose_of_planning_files_is_never_rewritten() {
        let prose = format!(
            "# Review record: x\n\n<!-- {}1 -->\n\nRun `{} validate`; see <!-- {}2 --> in the docs.\n",
            legacy::REVIEW_MARKER,
            legacy::BINARY_STEM,
            legacy::REVIEW_MARKER
        );
        let rewritten = rewrite_record_marker(&prose).unwrap();
        assert!(
            rewritten.contains("<!-- compagnon-review:1 -->\n"),
            "{rewritten}"
        );
        assert!(
            rewritten.contains(&format!("Run `{} validate`", legacy::BINARY_STEM)),
            "{rewritten}"
        );
        assert!(
            rewritten.contains(&format!("see <!-- {}2 --> in", legacy::REVIEW_MARKER)),
            "a marker inside a sentence is prose: {rewritten}"
        );
    }

    #[test]
    fn git_sources_ask_for_a_sources_update() {
        let outcome = plan(&Facts {
            has_git_sources: true,
            ..facts()
        });
        assert!(matches!(
            outcome,
            Outcome::Migrate {
                sources_update: true,
                ..
            }
        ));
    }
}
