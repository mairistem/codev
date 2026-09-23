# Design : ajout de la cible Windows

## Contexte

Voir `proposal.md`. Extension purement additive de la capacité
`distribution` — 4 cibles au lieu de 3, un script d'install en plus.

## Objectifs / Hors objectifs

Ce design cadre : le job Windows dans la matrix, le format d'archive
Windows, la mécanique du script `install.ps1`, la vérification SHA-256
en PowerShell, la mise à jour de la doc. Il ne cadre pas Windows ARM64,
les package managers Windows (Scoop, Chocolatey, winget), ni la
modification automatique du PATH utilisateur.

## Décisions

### Décision : `.zip` pour Windows, `.tar.gz` pour Unix

| Format | Windows | Unix |
|---|---|---|
| `.tar.gz` | Extraction possible (`tar` natif depuis Windows 10 build 1803) mais culturellement Unix | Standard |
| `.zip` | Standard Windows, `Expand-Archive` natif dès PowerShell 5.1 | Nécessite `unzip` séparé sur Linux minimal |

**Choisi** : un format par famille d'OS. C'est ce que font ripgrep,
fd, starship, bat, le pattern est stabilisé.

### Décision : runner `windows-latest`, pas de version épinglée

`windows-latest` pointe aujourd'hui vers Windows Server 2022. Un
`windows-2019` figé nous ferait dépendre d'une machine que GitHub
finira par déprécier — et rien ne justifie cette précaution.

**Alternative écartée** : `windows-2022`. Aucun bénéfice
supplémentaire, un léger risque si GitHub renomme.

### Décision : `x86_64-pc-windows-msvc`, pas GNU

Rust supporte deux toolchains Windows :

- `x86_64-pc-windows-msvc` : lié dynamiquement à `msvcrt.dll` (Visual
  C++ Runtime) ; c'est la voie officielle Microsoft.
- `x86_64-pc-windows-gnu` : lié à MinGW ; plus portable en théorie,
  mais peu utilisé.

**Choisi : MSVC.** C'est le défaut sur Windows dans la communauté
Rust. Les runners `windows-latest` fournissent déjà les headers/libs
Microsoft ; aucune installation supplémentaire.

### Décision : le script `install.ps1` ne modifie pas le PATH

Symétrie avec `install.sh` qui n'écrit jamais dans `.zshrc`/`.bashrc`.
Modifier automatiquement le PATH utilisateur avec `setx` ou
`[Environment]::SetEnvironmentVariable` est **irréversible en pratique**
côté utilisateur — s'il désinstalle codev, l'entrée reste dans son
registre.

Le script affiche la commande exacte à taper. C'est deux lignes de
plus pour l'utilisateur, contre du bruit permanent qu'on lui
imposerait.

### Décision : `iwr -useb … | iex` comme incantation officielle

Le pattern PowerShell canonique — utilisé par rustup, Scoop, oh-my-posh,
starship. `iwr` (alias de `Invoke-WebRequest`), `-useb` (alias de
`-UseBasicParsing`, contourne la dépendance à IE assets), `iex`
(alias de `Invoke-Expression`) exécute le contenu comme script.

Alternative : `Set-ExecutionPolicy Bypass -Scope Process -Force ; ...`.
Pas nécessaire — un script piped depuis stdin passe sans restriction
`Set-ExecutionPolicy`.

### Décision : destination `$env:LOCALAPPDATA\Programs\codev\`

`$env:LOCALAPPDATA` est le dossier de données locales de l'utilisateur
(`C:\Users\<user>\AppData\Local\` typiquement). `Programs\` y héberge
les binaires installés au niveau utilisateur — pattern Windows Terminal,
VS Code, PowerShell 7.

**Alternative écartée** : `$env:USERPROFILE\.local\bin\` par symétrie
avec Unix. Fonctionne mais peu idiomatique côté Windows (les
utilisateurs cherchent leurs binaires dans `%LOCALAPPDATA%`).

### Décision : matrix étendue, pas de nouveau workflow

Une nouvelle ligne dans la matrix `strategy.matrix.include` du workflow
existant, plus une branche conditionnelle sur l'OS pour l'empaquetage
(`.zip` vs `.tar.gz`). Un `if: matrix.target == 'x86_64-pc-windows-msvc'`
gère les spécificités Windows dans le même job.

**Alternative écartée** : deux workflows séparés (`release-unix.yml` +
`release-windows.yml`). Duplication du job `release` final qui agrège.
Rejeté au titre de la simplicité.

### Décision : le job `release` utilise le bon binaire dans le SHA256SUMS

Le job qui agrège calcule le SHA-256 de **tous** les fichiers `.tar.gz`
et `.zip` du dossier `dist/`, quelle que soit leur extension. La ligne
actuelle `shasum -a 256 *.tar.gz > SHA256SUMS` devient
`shasum -a 256 *.tar.gz *.zip > SHA256SUMS`.

## Risques et compromis

- **PowerShell 5.1 vs 7** — `Expand-Archive`, `Get-FileHash`,
  `Invoke-WebRequest` existent tous depuis PowerShell 5.1. Le script
  ne demande rien de récent, il marche sur un Windows 10 vanilla.
- **Anti-virus qui bloque le binaire téléchargé** — Windows Defender
  peut être overzealous sur un `.exe` téléchargé et exécuté depuis
  `%LOCALAPPDATA%`. Documenté dans la FAQ : « en cas de blocage,
  déclarer `codev.exe` comme excluded ou signer plus tard. »
- **Signature de code Windows (Authenticode)** — non couvert V1. Un
  utilisateur qui exécute `codev.exe` verra un warning « publisher
  inconnu ». Reportable jusqu'à demande.

## Plan de migration

Aucune. Les 3 cibles existantes et `install.sh` sont préservés. Un
utilisateur Windows qui installait via `cargo install` peut migrer
en une commande :

```powershell
iwr -useb https://raw.githubusercontent.com/mairistem/codev/main/install.ps1 | iex
```
