//! Reconnaissance de la licence depuis le fichier `LICENSE` racine.
//!
//! Cinq licences courantes sont matchées par regex sur le premier
//! kilo-octet du fichier — ça suffit largement à voir « MIT License »,
//! « Apache License, Version 2.0 », etc. Aucune tentative d'analyse fine.

pub fn identify(bytes: &[u8]) -> Option<String> {
    let sample = std::str::from_utf8(bytes).ok()?;
    let sample = &sample[..sample.len().min(1024)].to_ascii_lowercase();

    if sample.contains("apache license") && sample.contains("version 2.0") {
        return Some("Apache-2.0".to_string());
    }
    if sample.contains("mit license") || sample.contains("permission is hereby granted, free of charge") {
        return Some("MIT".to_string());
    }
    if sample.contains("bsd 3-clause") || sample.contains("neither the name of") {
        return Some("BSD-3-Clause".to_string());
    }
    if sample.contains("mozilla public license") && sample.contains("2.0") {
        return Some("MPL-2.0".to_string());
    }
    if sample.contains("gnu general public license") {
        if sample.contains("version 3") {
            return Some("GPL-3.0".to_string());
        }
        return Some("GPL".to_string());
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mit_reconnue() {
        let mit = b"MIT License\n\nCopyright (c) 2026\n\nPermission is hereby granted, free of charge, to any person...";
        assert_eq!(identify(mit).as_deref(), Some("MIT"));
    }

    #[test]
    fn apache2_reconnue() {
        let apache = b"                                 Apache License\n                           Version 2.0, January 2004\n";
        assert_eq!(identify(apache).as_deref(), Some("Apache-2.0"));
    }

    #[test]
    fn bsd3_reconnue() {
        let bsd = b"BSD 3-Clause License\n\nRedistribution and use in source and binary forms";
        assert_eq!(identify(bsd).as_deref(), Some("BSD-3-Clause"));
    }

    #[test]
    fn mpl_reconnue() {
        let mpl = b"Mozilla Public License Version 2.0\n\n1. Definitions";
        assert_eq!(identify(mpl).as_deref(), Some("MPL-2.0"));
    }

    #[test]
    fn gpl3_reconnue() {
        let gpl = b"                    GNU General Public License\n                       Version 3, 29 June 2007";
        assert_eq!(identify(gpl).as_deref(), Some("GPL-3.0"));
    }

    #[test]
    fn texte_quelconque_retourne_none() {
        assert!(identify(b"just some readme content").is_none());
    }
}
