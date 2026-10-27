//! Titles to URL slugs.

use unicode_normalization::UnicodeNormalization;

/// The slug of `title`: lowercase ASCII letters and digits, words joined by
/// a single dash. Accents are dropped, not transliterated.
pub fn slugify(title: &str) -> String {
    let ascii: String = title.nfd().filter(char::is_ascii).collect();
    ascii
        .to_lowercase()
        .split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|w| !w.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accents_are_dropped() {
        assert_eq!(slugify("Été à Paris"), "ete-a-paris");
    }

    #[test]
    fn punctuation_joins_words_once() {
        assert_eq!(slugify("Hello,  world!"), "hello-world");
    }
}
