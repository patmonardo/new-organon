Knowledge module — TAW <knowledge>
===================================

This folder contains the schemas for the TAW special object: <knowledge>.

Scope (scientific, implementation-focused):
- Encodes knowledge units (claims + justification + provenance)
- Tracks origin (empirical, transcendental, synthetic, reflective)
- Supports epistemic status and confidence
- Provides a light knowledge graph (nodes/edges) and a TopicModel wrapper

Design notes:
- Zod schemas are the source of truth; factories are convenience only.
- No runtime dependencies on model or logic; this layer is standalone within @organon/task.
- The graph APIs here are minimal and safe to evolve.
