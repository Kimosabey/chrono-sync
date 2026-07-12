//! Demo: two nodes exchange a message; show that causality is tracked and that a
//! concurrent local event is correctly detected. `cargo run`.

use chrono_sync::{Causality, VectorClock};

fn label(c: Causality) -> &'static str {
    match c {
        Causality::Before => "happened-before",
        Causality::After => "happened-after",
        Causality::Equal => "equal",
        Causality::Concurrent => "CONCURRENT",
    }
}

fn main() {
    let mut a = VectorClock::new();
    let mut b = VectorClock::new();

    a.tick("A"); // event a1
    let a1 = a.clone();
    println!("a1 = {a1:?}");

    // A -> B: B receives a1, then does its own event b1
    b.merge(&a1);
    b.tick("B"); // event b1
    let b1 = b.clone();
    println!("b1 = {b1:?}");
    println!("a1 vs b1: {}", label(a1.compare(&b1))); // happened-before

    // A does another local event without hearing from B
    a.tick("A"); // event a2
    println!("a2 = {a:?}");
    println!("a2 vs b1: {}", label(a.compare(&b1))); // CONCURRENT

    // B later hears about a2
    b.merge(&a);
    b.tick("B"); // event b2
    println!("b2 = {b:?}");
    println!("a2 vs b2: {}", label(a.compare(&b))); // happened-before
}
