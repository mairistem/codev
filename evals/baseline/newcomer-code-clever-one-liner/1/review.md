# Review record: group-prices

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### newcomer

#### R-newcomer-1 `bucket` packs its premium rule into a bitwise one-liner
```yaml
severity: medium
confidence: high
location: "src/pricing.ts:9-10"
evidence: |
  export const bucket = (ps: Price[]) =>
    ps.reduce<Record<string, Price[]>>((a, p) => ((a[(p.cents >> 4) & 0x3e8 ? "hi" : "lo"] ||= []).push(p), a), {});
recommendation: >-
  A newcomer reading the docstring "Splits prices into the cheap and the
  premium ones" cannot recover the premium rule from
  `(p.cents >> 4) & 0x3e8` without the plan (which says premium means
  10 euros and more): a shift, a hex mask and a ternary nested inside an
  index, inside a `||=`, inside a comma expression in a `reduce` callback.
  The shape hides what it computes — it is not a threshold at all: 200
  cents gives `(200 >> 4) & 0x3e8 === 8`, so a 2-euro price lands in "hi",
  while 1000 cents lands there too. Rewrite it as a plain loop or two
  `filter`s on a named constant, e.g.
  `const PREMIUM_MIN_CENTS = 1000;` and
  `premium = prices.filter(p => p.cents >= PREMIUM_MIN_CENTS)`, returning a
  typed `{ cheap: Price[]; premium: Price[] }` instead of a
  `Record<string, Price[]>` keyed by the magic strings "hi"/"lo" (whose
  keys are missing when a band is empty); and name it and its parameter
  for what they are (`splitByPriceBand(prices)` rather than `bucket(ps)`,
  `a`, `p`).
history:
  - status: open
    by: newcomer
```
