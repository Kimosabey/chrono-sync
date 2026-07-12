# Architecture — ChronoSync

## High-Level Design (HLD)
ChronoSync implements vector clocks in Rust so distributed nodes can establish causal ordering of events without a global clock, and detect concurrent updates.

```mermaid
%%{init: {'theme':'base','themeVariables':{'primaryColor':'#ffffff','lineColor':'#2563eb','mainBkg':'#ffffff'}}}%%
graph LR
    A([Node A])
    B([Node B])
    C([Vector-Clock Merge])
    D([Causal Order])
    A --> B
    B --> C
    C --> D
    style A fill:#eff6ff,stroke:#2563eb,stroke-width:2px,color:#1e40af
    style B fill:#eff6ff,stroke:#2563eb,stroke-width:2px,color:#1e40af
    style C fill:#eff6ff,stroke:#2563eb,stroke-width:2px,color:#1e40af
    style D fill:#eff6ff,stroke:#2563eb,stroke-width:2px,color:#1e40af
```

**Flow:** Node A → Node B → Vector-Clock Merge → Causal Order

## Low-Level Design (LLD)
- **Components:** `Rust`, `Vector Clocks`
- **Interfaces / contracts:** to be finalized during implementation.
- **Data model:** to be defined per component.

## Decision Log
- **Why this stack:** **Rust** — memory-safe performance core; **Vector Clocks** — logical-clock ordering.
- **Antigravity constraint:** run logic/state/UI locally; offload heavy reasoning to cloud APIs; target modest hardware.

## Concept Deep Dive
Getting causal ordering right — the subtle correctness core of many distributed systems.
