#!/bin/sh
# codev — installer for macOS and Linux.
#
# Usage:
#
#   # Latest release
#   curl -sSL https://raw.githubusercontent.com/mairistem/codev/main/install.sh | sh
#
#   # Specific version
#   curl -sSL https://raw.githubusercontent.com/mairistem/codev/main/install.sh | \
#     CODEV_VERSION=0.2.0 sh
#
# Detects OS and architecture, downloads the matching binary from GitHub
# Releases, verifies its SHA-256 checksum, then copies it to
# ~/.local/bin/codev.
#
# No Rust toolchain needed. Requires: POSIX sh, curl, tar, and sha256sum or
# shasum.

set -eu

REPO="mairistem/codev"
INSTALL_DIR="${HOME}/.local/bin"
BIN_NAME="codev"

# ─────────────────────────── helpers ───────────────────────────

log()  { printf '==> %s\n' "$*"; }
warn() { printf 'warning: %s\n' "$*" >&2; }
die()  { printf 'error: %s\n' "$*" >&2; exit 1; }

need() {
  command -v "$1" >/dev/null 2>&1 || die "missing required command: $1"
}

need curl
need tar
need uname

# sha256sum on Linux, shasum -a 256 on macOS.
if command -v sha256sum >/dev/null 2>&1; then
  SHA256="sha256sum"
elif command -v shasum >/dev/null 2>&1; then
  SHA256="shasum -a 256"
else
  die "neither sha256sum nor shasum is available — cannot verify integrity"
fi

# ─────────────────────────── detection ───────────────────────────

os="$(uname -s)"
arch="$(uname -m)"

case "${os}-${arch}" in
  Darwin-arm64)   target="aarch64-apple-darwin" ;;
  Darwin-x86_64)  target="x86_64-apple-darwin" ;;
  Linux-x86_64)   target="x86_64-unknown-linux-musl" ;;
  *)
    die "unsupported platform: ${os}-${arch}
Prebuilt binaries: macOS arm64, macOS x86_64, Linux x86_64 (and Windows x86_64
via install.ps1). For other platforms, install the Rust toolchain and run
'cargo install --path crates/codev-cli' from a clone of the repository."
    ;;
esac

log "detected platform: ${target}"

# ─────────────────────────── version ───────────────────────────

version="${CODEV_VERSION:-}"

if [ -z "${version}" ]; then
  log "resolving the latest release from the GitHub API..."
  # Minimal JSON extraction (grep + cut) to avoid depending on jq.
  latest="$(curl -sSL "https://api.github.com/repos/${REPO}/releases/latest" \
    | grep '"tag_name":' | head -1 | cut -d '"' -f 4)"
  if [ -z "${latest}" ]; then
    die "could not resolve the latest release (GitHub API request failed). \
Try again later, or pin a version with CODEV_VERSION=x.y.z."
  fi
  version="${latest#v}"
fi

log "target version: ${version}"

# ─────────────────────────── download ───────────────────────────

archive="codev-${version}-${target}.tar.gz"
base_url="https://github.com/${REPO}/releases/download/v${version}"
tmp="$(mktemp -d)"
trap 'rm -rf "${tmp}"' EXIT INT TERM

log "downloading ${base_url}/${archive}"
curl -fSL --progress-bar -o "${tmp}/${archive}" "${base_url}/${archive}" \
  || die "download failed (no asset for this version and platform?)"

log "downloading ${base_url}/SHA256SUMS"
curl -fSL -o "${tmp}/SHA256SUMS" "${base_url}/SHA256SUMS" \
  || die "could not download SHA256SUMS — integrity cannot be verified"

# ─────────────────────────── SHA-256 verification ───────────────────────────

log "verifying SHA-256 checksum..."
expected="$(grep " ${archive}\$" "${tmp}/SHA256SUMS" | head -1 | cut -d ' ' -f 1)"
if [ -z "${expected}" ]; then
  die "SHA256SUMS has no entry for ${archive} — refusing to install"
fi

actual="$( ${SHA256} "${tmp}/${archive}" | cut -d ' ' -f 1)"
if [ "${actual}" != "${expected}" ]; then
  die "SHA-256 mismatch (expected ${expected}, got ${actual}) — refusing to install"
fi

log "SHA-256 checksum verified"

# ─────────────────────────── installation ───────────────────────────

log "extracting..."
tar -xzf "${tmp}/${archive}" -C "${tmp}"

extracted="${tmp}/codev-${version}-${target}/${BIN_NAME}"
if [ ! -f "${extracted}" ]; then
  die "binary not found in archive: ${extracted}"
fi

mkdir -p "${INSTALL_DIR}"
cp "${extracted}" "${INSTALL_DIR}/${BIN_NAME}"
chmod 755 "${INSTALL_DIR}/${BIN_NAME}"

log "installed ${INSTALL_DIR}/${BIN_NAME}"

# ─────────────────────────── PATH ───────────────────────────

case ":${PATH:-}:" in
  *":${INSTALL_DIR}:"*)
    log "${INSTALL_DIR} is already on your PATH."
    log "Check with: codev --version"
    ;;
  *)
    warn "${INSTALL_DIR} is not on your PATH."
    printf '\nAdd this line to your ~/.zshrc or ~/.bashrc:\n\n'
    printf '  export PATH="%s:$PATH"\n\n' "${INSTALL_DIR}"
    printf 'Then open a new shell, or run: source ~/.zshrc\n'
    ;;
esac

log "codev v${version} is ready. Next: cd into a repository and run codev init"
