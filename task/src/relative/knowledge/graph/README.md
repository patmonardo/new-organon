Graph Port (thin abstraction)
=============================

Purpose:
- Provide a tiny, swappable interface (GraphPort) for agents/workflows to interact with a graph.
- Keep usage simple now; later, swap in @organon/gds adapters without changing call sites.

Files:
- port.ts — interface + minimal query type.
- memoryAdapter.ts — in-memory implementation for tests/local runs.

Note: These files are not included in the current tsconfig build to keep the schema-only compile tight.
