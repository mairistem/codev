//! Ranking documents against a query, for `codev search`: words lower-cased,
//! accents folded, common words removed, stems taken in the language the
//! specs are written in, then BM25. Pure — the engine reads the documents
//! and hands their text in — and deterministic: the same documents and
//! query always rank the same way, ties ordered by identifier.

use std::collections::{BTreeMap, BTreeSet};

use rust_stemmers::{Algorithm, Stemmer};

use crate::keywords::Lang;
use crate::parser::{Requirement, Spec};

/// BM25's saturation of a word's frequency in a document.
const K1: f64 = 1.2;
/// BM25's normalization by the document's length.
const B: f64 = 0.75;

/// A document to rank: its identifier and its whole text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Document {
    pub id: String,
    pub text: String,
}

/// A ranked document, the most relevant first.
#[derive(Debug, Clone, PartialEq)]
pub struct Ranked {
    pub id: String,
    pub score: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SearchError {
    /// Nothing is left of the query once its common words are removed.
    #[error("the query holds no word to search: only common words, or none")]
    EmptyQuery,
}

/// Words without meaning for a search, in the languages specs are written
/// in. Both lists apply whatever the language, so that a query mixing
/// languages loses its filler words too. Already folded: no accents.
const COMMON_WORDS: &[&str] = &[
    // English
    "a", "about", "after", "all", "also", "an", "and", "any", "are", "as", "at", "be", "been",
    "before", "but", "by", "can", "could", "do", "does", "each", "for", "from", "had", "has",
    "have", "how", "i", "if", "in", "into", "is", "it", "its", "may", "more", "must", "no", "not",
    "of", "on", "one", "or", "other", "our", "shall", "should", "so", "some", "such", "than",
    "that", "the", "their", "them", "then", "there", "these", "they", "this", "those", "to", "too",
    "under", "up", "was", "we", "were", "what", "when", "where", "which", "while", "who", "will",
    "with", "would", "you", "your", // French
    "au", "aux", "avec", "ce", "ces", "cet", "cette", "dans", "de", "des", "doit", "du", "elle",
    "en", "est", "et", "il", "ils", "je", "la", "le", "les", "leur", "leurs", "lui", "mais", "me",
    "mes", "mon", "ne", "nous", "on", "ou", "par", "pas", "pour", "qu", "que", "qui", "sa", "se",
    "ses", "son", "sont", "sur", "ta", "te", "tes", "ton", "tu", "un", "une", "vos", "votre",
    "vous", "y",
];

/// The Snowball algorithm of a catalog language; English for a language
/// Snowball does not know.
fn algorithm(lang: Lang) -> Algorithm {
    match lang.code() {
        "fr" => Algorithm::French,
        "es" => Algorithm::Spanish,
        "de" => Algorithm::German,
        "it" => Algorithm::Italian,
        "pt" => Algorithm::Portuguese,
        _ => Algorithm::English,
    }
}

/// A Latin letter without its accent: `é` → `e`, `ç` → `c`, `œ` → `oe`.
fn fold(c: char, out: &mut String) {
    let folded = match c {
        'à' | 'á' | 'â' | 'ã' | 'ä' | 'å' => "a",
        'ç' => "c",
        'è' | 'é' | 'ê' | 'ë' => "e",
        'ì' | 'í' | 'î' | 'ï' => "i",
        'ñ' => "n",
        'ò' | 'ó' | 'ô' | 'õ' | 'ö' => "o",
        'ù' | 'ú' | 'û' | 'ü' => "u",
        'ý' | 'ÿ' => "y",
        'œ' => "oe",
        'æ' => "ae",
        'ß' => "ss",
        _ => {
            out.push(c);
            return;
        }
    };
    out.push_str(folded);
}

/// The words of a text, as the index compares them: lower-cased, accents
/// folded, split on anything that is neither a letter nor a digit, common
/// words removed, each word reduced to its stem.
pub fn terms(text: &str, lang: Lang) -> Vec<String> {
    let stemmer = Stemmer::create(algorithm(lang));
    let mut folded = String::with_capacity(text.len());
    for c in text.chars().flat_map(char::to_lowercase) {
        fold(c, &mut folded);
    }
    folded
        .split(|c: char| !c.is_alphanumeric())
        .filter(|w| !w.is_empty() && !COMMON_WORDS.contains(w))
        .map(|w| stemmer.stem(w).into_owned())
        .collect()
}

/// Ranks `documents` against `query` with BM25, in `lang`: every document
/// holding at least one term of the query, the most relevant first, ties
/// ordered by identifier. A query with no term left after its common words
/// are removed is an error.
pub fn rank(documents: &[Document], query: &str, lang: Lang) -> Result<Vec<Ranked>, SearchError> {
    let query_terms: BTreeSet<String> = terms(query, lang).into_iter().collect();
    if query_terms.is_empty() {
        return Err(SearchError::EmptyQuery);
    }
    // Each document's term frequencies, for the query's terms only, and
    // its length in terms.
    let indexed: Vec<(BTreeMap<&str, f64>, f64)> = documents
        .iter()
        .map(|d| {
            let words = terms(&d.text, lang);
            let mut frequencies: BTreeMap<&str, f64> = BTreeMap::new();
            for word in &words {
                if let Some(term) = query_terms.get(word) {
                    *frequencies.entry(term.as_str()).or_default() += 1.0;
                }
            }
            (frequencies, words.len() as f64)
        })
        .collect();
    let count = documents.len() as f64;
    let average_length = if documents.is_empty() {
        0.0
    } else {
        indexed.iter().map(|(_, len)| len).sum::<f64>() / count
    };
    // A rare term weighs more than a frequent one.
    let idf: BTreeMap<&str, f64> = query_terms
        .iter()
        .map(|term| {
            let holding = indexed
                .iter()
                .filter(|(f, _)| f.contains_key(term.as_str()))
                .count() as f64;
            let weight = ((count - holding + 0.5) / (holding + 0.5) + 1.0).ln();
            (term.as_str(), weight)
        })
        .collect();
    let mut ranked: Vec<Ranked> = documents
        .iter()
        .zip(&indexed)
        .filter(|(_, (frequencies, _))| !frequencies.is_empty())
        .map(|(document, (frequencies, length))| {
            let norm = if average_length > 0.0 {
                K1 * (1.0 - B + B * length / average_length)
            } else {
                K1
            };
            let score = frequencies
                .iter()
                .map(|(term, f)| idf[term] * f * (K1 + 1.0) / (f + norm))
                .sum();
            Ranked {
                id: document.id.clone(),
                score,
            }
        })
        .collect();
    ranked.sort_by(|a, b| b.score.total_cmp(&a.score).then_with(|| a.id.cmp(&b.id)));
    Ok(ranked)
}

/// A requirement as one document: its name, its description and its
/// scenarios, identified as `<capability>::<name>`.
pub fn requirement_document(capability: &str, requirement: &Requirement) -> Document {
    let mut text = format!("{}\n{}\n", requirement.name, requirement.description);
    for scenario in &requirement.scenarios {
        text.push_str(&scenario.name);
        text.push('\n');
        text.push_str(&scenario.body);
        text.push('\n');
    }
    Document {
        id: format!("{capability}::{}", requirement.name),
        text,
    }
}

/// A heading compared ignoring case and runs of whitespace.
fn normalize(name: &str) -> String {
    name.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

/// The requirement of `spec` named `name`, compared ignoring case and runs
/// of whitespace.
pub fn find_requirement<'a>(spec: &'a Spec, name: &str) -> Option<&'a Requirement> {
    let wanted = normalize(name);
    spec.requirements
        .iter()
        .find(|r| normalize(&r.name) == wanted)
}

/// A requirement's text as its spec holds it — heading, description and
/// scenarios, byte for byte — without the blank lines that separate it from
/// the next block.
pub fn requirement_source<'a>(source: &'a str, requirement: &Requirement) -> &'a str {
    let range = &requirement.span.byte_range;
    source
        .get(range.start..range.end.min(source.len()))
        .unwrap_or("")
        .trim_end()
}

/// The language to stem a corpus in: the one every spec is written in,
/// English when they mix several or there is none.
pub fn corpus_lang(specs: impl IntoIterator<Item = Lang>) -> Lang {
    let mut langs = specs.into_iter();
    match langs.next() {
        Some(first) if langs.all(|l| l == first) => first,
        _ => Lang::EN,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fr() -> Lang {
        Lang::all()
            .find(|l| l.code() == "fr")
            .expect("French is in the catalog")
    }

    fn docs(items: &[(&str, &str)]) -> Vec<Document> {
        items
            .iter()
            .map(|(id, text)| Document {
                id: id.to_string(),
                text: text.to_string(),
            })
            .collect()
    }

    fn ids(ranked: &[Ranked]) -> Vec<&str> {
        ranked.iter().map(|r| r.id.as_str()).collect()
    }

    #[test]
    fn a_rare_word_counts_more_than_a_frequent_one() {
        // `change` is in every document, `seal` in one: a query holding
        // both ranks the sealed one first.
        let documents = docs(&[
            ("a", "A change holds its files."),
            ("b", "A change is archived."),
            ("c", "A change records the seal of a decision."),
            ("d", "A change has a name."),
        ]);
        let ranked = rank(&documents, "change seal", Lang::EN).unwrap();
        assert_eq!(ranked[0].id, "c");
        assert_eq!(ranked.len(), 4);
    }

    #[test]
    fn a_plural_finds_its_singular() {
        let documents = docs(&[
            ("decision", "Creating a decision through the command line."),
            ("other", "Listing the specs."),
        ]);
        let ranked = rank(&documents, "decisions", Lang::EN).unwrap();
        assert_eq!(ids(&ranked), ["decision"]);
    }

    #[test]
    fn french_words_match_without_accents() {
        let documents = docs(&[
            ("acceptee", "Une décision acceptée est scellée."),
            ("autre", "La liste des spécifications."),
        ]);
        let ranked = rank(&documents, "decisions acceptees", fr()).unwrap();
        assert_eq!(ids(&ranked), ["acceptee"]);
    }

    #[test]
    fn ties_are_ordered_by_identifier() {
        let documents = docs(&[("b", "seal"), ("c", "seal"), ("a", "seal")]);
        let ranked = rank(&documents, "seal", Lang::EN).unwrap();
        assert_eq!(ids(&ranked), ["a", "b", "c"]);
    }

    #[test]
    fn a_query_of_common_words_is_empty() {
        let documents = docs(&[("a", "the of")]);
        assert_eq!(
            rank(&documents, "the of", Lang::EN),
            Err(SearchError::EmptyQuery)
        );
        assert_eq!(
            rank(&documents, "  ", Lang::EN),
            Err(SearchError::EmptyQuery)
        );
        assert_eq!(
            rank(&documents, "le de la", fr()),
            Err(SearchError::EmptyQuery)
        );
    }

    #[test]
    fn the_same_input_ranks_the_same_way() {
        let documents = docs(&[
            ("x", "The review record keeps every finding."),
            ("y", "A finding has a severity and a confidence."),
            ("z", "The skeptic reviews the plan."),
        ]);
        let first = rank(&documents, "review finding severity", Lang::EN).unwrap();
        let second = rank(&documents, "review finding severity", Lang::EN).unwrap();
        assert_eq!(first, second);
        assert!(rank(&documents, "xylophone", Lang::EN).unwrap().is_empty());
    }

    const SPEC: &str = "# Decisions\n\n## Purpose\n\nRecord decisions.\n\n## Requirements\n\n\
### Requirement: Creating a decision through the CLI\n\nThe CLI SHALL create one.\n\n\
#### Scenario: Create\n\n- **WHEN** asked\n- **THEN** it creates\n\n\
#### Scenario: Refuse\n\n- **WHEN** the name is empty\n- **THEN** it refuses\n\n\
### Requirement: Listing decisions\n\nThe CLI SHALL list them.\n\n\
#### Scenario: List\n\n- **WHEN** asked\n- **THEN** it lists\n";

    #[test]
    fn a_requirement_is_sliced_verbatim() {
        let spec = crate::parser::parse_spec(SPEC).value;
        let requirement = find_requirement(&spec, "Creating a decision through the CLI").unwrap();
        let text = requirement_source(SPEC, requirement);
        assert!(text.starts_with("### Requirement: Creating a decision through the CLI\n"));
        assert!(text.ends_with("- **THEN** it refuses"), "{text}");
        assert!(!text.contains("Listing decisions"));
        let last = find_requirement(&spec, "Listing decisions").unwrap();
        assert!(requirement_source(SPEC, last).ends_with("- **THEN** it lists"));
        let document = requirement_document("decisions", requirement);
        assert_eq!(
            document.id,
            "decisions::Creating a decision through the CLI"
        );
        assert!(document.text.contains("the name is empty"));
    }

    #[test]
    fn a_requirement_name_matches_ignoring_case_and_spaces() {
        let spec = crate::parser::parse_spec(SPEC).value;
        let found = find_requirement(&spec, "creating a decision  through the cli").unwrap();
        assert_eq!(found.name, "Creating a decision through the CLI");
        assert!(find_requirement(&spec, "Deleting a decision").is_none());
    }

    #[test]
    fn mixed_specs_are_stemmed_in_english() {
        assert_eq!(corpus_lang([Lang::EN, fr()]), Lang::EN);
        assert_eq!(corpus_lang([]), Lang::EN);
        assert_eq!(corpus_lang([Lang::EN, Lang::EN]), Lang::EN);
    }

    #[test]
    fn french_specs_are_stemmed_in_french() {
        let source = "# Décisions\n\n## Exigences\n\n### Exigence : Une décision acceptée est scellée\n\n\
Le système DOIT la sceller.\n\n#### Scénario : Sceller\n\n- **QUAND** on accepte\n- **ALORS** il scelle\n";
        let spec = crate::parser::parse_spec(source).value;
        assert_eq!(spec.lang, fr());
        assert_eq!(corpus_lang([spec.lang, spec.lang]), fr());
    }
}
