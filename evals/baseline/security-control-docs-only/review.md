# Review record: document-token-rotation

<!-- codev-review: 1 -->

## code.review

### security

#### R-security-1 Documented revocation guarantee has no implementation in this repository

```yaml
severity: medium
confidence: low
location: "README.md:19-20"
evidence: |
  3. Revoke the old token from the same page. Requests that still carry it
     are refused with `401 Unauthorized`.
recommendation: Point the section at the code that enforces token revocation (or the service that owns it), or drop the guarantee until it is backed by an implementation and a test; an integrator relying on revocation after an exposure must not be told it works if it does not.
history:
  - status: open
    by: security
```

The repository's code (`src/api.rs`, `src/db.rs`, `src/lib.rs`) holds no
token, authentication or revocation logic, and no "Settings → API tokens"
page; the change touches no code. The README thus promises a security
control — revoked tokens refused with 401 — that nothing in this change or
in the repository shows. Confidence is low because the control may live in
another service the reviewer could not see.

#### R-security-2 Non-expiring tokens documented without a periodic rotation recommendation

```yaml
severity: low
confidence: high
location: "README.md:13-14"
evidence: |
  API tokens do not expire. Rotate a token when a person who knew it leaves
  the team, or when it may have been exposed:
recommendation: Recommend a periodic rotation as well (for example every 90 days), since a leak that goes unnoticed is otherwise valid forever; consider tracking token expiry as a follow-up for the service itself.
history:
  - status: open
    by: security
```

Long-lived credentials (CWE-262) are only rotated on a known trigger, so an
unnoticed exposure stays exploitable indefinitely. The secret-storage
advice on line 17 ("never into source code") is sound.

### newcomer

Disabled by configuration.
