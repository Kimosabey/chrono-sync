# Interview Q&A — ChronoSync

### "Tell me about this project."
ChronoSync is distributed time and ordering with vector clocks in Rust. ChronoSync implements vector clocks in Rust so distributed nodes can establish causal ordering of events without a global clock, and detect concurrent updates.

### "What was the hardest part?"
Getting causal ordering right — the subtle correctness core of many distributed systems.

### "Why did you choose this stack?"
- **Rust** — memory-safe performance core.
- **Vector Clocks** — logical-clock ordering.

### "How does it fit the rest of your portfolio?"
It follows my "Antigravity" model — local logic/state/UI, cloud reasoning where it earns its cost — and shares the documentation and deployment conventions used across all my projects (#52).
