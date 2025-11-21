# ADR-00X: Reason ↔ Understanding — Fichtean Project mapped into ORGANON

## Status
Draft

## Context
Fichte’s project: derive multiplicity from the One (Reason/Idea) and reduce multiplicity back to the One via the activity of Understanding (concept-formation, judgment). ORGANON must capture this dialectical movement as a working pattern: Reason supplies the seed/Idea (pure intelligible unity); Understanding generates concepts, representations and structures that mediate enactment.

Samkhya and Vedanta provide complementary resources:
- Samkhya: gating of representations (selective generation/activation of Gunas / representational filters).
- Vedanta (Pratyaya‑śāra): essence of representation — the concentrated seed/meaning behind appearances (OM as seed-symbol).

We must translate this into our pipeline (BEC → MVC → TAW) and into the EssenceGraph / transformer stack.

## Decision
Treat Reason and Understanding as first‑class, linked artifacts in ORGANON:

- Reason (Idea)
  - Location: @logic / BEC
  - Role: regulative seed, pure principle, generative unity
  - Representation in system: BEC node (Idea) with canonical form, axioms, aishvarya profile

- Understanding (Concept faculty)
  - Location: @model / MVC
  - Role: concept formation, schemas, model templates, gating policies
  - Representation: Concept nodes, Model templates, view/perspective gating

- Synthesis / Enactment
  - Location: @task / TAW
  - Role: satkarya / verified enactment (workflow as truth‑effect)
  - Representation: Workflow nodes, verifiers, REST stabilization

- Pratyaya‑śāra (Essence of Representation)
  - Location: EssenceGraph metadata
  - Role: canonical semantic seed for views/transformers (used to guide GenAI dyads)

## Rationale
- Preserves Fichtean dialectic (One → Many → One) operationally.
- Provides explicit, auditable mapping between supersensible principles and empirical enactments.
- Enables GenAI transformers to instantiate Concepts from Ideas while respecting gating/antakarana (supervisory control).

## Consequences / Implementation sketch
1. EssenceGraph: add node types {Idea, Concept, PratyayaSeed, GatePolicy, SatkaryaProof} and directed traversal order metadata.
2. Transformer: GenAI dyad interface must accept (Idea + GatePolicy) → Concept definition (schema + representation tokens).
3. Controller/Workflow metadata: include regulativeRef → Idea.id and pratyayaRef → PratyayaSeed.id; workflows record satkarya proofs and REST snapshots.
4. Gating: implement GatePolicy as first‑class object (filters, selection heuristics, aishvarya profile) applied during transformation and at agent runtime.
5. Tests / Experiments: pick a single Idea (e.g., “FormModel for X”), produce a Concept via transformer, generate an MVC instance, execute a Workflow that produces/records satkarya.

## Next steps
- File this ADR (done).
- Scaffold EssenceGraph node schemas and Controller/Workflow metadata fields (regulativeRef, pratyayaRef, gatePolicy).
- Prototype one full pipeline (Idea → Concept → MVC → Workflow) with proof traces captured.
- Iterate gate policies (Samkhya‑inspired) and tune GenAI dyads to respect Pratyaya seeds.
