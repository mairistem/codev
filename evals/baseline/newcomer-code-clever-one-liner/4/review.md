# Review record: group-prices

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### newcomer

#### R-newcomer-1 `bucket` hides its premium threshold in a bit-twiddling one-liner

```yaml
severity: medium
confidence: high
location: "src/pricing.ts:9-10"
evidence: |
  export const bucket = (ps: Price[]) =>
    ps.reduce<Record<string, Price[]>>((a, p) => ((a[(p.cents >> 4) & 0x3e8 ? "hi" : "lo"] ||= []).push(p), a), {});
recommendation: >-
  Rewrite as a plain function a reader can follow without the plan: a named
  constant for the premium threshold (e.g. `PREMIUM_THRESHOLD_CENTS`) with a
  comparison `price.cents >= PREMIUM_THRESHOLD_CENTS` instead of
  `(p.cents >> 4) & 0x3e8`, two explicit arrays (or a `{ cheap, premium }`
  result type instead of `Record<string, Price[]>` with "hi"/"lo" keys),
  a simple loop instead of the comma-expression reducer with `||=`, and
  descriptive names instead of `ps`, `a`, `p`. A newcomer reading the doc
  comment "Splits prices into the cheap and the premium ones" cannot tell
  from line 10 where "premium" starts: `>> 4` divides by 16 and the mask
  `0x3e8` (1000) tests scattered bits rather than a threshold, so they will
  likely read it as "cents above some value" when it is not a monotonic
  comparison (e.g. 16000 cents gives 1000 & 1000 → "hi", but 16384 cents
  gives 1024 & 1000 = 0 → "lo"). The developer should confirm the intended
  rule; the readable version makes it visible. Also a bucket key is missing
  entirely when no price falls in it, which the `Record<string, …>` type
  does not tell the caller.
history:
  - status: open
    by: newcomer
```
