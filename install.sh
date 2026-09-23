#!/bin/sh
# codev — script d'installation.
#
# Usage :
#
#   # Dernière version
#   curl -sSL https://raw.githubusercontent.com/mairistem/codev/main/install.sh | sh
#
#   # Version précise
#   curl -sSL https://raw.githubusercontent.com/mairistem/codev/main/install.sh | \
#     CODEV_VERSION=0.2.0 sh
#
# Le script détecte OS + arch, télécharge le binaire correspondant depuis
# GitHub Releases, vérifie son SHA-256 puis le copie dans ~/.local/bin/codev.
#
# Aucune dépendance à Rust. Requiert : sh POSIX, curl, tar, sha256sum ou
# shasum.
#
# Sans MCP branché : le fallback silencieux de codev reste valide — la
# détection de tickets Jira dans /codev-propose n'est simplement pas active.

set -eu

REPO="mairistem/codev"
INSTALL_DIR="${HOME}/.local/bin"
BIN_NAME="codev"

# ─────────────────────────── outils ───────────────────────────

log()  { printf '==> %s\n' "$*"; }
warn() { printf 'attention : %s\n' "$*" >&2; }
die()  { printf 'erreur : %s\n' "$*" >&2; exit 1; }

need() {
  command -v "$1" >/dev/null 2>&1 || die "commande manquante : $1"
}

need curl
need tar
need uname

# sha256sum sur Linux, shasum -a 256 sur macOS.
if command -v sha256sum >/dev/null 2>&1; then
  SHA256="sha256sum"
elif command -v shasum >/dev/null 2>&1; then
  SHA256="shasum -a 256"
else
  die "ni sha256sum ni shasum disponibles — impossible de vérifier l'intégrité"
fi

# ─────────────────────────── détection ───────────────────────────

os="$(uname -s)"
arch="$(uname -m)"

case "${os}-${arch}" in
  Darwin-arm64)   target="aarch64-apple-darwin" ;;
  Darwin-x86_64)  target="x86_64-apple-darwin" ;;
  Linux-x86_64)   target="x86_64-unknown-linux-musl" ;;
  *)
    die "plateforme non supportée : ${os}-${arch}
Cibles précompilées disponibles : macOS arm64, macOS x86_64, Linux x86_64.
Pour Windows, Linux ARM64 ou autre : installe la toolchain Rust et lance
'cargo install --path crates/codev-cli' depuis un clone du dépôt."
    ;;
esac

log "plateforme détectée : ${target}"

# ─────────────────────────── version ───────────────────────────

version="${CODEV_VERSION:-}"

if [ -z "${version}" ]; then
  log "résolution de la dernière version via l'API GitHub..."
  # Simple parseur JSON (grep + cut) pour éviter une dépendance à jq.
  latest="$(curl -sSL "https://api.github.com/repos/${REPO}/releases/latest" \
    | grep '"tag_name":' | head -1 | cut -d '"' -f 4)"
  if [ -z "${latest}" ]; then
    die "impossible de résoudre la dernière version (API GitHub ratée). \
Réessaie plus tard, ou fixe une version avec CODEV_VERSION=x.y.z."
  fi
  version="${latest#v}"
fi

log "version cible : ${version}"

# ─────────────────────────── téléchargement ───────────────────────────

archive="codev-${version}-${target}.tar.gz"
base_url="https://github.com/${REPO}/releases/download/v${version}"
tmp="$(mktemp -d)"
trap 'rm -rf "${tmp}"' EXIT INT TERM

log "téléchargement : ${base_url}/${archive}"
curl -fSL --progress-bar -o "${tmp}/${archive}" "${base_url}/${archive}" \
  || die "téléchargement raté (asset introuvable pour cette version/plateforme ?)"

log "téléchargement : ${base_url}/SHA256SUMS"
curl -fSL -o "${tmp}/SHA256SUMS" "${base_url}/SHA256SUMS" \
  || die "impossible de récupérer SHA256SUMS — l'intégrité ne peut pas être vérifiée"

# ─────────────────────────── vérification SHA-256 ───────────────────────────

log "vérification SHA-256..."
expected="$(grep " ${archive}\$" "${tmp}/SHA256SUMS" | head -1 | cut -d ' ' -f 1)"
if [ -z "${expected}" ]; then
  die "SHA256SUMS ne référence pas ${archive} — refus d'installer"
fi

actual="$( ${SHA256} "${tmp}/${archive}" | cut -d ' ' -f 1)"
if [ "${actual}" != "${expected}" ]; then
  die "SHA-256 divergent (attendu ${expected}, obtenu ${actual}) — refus d'installer"
fi

log "SHA-256 vérifié"

# ─────────────────────────── installation ───────────────────────────

log "extraction..."
tar -xzf "${tmp}/${archive}" -C "${tmp}"

extracted="${tmp}/codev-${version}-${target}/${BIN_NAME}"
if [ ! -f "${extracted}" ]; then
  die "binaire absent dans l'archive : ${extracted}"
fi

mkdir -p "${INSTALL_DIR}"
cp "${extracted}" "${INSTALL_DIR}/${BIN_NAME}"
chmod 755 "${INSTALL_DIR}/${BIN_NAME}"

log "installé : ${INSTALL_DIR}/${BIN_NAME}"

# ─────────────────────────── PATH ───────────────────────────

case ":${PATH:-}:" in
  *":${INSTALL_DIR}:"*)
    log "${INSTALL_DIR} est déjà dans ton PATH."
    log "Vérifie avec : codev --version"
    ;;
  *)
    warn "${INSTALL_DIR} n'est pas dans ton PATH."
    printf '\nAjoute cette ligne à ton ~/.zshrc ou ~/.bashrc :\n\n'
    printf '  export PATH="%s:$PATH"\n\n' "${INSTALL_DIR}"
    printf 'Puis ouvre un nouveau shell ou : source ~/.zshrc\n'
    ;;
esac

log "codev v${version} prêt. Lance : codev docs"
