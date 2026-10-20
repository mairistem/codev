---
id: security
name: { en: Security, fr: Sécurité }
at: [plan.challenge, code.review]
domains: [security]
question: Can an attacker abuse what this change adds?
model: sonnet
done_when:
  - Every input crossing a trust boundary is validated where it enters,
    with an allow-list where possible
  - Authorization is enforced on the server for every new operation, not
    only hidden in the client
  - No secret is written in code, logs, error messages or version control
  - Queries, commands and file paths are never built by concatenating
    untrusted input
  - Untrusted data is never deserialized into arbitrary types
  - Every new dependency is justified, maintained, and pinned by the lock
    file
# What suggests the change touches security: authentication and secret
# material, the dependency manifests (every new dependency is a supply
# chain decision), and calls that are injection or deserialization sinks.
triggers:
  - domain: security
    paths:
      - '**/auth/**'
      - '**/authn/**'
      - '**/authz/**'
      - '**/security/**'
      - '**/crypto/**'
      - '**/*login*'
      - '**/*password*'
      - '**/*secret*'
      - '**/*permission*'
      - '**/.env'
      - '**/.env.*'
      - '**/*.pem'
      - '**/*.key'
    manifests:
      - Cargo.toml
      - package.json
      - pyproject.toml
      - requirements.txt
      - Pipfile
      - go.mod
      - pom.xml
      - build.gradle
      - build.gradle.kts
      - Gemfile
      - composer.json
    patterns:
      - '(?i)\b(password|passwd|passphrase|secret[_-]?key|client[_-]?secret|api[_-]?key|private[_-]?key|access[_-]?token|refresh[_-]?token|auth[_-]?token|bearer)\b'
      - '(?i)\b(md5|sha1)\s*\('
      - '\b(eval|exec)\s*\('
      - '\b(innerHTML|outerHTML|dangerouslySetInnerHTML)\b'
      - '\bpickle\.loads?\s*\('
      - '\byaml\.load\s*\('
      - 'shell\s*=\s*True'
      - '\bchild_process\b'
      - '\bunsafe\s*\{'
      - 'Command::new\(\s*"(sh|bash|cmd|powershell)"'
---

## Scope

Look at what the change adds or modifies: entry points (HTTP handlers,
CLI arguments, message consumers, file readers), authorization checks,
secrets, calls to interpreters (SQL, shell, templates), deserialization,
and new dependencies. Leave aside code the change does not touch, purely
theoretical attacks with no reachable entry point, and style.

Do not report:

- a hardening practice — an extra header, a shorter lifetime, a
  stricter allow-list — when you cannot name an attack the change makes
  possible without it;
- a protection the plan defers in its Non-Goals, unless the change is
  unsafe to ship without it; then say which attack the deferral opens;
- documentation advice as a vulnerability: a procedure that could be
  safer is `low`, unless following it as written exposes data or a
  secret.

## Signals

- **Missing validation at a boundary**: a request field used as is.
  Example: `limit` from the query string passed straight to the database.
- **Client-side only authorization**: a button hidden in the UI, the
  endpoint left open. Example: `DELETE /users/{id}` with no ownership check.
- **Insecure direct object reference**: an id from the request selects a
  record without checking it belongs to the caller. Example:
  `GET /invoices/42` returns any customer's invoice.
- **SQL injection**: a query built by string formatting. Example:
  `"SELECT * FROM users WHERE name = '" + name + "'"`.
- **Command injection**: a shell invoked with user text. Example:
  `system("convert " + filename)`.
- **Path traversal**: a user-supplied name joined to a directory. Example:
  `open(base + "/" + request.file)` with `../../etc/passwd`.
- **Unsafe deserialization**: a format that can instantiate types, fed
  untrusted bytes. Example: `pickle.loads(body)`.
- **Secret in the wrong place**: a token in source, a test fixture, a log
  line or an exception message. Example: `log.info("auth header: %s", h)`.
- **Weak or home-made cryptography**: MD5 or SHA-1 for passwords, a fixed
  IV, a custom token format. Example: `sha1(password)`.
- **Server-side request forgery**: the server fetches a URL the user
  chose. Example: a "preview link" feature reaching `http://169.254.169.254/`.
- **Risky dependency**: a new package, unmaintained or unpinned. Example:
  a version range `*` in the manifest.

## By stack

- **Rust**: `unsafe` blocks without a stated invariant;
  `std::process::Command` built from user input passed to `sh -c`;
  `format!` into SQL instead of bound parameters (`.bind()` in sqlx,
  `params!` in rusqlite); path joins without canonicalizing and checking the prefix;
  `cargo audit` for known advisories.
- **TypeScript/JavaScript**: `innerHTML`, `dangerouslySetInnerHTML` or
  `eval` with user data (cross-site scripting); template literals in SQL;
  `child_process.exec` with user input (prefer `execFile` with an argument
  array); prototype pollution through deep merges of request bodies;
  `npm audit` for known advisories.
- **Python**: `pickle`, `yaml.load` without `SafeLoader`, `eval`/`exec` on
  input; `subprocess` with `shell=True`; f-strings in SQL instead of
  parameters; `os.path.join` with an absolute user path, which discards
  the base.
- **Go**: `fmt.Sprintf` into SQL instead of `db.Query(q, args...)`;
  `exec.Command("sh", "-c", ...)` with input; `filepath.Join` without
  checking the result stays under the base; `text/template` where
  `html/template` is needed for HTML; `govulncheck` for known
  vulnerabilities.
- **Java**: `Statement` with concatenation instead of
  `PreparedStatement`; `ObjectInputStream.readObject` on untrusted data;
  XML parsers with external entities enabled (XXE); `Runtime.exec` with a
  single concatenated string.

## References

- OWASP Top 10 — the most critical web application security risks.
- OWASP Application Security Verification Standard (ASVS) — verifiable
  requirements per level.
- OWASP Cheat Sheet Series — remediation per topic (input validation,
  SQL injection prevention, secrets management).
- CWE (Common Weakness Enumeration, MITRE) — for example CWE-89 (SQL
  injection), CWE-78 (OS command injection), CWE-22 (path traversal),
  CWE-79 (cross-site scripting), CWE-502 (deserialization of untrusted
  data), CWE-798 (hard-coded credentials), CWE-862 (missing
  authorization), CWE-918 (server-side request forgery).

## Reporting

One line per finding: the location, the attack in one sentence (who sends
what, and what they get), the severity (high when exploitable remotely
without privileges or when it exposes data or secrets; medium when it needs
privileges or an unlikely condition; low otherwise), and the fix. Cite the
CWE when it is certain. Never report a vulnerability you cannot point to in
the change: a finding names who sends what, through which entry point,
and what they get. An attack you cannot complete from what you read — it needs a
caller, a deployment or a configuration you could not see — is recorded
with confidence `low`.
