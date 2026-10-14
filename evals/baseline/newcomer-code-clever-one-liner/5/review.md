# Review record: group-prices

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### newcomer

#### R-newcomer-1 `bucket` packs its threshold into a bitwise one-liner nobody can read
```yaml
severity: medium
confidence: high
location: "src/pricing.ts:9-10"
evidence: |
  export const bucket = (ps: Price[]) =>
    ps.reduce<Record<string, Price[]>>((a, p) => ((a[(p.cents >> 4) & 0x3e8 ? "hi" : "lo"] ||= []).push(p), a), {});
recommendation: >-
  Replace the one-liner with a plain function: a named constant
  `PREMIUM_THRESHOLD_CENTS = 1000` (10 euros, the premium band of the
  proposal), a loop or two `filter` calls comparing `price.cents >=
  PREMIUM_THRESHOLD_CENTS`, and an explicit return type such as
  `{ cheap: Price[]; premium: Price[] }` instead of
  `Record<string, Price[]>` keyed by the bare strings "hi" / "lo". Name the
  parameters (`prices`, `groups`, `price`) instead of `ps`, `a`, `p`. A
  reader cannot tell from `(p.cents >> 4) & 0x3e8` where the premium band
  starts — it is not "10 euros and more": 500 cents gives
  `31 & 1000 = 8`, truthy, so a 5-euro price lands in "hi", while the
  docstring promises a cheap/premium split. The shape is what hides this;
  the simple comparison would make the threshold, and any defect in it,
  visible. Callers also cannot know the keys without reading the
  expression, and a group with no price is absent from the result rather
  than empty.
history:
  - status: open
    by: newcomer
```
