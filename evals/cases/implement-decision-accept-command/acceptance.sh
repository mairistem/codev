#!/usr/bin/env bash
# Acceptance of decision-accept-command: drives $CODEV as a user would.
set -u
export CODEV_NO_UPDATE_CHECK=1
say() { if [ "$2" = 0 ]; then echo "ok $1"; else echo "not ok $1"; fi; }
check() { local name=$1; shift; "$@" >/dev/null 2>&1; say "$name" $?; }
body() { awk 'n>=2{print} /^---$/{n++}' "$1"; }

git init -q . && git config user.email e@x && git config user.name e
"$CODEV" init --yes --no-detect >/dev/null 2>&1
D=_codev/decisions

"$CODEV" decision new "A first choice" >/dev/null 2>&1
F=$(ls $D/0001-*.md 2>/dev/null | head -1)
check new-is-proposed grep -q '^status: proposed' "${F:-/none}"
check new-proposed-writes-no-seal sh -c "! grep -q \"'0001'\" $D/seal.yaml 2>/dev/null"
"$CODEV" decision new "A second choice" --json > second.json 2>/dev/null
check new-proposed-json-has-no-hash sh -c '! grep -q bodySha256 second.json && grep -q "\"0002\"" second.json'
"$CODEV" decision new "Settled choice" --status accepted >/dev/null 2>&1
check new-accepted-is-sealed sh -c "grep -q '^status: accepted' $D/0003-*.md && grep -q \"'0003'\" $D/seal.yaml"

printf '\n## Context\n\nWritten by hand before the acceptance.\n' >> "$F"
body "$F" > body.before
"$CODEV" decision accept 0001 --json > accept.json 2>/dev/null
check accept-moves-the-status grep -q '^status: accepted' "$F"
body "$F" > body.after
check accept-keeps-the-body cmp -s body.before body.after
check accept-seals-the-body grep -q "'0001'" $D/seal.yaml
check accept-json-gives-the-decision sh -c 'grep -q "\"id\": \"0001\"" accept.json && grep -q "\"status\": \"accepted\"" accept.json && grep -q "\"bodySha256\": \"sha256:" accept.json'
"$CODEV" validate --json > validate.json 2>/dev/null
check accept-then-validate-is-clean sh -c '! grep -q "decision_seal_mismatch\|decision_unsealed" validate.json'

cp "$F" accepted.copy
cp $D/seal.yaml seal.copy
"$CODEV" decision accept 0001 --json > twice.json 2>/dev/null
check accept-twice-is-refused sh -c 'grep -q decision_not_proposed twice.json && cmp -s accepted.copy '"$F"' && cmp -s seal.copy '"$D"'/seal.yaml'
"$CODEV" decision accept 9999 --json > unknown.json 2>/dev/null
check accept-of-an-unknown-id-is-refused grep -q unknown_decision_id unknown.json
