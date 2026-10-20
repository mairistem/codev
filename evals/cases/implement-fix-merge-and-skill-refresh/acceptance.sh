#!/usr/bin/env bash
# Acceptance of fix-merge-and-skill-refresh: drives $CODEV as a user would.
set -u
export CODEV_NO_UPDATE_CHECK=1
check() { if "$@" >/dev/null 2>&1; then echo "ok $NAME"; else echo "not ok $NAME"; fi; }

git init -q . && git config user.email e@x && git config user.name e
"$CODEV" init --yes --no-detect >/dev/null 2>&1

mkdir -p _codev/specs/auth
cat > _codev/specs/auth/spec.md <<'S'
# Auth Specification

## Purpose

Sign users in.

## Requirements

### Requirement: Logout

The system SHALL end the session on logout.

#### Scenario: Logout ends the session

- **WHEN** the user logs out
- **THEN** the session ends

### Requirement: Login

The system SHALL sign a user in.

#### Scenario: Valid credentials

- **WHEN** the user gives valid credentials
- **THEN** they are signed in
S
"$CODEV" new change login-export >/dev/null 2>&1
C=_codev/changes/login-export
cat > $C/proposal.md <<'S'
# Proposal: Login and export

## Why

Users need a longer login and an export.

## What Changes

- Login checks the account is active.
- An export.

## Capabilities

### New Capabilities

### Modified Capabilities

- auth

### Removed Capabilities

## Impact

None.

## Domains

None

## Release note

None
S
mkdir -p $C/specs/auth
cat > $C/specs/auth/spec.md <<'S'
## MODIFIED Requirements

### Requirement: Login

The system SHALL sign a user in when the account is active, and refuse
an inactive account with a message that names the reason.

#### Scenario: Valid credentials

- **WHEN** the user gives valid credentials for an active account
- **THEN** they are signed in

#### Scenario: Inactive account

- **WHEN** the user gives valid credentials for an inactive account
- **THEN** they are refused, with the reason

## ADDED Requirements

### Requirement: Export

The system SHALL export a user's data on request.

#### Scenario: Export on request

- **WHEN** the user asks for an export
- **THEN** they receive their data
S
printf '# Tasks\n\n## 1. Do\n\n- [x] 1.1 Done, verified by a test\n' > $C/tasks.md
"$CODEV" sync --change login-export >/dev/null 2>&1
SPEC=_codev/specs/auth/spec.md
NAME=last-requirement-modified-kept-whole
if grep -q "an inactive account with a message that names the reason." "$SPEC" \
  && grep -q -- "- \*\*THEN\*\* they are refused, with the reason" "$SPEC" \
  && awk '/Requirement: Login/{l=NR} /Requirement: Export/{e=NR} END{exit !(l && e && l<e)}' "$SPEC" \
  && [ "$(grep -c 'Scenario: Valid credentials' "$SPEC")" = 1 ] \
  && [ "$(grep -v '^[[:space:]]*$' "$SPEC" | tail -1)" = "- **THEN** they receive their data" ]; then
  echo "ok $NAME"; else echo "not ok $NAME"; fi
cp "$SPEC" "$SPEC.first"
"$CODEV" sync --change login-export >/dev/null 2>&1
NAME=second-sync-changes-nothing check cmp -s "$SPEC" "$SPEC.first"
rm -f "$SPEC.first"

F=.claude/skills/codev-apply/SKILL.md
stamped() { sed -n 's/^  checksum: "\(.*\)"$/\1/p' "$F" | head -1; }
# The SHA-256 of the skill without its (first) checksum line, byte for byte.
unsealed_hash() { perl -0pe 's/^  checksum: .*\n//m' "$F" | shasum -a 256 | cut -d' ' -f1; }
NAME=skill-carries-a-checksum check test -n "$(stamped)"
NAME=checksum-matches-the-content check test "$(stamped)" = "$(unsealed_hash)"

echo "A line added by hand." >> "$F"
"$CODEV" update --json > update.json 2>/dev/null
NAME=hand-edit-of-a-checksummed-skill-preserved check grep -q "A line added by hand." "$F"

sed -i.bak 's/^  version: ".*"$/  version: "0.0.1"/' "$F" && rm -f "$F.bak"
"$CODEV" update >/dev/null 2>&1
NAME=hand-edit-on-another-version-rewritten check test "$(grep -c 'A line added by hand.' "$F")" = 0

"$CODEV" update > second.txt 2>&1
NAME=second-update-writes-nothing check grep -q "Nothing to do" second.txt
