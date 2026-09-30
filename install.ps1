# codev - installer for Windows.
#
# Usage:
#
#   # Latest release
#   iwr -useb https://raw.githubusercontent.com/mairistem/codev/main/install.ps1 | iex
#
#   # Specific version
#   $env:CODEV_VERSION = '0.2.0'
#   iwr -useb https://raw.githubusercontent.com/mairistem/codev/main/install.ps1 | iex
#
# Detects the architecture, downloads the binary from GitHub Releases,
# verifies its SHA-256 checksum, then copies it to
# %LOCALAPPDATA%\Programs\codev\codev.exe.
#
# No Rust toolchain needed. Requires PowerShell 5.1+ (built into Windows 10+).
# Does not call `Set-ExecutionPolicy` - a script piped through `iex` is not
# subject to it. Does not modify the user PATH: instructions are printed at
# the end instead.
#
# This file is intentionally pure ASCII: Windows PowerShell 5.1 reads
# BOM-less scripts with the ANSI code page, which would mangle any
# non-ASCII character when the script is run from disk.

$ErrorActionPreference = 'Stop'

$repo       = 'mairistem/codev'
$installDir = Join-Path $env:LOCALAPPDATA 'Programs\codev'
$binName    = 'codev.exe'

function Say([string]$msg)  { Write-Host "==> $msg" }
function Warn([string]$msg) { Write-Warning $msg }
function Die([string]$msg)  { Write-Error $msg; exit 1 }

# --------------------------- architecture ---------------------------

$arch = $env:PROCESSOR_ARCHITECTURE
switch ($arch) {
    'AMD64' { $target = 'x86_64-pc-windows-msvc' }
    default {
        Die @"
Unsupported architecture: $arch
The only prebuilt Windows target is x86_64 (AMD64).
For ARM64 or other architectures, install the Rust toolchain and run
'cargo install --path crates/codev-cli' from a clone of the repository.
"@
    }
}
Say "detected platform: $target"

# ----------------------------- version ------------------------------

$version = $env:CODEV_VERSION
if ([string]::IsNullOrEmpty($version)) {
    Say 'resolving the latest release from the GitHub API...'
    try {
        $latest = Invoke-RestMethod -Uri "https://api.github.com/repos/$repo/releases/latest" -Headers @{ 'User-Agent' = 'codev-installer' }
        $version = $latest.tag_name -replace '^v', ''
    } catch {
        Die "could not resolve the latest release (GitHub API request failed: $_). Pin a version with `$env:CODEV_VERSION = 'x.y.z'`."
    }
}
Say "target version: $version"

# ----------------------------- download -----------------------------

$archive     = "codev-$version-$target.zip"
$baseUrl     = "https://github.com/$repo/releases/download/v$version"
$tmp         = New-Item -ItemType Directory -Path (Join-Path $env:TEMP "codev-install-$([guid]::NewGuid())") -Force
$archivePath = Join-Path $tmp $archive
$sumsPath    = Join-Path $tmp 'SHA256SUMS'

try {
    Say "downloading $baseUrl/$archive"
    try {
        Invoke-WebRequest -Uri "$baseUrl/$archive" -OutFile $archivePath -UseBasicParsing
    } catch {
        Die "download failed (no asset for this version and platform?): $_"
    }

    Say "downloading $baseUrl/SHA256SUMS"
    try {
        Invoke-WebRequest -Uri "$baseUrl/SHA256SUMS" -OutFile $sumsPath -UseBasicParsing
    } catch {
        Die "could not download SHA256SUMS - integrity cannot be verified: $_"
    }

    # ----------------------- SHA-256 verification -----------------------

    Say 'verifying SHA-256 checksum...'
    $sumsContent = Get-Content $sumsPath
    $expectedLine = $sumsContent | Where-Object { $_ -match "\s$([regex]::Escape($archive))$" } | Select-Object -First 1
    if (-not $expectedLine) {
        Die "SHA256SUMS has no entry for $archive - refusing to install"
    }
    $expected = ($expectedLine -split '\s+')[0].ToLower()
    $actual   = (Get-FileHash -Path $archivePath -Algorithm SHA256).Hash.ToLower()
    if ($actual -ne $expected) {
        Die "SHA-256 mismatch (expected $expected, got $actual) - refusing to install"
    }
    Say 'SHA-256 checksum verified'

    # --------------------------- installation ---------------------------

    Say 'extracting...'
    Expand-Archive -Path $archivePath -DestinationPath $tmp -Force

    $extractedBin = Join-Path $tmp "codev-$version-$target\$binName"
    if (-not (Test-Path $extractedBin)) {
        Die "binary not found in archive: $extractedBin"
    }

    New-Item -ItemType Directory -Path $installDir -Force | Out-Null
    Copy-Item -Path $extractedBin -Destination (Join-Path $installDir $binName) -Force
    Say "installed $(Join-Path $installDir $binName)"

    # ------------------------------- PATH -------------------------------

    $userPath = [Environment]::GetEnvironmentVariable('PATH', 'User')
    $entries  = if ($userPath) { $userPath -split ';' } else { @() }
    if ($entries -contains $installDir) {
        Say "$installDir is already on your user PATH."
        Say 'Check with: codev --version'
    } else {
        Warn "$installDir is not on your user PATH."
        Write-Host ''
        Write-Host 'Add it with this command:'
        Write-Host ''
        # Writes the *user* PATH only. `setx PATH "$env:PATH;..."` is avoided on
        # purpose: it truncates at 1024 characters and copies the machine PATH
        # into the user PATH.
        Write-Host "  [Environment]::SetEnvironmentVariable('PATH', [Environment]::GetEnvironmentVariable('PATH', 'User') + ';$installDir', 'User')"
        Write-Host ''
        Write-Host 'Or use Settings > System > About > Advanced system settings > Environment Variables.'
        Write-Host 'Then open a new PowerShell window to reload PATH.'
    }

    Say "codev v$version is ready. Next: cd into a repository and run codev init"
}
finally {
    Remove-Item -Path $tmp -Recurse -Force -ErrorAction SilentlyContinue
}
