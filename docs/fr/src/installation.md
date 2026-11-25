# Installation

Compagnon est un binaire unique. Des binaires précompilés sont publiés sur
[GitHub Releases](https://github.com/mairistem/compagnon/releases) pour les
plateformes suivantes :

| Plateforme | Cible |
|---|---|
| macOS, Apple Silicon | `aarch64-apple-darwin` |
| macOS, Intel | `x86_64-apple-darwin` |
| Linux, x86_64 (lié statiquement) | `x86_64-unknown-linux-musl` |
| Windows, x86_64 | `x86_64-pc-windows-msvc` |

Il existe quatre façons de l'installer. Les deux premières sont les plus
rapides et ne nécessitent pas Rust.

## macOS et Linux : install.sh

```bash
curl -sSL https://raw.githubusercontent.com/mairistem/compagnon/main/install.sh | sh
```

Le script détecte votre système et votre architecture, télécharge l'archive
correspondante et le fichier `SHA256SUMS` depuis la dernière GitHub Release,
vérifie l'empreinte SHA-256 de l'archive et installe le binaire dans
`~/.local/bin/compagnon`. Il refuse d'installer si l'empreinte ne correspond pas,
et crée `~/.local/bin` si nécessaire.

Si `~/.local/bin` n'est pas dans votre `PATH`, le script affiche la ligne
exacte à ajouter à `~/.zshrc` ou `~/.bashrc` :

```bash
export PATH="$HOME/.local/bin:$PATH"
```

Pour installer une version précise, définissez `COMPAGNON_VERSION` :

```bash
curl -sSL https://raw.githubusercontent.com/mairistem/compagnon/main/install.sh | COMPAGNON_VERSION=0.6.1 sh
```

Le script nécessite `curl`, `tar`, ainsi que `sha256sum` ou `shasum`.

## Windows : install.ps1

Dans PowerShell 5.1 ou ultérieur (PowerShell 7 compris) :

```powershell
iwr -useb https://raw.githubusercontent.com/mairistem/compagnon/main/install.ps1 | iex
```

Le script télécharge l'archive `.zip` et `SHA256SUMS`, vérifie l'archive avec
`Get-FileHash` et copie `compagnon.exe` dans `%LOCALAPPDATA%\Programs\compagnon\`.

Il ne modifie **pas** votre `PATH`. Si le dossier est absent du `PATH` de
votre utilisateur, il affiche la commande pour l'y ajouter :

```powershell
[Environment]::SetEnvironmentVariable('PATH', [Environment]::GetEnvironmentVariable('PATH', 'User') + ";$env:LOCALAPPDATA\Programs\compagnon", 'User')
```

Cette commande ne modifie que le `PATH` *utilisateur*. Évitez
`setx PATH "$env:PATH;..."` : cette commande tronque les valeurs de plus de
1 024 caractères et recopie le `PATH` de la machine dans le `PATH`
utilisateur. Ouvrez ensuite une nouvelle fenêtre PowerShell pour que la
modification prenne effet.

Pour installer une version précise :

```powershell
$env:COMPAGNON_VERSION = '0.6.1'
iwr -useb https://raw.githubusercontent.com/mairistem/compagnon/main/install.ps1 | iex
```

Sous Windows, seule l'architecture x86_64 (`AMD64`) est disponible en binaire
précompilé. Sur ARM64, compilez depuis les sources.

## Téléchargement manuel depuis GitHub Releases

Si vous préférez ne pas exécuter un script directement dans votre shell,
téléchargez vous-même les fichiers depuis la
[dernière release](https://github.com/mairistem/compagnon/releases/latest). Chaque
release contient une archive par cible, nommée
`compagnon-<version>-<target>.tar.gz` (`.zip` sous Windows), ainsi qu'un fichier
`SHA256SUMS`.

**macOS et Linux.** Téléchargez votre archive et `SHA256SUMS` dans le même
dossier, puis :

```bash
# Vérifier l'empreinte — sur macOS : shasum -a 256 -c SHA256SUMS --ignore-missing
sha256sum -c SHA256SUMS --ignore-missing

tar -xzf compagnon-*.tar.gz
mkdir -p ~/.local/bin
cp compagnon-*/compagnon ~/.local/bin/
chmod 755 ~/.local/bin/compagnon
```

**Windows.** Téléchargez `compagnon-<version>-x86_64-pc-windows-msvc.zip` et
`SHA256SUMS`, puis dans PowerShell :

```powershell
# Vérifier l'empreinte
$zip = Get-Item compagnon-*-x86_64-pc-windows-msvc.zip
$expected = (Select-String -Path SHA256SUMS -Pattern $zip.Name).Line.Split(' ')[0]
$actual = (Get-FileHash $zip -Algorithm SHA256).Hash.ToLower()
if ($actual -ne $expected) { throw "SHA-256 mismatch" }

# Extraire et installer
Expand-Archive $zip -DestinationPath .
$dst = "$env:LOCALAPPDATA\Programs\compagnon"
New-Item -ItemType Directory -Path $dst -Force | Out-Null
Copy-Item compagnon-*\compagnon.exe $dst
```

Ajoutez ensuite le dossier au `PATH` de votre utilisateur comme indiqué
plus haut.

## Depuis les sources : cargo install

Avec une chaîne d'outils Rust (1.89 ou ultérieure), vous pouvez compiler Compagnon
à partir d'un clone du dépôt. C'est la voie à suivre pour les contributeurs et
pour les plateformes sans binaire précompilé.

```bash
git clone https://github.com/mairistem/compagnon.git
cd compagnon
cargo install --path crates/compagnon-cli
```

Le binaire est installé dans `~/.cargo/bin/compagnon`.

## Vérifier l'installation

```bash
compagnon --version
```

Sous macOS et Linux, `command -v compagnon` indique quel binaire votre shell
exécute. Sous Windows, utilisez `Get-Command compagnon`. Si la commande est
introuvable, le dossier d'installation n'est pas dans votre `PATH` ; reportez-vous
aux instructions ci-dessus pour votre plateforme.

## Complétion du shell

`compagnon completions <SHELL>` écrit sur la sortie standard un script de
complétion pour `bash`, `zsh`, `fish`, `powershell` ou `elvish` :

```bash
# bash
compagnon completions bash > ~/.local/share/bash-completion/completions/compagnon

# zsh — puis lancez `compinit`
compagnon completions zsh > "${fpath[1]}/_compagnon"

# fish
compagnon completions fish > ~/.config/fish/completions/compagnon.fish
```

```powershell
# PowerShell, session en cours
compagnon completions powershell | Out-String | Invoke-Expression
```

## Prérequis

- **Claude Code**, pour utiliser les skills. La CLI elle-même fonctionne
  sans.
- **git**, uniquement pour les [sources héritées](guides/inherited-sources.md)
  hébergées dans un dépôt git.

## Mise à jour

Lancez `compagnon self-update` :

```bash
compagnon self-update                  # demande confirmation dans un terminal
compagnon self-update --check          # indique seulement la version installée et la plus récente
compagnon self-update --version 0.5.0  # une version précise, rétrogradation comprise
```

La commande télécharge la release la plus récente pour votre plateforme,
vérifie son SHA-256 avec le `SHA256SUMS` de la release — comme le font les
installateurs — et remplace le binaire sur place. Ensuite, dans un projet
Compagnon, elle lance `compagnon update` avec le nouveau binaire. Elle a besoin de
`curl` et de `tar`, fournis par macOS, la plupart des distributions Linux et
Windows 10 ou ultérieur.

`compagnon self-update` ne remplace qu'un binaire installé par `install.sh`,
`install.ps1` ou à la main. Si Compagnon a été installé avec `cargo install`,
compilé depuis un clone ou installé par un gestionnaire de paquets, elle
affiche la commande qui le met à jour et ne modifie rien. Elle ne rétrograde
jamais, sauf si vous passez `--version`. Sous Windows, l'ancien binaire est
conservé sous le nom `compagnon.old.exe` à côté du nouveau, jusqu'à ce que la
commande Compagnon suivante le supprime.

`compagnon status` vous signale l'existence d'une release plus récente, au plus
une fois par jour ; voir
[Compagnon vérifie-t-il les mises à jour ?](faq.md#compagnon-vérifie-t-il-les-mises-à-jour-)
pour le désactiver.

Vous pouvez toujours relancer l'installateur ou télécharger une release plus
récente. Ensuite, dans chaque projet, régénérez les skills pour qu'elles
correspondent à la nouvelle version :

```bash
compagnon update
```

`compagnon update` rafraîchit aussi les sections gérées par Compagnon dans `CLAUDE.md`
et dans le modèle de pull request, ainsi que la version de Compagnon épinglée dans
`.github/workflows/compagnon.yml`, là où `compagnon init` les a écrites. Consultez
[`compagnon update`](reference/cli.md#compagnon-update) pour savoir comment sont
traitées les skills modifiées à la main, et `compagnon doctor` pour vérifier le
résultat.
