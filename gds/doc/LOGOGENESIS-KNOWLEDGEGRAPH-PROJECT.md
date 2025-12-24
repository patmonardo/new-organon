# The Logogenesis KnowledgeGraph (Kant–Fichte–Hegel as One Executable Project)

This repo’s “real project” is not BI, not dashboards, and not agent glue. It is the construction of a **KnowledgeGraph of Logogenesis**: a graph of how knowledge *comes to be*—from ignorance to stabilized concept, judgment, identity, and workflow.

The guiding synthesis:

- **Kant**: conditions of possibility (categories/forms of judgment; reflection as the discursive instrument).
- **Fichte**: knowing as lawful activity/positing (kernel as sublingual act; “I → Not‑I” externalization into transmissible form).
- **Hegel**: the System of Science as **Concept → Judgment → Syllogism**, and the genetic development (logogenesis) whereby these moments become objective.

---

## 1) What is “Logogenesis” in engineering terms?

Logogenesis is a *traceable, replayable* process that converts:

- **Avidyā (ignorance)**: not-yet-stabilized concept/judgment; reliance on verification and search
into
- **Vidyā (knowledge)**: stabilized concept/judgment plus compiled identities (Morphs) that apply automatically.

The key artifact is a **WorkflowRun** that records the system operating in *all its moments* (kernel evidence → concept/judgment stabilization → morph application → promotion into knowing).

See: `gds/doc/IGNORANCE-TO-KNOWLEDGE-AS-WORKFLOW.md`.

---

## 2) The KnowledgeGraph schema (conceptual)

### 2.1 Node kinds

- **Principle**
  - The kernel-side possession that makes determinate science possible.
- **Concept (Shape)**
  - Genus/species structure of a domain (e.g. Boolean Algebra, Trig, Pure Mechanics).
- **Judgment (Context / Reflection)**
  - Subject/predicate + scope + conditions + reflective evaluation surfaces (truth tables, regimes).
- **Identity / Morph**
  - Compiled operator: lawful rewrite/proof step (fast “knowing”).
- **Evidence**
  - Kernel outputs: truth-table results, SAT/BDD certificates, graph/GNN artifacts, traversal proofs.
- **Workflow**
  - Executable time-structure (TAW) derived from stabilized concept/judgment.
- **WorkflowRun**
  - The “rare perfect document”: trace/proof sufficient for review and re-entry.
- **Artifact**
  - Produced entities/properties/aspects, proofs, prints, derived facts.
- **Parameter**
  - A value that *situates* a universal rule-form (dharma) into a particular application instance.
  - Example: the universal law “equality is invariant under translation” becomes the concrete step “subtract 5 from both sides” by choosing the parameter $c=-5$.

### 2.2 Edge kinds (the genetic relations)

- **grounds**: Evidence → (Judgment/Identity)
- **stabilizes**: (Evidence + Judgment) → Concept
- **derives**: (Concept + Judgment) → Identity/Morph
- **applies**: Identity/Morph → (Artifact / next Judgment)
- **promotes**: (Verified step) → Identity/Morph (upgrade from reflection to knowing)
- **executes**: Workflow → WorkflowRun
- **records**: WorkflowRun → (Evidence, Steps, Artifacts)
- **re-enters**: WorkflowRun → (Concept/Judgment revision)
- **situates**: (Rule-form / Identity schema) → (Applied step), via Parameter
  - This is the “dharma builds itself into a situation” edge: a universal form becomes a particular act by choosing parameters and binding them into the concrete appearance.

This graph is the “encyclopedia” you’re aiming for: not just results, but *how results become possible*.

---

## 3) Where the synthesis lives in the stack

### 3.1 Kernel (GDS) — Fichtean knowing / evidence generation

- Kernel produces **Evidence** and (eventually) participates in **Principle possession**.
- It must remain non-discursive: compute, don’t narrate.

### 3.2 Logic (TS) — Kantian judgment / Hegelian logogenesis control

- Logic stabilizes **Concept–Judgment** and compiles **Morphs** as identities/operators.
- It owns meaning, scope, admissibility, contradiction, and revision.

### 3.3 Controllers + Workflows (MVC/TAW) — Kriya / objective enactment

- Controllers invoke kernels and services; they do not “think.”
- Workflows are the executable enactment that yields WorkflowRuns as sacred documents.

See: `logic/doc/concept-controller-workflow.md`.

---

## 4) First proof domains (why we chose them)

- **Boolean Algebra / Set Theory**: brutally checkable reflection surfaces (truth tables / SAT / BDD), ideal for showing the Ignorance→Knowledge upgrade into Morphs.
  - See: `gds/doc/BOOLEAN-ALGEBRA-AS-CONCEPT-JUDGMENT-SYLLOGISM.md`
  - See: `gds/doc/BOOLEAN-INFERENCE-GENESIS.md`
- **Kantian Pure Mechanics**: clear separation of objective moments (kinematics + dynamics as reflection) yielding a Mechanical Concept (control of inference).
  - See: `gds/doc/KANTIAN-PURE-MECHANICS-AS-CONCEPT-JUDGMENT.md`

---

## 5) The project commitment (what makes it “not BI”)

The commitment is: **we do not collapse cognition into outcome labels**.

We treat “ordinary formalism” (SAT/BDD/truth tables, later numerical solvers) as **Reflection instruments** and verification rails, while the project is the construction of a **genetic KnowledgeGraph** of:

- how concepts stabilize,
- how judgments constrain,
- how identities compile,
- and how workflows become the objective record of knowing.

That is the Kant–Fichte–Hegel synthesis as an executable system: a real KnowledgeGraph of Logogenesis.

## Appendix: vocabulary discipline (Intuition vs Concept)

Because modern technical writing often uses “intuitive” loosely, we keep a short glossary:

- `gds/doc/INTUITION-VS-CONCEPT-GLOSSARY.md`


