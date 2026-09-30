# Security Policy

## Supported versions

| Version | Supported |
|---------|-----------|
| 0.x     | Latest minor release only |

codev is pre-1.0: fixes land on the latest `0.x` release, and users are
expected to upgrade. Once codev reaches 1.0, the most recent major version
will be actively supported, and this table will be updated.

## Reporting a vulnerability

**Please do not report security vulnerabilities through public GitHub
issues, discussions, or pull requests.**

Use GitHub's private vulnerability reporting instead:

1. Go to [github.com/mairistem/codev/security/advisories/new](https://github.com/mairistem/codev/security/advisories/new).
2. Describe the vulnerability, a reproducible scenario, and the expected
   impact.

This channel is private, tracked, and allows coordinating a CVE when
appropriate.

## What to expect

- **Acknowledgement within 72 hours.** If you have not heard back by then,
  open a minimal public issue titled "Pending security report" — without any
  detail — so the maintainers know a private report is waiting.
- An assessment and, when confirmed, a fix timeline shared with you.
- Credit in the release notes and advisory, unless you prefer to remain
  anonymous.

Please give us a reasonable amount of time to release a fix before any
public disclosure.

## Scope

In scope:

- The `codev` binary (crates `codev-core`, `codev-engine`, `codev-agents`,
  `codev-cli`), including how it reads inherited sources from local paths
  and remote git repositories.
- The installers `install.sh` and `install.ps1` (download chain, SHA-256
  verification).
- The release pipeline (`.github/workflows/release.yml`).
- The skills generated into `.claude/skills/`, in particular the tool
  permissions they grant to Claude Code.

Out of scope:

- Vulnerabilities in third-party dependencies with no demonstrated impact on
  codev — please report those upstream.
- Behavior of Claude Code itself or of third-party MCP servers.
- Usage issues that are not security vulnerabilities — please open a regular
  issue.
