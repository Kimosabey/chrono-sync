# ChronoSync
### Distributed time and ordering with vector clocks in Rust.

![Status](https://img.shields.io/badge/Status-In_Development-blue?style=for-the-badge)
![Category](https://img.shields.io/badge/Category-Portfolio_Project-111?style=for-the-badge)

## 📖 Overview
ChronoSync implements vector clocks in Rust so distributed nodes can establish causal ordering of events without a global clock, and detect concurrent updates.

> Part of my Senior Hybrid Engineer 2026 portfolio (`#52`). Built on the "Antigravity" model — logic, state, and UI run locally in Docker while heavy reasoning is offloaded to cloud APIs, so the whole system runs on modest hardware.

## 🚀 Quick Start
```bash
# 1. Clone
git clone https://github.com/Kimosabey/chrono-sync.git
cd chrono-sync

# 2. Install
# (see docs/GETTING_STARTED.md for the full setup)

# 3. Run
docker compose up
```

## ✨ Key Features
- Vector-clock causal ordering
- Concurrency (conflict) detection
- Memory-safe Rust core
- No dependence on synchronized wall-clocks

## 🏗️ Architecture
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

Getting causal ordering right — the subtle correctness core of many distributed systems.

See [docs/ARCHITECTURE.md](./docs/ARCHITECTURE.md) for the full HLD/LLD and design decisions.

## 🧰 Tech Stack
| Layer | Technology | Role |
| :--- | :--- | :--- |
| Rust | `Rust` | Memory-safe performance core |
| Vector Clocks | `Vector Clocks` | Logical-clock ordering |

## 📚 Documentation
- [Architecture](./docs/ARCHITECTURE.md) — high- and low-level design, decision log
- [Getting Started](./docs/GETTING_STARTED.md) — prerequisites, setup, environment
- [Failure Scenarios](./docs/FAILURE_SCENARIOS.md) — fault analysis and recovery
- [Interview Q&A](./docs/INTERVIEW_QA.md) — deep-dive walkthrough

## 🔭 Future Enhancements
- Conflict-resolution strategies
- Clock compaction
- Bench vs. Lamport clocks

## 📄 License
Released under the MIT License.

## 👤 Author

**Harshan Aiyappa**
Senior Full-Stack Hybrid AI Engineer
Voice AI • Distributed Systems • Infrastructure

[![Portfolio](https://img.shields.io/badge/Portfolio-kimo--nexus.vercel.app-00C7B7?style=flat&logo=vercel)](https://kimo-nexus.vercel.app/)
[![GitHub](https://img.shields.io/badge/GitHub-Kimosabey-black?style=flat&logo=github)](https://github.com/Kimosabey)
[![LinkedIn](https://img.shields.io/badge/LinkedIn-Harshan_Aiyappa-blue?style=flat&logo=linkedin)](https://linkedin.com/in/harshan-aiyappa)
[![X](https://img.shields.io/badge/X-@HarshanAiyappa-black?style=flat&logo=x)](https://x.com/HarshanAiyappa)
