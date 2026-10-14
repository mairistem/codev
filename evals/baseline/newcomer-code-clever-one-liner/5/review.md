# Review record: group-prices

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### newcomer

#### R-newcomer-1 `bucket` packs the premium rule into an unreadable one-liner that hides its threshold

```yaml
severity: medium
confidence: high
location: "src/pricing.ts:9-10"
evidence: |
  export const bucket = (ps: Price[]) =>
    ps.reduce<Record<string, Price[]>>((a, p) => ((a[(p.cents >> 4) & 0x3e8 ? "hi" : "lo"] ||= []).push(p), a), {});
recommendation: |
  Rewrite `bucket` as a plain function a newcomer can read without the plan:
  a named constant `PREMIUM_THRESHOLD_CENTS = 1000` (10 euros, the premium
  band), a comparison `price.cents >= PREMIUM_THRESHOLD_CENTS`, a return type
  with named keys (`{ cheap: Price[]; premium: Price[] }`, matching the
  docstring) instead of `Record<string, Price[]>` with "hi"/"lo", and full
  names instead of `ps`, `a`, `p`. A reader stumbles on line 10: the
  comma-operator arrow, the `||=` push inside an index expression and the
  bit shift masked with `0x3e8` cannot be decoded without knowing that
  premium means 10 euros and more; nothing says 0x3e8 is 1000 cents, and a
  reader who works it out would read the expression as "cents >= 1000"
  when it is a bitwise test (`(cents >> 4) & 1000`): 999 cents or 200 cents
  land in "hi", 100 cents in "lo". The dense shape is what hides that the
  rule does not match the stated premium band; the developer should confirm
  the intended rule when simplifying, since that part changes behavior.
history:
  - status: open
    by: newcomer
```
