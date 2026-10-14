# Review record: group-prices

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### newcomer

#### R-newcomer-1 `bucket` packs its whole rule into an unreadable one-liner
```yaml
severity: medium
confidence: high
location: "src/pricing.ts:9-10"
evidence: |
  export const bucket = (ps: Price[]) =>
    ps.reduce<Record<string, Price[]>>((a, p) => ((a[(p.cents >> 4) & 0x3e8 ? "hi" : "lo"] ||= []).push(p), a), {});
recommendation: >-
  Rewrite it as a plain function a newcomer can read: a named constant for
  the premium threshold (e.g. `PREMIUM_THRESHOLD_CENTS`), a plain comparison
  `price.cents >= PREMIUM_THRESHOLD_CENTS` instead of the shift-and-mask,
  a loop or two `filter` calls instead of the `reduce` with a comma
  expression and `||=`, descriptive names (`prices`, `price`, `groups`)
  instead of `ps`, `a`, `p`, and a return type such as
  `{ cheap: Price[]; premium: Price[] }` instead of
  `Record<string, Price[]>` with the magic keys "hi"/"lo". The docstring
  says "cheap and premium", but the code never says where the line is: a
  reader takes `0x3e8` (1000) for a threshold, while `(cents >> 4) & 0x3e8`
  tests bits, not a magnitude — e.g. 128 cents gives `8 & 1000 = 8`, so
  "hi", while 16384 cents gives `1024 & 1000 = 0`, so "lo". The density of
  the line is what hides this; the readable version makes the intended
  threshold explicit and lets the developer confirm it.
history:
  - status: open
    by: newcomer
```
