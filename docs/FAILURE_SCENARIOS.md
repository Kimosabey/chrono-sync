# Failure Scenarios — ChronoSync

ChronoSync is a pure algorithm library, so "failure" here means edge cases in distributed clock handling
rather than crashing services. This tracks how the design holds up.

## Fault Analysis
- **A node never receives another's messages (partition).** Both keep ticking locally. Their clocks
  diverge and `compare` correctly reports `Concurrent` — the library surfaces the conflict rather than
  hiding it. Resolution (which write wins) is the caller's policy, not the clock's job.
- **Out-of-order / duplicate message delivery.** `merge` is idempotent and commutative (element-wise max),
  so applying the same or reordered clocks converges to the same result. Duplicates are harmless.
- **A brand-new node with no history.** Unknown node ids read as `0`, so comparisons stay well-defined
  without special-casing.
- **Clock skew / wall-clock drift.** Irrelevant by design — ordering is logical, not time-based.

## Known limits
- **Unbounded growth**: the clock carries one entry per node ever seen. For large or churning node sets
  this grows; production use would prune retired nodes or cap the set (see Future Enhancements).

## Verification
- Correctness is covered by tests: `independent_events_on_different_nodes_are_concurrent`,
  `message_passing_establishes_causality`, a 3-node gossip chain, and an explicit partition scenario.
