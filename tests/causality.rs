//! Integration tests: simulate distributed message passing and assert the causal
//! relationships hold across a small run.

use chrono_sync::{Causality, VectorClock};

#[test]
fn three_node_gossip_preserves_causality() {
    let (mut a, mut b, mut c) = (VectorClock::new(), VectorClock::new(), VectorClock::new());

    a.tick("A"); // a1
    let a1 = a.clone();

    b.merge(&a1);
    b.tick("B"); // b1 (after a1)
    let b1 = b.clone();

    c.merge(&b1);
    c.tick("C"); // c1 (after b1, transitively after a1)
    let c1 = c.clone();

    assert_eq!(a1.compare(&b1), Causality::Before);
    assert_eq!(b1.compare(&c1), Causality::Before);
    assert_eq!(a1.compare(&c1), Causality::Before, "causality is transitive");
}

#[test]
fn a_partition_produces_concurrent_events() {
    let mut a = VectorClock::new();
    let mut b = VectorClock::new();

    // shared history
    a.tick("A");
    b.merge(&a);
    b.tick("B");
    a.merge(&b);

    // network partition: both act independently
    let a_side = {
        let mut x = a.clone();
        x.tick("A");
        x
    };
    let b_side = {
        let mut y = b.clone();
        y.tick("B");
        y
    };

    assert_eq!(a_side.compare(&b_side), Causality::Concurrent);
}
