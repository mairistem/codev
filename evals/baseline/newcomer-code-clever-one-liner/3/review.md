# Review record: group-prices

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### newcomer

#### R-newcomer-1 `bucket` packs the premium rule into an unreadable bit-twiddling one-liner

```yaml
severity: medium
confidence: high
location: "src/pricing.ts:8-10"
evidence: |
  export const bucket = (ps: Price[]) =>
    ps.reduce<Record<string, Price[]>>((a, p) => ((a[(p.cents >> 4) & 0x3e8 ? "hi" : "lo"] ||= []).push(p), a), {});
recommendation: >
  Rewrite `bucket` as a plain function a reader can follow without the plan:
  a named constant `PREMIUM_THRESHOLD_CENTS = 1000` (10 euros, the rule the
  proposal states), a readable comparison `price.cents >= PREMIUM_THRESHOLD_CENTS`,
  full names (`prices`, `price`, `groups` instead of `ps`, `p`, `a`), and a
  typed result such as `{ cheap: Price[]; premium: Price[] }` built with two
  `filter` calls or a simple loop — instead of the `reduce` with a comma
  expression and `||=` in a computed key. The doc comment speaks of "cheap"
  and "premium" while the keys are "hi" and "lo", and `Record<string, Price[]>`
  hides that a key may be missing when no price falls in it; aligning the
  names and the type removes both questions. Where a newcomer stumbles: the
  condition `(p.cents >> 4) & 0x3e8` — nothing says what the shift and the
  mask mean, nor that 0x3e8 is 1000, and a reader cannot tell whether it is
  meant to be "10 euros and more". It is not: 512 cents gives 32 & 1000 = 32,
  so a 5.12-euro price lands in "hi", while 16384 cents gives 1024 & 1000 = 0,
  so a 163.84-euro price lands in "lo" — the bit mask does not express a
  threshold at all. The dense shape is what hides this; the developer should
  confirm the intended rule when rewriting it.
history:
  - status: open
    by: newcomer
```
