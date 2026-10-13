//! The fixture lint: a case's planning artifacts cite only files and tests
//! its fixture contains, or that the case declares as `planned`.
//!
//! A plan that names a file the fixture lacks hands the reviewer a gap that
//! is the case's fault, not the role's: the first baseline counted about
//! nine such false positives. The rule reads only backquoted spans, where
//! plans name files and tests, and leaves what it cannot recognise alone
//! rather than guess.

use std::collections::{BTreeMap, BTreeSet};

/// The extensions that make a backquoted span a file path.
const EXTENSIONS: [&str; 25] = [
    "rs", "ts", "tsx", "js", "jsx", "py", "sql", "html", "css", "scss", "md", "toml", "json",
    "yaml", "yml", "lock", "vue", "svelte", "go", "java", "kt", "rb", "sh", "txt", "csv",
];

/// What a span cites.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Reference {
    Path(String),
    Test(String),
}

impl Reference {
    fn text(&self) -> &str {
        match self {
            Reference::Path(p) | Reference::Test(p) => p,
        }
    }
}

/// Every problem of the change's artifacts: citations the fixture cannot
/// answer, and `planned` entries that are stale or cited by nothing. `files`
/// is the fixture as the change leaves it; `planned` is normalized.
pub fn lint(change_dir: &str, files: &BTreeMap<&str, &[u8]>, planned: &[String]) -> Vec<String> {
    let texts: BTreeMap<&str, String> = files
        .iter()
        .map(|(path, bytes)| (*path, String::from_utf8_lossy(bytes).into_owned()))
        .collect();
    let prefix = format!("{change_dir}/");
    let mut problems = BTreeSet::new();
    let mut cited = BTreeSet::new();
    for (path, text) in &texts {
        let Some(artifact) = path.strip_prefix(&prefix) else {
            continue;
        };
        if !artifact.ends_with(".md") {
            continue;
        }
        let is_tasks = artifact == "tasks.md";
        for reference in references(text, is_tasks) {
            cited.insert(reference.text().to_string());
            if planned.iter().any(|p| p == reference.text())
                || resolves(&reference, &texts, change_dir)
            {
                continue;
            }
            problems.insert(match &reference {
                Reference::Path(p) => {
                    format!("`{artifact}` cites `{p}`, which the fixture does not have")
                }
                Reference::Test(t) => format!(
                    "`{artifact}` cites the test `{t}`, which no test file of the fixture holds"
                ),
            });
        }
    }
    for entry in planned {
        let as_path = Reference::Path(entry.clone());
        let as_test = Reference::Test(entry.clone());
        if resolves(&as_path, &texts, change_dir) || resolves(&as_test, &texts, change_dir) {
            problems.insert(format!(
                "`planned` lists `{entry}`, which the fixture already has"
            ));
        } else if !cited.contains(entry) {
            problems.insert(format!(
                "`planned` lists `{entry}`, which no artifact cites"
            ));
        }
    }
    problems.into_iter().collect()
}

/// The references of a markdown text: backquoted spans outside fenced code
/// blocks. Tests are read in `tasks.md` only.
fn references(text: &str, is_tasks: bool) -> Vec<Reference> {
    let mut found = Vec::new();
    let mut fenced = false;
    for line in text.lines() {
        if line.trim_start().starts_with("```") {
            fenced = !fenced;
            continue;
        }
        if fenced {
            continue;
        }
        // Snake-case spans are tests when introduced by "test" or "tests",
        // and so are those listed after them ("tests `a` and `b`").
        let mut in_test_list = false;
        let parts: Vec<&str> = line.split('`').collect();
        for (n, span) in parts.iter().enumerate().skip(1).step_by(2) {
            let before = parts[n - 1].trim().to_lowercase();
            in_test_list = if before.ends_with("test") || before.ends_with("tests") {
                true
            } else {
                in_test_list
                    && matches!(
                        without_parentheses(&before).trim(),
                        "" | "," | "and" | ", and" | "or"
                    )
            };
            let span = span.trim();
            if span.is_empty() {
                continue;
            }
            let words: Vec<&str> = span.split_whitespace().collect();
            if words.len() == 1 {
                found.extend(path_of(span, true).map(Reference::Path));
            } else {
                found.extend(
                    words
                        .iter()
                        .filter(|w| w.contains('/') || w.contains('\\'))
                        .filter_map(|w| path_of(w, false))
                        .map(Reference::Path),
                );
            }
            if is_tasks {
                if words.len() == 1 && in_test_list && is_snake_case(span) {
                    found.push(Reference::Test(span.to_string()));
                }
                found.extend(selected_test(&words).map(Reference::Test));
            }
        }
    }
    found
}

/// `text` without its parenthesized parts: `("A quiet import") and`
/// becomes ` and`.
fn without_parentheses(text: &str) -> String {
    let mut depth = 0usize;
    text.chars()
        .filter(|c| match c {
            '(' => {
                depth += 1;
                false
            }
            ')' => {
                depth = depth.saturating_sub(1);
                false
            }
            _ => depth == 0,
        })
        .collect()
}

/// A file path, normalized, if `word` is one: `\` read as `/`, a `:12`
/// or `:12-14` suffix and a `#heading` removed. A bare name (no `/`)
/// counts only when `bare` allows it.
fn path_of(word: &str, bare: bool) -> Option<String> {
    let word = word.replace('\\', "/");
    let word = word.trim_end_matches([',', '.', ';', ')']);
    let word = word.split('#').next().unwrap_or_default();
    let word = strip_line_suffix(word);
    let word = word.strip_prefix("./").unwrap_or(word);
    if word.is_empty()
        || word.starts_with('/')
        || word.starts_with('-')
        || word.contains("://")
        || word.contains(['*', '?', '{', '<', '$', '(', '=', '@'])
    {
        return None;
    }
    if word.ends_with('/') {
        return (word.len() > 1).then(|| word.to_string());
    }
    let last = word.rsplit('/').next().unwrap_or(word);
    let has_extension = last
        .rsplit_once('.')
        .is_some_and(|(stem, ext)| !stem.is_empty() && EXTENSIONS.contains(&ext));
    if !has_extension || (!bare && !word.contains('/')) {
        return None;
    }
    Some(word.to_string())
}

/// `src/a.rs:12` and `src/a.rs:12-14` without their line suffix.
fn strip_line_suffix(word: &str) -> &str {
    if let Some((path, lines)) = word.rsplit_once(':') {
        let numeric = |s: &str| !s.is_empty() && s.chars().all(|c| c.is_ascii_digit());
        let is_range = match lines.split_once('-') {
            Some((a, b)) => numeric(a) && numeric(b),
            None => numeric(lines),
        };
        if is_range {
            return path;
        }
    }
    word
}

/// `^[a-z][a-z0-9]*(_[a-z0-9]+)+$`: a test function's name.
fn is_snake_case(word: &str) -> bool {
    let mut parts = word.split('_');
    let first = parts.next().unwrap_or_default();
    let starts = first.chars().next().is_some_and(|c| c.is_ascii_lowercase());
    let part_ok = |p: &str| {
        !p.is_empty()
            && p.chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
    };
    let rest: Vec<&str> = parts.collect();
    starts && part_ok(first) && !rest.is_empty() && rest.iter().all(|p| part_ok(p))
}

/// Flags of `cargo test` that take a value: the value is not the filter.
const CARGO_VALUE_FLAGS: [&str; 8] = [
    "-p",
    "--package",
    "--test",
    "--bin",
    "--features",
    "-F",
    "--manifest-path",
    "--target",
];

/// The test a command selects: the first word after `cargo test` that is
/// neither a flag nor a flag's value, the first non-flag word after `--`
/// in `npm test --`, or the word after `-k` in `pytest -k`.
fn selected_test(words: &[&str]) -> Option<String> {
    let unquote = |w: &str| w.trim_matches(['"', '\'']).to_string();
    match words {
        ["cargo", "test", rest @ ..] => {
            let mut skip_value = false;
            for word in rest {
                if *word == "--" {
                    return None;
                }
                if skip_value {
                    skip_value = false;
                } else if word.starts_with('-') {
                    skip_value = CARGO_VALUE_FLAGS.contains(word);
                } else {
                    return Some(unquote(word));
                }
            }
            None
        }
        ["npm", "test", "--", rest @ ..] => rest
            .iter()
            .find(|w| !w.starts_with('-'))
            .map(|w| unquote(w)),
        ["pytest", ..] => words
            .iter()
            .position(|w| *w == "-k")
            .and_then(|i| words.get(i + 1))
            .map(|w| unquote(w)),
        _ => None,
    }
}

/// Whether the fixture answers the reference: a file or directory, relative
/// to the project or to the change's directory, or any file of a bare name;
/// a test held by a test file.
fn resolves(reference: &Reference, texts: &BTreeMap<&str, String>, change_dir: &str) -> bool {
    match reference {
        Reference::Path(path) => {
            if let Some(dir) = path.strip_suffix('/') {
                let in_project = format!("{dir}/");
                let in_change = format!("{change_dir}/{dir}/");
                return texts
                    .keys()
                    .any(|f| f.starts_with(&in_project) || f.starts_with(&in_change));
            }
            if !path.contains('/') {
                return texts
                    .keys()
                    .any(|f| f.rsplit('/').next() == Some(path.as_str()));
            }
            texts.contains_key(path.as_str())
                || texts.contains_key(format!("{change_dir}/{path}").as_str())
        }
        Reference::Test(name) => texts.iter().any(|(path, text)| {
            let is_test_file = !path.starts_with("_codev/")
                && (path.contains("test")
                    || path.contains("spec")
                    || (path.ends_with(".rs") && text.contains("#[test]")));
            is_test_file && text.contains(name.as_str())
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture(files: &[(&'static str, &'static str)]) -> BTreeMap<&'static str, &'static [u8]> {
        files.iter().map(|(p, t)| (*p, t.as_bytes())).collect()
    }

    const DIR: &str = "_codev/changes/add-quiet-flag";

    #[test]
    fn a_cited_file_the_fixture_lacks_is_named_with_its_artifact() {
        let files = fixture(&[
            ("src/lib.rs", ""),
            (
                "_codev/changes/add-quiet-flag/proposal.md",
                "## Impact\n\n- `src/cli/progress.rs`, `src/lib.rs`.\n",
            ),
        ]);
        assert_eq!(
            lint(DIR, &files, &[]),
            ["`proposal.md` cites `src/cli/progress.rs`, which the fixture does not have"]
        );
    }

    #[test]
    fn locations_headings_directories_and_bare_names_resolve() {
        let files = fixture(&[
            ("web/src/settings/Page.tsx", ""),
            ("README.md", ""),
            ("_codev/changes/add-quiet-flag/design.md", "# D\n"),
            (
                "_codev/changes/add-quiet-flag/proposal.md",
                "See `web/src/settings/`, `README.md`, `design.md#D`, `web/src/settings/Page.tsx:3-4`.\n",
            ),
        ]);
        assert_eq!(lint(DIR, &files, &[]), Vec::<String>::new());
    }

    #[test]
    fn what_is_not_a_path_is_not_checked() {
        let files = fixture(&[(
            "_codev/changes/add-quiet-flag/proposal.md",
            "`text/csv`, `GET /users/{id}`, `https://x.io/a.md`, `--out`, `src/*.rs`, \
             `Progress::new(quiet)`, `ledger import --quiet data.csv`, `v1.2.3`\n\
             ```\nsrc/missing.rs `src/fenced.rs`\n```\n",
        )]);
        assert_eq!(lint(DIR, &files, &[]), Vec::<String>::new());
    }

    #[test]
    fn a_path_inside_a_command_is_checked() {
        let files = fixture(&[(
            "_codev/changes/add-quiet-flag/tasks.md",
            "- [ ] 1.1 Run `pytest tests/test_signup.py`\n",
        )]);
        assert_eq!(
            lint(DIR, &files, &[]),
            ["`tasks.md` cites `tests/test_signup.py`, which the fixture does not have"]
        );
    }

    #[test]
    fn tests_cited_by_tasks_must_be_in_a_test_file() {
        let tasks = "- [x] 1.1 Flag, verified by tests `a_quiet_import_prints_nothing` and \
                     `cargo test docs_mention_every_flag`\n- [x] 1.2 `npm test -- pricing`\n";
        let files = fixture(&[
            (
                "src/import.rs",
                "#[test]\nfn a_quiet_import_prints_nothing() {}\n",
            ),
            ("src/pricing.ts", "export const pricing = 1;\n"),
            ("_codev/changes/add-quiet-flag/tasks.md", tasks),
        ]);
        assert_eq!(
            lint(DIR, &files, &[]),
            [
                "`tasks.md` cites the test `docs_mention_every_flag`, which no test file of the fixture holds",
                "`tasks.md` cites the test `pricing`, which no test file of the fixture holds",
            ]
        );
        let files = fixture(&[
            (
                "src/import.rs",
                "#[test]\nfn a_quiet_import_prints_nothing() {}\n",
            ),
            ("tests/docs.rs", "fn docs_mention_every_flag() {}\n"),
            ("src/pricing.test.ts", "describe('pricing', () => {});\n"),
            ("_codev/changes/add-quiet-flag/tasks.md", tasks),
        ]);
        assert_eq!(lint(DIR, &files, &[]), Vec::<String>::new());
    }

    #[test]
    fn a_snake_case_span_is_a_test_only_when_introduced_as_one_in_tasks() {
        let files = fixture(&[
            (
                "_codev/changes/add-quiet-flag/tasks.md",
                "- [ ] 1.1 Rename `customer_id` in the payload, verified by `cargo test -p orders-api --test api`\n\
                 - [ ] 1.2 `fetch_rates`, verified by tests `a_b` (\"A b\") and `c_d`, then `e_f`\n",
            ),
            ("tests/api.rs", "#[test]\nfn a_b() {}\nfn c_d() {}\n"),
            (
                "_codev/changes/add-quiet-flag/design.md",
                "Verified by the test `some_test_name`.\n",
            ),
        ]);
        assert_eq!(lint(DIR, &files, &[]), Vec::<String>::new());
    }

    #[test]
    fn planned_entries_are_accepted_and_checked() {
        let files = fixture(&[
            ("src/lib.rs", ""),
            (
                "_codev/changes/add-quiet-flag/tasks.md",
                "- [ ] 1.1 `src/export/csv.rs`, verified by the test `exports_one_line_per_invoice`\n",
            ),
        ]);
        let planned = [
            "src/export/csv.rs".to_string(),
            "exports_one_line_per_invoice".to_string(),
        ];
        assert_eq!(lint(DIR, &files, &planned), Vec::<String>::new());
        let stale = ["src/lib.rs".to_string(), "src/unused.rs".to_string()];
        let problems = lint(DIR, &files, &stale);
        assert!(
            problems.contains(
                &"`planned` lists `src/lib.rs`, which the fixture already has".to_string()
            )
        );
        assert!(
            problems
                .contains(&"`planned` lists `src/unused.rs`, which no artifact cites".to_string())
        );
    }

    #[test]
    fn the_lint_reads_windows_paths_and_crlf() {
        let files = fixture(&[
            ("src/db/orders.rs", ""),
            (
                "_codev/changes/add-quiet-flag/proposal.md",
                "## Impact\r\n\r\n- `src\\db\\orders.rs`\r\n- `.\\src\\db\\missing.rs`\r\n",
            ),
        ]);
        assert_eq!(
            lint(DIR, &files, &[]),
            ["`proposal.md` cites `src/db/missing.rs`, which the fixture does not have"]
        );
    }
}
