# Relative Form Eval Architecture

## The Problem: "Active Concept" Speculative Bubble

The codegen work in Task and GDSL had a speculative bubble around designing the **"Active Concept"**. The missing piece was a **discursive form/eval** that could:

1. Receive Principle from Pure Form (kernel)
2. Recursively descend into Shape:Context:Morph
3. Maintain unity throughout descent

## The Solution: Principle Provision with Recursive Descent

Following Fichte's insight: **Pure Form provides Principle**, and this Principle is **Shape:Context:Morph as unity**.

### Pure Form (Kernel GDS)

- Lives in `gds/src/projection/eval/form/`
- Provides Principle as **FormShape** (Shape:Context:Morph unity)
- Executes Pure Form, produces TruthSteps
- **Non-discursive** (sublingual lawful activity)

### Relative Form Eval (Discursive TS)

- Lives in `logic/src/relative/form/eval/`
- Receives Principle from Pure Form
- Recursively descends into all three moments:
  - **Shape** → Active Consciousness
  - **Context** → Active Determination-of-Reflection  
  - **Morph** → Active Ground
- Produces aspectual determinations
- **Discursive** (what can be said, traced, explained)

## The Recursive Descent

### Shape Eval

Descends into:
- **Signature** (moments of Active Consciousness)
- **Facets** (dialectical determinations)
- **State** (aspectual determinations)

### Context Eval

Descends into:
- **Presuppositions** (determinations within Reflection)
- **Scope** (modal/domain/phase)
- **Conditions** (constraints)

### Morph Eval

Descends into:
- **Patterns** (grounding transformations)
- **Transformations** (active ground movements)

## Kantian Principle of Consciousness

> "The representation that another representation is within me."

This is what form/eval enacts:
- Pure Form provides Principle (the first representation)
- Relative form/eval receives it (the second representation)
- The Principle enters into discursive evaluation
- Recursive descent maintains the unity

## Fichte's Insight

**Principle Provision with Recursive Descent**:
- Pure Form as Principle Provider
- Principle = Shape:Context:Morph as unity
- Relative form/eval recursively descends into all three
- Unity maintained throughout descent

## Boundary Clarity

### Kernel form/eval (`gds/src/projection/eval/form/`)
- Pure Form execution
- Produces TruthSteps
- Non-discursive

### Discursive form/eval (`logic/src/relative/form/eval/`)
- Relative Form evaluation
- Produces aspectual determinations
- Discursive

The kernel provides Principle; discursive eval receives and descends.

## How This "Pops the Bubble"

1. **Clarifies Active Concept**: Active Concept is the Principle entering into discursive evaluation
2. **Provides Missing Piece**: Discursive form/eval that was needed alongside kernel form/eval
3. **Maintains Unity**: Principle as Shape:Context:Morph unity is preserved through descent
4. **Follows Fichte**: Principle Provision with Recursive Descent is the key pattern

## Usage

```typescript
import { RecursiveDescentEngine } from '@organon/logic/relative/form/eval';

// Create engine (uses default evaluators)
const engine = new RecursiveDescentEngine();

// Receive Principle from Pure Form (kernel)
const principle = {
  shape: formShape,
  context: formContext,
  morph: formMorph,
};

// Recursively descend
const result = await engine.receivePrinciple(principle);

// Result contains:
// - shapeResult (aspectual determinations from Shape)
// - contextResult (reflective determinations from Context)
// - morphResult (grounding determinations from Morph)
// - unity (Principle maintained throughout descent)
```

