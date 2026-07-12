//! # ChronoSync
//!
//! Vector clocks for distributed causal ordering. Each node keeps a per-node counter;
//! comparing two clocks tells you whether one event *happened before* another, whether
//! they are *equal*, or whether they are *concurrent* — all without a synchronized wall clock.
//!
//! - `tick`  — a local event on a node (increment its own counter).
//! - `merge` — receiving a message (element-wise max of both clocks).
//! - `compare` — the causal relationship, including the `Concurrent` case a scalar clock can't express.

use std::collections::{BTreeMap, BTreeSet};

/// The causal relationship between two vector clocks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Causality {
    /// `self` happened before `other`.
    Before,
    /// `self` happened after `other`.
    After,
    /// identical clocks.
    Equal,
    /// neither dominates — the events are causally independent.
    Concurrent,
}

/// A vector clock: node id -> logical counter. Missing nodes read as 0.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct VectorClock {
    counts: BTreeMap<String, u64>,
}

impl VectorClock {
    pub fn new() -> Self {
        Self { counts: BTreeMap::new() }
    }

    /// Register a local event on `node`.
    pub fn tick(&mut self, node: &str) {
        *self.counts.entry(node.to_string()).or_insert(0) += 1;
    }

    /// The counter for `node` (0 if never seen).
    pub fn get(&self, node: &str) -> u64 {
        *self.counts.get(node).unwrap_or(&0)
    }

    /// Merge another clock into this one (element-wise max) — the receive step.
    pub fn merge(&mut self, other: &VectorClock) {
        for (node, &c) in &other.counts {
            let entry = self.counts.entry(node.clone()).or_insert(0);
            if c > *entry {
                *entry = c;
            }
        }
    }

    /// Causal comparison. `Concurrent` when neither clock dominates the other.
    pub fn compare(&self, other: &VectorClock) -> Causality {
        let mut less = false; // some component of self < other
        let mut greater = false; // some component of self > other
        let keys: BTreeSet<&String> = self.counts.keys().chain(other.counts.keys()).collect();
        for k in keys {
            let a = self.get(k);
            let b = other.get(k);
            if a < b {
                less = true;
            } else if a > b {
                greater = true;
            }
        }
        match (less, greater) {
            (false, false) => Causality::Equal,
            (true, false) => Causality::Before,
            (false, true) => Causality::After,
            (true, true) => Causality::Concurrent,
        }
    }

    pub fn happens_before(&self, other: &VectorClock) -> bool {
        self.compare(other) == Causality::Before
    }

    pub fn concurrent_with(&self, other: &VectorClock) -> bool {
        self.compare(other) == Causality::Concurrent
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_clocks_are_equal() {
        assert_eq!(VectorClock::new().compare(&VectorClock::new()), Causality::Equal);
    }

    #[test]
    fn a_tick_happens_after_the_prior_state() {
        let mut a = VectorClock::new();
        let before = a.clone();
        a.tick("A");
        assert_eq!(before.compare(&a), Causality::Before);
        assert_eq!(a.compare(&before), Causality::After);
        assert!(before.happens_before(&a));
    }

    #[test]
    fn independent_events_on_different_nodes_are_concurrent() {
        let mut a = VectorClock::new();
        let mut b = VectorClock::new();
        a.tick("A"); // a's local event
        b.tick("B"); // b's local event, no communication
        assert_eq!(a.compare(&b), Causality::Concurrent);
        assert!(a.concurrent_with(&b));
    }

    #[test]
    fn merge_takes_the_elementwise_max() {
        let mut a = VectorClock::new();
        a.tick("A");
        a.tick("A"); // A=2
        let mut b = VectorClock::new();
        b.tick("B"); // B=1
        b.merge(&a); // b now knows A=2, B=1
        assert_eq!(b.get("A"), 2);
        assert_eq!(b.get("B"), 1);
        // b now causally follows a
        assert_eq!(a.compare(&b), Causality::Before);
    }

    #[test]
    fn message_passing_establishes_causality() {
        // A does a1, sends to B; B merges + ticks (b1). b1 must be After a1.
        let mut a = VectorClock::new();
        a.tick("A"); // a1
        let a1 = a.clone();
        let mut b = VectorClock::new();
        b.merge(&a1);
        b.tick("B"); // b1
        assert_eq!(a1.compare(&b), Causality::Before);

        // Meanwhile A ticks again without hearing from B -> concurrent with b1.
        a.tick("A"); // a2
        assert_eq!(a.compare(&b), Causality::Concurrent);
    }
}
