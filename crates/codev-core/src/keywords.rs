//! The structural keywords of codev documents, in every supported language.
//!
//! Each keyword has a stable [`Key`]; each language is one embedded file,
//! `assets/keywords/<code>.yaml`, giving the text of every key — a list
//! when the language has several spellings, the first being the one codev
//! writes. Adding a language is adding a file to [`LANGUAGE_FILES`].
//!
//! Readers ask this module instead of comparing literals, and every reader
//! recognizes every language. English keeps, key by key, the comparison
//! its reader applied before the catalog existed ([`EnRule`]), so that no
//! English document is read differently; the other languages are compared
//! after [`fold`], which forgives case, French accents and the space —
//! breaking or not — that French typography puts before a colon.
//!
//! Which function applies which English rule: [`section_title`] applies
//! `DeltaKind`, `Exact` and `IgnoreAsciiCase`; [`section_line`] `Exact` and
//! `IgnoreAsciiCase`; [`named_heading_at`] `ExactPrefix` and
//! `ExactPrefixThenSpace`; [`label`] `Labelled` and `Exact`; [`word`]
//! `ExactLowerOrUpper` and `IgnoreAsciiCase`; [`is_normative`] `Substring`,
//! in every language. A rule a function does not apply means the key is not
//! read by that function: it matches nothing there.

use std::collections::BTreeMap;
use std::sync::OnceLock;

/// The embedded language files, English first: [`Lang::EN`] is index 0.
const LANGUAGE_FILES: &[(&str, &str)] = &[
    ("en", include_str!("../../../assets/keywords/en.yaml")),
    ("fr", include_str!("../../../assets/keywords/fr.yaml")),
];

/// A language of the catalog.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Lang(usize);

impl Lang {
    /// English, the language codev wrote before the catalog existed and
    /// the fallback when a document's language is unknown.
    pub const EN: Lang = Lang(0);

    /// The ISO 639 code of the language (`en`, `fr`).
    pub fn code(self) -> &'static str {
        LANGUAGE_FILES[self.0].0
    }

    /// Every language of the catalog, English first.
    pub fn all() -> impl Iterator<Item = Lang> {
        (0..LANGUAGE_FILES.len()).map(Lang)
    }

    /// The language a project writes its keywords in, from its
    /// `language:`: the catalog language of the setting's primary subtag
    /// (`fr-CA` → `fr`), compared without regard to case; English when the
    /// catalog does not ship it.
    pub fn for_setting(language: &str) -> Lang {
        let primary = language.split(['-', '_']).next().unwrap_or_default();
        Lang::all()
            .find(|lang| lang.code().eq_ignore_ascii_case(primary.trim()))
            .unwrap_or(Lang::EN)
    }
}

impl Default for Lang {
    fn default() -> Self {
        Lang::EN
    }
}

/// A structural keyword, named as in the language files.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Key {
    SpecTitle,
    SpecPurpose,
    SpecRequirements,
    Requirement,
    Scenario,
    DeltaAdded,
    DeltaModified,
    DeltaRemoved,
    DeltaRenamed,
    RemovedReason,
    RemovedMigration,
    RenamedFrom,
    RenamedTo,
    Normative,
    WhatChanges,
    Domains,
    ReleaseNote,
    None,
    CategoryAdded,
    CategoryChanged,
    CategoryDeprecated,
    CategoryRemoved,
    CategoryFixed,
    CategorySecurity,
    DesignDecisions,
    DesignDecision,
}

impl Key {
    pub const ALL: [Key; 26] = [
        Key::SpecTitle,
        Key::SpecPurpose,
        Key::SpecRequirements,
        Key::Requirement,
        Key::Scenario,
        Key::DeltaAdded,
        Key::DeltaModified,
        Key::DeltaRemoved,
        Key::DeltaRenamed,
        Key::RemovedReason,
        Key::RemovedMigration,
        Key::RenamedFrom,
        Key::RenamedTo,
        Key::Normative,
        Key::WhatChanges,
        Key::Domains,
        Key::ReleaseNote,
        Key::None,
        Key::CategoryAdded,
        Key::CategoryChanged,
        Key::CategoryDeprecated,
        Key::CategoryRemoved,
        Key::CategoryFixed,
        Key::CategorySecurity,
        Key::DesignDecisions,
        Key::DesignDecision,
    ];

    /// Whether a writer picks among the key's texts (`normative`, `none`)
    /// rather than writing its first one.
    pub fn is_choice(self) -> bool {
        matches!(self, Key::Normative | Key::None)
    }

    /// The key as written in the language files.
    pub fn name(self) -> &'static str {
        match self {
            Key::SpecTitle => "spec.title",
            Key::SpecPurpose => "spec.purpose",
            Key::SpecRequirements => "spec.requirements",
            Key::Requirement => "requirement",
            Key::Scenario => "scenario",
            Key::DeltaAdded => "delta.added",
            Key::DeltaModified => "delta.modified",
            Key::DeltaRemoved => "delta.removed",
            Key::DeltaRenamed => "delta.renamed",
            Key::RemovedReason => "removed.reason",
            Key::RemovedMigration => "removed.migration",
            Key::RenamedFrom => "renamed.from",
            Key::RenamedTo => "renamed.to",
            Key::Normative => "normative",
            Key::WhatChanges => "proposal.what_changes",
            Key::Domains => "proposal.domains",
            Key::ReleaseNote => "proposal.release_note",
            Key::None => "none",
            Key::CategoryAdded => "category.added",
            Key::CategoryChanged => "category.changed",
            Key::CategoryDeprecated => "category.deprecated",
            Key::CategoryRemoved => "category.removed",
            Key::CategoryFixed => "category.fixed",
            Key::CategorySecurity => "category.security",
            Key::DesignDecisions => "design.decisions",
            Key::DesignDecision => "design.decision",
        }
    }

    fn from_name(name: &str) -> Option<Key> {
        Key::ALL.into_iter().find(|key| key.name() == name)
    }

    /// The four delta section keys, in the order of the delta operations.
    pub const DELTA: [Key; 4] = [
        Key::DeltaAdded,
        Key::DeltaModified,
        Key::DeltaRemoved,
        Key::DeltaRenamed,
    ];

    /// How the English texts of this key are compared: the rule its reader
    /// applied before the catalog, kept so English is read as before.
    fn en_rule(self) -> EnRule {
        match self {
            Key::Requirement | Key::Scenario => EnRule::ExactPrefix,
            Key::DesignDecision => EnRule::ExactPrefixThenSpace,
            Key::DeltaAdded | Key::DeltaModified | Key::DeltaRemoved | Key::DeltaRenamed => {
                EnRule::DeltaKind
            }
            Key::RenamedFrom | Key::RenamedTo | Key::DesignDecisions => EnRule::Exact,
            Key::RemovedReason | Key::RemovedMigration => EnRule::Labelled,
            Key::None => EnRule::ExactLowerOrUpper,
            Key::Normative => EnRule::Substring,
            Key::SpecTitle
            | Key::SpecPurpose
            | Key::SpecRequirements
            | Key::WhatChanges
            | Key::Domains
            | Key::ReleaseNote
            | Key::CategoryAdded
            | Key::CategoryChanged
            | Key::CategoryDeprecated
            | Key::CategoryRemoved
            | Key::CategoryFixed
            | Key::CategorySecurity => EnRule::IgnoreAsciiCase,
        }
    }
}

/// The comparison applied to an English text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EnRule {
    /// The line starts with the text, and the rest does not open a deeper
    /// heading (`### Requirement:`).
    ExactPrefix,
    /// The line starts with the text followed by a space (`### Decision: `).
    ExactPrefixThenSpace,
    /// Upper-cased, `REQUIREMENTS` suffix stripped, then the operation word
    /// compared (`## ADDED   Requirements` is accepted).
    DeltaKind,
    /// Byte for byte.
    Exact,
    /// `**Reason**: …`, or bare `Reason: …`.
    Labelled,
    /// The text, or its lower or upper case (`None`, `none`, `NONE`).
    ExactLowerOrUpper,
    /// Contained anywhere, case-sensitively (`SHALL`).
    Substring,
    /// Equal, ignoring ASCII case.
    IgnoreAsciiCase,
}

type Texts = BTreeMap<Key, Vec<String>>;

/// One parsed language file, or what is wrong with it.
fn parse_language(source: &str) -> Result<Texts, String> {
    #[derive(serde::Deserialize)]
    #[serde(untagged)]
    enum OneOrMany {
        One(String),
        Many(Vec<String>),
    }
    let raw: BTreeMap<String, OneOrMany> =
        serde_norway::from_str(source).map_err(|e| e.to_string())?;
    let mut texts = Texts::new();
    for (name, value) in raw {
        let key = Key::from_name(&name).ok_or_else(|| format!("unknown key `{name}`"))?;
        let list = match value {
            OneOrMany::One(text) => vec![text],
            OneOrMany::Many(list) => list,
        };
        if list.is_empty() || list.iter().any(|t| t.trim().is_empty()) {
            return Err(format!("key `{name}` has an empty text"));
        }
        texts.insert(key, list);
    }
    if let Some(missing) = Key::ALL.iter().find(|key| !texts.contains_key(key)) {
        return Err(format!("missing key `{}`", missing.name()));
    }
    Ok(texts)
}

fn catalog() -> &'static [Texts] {
    static CATALOG: OnceLock<Vec<Texts>> = OnceLock::new();
    CATALOG.get_or_init(|| {
        // The files are embedded: a broken one is a build bug, which the
        // test `every_shipped_language_defines_every_key` catches before
        // any release.
        LANGUAGE_FILES
            .iter()
            .map(|(code, source)| {
                parse_language(source)
                    .unwrap_or_else(|e| panic!("assets/keywords/{code}.yaml: {e}"))
            })
            .collect()
    })
}

/// Every text of `key` in `lang`, the written form first.
pub fn texts(key: Key, lang: Lang) -> &'static [String] {
    &catalog()[lang.0][&key]
}

/// The form codev writes for `key` in `lang`.
pub fn text(key: Key, lang: Lang) -> &'static str {
    &texts(key, lang)[0]
}

/// What codev writes for every key in `lang`, in [`Key::ALL`] order: the
/// written form, or every text for the keys a writer picks among
/// (`normative`, `none`).
pub fn written(lang: Lang) -> Vec<(Key, Vec<&'static str>)> {
    Key::ALL
        .into_iter()
        .map(|key| {
            let all = texts(key, lang);
            let shown: Vec<&'static str> = if key.is_choice() {
                all.iter().map(String::as_str).collect()
            } else {
                vec![all[0].as_str()]
            };
            (key, shown)
        })
        .collect()
}

/// The language of the structural keyword a heading line holds — a
/// section, a delta section or a named heading — or `None` when it holds
/// none.
pub fn heading_language(line: &str) -> Option<Lang> {
    if !line.starts_with('#') {
        return None;
    }
    if let Some((_, lang)) = line.strip_prefix("## ").and_then(|t| delta_title(t.trim())) {
        return Some(lang);
    }
    Key::ALL.into_iter().find_map(|key| {
        section_line(line, key).or_else(|| named_heading(line, key).map(|(lang, _)| lang))
    })
}

/// The language of the first structural keyword heading `text` holds, or
/// `None` when it holds none.
pub fn text_language(text: &str) -> Option<Lang> {
    text.lines().find_map(heading_language)
}

/// Lower case, French accents folded to their base letter, runs of
/// whitespace — no-break spaces included — collapsed to one space, and no
/// space before a colon.
pub fn fold(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut pending_space = false;
    for c in s.chars().flat_map(char::to_lowercase) {
        if c.is_whitespace() {
            pending_space = true;
            continue;
        }
        if c == ':' {
            pending_space = false;
            out.push(':');
            continue;
        }
        if pending_space && !out.is_empty() {
            out.push(' ');
        }
        pending_space = false;
        match c {
            'à' | 'â' | 'ä' => out.push('a'),
            'ç' => out.push('c'),
            'é' | 'è' | 'ê' | 'ë' => out.push('e'),
            'î' | 'ï' => out.push('i'),
            'ô' | 'ö' => out.push('o'),
            'ù' | 'û' | 'ü' => out.push('u'),
            'ÿ' => out.push('y'),
            'œ' => out.push_str("oe"),
            'æ' => out.push_str("ae"),
            _ => out.push(c),
        }
    }
    out
}

/// The heading level of a text (`### Requirement:` → 3), 0 for a word.
fn level(text: &str) -> usize {
    text.chars().take_while(|c| *c == '#').count()
}

/// A text without its heading marks, emphasis and final colon:
/// `### Exigence :` → `Exigence`, `**Raison**` → `Raison`.
fn bare(text: &str) -> &str {
    text.trim_start_matches('#')
        .trim()
        .trim_matches('*')
        .trim_end_matches(':')
        .trim_end()
        .trim_matches('*')
        .trim()
}

/// The language of a level-2 section title (what follows `## `), when it is
/// the title of `key`.
pub fn section_title(title: &str, key: Key) -> Option<Lang> {
    let title = title.trim();
    Lang::all().find(|&lang| {
        texts(key, lang).iter().any(|text| {
            let expected = text.trim_start_matches('#').trim();
            if lang == Lang::EN {
                match key.en_rule() {
                    EnRule::DeltaKind => delta_word(title)
                        .is_some_and(|w| delta_word(expected).is_some_and(|e| w == e)),
                    EnRule::Exact => title == expected,
                    EnRule::IgnoreAsciiCase => title.eq_ignore_ascii_case(expected),
                    EnRule::ExactPrefix
                    | EnRule::ExactPrefixThenSpace
                    | EnRule::Labelled
                    | EnRule::ExactLowerOrUpper
                    | EnRule::Substring => false,
                }
            } else {
                fold(title) == fold(expected)
            }
        })
    })
}

/// `ADDED   Requirements` → `ADDED`: how English delta titles were read.
fn delta_word(title: &str) -> Option<String> {
    let upper = title.to_ascii_uppercase();
    let word = upper.strip_suffix("REQUIREMENTS")?.trim_end();
    Some(word.to_string())
}

/// The delta operation a level-2 title names, with its language.
pub fn delta_title(title: &str) -> Option<(Key, Lang)> {
    Key::DELTA
        .into_iter()
        .find_map(|key| section_title(title, key).map(|lang| (key, lang)))
}

/// The language of a whole heading line (`## Release note`), when it is the
/// heading of `key`. Leading whitespace is not a heading.
pub fn section_line(line: &str, key: Key) -> Option<Lang> {
    let line = line.trim_end_matches(['\r', '\n']);
    let hashes = "#".repeat(level(text(key, Lang::EN)));
    let title = line.strip_prefix(&hashes)?.strip_prefix(' ')?;
    if title.starts_with('#') {
        return None;
    }
    Lang::all().find(|&lang| {
        texts(key, lang).iter().any(|text| {
            let expected = text.trim_start_matches('#').trim();
            if lang == Lang::EN {
                match key.en_rule() {
                    // `## Decisions` was matched byte for byte.
                    EnRule::Exact => line == text,
                    // Case-insensitively, trailing spaces ignored, as
                    // `parser::section` always did.
                    EnRule::IgnoreAsciiCase => title.trim_end().eq_ignore_ascii_case(expected),
                    EnRule::ExactPrefix
                    | EnRule::ExactPrefixThenSpace
                    | EnRule::DeltaKind
                    | EnRule::Labelled
                    | EnRule::ExactLowerOrUpper
                    | EnRule::Substring => false,
                }
            } else {
                fold(title) == fold(expected)
            }
        })
    })
}

/// A heading that carries a name — `### Requirement: <name>` — at the level
/// of `key`'s text: its language and the trimmed name.
pub fn named_heading(line: &str, key: Key) -> Option<(Lang, String)> {
    named_heading_at(line, key, level(text(key, Lang::EN)))
}

/// Like [`named_heading`], at another level: `### Scenario:` is the trap of
/// a scenario written one level too high.
pub fn named_heading_at(line: &str, key: Key, at: usize) -> Option<(Lang, String)> {
    let trimmed = line.trim_start();
    Lang::all().find_map(|lang| {
        texts(key, lang).iter().find_map(|text| {
            let word = text.trim_start_matches('#').trim_start();
            if lang == Lang::EN {
                let prefix = format!("{} {word}", "#".repeat(at));
                let after = trimmed.strip_prefix(&prefix)?;
                match key.en_rule() {
                    EnRule::ExactPrefixThenSpace => {
                        Some((lang, after.strip_prefix(' ')?.trim().to_string()))
                    }
                    // A `#` right after is a deeper heading, not this one.
                    EnRule::ExactPrefix if after.starts_with('#') => None,
                    EnRule::ExactPrefix => Some((lang, after.trim().to_string())),
                    EnRule::DeltaKind
                    | EnRule::Exact
                    | EnRule::Labelled
                    | EnRule::ExactLowerOrUpper
                    | EnRule::Substring
                    | EnRule::IgnoreAsciiCase => None,
                }
            } else {
                let rest = trimmed.strip_prefix(&"#".repeat(at))?.strip_prefix(' ')?;
                if rest.starts_with('#') {
                    return None;
                }
                let colon = rest.find(':')?;
                (fold(&rest[..colon]) == fold(bare(text)))
                    .then(|| (lang, rest[colon + 1..].trim().to_string()))
            }
        })
    })
}

/// A labelled line — `**Reason**: …`, `FROM: …` — at its start: its language
/// and the trimmed value after the colon.
pub fn label(line: &str, key: Key) -> Option<(Lang, String)> {
    let trimmed = line.trim_start();
    Lang::all().find_map(|lang| {
        texts(key, lang).iter().find_map(|text| {
            if lang == Lang::EN {
                let candidates = match key.en_rule() {
                    // `**Reason**` was also accepted bare (`Reason:`).
                    EnRule::Labelled => vec![format!("{text}:"), format!("{}:", bare(text))],
                    // `FROM:` only as written.
                    EnRule::Exact => vec![text.clone()],
                    EnRule::ExactPrefix
                    | EnRule::ExactPrefixThenSpace
                    | EnRule::DeltaKind
                    | EnRule::ExactLowerOrUpper
                    | EnRule::Substring
                    | EnRule::IgnoreAsciiCase => Vec::new(),
                };
                candidates
                    .iter()
                    .find_map(|c| trimmed.strip_prefix(c.as_str()))
                    .map(|rest| (lang, rest.trim().to_string()))
            } else {
                let colon = trimmed.find(':')?;
                let head = trimmed[..colon].trim().trim_matches('*').trim();
                (fold(head) == fold(bare(text)))
                    .then(|| (lang, trimmed[colon + 1..].trim().to_string()))
            }
        })
    })
}

/// The language of a word — a release note category, `None` — when it is
/// one of `key`'s texts.
pub fn word(s: &str, key: Key) -> Option<Lang> {
    Lang::all().find(|&lang| {
        texts(key, lang).iter().any(|text| {
            if lang == Lang::EN {
                match key.en_rule() {
                    EnRule::ExactLowerOrUpper => {
                        s == text || s == text.to_lowercase() || s == text.to_uppercase()
                    }
                    EnRule::IgnoreAsciiCase => s.eq_ignore_ascii_case(text),
                    EnRule::ExactPrefix
                    | EnRule::ExactPrefixThenSpace
                    | EnRule::DeltaKind
                    | EnRule::Exact
                    | EnRule::Labelled
                    | EnRule::Substring => false,
                }
            } else {
                fold(s) == fold(text)
            }
        })
    })
}

/// Whether a requirement's description holds a normative keyword of any
/// language — the `Substring` rule, applied to every language because
/// `doit` in lower case is ordinary prose.
pub fn is_normative(description: &str) -> bool {
    normative_words().any(|w| description.contains(w))
}

/// The normative keywords of every language, English first.
pub fn normative_words() -> impl Iterator<Item = &'static str> {
    Lang::all().flat_map(|lang| texts(Key::Normative, lang).iter().map(String::as_str))
}

/// The written requirement heading of every language, for messages:
/// `` `### Requirement: <name>` or `### Exigence : <name>` ``.
pub fn requirement_headings() -> String {
    Lang::all()
        .map(|lang| format!("`{} <name>`", text(Key::Requirement, lang)))
        .collect::<Vec<_>>()
        .join(" or ")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fr() -> Lang {
        Lang::all().find(|l| l.code() == "fr").unwrap()
    }

    #[test]
    fn every_shipped_language_defines_every_key() {
        for (code, source) in LANGUAGE_FILES {
            parse_language(source).unwrap_or_else(|e| panic!("{code}: {e}"));
        }
        assert_eq!(Lang::EN.code(), "en");
        assert_eq!(text(Key::Requirement, fr()), "### Exigence :");
    }

    #[test]
    fn an_unknown_key_in_a_language_file_is_rejected() {
        let err = parse_language("requirment: \"### Requirement:\"\n").unwrap_err();
        assert!(err.contains("unknown key `requirment`"), "{err}");
        let err = parse_language("requirement: \"### Requirement:\"\n").unwrap_err();
        assert!(err.contains("missing key"), "{err}");
    }

    #[test]
    fn normalized_texts_never_collide_at_the_same_level() {
        let mut seen: BTreeMap<(usize, String), Key> = BTreeMap::new();
        for lang in Lang::all() {
            for key in Key::ALL {
                if matches!(key, Key::Normative | Key::SpecTitle) {
                    continue;
                }
                for t in texts(key, lang) {
                    let id = (level(t), fold(bare(t)));
                    if let Some(other) = seen.insert(id.clone(), key) {
                        assert_eq!(
                            other,
                            key,
                            "`{t}` ({}) collides with {other:?}",
                            lang.code()
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn a_prefix_matched_text_never_prefixes_another_key() {
        let prefixed = [Key::Requirement, Key::Scenario, Key::DesignDecision];
        for lang in Lang::all() {
            for key in prefixed {
                let head = fold(bare(text(key, lang)));
                for other in Key::ALL.into_iter().filter(|k| *k != key) {
                    for other_lang in Lang::all() {
                        for t in texts(other, other_lang) {
                            if level(t) == level(text(key, lang)) {
                                assert!(
                                    !fold(bare(t)).starts_with(&head),
                                    "`{t}` starts with the {key:?} keyword `{head}`"
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn every_catalog_text_folds_to_ascii() {
        for lang in Lang::all() {
            for key in Key::ALL {
                for t in texts(key, lang) {
                    assert!(fold(t).is_ascii(), "`{t}` folds to `{}`", fold(t));
                }
            }
        }
    }

    #[test]
    fn french_headings_match_ignoring_case_accents_and_space_before_colon() {
        assert_eq!(
            named_heading("### Exigence: Annulation", Key::Requirement),
            Some((fr(), "Annulation".to_string()))
        );
        assert_eq!(
            named_heading("#### Scenario : Dans les temps", Key::Scenario),
            Some((fr(), "Dans les temps".to_string()))
        );
        assert_eq!(
            delta_title("exigences ajoutees"),
            Some((Key::DeltaAdded, fr()))
        );
        assert_eq!(
            delta_title("Exigences  RENOMMÉES"),
            Some((Key::DeltaRenamed, fr()))
        );
        assert_eq!(
            section_title("Exigences AJOUTÉES", Key::SpecRequirements),
            None
        );
    }

    #[test]
    fn a_no_break_space_before_the_colon_is_accepted() {
        for space in ['\u{a0}', '\u{202f}'] {
            let line = format!("### Exigence{space}: Annulation");
            assert_eq!(
                named_heading(&line, Key::Requirement),
                Some((fr(), "Annulation".to_string()))
            );
        }
        assert_eq!(
            label("DE\u{a0}: Ancien", Key::RenamedFrom),
            Some((fr(), "Ancien".to_string()))
        );
    }

    #[test]
    fn english_texts_keep_their_current_comparison() {
        // Exact and case-sensitive, as `requirement_header` always was.
        assert_eq!(named_heading("### requirement: x", Key::Requirement), None);
        // Only the French tolerance reads this one, as the French keyword.
        assert_eq!(
            named_heading("#### Scenario : y", Key::Scenario).map(|(l, _)| l.code()),
            Some("fr")
        );
        assert_eq!(
            named_heading("### Requirement:x", Key::Requirement),
            Some((Lang::EN, "x".to_string()))
        );
        assert_eq!(named_heading("#### Requirement: x", Key::Requirement), None);
        // The delta operation word, extra spaces and case forgiven.
        assert_eq!(
            delta_title("ADDED   Requirements"),
            Some((Key::DeltaAdded, Lang::EN))
        );
        assert_eq!(
            delta_title("added requirements"),
            Some((Key::DeltaAdded, Lang::EN))
        );
        // `### Decision: ` needs its space in English; without it, only the
        // French tolerance reads the line.
        assert_eq!(
            named_heading("### Decision:X", Key::DesignDecision).map(|(l, _)| l.code()),
            Some("fr")
        );
        assert_eq!(
            named_heading("### Decision: X", Key::DesignDecision),
            Some((Lang::EN, "X".to_string()))
        );
        assert_eq!(label("from: x", Key::RenamedFrom), None);
        assert_eq!(
            label("Reason: gone", Key::RemovedReason),
            Some((Lang::EN, "gone".to_string()))
        );
        assert_eq!(word("NoNe", Key::None), None);
        assert_eq!(word("none", Key::None), Some(Lang::EN));
    }

    #[test]
    fn lower_case_doit_is_not_normative() {
        assert!(!is_normative("Le portail doit afficher la date."));
        assert!(is_normative("Le portail DOIT afficher la date."));
        assert!(is_normative("Les portails DOIVENT afficher la date."));
        assert!(is_normative("The system SHALL x."));
    }

    #[test]
    fn section_lines_are_whole_lines() {
        assert_eq!(section_line("## Domaines", Key::Domains), Some(fr()));
        assert_eq!(
            section_line("## release NOTE  ", Key::ReleaseNote),
            Some(Lang::EN)
        );
        assert_eq!(section_line("## Domaines du change", Key::Domains), None);
        assert_eq!(section_line(" ## Domains", Key::Domains), None);
        assert_eq!(section_line("### Domains", Key::Domains), None);
        assert_eq!(
            section_line("## Decisions", Key::DesignDecisions),
            Some(Lang::EN)
        );
        // Not English any more (exact match), but the French tolerance reads it.
        assert_eq!(
            section_line("## decisions", Key::DesignDecisions),
            Some(fr())
        );
        assert_eq!(
            section_line("## décisions", Key::DesignDecisions),
            Some(fr())
        );
    }

    #[test]
    fn keyword_language_of_a_shipped_language() {
        assert_eq!(Lang::for_setting("fr"), fr());
        assert_eq!(Lang::for_setting("en"), Lang::EN);
    }

    #[test]
    fn keyword_language_of_a_regional_variant() {
        assert_eq!(Lang::for_setting("fr-CA"), fr());
    }

    #[test]
    fn keyword_language_ignores_case() {
        assert_eq!(Lang::for_setting("FR"), fr());
        assert_eq!(Lang::for_setting("Fr-ca"), fr());
    }

    #[test]
    fn keyword_language_of_an_unshipped_language_is_english() {
        assert_eq!(Lang::for_setting("pt-BR"), Lang::EN);
    }

    #[test]
    fn keyword_language_without_a_setting_is_english() {
        assert_eq!(Lang::for_setting(""), Lang::EN);
    }

    #[test]
    fn every_key_is_written_and_the_choices_keep_all_their_texts() {
        let written = written(fr());
        assert_eq!(written.len(), Key::ALL.len());
        let of = |key: Key| written.iter().find(|(k, _)| *k == key).unwrap().1.clone();
        assert_eq!(of(Key::Requirement), ["### Exigence :"]);
        assert_eq!(of(Key::Normative), ["DOIT", "DOIVENT"]);
        assert_eq!(of(Key::None), ["Aucun", "Aucune"]);
    }
}
