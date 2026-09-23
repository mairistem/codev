use std::fmt;

use crate::error::{CoreError, Result};

/// Identifiant de change, en kebab-case strict.
///
/// Un chiffre en tête est autorisé : cela permet de préfixer les changes pour
/// les ordonner ou les échelonner (`100-add-billing`, `00001-add-auth`).
///
/// L'identifiant sert de nom de dossier sur trois systèmes de fichiers
/// différents, dont un insensible à la casse (macOS) : d'où le refus des
/// majuscules, qui rendraient `Add-Auth` et `add-auth` indistinguables ici et
/// distincts sur un poste Linux.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ChangeId(String);

impl ChangeId {
    pub fn parse(raw: &str) -> Result<Self> {
        let reason = kebab_violation(raw);
        match reason {
            Some(reason) => Err(CoreError::InvalidChangeId {
                raw: raw.to_string(),
                reason,
            }),
            None => Ok(Self(raw.to_string())),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ChangeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl AsRef<str> for ChangeId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Vrai si `raw` est un identifiant kebab-case acceptable.
pub fn is_kebab_case(raw: &str) -> bool {
    kebab_violation(raw).is_none()
}

/// Retourne la première règle enfreinte, formulée pour être lue par un humain.
///
/// Un booléen ne suffirait pas : « nom invalide » sans dire laquelle des règles
/// a été enfreinte oblige l'utilisateur à deviner.
fn kebab_violation(raw: &str) -> Option<String> {
    if raw.is_empty() {
        return Some("il est vide".into());
    }
    if raw.starts_with('-') || raw.ends_with('-') {
        return Some("il commence ou finit par un tiret".into());
    }
    if raw.contains("--") {
        return Some("il contient deux tirets consécutifs".into());
    }
    if raw.contains(' ') {
        return Some("il contient une espace ; utilise des tirets".into());
    }
    if raw.contains('_') {
        return Some("il contient un souligné ; utilise des tirets".into());
    }
    if raw.chars().any(|c| c.is_ascii_uppercase()) {
        return Some("il contient une majuscule ; tout en minuscules".into());
    }
    if let Some(bad) = raw
        .chars()
        .find(|c| !(c.is_ascii_lowercase() || c.is_ascii_digit() || *c == '-'))
    {
        return Some(format!(
            "il contient le caractère « {bad} » ; seuls les minuscules, les chiffres et les tirets sont admis"
        ));
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepte_le_kebab_case() {
        for ok in [
            "add-auth",
            "fix-bug",
            "a",
            "100-add-feature",
            "00001-add-auth",
            "v2-migration",
        ] {
            assert!(ChangeId::parse(ok).is_ok(), "{ok} devrait être accepté");
        }
    }

    #[test]
    fn refuse_et_explique() {
        let cases = [
            ("", "vide"),
            ("-lead", "tiret"),
            ("trail-", "tiret"),
            ("double--tiret", "consécutifs"),
            ("avec espace", "espace"),
            ("avec_souligne", "souligné"),
            ("AddAuth", "majuscule"),
            ("accentué", "caractère"),
        ];
        for (raw, attendu) in cases {
            let err = ChangeId::parse(raw)
                .err()
                .unwrap_or_else(|| panic!("« {raw} » devrait être refusé"));
            let message = err.to_string();
            assert!(
                message.contains(attendu),
                "« {raw} » : le message « {message} » devrait mentionner « {attendu} »"
            );
        }
    }

    #[test]
    fn le_code_derreur_est_stable() {
        let err = ChangeId::parse("Nope").unwrap_err();
        assert_eq!(err.code(), "invalid_change_id");
    }
}
