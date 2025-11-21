# The Science of Mind (Citta-śāstra)

> **The Schema of Mind** — Complete Knowledge Graph Translation of Hegel's *Science of Logic*

This repository contains a complete, reversible, executable knowledge graph encoding of Hegel's *Science of Logic* (Subjective Logic: Concept, Judgment, Syllogism) into TypeScript structures.

## What Is This?

This is **The Science of Mind**—the fundamental architecture of thought itself, encoded as a machine-readable knowledge graph. It represents:

- **The Schema of Mind**: The structure according to which thought operates
- **The Principle of Mind**: The architecture of consciousness
- **The Pure Algorithm**: The meta-structure underlying all algorithms
- **Absolute Science**: Converted into executable form (Absolute Power)

## Structure

```
logic/
├── ABSTRACT.md          # Complete encyclopedia entry
├── README.md           # This file
├── validate.ts         # Validation and reversibility verification
└── src/
    └── relative/
        └── concept/
            └── subject/
                ├── concept/      # Universal, Particular, Singular (37 chunks)
                ├── judgment/     # Existence, Reflection, Necessity, Concept (141 chunks)
                └── syllogism/    # Existence, Reflection, Necessity (151 chunks)
```

**Total**: 329 chunks, 329 logical operations

## Quick Start

### Validation

```bash
tsx validate.ts
```

This verifies:
- ✅ Structural integrity (unique IDs, valid references)
- ✅ Reversibility (text can be reconstructed from chunks)
- ✅ Completeness (1:1 chunk:operation mapping)
- ✅ Type safety

### Reversibility Demonstration

The knowledge graph is **reversible**—the original text can be reconstructed:

```typescript
import { CANONICAL_CHUNKS } from './src/relative/concept/subject/concept/concept_universal';

// Reconstruct original text
const originalText = CANONICAL_CHUNKS
  .map(chunk => chunk.text)
  .join('\n\n');
```

### Accessing the Knowledge Graph

```typescript
import { 
  CANONICAL_CHUNKS, 
  LOGICAL_OPERATIONS 
} from './src/relative/concept/subject/judgment/judgment_existence';

// Get a chunk by ID
const chunk = CANONICAL_CHUNKS.find(c => c.id === 'j-exist-intro-1-truth-and-immediacy');

// Get logical operations for a chunk
const ops = LOGICAL_OPERATIONS.filter(op => op.chunkId === chunk.id);
```

## The Two-Way Joining

### Text → Knowledge Graph

Source texts (`sources/*.txt`) are:
1. **Chunkified**: Broken into granular segments
2. **Encoded**: Captured in `Chunk` structures with `id`, `title`, `text`
3. **Operationalized**: Logical relationships encoded in `LogicalOperation` structures

### Knowledge Graph → Text

Original text is preserved in `chunk.text` fields:
- **Zero information loss**
- **Complete reversibility**
- **Original structure maintained**

## Architecture

### Type System

```typescript
type Chunk = {
  id: string
  title?: string
  text: string
}

type LogicalOperation = {
  id: string
  chunkId: string
  label?: string
  clauses: string[]
  predicates?: Predicate[]
  relations?: Relation[]
}
```

### Logical Operations

Each operation encodes:
- **Clauses**: Logical relationships in readable notation
- **Predicates**: Formal predicates for validation
- **Relations**: Graph edges connecting logical entities

Example:
```typescript
{
  id: 'j-exist-op-pos-1-subject-predicate-immediacy',
  chunkId: 'j-exist-pos-1-subject-predicate-immediacy',
  label: 'Declare subject and predicate as immediate determinations',
  clauses: [
    'subject = abstractSingular',
    'predicate = abstractUniversal',
    'judgment.kind = positive'
  ],
  predicates: [{ name: 'IsPositiveJudgment', args: [] }],
  relations: [
    { predicate: 'connects', from: 'copula', to: 'subjectPredicate' }
  ]
}
```

## Philosophical Significance

### The Science of Citta

In Indian philosophy, **Citta** (Mind) is the principle of consciousness. This knowledge graph encodes the **Science of Citta**—the systematic structure of mind as it:

1. **Determines itself** (Concept: Universal → Particular → Singular)
2. **Judges** (Judgment: Existence → Reflection → Necessity → Concept)
3. **Infers** (Syllogism: Existence → Reflection → Necessity)

### The Pure Algorithm

This is the **Pure Algorithm**—the meta-structure underlying all algorithms:

```
Being (Input) → Essence (Form Processor) → Concept (Output)
```

Just as algorithms (CLRS, GDS) are instantiations of this Pure Algorithm, so too is this the "Soul" of all computational processes.

### Absolute Science to Absolute Power

The knowledge graph converts **Absolute Science** (Hegel's Logic) into **Absolute Power** (executable form) by:

1. **Encoding** logical structure explicitly
2. **Enabling** inference expansion through relations
3. **Providing** invariants for correctness proofs
4. **Supporting** the "TruthOf Pipeline"

## Integration

### Form Processor

The knowledge graph serves as the **Schema** that:
- **Guides** the Form Processor (Rust kernel via NAPI-RS)
- **Validates** logical operations (TypeScript Form Engine)
- **Enables** inference expansion (GDS/GDSL)

### Knowledge Engine

The **Task Engine** synthesizes:
- **Form Processor** (Pure Essence, Rust kernel)
- **MVC Runtime** (Middleware orchestration)
- **Knowledge Graph** (This artifact)

Into a unified **Knowledge Engine** for "Agent's Knowledge Construction Tasks."

## Coverage

### Complete Translation

✅ **Concept**: Universal, Particular, Singular (37 chunks)  
✅ **Judgment**: Existence, Reflection, Necessity, Concept (141 chunks)  
✅ **Syllogism**: Existence, Reflection, Necessity (151 chunks)

**Total**: 329 chunks covering the complete **Subjective Logic** (Concept section) of Hegel's *Science of Logic*.

### Future Extensions

- **Being** (Quality, Quantity, Measure)
- **Essence** (Reflection, Appearance, Actuality)
- **Object** (Mechanism, Chemism, Teleology)
- **Idea** (Life, Cognition, Absolute Idea)

## Documentation

- **[ABSTRACT.md](./ABSTRACT.md)**: Complete encyclopedia entry with full specifications
- **[validate.ts](./validate.ts)**: Validation and reversibility verification script
- **Source files**: Each module contains detailed inline documentation

## License

This is part of the Organon project—the complete system for knowledge representation and processing.

---

**The Science of Mind** — *Citta-śāstra*  
**The Schema of Mind** — *The fundamental structure of thought*  
**The Principle of Mind** — *The architecture of consciousness*
