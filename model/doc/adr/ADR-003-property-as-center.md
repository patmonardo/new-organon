# ADR-003: Property as the Center of Codegen and Eval

## Status
Accepted

## Context

This ADR captures the central organizing principle of the entire ORGANON architecture.

## The Core Insight

**Property is the invariant center of both Codegen and Eval.**

Everything in the system structures itself around this single idea.

```
                    ┌─────────────────┐
                    │    PROPERTY     │
                    │   (The Center)  │
                    │                 │
                    │ schema ─┬─ values│
                    └────────┼────────┘
                             │
         ┌───────────────────┼───────────────────┐
         │                   │                   │
         ▼                   │                   ▼
    ┌─────────┐              │              ┌─────────┐
    │ CODEGEN │              │              │  EVAL   │
    └─────────┘              │              └─────────┘
         │                   │                   │
    Static                   │              Dynamic
    Compile-time             │              Runtime
    Generate from            │              Execute with
    schema                   │              values
```

## Property Structure

```rust
trait Property {
    fn schema(&self) -> &PropertySchema;      // Structure (for Codegen)
    fn values(&self) -> Arc<dyn PropertyValues>; // Content (for Eval)
}
```

**Property has two faces:**
1. `schema()` — The structure/shape definition (Codegen reads this)
2. `values()` — The actual data content (Eval uses this)

## Property as Ontological Middle

Property is not just data — it's the **ontological middle** that connects:

```
Store ←── Property ──→ Values
              │
          (Center)
     (Ontological Middle)
```

| Component | Role |
|-----------|------|
| **Store** | Container (repository) |
| **Property** | Middle (mediator) — THE CENTER |
| **Values** | Content (determinate) |

## How Everything Serves This Center

### Layer Architecture

| Layer | Codegen (from schema) | Eval (with values) |
|-------|----------------------|-------------------|
| **GDS** | Procedures, Pipelines | Graph execution |
| **@logic** | FormShape definition | FormEntity processing |
| **@model** | MVC generation, UI schemas | Form execution, Server Actions |
| **@task** | Workflow generation | Knowledge orchestration |

### Concrete Mappings

| Concept | Property Aspect | Phase |
|---------|----------------|-------|
| FormShape | `schema()` | Codegen |
| FormEntity | `values()` | Eval |
| Store | Container for Properties | Both |
| GDSL/SDSL | Languages expressing Property operations | Both |

### The God-to-Mortal Mapping

The entire "God-to-Mortal" architecture serves this:

```
Absolute Form Server (GDS)     ← Property schemas define Reason
         ↓
Relative Form Server (@logic)  ← Property mediates Understanding
         ↓
MVC App Server (@model)        ← Property drives business forms
         ↓
Mortal Applications            ← Property values render to UI
```

## Projection Levels

Property emerges at the **Triadic** level with a **CenterMark**:

```
Monadic  (1) — Simple Unity
Dyadic   (2) — Reflective Mark (no middle)
Triadic  (3) — Determinate Mark (WITH middle) ← Property emerges as Center
Tetradic (4) — ML Pipeline
Pentadic (5) — Absolute Idea
```

The Triadic introduces the middle element that determines the relation.
This is where Property becomes the ontological mediator.

## The Projection System

```
Property.schema() → Codegen → Generated Code
                              ↓
Property.values() → Eval ──→ Execution Result
```

Property is the **single source of truth** that both:
1. **Codegen** reads to generate code (static, compile-time)
2. **Eval** reads to execute that code (dynamic, runtime)

## Why This Matters

If you understand that **Property is the center of Codegen and Eval**, you understand:

1. Why GDS has PropertyStore at its core
2. Why @logic defines FormShape/FormEntity as Property aspects
3. Why @model MVC generates from schema and executes with values
4. Why the Store pattern is universal (GraphStore, FactStore, FormStore)
5. Why GDSL/SDSL are structured as they are

**Everything in the system structures itself to serve Property as the invariant center.**

## Consequences

- All architectural decisions flow from this principle
- New features must respect Property as center
- Codegen and Eval are not separate concerns — they are two faces of Property
- Store implementations are Property containers
- Languages (GDSL/SDSL) are Property expression systems

---

*This is the foundational ADR. All other architectural decisions derive from this principle.*

