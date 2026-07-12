# Interview Q&A — ChronoSync

### "Tell me about this project."
ChronoSync is a small Rust library that implements **vector clocks** — the mechanism distributed systems
use to order events causally without a global clock. Each node keeps a per-node counter; comparing two
clocks tells you whether one event happened before another, whether they're equal, or whether they're
**concurrent**. It ships with a runnable two-node demo and 7 tests.

### "What was the hardest / most interesting part?"
The **concurrent** case. A scalar clock (like a Lamport timestamp) always gives you a total order, which
hides genuine conflicts. Vector clocks give a *partial* order: two events are concurrent when neither
clock dominates the other on every component. Getting that comparison exactly right — and proving it with
a partition test — is the core of the project.

### "Why Rust, and why these data structures?"
Rust for a memory-safe, zero-dependency, provable core. I used a `BTreeMap` (not `HashMap`) for
deterministic ordering, which keeps clock output stable and test diffs readable. I chose an explicit
`Causality` enum over implementing `PartialOrd`, because `PartialOrd` would collapse "concurrent" into
`None` — I wanted concurrency to be a named, first-class outcome.

### "Where would this be used for real?"
It's the foundation under conflict detection in eventually-consistent stores (Dynamo-style databases) and
CRDTs. `merge` being commutative, associative, and idempotent is exactly what those systems rely on.

### "How does it fit your portfolio?"
It's my system-design / distributed-systems piece and my first Rust project — deliberately chosen to back
the "polyglot" claim with real, tested code rather than an assertion (`#52`).
