# Review record: group-prices

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### newcomer

#### R-newcomer-1 `bucket` packs the premium rule into an unreadable bitwise one-liner

```yaml
severity: medium
confidence: high
location: "src/pricing.ts:9-10"
evidence: |
  export const bucket = (ps: Price[]) =>
    ps.reduce<Record<string, Price[]>>((a, p) => ((a[(p.cents >> 4) & 0x3e8 ? "hi" : "lo"] ||= []).push(p), a), {});
recommendation: |
  Rewrite as a plain function a reader can follow without the plan: a named
  constant for the threshold (`const PREMIUM_THRESHOLD_CENTS = 1000; // 10 euros`),
  a named predicate (`price.cents >= PREMIUM_THRESHOLD_CENTS`), descriptive names
  instead of `ps`, `a`, `p`, and a typed result whose keys match the doc comment
  (`{ cheap: Price[]; premium: Price[] }` instead of `Record<string, Price[]>` with
  "hi"/"lo"), built with a simple loop or two `filter` calls rather than a
  comma-expression inside `reduce` with `||=`. A newcomer cannot tell from
  `(p.cents >> 4) & 0x3e8` what the rule is: 0x3e8 is 1000, which suggests
  "10 euros", but a shift-and-mask is not a comparison — 500 cents (31 & 1000 = 8)
  and 128 cents land in "hi", while 16384 cents (1024 & 1000 = 0) lands in "lo".
  The shape hides this; the readable version makes the intended `>= 1000` check
  explicit. Since the result also has no "hi"/"lo" key when a band is empty, the
  typed version should initialise both arrays.
history:
  - status: open
    by: newcomer
```
