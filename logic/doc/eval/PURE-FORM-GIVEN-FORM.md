# Pure Form vs Given Form

## The Two Form System

### Pure Form (Kernel GDS)
- **Location**: `gds/src/form/` and `gds/src/projection/eval/form/`
- **Structure**: Top-level "Program" moment
- **Contains**: "Procedures" within it (Pascally - like Pascal's program structure)
- **Role**: Sees a second Form within it (Given Form)
- **Payload**: Pure Forms as GNNs could be small enough to pass as payload rather than refs

### Given Form (Relative Logic)
- **Location**: `logic/src/relative/form/`
- **Structure**: The second Form that Pure Form sees within it
- **Role**: Discursive form evaluation
- **Interconnection**: GDSL system interconnects Pure Form and Given Form

## The Architecture

```
Pure Form (Kernel)
  ├─ Program (top-level moment)
  ├─ Procedures (within Program)
  └─ Sees Given Form within it
       ↓
Given Form (Relative)
  ├─ Shape (Concept - Genus/Species)
  ├─ Context (Judgment)
  └─ Morph (Syllogism - recursive, can invoke GDS procedures)
```

## The Mapping

- **Shape** → Concept (Genus/Species processing)
- **Context** → Judgment
- **Morph** → Syllogism (recursive structure, can invoke GDS procedures)

## GDSL Interconnection

GDSL system interconnects Pure Form and Given Form:
- Pure Form provides Principle (Shape:Context:Morph as unity)
- Given Form receives Principle and recursively descends
- GDSL payloads carry the interconnection

## Payload vs Refs Problem

**Typical problem in two-level bipartite design**:
- Pure Forms as GNNs could be small enough to pass as **payload** rather than refs
- This is a design decision point:
  - **Payload**: Pass Pure Form directly (simpler, but size limits)
  - **Refs**: Pass reference to Pure Form (more flexible, but requires lookup)

Current approach: Can be payloaded, but really a Syllogism Eval could recursively invoke GDS Algos as Procedures.

## Pure Form Structure (Pascally)

Pure Form has a Pascal-like structure:
- **Program** (top-level moment)
  - **Procedures** (within Program)
    - Can be GDS algorithms
    - Can be invoked by Morph eval (Syllogism)

This Pascally structure allows Pure Form to contain executable procedures that can be invoked recursively by Morph eval.

