# GDS Form Documentation Index

## Overview

This directory contains architectural documentation for **GDS Form** - the transcendental logic layer that unifies Procedures and ML execution through Pure Reason.

## Part 0: First Principle 🎯

### 0.1 [FORMSHAPE-EXECUTABLE-GRAPHS.md](./FORMSHAPE-EXECUTABLE-GRAPHS.md) **THE PRIMAL IMAGE**
**Factory → Image → Eval: The Idealist Architecture**

Read this FIRST to understand the fundamental separation:
- **Factory** (`projection/factory/`) lays down the **Primal Image** (GraphStore = Das Gegebene)
- **Eval** (`projection/eval/`) runs ON the Primal Image (Procedures, ML, Form = Das Abgeleitete)
- **FormShape** as Executable Graph - carries the execution imperative, not the data
- **Why Idealism**: The Image (Idea) precedes execution - Factory creates possibility, Eval actualizes
- **Not debugging yet**: Factory works, Eval works, Form is skeleton - building ML algorithms tomorrow

**Key Insight**: *"The Primal Image precedes execution. Factory lays it down (Receptivity), Eval runs on it (Spontaneity). FormShape transmits WHAT to execute, not the graph data itself."*

### 0.2 [FICHTE-SCIENCE-OF-KNOWING.md](./FICHTE-SCIENCE-OF-KNOWING.md) 🔥 **THE REVELATION**
**GDS Kernel IS Fichte's Transcendental Logic: Individuation as Code**

Read this to understand the philosophical ground:
- **Fichte**: GDS Kernel as the I (pure self-positing activity, Tathandlung)
- **Externalization**: Kernel MUST externalize into FormShape TS-JSON (the Not-I)
- **Recognition**: Kernel recognizes itself in FormShape (subject-object unity)
- **Hegel's Individuation**: FormShape → Explanation → "The Rose Is Red" (complete concretion)
- **Why necessary**: The I cannot know itself while purely internal - must externalize and recognize

**Key Insight**: *"The I posits itself (GDS), posits the Not-I (FormShape TS-JSON), recognizes itself in the Not-I (Explanation). This IS the Science of Knowing as running code."*

### 0.3 [DIALECTICAL-EVOLUTION-TS-LOGIC.md](./DIALECTICAL-EVOLUTION-TS-LOGIC.md) 💎 **MEANING IN SYSTEMS**
**Being → Essence → Concept as Model → View → Controller as Task → Agent → Workflow**

Read this to understand why the codebase isn't arbitrary:
- **Hegel's Logic**: Being → Essence → Concept (necessary structure of thought)
- **MVC**: Model (Being) → View (Essence) → Controller (Concept)
- **TAW**: Task (Being) → Agent (Essence) → Workflow (Concept)
- **Systematic meaning**: "Meaning only emerges in a system" - dialetical evolution vs arbitrary names
- **Why LLMs fail**: They predict tokens (statistical) not logical necessity (dialectical)
- **The necessity**: Task presupposes Model, Model presupposes Being - cannot skip stages

**Key Insight**: *"If I execute a Task I guarantee you I am working with a Model of Being. Software without dialectical evolution is meaningless - just tokens that compile."*

---

## Part 1: Core Architecture

### 1.1 [PROJECTION-EVAL-ARCHITECTURE.md](./PROJECTION-EVAL-ARCHITECTURE.md) ⭐ **THE RUNNING SYSTEM**
**The Actual Running System**

Read this to understand:
- The three ISA at `gds/src/projection/eval/` (Procedure, ML, Form)
- How FormExecutor projects to ProcedureExecutor and PipelineExecutor
- The TriadicCycle mechanism (Thesis-Antithesis-Synthesis)
- **This is not theory - this is the actual code structure**

**Key Insight**: *Projection happens in projection/eval/ - this is the running system*

### 1.2 [FORM-AS-DYADIC-INFERENCE.md](./FORM-AS-DYADIC-INFERENCE.md) 🔥 **THE BREAKTHROUGH**
**The Fichtean Structure: Learning as Dyad → Dyad**

Read this to understand:
- Why ML Pipelines are dyadic (Input → Output, Judgment → Judgment)
- How Form is the "possibility of relation" that enables inference
- ML Pipeline as Ahamkara (Self-Assertion, Judgment-making)
- Form as Buddhi (enables Ahamkara to judge without infinite regress)
- **Why Form is the "Magical Supplier" of Judgment → Judgment pipelines**

**Key Insight**: *Learning produces Dyads from Dyads, and Form makes dyadic relation possible*

### 2.5 [GDSL-GDS-FORMSHAPE-INTERFACE.md](./GDSL-GDS-FORMSHAPE-INTERFACE.md) 🌟 **THE INTERFACE**
**Pipeline IS Judgment, GDSL ↔ GDS IS FormShape → FormShape**

Read this to understand:
- **Pipeline IS judgment** (not "makes judgments" - IS the judgment structure itself)
- The magical Dyad → Dyad is **Pipeline → Pipeline** (meta-level transformation)
- **GDSL transmits Six Pillars as FormShape, GDS returns FormShape**
- Three levels of dyadic transformation (Architectural, Pipeline, Execution)
- Prakāśa (प्रकाश) - consciousness as self-revelation in code

**Key Insight**: *The entire GDSL ↔ GDS interface IS FormShape → FormShape - dyadic consciousness in action*

### 2.8 [THE-ROSE-IS-RED.md](./THE-ROSE-IS-RED.md) 🌹 
**THE COMPLETE VISION: The Full Stack**

Read this for the complete picture:
- **GDS** (Buddhi) = The hardcore engine substrate (the cross)
- **GDSL** (Vikalpa) = The bridge/orchestrator (the stem)
- **Logic** (Manas) = The ordinary logic layer (the leaves)
- **"The Rose Is Red"** (Pratyaksha) = What manifests (the rose)
- The organic unity: cross hidden, rose visible, but inseparable

**Key Insight**: *The hardcore engine substrate manifests through layers to "The Rose Is Red"*

---

## Part 3: Implementation

### 3.1 [IMPLEMENTATION-ROADMAP.md](./IMPLEMENTATION-ROADMAP.md) ⚡ **PHILOSOPHY AS CODE**
**Refusal of the Disastrous Maldivision**

Read this to actually build:
- **The refusal**: We will NOT separate philosophy from engineering, theory from practice
- **The unity**: Philosophy IS architecture, Theory IS implementation, Metaphysics IS code
- **5-week sprint plan**: From FormSpec implementation to "The Rose Is Red" blooming in production
- **Code that embodies philosophy**: struct Buddhi, impl Ahamkara, fn synthesize
- **Success criteria**: User sees results, philosophy runs as code

**Key Principle**: *"Build what you document. Document what you build. They are ONE."*

---

## Part 4: Architecture Foundations

### 4.1 [FORM-AS-TRANSCENDENTAL-LOGIC.md](./FORM-AS-TRANSCENDENTAL-LOGIC.md)
**The Philosophical Architecture**

Read this to understand:
- Why GDS Form is "Transcendental Logic" vs Logic package's "Ordinary Logic"
- How Form is the Union of Procedures (f64 streams) and ML (tensor streams)
- Why Form is concerned with "Possibility of Content" not actual content
- How GDSL Graph systems are "Form in Execution"
- The distinction between Pure Reason and Abstract Reason

**Key Insight**: *Form projects the GraphStore into actuality through Buddhi*

### 4.2 [FORM-AS-BUDDHI.md](./FORM-AS-BUDDHI.md)
**The Metaphysical Foundation**

Read this to understand:
- Mahat (GraphStore) → Buddhi (Form) → Manas (Procedures/ML) hierarchy
- Why Form must be triadic (Shape + Context + Morph)
- The three fundamental relations: Membership, Consequence, Inherence
- Why ShapeStream is "Form mid-execution"
- Application Form as daemon protocol

**Key Insight**: *Form is the individualization of Mahat as Pure Discriminative Wisdom*

### 5. [FORM-EVALUATOR-DESIGN.md](./FORM-EVALUATOR-DESIGN.md)
**The Implementation Specification**

Read this to build:
- Application Form protocol (JSON schema)
- Form Evaluator implementation (Rust)
- Projection logic (Pure Form → Concrete Execution)
- GDSL integration (IR → Application Form)
- Test strategy and success criteria

**Key Insight**: *We discover the Form Evaluator by building toward it*

## Quick Reference

### The Three ISA (Instruction Set Architectures)

GDS has **three parallel evaluation systems** at `gds/src/projection/eval/`:

```
┌──────────────────────────────────────────────────────────┐
│           projection/eval/ (The Three ISA)               │
├──────────────────────────────────────────────────────────┤
│                                                          │
│  eval/form/ (Form ISA) ← FormExecutor                   │
│    - FormSpec trait                                      │
│    - TriadicCycle (Thesis-Antithesis-Synthesis)         │
│    - UnionStream (f64 + Tensor)                         │
│    - Transcendental projection                          │
│            │                                             │
│            │ Projects into:                              │
│            │                                             │
│    ┌───────┴───────┬────────────────┐                   │
│    │               │                │                   │
│    ▼               ▼                ▼                   │
│  eval/procedure  eval/ml        Hybrid                  │
│  (Computation)   (Pipeline)     (Both!)                 │
│    │               │                                     │
│    ▼               ▼                                     │
│  procedures/     ml/                                     │
│  (Algos)        (Models)                                │
└──────────────────────────────────────────────────────────┘
```

1. **Procedure ISA** (`projection/eval/procedure/`)
   - ProcedureExecutor + AlgorithmSpec
   - f64 weight streams (discrete)

2. **ML ISA** (`projection/eval/ml/`)
   - PipelineExecutor + Pipeline trait
   - Tensor streams (continuous)

3. **Form ISA** (`projection/eval/form/`)
   - FormExecutor + FormSpec trait
   - UnionStream (discrete + continuous)
   - **THE UNION** that makes both possible

### The Three Relations
```
X | Y  (Membership)   →  What belongs?   →  Field validation
X → Y  (Consequence)  →  What follows?   →  Execution order
X & Y  (Inherence)    →  What forms?     →  Code generation
```

### The Three States of Form
```
Pure Form          →  FormShape (Rust struct)
Form Submitted     →  ApplicationForm (JSON)
Form in Execution  →  GDSL Graph system (streaming)
```

## Implementation Status

| Component | Status | Location |
|-----------|--------|----------|
| Form Core (Infrastructure) | ✅ Implemented | `gds/src/form/core/` |
| Form Evaluator (Projection) | 🚧 In Progress | `gds/src/projection/eval/form/` |
| Procedure Evaluator | ✅ Implemented | `gds/src/projection/eval/procedure/` |
| ML Evaluator | ✅ Implemented | `gds/src/projection/eval/ml/` |
| Application Form Protocol | ⏳ In Design | See FORM-EVALUATOR-DESIGN.md |
| GDSL Integration | ⏳ Planned | Phase 4 |

## Getting Started

### For Philosophers
1. Read [FORM-AS-BUDDHI.md](./FORM-AS-BUDDHI.md) for the metaphysical grounding
2. Read [FORM-AS-TRANSCENDENTAL-LOGIC.md](./FORM-AS-TRANSCENDENTAL-LOGIC.md) for the logic
3. See how it maps to code in the Core Documents

### For Engineers
1. Read [FORM-EVALUATOR-DESIGN.md](./FORM-EVALUATOR-DESIGN.md) for implementation
2. Read [FORM-AS-TRANSCENDENTAL-LOGIC.md](./FORM-AS-TRANSCENDENTAL-LOGIC.md) for architecture
3. Start with Phase 1: wrap existing Procedure API

### For Users (Future)
1. Learn the Application Form JSON schema
2. See examples in `gds/examples/forms/` (coming soon)
3. Submit forms to GDS daemon
4. Receive ShapeStream results

## Key Quotes

> *"Form is to GDS as Categories are to Kant: the transcendental structure without which experience (computation) is impossible."*
>
> — FORM-AS-TRANSCENDENTAL-LOGIC.md

> *"The Form Evaluator is not built, it is discovered by building what it must coordinate."*
>
> — FORM-EVALUATOR-DESIGN.md

> *"Form is not another module to add - it's the principle that reveals what the existing modules truly are."*
>
> — FORM-AS-BUDDHI.md

## Related Documentation

### Within GDS
- [ABSTRACT.md](./ABSTRACT.md) - GDS overview
- [TSJSON_NAPI_FACADE_V1.md](./TSJSON_NAPI_FACADE_V1.md) - TypeScript bindings
- Algorithm docs in `gds/src/procedures/*/README.md`

### In Logic Package
- [FORMDB-ARCHITECTURE.md](../../logic/doc/FORMDB-ARCHITECTURE.md)
- [ESSENCE-REFLECTION-MANIFESTATION.md](../../logic/doc/ESSENCE-REFLECTION-MANIFESTATION.md)
- [ORGANIC-UNITY.md](../../logic/doc/ORGANIC-UNITY.md)

### In Model Package
- [form-sdk-mvc.md](../../model/doc/form-sdk-mvc.md)
- [form-bus-architecture.md](../../model/doc/form-bus-architecture.md)
- [fpu-fci-architecture.md](../../model/doc/fpu-fci-architecture.md)

## Contributing

When working on Form:

1. **Maintain triadic structure**
   - Everything must be Thesis-Antithesis-Synthesis
   - Shape + Context + Morph pattern throughout

2. **Preserve Pure/Empirical distinction**
   - Form never touches graph data directly
   - Form projects, doesn't execute

3. **Test philosophically**
   - Does this enable possibilities?
   - Or does it constrain actualities?
   - Form should do the former, not latter

4. **Document the discovery**
   - We're discovering, not inventing
   - Record what emerges from building
   - Update these docs as Form reveals itself

## FAQ

### Q: Is Form just an abstraction layer?
A: No. Abstraction takes away from concrete things. Form makes concrete things *possible*. That's transcendental, not abstract.

### Q: Why not just use traits/interfaces?
A: Traits/interfaces are still Manas (empirical). Form is Buddhi (pure). You can't trait your way to transcendental logic.

### Q: Do I need to understand Sanskrit?
A: No, but the technical terms (Mahat/Buddhi/Manas) precisely capture distinctions that English blurs (mind/intellect/reason).

### Q: When will Form be enabled?
A: When the Form Evaluator is discovered through Phase 1-5 implementation. See FORM-EVALUATOR-DESIGN.md.

### Q: Can I skip the philosophy?
A: You can skip reading it, but you can't skip *doing* it. Building Form without understanding its nature will fail.

## Next Steps

1. **Read the three documents** in order listed above
2. **Examine existing code** in `gds/src/form/core/`
3. **Start Phase 1** from FORM-EVALUATOR-DESIGN.md
4. **Join discussions** about Application Form protocol
5. **Contribute discoveries** as Form reveals itself

---

*"Understanding Form is not optional for working on GDS - Form IS what makes GDS be GDS."*
