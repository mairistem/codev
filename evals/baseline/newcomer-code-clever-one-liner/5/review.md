# Review record: group-prices

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### newcomer

#### R-newcomer-1 `bucket` packs the premium rule into an unreadable bit-twiddling one-liner

```yaml
severity: medium
confidence: high
location: "src/pricing.ts:9-10"
evidence: |
  export const bucket = (ps: Price[]) =>
    ps.reduce<Record<string, Price[]>>((a, p) => ((a[(p.cents >> 4) & 0x3e8 ? "hi" : "lo"] ||= []).push(p), a), {});
recommendation: >-
  A developer who has not read the proposal cannot tell from this line where
  "premium" starts: the threshold is hidden behind a shift and a hex mask
  (`(p.cents >> 4) & 0x3e8`), with no named constant and no comment; the
  reduce relies on the comma operator and `||=` inside an index expression;
  the names `ps`, `a`, `p`, `bucket` and the untyped string keys `"hi"` /
  `"lo"` say nothing about cheap versus premium. A reader is likely to
  misread the condition as "1000 cents or more" (0x3e8 is 1000) — and it is
  not: 500 cents gives `(500 >> 4) & 0x3e8 === 8`, so a 5-euro product lands
  in "hi", which the shape of the line hides (correctness itself is the
  developer's to confirm). Write it plainly: a named constant
  `PREMIUM_THRESHOLD_CENTS = 1000` ("10 euros", from the proposal), a named
  function such as `splitByPriceBand(prices)` returning
  `{ cheap: Price[]; premium: Price[] }`, built with two `filter` calls or a
  `for … of` loop on `price.cents >= PREMIUM_THRESHOLD_CENTS`.
history:
  - status: open
    by: newcomer
```
