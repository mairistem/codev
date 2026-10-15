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
  Rewrite as a plain function a newcomer can read without the plan: name the
  threshold (`const PREMIUM_THRESHOLD_CENTS = 1000; // 10 euros`), test it
  with a comparison (`p.cents >= PREMIUM_THRESHOLD_CENTS`), and push into two
  explicit arrays returned as a typed result, e.g.
  `function splitByPremium(prices: Price[]): { cheap: Price[]; premium: Price[] }`
  with a `for…of` loop or two `filter` calls. Name the parameter `prices`, not
  `ps`/`a`/`p`. Where the reader stumbles today: `(p.cents >> 4) & 0x3e8` cannot
  be read as "10 euros or more" — 0x3e8 is 1000 used as a bit mask on cents/16,
  so the shape hides what the rule actually does (e.g. 160 cents gives
  `10 & 1000 = 8`, truthy, so it lands in "hi"; the developer should check this
  against the proposal's "10 euros and more"). The comma operator, the
  `||=` lazy init inside an index expression, and the magic keys "hi"/"lo" in a
  `Record<string, Price[]>` (which also means a missing bucket is `undefined`
  for callers) add to the cost; "cheap"/"premium" from the doc comment are the
  names the result should carry.
history:
  - status: open
    by: newcomer
```
