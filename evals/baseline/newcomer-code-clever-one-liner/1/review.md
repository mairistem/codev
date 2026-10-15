# Review record: group-prices

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### newcomer

#### R-newcomer-1 `bucket` packs a bit-trick threshold, an assignment and a push into one unreadable line

```yaml
severity: medium
confidence: high
location: "src/pricing.ts:9-10"
evidence: |
  export const bucket = (ps: Price[]) =>
    ps.reduce<Record<string, Price[]>>((a, p) => ((a[(p.cents >> 4) & 0x3e8 ? "hi" : "lo"] ||= []).push(p), a), {});
recommendation: >-
  Replace the one-liner with a plain loop over a named threshold, e.g.
  `const PREMIUM_THRESHOLD_CENTS = 1000;` and
  `function splitByPriceBand(prices: Price[]): { cheap: Price[]; premium: Price[] }`
  that pushes each price into `premium` when `price.cents >= PREMIUM_THRESHOLD_CENTS`,
  else into `cheap`. A newcomer cannot tell from `(p.cents >> 4) & 0x3e8` where
  "premium" starts (the doc comment does not say; the 10-euro rule lives only in
  the proposal), and the dense shape hides that the mask is not a threshold at all:
  a 128-cent price gives `(128 >> 4) & 0x3e8 = 8`, so it lands in "hi", while
  1000 cents is the intended boundary. The comma operator returning `a`, the
  `||=` inside an index, the single-letter names `ps`/`a`/`p`, the untyped
  `"hi"`/`"lo"` keys behind `Record<string, Price[]>` (callers must guess the keys,
  and a missing bucket is `undefined`), and a name `bucket` that does not say by
  what all add to the cost; the simple version removes all of them at once.
history:
  - status: open
    by: newcomer
```
