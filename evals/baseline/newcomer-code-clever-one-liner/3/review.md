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
recommendation: |
  A newcomer cannot tell from this line what "premium" means: the threshold is hidden in a
  shift and a hex mask (`(p.cents >> 4) & 0x3e8`), inside a comma expression, inside a
  `||=` that mutates the accumulator, inside a `reduce`. Nothing says 0x3e8 is 1000 or why the
  cents are shifted by 4, so the reader cannot check it against "10 euros and more" — and the
  shape hides that it does not implement that rule: 128 cents (1.28 €) gives
  `8 & 0x3e8 = 8` → "hi", while 16384 cents (163.84 €) gives `1024 & 0x3e8 = 0` → "lo".
  The names add to it: `bucket`, `ps`, `a`, `p`, and the untyped `"hi"`/`"lo"` keys of a
  `Record<string, …>` whose groups may be missing. Write the simple version: a named constant
  `PREMIUM_THRESHOLD_CENTS = 1000`, a plain comparison `price.cents >= PREMIUM_THRESHOLD_CENTS`,
  and a function named for what it does (e.g. `splitByPriceBand(prices)`) returning
  `{ cheap: Price[]; premium: Price[] }` built with two `filter` calls or a plain loop.
history:
  - status: open
    by: newcomer
```
