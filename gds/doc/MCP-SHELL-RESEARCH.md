# MCP Shell Research (Skrub)

## Intent

Skrub represents a **shell-like layer** for the MCP world: a scripting-oriented, user-facing environment for iterative, sklearn-style workflows. It is not part of MCP core architecture, but extends the MCP ecosystem beyond specialized GraphStores.

## Positioning

- **Not part of MCP core architecture.**
- **Built on top of Collections** (which organize API extensions to Rust Collections + Polars).
- Collections likely evolves into a **Collections/Data subsystem** to absorb Polars concepts, split into **data/frame** and **data/ops**.
- **GraphStore** remains a **narrow, specialized core tool** within MCP.
- **Skrub** is **scripty by design**, aligning with shell expectations and rapid iteration.
- Supports **Python/Pandas** and **Rust/Polars** workflows, with **Collections** handling the data-backend boundary.
- Long-term implementation target is **Rust-first**, not “Python vs Rust.”
- Skrub splits into two layers:
  - **DataOps**: DataFrame-oriented scripts inside Collections/data.
  - **ML Shell**: sklearn-oriented scripts **outside** Collections.
- Potentially a **Collections Shell** and **platform shell** concept (top-level system role).

## Conceptual Model

- MCP as an operating system.
- Skrub as the shell for that OS.
- Collections provide the foundational API surface.
- Skrub consumes Collections and exposes a scripting-first UX.

## Implications

- Expand Collections into a **data subsystem** (Polars-aligned).
- Treat Skrub's DataFrame scripts as **DataOps** within Collections.
- Keep **Machine Learning scripts** out of Collections; build them as a **separate package** on top of Collections/data.
- Treat Skrub as a **platform shell** for the MCP world.
- Emphasize sklearn-like, exploratory workflows.
- Treat Python as **compatibility and reference**, not the primary client.
- Rewrite Skrub into **Rust-first** tooling that still interoperates through Collections.

## Implementation Notes (Deferred)

Implementation details are intentionally deferred. We will revisit module layout after research.

## Open Questions

- Is this best framed as a **Collections Shell** with a top-level system role?
- What boundaries separate Collections/data from GraphStore and other core tools?
- What is the clean interface between Collections/DataOps and the ML shell package?
- Where should it live in the repo once we decide to implement?
- What minimal API should Collections expose to support Skrub shells?
- How should Skrub integrate with GraphStore without coupling to MCP core?
- What is the smallest Rust-first scripting surface that still feels like a shell?
