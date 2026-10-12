---
id: privacy
name: { en: Privacy, fr: Données personnelles }
at: [plan.challenge, code.review]
domains: [personal-data]
question: Does this change collect, keep or share personal data only as it must?
done_when:
  - Every new piece of personal data has a stated purpose and lawful basis,
    and no more is collected than that purpose needs
  - Its retention period and its deletion are defined, backups included
  - The data subject's rights still work — access, rectification, erasure
    and portability reach the new data
  - No personal data goes to logs, analytics, error trackers or URLs
  - Every new processor or transfer outside the EEA is identified
---

## Scope

Look at personal data the change adds, copies, exposes or keeps: new
fields, tables, events, logs, exports, third-party calls and caches.
Personal data is any information relating to an identified or identifiable
person — an e-mail address, an IP address, a device or customer
identifier, a location. Leave aside anonymous aggregates, and legal
questions the code cannot answer (the controller decides the lawful basis;
the change states it).

## Signals

- **Collected "just in case"**: a field with no use in the change.
  Example: a birth date stored at sign-up for a newsletter.
- **No retention rule**: a new table or bucket with no expiry or purge.
  Example: raw uploads kept forever after processing.
- **Erasure that misses copies**: account deletion that leaves the new
  data in another table, a search index, a cache or an export.
- **Personal data in logs or telemetry**: Example:
  `log.info("login failed for {email}")`, or an analytics event carrying
  the user's name.
- **Personal data in URLs**: query strings end up in access logs, browser
  history and referrer headers. Example: `/reset?email=jane@example.com`.
- **Special categories without a guard**: health, biometric or ethnic
  data, political opinions, sexual orientation, religious beliefs — they
  need an explicit legal ground.
- **Silent sharing**: a new SDK, webhook or vendor receiving user data.
  Example: sending full user records to a support tool when an id would do.
- **Identifiers that are not pseudonymous**: an e-mail address used as a
  key in a dataset where an opaque id would do.

## By stack

- **Rust**: `#[derive(Debug)]` on a struct holding personal data prints it
  in logs and panics — implement `Debug` by hand or wrap the field;
  `tracing` spans recording request fields by default.
- **TypeScript/JavaScript**: `localStorage` holding personal data
  (readable by any script on the origin); analytics SDKs capturing page
  URLs or form fields automatically; `console.log` of whole user objects.
- **Python**: logging a model instance whose `__repr__` lists every field;
  Django or SQLAlchemy models serialized whole into API responses;
  `send_default_pii=True` in the Sentry SDK.
- **Go**: `%+v` in log calls printing whole structs; JSON encoding a struct
  with personal fields lacking `json:"-"`.
- **Java**: Lombok `@Data` or `@ToString` generating a `toString` that
  prints every field; entities returned directly by REST controllers.

## References

- GDPR — Regulation (EU) 2016/679 (RGPD in French): Art. 5 (principles:
  lawfulness, fairness and transparency, purpose limitation, data
  minimisation, accuracy, storage limitation, integrity and
  confidentiality, accountability), Art. 6 (lawfulness of processing),
  Art. 9 (special categories), Art. 15–20 (rights of access,
  rectification, erasure, restriction, portability), Art. 25 (data
  protection by design and by default), Art. 28 (processors), Art. 30
  (records of processing), Art. 32 (security of processing), Art. 35 (data
  protection impact assessment), Chapter V (transfers to third countries).
- CNIL — guidance for developers (guide RGPD du développeur).
- ePrivacy Directive 2002/58/EC — consent for cookies and similar trackers.

## Reporting

One line per finding: the data, where it flows or stays, the principle at
stake (cite the GDPR article when it is one of the above), the severity
(high when personal data leaks to logs, third parties or the public, or
special-category data is processed without a guard; medium when retention
or erasure is undefined; low otherwise), and the fix. Raise a missing
lawful basis or impact assessment as a question for the team, not as a
code fix.
