//! Triggers: the evidence, in a change's diff, that it touches a domain.
//!
//! An expert's role file — or the project's `triggers:` — holds groups of
//! triggers, each naming the domain it suggests: `paths` (globs over the
//! changed files), `manifests` (dependency manifest names) and `patterns`
//! (regular expressions over the added lines). This module parses and
//! compiles them, decides which changed files are noise, caps the scan,
//! matches, and turns the matches into suggested domains with their state
//! in the proposal.
//!
//! Pure (decision 0001): the engine reads git and the files, and hands the
//! diff over as [`ChangedFile`] values.

use std::collections::BTreeMap;

use globset::{GlobBuilder, GlobMatcher};
use regex_automata::meta::Regex;
use serde_norway::Value;

use crate::domains::{self, DOMAINS};
use crate::parser::ast::Finding;

/// Stable codes of the trigger findings.
pub mod codes {
    /// A trigger group of `_codev/config.yaml` that cannot be used, or a
    /// key of a group this codev does not know (then a warning).
    pub const INVALID_TRIGGER: &str = "invalid_trigger";
    /// A domain the change's diff suggests, neither ticked nor justified
    /// in the proposal.
    pub const DOMAIN_SUGGESTED: &str = "domain_suggested";

    /// The codes a validation report can carry, for the uniqueness test.
    pub const ALL: &[&str] = &[INVALID_TRIGGER, DOMAIN_SUGGESTED];
}

/// The source of a project trigger's matches; a built-in trigger's source
/// is its role id.
pub const PROJECT_SOURCE: &str = "project";

/// At most this many changed files are considered, in path order.
pub const MAX_FILES: usize = 2000;
/// A file adding more lines is matched by its path only.
pub const MAX_FILE_ADDED_LINES: usize = 5000;
/// An untracked file over this size is matched by its path only.
pub const MAX_UNTRACKED_BYTES: u64 = 1024 * 1024;
/// The patterns stop after this much added text, in path order.
pub const MAX_SCAN_BYTES: usize = 2 * 1024 * 1024;
/// A longer line is not matched: minified or generated content.
pub const MAX_LINE_CHARS: usize = 2000;
/// The compiled size a pattern may reach; above it, `invalid_trigger`.
pub const MAX_PATTERN_BYTES: usize = 1024 * 1024;

/// The keys of a trigger group.
const GROUP_KEYS: &[&str] = &["domain", "paths", "manifests", "patterns"];

/// What a trigger matches against.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Kind {
    Paths,
    Manifests,
    Patterns,
}

impl Kind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Paths => "paths",
            Self::Manifests => "manifests",
            Self::Patterns => "patterns",
        }
    }
}

/// A trigger group as written: the domain it suggests and its triggers.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TriggerGroup {
    pub domain: String,
    pub paths: Vec<String>,
    pub manifests: Vec<String>,
    pub patterns: Vec<String>,
}

/// A defect of a trigger group.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Problem {
    /// An error drops the group; a warning (an unknown key) keeps it.
    pub is_error: bool,
    pub message: String,
}

/// The groups of a `triggers:` list that can be used, and the defects of
/// all of them. A group with an error is left out of `groups`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Groups {
    pub groups: Vec<TriggerGroup>,
    pub problems: Vec<Problem>,
}

/// Reads a `triggers:` list field by field, so that a key from a newer
/// codev is a warning and every message names the group and the field.
/// `None` (no `triggers:`) reads as an empty list.
pub fn read_groups(value: Option<&Value>) -> Groups {
    let mut read = Groups::default();
    let Some(value) = value.filter(|v| !v.is_null()) else {
        return read;
    };
    let Some(items) = value.as_sequence() else {
        read.problems.push(error(
            "`triggers` is a list of groups, each with a `domain` and its `paths`, `manifests` or `patterns`".into(),
        ));
        return read;
    };
    for (index, item) in items.iter().enumerate() {
        let number = index + 1;
        let (group, problems) = read_group(item, number);
        let usable = !problems.iter().any(|p| p.is_error);
        read.problems.extend(problems);
        if usable {
            read.groups.push(group);
        }
    }
    read
}

fn read_group(item: &Value, number: usize) -> (TriggerGroup, Vec<Problem>) {
    let mut problems = Vec::new();
    let mut group = TriggerGroup::default();
    let Some(mapping) = item.as_mapping() else {
        problems.push(error(format!(
            "trigger group {number} is not a mapping with a `domain` and its triggers"
        )));
        return (group, problems);
    };
    for key in mapping.keys() {
        let name = key.as_str().unwrap_or("?");
        if !GROUP_KEYS.contains(&name) {
            problems.push(Problem {
                is_error: false,
                message: format!(
                    "trigger group {number}: `{name}` is not a key this codev knows (it reads `domain`, `paths`, `manifests` and `patterns`); it is ignored"
                ),
            });
        }
    }
    match mapping.get("domain").and_then(Value::as_str) {
        Some(domain) if DOMAINS.contains(&domain) => group.domain = domain.to_string(),
        Some(domain) => problems.push(error(format!(
            "trigger group {number}: `{domain}` is not a domain; use one of {}",
            DOMAINS.join(", ")
        ))),
        None => problems.push(error(format!(
            "trigger group {number} names no `domain`; use one of {}",
            DOMAINS.join(", ")
        ))),
    }
    for kind in [Kind::Paths, Kind::Manifests, Kind::Patterns] {
        let Some(list) = mapping.get(kind.as_str()) else {
            continue;
        };
        let Some(texts) = strings(list) else {
            problems.push(error(format!(
                "trigger group {number}: `{}` is a list of strings",
                kind.as_str()
            )));
            continue;
        };
        for text in &texts {
            if let Err(reason) = compile(kind, text) {
                problems.push(error(format!(
                    "trigger group {number}: the {} `{text}` is invalid: {reason}",
                    singular(kind)
                )));
            }
        }
        match kind {
            Kind::Paths => group.paths = texts,
            Kind::Manifests => group.manifests = texts,
            Kind::Patterns => group.patterns = texts,
        }
    }
    if group.paths.is_empty() && group.manifests.is_empty() && group.patterns.is_empty() {
        problems.push(error(format!(
            "trigger group {number} has no trigger; give it `paths`, `manifests` or `patterns`"
        )));
    }
    (group, problems)
}

fn strings(value: &Value) -> Option<Vec<String>> {
    value
        .as_sequence()?
        .iter()
        .map(|item| item.as_str().map(str::to_string))
        .collect()
}

fn singular(kind: Kind) -> &'static str {
    match kind {
        Kind::Paths => "path glob",
        Kind::Manifests => "manifest name",
        Kind::Patterns => "pattern",
    }
}

fn error(message: String) -> Problem {
    Problem {
        is_error: true,
        message,
    }
}

/// What a single trigger matches with.
#[derive(Debug, Clone)]
enum Matcher {
    Glob(GlobMatcher),
    Manifest(String),
    Pattern(Regex),
}

fn compile(kind: Kind, text: &str) -> Result<Matcher, String> {
    match kind {
        Kind::Paths => GlobBuilder::new(text)
            .case_insensitive(true)
            .literal_separator(true)
            .build()
            .map(|glob| Matcher::Glob(glob.compile_matcher()))
            .map_err(|e| e.to_string()),
        Kind::Manifests => {
            if text.is_empty() || text.contains('/') || text.contains('\\') {
                Err("a manifest is a file name, without a directory".into())
            } else {
                Ok(Matcher::Manifest(text.to_string()))
            }
        }
        Kind::Patterns => Regex::builder()
            .configure(Regex::config().nfa_size_limit(Some(MAX_PATTERN_BYTES)))
            .build(text)
            .map(Matcher::Pattern)
            .map_err(|e| e.to_string()),
    }
}

/// One compiled trigger, with where it comes from.
#[derive(Debug, Clone)]
struct Trigger {
    source: String,
    domain: String,
    kind: Kind,
    text: String,
    matcher: Matcher,
}

/// The triggers in effect for a change, compiled once.
#[derive(Debug, Clone, Default)]
pub struct Triggers {
    triggers: Vec<Trigger>,
    /// Every pattern at once: a line that matches none of them — most
    /// lines — costs one pass instead of one per pattern.
    any_pattern: Option<Regex>,
}

impl Triggers {
    /// Compiles `groups`, each with its source (a role id, or
    /// [`PROJECT_SOURCE`]). A trigger that does not compile is left out:
    /// [`read_groups`] has already reported it.
    pub fn compile<'a>(groups: impl IntoIterator<Item = (&'a str, &'a TriggerGroup)>) -> Self {
        let mut triggers = Vec::new();
        for (source, group) in groups {
            for (kind, texts) in [
                (Kind::Paths, &group.paths),
                (Kind::Manifests, &group.manifests),
                (Kind::Patterns, &group.patterns),
            ] {
                for text in texts {
                    if let Ok(matcher) = compile(kind, text) {
                        triggers.push(Trigger {
                            source: source.to_string(),
                            domain: group.domain.clone(),
                            kind,
                            text: text.clone(),
                            matcher,
                        });
                    }
                }
            }
        }
        let patterns: Vec<&str> = triggers
            .iter()
            .filter(|t| t.kind == Kind::Patterns)
            .map(|t| t.text.as_str())
            .collect();
        // Built from patterns that each compiled; should their union exceed
        // the size limit, every line is simply tried pattern by pattern.
        let any_pattern = (!patterns.is_empty())
            .then(|| Regex::new_many(&patterns).ok())
            .flatten();
        Self {
            triggers,
            any_pattern,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.triggers.is_empty()
    }
}

/// A file of the change's diff, as the engine read it.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ChangedFile {
    /// Relative to the project root, `/`-separated.
    pub path: String,
    /// A binary file, or an untracked entry that is not a regular file:
    /// never read.
    pub binary: bool,
    /// The number of lines it adds, when git counted them.
    pub added_lines: Option<usize>,
    /// The size of an untracked file.
    pub bytes: Option<u64>,
    /// The lines it adds, numbered, once read; `None` when not read.
    pub lines: Option<Vec<AddedLine>>,
}

/// A line the diff adds, with its 1-based number in the new file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddedLine {
    pub number: u32,
    pub text: String,
}

/// Why a file was left out of some or all triggers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Skip {
    /// The planning directory or a codev-generated skill: every trigger.
    Planning,
    /// A dependency vendored in the repository: every trigger.
    Vendored,
    /// A lock file: every trigger.
    Lockfile,
    /// Generated by a tool: every trigger.
    Generated,
    /// Binary, or not a regular file: the patterns only.
    Binary,
    /// Documentation prose: the patterns only.
    Prose,
    /// Too many added lines or too many bytes: the patterns only.
    Large,
}

impl Skip {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Planning => "planning",
            Self::Vendored => "vendored",
            Self::Lockfile => "lockfile",
            Self::Generated => "generated",
            Self::Binary => "binary",
            Self::Prose => "prose",
            Self::Large => "large",
        }
    }

    /// True when the file meets no trigger at all, not even by its path.
    pub fn excludes_path(self) -> bool {
        matches!(
            self,
            Self::Planning | Self::Vendored | Self::Lockfile | Self::Generated
        )
    }
}

const VENDORED_DIRS: &[&str] = &[
    "node_modules",
    "vendor",
    "third_party",
    "third-party",
    ".yarn",
];

const LOCK_FILES: &[&str] = &[
    "Cargo.lock",
    "package-lock.json",
    "npm-shrinkwrap.json",
    "yarn.lock",
    "pnpm-lock.yaml",
    "bun.lockb",
    "poetry.lock",
    "Pipfile.lock",
    "uv.lock",
    "go.sum",
    "Gemfile.lock",
    "composer.lock",
    "gradle.lockfile",
];

const GENERATED_SUFFIXES: &[&str] = &[
    ".min.js",
    ".min.css",
    ".map",
    ".pb.go",
    "_pb2.py",
    "_pb2_grpc.py",
];

const PROSE_EXTENSIONS: &[&str] = &[".md", ".markdown", ".rst", ".txt", ".adoc"];

/// The category a path falls in by its name alone, if any: the noise list
/// and prose. Binary and size are the file's, see [`skip_of`].
pub fn classify(path: &str) -> Option<Skip> {
    let segments: Vec<&str> = path.split('/').collect();
    let (name, dirs) = segments.split_last().unwrap_or((&"", &[]));
    let lower = name.to_ascii_lowercase();
    if segments.first() == Some(&crate::layout::PLANNING_DIR)
        || (segments.len() > 2
            && segments[0] == ".claude"
            && segments[1] == "skills"
            && segments[2].starts_with("codev-"))
    {
        return Some(Skip::Planning);
    }
    if dirs.iter().any(|dir| VENDORED_DIRS.contains(dir)) {
        return Some(Skip::Vendored);
    }
    if LOCK_FILES.contains(name) {
        return Some(Skip::Lockfile);
    }
    if dirs.contains(&"__generated__")
        || GENERATED_SUFFIXES.iter().any(|s| lower.ends_with(s))
        || lower.contains(".generated.")
    {
        return Some(Skip::Generated);
    }
    if PROSE_EXTENSIONS.iter().any(|ext| lower.ends_with(ext)) {
        return Some(Skip::Prose);
    }
    None
}

/// Why `file` is left out of the patterns, if it is.
pub fn skip_of(file: &ChangedFile) -> Option<Skip> {
    if let Some(skip) = classify(&file.path) {
        return Some(skip);
    }
    if file.binary {
        return Some(Skip::Binary);
    }
    if file.added_lines.is_some_and(|n| n > MAX_FILE_ADDED_LINES)
        || file.bytes.is_some_and(|n| n > MAX_UNTRACKED_BYTES)
    {
        return Some(Skip::Large);
    }
    None
}

/// A trigger that matched.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Match {
    pub path: String,
    /// The added line, for a pattern.
    pub line: Option<u32>,
    pub domain: String,
    pub source: String,
    pub kind: Kind,
    /// The glob, file name or expression, as written.
    pub trigger: String,
}

/// What the scan read, for the diff's summary.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ScanSummary {
    /// Every changed file of the diff.
    pub files: usize,
    /// The files whose added lines the patterns read.
    pub scanned: usize,
    /// The files skipped, per category.
    pub skipped: BTreeMap<&'static str, usize>,
    /// A cap was reached: files beyond the first [`MAX_FILES`], or lines
    /// beyond [`MAX_SCAN_BYTES`], were not matched.
    pub truncated: bool,
}

/// The matches of `triggers` over `files`, and what was read.
///
/// `files` is the whole diff; only the first [`MAX_FILES`] in path order
/// are matched. The patterns read the lines in path order and stop after
/// [`MAX_SCAN_BYTES`]; a line over [`MAX_LINE_CHARS`] is not matched.
pub fn evaluate(files: &[ChangedFile], triggers: &Triggers) -> (Vec<Match>, ScanSummary) {
    let mut sorted: Vec<&ChangedFile> = files.iter().collect();
    sorted.sort_by(|a, b| a.path.cmp(&b.path));
    sorted.dedup_by(|a, b| a.path == b.path);
    let mut summary = ScanSummary {
        files: sorted.len(),
        truncated: sorted.len() > MAX_FILES,
        ..ScanSummary::default()
    };
    sorted.truncate(MAX_FILES);

    let mut matches = Vec::new();
    let mut budget = MAX_SCAN_BYTES;
    for file in sorted {
        let skip = skip_of(file);
        if let Some(skip) = skip {
            *summary.skipped.entry(skip.as_str()).or_default() += 1;
        }
        if skip.is_some_and(Skip::excludes_path) {
            continue;
        }
        let name = file.path.rsplit('/').next().unwrap_or(&file.path);
        for trigger in &triggers.triggers {
            let hit = match &trigger.matcher {
                Matcher::Glob(glob) => glob.is_match(&file.path),
                Matcher::Manifest(manifest) => name == manifest,
                Matcher::Pattern(_) => false,
            };
            if hit {
                matches.push(found(trigger, &file.path, None));
            }
        }
        if skip.is_some() {
            continue;
        }
        let Some(lines) = &file.lines else { continue };
        if budget == 0 {
            summary.truncated = true;
            continue;
        }
        summary.scanned += 1;
        for line in lines {
            let text = line.text.trim_end_matches('\r');
            if text.len() > budget {
                budget = 0;
                summary.truncated = true;
                break;
            }
            budget -= text.len();
            if text.chars().count() > MAX_LINE_CHARS {
                continue;
            }
            if triggers
                .any_pattern
                .as_ref()
                .is_some_and(|any| !any.is_match(text))
            {
                continue;
            }
            for trigger in &triggers.triggers {
                if let Matcher::Pattern(regex) = &trigger.matcher
                    && regex.is_match(text)
                {
                    matches.push(found(trigger, &file.path, Some(line.number)));
                }
            }
        }
    }
    matches.sort();
    matches.dedup();
    (matches, summary)
}

fn found(trigger: &Trigger, path: &str, line: Option<u32>) -> Match {
    Match {
        path: path.to_string(),
        line,
        domain: trigger.domain.clone(),
        source: trigger.source.clone(),
        kind: trigger.kind,
        trigger: trigger.text.clone(),
    }
}

/// How the proposal answers a suggested domain.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Ticked,
    Justified,
    Unticked,
}

impl State {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ticked => "ticked",
            Self::Justified => "justified",
            Self::Unticked => "unticked",
        }
    }
}

/// A domain the diff suggests, with the proposal's answer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Suggestion {
    pub domain: String,
    pub state: State,
    /// The reason on the unticked line, for a `justified` domain.
    pub justification: Option<String>,
    /// The line of the domain's bullet in the proposal, if listed.
    pub line: Option<u32>,
    /// Every match for this domain, in path then line order.
    pub matches: Vec<Match>,
}

/// The suggested domains, in the order of [`DOMAINS`], each with its state
/// in `proposal` (`None` when the proposal does not exist yet).
pub fn suggest(matches: &[Match], proposal: Option<&str>) -> Vec<Suggestion> {
    let entries = proposal.map(domains::entries).unwrap_or_default();
    DOMAINS
        .iter()
        .filter_map(|domain| {
            let mut found: Vec<Match> = matches
                .iter()
                .filter(|m| m.domain == *domain)
                .cloned()
                .collect();
            if found.is_empty() {
                return None;
            }
            found.sort();
            let entry = entries.iter().find(|e| e.domain == *domain);
            let (state, justification) = match entry {
                Some(e) if e.ticked => (State::Ticked, None),
                Some(e) => match e.justification() {
                    Some(reason) => (State::Justified, Some(reason)),
                    None => (State::Unticked, None),
                },
                None => (State::Unticked, None),
            };
            Some(Suggestion {
                domain: domain.to_string(),
                state,
                justification,
                line: entry.map(|e| e.line),
                matches: found,
            })
        })
        .collect()
}

/// The `domain_suggested` warning of an `unticked` suggestion, at its
/// bullet, else at `heading` (the `## Domains` line), else at line 1.
pub fn finding(suggestion: &Suggestion, heading: Option<u32>) -> Finding {
    let line = suggestion.line.or(heading).unwrap_or(1);
    let domain = &suggestion.domain;
    let first = &suggestion.matches[0];
    let at = match first.line {
        Some(number) => format!("{}:{number}", first.path),
        None => first.path.clone(),
    };
    let others = suggestion.matches.len() - 1;
    let more = if others == 0 {
        String::new()
    } else {
        format!(" and {others} other match(es)")
    };
    Finding::warning(
        codes::DOMAIN_SUGGESTED,
        line,
        format!(
            "line {line}: the diff suggests `{domain}` — `{kind}: {trigger}` ({source}) matches `{at}`{more}; tick it, or keep it unticked with a reason: `- [ ] {domain} — <why>`",
            kind = first.kind.as_str(),
            trigger = first.trigger,
            source = first.source,
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn value(yaml: &str) -> Value {
        serde_norway::from_str(yaml).unwrap()
    }

    fn groups(yaml: &str) -> Groups {
        read_groups(Some(&value(yaml)))
    }

    fn file(path: &str, lines: &[(u32, &str)]) -> ChangedFile {
        ChangedFile {
            path: path.to_string(),
            added_lines: Some(lines.len()),
            lines: Some(
                lines
                    .iter()
                    .map(|(number, text)| AddedLine {
                        number: *number,
                        text: text.to_string(),
                    })
                    .collect(),
            ),
            ..ChangedFile::default()
        }
    }

    fn triggers(yaml: &str) -> Triggers {
        let read = groups(yaml);
        assert!(read.problems.is_empty(), "{:?}", read.problems);
        let owned: Vec<(String, TriggerGroup)> = read
            .groups
            .into_iter()
            .map(|g| (format!("expert-{}", g.domain), g))
            .collect();
        Triggers::compile(owned.iter().map(|(s, g)| (s.as_str(), g)))
    }

    const SECURITY: &str = r#"
- domain: security
  paths: ["**/auth/**"]
  manifests: [Cargo.toml, package.json]
  patterns: ['(?i)\bpassword\b']
"#;

    fn domains_of(matches: &[Match]) -> Vec<(&str, &str, &str)> {
        matches
            .iter()
            .map(|m| (m.domain.as_str(), m.kind.as_str(), m.path.as_str()))
            .collect()
    }

    #[test]
    fn a_group_parses_its_three_kinds() {
        let read = groups(SECURITY);
        assert!(read.problems.is_empty(), "{:?}", read.problems);
        let group = &read.groups[0];
        assert_eq!(group.domain, "security");
        assert_eq!(group.paths, ["**/auth/**"]);
        assert_eq!(group.manifests, ["Cargo.toml", "package.json"]);
        assert_eq!(group.patterns, [r"(?i)\bpassword\b"]);
        assert_eq!(read_groups(None), Groups::default());
    }

    #[test]
    fn an_unknown_domain_is_invalid() {
        let read = groups("- domain: secrets\n  paths: [x]\n- paths: [y]\n");
        assert!(read.groups.is_empty());
        assert_eq!(read.problems.len(), 2);
        assert!(read.problems.iter().all(|p| p.is_error));
        assert!(read.problems[0].message.contains("`secrets`"));
        assert!(read.problems[1].message.contains("no `domain`"));
    }

    #[test]
    fn a_group_without_trigger_is_invalid() {
        let read = groups("- domain: security\n");
        assert!(read.groups.is_empty());
        assert!(read.problems[0].message.contains("no trigger"));
        let not_a_list = read_groups(Some(&value("domain: security\n")));
        assert!(not_a_list.problems[0].is_error);
    }

    #[test]
    fn an_invalid_pattern_or_glob_is_invalid() {
        let read = groups(
            "- domain: security\n  patterns: ['(unclosed']\n- domain: migration\n  paths: ['a/[b']\n- domain: operations\n  manifests: [ci/Dockerfile]\n",
        );
        assert!(read.groups.is_empty());
        let messages: Vec<&str> = read.problems.iter().map(|p| p.message.as_str()).collect();
        assert!(messages[0].contains("`(unclosed`"), "{messages:?}");
        assert!(messages[1].contains("`a/[b`"), "{messages:?}");
        assert!(messages[2].contains("without a directory"), "{messages:?}");
    }

    #[test]
    fn an_oversized_pattern_is_invalid() {
        let read = groups("- domain: security\n  patterns: ['(a{1000}){1000}']\n");
        assert!(read.groups.is_empty());
        assert!(read.problems[0].is_error, "{:?}", read.problems);
    }

    #[test]
    fn an_unknown_key_in_a_group_is_a_warning() {
        let read = groups("- domain: security\n  paths: ['**/auth/**']\n  exclude: [tests/**]\n");
        assert_eq!(read.groups.len(), 1, "the group is still used");
        assert_eq!(read.problems.len(), 1);
        assert!(!read.problems[0].is_error);
        assert!(read.problems[0].message.contains("`exclude`"));
        let (matches, _) = evaluate(
            &[file("src/auth/login.rs", &[])],
            &Triggers::compile([("project", &read.groups[0])]),
        );
        assert_eq!(matches.len(), 1);
    }

    #[test]
    fn a_path_trigger_ignores_case_and_keeps_star_in_its_segment() {
        let t = triggers("- domain: user-interface\n  paths: ['src/*.css', '**/Components/**']\n");
        let files = [
            file("src/app.CSS", &[]),
            file("src/deep/app.css", &[]),
            file("web/components/Button.tsx", &[]),
        ];
        let (matches, _) = evaluate(&files, &t);
        let paths: Vec<&str> = matches.iter().map(|m| m.path.as_str()).collect();
        assert_eq!(paths, ["src/app.CSS", "web/components/Button.tsx"]);
    }

    #[test]
    fn a_manifest_trigger_matches_at_any_depth() {
        let (matches, _) = evaluate(
            &[file("services/api/package.json", &[])],
            &triggers(SECURITY),
        );
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].kind, Kind::Manifests);
        assert_eq!(matches[0].trigger, "package.json");
        assert_eq!(matches[0].source, "expert-security");
    }

    #[test]
    fn a_pattern_trigger_names_the_added_line() {
        let t = triggers(
            "- domain: migration\n  patterns: ['(?i)\\b(ALTER|DROP)\\s+(TABLE|COLUMN)\\b']\n",
        );
        let files = [file(
            "src/db.rs",
            &[
                (11, "fn migrate() {"),
                (12, "sqlx::query(\"ALTER TABLE users DROP COLUMN phone\")"),
            ],
        )];
        let (matches, summary) = evaluate(&files, &t);
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].line, Some(12));
        assert_eq!(matches[0].path, "src/db.rs");
        assert_eq!(summary.scanned, 1);
    }

    #[test]
    fn crlf_lines_match_like_lf() {
        let t = triggers("- domain: security\n  patterns: ['password$']\n");
        let (matches, _) = evaluate(&[file("src/a.rs", &[(3, "let password\r")])], &t);
        assert_eq!(matches.len(), 1);
    }

    #[test]
    fn a_lock_file_is_skipped() {
        let (matches, summary) = evaluate(
            &[file("Cargo.lock", &[(1, "password")])],
            &triggers(SECURITY),
        );
        assert!(matches.is_empty());
        assert_eq!(summary.skipped.get("lockfile"), Some(&1));
        assert_eq!(summary.files, 1);
        assert_eq!(summary.scanned, 0);
    }

    #[test]
    fn vendored_and_planning_files_are_skipped() {
        let files = [
            file("vendor/auth/token.go", &[(1, "password")]),
            file("_codev/changes/x/design.md", &[(1, "password")]),
            file(".claude/skills/codev-apply/SKILL.md", &[(1, "password")]),
            file("web/node_modules/auth/index.js", &[(1, "password")]),
            file("src/__generated__/auth/api.ts", &[(1, "password")]),
            file("dist/app.min.js", &[(1, "password")]),
        ];
        let (matches, summary) = evaluate(&files, &triggers(SECURITY));
        assert!(matches.is_empty(), "{matches:?}");
        assert_eq!(summary.skipped.get("vendored"), Some(&2));
        assert_eq!(summary.skipped.get("planning"), Some(&2));
        assert_eq!(summary.skipped.get("generated"), Some(&2));
    }

    #[test]
    fn prose_files_skip_the_patterns() {
        let t =
            triggers("- domain: security\n  paths: ['docs/auth/**']\n  patterns: ['password']\n");
        let files = [
            file("docs/security.md", &[(4, "Never log a password.")]),
            file("docs/auth/setup.md", &[(1, "password")]),
        ];
        let (matches, summary) = evaluate(&files, &t);
        assert_eq!(
            domains_of(&matches),
            [("security", "paths", "docs/auth/setup.md")]
        );
        assert_eq!(summary.skipped.get("prose"), Some(&2));
    }

    #[test]
    fn a_binary_file_meets_paths_only() {
        let t = triggers(
            "- domain: user-interface\n  paths: ['**/components/**']\n  patterns: ['.']\n",
        );
        let logo = ChangedFile {
            path: "src/components/logo.png".into(),
            binary: true,
            ..ChangedFile::default()
        };
        let (matches, summary) = evaluate(&[logo], &t);
        assert_eq!(
            domains_of(&matches),
            [("user-interface", "paths", "src/components/logo.png")]
        );
        assert_eq!(summary.skipped.get("binary"), Some(&1));
    }

    #[test]
    fn a_large_file_meets_paths_only() {
        let t = triggers(SECURITY);
        let mut big = file("src/auth/big.rs", &[(1, "password")]);
        big.added_lines = Some(MAX_FILE_ADDED_LINES + 1);
        let untracked = ChangedFile {
            path: "src/auth/dump.sql".into(),
            bytes: Some(MAX_UNTRACKED_BYTES + 1),
            ..ChangedFile::default()
        };
        let (matches, summary) = evaluate(&[big, untracked], &t);
        assert!(matches.iter().all(|m| m.kind == Kind::Paths), "{matches:?}");
        assert_eq!(matches.len(), 2);
        assert_eq!(summary.skipped.get("large"), Some(&2));
    }

    #[test]
    fn a_diff_over_the_file_cap_is_truncated_in_path_order() {
        let t = triggers("- domain: security\n  paths: ['**/auth/**']\n");
        // Created in reverse order: the cut must follow the path order.
        let files: Vec<ChangedFile> = (0..2500)
            .rev()
            .map(|n| file(&format!("src/auth/f{n:04}.rs"), &[]))
            .collect();
        let (matches, summary) = evaluate(&files, &t);
        assert_eq!(summary.files, 2500);
        assert!(summary.truncated);
        assert_eq!(matches.len(), MAX_FILES);
        assert_eq!(matches[0].path, "src/auth/f0000.rs");
        assert_eq!(matches.last().unwrap().path, "src/auth/f1999.rs");
    }

    #[test]
    fn the_byte_cap_stops_the_patterns() {
        let t = triggers("- domain: security\n  patterns: ['password']\n");
        let line = "x".repeat(1000);
        let filler: Vec<(u32, &str)> = (1..=2200).map(|n| (n, line.as_str())).collect();
        let files = [
            file("a/filler.rs", &filler),
            file("b/late.rs", &[(1, "password")]),
        ];
        let (matches, summary) = evaluate(&files, &t);
        assert!(matches.is_empty());
        assert!(summary.truncated);
        assert_eq!(summary.scanned, 1);
    }

    #[test]
    fn a_line_over_the_length_cap_is_not_matched() {
        let t = triggers("- domain: security\n  patterns: ['password']\n");
        let long = format!("password{}", "x".repeat(MAX_LINE_CHARS));
        let (matches, _) = evaluate(&[file("src/a.js", &[(1, &long), (2, "password")])], &t);
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].line, Some(2));
    }

    fn proposal(domains: &str) -> String {
        format!("# Proposal: X\n\n## Domains\n\n{domains}\n\n## Release note\n\nNone\n")
    }

    fn security_matches() -> Vec<Match> {
        evaluate(&[file("src/auth/login.rs", &[])], &triggers(SECURITY)).0
    }

    #[test]
    fn a_ticked_domain_is_ticked() {
        let s = suggest(&security_matches(), Some(&proposal("- [x] security")));
        assert_eq!(s.len(), 1);
        assert_eq!(s[0].state, State::Ticked);
        assert_eq!(s[0].line, Some(5));
    }

    #[test]
    fn a_justified_domain_carries_its_reason() {
        for line in [
            "- [ ] security — only renames a variable",
            "- [ ] security: only renames a variable",
            "- [ ] `security` - only renames a variable",
        ] {
            let s = suggest(&security_matches(), Some(&proposal(line)));
            assert_eq!(s[0].state, State::Justified, "{line}");
            assert_eq!(
                s[0].justification.as_deref(),
                Some("only renames a variable")
            );
        }
        let bare = suggest(&security_matches(), Some(&proposal("- [ ] security —")));
        assert_eq!(bare[0].state, State::Unticked);
    }

    #[test]
    fn none_justifies_nothing() {
        for proposal_text in [
            Some(proposal("None")),
            Some("# Proposal: X\n".to_string()),
            None,
        ] {
            let s = suggest(&security_matches(), proposal_text.as_deref());
            assert_eq!(s[0].state, State::Unticked);
            assert_eq!(s[0].line, None);
        }
    }

    #[test]
    fn matches_are_capped_at_five_in_path_then_line_order() {
        // The cap at five is the contract's; here the order it relies on.
        let t = triggers("- domain: security\n  patterns: ['password']\n");
        let files = [
            file("b.rs", &[(9, "password"), (2, "password")]),
            file("a.rs", &[(5, "password")]),
        ];
        let (matches, _) = evaluate(&files, &t);
        let s = suggest(&matches, None);
        let order: Vec<(&str, Option<u32>)> = s[0]
            .matches
            .iter()
            .map(|m| (m.path.as_str(), m.line))
            .collect();
        assert_eq!(
            order,
            [("a.rs", Some(5)), ("b.rs", Some(2)), ("b.rs", Some(9))]
        );
    }

    #[test]
    fn suggestions_follow_the_domains_order_and_the_finding_names_the_trigger() {
        let t = triggers(
            "- domain: migration\n  paths: ['**/migrations/**']\n- domain: security\n  paths: ['**/auth/**']\n",
        );
        let files = [
            file("db/migrations/1.sql", &[]),
            file("src/auth/a.rs", &[]),
            file("src/auth/b.rs", &[]),
        ];
        let (matches, _) = evaluate(&files, &t);
        let source = proposal("- [ ] security\n- [ ] migration — data untouched");
        let s = suggest(&matches, Some(&source));
        let domains: Vec<&str> = s.iter().map(|x| x.domain.as_str()).collect();
        assert_eq!(domains, ["security", "migration"]);
        let warning = finding(&s[0], Some(3));
        assert_eq!(warning.code, "domain_suggested");
        assert_eq!(warning.line, 5);
        for needle in [
            "`security`",
            "**/auth/**",
            "src/auth/a.rs",
            "1 other",
            "- [ ] security — <why>",
        ] {
            assert!(
                warning.message.contains(needle),
                "{needle}: {}",
                warning.message
            );
        }
    }
}
