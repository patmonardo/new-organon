# ADR-002: Form Server Architecture - GDS, Logic, Model Layering

## Status
Draft - Under Discussion

## Context

We need to clarify the layered architecture of Form Servers across GDS, @logic, and @model packages.

**GDSL defines how both Form Servers work:**
- Absolute Form Server (GDS) — mocked for now
- Relative Form Server (@logic) — what we are defining

The immediate work is defining the **Relative Form Server in @logic**.

## The Layers

### GDS - Absolute Form Server (Implements Reason)

GDS provides a Form Server at the Application Layer:
- **Immediate**: Graph Algorithms as Procedures
- **Mediated**: Pipelines (composed procedures)
- **Status**: Defined and implemented to Gamma (connection mocked for now)
- **Implements**: REASON (Vernunft) — Transcendental
- **Mode**: TRANSCENDENTAL

GDS Form Server is **Pure** relative to @logic because @logic presupposes it as **Absolute**.

```
GDS Form Server (Absolute) — Implemented to Gamma
├── Implements REASON (Vernunft)
├── Mode: TRANSCENDENTAL
├── Procedures (immediate Graph Algo)
├── Pipelines (mediated composition)
└── Connection mocked for development
```

### @logic - Relative Form Server (Implements Understanding)

@logic is the **Relative Form Server**:
- Presupposes GDS as Absolute (connection mocked for now)
- Is Relative to GDS
- BEC IR, FactStore, ShapeEngine
- TypeScript representation of dialectical logic
- **Implements**: UNDERSTANDING (Verstand) — Ordinary
- **Mode**: ORDINARY (Understanding with Abstract Reason is Dialectical)
- **This is what we are currently defining**

```
@logic Relative Form Server — DEFINING NOW
├── Implements UNDERSTANDING (Verstand)
├── Mode: ORDINARY
├── Presupposes GDS as Absolute (connection mocked)
├── BEC IR (Being-Essence-Concept)
├── FactStore, ShapeEngine, EntityEngine
├── FormShape, FormEntity
└── TypeScript dialectical logic
```

### @model - MVC App Server (SDSL Runtime)

MVC (@model) is an App Server:
- **Runtime** - executes form processing
- **Client/Proxy to GDSL** - speaks the GDS language
- **In itself is a SDSL** - Special Data Science Language for MVC

```
@model MVC App Server
├── Runtime (executes)
├── Client/Proxy to GDSL
├── SDSL in itself (MVC Standard Library)
└── FormStore, Server Actions, React Adapters
```

## The Relationships

```
┌─────────────────────────────────────────────────────────────────┐
│  GDS - PURE FORM SERVER (Application Layer)                     │
│  ─────────────────────────────────────────────────────────────  │
│  • Procedures: Immediate Graph Algorithms                       │
│  • Pipelines: Mediated Compositions                             │
│  • [Definition in progress]                                     │
│  • Is ABSOLUTE from @logic's perspective                        │
└─────────────────────────────────────────────────────────────────┘
                              ▲
                              │ presupposes as Absolute
                              │
┌─────────────────────────────────────────────────────────────────┐
│  @logic - FORM PROCESSOR (Relative to GDS)                      │
│  ─────────────────────────────────────────────────────────────  │
│  • BEC IR, FactStore, ShapeEngine                               │
│  • Presupposes GDS as Absolute                                  │
│  • TypeScript dialectical logic                                 │
└─────────────────────────────────────────────────────────────────┘
                              ▲
                              │
┌─────────────────────────────────────────────────────────────────┐
│  @model - MVC APP SERVER (SDSL Runtime)                         │
│  ─────────────────────────────────────────────────────────────  │
│  • Runtime for form processing                                  │
│  • Client/Proxy to GDSL                                         │
│  • In itself is a SDSL                                          │
│  • FormStore, MVC triadic, Server Actions                       │
└─────────────────────────────────────────────────────────────────┘
```

## Key Distinctions

| Package | Role | Implements | Mode |
|---------|------|------------|------|
| GDS | Absolute Form Server | **Reason** (Vernunft) | TRANSCENDENTAL |
| @logic | Relative Form Server | **Understanding** (Verstand) | ORDINARY |
| @model | MVC App Server | SDSL Runtime | Client/Proxy |

### Understanding vs Reason — Ordinary vs Transcendental

- **Understanding (Verstand)**: ORDINARY mode
- **Reason (Vernunft)**: TRANSCENDENTAL mode
- **Understanding with Abstract Reason**: DIALECTICAL

The Relative Form Server (@logic) operates at the level of Understanding in Ordinary mode.

The Absolute Form Server (GDS) operates at the level of Reason in Transcendental mode.

When Understanding engages with Abstract Reason, the result is Dialectical processing.

## GDSL vs SDSL

- **GDSL** (Graph Data Science Language): The language GDS speaks
- **SDSL** (Special Data Science Language): Domain-specific languages built on GDSL
- **MVC SDSL**: The first SDSL - Standard Library for MVC forms

@model is:
1. A **Client/Proxy** to GDSL (speaks to GDS)
2. An **SDSL in itself** (provides MVC standard library)

## Current Work

**GDSL defines both Form Servers:**
1. Absolute Form Server (GDS) — mocked for now
2. Relative Form Server (@logic) — **defining now**

**Immediate Focus:**
- Define @logic Relative Form Server
- Mock GDS Absolute Form Server interface
- @model MVC SDSL consumes @logic Relative Form Server

## Open Questions

1. What is the complete interface of @logic Relative Form Server?
2. What does the mock GDS Absolute Form Server look like?
3. How does @model MVC SDSL consume @logic?

## Consequences

- @logic Relative Form Server is the foundation
- GDS Absolute Form Server can be mocked until ready
- @model MVC SDSL builds on @logic Relative Form Server
- Ordinary business forms run on dialectical foundation

## Store Architecture - Property as Ontological Middle

**Store** has a precise meaning in the context of **PropertyStore**:

```
Store ←── Property ──→ Values
              │
          (Center)
     (Ontological Middle)
```

### The Triadic Structure

| Component | Role | Example |
|-----------|------|---------|
| **Store** | Container (repository) | GraphStore, FactStore, FormStore |
| **Property** | Middle (mediator) | Connects Store to Values |
| **Values** | Content (determinate) | The actual data |

### Property as Center

Property is the **Center** for the root Codegen/Eval Projection system:

```rust
trait Property {
    fn schema(&self) -> &PropertySchema;  // Shape/Structure
    fn values(&self) -> Arc<dyn PropertyValues>;  // Content
}
```

Property:
1. Has `schema()` — the shape/structure definition
2. Has `values()` — the actual data values
3. Is the **pivot point** for Codegen/Eval

### Projection Levels (HyperPropertyStore)

```
Monadic  (1) — Simple Unity
Dyadic   (2) — Reflective Mark (no middle)
Triadic  (3) — Determinate Mark (with middle) ← Property as Center
Tetradic (4) — ML Pipeline (4-4-4-4 pattern)
Pentadic (5) — Absolute Idea
```

The Triadic level introduces the **CenterMark** — the middle element that determines the relation. This is where Property becomes the ontological mediator.

---

*This ADR is a draft for discussion. Terminology and relationships to be refined.*

