## MODIFIED Requirements

### Requirement: La documentation cite les trois voies d'installation

Note : le titre historique reste « trois voies » pour préserver la
compatibilité de nom avec la spec principale ; le contenu ci-dessous
décrit **quatre** voies après ajout de Windows. Un renommage propre
viendra dans un cycle dédié.

La section Installation de `docs/codev.md` MUST citer, dans cet ordre :

1. **Voie recommandée Unix** — `curl -sSL … | sh` pour macOS/Linux.
2. **Voie recommandée Windows** — `iwr -useb … | iex` pour Windows
   dans PowerShell.
3. **Voie manuelle** — téléchargement depuis GitHub Releases + vérif
   `sha256sum -c` (Unix) ou `Get-FileHash` (Windows).
4. **Voie contributeur** — `cargo install --path crates/codev-cli`
   depuis un clone du dépôt.

Le README du dépôt MUST mentionner au moins la première **et** la
deuxième voie (avec les one-liners `curl … | sh` et `iwr … | iex`).

#### Scenario: la section Installation de docs/codev.md liste les quatre voies dans l'ordre

- **GIVEN** un lecteur qui ouvre `docs/codev.md` à la section
  Installation
- **WHEN** il parcourt les sous-sections dans l'ordre
- **THEN** il rencontre successivement la voie Unix (`curl | sh`),
  la voie Windows (`iwr | iex`), la voie manuelle (téléchargement
  depuis GitHub Releases avec vérif SHA-256), et la voie contributeur
  (`cargo install --path`)

#### Scenario: le README pointe au moins les deux voies « sans Rust » dans son Démarrage

- **GIVEN** un lecteur qui ouvre `README.md` à la racine du dépôt
- **WHEN** il parcourt la section « Démarrage »
- **THEN** il voit l'exemple `curl -sSL … | sh` pour macOS/Linux
- **AND** il voit l'exemple `iwr -useb … | iex` pour Windows dans
  PowerShell
