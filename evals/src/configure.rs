//! Scoring a configure run, without a model: whether `/codev-configure`
//! wrote a `context:` and `rules:` of the expected shape, grounded in the
//! project, and kept every other field and comment of
//! `_codev/config.yaml` — or wrote nothing on a no, or refused without a
//! configuration. The keys and the rules are read as YAML; the comments
//! and the lines of `context` as text, where a folded block keeps its
//! lines. Pure: it reads the observation only.

use serde_norway::{Mapping, Value};

use crate::case::{ConfigureCase, ConfigureOutcome};
use crate::checked::{CheckScore, Checks, holds};
use crate::observed::{ConfigureObserved, Observed};

/// The comment the skill writes directly above `context:`.
pub const MARKER: &str = "# written by /codev-configure";

/// The fields the skill writes; every other top-level key is preserved.
const WRITTEN_KEYS: [&str; 2] = ["context", "rules"];

fn mapping(text: Option<&str>) -> Option<Mapping> {
    match serde_norway::from_str::<Value>(text?).ok()? {
        Value::Mapping(map) => Some(map),
        _ => None,
    }
}

fn is_comment(line: &str) -> bool {
    line.trim_start().starts_with('#')
}

/// The index of the top-level `context:` line.
fn context_line(lines: &[&str]) -> Option<usize> {
    lines
        .iter()
        .position(|l| l.starts_with("context:") || l.starts_with("\"context\":"))
}

/// The comment lines of `before` the skill must keep: all of them but the
/// one directly above `context:`, which the marker replaces.
fn kept_comments(before: &str) -> Vec<String> {
    let lines: Vec<&str> = before.lines().collect();
    let replaced = context_line(&lines)
        .and_then(|at| at.checked_sub(1))
        .filter(|above| is_comment(lines[*above]));
    lines
        .iter()
        .enumerate()
        .filter(|(i, line)| is_comment(line) && Some(*i) != replaced)
        .map(|(_, line)| line.trim().to_string())
        .collect()
}

/// The non-empty lines the value of `context` spans in the text, whatever
/// its YAML style: its own line unless it holds only a block indicator,
/// then every indented line up to the next top-level key.
fn context_lines(after: &str) -> usize {
    let lines: Vec<&str> = after.lines().collect();
    let Some(at) = context_line(&lines) else {
        return 0;
    };
    let inline = lines[at].split_once(':').map_or("", |(_, v)| v).trim();
    let indicator = inline.is_empty()
        || inline
            .chars()
            .all(|c| matches!(c, '|' | '>' | '-' | '+') || c.is_ascii_digit());
    let own = usize::from(!indicator);
    own + lines[at + 1..]
        .iter()
        .take_while(|l| l.trim().is_empty() || l.starts_with(char::is_whitespace))
        .filter(|l| !l.trim().is_empty())
        .count()
}

/// The texts of a part of the proposal: `context`, or the rules of
/// `specs`, `design` or `tasks`, one per line.
fn proposal_text(config: &Mapping, name: &str) -> String {
    if name == "context" {
        return config
            .get("context")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string();
    }
    rules(config, name).join("\n")
}

fn rules(config: &Mapping, name: &str) -> Vec<String> {
    config
        .get("rules")
        .and_then(|r| r.get(name))
        .and_then(Value::as_sequence)
        .into_iter()
        .flatten()
        .filter_map(|r| r.as_str().map(str::to_string))
        .collect()
}

/// Whether `rules` holds `specs` and `design` with one or two rules each,
/// and `tasks` with at least one.
fn rules_shaped(config: &Mapping) -> bool {
    let count = |name: &str| rules(config, name).len();
    (1..=2).contains(&count("specs")) && (1..=2).contains(&count("design")) && count("tasks") >= 1
}

/// Whether every top-level key of `old` but `context` and `rules` holds
/// the same value in `new`.
fn keys_preserved(old: Option<&Mapping>, new: Option<&Mapping>) -> bool {
    let (Some(old), Some(new)) = (old, new) else {
        return false;
    };
    old.iter()
        .filter(|(k, _)| !WRITTEN_KEYS.iter().any(|w| k.as_str() == Some(*w)))
        .all(|(k, v)| new.get(k) == Some(v))
}

/// Whether `after` still holds every comment of `before` the skill must
/// keep.
fn comments_kept(before: Option<&str>, after: Option<&str>) -> bool {
    let (Some(before), Some(after)) = (before, after) else {
        return false;
    };
    let now: Vec<&str> = after.lines().map(str::trim).collect();
    kept_comments(before)
        .iter()
        .all(|c| now.contains(&c.as_str()))
}

fn score_written(expect: &ConfigureCase, seen: Option<&ConfigureObserved>, checks: &mut Checks) {
    let before = seen.and_then(|s| s.before.as_deref());
    let after = seen.and_then(|s| s.after.as_deref());
    let (old, new) = (mapping(before), mapping(after));
    checks.check(
        "keys other than `context` and `rules` preserved",
        keys_preserved(old.as_ref(), new.as_ref()),
    );
    checks.check("comments kept", comments_kept(before, after));
    let marked = after.is_some_and(|after| {
        let lines: Vec<&str> = after.lines().collect();
        context_line(&lines)
            .and_then(|at| at.checked_sub(1))
            .is_some_and(|above| lines[above].trim() == MARKER)
    });
    checks.check(format!("`{MARKER}` above `context:`"), marked);
    let spans = after.map_or(0, context_lines);
    checks.check(
        "`context` spans two to five lines",
        (2..=5).contains(&spans),
    );
    checks.check(
        "`rules` holds one or two `specs` and `design` rules, and `tasks` rules",
        new.as_ref().is_some_and(rules_shaped),
    );
    for (name, texts) in &expect.written {
        let held = new
            .as_ref()
            .map(|n| proposal_text(n, name))
            .unwrap_or_default();
        for text in texts {
            checks.check(
                format!("`{name}` holds \"{text}\""),
                new.is_some() && holds(&held, text),
            );
        }
    }
    for (name, texts) in &expect.unwritten {
        let held = new
            .as_ref()
            .map(|n| proposal_text(n, name))
            .unwrap_or_default();
        for text in texts {
            checks.check(
                format!("`{name}` does not hold \"{text}\""),
                new.is_some() && !holds(&held, text),
            );
        }
    }
    checks.check(
        "`codev status --json` reads the configuration",
        seen.is_some_and(|s| s.status_error.is_none() && s.after.is_some()),
    );
}

/// Scores one configure run. A run that stopped is scored as one that
/// left nothing to read: every check that needs its outcome fails.
pub fn score_configure(expect: &ConfigureCase, observed: &Observed) -> CheckScore {
    let seen = observed
        .configure
        .as_ref()
        .filter(|_| observed.error.is_none());
    let mut checks = Checks::default();
    match expect.outcome {
        ConfigureOutcome::Written => score_written(expect, seen, &mut checks),
        ConfigureOutcome::Unchanged => checks.check(
            "configuration byte-identical to the fixture's",
            seen.is_some_and(|s| s.after.is_some() && s.after == s.before),
        ),
        ConfigureOutcome::Refused => checks.check(
            "no `_codev/` folder created",
            seen.is_some_and(|s| !s.codev_root && s.after.is_none()),
        ),
    }
    checks.check(
        "no file modified outside the step",
        seen.is_some() && observed.strayed.is_empty(),
    );
    let messages = observed.messages.join("\n");
    for text in &expect.contains {
        checks.check(
            format!("says \"{text}\""),
            seen.is_some() && holds(&messages, text),
        );
    }
    checks.score()
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::observed::FORMAT;
    use std::collections::BTreeMap;

    const BEFORE: &str = "\
# codev configuration.
schema: spec-driven
language: fr
codev_version: \">=0.6\"

# The Jira MCP.
mcp:
  jira_tool: mcp__jira__getIssue

# detected from pyproject.toml
context: |
  Python project.

# Paths only this repository knows to be sensitive
triggers:
  - domain: security
    paths: [app/auth.py]
";

    const WRITTEN: &str = "\
# codev configuration.
schema: spec-driven
language: fr
codev_version: \">=0.6\"

# The Jira MCP.
mcp:
  jira_tool: mcp__jira__getIssue

# written by /codev-configure
context: >
  Service FastAPI en Python 3.12.
  Erreurs levées en exceptions typées.
  Commentaires en français.

rules:
  specs:
    - Une spec décrit un comportement observable.
    - Chaque scénario nomme l'endpoint concerné.
  design:
    - Citer la décision qui contraint le choix.
  tasks:
    - Tests pytest sous tests/, miroir du paquet app/.
    - Lancer pytest et ruff check avant de cocher.
    - Chaque tâche nomme le test qui la vérifie.

# Paths only this repository knows to be sensitive
triggers:
  - domain: security
    paths: [app/auth.py]
";

    fn expect(outcome: ConfigureOutcome) -> ConfigureCase {
        ConfigureCase {
            answer: outcome != ConfigureOutcome::Unchanged,
            outcome,
            written: BTreeMap::new(),
            unwritten: BTreeMap::new(),
            contains: vec![],
            config: Some(BEFORE.into()),
        }
    }

    fn run(before: Option<&str>, after: Option<&str>, messages: &[&str]) -> Observed {
        Observed {
            format: FORMAT,
            case: "configure-x".into(),
            error: None,
            cost_usd: Some(0.4),
            duration_ms: Some(1000),
            consumption: None,
            strayed: vec![],
            review: None,
            files: BTreeMap::new(),
            validate: BTreeMap::new(),
            validate_before: None,
            messages: messages.iter().map(|m| m.to_string()).collect(),
            implement: None,
            merge: None,
            configure: Some(ConfigureObserved {
                before: before.map(str::to_string),
                after: after.map(str::to_string),
                codev_root: after.is_some(),
                status_error: None,
            }),
        }
    }

    fn failed(score: &CheckScore) -> Vec<&str> {
        score
            .checks
            .iter()
            .filter(|c| !c.passed)
            .map(|c| c.check.as_str())
            .collect()
    }

    #[test]
    fn a_well_written_configuration_passes() {
        let score = score_configure(
            &expect(ConfigureOutcome::Written),
            &run(Some(BEFORE), Some(WRITTEN), &[]),
        );
        assert_eq!(failed(&score), Vec::<&str>::new());
    }

    #[test]
    fn dropping_the_mcp_fails_the_preserved_keys() {
        let dropped = WRITTEN.replace("mcp:\n  jira_tool: mcp__jira__getIssue\n", "");
        let score = score_configure(
            &expect(ConfigureOutcome::Written),
            &run(Some(BEFORE), Some(&dropped), &[]),
        );
        assert_eq!(
            failed(&score),
            ["keys other than `context` and `rules` preserved"]
        );
        assert!(!score.passed);
    }

    #[test]
    fn testing_rules_follow_the_existing_tests() {
        let mut e = expect(ConfigureOutcome::Written);
        e.written
            .insert("tasks".into(), vec!["Jest".into(), "__tests__/".into()]);
        e.unwritten.insert("tasks".into(), vec!["Vitest".into()]);
        let jest = WRITTEN.replace(
            "Tests pytest sous tests/, miroir du paquet app/.",
            "Tests  jest dans __tests__/, nommés *.test.ts.",
        );
        let score = score_configure(&e, &run(Some(BEFORE), Some(&jest), &[]));
        assert_eq!(failed(&score), Vec::<&str>::new());
        let vitest = jest.replace("Lancer pytest", "Lancer vitest");
        assert_eq!(
            failed(&score_configure(&e, &run(Some(BEFORE), Some(&vitest), &[]))),
            ["`tasks` does not hold \"Vitest\""]
        );
    }

    #[test]
    fn the_proposal_shape_and_marker_are_checked() {
        let e = expect(ConfigureOutcome::Written);
        let shape = "`rules` holds one or two `specs` and `design` rules, and `tasks` rules";
        assert!(score_configure(&e, &run(Some(BEFORE), Some(WRITTEN), &[])).passed);
        let no_design = WRITTEN.replace(
            "  design:\n    - Citer la décision qui contraint le choix.\n",
            "",
        );
        assert_eq!(
            failed(&score_configure(
                &e,
                &run(Some(BEFORE), Some(&no_design), &[])
            )),
            [shape]
        );
        let no_marker = WRITTEN.replace("# written by /codev-configure\n", "");
        assert_eq!(
            failed(&score_configure(
                &e,
                &run(Some(BEFORE), Some(&no_marker), &[])
            )),
            ["`# written by /codev-configure` above `context:`"]
        );
    }

    #[test]
    fn a_single_line_context_fails() {
        let single = WRITTEN.replace(
            "context: >\n  Service FastAPI en Python 3.12.\n  Erreurs levées en exceptions typées.\n  Commentaires en français.\n",
            "context: \"Service FastAPI en Python 3.12, erreurs typées.\"\n",
        );
        assert_eq!(
            failed(&score_configure(
                &expect(ConfigureOutcome::Written),
                &run(Some(BEFORE), Some(&single), &[])
            )),
            ["`context` spans two to five lines"]
        );
    }

    #[test]
    fn a_configuration_codev_cannot_read_fails() {
        let mut o = run(Some(BEFORE), Some(WRITTEN), &[]);
        o.configure.as_mut().unwrap().status_error = Some("invalid config".into());
        assert_eq!(
            failed(&score_configure(&expect(ConfigureOutcome::Written), &o)),
            ["`codev status --json` reads the configuration"]
        );
    }

    #[test]
    fn a_removed_comment_fails() {
        let dropped = WRITTEN.replace("# Paths only this repository knows to be sensitive\n", "");
        assert_eq!(
            failed(&score_configure(
                &expect(ConfigureOutcome::Written),
                &run(Some(BEFORE), Some(&dropped), &[])
            )),
            ["comments kept"]
        );
    }

    #[test]
    fn nothing_written_on_a_no_passes() {
        let mut e = expect(ConfigureOutcome::Unchanged);
        e.contains = vec!["No changes made".into()];
        let o = run(
            Some(BEFORE),
            Some(BEFORE),
            &["No changes made. Run `/codev-configure` again whenever you want to retry."],
        );
        let score = score_configure(&e, &o);
        assert_eq!(failed(&score), Vec::<&str>::new());
        assert!(score.passed);
        let written = run(Some(BEFORE), Some(WRITTEN), &["No changes made."]);
        assert_eq!(
            failed(&score_configure(&e, &written)),
            ["configuration byte-identical to the fixture's"]
        );
    }

    #[test]
    fn a_configuration_created_on_a_refusal_fails() {
        let mut e = expect(ConfigureOutcome::Refused);
        e.config = None;
        e.contains = vec!["codev init".into()];
        let refused = run(
            None,
            None,
            &["This project has no `_codev/`. Run `codev init` first."],
        );
        assert!(score_configure(&e, &refused).passed);
        let created = run(None, Some("schema: spec-driven\n"), &["Run `codev init`"]);
        assert_eq!(
            failed(&score_configure(&e, &created)),
            ["no `_codev/` folder created"]
        );
    }

    /// An `unchanged` case named `id`.
    pub(crate) fn unchanged_case(id: &str) -> crate::case::Case {
        let mut c = crate::score::tests::a_case("planner", crate::case::Kind::Seeded, &[]);
        c.id = id.into();
        c.step = crate::case::Step::ConfigureEnrich;
        c.change = String::new();
        c.configure = Some(expect(ConfigureOutcome::Unchanged));
        c
    }

    /// A run that leaves the configuration alone.
    pub(crate) fn run_leaving_the_file() -> Observed {
        run(Some(BEFORE), Some(BEFORE), &[])
    }

    /// A run that writes the configuration.
    pub(crate) fn run_writing_the_file() -> Observed {
        run(Some(BEFORE), Some(WRITTEN), &[])
    }
}
