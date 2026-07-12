# Getting Started — ChronoSync

## Prerequisites
- **Rust** 1.70+ (install via [rustup](https://rustup.rs)). No other dependencies — the crate is pure Rust.

## Build & run
```bash
git clone https://github.com/Kimosabey/chrono-sync.git
cd chrono-sync

cargo build          # compile the library + demo binary
cargo run            # run the two-node message-passing demo
```

## Running tests
```bash
cargo test           # 5 unit tests (in src/lib.rs) + 2 integration tests (tests/causality.rs)
```

## Using it as a library
Add it as a path/git dependency and use the API:
```rust
use chrono_sync::{VectorClock, Causality};

let mut a = VectorClock::new();
a.tick("A");                       // a local event on node A
let mut b = VectorClock::new();
b.merge(&a);                       // node B receives A's clock
b.tick("B");                       // a local event on node B
assert_eq!(a.compare(&b), Causality::Before);
```

There are no environment variables or services to configure — it's a self-contained algorithm crate.
