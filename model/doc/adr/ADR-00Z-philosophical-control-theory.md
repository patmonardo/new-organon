# ADR-00Z: Philosophical Control Theory — Antakarana & Satkarya Mapping

## Status
Draft

## Context
ORGANON separates supersensible logic (BEC) and empirical processing (MVC, TAW). We need a concise ADR that formalizes:
- Antakarana as the inner control (MVC/Essence-dominant mediating faculty).
- Satkarya as the truth-effect (TAW/Satkarya as enacted knowledge, Workflow as embodiment).

## Decision
Adopt the following mapping and mandates:

1. Antakarana (Inner Control / MVC)  
   - MVC components implement Antakarana: controllers and views carry regulative provenance and adaptation policies.  
   - Antakarana metadata must be present on controllers: regulativeRef, aishvaryaProfile, antakaranaPolicy.

2. Satkarya (Truth-Effect / TAW)  
   - TAW artifacts (Task, Agent, Workflow) represent Satkarya: verified enactments of control-concept synthesis.  
   - A Workflow is the primary Satkarya object: it is not a recipe but a verified dialectical emergence (Concept + Control → Workflow).  
   - Workflows must carry satkaryaSpec: expected truth-effect, success criteria, verifier hooks.

3. Interface Contract  
   - Controllers expose supervisory hooks; Agents/Workflows call these to verify and adapt enactments.  
   - EssenceGraph must encode antakarana→satkarya links for traversal and audit.

## Rationale
- Makes the connection between the pure (BEC) and the empirical explicit and auditable.  
- Distinguishes inner control (design, adaptation) from enacted truth (runtime verification).  
- Aligns platform semantics with classical Samkhya/Śaiva notions while remaining implementable.

## Consequences
- All new controllers and workflows must include antakarana and satkarya metadata.  
- The transformer and adapter pipelines must propagate these fields from Absolute→Relative MVC→TAW.  
- Monitoring and verification adapters will be required to materialize satkarya verification at runtime.

## Next steps
1. Add satkarya and antakarana fields to Controller and Workflow schemas.  
2. Update EssenceGraph schema to link regulative BEC nodes to controller nodes and workflow nodes.  
3. Prototype a simple Pipeline: BEC Concept → Absolute Transformer → MVC Controller (antakarana) → Workflow (satkarya) with verification hook.  
4. Document mapping in README and adapter guidelines.
