# Architecture Guardrails (Concept → Controller → Workflow)

Use this as a review checklist. If a change violates a guardrail, either redesign it or explicitly justify the exception.

## Guardrails

- **Controller non-thinking rule:** Controllers may only *instantiate/mediate* (IO, validation, normalization, orchestration) and *report* results; they must not decide meaning, revise universals, or resolve contradictions.

- **Concept-first rule:** The Agent’s durable “being” is `Concept` (Universal). Every workflow run must be traceable back to an explicit Concept (even if minimal).

- **Workflow as sublation:** A Workflow is the recorded time-structure of Concept enacted through particulars. A WorkflowRun must include a trace/proof sufficient for re-entry into Logic.

- **Phenomenology stays in Logic:** Contradictions, foundation, judgment seeds, and revision decisions live in Logic (Concept/Phenomenology). Controllers may transport them, never interpret them.

- **Kernel remains non-discursive:** The Rust kernel executes non-discursive compute and emits proof/witness payloads. It does not interpret Cypher, natural language, or philosophical semantics.

- **Stable protocol boundary:** Cross-boundary calls use stable IDs (`gds.<facade>.<op>`) and schema-first shapes. If meaning changes, version the shape or add a new op—don’t silently drift.

- **Determinism by default:** Prefer deterministic engines (local ML/GNN, rules) and record all non-determinism (LLM calls, sampling, randomness) explicitly in trace/provenance.

- **Separation of concerns in failures:** Errors from IO/compute are controller/kernel concerns; “what the error means” (contradiction, revision) is a Logic concern.

## Quick litmus tests

- If you can replace an LLM with deterministic logic and the architecture still works, you’re building a Knowledge Agent system (not BI automation).
- If controllers start containing “reasoning prompts” or “meaning selection,” you’re collapsing Logic into glue—stop and refactor.
