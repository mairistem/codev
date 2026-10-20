---
id: performance
name: { en: Performance, fr: Performance }
at: [plan.challenge, code.review]
domains: [performance]
question: Does this change stay fast as data and load grow?
model: sonnet
done_when:
  - No query or remote call runs once per item of a loop when one call
    can serve the whole set
  - Every result set that can grow is bounded or paginated
  - No blocking I/O runs on a hot path or an event loop
  - Loops over growing data do not allocate or copy needlessly
  - Every new dependency's cost — size, startup, transitive packages — is
    weighed against what it saves
  - A claim of speed is backed by a measurement, not a guess
# What suggests the change touches performance: queries and the code that
# runs them, concurrency, caches and pagination.
triggers:
  - domain: performance
    paths:
      - '**/*.sql'
      - '**/queries/**'
      - '**/repositories/**'
      - '**/repository/**'
      - '**/benches/**'
    patterns:
      - '(?i)\b(SELECT\s.+\sFROM|INSERT\s+INTO|UPDATE\s+\w+\s+SET|DELETE\s+FROM)\b'
      - '\b(Promise\.all|asyncio\.gather|tokio::spawn|thread::spawn|rayon::|ThreadPoolExecutor|parallelStream|go func)\b'
      - '(?i)\b(cache|caching|memoi[sz]e|lru)\b'
      - '(?i)\b(paginat\w*|page[-_]?size|batch[-_]?size)\b'
---

## Scope

Look at code paths the change adds or makes hotter: request handlers,
loops over collections that grow with users or data, queries, caches,
startup, and bundle or binary size. Leave aside micro-optimizations on
cold paths, and code that runs once per process.

Do not report:

- a constant-factor saving — a copy, a clone, a string formatted per item,
  one more query per request — whose cost does not grow faster than the
  work the code already does for each item;
- a back-end dependency's build time or binary size: only a front-end
  bundle's size reaches users;
- a missing index you cannot see: when the schema is not in the
  repository, say what to check with confidence `low`.

Work repeated for each item stays a finding when that work is expensive
compared with the item: a query, a remote call, compiling a pattern,
parsing a file, opening a connection.

## Signals

- **N+1 queries**: one query for the list, one per item. Example:
  `for order in orders: order.customer.name` with lazy loading.
- **Remote call in a loop**: Example: an HTTP request per id where the API
  accepts a batch.
- **Unbounded result**: Example: `SELECT * FROM events` with no `LIMIT`, or
  an endpoint returning every row.
- **Missing index**: a new filter or sort on a column with no index.
- **Quadratic work**: a lookup in a list inside a loop. Example:
  `if x in seen_list` where a set would do.
- **Blocking on a hot path**: synchronous file or network I/O inside an
  async handler or a UI thread.
- **Allocation in a tight loop**: building a string or a collection per
  iteration where one buffer would do.
- **Cache without a bound**: a map that grows forever, or no invalidation.
- **Heavy dependency for a small need**: a whole library for one function.

## By stack

- **Rust**: `.clone()` or `.to_string()` inside a loop where a borrow
  works; `collect()` into a `Vec` only to iterate again; blocking calls
  (`std::fs`, `std::thread::sleep`) inside an async task — use the
  runtime's async or `spawn_blocking`; `Vec` without `with_capacity` when
  the size is known.
- **TypeScript/JavaScript**: `await` inside a `for` loop where
  `Promise.all` would do; synchronous `fs.*Sync` in a server handler;
  re-renders caused by new object or function props on every render;
  importing a whole library (`lodash`) for one function in a front-end
  bundle.
- **Python**: ORM lazy loading in loops (`select_related` /
  `prefetch_related` in Django, `selectinload` in SQLAlchemy); string
  concatenation in a loop instead of `"".join`; blocking calls inside
  `async def`.
- **Go**: a goroutine per item with no limit; `append` in a loop without
  preallocating with `make([]T, 0, n)`; `defer` inside a long loop;
  missing `context` deadline on outgoing calls.
- **Java**: lazy-loaded JPA associations in loops (fetch joins or entity
  graphs); string `+` in loops instead of `StringBuilder`; boxing in hot
  numeric code; unbounded thread pools.

## References

- Use the profiler of the stack rather than intuition: `perf` or
  `cargo flamegraph` (Rust), the browser's Performance panel and Lighthouse
  (front end), `cProfile` or `py-spy` (Python), `pprof` (Go), Java Flight
  Recorder (Java).
- The database's query plan (`EXPLAIN`) for every new query on a growing
  table.

## Reporting

One line per finding: the location, the cost as a function of growth (per
item, per request, per row), the severity (high when cost grows with data
or traffic on a user-facing path; medium on a background path; low
otherwise), and the fix. A cost you cannot state as a function of growth,
or that depends on volumes you could not see, is recorded with confidence
`low`. Never change ordering, results or error behavior
in the name of speed; report it instead.
