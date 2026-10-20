#!/usr/bin/env bash
# Acceptance of forward-compatible-config: drives $CODEV as a user would.
set -u
export CODEV_NO_UPDATE_CHECK=1
# No update cache from the operator's machine.
export HOME="$PWD/.home" XDG_CACHE_HOME="$PWD/.home/cache"
mkdir -p "$HOME"
say() { if [ "$2" = 0 ]; then echo "ok $1"; else echo "not ok $1"; fi; }
check() { local name=$1; shift; "$@" >/dev/null 2>&1; say "$name" $?; }

# A fresh project with one active change, its configuration then rewritten.
project() {
  rm -rf p && mkdir p && cd p || exit 1
  git init -q . && git config user.email e@x && git config user.name e
  "$CODEV" init --yes --no-detect >/dev/null 2>&1
  "$CODEV" new change some-change >/dev/null 2>&1
}
config() { printf '%s\n' "$@" > _codev/config.yaml; }

project
config "schema: spec-driven" 'codev_version: ">=0.9"' "review_policy: strict"
"$CODEV" status --json > out.txt 2>&1; code=$?
check unknown-top-level-key-warns-with-the-requirement sh -c "[ $code = 0 ] && grep -q unknown_config_key out.txt && grep -q review_policy out.txt && grep -q '>=0.9' out.txt"
cd ..

project
config "shema: spec-driven"
"$CODEV" status > out.txt 2>&1; code=$?
check misspelled-top-level-key-warns sh -c "[ $code = 0 ] && grep -q shema out.txt && grep -q 'newer codev' out.txt"
cd ..

project
config "schema: spec-driven" "workflows: [propose, apply]" "mcp:" "  jira_tool: mcp__a__getJiraIssue" "  design_tool: mcp__b__get"
# --force: the skills init rendered without the Jira tool read as edited.
"$CODEV" update --force > out.txt 2>&1; code=$?
check unknown-mcp-key-keeps-the-update sh -c "[ $code = 0 ] && grep -q mcp__a__getJiraIssue .claude/skills/codev-propose/SKILL.md && grep -q mcp.design_tool out.txt"
cd ..

project
config "schema: spec-driven" 'codev_version: ">=0.9"' "changelog:" "  since: v1.0.0"
"$CODEV" status > out.txt 2>&1; code=$?
check unknown-changelog-key-fails-quoting-the-requirement sh -c "[ $code != 0 ] && grep -q since out.txt && grep -q '>=0.9' out.txt && grep -q 'codev self-update' out.txt"
cd ..

project
rm -rf _codev/changes/some-change
config "schema: spec-driven" "rules:" "  specs: [one rule]" "review_policy: strict"
"$CODEV" validate --strict --json > strict.json 2>&1
check strict-validation-reports-a-config-item sh -c "grep -q '\"kind\": \"config\"' strict.json && grep -q unknown_config_key strict.json && grep -q '\"line\": 4' strict.json"
"$CODEV" validate > plain.txt 2>&1; code=$?
check non-strict-validation-warns-and-succeeds sh -c "[ $code = 0 ] && grep -q unknown_config_key plain.txt"
"$CODEV" doctor > doctor.txt 2>&1
check doctor-reports-the-unknown-key sh -c "grep -q config.keys doctor.txt && grep -q review_policy doctor.txt"
cd ..

project
config "schema: spec-driven"
"$CODEV" validate --all --json > clean.json 2>&1
check clean-configuration-has-no-config-item sh -c "! grep -q '\"kind\": \"config\"' clean.json"
config "schema: spec-driven" 'codev_version: "<=0.0.9"'
"$CODEV" doctor > doctor.txt 2>&1
check doctor-fixes-a-codev-above-the-requirement grep -q "codev self-update --version 0.0.9" doctor.txt
cd ..
