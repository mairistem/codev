# codev — script d'installation Windows.
#
# Usage :
#
#   # Dernière version
#   iwr -useb https://raw.githubusercontent.com/mairistem/codev/main/install.ps1 | iex
#
#   # Version précise
#   $env:CODEV_VERSION = '0.2.0'
#   iwr -useb https://raw.githubusercontent.com/mairistem/codev/main/install.ps1 | iex
#
# Détecte l'arch, télécharge le binaire depuis GitHub Releases, vérifie
# son SHA-256 puis le copie dans %LOCALAPPDATA%\Programs\codev\codev.exe.
#
# Aucune dépendance à Rust. Requiert PowerShell 5.1+ (built-in Windows 10+).
# N'appelle pas `Set-ExecutionPolicy` — un script piped depuis stdin passe
# sans restriction. Ne modifie pas le PATH utilisateur : les instructions
# sont affichées à la fin.

$ErrorActionPreference = 'Stop'

$repo       = 'mairistem/codev'
$installDir = Join-Path $env:LOCALAPPDATA 'Programs\codev'
$binName    = 'codev.exe'

function Say([string]$msg)  { Write-Host "==> $msg" }
function Warn([string]$msg) { Write-Warning $msg }
function Die([string]$msg)  { Write-Error $msg; exit 1 }

# ─────────────────────────── détection arch ───────────────────────────

$arch = $env:PROCESSOR_ARCHITECTURE
switch ($arch) {
    'AMD64' { $target = 'x86_64-pc-windows-msvc' }
    default {
        Die @"
Architecture non supportée : $arch
La seule cible Windows précompilée est x86_64 (AMD64).
Pour ARM64 ou autre, installe la toolchain Rust et lance
'cargo install --path crates/codev-cli' depuis un clone du dépôt.
"@
    }
}
Say "plateforme détectée : $target"

# ─────────────────────────── version ───────────────────────────

$version = $env:CODEV_VERSION
if ([string]::IsNullOrEmpty($version)) {
    Say 'résolution de la dernière version via l''API GitHub...'
    try {
        $latest = Invoke-RestMethod -Uri "https://api.github.com/repos/$repo/releases/latest" -Headers @{ 'User-Agent' = 'codev-installer' }
        $version = $latest.tag_name -replace '^v', ''
    } catch {
        Die "impossible de résoudre la dernière version (API GitHub ratée : $_). Fixe une version avec `$env:CODEV_VERSION = 'x.y.z'`."
    }
}
Say "version cible : $version"

# ─────────────────────────── téléchargement ───────────────────────────

$archive     = "codev-$version-$target.zip"
$baseUrl     = "https://github.com/$repo/releases/download/v$version"
$tmp         = New-Item -ItemType Directory -Path (Join-Path $env:TEMP "codev-install-$([guid]::NewGuid())") -Force
$archivePath = Join-Path $tmp $archive
$sumsPath    = Join-Path $tmp 'SHA256SUMS'

try {
    Say "téléchargement : $baseUrl/$archive"
    try {
        Invoke-WebRequest -Uri "$baseUrl/$archive" -OutFile $archivePath -UseBasicParsing
    } catch {
        Die "téléchargement raté (asset introuvable pour cette version/plateforme ?) : $_"
    }

    Say "téléchargement : $baseUrl/SHA256SUMS"
    try {
        Invoke-WebRequest -Uri "$baseUrl/SHA256SUMS" -OutFile $sumsPath -UseBasicParsing
    } catch {
        Die "impossible de récupérer SHA256SUMS — l'intégrité ne peut pas être vérifiée : $_"
    }

    # ─────────────────────────── vérification SHA-256 ───────────────────────────

    Say 'vérification SHA-256...'
    $sumsContent = Get-Content $sumsPath
    $expectedLine = $sumsContent | Where-Object { $_ -match "\s$([regex]::Escape($archive))$" } | Select-Object -First 1
    if (-not $expectedLine) {
        Die "SHA256SUMS ne référence pas $archive — refus d'installer"
    }
    $expected = ($expectedLine -split '\s+')[0].ToLower()
    $actual   = (Get-FileHash -Path $archivePath -Algorithm SHA256).Hash.ToLower()
    if ($actual -ne $expected) {
        Die "SHA-256 divergent (attendu $expected, obtenu $actual) — refus d'installer"
    }
    Say 'SHA-256 vérifié'

    # ─────────────────────────── installation ───────────────────────────

    Say 'extraction...'
    Expand-Archive -Path $archivePath -DestinationPath $tmp -Force

    $extractedBin = Join-Path $tmp "codev-$version-$target\$binName"
    if (-not (Test-Path $extractedBin)) {
        Die "binaire absent dans l'archive : $extractedBin"
    }

    New-Item -ItemType Directory -Path $installDir -Force | Out-Null
    Copy-Item -Path $extractedBin -Destination (Join-Path $installDir $binName) -Force
    Say "installé : $(Join-Path $installDir $binName)"

    # ─────────────────────────── PATH ───────────────────────────

    $userPath = [Environment]::GetEnvironmentVariable('PATH', 'User')
    $entries  = if ($userPath) { $userPath -split ';' } else { @() }
    if ($entries -contains $installDir) {
        Say "$installDir est déjà dans ton PATH utilisateur."
        Say 'Vérifie avec : codev --version'
    } else {
        Warn "$installDir n'est pas dans ton PATH utilisateur."
        Write-Host ''
        Write-Host 'Ajoute-le avec cette commande, dans un shell séparé :'
        Write-Host ''
        Write-Host "  setx PATH `"`$env:PATH;$installDir`""
        Write-Host ''
        Write-Host 'Ou par Paramètres → Système → Variables d''environnement.'
        Write-Host 'Puis ouvre un nouveau PowerShell pour recharger le PATH.'
    }

    Say "codev v$version prêt. Lance : codev docs"
}
finally {
    Remove-Item -Path $tmp -Recurse -Force -ErrorAction SilentlyContinue
}
