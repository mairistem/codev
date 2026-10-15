//! `codev search` and `codev show`: reading the main specs, the decision
//! index and, on request, the archive through the ports, and handing their
//! text to the pure ranking of `codev-core::search`. Both only read.

use std::collections::{BTreeMap, BTreeSet};

use codev_core::Layout;
use codev_core::decisions::DecisionStatus;
use codev_core::parser::parse_spec;
use codev_core::search::{
    Document, SearchError, corpus_lang, find_requirement, rank, requirement_document,
    requirement_source,
};

use crate::config::ResolvedConfig;
use crate::decisions::{self, Origin};
use crate::error::{EngineError, Result};
use crate::ports::{Env, FileSystem};
use crate::specs;

/// The results printed unless `--limit` asks for another number.
pub const DEFAULT_LIMIT: usize = 10;
/// The most results `--limit` accepts: the output stays small whatever the
/// project's size.
pub const MAX_LIMIT: usize = 50;
/// The longest excerpt of a result, in characters.
const EXCERPT_CHARS: usize = 160;

/// A decision as a result shows it: enough to tell whether it is in force.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionHit {
    pub id: String,
    pub qualified_id: String,
    pub title: String,
    pub origin: String,
    pub status: String,
    pub in_effect: bool,
    /// The qualified identifier of the accepted decision that supersedes
    /// this one, if any.
    pub superseded_by: Option<String>,
}

/// One result: a requirement or a decision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Hit {
    Requirement {
        capability: String,
        requirement: String,
        excerpt: String,
    },
    Decision {
        decision: DecisionHit,
        excerpt: String,
    },
}

/// An archived change that matches, ranked apart.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchivedHit {
    pub change: String,
    pub excerpt: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchResult {
    pub query: String,
    /// The language the words were stemmed in (`en`, `fr`).
    pub language: String,
    pub results: Vec<Hit>,
    /// `Some` only with `--archive`.
    pub archived: Option<Vec<ArchivedHit>>,
}

/// The first `EXCERPT_CHARS` characters of a text, its whitespace collapsed.
fn excerpt(text: &str) -> String {
    let flat = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if flat.chars().count() <= EXCERPT_CHARS {
        return flat;
    }
    let cut: String = flat.chars().take(EXCERPT_CHARS - 1).collect();
    format!("{}…", cut.trim_end())
}

/// Ranks the requirements of the main specs and the decisions of the index
/// against `query`, and with `archive` the archived changes apart; at most
/// `limit` of each.
pub fn search(
    fs: &dyn FileSystem,
    env: &dyn Env,
    layout: &Layout,
    config: &ResolvedConfig,
    query: &str,
    limit: usize,
    archive: bool,
) -> Result<SearchResult> {
    if !(1..=MAX_LIMIT).contains(&limit) {
        return Err(EngineError::SearchLimitInvalid {
            limit,
            max: MAX_LIMIT,
        });
    }
    let mut documents = Vec::new();
    let mut hits: BTreeMap<String, Hit> = BTreeMap::new();
    let mut langs = Vec::new();
    for capability in specs::list(fs, layout) {
        let path = layout.spec_file(&capability);
        let source = fs
            .read_to_string(&path)
            .map_err(|e| EngineError::Unreadable {
                path: path.clone(),
                reason: e.to_string(),
            })?;
        let spec = parse_spec(&source).value;
        langs.push(spec.lang);
        for requirement in &spec.requirements {
            let document = requirement_document(&capability, requirement);
            let summary = if requirement.description.trim().is_empty() {
                requirement
                    .scenarios
                    .first()
                    .map(|s| s.body.as_str())
                    .unwrap_or("")
            } else {
                requirement.description.as_str()
            };
            hits.insert(
                document.id.clone(),
                Hit::Requirement {
                    capability: capability.clone(),
                    requirement: requirement.name.clone(),
                    excerpt: excerpt(summary),
                },
            );
            documents.push(document);
        }
    }
    let lang = corpus_lang(langs);

    let index = decisions::index(fs, env, layout, config)?;
    let in_effect: BTreeSet<String> = index.in_effect.iter().map(|q| q.as_str()).collect();
    let mut superseded_by: BTreeMap<String, String> = BTreeMap::new();
    for entry in &index.entries {
        if entry.decision.status == DecisionStatus::Accepted {
            for target in &entry.decision.supersedes {
                superseded_by.insert(target.clone(), entry.qualified_id.as_str());
            }
        }
    }
    for entry in &index.entries {
        let qualified = entry.qualified_id.as_str();
        let body: Vec<&str> = entry
            .decision
            .sections
            .iter()
            .map(|s| s.body.as_str())
            .collect();
        // A decision's identifier cannot collide with a requirement's,
        // which always holds `::`.
        let id = format!("decision {qualified}");
        documents.push(Document {
            id: id.clone(),
            text: format!("{}\n{}", entry.decision.title, body.join("\n")),
        });
        let status = match &entry.decision.status {
            DecisionStatus::Unknown(raw) => raw.clone(),
            other => other.as_str().to_string(),
        };
        let first = body.iter().find(|b| !b.trim().is_empty()).unwrap_or(&"");
        hits.insert(
            id,
            Hit::Decision {
                decision: DecisionHit {
                    id: entry.decision.id.clone(),
                    qualified_id: qualified.clone(),
                    title: entry.decision.title.clone(),
                    origin: match &entry.qualified_id.origin {
                        Origin::Project => "project".into(),
                        other => other.as_str(),
                    },
                    status,
                    in_effect: in_effect.contains(&qualified),
                    superseded_by: superseded_by.get(&entry.decision.id).cloned(),
                },
                excerpt: excerpt(first),
            },
        );
    }

    let ranked = rank(&documents, query, lang)
        .map_err(|SearchError::EmptyQuery| EngineError::SearchQueryEmpty)?;
    let results = ranked
        .into_iter()
        .take(limit)
        .filter_map(|r| hits.remove(&r.id))
        .collect();

    let archived = if archive {
        Some(search_archive(fs, layout, query, lang, limit)?)
    } else {
        None
    };
    Ok(SearchResult {
        query: query.to_string(),
        language: lang.code().to_string(),
        results,
        archived,
    })
}

/// The archived changes, each its proposal and design, ranked on their own
/// so that a long design never pushes a requirement out.
fn search_archive(
    fs: &dyn FileSystem,
    layout: &Layout,
    query: &str,
    lang: codev_core::keywords::Lang,
    limit: usize,
) -> Result<Vec<ArchivedHit>> {
    let dir = layout.archive_dir();
    let mut names = fs.list_dir(&dir).unwrap_or_default();
    names.sort();
    let mut documents = Vec::new();
    let mut excerpts = BTreeMap::new();
    for name in names {
        let mut text = String::new();
        let mut why = String::new();
        for file in ["proposal.md", "design.md"] {
            if let Ok(content) = fs.read_to_string(&dir.join(&name).join(file)) {
                if file == "proposal.md" {
                    why = content
                        .lines()
                        .skip_while(|l| !l.starts_with("## Why"))
                        .skip(1)
                        .take_while(|l| !l.starts_with("## "))
                        .collect::<Vec<_>>()
                        .join(" ");
                }
                text.push_str(&content);
                text.push('\n');
            }
        }
        if text.is_empty() {
            continue;
        }
        excerpts.insert(name.clone(), excerpt(&why));
        documents.push(Document { id: name, text });
    }
    let ranked = rank(&documents, query, lang).map_err(|_| EngineError::SearchQueryEmpty)?;
    Ok(ranked
        .into_iter()
        .take(limit)
        .map(|r| ArchivedHit {
            excerpt: excerpts.remove(&r.id).unwrap_or_default(),
            change: r.id,
        })
        .collect())
}

/// What `codev show` prints.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShowResult {
    /// One requirement, verbatim.
    Requirement {
        capability: String,
        name: String,
        text: String,
    },
    /// A capability's purpose and requirement names, in the spec's order.
    Capability {
        capability: String,
        purpose: Option<String>,
        requirements: Vec<String>,
    },
}

/// `<capability>` or `<capability>::<requirement>`. The capability must be
/// one of the main specs as `codev list --specs` names them: any other
/// text, a path leading outside `specs/` included, is unknown and no file
/// is read for it.
pub fn show(fs: &dyn FileSystem, layout: &Layout, target: &str) -> Result<ShowResult> {
    let (capability, requirement) = match target.split_once("::") {
        Some((capability, requirement)) => (capability.trim(), Some(requirement.trim())),
        None => (target.trim(), None),
    };
    if !specs::list(fs, layout).iter().any(|c| c == capability) {
        return Err(EngineError::UnknownCapability {
            capability: capability.to_string(),
        });
    }
    let path = layout.spec_file(capability);
    let source = fs
        .read_to_string(&path)
        .map_err(|e| EngineError::Unreadable {
            path: path.clone(),
            reason: e.to_string(),
        })?;
    let spec = parse_spec(&source).value;
    match requirement {
        None => Ok(ShowResult::Capability {
            capability: capability.to_string(),
            purpose: spec.purpose.as_ref().map(|p| p.text.trim().to_string()),
            requirements: spec.requirements.iter().map(|r| r.name.clone()).collect(),
        }),
        Some(name) => match find_requirement(&spec, name) {
            Some(found) => Ok(ShowResult::Requirement {
                capability: capability.to_string(),
                name: found.name.clone(),
                text: requirement_source(&source, found).to_string(),
            }),
            None => Err(EngineError::UnknownRequirement {
                capability: capability.to_string(),
                requirement: name.to_string(),
                known: spec.requirements.iter().map(|r| r.name.clone()).collect(),
            }),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config;
    use crate::ports::{FixedEnv, MemoryFileSystem};

    fn env() -> FixedEnv {
        let mut env = FixedEnv::at("/p");
        env.vars.insert("HOME".into(), "/home".into());
        env
    }

    fn requirement(name: &str, text: &str) -> String {
        format!(
            "### Requirement: {name}\n\n{text}\n\n#### Scenario: It works\n\n- **WHEN** asked\n- **THEN** it works\n\n"
        )
    }

    fn spec(requirements: &[(&str, &str)]) -> String {
        let body: String = requirements
            .iter()
            .map(|(n, t)| requirement(n, t))
            .collect();
        format!("# Spec\n\n## Purpose\n\nWhat it is for.\n\n## Requirements\n\n{body}")
    }

    fn adr(id: &str, title: &str, status: &str, supersedes: &[&str], body: &str) -> String {
        let sup = if supersedes.is_empty() {
            String::new()
        } else {
            format!("supersedes: [{}]\n", supersedes.join(", "))
        };
        format!(
            "---\nid: \"{id}\"\ntitle: \"{title}\"\nstatus: {status}\ndate: 2026-10-15\n{sup}---\n\n## Context\n\n{body}\n\n## Decision\n\nIt is decided.\n"
        )
    }

    fn project() -> MemoryFileSystem {
        MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file(
                "/p/_codev/specs/decisions/spec.md",
                spec(&[
                    (
                        "Creating a decision through the CLI",
                        "`codev decision new` SHALL create a decision file from the command line.",
                    ),
                    (
                        "Listing decisions",
                        "`codev decision list` SHALL list them.",
                    ),
                ]),
            )
            .with_file(
                "/p/_codev/specs/review-record/spec.md",
                spec(&[(
                    "Findings carry a severity",
                    "Each finding of a review SHALL carry a severity.",
                )]),
            )
    }

    fn run(
        fs: &MemoryFileSystem,
        query: &str,
        limit: usize,
        archive: bool,
    ) -> Result<SearchResult> {
        let layout = Layout::new("/p");
        let cfg = config::resolve(fs, &env(), &layout).unwrap();
        search(fs, &env(), &layout, &cfg, query, limit, archive)
    }

    fn requirement_ids(result: &SearchResult) -> Vec<String> {
        result
            .results
            .iter()
            .filter_map(|h| match h {
                Hit::Requirement {
                    capability,
                    requirement,
                    ..
                } => Some(format!("{capability}::{requirement}")),
                Hit::Decision { .. } => None,
            })
            .collect()
    }

    #[test]
    fn search_finds_the_requirement_a_query_describes() {
        let result = run(
            &project(),
            "create a new decision from the command line",
            10,
            false,
        )
        .unwrap();
        assert_eq!(
            requirement_ids(&result)[0],
            "decisions::Creating a decision through the CLI"
        );
        let Hit::Requirement { excerpt, .. } = &result.results[0] else {
            panic!("a requirement first");
        };
        assert!(
            excerpt.contains("SHALL create a decision file"),
            "{excerpt}"
        );
        assert_eq!(result.language, "en");
    }

    #[test]
    fn search_finds_an_inherited_decision_with_its_origin() {
        let fs = project()
            .with_file("/p/_codev/config.yaml", "inherits:\n  - path: /shared\n")
            .with_file(
                "/shared/_codev/decisions/0100-errors.md",
                adr(
                    "0100",
                    "Errors are typed",
                    "accepted",
                    &[],
                    "Errors carry a type.",
                ),
            );
        let result = run(&fs, "typed errors", 10, false).unwrap();
        let decision = result
            .results
            .iter()
            .find_map(|h| match h {
                Hit::Decision { decision, .. } => Some(decision),
                Hit::Requirement { .. } => None,
            })
            .expect("the inherited decision is found");
        assert_eq!(decision.id, "0100");
        assert_eq!(decision.title, "Errors are typed");
        assert_eq!(decision.origin, "path:/shared");
    }

    #[test]
    fn search_prints_at_most_the_limit() {
        let names: Vec<(String, String)> = (0..300)
            .map(|i| {
                let text = if i < 120 {
                    format!("Item {i} reads its config.")
                } else {
                    format!("Item {i} does something else.")
                };
                (format!("Requirement number {i:03}"), text)
            })
            .collect();
        let refs: Vec<(&str, &str)> = names
            .iter()
            .map(|(n, t)| (n.as_str(), t.as_str()))
            .collect();
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file("/p/_codev/specs/big/spec.md", spec(&refs));
        assert_eq!(
            run(&fs, "config", DEFAULT_LIMIT, false)
                .unwrap()
                .results
                .len(),
            10
        );
        assert_eq!(run(&fs, "config", 3, false).unwrap().results.len(), 3);
        assert!(matches!(
            run(&fs, "config", 0, false),
            Err(EngineError::SearchLimitInvalid { limit: 0, .. })
        ));
        assert!(matches!(
            run(&fs, "config", 51, false),
            Err(EngineError::SearchLimitInvalid { .. })
        ));
    }

    fn with_archive(fs: MemoryFileSystem, design: &str) -> MemoryFileSystem {
        fs.with_file(
            "/p/_codev/changes/archive/2026-09-01-add-seals/proposal.md",
            "# Proposal: Seals\n\n## Why\n\nDecisions need a seal so that edits are seen.\n\n## What Changes\n\n- a decision seal\n",
        )
        .with_file(
            "/p/_codev/changes/archive/2026-09-01-add-seals/design.md",
            design,
        )
        .with_file(
            "/p/_codev/changes/archive/2026-09-02-other/proposal.md",
            "# Proposal: Other\n\n## Why\n\nSomething unrelated.\n",
        )
    }

    #[test]
    fn the_archive_is_ranked_apart() {
        let fs = with_archive(project(), "# Design\n\nThe seal is a hash.\n");
        let plain = run(&fs, "decision seal", 10, false).unwrap();
        let with = run(&fs, "decision seal", 10, true).unwrap();
        assert!(plain.archived.is_none());
        let archived = with.archived.as_ref().unwrap();
        assert_eq!(archived[0].change, "2026-09-01-add-seals");
        assert!(archived[0].excerpt.contains("Decisions need a seal"));
        assert_eq!(plain.results, with.results);
    }

    #[test]
    fn a_long_archived_design_leaves_the_requirements_unchanged() {
        let long = "review ".repeat(5000);
        let fs = with_archive(project(), &format!("# Design\n\n{long}\n"));
        let plain = run(&fs, "review", 10, false).unwrap();
        let with = run(&fs, "review", 10, true).unwrap();
        assert_eq!(
            requirement_ids(&plain),
            ["review-record::Findings carry a severity"]
        );
        assert_eq!(plain.results, with.results);
    }

    #[test]
    fn without_archive_the_archive_is_not_read() {
        let fs = with_archive(project(), "# Design\n");
        run(&fs, "decision seal", 10, false).unwrap();
        assert!(
            !fs.reads()
                .iter()
                .any(|p| p.starts_with("/p/_codev/changes/archive")),
            "{:?}",
            fs.reads()
        );
    }

    #[test]
    fn search_in_a_french_project_matches_accentless_plurals() {
        let source = "# Décisions\n\n## Objectif\n\nLes décisions.\n\n## Exigences\n\n\
### Exigence : Une décision acceptée est scellée\n\nLe système DOIT sceller une décision acceptée.\n\n\
#### Scénario : Sceller\n\n- **QUAND** on accepte\n- **ALORS** il scelle\n\n\
### Exigence : Lister\n\nLe système DOIT lister les fichiers.\n\n\
#### Scénario : Lister\n\n- **QUAND** on demande\n- **ALORS** il liste\n";
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file("/p/_codev/specs/decisions/spec.md", source);
        let result = run(&fs, "decisions acceptees", 10, false).unwrap();
        assert_eq!(result.language, "fr");
        assert_eq!(
            requirement_ids(&result),
            ["decisions::Une décision acceptée est scellée"]
        );
    }

    #[test]
    fn a_superseded_decision_is_not_presented_as_in_effect() {
        let fs = project()
            .with_file(
                "/p/_codev/decisions/0003-root.md",
                adr(
                    "0003",
                    "The planning folder is hidden",
                    "superseded",
                    &[],
                    "The planning folder.",
                ),
            )
            .with_file(
                "/p/_codev/decisions/0009-visible.md",
                adr(
                    "0009",
                    "The planning folder is visible",
                    "accepted",
                    &["\"0003\""],
                    "The planning folder.",
                ),
            );
        let result = run(&fs, "planning folder", 10, false).unwrap();
        let decisions: BTreeMap<String, DecisionHit> = result
            .results
            .iter()
            .filter_map(|h| match h {
                Hit::Decision { decision, .. } => Some((decision.id.clone(), decision.clone())),
                Hit::Requirement { .. } => None,
            })
            .collect();
        assert_eq!(decisions["0003"].status, "superseded");
        assert!(!decisions["0003"].in_effect);
        assert_eq!(
            decisions["0003"].superseded_by.as_deref(),
            Some("project/0009")
        );
        assert!(decisions["0009"].in_effect);
    }

    #[test]
    fn show_prints_one_requirement_and_nothing_else() {
        let fs = project();
        let shown = show(
            &fs,
            &Layout::new("/p"),
            "decisions::creating a decision  through the CLI",
        )
        .unwrap();
        let ShowResult::Requirement { name, text, .. } = shown else {
            panic!("one requirement");
        };
        assert_eq!(name, "Creating a decision through the CLI");
        assert!(text.starts_with("### Requirement: Creating a decision through the CLI"));
        assert!(text.ends_with("- **THEN** it works"));
        assert!(!text.contains("Listing decisions"));
    }

    #[test]
    fn show_lists_a_capabilitys_requirements() {
        let shown = show(&project(), &Layout::new("/p"), "decisions").unwrap();
        assert_eq!(
            shown,
            ShowResult::Capability {
                capability: "decisions".into(),
                purpose: Some("What it is for.".into()),
                requirements: vec![
                    "Creating a decision through the CLI".into(),
                    "Listing decisions".into()
                ],
            }
        );
    }

    #[test]
    fn show_names_the_requirements_of_a_capability_when_one_is_unknown() {
        let err = show(
            &project(),
            &Layout::new("/p"),
            "decisions::Deleting a decision",
        )
        .unwrap_err();
        assert_eq!(err.code(), "unknown_requirement");
        assert!(err.to_string().contains("Listing decisions"), "{err}");
    }

    #[test]
    fn show_refuses_an_unknown_capability() {
        let err = show(&project(), &Layout::new("/p"), "billing").unwrap_err();
        assert_eq!(err.code(), "unknown_capability");
        assert!(err.to_string().contains("codev list --specs"));
    }

    #[test]
    fn show_refuses_a_capability_outside_the_specs() {
        let fs = project().with_file("/p/spec.md", spec(&[("Secret", "Outside.")]));
        for target in ["../../", "../..", "/p", "/"] {
            let err = show(&fs, &Layout::new("/p"), target).unwrap_err();
            assert_eq!(err.code(), "unknown_capability", "{target}");
        }
        assert!(
            !fs.reads()
                .iter()
                .any(|p| p == std::path::Path::new("/p/spec.md"))
        );
    }

    #[test]
    fn search_and_show_write_no_file() {
        let fs = with_archive(project(), "# Design\n");
        let before: Vec<(std::path::PathBuf, Option<String>)> = fs
            .paths()
            .into_iter()
            .map(|p| {
                let c = fs.read(&p);
                (p, c)
            })
            .collect();
        run(&fs, "decision seal", 10, true).unwrap();
        show(&fs, &Layout::new("/p"), "decisions").unwrap();
        show(&fs, &Layout::new("/p"), "decisions::Listing decisions").unwrap();
        let after: Vec<(std::path::PathBuf, Option<String>)> = fs
            .paths()
            .into_iter()
            .map(|p| {
                let c = fs.read(&p);
                (p, c)
            })
            .collect();
        assert_eq!(before, after);
    }
}
