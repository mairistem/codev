//! Repository integration: the CI workflow and the pull request template.
//!
//! Two steps of `codev init` (and their refresh by `codev update`) that
//! depend on the forge, not on the agent — `CLAUDE.md` is planned by
//! `codev-agents`, which reuses [`plan_managed_markdown`] and
//! [`IntegrationStep`].
//!
//! [`RepoFacts::probe`] reads; everything else is pure planning over those
//! facts, and the shell executes the resulting [`Plan`]
//! (`_codev/decisions/0001-functional-core-imperative-shell.md`). Nothing
//! here writes a byte outside codev's managed section or managed file.

use std::path::{Path, PathBuf};

use codev_core::detect::forge::{self, Forge};
use codev_core::managed::{self, Splice, SpliceAction, SpliceMode};
use codev_core::{Plan, WriteMode};

use crate::error::Warning;
use crate::ports::FileSystem;

/// The official installer, fetched by the generated CI.
pub const INSTALLER_URL: &str = "https://raw.githubusercontent.com/mairistem/codev/main/install.sh";

/// The workflow file codev owns.
pub const CODEV_WORKFLOW: &str = ".github/workflows/codev.yml";

/// Where a new pull request template is created.
pub const DEFAULT_PR_TEMPLATE: &str = ".github/PULL_REQUEST_TEMPLATE.md";

/// The folders GitHub looks into for a single pull request template, in
/// the order codev picks an existing one.
const PR_TEMPLATE_DIRS: &[&str] = &[".github", "", "docs"];
const PR_TEMPLATE_NAME: &str = "pull_request_template.md";

/// Which integration file a step is about.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Step {
    ClaudeMd,
    Ci,
    PrTemplate,
}

impl Step {
    /// Stable identifier, exposed in the JSON contract.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ClaudeMd => "claude_md",
            Self::Ci => "ci",
            Self::PrTemplate => "pr_template",
        }
    }
}

/// What a step does to its file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepAction {
    Created,
    Appended,
    Refreshed,
    Unchanged,
    Skipped,
}

impl StepAction {
    /// Stable identifier, exposed in the JSON contract.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Created => "created",
            Self::Appended => "appended",
            Self::Refreshed => "refreshed",
            Self::Unchanged => "unchanged",
            Self::Skipped => "skipped",
        }
    }
}

impl From<SpliceAction> for StepAction {
    fn from(action: SpliceAction) -> Self {
        match action {
            SpliceAction::Created => Self::Created,
            SpliceAction::Appended => Self::Appended,
            SpliceAction::Refreshed => Self::Refreshed,
        }
    }
}

/// One integration step, as reported to the user.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegrationStep {
    pub step: Step,
    pub action: StepAction,
    /// Relative to the project root; `None` when no file is concerned.
    pub path: Option<PathBuf>,
    pub reason: Option<String>,
    /// A ready-to-paste snippet for the user (the GitLab job). Human
    /// output only.
    pub snippet: Option<String>,
}

impl IntegrationStep {
    pub fn skipped(step: Step, path: Option<PathBuf>, reason: impl Into<String>) -> Self {
        Self {
            step,
            action: StepAction::Skipped,
            path,
            reason: Some(reason.into()),
            snippet: None,
        }
    }
}

/// The plan and report of one or several steps.
#[derive(Debug, Default)]
pub struct IntegrationPlan {
    pub plan: Plan,
    pub steps: Vec<IntegrationStep>,
    pub warnings: Vec<Warning>,
}

impl IntegrationPlan {
    pub fn merge(&mut self, other: IntegrationPlan) {
        self.plan.merge(other.plan);
        self.steps.extend(other.steps);
        self.warnings.extend(other.warnings);
    }
}

/// What the integration needs to know about the repository.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RepoFacts {
    pub root: PathBuf,
    /// A `.github/` folder, or a remote on GitHub.
    pub is_github: bool,
    /// A `.gitlab-ci.yml` file, or a remote on GitLab.
    pub is_gitlab: bool,
    /// `(relative path, content)` of each `.github/workflows/*.yml|yaml`.
    pub workflows: Vec<(String, String)>,
    /// Content of `.gitlab-ci.yml`, when it exists.
    pub gitlab_ci: Option<String>,
    /// `(relative path, content)` of the first existing pull request
    /// template.
    pub pr_template: Option<(PathBuf, String)>,
}

impl RepoFacts {
    /// Reads the repository. Read-only, no process spawned: the remote
    /// comes from the text of `.git/config`.
    pub fn probe(fs: &dyn FileSystem, root: &Path) -> Self {
        let remote_forge = fs
            .read_to_string(&root.join(".git").join("config"))
            .ok()
            .and_then(|text| forge::from_git_config(&text));
        let gitlab_ci = fs.read_to_string(&root.join(".gitlab-ci.yml")).ok();

        let workflows_dir = root.join(".github").join("workflows");
        let mut workflows = Vec::new();
        for name in fs.list_dir(&workflows_dir).unwrap_or_default() {
            if !(name.ends_with(".yml") || name.ends_with(".yaml")) {
                continue;
            }
            if let Ok(content) = fs.read_to_string(&workflows_dir.join(&name)) {
                workflows.push((format!(".github/workflows/{name}"), content));
            }
        }

        let pr_template = PR_TEMPLATE_DIRS.iter().find_map(|dir| {
            let folder = if dir.is_empty() {
                root.to_path_buf()
            } else {
                root.join(dir)
            };
            let name = fs
                .list_dir(&folder)
                .unwrap_or_default()
                .into_iter()
                .find(|n| n.eq_ignore_ascii_case(PR_TEMPLATE_NAME))?;
            let relative = Path::new(dir).join(&name);
            let content = fs.read_to_string(&root.join(&relative)).ok()?;
            Some((relative, content))
        });

        Self {
            root: root.to_path_buf(),
            is_github: fs.exists(&root.join(".github")) || remote_forge == Some(Forge::GitHub),
            is_gitlab: gitlab_ci.is_some() || remote_forge == Some(Forge::GitLab),
            workflows,
            gitlab_ci,
            pr_template,
        }
    }

    /// The CI files, relative, that run `codev validate`.
    pub fn ci_files_running_validate(&self) -> Vec<String> {
        let mut files: Vec<String> = self
            .workflows
            .iter()
            .filter(|(_, content)| runs_codev_validate(content))
            .map(|(path, _)| path.clone())
            .collect();
        if self.gitlab_ci.as_deref().is_some_and(runs_codev_validate) {
            files.push(".gitlab-ci.yml".to_string());
        }
        files
    }

    fn codev_workflow(&self) -> Option<&str> {
        self.workflows
            .iter()
            .find(|(path, _)| path == CODEV_WORKFLOW)
            .map(|(_, content)| content.as_str())
    }
}

/// Whether a CI file has a line running `codev validate`: the word `codev`,
/// then `validate` later on the same line, outside a comment. So
/// `cargo run --bin codev -- validate` counts.
pub fn runs_codev_validate(text: &str) -> bool {
    text.lines().any(|line| {
        let line = line.trim_start();
        if line.starts_with('#') {
            return false;
        }
        line.find("codev")
            .is_some_and(|at| line[at + "codev".len()..].contains("validate"))
    })
}

/// The generated GitHub Actions workflow.
pub fn render_github_workflow(version: &str) -> String {
    format!(
        "{marker} — generated by `codev init`, refreshed by `codev update`.
# Delete this first line to take the file over: codev will then leave it alone.
name: codev

on:
  push:
  pull_request:

jobs:
  validate:
    name: codev validate
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v7
      - name: Install codev {version}
        run: |
          curl -sSL {INSTALLER_URL} | CODEV_VERSION={version} sh
          echo \"$HOME/.local/bin\" >> \"$GITHUB_PATH\"
      - name: Validate specs and changes
        run: codev validate --strict
",
        marker = managed::MANAGED_YAML_MARKER,
    )
}

/// The GitLab CI job printed for the user to paste.
pub fn render_gitlab_snippet(version: &str) -> String {
    format!(
        "codev:
  image: debian:stable-slim
  before_script:
    - apt-get update -qq && apt-get install -y -qq curl ca-certificates
    - curl -sSL {INSTALLER_URL} | CODEV_VERSION={version} sh
  script:
    - ~/.local/bin/codev validate --strict
"
    )
}

/// The body of the pull request template's managed section.
pub fn pr_template_body() -> String {
    "## codev\n\n\
     - codev change: <!-- folder under `_codev/changes/`, or \"trivial fix, no change\" -->\n\
     - [ ] `codev validate --strict` passes."
        .to_string()
}

/// Plans the CI step.
///
/// `Init` creates or refreshes `.github/workflows/codev.yml` on GitHub and
/// reports why it does not otherwise; `Refresh` only rewrites a managed
/// `codev.yml` and reports nothing when there is none.
pub fn plan_ci(facts: &RepoFacts, version: &str, mode: SpliceMode) -> IntegrationPlan {
    let mut out = IntegrationPlan::default();
    let path = PathBuf::from(CODEV_WORKFLOW);
    let desired = render_github_workflow(version);

    if let Some(existing) = facts.codev_workflow() {
        if managed::is_managed_file(existing) {
            let action = if existing == desired {
                StepAction::Unchanged
            } else {
                out.plan
                    .write(facts.root.join(&path), desired, WriteMode::Overwrite);
                StepAction::Refreshed
            };
            out.steps.push(IntegrationStep {
                step: Step::Ci,
                action,
                path: Some(path),
                reason: None,
                snippet: None,
            });
        } else if mode == SpliceMode::Init {
            out.steps.push(IntegrationStep::skipped(
                Step::Ci,
                Some(path),
                format!(
                    "{CODEV_WORKFLOW} exists and is not managed by codev (no `{}` first line)",
                    managed::MANAGED_YAML_MARKER
                ),
            ));
        }
        return out;
    }
    if mode == SpliceMode::Refresh {
        return out;
    }

    if !facts.is_github {
        let step = if facts.is_gitlab {
            IntegrationStep {
                snippet: Some(render_gitlab_snippet(version)),
                ..IntegrationStep::skipped(
                    Step::Ci,
                    Some(PathBuf::from(".gitlab-ci.yml")),
                    "codev never edits .gitlab-ci.yml — add the job below to run codev validate",
                )
            }
        } else {
            IntegrationStep::skipped(
                Step::Ci,
                None,
                "no GitHub or GitLab repository detected — add `codev validate --strict` to your CI",
            )
        };
        out.steps.push(step);
        return out;
    }

    if let Some(file) = facts.ci_files_running_validate().first() {
        out.steps.push(IntegrationStep::skipped(
            Step::Ci,
            Some(path),
            format!("{file} already runs codev validate"),
        ));
        return out;
    }

    out.plan.dir(facts.root.join(".github").join("workflows"));
    out.plan
        .write(facts.root.join(&path), desired, WriteMode::CreateOnly);
    out.steps.push(IntegrationStep {
        step: Step::Ci,
        action: StepAction::Created,
        path: Some(path),
        reason: None,
        snippet: None,
    });
    out
}

/// Plans the pull request template step (GitHub only).
pub fn plan_pr_template(facts: &RepoFacts, mode: SpliceMode) -> IntegrationPlan {
    if !facts.is_github {
        let mut out = IntegrationPlan::default();
        if mode == SpliceMode::Init {
            out.steps.push(IntegrationStep::skipped(
                Step::PrTemplate,
                None,
                "not a GitHub repository",
            ));
        }
        return out;
    }
    let (path, existing) = match &facts.pr_template {
        Some((path, content)) => (path.clone(), Some(content.as_str())),
        None => (PathBuf::from(DEFAULT_PR_TEMPLATE), None),
    };
    plan_managed_markdown(
        &facts.root,
        &path,
        existing,
        &pr_template_body(),
        mode,
        Step::PrTemplate,
    )
}

/// Plans a managed section in a markdown file the user owns.
///
/// Shared by `CLAUDE.md` and the pull request template: a malformed file is
/// left alone with a `managed_section_malformed` warning, and `Refresh`
/// reports nothing for a file without a section.
pub fn plan_managed_markdown(
    root: &Path,
    relative: &Path,
    existing: Option<&str>,
    body: &str,
    mode: SpliceMode,
    step: Step,
) -> IntegrationPlan {
    let mut out = IntegrationPlan::default();
    let report = |action: StepAction, reason: Option<String>| IntegrationStep {
        step,
        action,
        path: Some(relative.to_path_buf()),
        reason,
        snippet: None,
    };
    match managed::splice(existing, body, mode) {
        Splice::Write { contents, action } => {
            let target = root.join(relative);
            if let Some(parent) = target.parent() {
                out.plan.dir(parent);
            }
            // CreateOnly for a new file: if it appeared since the probe,
            // the user's file wins.
            let write_mode = if existing.is_some() {
                WriteMode::Overwrite
            } else {
                WriteMode::CreateOnly
            };
            out.plan.write(target, contents, write_mode);
            out.steps.push(report(action.into(), None));
        }
        Splice::Unchanged => out.steps.push(report(StepAction::Unchanged, None)),
        Splice::Absent => {}
        Splice::Malformed(reason) => {
            out.warnings.push(Warning::new(
                "managed_section_malformed",
                format!(
                    "{} was left untouched: {reason} (`{}` … `{}`)",
                    relative.display(),
                    managed::BEGIN_PREFIX,
                    managed::END_LINE
                ),
            ));
            out.steps.push(report(
                StepAction::Skipped,
                Some(format!("malformed codev markers: {reason}")),
            ));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::apply;
    use crate::ports::MemoryFileSystem;

    const VERSION: &str = "0.4.0";

    fn facts(fs: &MemoryFileSystem) -> RepoFacts {
        RepoFacts::probe(fs, Path::new("/p"))
    }

    #[test]
    fn github_project_gets_a_managed_workflow_pinned_to_the_version() {
        let fs = MemoryFileSystem::new().with_file("/p/.github/dependabot.yml", "version: 2\n");
        let planned = plan_ci(&facts(&fs), VERSION, SpliceMode::Init);
        assert_eq!(planned.steps[0].action, StepAction::Created);
        apply::execute(&planned.plan, &fs).unwrap();
        let workflow = fs.read("/p/.github/workflows/codev.yml").unwrap();
        assert!(workflow.starts_with("# codev:managed"));
        assert!(workflow.contains("CODEV_VERSION=0.4.0 sh"));
        assert!(workflow.contains("run: codev validate --strict"));
        let parsed: serde_norway::Value = serde_norway::from_str(&workflow).unwrap();
        assert_eq!(parsed["name"].as_str(), Some("codev"));
    }

    #[test]
    fn a_github_remote_is_enough() {
        let fs = MemoryFileSystem::new().with_file(
            "/p/.git/config",
            "[remote \"origin\"]\n\turl = git@github.com:acme/app.git\n",
        );
        assert!(facts(&fs).is_github);
    }

    #[test]
    fn gitlab_project_gets_a_snippet_and_no_write() {
        let fs = MemoryFileSystem::new().with_file("/p/.gitlab-ci.yml", "stages: [test]\n");
        let planned = plan_ci(&facts(&fs), VERSION, SpliceMode::Init);
        assert!(planned.plan.is_empty());
        let step = &planned.steps[0];
        assert_eq!(step.action, StepAction::Skipped);
        let snippet = step.snippet.as_deref().unwrap();
        assert!(snippet.contains("CODEV_VERSION=0.4.0"));
        assert!(snippet.contains("codev validate --strict"));
        let parsed: serde_norway::Value = serde_norway::from_str(snippet).unwrap();
        assert!(parsed["codev"]["script"].is_sequence());
    }

    #[test]
    fn no_forge_skips_ci_and_pr_template_with_a_reason() {
        let fs = MemoryFileSystem::new();
        let ci = plan_ci(&facts(&fs), VERSION, SpliceMode::Init);
        let pr = plan_pr_template(&facts(&fs), SpliceMode::Init);
        assert!(ci.plan.is_empty() && pr.plan.is_empty());
        assert_eq!(ci.steps[0].action, StepAction::Skipped);
        assert!(ci.steps[0].reason.as_deref().unwrap().contains("GitHub"));
        assert_eq!(
            pr.steps[0].reason.as_deref(),
            Some("not a GitHub repository")
        );
    }

    #[test]
    fn unmanaged_codev_yml_is_left_alone() {
        let fs =
            MemoryFileSystem::new().with_file("/p/.github/workflows/codev.yml", "name: mine\n");
        let planned = plan_ci(&facts(&fs), VERSION, SpliceMode::Init);
        assert!(planned.plan.is_empty());
        assert!(
            planned.steps[0]
                .reason
                .as_deref()
                .unwrap()
                .contains("not managed by codev")
        );
    }

    #[test]
    fn existing_validate_workflow_skips_ci() {
        let fs = MemoryFileSystem::new().with_file(
            "/p/.github/workflows/ci.yml",
            "jobs:\n  x:\n    steps:\n      - run: cargo run --locked --quiet --bin codev -- validate --strict\n",
        );
        let f = facts(&fs);
        assert_eq!(f.ci_files_running_validate(), [".github/workflows/ci.yml"]);
        let planned = plan_ci(&f, VERSION, SpliceMode::Init);
        assert!(planned.plan.is_empty());
        assert_eq!(
            planned.steps[0].reason.as_deref(),
            Some(".github/workflows/ci.yml already runs codev validate")
        );
    }

    #[test]
    fn commented_out_validate_does_not_count() {
        assert!(!runs_codev_validate("# - run: codev validate\n"));
        assert!(runs_codev_validate("  - run: codev validate --strict\n"));
        assert!(!runs_codev_validate("  - run: validate codev\n"));
    }

    #[test]
    fn pr_template_section_goes_into_the_first_existing_template() {
        let user = "## Summary\n\nDescribe the change.\n";
        let fs = MemoryFileSystem::new()
            .with_file("/p/.github/dependabot.yml", "version: 2\n")
            .with_file("/p/docs/pull_request_template.md", user);
        let planned = plan_pr_template(&facts(&fs), SpliceMode::Init);
        assert_eq!(planned.steps[0].action, StepAction::Appended);
        assert_eq!(
            planned.steps[0].path.as_deref(),
            Some(Path::new("docs/pull_request_template.md"))
        );
        apply::execute(&planned.plan, &fs).unwrap();
        let written = fs.read("/p/docs/pull_request_template.md").unwrap();
        assert!(written.starts_with(user));
        assert!(written.contains("- codev change:"));
        assert!(written.contains("- [ ] `codev validate --strict` passes."));
        assert!(!fs.exists(Path::new("/p/.github/PULL_REQUEST_TEMPLATE.md")));
    }

    #[test]
    fn pr_template_is_created_when_none_exists() {
        let fs = MemoryFileSystem::new().with_file("/p/.github/dependabot.yml", "version: 2\n");
        let planned = plan_pr_template(&facts(&fs), SpliceMode::Init);
        assert_eq!(planned.steps[0].action, StepAction::Created);
        apply::execute(&planned.plan, &fs).unwrap();
        let written = fs.read("/p/.github/PULL_REQUEST_TEMPLATE.md").unwrap();
        assert!(managed::has_section(&written));
    }

    #[test]
    fn refresh_bumps_the_managed_workflow_and_creates_nothing() {
        let fs = MemoryFileSystem::new().with_file(
            "/p/.github/workflows/codev.yml",
            render_github_workflow("0.3.0"),
        );
        let planned = plan_ci(&facts(&fs), VERSION, SpliceMode::Refresh);
        assert_eq!(planned.steps[0].action, StepAction::Refreshed);
        apply::execute(&planned.plan, &fs).unwrap();
        assert!(
            fs.read("/p/.github/workflows/codev.yml")
                .unwrap()
                .contains("CODEV_VERSION=0.4.0")
        );

        // A GitHub project without codev.yml nor template: refresh adds nothing.
        let fs = MemoryFileSystem::new().with_file("/p/.github/dependabot.yml", "version: 2\n");
        let ci = plan_ci(&facts(&fs), VERSION, SpliceMode::Refresh);
        let pr = plan_pr_template(&facts(&fs), SpliceMode::Refresh);
        assert!(ci.plan.is_empty() && ci.steps.is_empty());
        assert!(pr.plan.is_empty() && pr.steps.is_empty());
    }

    #[test]
    fn malformed_markers_warn_and_write_nothing() {
        let fs = MemoryFileSystem::new().with_file(
            "/p/.github/PULL_REQUEST_TEMPLATE.md",
            "<!-- codev:begin\nx\n",
        );
        let planned = plan_pr_template(&facts(&fs), SpliceMode::Init);
        assert!(planned.plan.is_empty());
        assert_eq!(planned.warnings[0].code, "managed_section_malformed");
        assert_eq!(planned.steps[0].action, StepAction::Skipped);
    }
}
