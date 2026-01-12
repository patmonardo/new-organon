# Relative Form Eval (Discursive Form Evaluation)

**Discursive form evaluation** - the TS-side evaluation engine that recursively descends into Shape:Context:Morph.

**This is Determinate Science (scientific speech), not ordinary speech (BI/Business Forms).**

## The Principle

Following Fichte: **Pure Form (kernel) provides Principle**, and this Principle is **Relative Form Shape:Context:Morph as a unity**.

The **Relative form/eval** must **Recursively Descend into all three** (Shape, Context, Morph).

## Kantian Principle of Consciousness

> "The representation that another representation is within me."

This is what form/eval enacts:
- Pure Form (kernel) provides the Principle
- Relative form/eval receives this Principle
- The Principle **enters into** the discursive evaluation (like brahman entering into manifestation)
- Relative form/eval recursively descends into Shape → Context → Morph

## Architecture

```
Pure Form (Kernel GDS)
  ↓ provides Principle
Relative Form Eval (TS)
  ↓ recursively descends into
Shape → Context → Morph
```

### The Recursive Descent

1. **Shape Eval**: Evaluates Active Consciousness moment
   - Receives Shape as Principle
   - **Concept** (Genus/Species) - Determinate Science
   - Descends into Shape's structure
   - Produces aspectual determinations

2. **Context Eval**: Evaluates Active Determination-of-Reflection
   - Receives Context as Principle
   - **Judgment** (Subject/Predicate) - Determinate Science
   - Descends into Context's presuppositions, scope, conditions
   - Produces reflective determinations

3. **Morph Eval**: Evaluates Active Ground
   - Receives Morph as Principle
   - **Syllogism** (can invoke GDS procedures) - Determinate Science
   - Descends into Morph's patterns/transformations
   - Produces grounding determinations

### The Unity

The Principle is **Shape:Context:Morph as unity** - not three separate things, but one Principle that manifests as three moments.

Relative form/eval must:
- Receive the Principle as unity
- Descend into each moment
- Maintain the unity throughout descent

## Boundary with Kernel

- **Kernel form/eval** (`gds/src/projection/eval/form/`): Pure Form execution, produces TruthSteps
- **Discursive form/eval** (`logic/src/relative/form/eval/`): Relative Form evaluation, produces aspectual determinations

## Eval as Witness (Emanant)

**Eval = Emanant** (external witness)  
**Dialectic = Immanent** (runs through everything)

- **Container** = value-less type system (Shape/Context/Morph)
- **Contained** = Value, mostly crafted into an Aspect
- **Dialectic** runs through everything (immanent process)
- **Eval** witnesses the dialectical process (emanant witness)

**E-val** = witnessing the val for **Extraction → Print** operation

Eval places itself **outside** the Value pipeline - it witnesses and extracts, it doesn't participate in the dialectical process itself.

See [EVAL-AS-WITNESS.md](./EVAL-AS-WITNESS.md) for the full architectural insight.

The kernel provides Principle; discursive eval receives and descends.

