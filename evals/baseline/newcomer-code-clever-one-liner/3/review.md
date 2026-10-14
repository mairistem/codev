# Review record: group-prices

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### newcomer

#### R-newcomer-1 `bucket` packs the premium rule into an unreadable bit-trick one-liner
```yaml
severity: medium
confidence: high
location: "src/pricing.ts:9-10"
evidence: |
  export const bucket = (ps: Price[]) =>
    ps.reduce<Record<string, Price[]>>((a, p) => ((a[(p.cents >> 4) & 0x3e8 ? "hi" : "lo"] ||= []).push(p), a), {});
recommendation: >-
  A reader cannot tell from this line what makes a price premium: the rule
  "10 euros and more" (1000 cents, from the proposal) is hidden behind a shift
  and a bitmask, `(p.cents >> 4) & 0x3e8`, which reads like a threshold of 1000
  but is not a comparison at all. The shape hides a wrong result: a 500-cent
  price gives `31 & 1000 = 8`, truthy, so it lands in "hi", while the docstring
  promises cheap vs premium. The comma expression inside `reduce`, the `||=`
  side effect, the one-letter names (`ps`, `a`, `p`), the `hi`/`lo` keys that
  do not match "cheap"/"premium", and the untyped `Record<string, ...>` return
  add to it. Write the simple version: a named constant
  `PREMIUM_THRESHOLD_CENTS = 1000`, a plain loop or two `filter` calls
  comparing `price.cents >= PREMIUM_THRESHOLD_CENTS`, and a return type
  `{ cheap: Price[]; premium: Price[] }` — fixing the misclassification is the
  developer's call, as it changes behavior.
history:
  - status: open
    by: newcomer
```
