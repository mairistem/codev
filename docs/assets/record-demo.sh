#!/bin/sh
# Records docs/assets/demo.gif, the demo shown in the README.
#
# 1. vhs plays docs/assets/demo.tape and renders the terminal on a deep-blue
#    Mairistem frame (docs/assets/demo-raw.gif, not committed);
# 2. ffmpeg stamps the white Mairistem logo in the bottom-left margin and
#    re-encodes the GIF with an optimized palette.
#
# Prerequisites: vhs, ffmpeg, rsvg-convert (librsvg), git, and a `codev`
# binary on PATH. Run from anywhere:
#
#   docs/assets/record-demo.sh

set -eu

cd "$(dirname "$0")/../.."
assets=docs/assets
work="$(mktemp -d)"
trap 'rm -rf "$work" "$assets/demo-raw.gif"' EXIT INT TERM

vhs "$assets/demo.tape"

# The logo sits in the 64 px bottom margin, aligned with the window's left edge.
rsvg-convert -h 40 "$assets/mairistem-logo-white.svg" -o "$work/logo.png"

ffmpeg -loglevel error -y -i "$assets/demo-raw.gif" -i "$work/logo.png" \
  -filter_complex "[0:v][1:v]overlay=x=64:y=main_h-overlay_h-12,split[a][b];[a]palettegen=stats_mode=diff[p];[b][p]paletteuse=dither=none:diff_mode=rectangle" \
  -fps_mode passthrough -loop 0 "$assets/demo.gif"

echo "Wrote $assets/demo.gif"
