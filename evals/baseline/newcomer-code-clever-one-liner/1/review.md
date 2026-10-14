# Review record: group-prices

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### newcomer

#### R-newcomer-1 `bucket` packs an unexplained bitmask threshold into a dense one-liner

```yaml
severity: medium
confidence: high
location: "src/pricing.ts:9-10"
evidence: |
  export const bucket = (ps: Price[]) =>
    ps.reduce<Record<string, Price[]>>((a, p) => ((a[(p.cents >> 4) & 0x3e8 ? "hi" : "lo"] ||= []).push(p), a), {});
recommendation: >-
  A developer who has not seen the plan cannot answer "at what price is a
  price premium?": the rule is hidden in `(p.cents >> 4) & 0x3e8`, a shift
  and a bitmask with a magic hex value, which reads as a threshold but is
  not one (for example 128 cents gives 8 & 1000 = 8, "hi", while 256 cents
  gives 16 & 1000 = 0, "lo" — the grouping is not monotonic in price). The comma operator,
  `||=` inside an index expression and the reduce accumulator fold all of
  this into one line, and the names `bucket`, `ps`, `a`, `p`, "hi"/"lo"
  and the `Record<string, Price[]>` return type do not match the doc
  comment's "cheap and premium". Rewrite it as a named function with a
  named constant for the threshold (e.g. `PREMIUM_THRESHOLD_CENTS`), a
  plain comparison (`p.cents >= PREMIUM_THRESHOLD_CENTS`), a loop or two
  `filter` calls, and an explicit return type such as
  `{ cheap: Price[]; premium: Price[] }`; if the bit arithmetic is
  intended, a comment must say why — otherwise the developer should
  check the rule against the spec, since the shape hides what it computes.
history:
  - status: open
    by: newcomer
```
