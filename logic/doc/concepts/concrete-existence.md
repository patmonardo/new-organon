# Concrete Existence — Essence/Reflection/Ground realized as E/P/R

Intent
- Make Hegel’s “Concrete Existence” operational for the Form Processor.
- Pin down Essence/Reflection/Ground as Entity/Property/Relation under a fixed Context.

Dialectic map (Essence-plane)
- Essence → Entity (Thing-as-determinate content)
- Reflection → Property (Determinations of reflection; measured predicates inside a Context)
- Ground → Relation (Essential/Absolute ties; constraint carriers; truth of Morph)

Principles as inputs (per run)
- Shape (Being) → types/kinds (what entities can be)
- Context (Existence-as-Reflection) → determinations (what counts; contradiction rules; property catalog)
- Morph (Being-for-self-as-Ground) → admissible constructions; energizes Essential Relations (propagation)

Thing with Properties (operationalized)
- Entities are instances of Shapes; they carry identity and determinacy.
- Properties are contextualized predicates/measures over Entities:
  - Always carry provenance: { contextId, contextVersion }
  - Validity is decided by Context’s determinations (Reflection); contradictions “fall to ground”
- Relations are Essential/Absolute when constraint-bearing:
  - Endpoints typed by Shape/Entity
  - May be directed; support two-way propagation (effects can be asymmetric)
  - Ground = propagation substrate; Morph specifies admissible constructions that seed/induce these ties

Processor semantics (Ring 1)
- Seed (Shape→Entity): build the universe of discourse
- Contextualize (Context→Property): materialize property catalog per Context; attach measurements/predicates to Entities
- Ground (Morph→Relation): apply Morph rules to induce Essential Relations; propagate constraints to a fixpoint
- Contradiction handling:
  - Property values that violate Context’s determinations are invalidated (fall to ground)
  - Essential Relations can enforce/derive properties or edges (backpropagation)

Invariants to preserve
- Context is fixed within a run; bumping its version invalidates prior Property variations
- Morph remains a principle (non-concurrent); its truth appears as Relations and derivations, not Morph mutation
- Repository updates use mutateFn with optional expectedRevision; get() returns null when missing

What to verify next (tests)
- Property invalidation on Context version bump (provenance respected)
- Contradiction: a Property outside allowed reflection domain is flagged/removed
- Essential Relation derives a missing edge or Property; propagation reaches fixpoint
- Describe paths surface enough provenance to audit decisions (ids, contextVersion)

Reading cues
- “Thing with Properties” sits in Essence/Appearance; Essential Relation is the truth of Ground (Being‑for‑self).
- Our E/P/R box is Concrete Existence; Rings 2–3 (Model/Controller → Task/Workflow) are supersensible projections/plans over it.
