# Installation

codev est un binaire unique. Des binaires précompilés sont publiés sur
[GitHub Releases](https://github.com/mairistem/codev/releases) pour les
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
curl -sSL https://raw.githubusercontent.com/mairistem/codev/main/install.sh | sh
```

Le script détecte votre système et votre architecture, télécharge l'archive
correspondante et le fichier `SHA256SUMS` depuis la dernière GitHub Release,
vérifie l'empreinte SHA-256 de l'archive et installe le binaire dans
`~/.local/bin/codev`. Il refuse d'installer si l'empreinte ne correspond pas,
et crée `~/.local/bin` si nécessaire.

Si `~/.local/bin` n'est pas dans votre `PATH`, le script affiche la ligne
exacte à ajouter à `~/.zshrc` ou `~/.bashrc` :

```bash
export PATH="$HOME/.local/bin:$PATH"
```

Pour installer une version précise, définissez `CODEV_VERSION` :

```bash
curl -sSL https://raw.githubusercontent.com/mairistem/codev/main/install.sh | CODEV_VERSION=0.3.2 sh
```

Le script nécessite `curl`, `tar`, ainsi que `sha256sum` ou `shasum`.

## Windows : install.ps1

Dans PowerShell 5.1 ou ultérieur (PowerShell 7 compris) :

```powershell
iwr -useb https://raw.githubusercontent.com/mairistem/codev/main/install.ps1 | iex
```

Le script télécharge l'archive `.zip` et `SHA256SUMS`, vérifie l'archive avec
`Get-FileHash` et copie `codev.exe` dans `%LOCALAPPDATA%\Programs\codev\`.

Il ne modifie **pas** votre `PATH`. Si le dossier est absent du `PATH` de
votre utilisateur, il affiche la commande pour l'y ajouter :

```powershell
[Environment]::SetEnvironmentVariable('PATH', [Environment]::GetEnvironmentVariable('PATH', 'User') + ";$env:LOCALAPPDATA\Programs\codev", 'User')
```

Cette commande ne modifie que le `PATH` *utilisateur*. Évitez
`setx PATH "$env:PATH;..."` : cette commande tronque les valeurs de plus de
1 024 caractères et recopie le `PATH` de la machine dans le `PATH`
utilisateur. Ouvrez ensuite une nouvelle fenêtre PowerShell pour que la
modification prenne effet.

Pour installer une version précise :

```powershell
$env:CODEV_VERSION = '0.3.2'
iwr -useb https://raw.githubusercontent.com/mairistem/codev/main/install.ps1 | iex
```

Sous Windows, seule l'architecture x86_64 (`AMD64`) est disponible en binaire
précompilé. Sur ARM64, compilez depuis les sources.

## Téléchargement manuel depuis GitHub Releases

Si vous préférez ne pas exécuter un script directement dans votre shell,
téléchargez vous-même les fichiers depuis la
[dernière release](https://github.com/mairistem/codev/releases/latest). Chaque
release contient une archive par cible, nommée
`codev-<version>-<target>.tar.gz` (`.zip` sous Windows), ainsi qu'un fichier
`SHA256SUMS`.

**macOS et Linux.** Téléchargez votre archive et `SHA256SUMS` dans le même
dossier, puis :

```bash
# Vérifier l'empreinte — sur macOS : shasum -a 256 -c SHA256SUMS --ignore-missing
sha256sum -c SHA256SUMS --ignore-missing

tar -xzf codev-*.tar.gz
mkdir -p ~/.local/bin
cp codev-*/codev ~/.local/bin/
chmod 755 ~/.local/bin/codev
```

**Windows.** Téléchargez `codev-<version>-x86_64-pc-windows-msvc.zip` et
`SHA256SUMS`, puis dans PowerShell :

```powershell
# Vérifier l'empreinte
$zip = Get-Item codev-*-x86_64-pc-windows-msvc.zip
$expected = (Select-String -Path SHA256SUMS -Pattern $zip.Name).Line.Split(' ')[0]
$actual = (Get-FileHash $zip -Algorithm SHA256).Hash.ToLower()
if ($actual -ne $expected) { throw "SHA-256 mismatch" }

# Extraire et installer
Expand-Archive $zip -DestinationPath .
$dst = "$env:LOCALAPPDATA\Programs\codev"
New-Item -ItemType Directory -Path $dst -Force | Out-Null
Copy-Item codev-*\codev.exe $dst
```

Ajoutez ensuite le dossier au `PATH` de votre utilisateur comme indiqué
plus haut.

## Depuis les sources : cargo install

Avec une chaîne d'outils Rust (1.89 ou ultérieure), vous pouvez compiler codev
à partir d'un clone du dépôt. C'est la voie à suivre pour les contributeurs et
pour les plateformes sans binaire précompilé.

```bash
git clone https://github.com/mairistem/codev.git
cd codev
cargo install --path crates/codev-cli
```

Le binaire est installé dans `~/.cargo/bin/codev`.

## Vérifier l'installation

```bash
codev --version
```

Sous macOS et Linux, `command -v codev` indique quel binaire votre shell
exécute. Sous Windows, utilisez `Get-Command codev`. Si la commande est
introuvable, le dossier d'installation n'est pas dans votre `PATH` ; reportez-vous
aux instructions ci-dessus pour votre plateforme.

## Complétion du shell

`codev completions <SHELL>` écrit sur la sortie standard un script de
complétion pour `bash`, `zsh`, `fish`, `powershell` ou `elvish` :

```bash
# bash
codev completions bash > ~/.local/share/bash-completion/completions/codev

# zsh — puis lancez `compinit`
codev completions zsh > "${fpath[1]}/_codev"

# fish
codev completions fish > ~/.config/fish/completions/codev.fish
```

```powershell
# PowerShell, session en cours
codev completions powershell | Out-String | Invoke-Expression
```

## Prérequis

- **Claude Code**, pour utiliser les skills. La CLI elle-même fonctionne
  sans.
- **git**, uniquement pour les [sources héritées](guides/inherited-sources.md)
  hébergées dans un dépôt git.

## Mise à jour

Relancez l'installateur ou téléchargez une release plus récente. Ensuite, dans
chaque projet, régénérez les skills pour qu'elles correspondent à la nouvelle
version :

```bash
codev update
```

Consultez [`codev update`](reference/cli.md#codev-update) pour savoir comment
sont traitées les skills modifiées à la main.
