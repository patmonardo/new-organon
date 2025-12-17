# FormShape as Executable Graphs: First Principle

*The Primal Image precedes Execution*

## The Idealist Architecture

### Factory → Image → Eval

```
┌─────────────────────────────────────────────────────────────┐
│                   THE ABSOLUTE FORM'S KERNEL                │
│                                                             │
│  projection/factory/                                        │
│  ↓                                                          │
│  LAYS DOWN THE PRIMAL IMAGE                                │
│  (GraphStore = The Given, Das Gegebene)                    │
│                                                             │
│  projection/eval/                                           │
│  ↓                                                          │
│  RUNS ON THE PRIMAL IMAGE                                  │
│  (Procedures, ML, Form = The Derived, Das Abgeleitete)     │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

## The Separation

**Not** debugging Projection Eval yet.
**First** understand: Factory creates, Eval executes.

### Factory (CAR - Given)

**Location**: `gds/src/projection/factory/`

**Purpose**: Ingestion of external data → GraphStore (Primal Image)

**What it does**:
- Takes Arrow tables, Polars DataFrames, etc. (data in memory)
- Infers schema
- Maps IDs (external → internal GDS IDs)
- Builds GraphStore (nodes + relationships + properties)
- **This IS the Primal Image** - the graph "as given"

**Key trait**:
```rust
pub trait GraphStoreFactory: Send + Sync {
    fn build_graph_store(&self, config: &Self::Config) 
        -> Result<DefaultGraphStore, Self::Error>;
}
```

**Implementations**:
- `arrow::ArrowNativeFactory` - Arrow → GraphStore
- `csr_huge::CsrHugeGraphStoreFactory` - CSR/Huge style (delegates to Arrow)
- Future: Polars, DuckDB, Neo4j connectors

**Philosophy**: This is **Receptivity** (Kant) - receiving the manifold of intuition.
The GraphStore is the **Sensibility** - space (nodes) and time (relationships).

### Eval (CDR - Derived)

**Location**: `gds/src/projection/eval/`

**Purpose**: Execution of computations ON the GraphStore (Primal Image)

**What it does**:
- Takes GraphStore (already constructed by Factory)
- Executes Procedures (PageRank, Dijkstra, Louvain...)
- Executes ML Pipelines (LinkPrediction, NodeClassification...)
- Executes Forms (Triadic synthesis of Procedure + ML)
- Returns results (f64 streams, tensors, FormShapes)

**The Three ISA**:
```rust
eval/procedure/  ← AlgorithmSpec → f64 streams
eval/ml/         ← Pipeline → tensors
eval/form/       ← FormSpec → FormShape (Union of above)
```

**Philosophy**: This is **Spontaneity** (Kant) - the understanding's concepts applied to intuition.
The Eval is **Understanding** - making judgments about the given.

## The Idealism

### Fichte's Science of Knowing

**This IS Fichte's Transcendental Logic as code!**

The GDS Kernel is the **I** (das Ich) - pure self-positing activity:
1. **Self-Positing**: Kernel exists as pure activity (not data, but Tathandlung)
2. **Externalization**: Kernel posits the Not-I (FormShape TS-JSON)
3. **Recognition**: Kernel recognizes itself in FormShape (execution returns FormShape)

**Individuation** (Hegel): The abstract I/Not-I individuates into concrete actuality:
- GDS Kernel (Universal activity) → FormShape (Particular structure) → Explanation (Individual knowledge) → "The Rose" (Actual presentation)

The entire architecture IS German Idealism:
- **Kant**: Factory (Receptivity) + Eval (Spontaneity)
- **Fichte**: Kernel externalizes into FormShape TS-JSON
- **Hegel**: FormShape individuates into Explanations for users

(See [FICHTE-SCIENCE-OF-KNOWING.md](./FICHTE-SCIENCE-OF-KNOWING.md) for complete analysis)

### Why "Idealist"?

The Image comes first. The execution follows.

1. **Factory lays down the Primal Image** (GraphStore)
   - This is the "Ideal" - the Form of the graph
   - Not yet computed, not yet executed
   - Pure structure (nodes, relationships, properties)

2. **Eval runs ON the Image**
   - Procedures project the Image into f64 streams
   - ML projects the Image into tensors
   - Form projects the Image into synthesis

The GraphStore is **not real data** - it's an **idealized representation**.
The graph doesn't "exist" until projected through Eval.

### Kant's Transcendental Idealism

```
Sensibility (Factory)    → Intuition (GraphStore)
     ↓
Understanding (Eval)     → Concepts (Algorithms)
     ↓
Judgment (Form)          → Synthesis (Results)
```

The GraphStore is the "manifold of intuition" - formless content.
The Eval applies "categories of understanding" - algorithms, pipelines.
The result is "experience" - PageRank scores, predictions, insights.

### Berkeley's Idealism

*"To be is to be perceived"*

The graph doesn't exist until executed:
- Factory creates the **possibility** of graph (esse = posse)
- Eval actualizes through execution (percipere = actualize)
- FormShape transmits the **perception** (not the data)

## FormShape as Executable Graph / RDF Dataset

### What IS FormShape?

FormShape is NOT the graph data.
FormShape IS the **execution specification** - the IDEA of what to compute.

Think of it as:
- **RDF Dataset structure** (semantic web, triples)
- **SPARQL Query** as first-order logical inference
- **Concept as transmissible entity**

The Logic package knowledge graph is essentially RDF:
- Chunks = RDF resources/entities
- Operations = RDF predicates/relations  
- FormShape = SPARQL-like query concept

```rust
pub struct FormShape {
    // Six Pillars (The Absolute Form's structure)
    id: FormId,              // Identity
    shape: Shape,            // Structure (what fields exist)
    context: Context,        // Relations (how they connect)
    morph: Morph,            // Operations (what to execute)
    // + two more from form/core/...
}
```

### The Protocol: FormShape → FormShape

```
GDSL (Logic layer)
  ↓ sends
FormShape {
    shape: "Need PageRank on graph X",
    context: "Use 20 iterations, damping 0.85",
    morph: ["pagerank"],
}
  ↓ to
GDS Kernel (eval layer)
  ↓ executes on
Primal Image (GraphStore from factory)
  ↓ returns
FormShape {
    shape: "PageRank scores for 1000 nodes",
    context: "Converged in 15 iterations",
    morph: ["pagerank_result"],
    data: [serialized scores],
}
  ↓ back to
GDSL (Logic layer)
  ↓ manifests as
"The Rose Is Red" (User sees results)
```

### Why "Executable"?

FormShape carries the **imperative** - the command to execute.

Not: "Here is data to process"
But: "Here is what I want you to DO with the Primal Image"

The FormShape is executable because:
1. It specifies operations (morph.patterns: ["pagerank"])
2. It provides context (graph name, parameters)
3. It defines expected output (shape.required_fields)
4. It can be transmitted, stored, replayed

### The Factory's Role

Factory creates GraphStore → **The Given** (Primal Image)

Eval receives FormShape → **The Task** (what to derive)

Eval runs on GraphStore using FormShape → **The Result** (derived)

**The Factory is NOT involved in execution.**

Factory runs ONCE: Arrow table → GraphStore
Eval runs MANY times: FormShape + GraphStore → Results

## The Current State

### What Works

✅ **Factory**: Arrow → GraphStore works
  - `projection/factory/arrow/` is complete
  - Can ingest nodes, relationships, properties
  - Produces GraphStore (Primal Image)

✅ **Eval/Procedure**: Some algorithms work
  - `projection/eval/procedure/` exists
  - Can run procedures on GraphStore
  - Returns f64 streams

🔄 **Eval/ML**: Under construction
  - `ml/` exists (not yet in eval/)
  - Pipelines defined but not wired
  - Tomorrow: implement ML Graph Algorithms

❌ **Eval/Form**: Skeleton only
  - `projection/eval/form/` structure exists
  - PageRankFormSpec created today
  - Not wired to actual execution yet
  - FormShape is stub type

### What's Next

**Tomorrow**: Implement ML Graph Algorithms
- Not Form Eval yet
- Just ML procedures (LinkPrediction, NodeClassification)
- Working on "eval/ml/" infrastructure

**Later**: Wire Form Evaluator
- Connect FormSpec to actual AlgorithmSpec and Pipeline
- Implement TriadicCycle.execute()
- FormShape JSON protocol

**Eventually**: Projection Factory emergence
- Factory creates more sophisticated projections
- Maybe: Factory can accept FormShape instructions?
- "Project graph X with these constraints" → GraphStore

## The Philosophical Stack

### Layer 1: Mahat (GraphStore)

**The Undifferentiated Whole**

Factory produces GraphStore:
```rust
pub struct DefaultGraphStore {
    nodes: NodeStore,
    relationships: RelationshipStore,
    properties: PropertyStore,
}
```

This is **pure potentiality** - all possible computations exist here virtually.

### Layer 2: Buddhi (Form)

**The Discriminating Intelligence**

Form receives FormShape and discriminates:
- Is this Procedure? (Thesis)
- Is this ML? (Antithesis)
- Is this both? (Synthesis)

Form projects GraphStore into **three possibilities**:
```rust
pub enum TriadicCycle {
    thesis: Thesis,        // Procedure
    antithesis: Antithesis, // ML
    synthesis: Synthesis,   // Union
}
```

### Layer 3: Ahamkara (Pipeline/Judgment)

**The Self-Assertion, "I Execute"**

Pipeline takes the discrimination and ASSERTS:
"I am LinkPrediction"
"I execute these steps"
"I produce this output"

```rust
impl Pipeline for LinkPredictionTrainingPipeline {
    fn execute(&self, graph: GraphStore) -> Result<Model> {
        // "I execute" - Ahamkara
    }
}
```

### Layer 4: Manas (Execution)

**The Concrete Mind, Actual Computation**

The actual running code:
- Memory access (load node properties)
- Arithmetic (compute PageRank formula)
- I/O (write results)

This is **no longer Form** - this is matter (code running on CPU).

## First Principle: The Image Precedes

**Factory**: Lays down the Primal Image (GraphStore)
**Eval**: Executes ON the Image (Procedures, ML, Form)
**FormShape**: Carries the execution imperative (what to do with Image)

The Image is not debugged yet.
The Eval is not debugged yet.
First: Build the ML algorithms.
Then: Wire Form to execute them.
Eventually: See how Factory emerges into consciousness.

---

*"The Primal Image precedes execution. Factory lays it down, Eval runs on it. This is Idealism - the Idea comes first, the execution follows."*

## Practical Next Steps

### Tomorrow: ML Graph Algorithms

**Don't touch**:
- Projection Factory (it works)
- Form Evaluator (not ready)

**Do implement**:
- ML Graph Algorithms in `ml/`
- Get them ready for `eval/ml/` integration
- Focus on Algorithm implementations, not Eval wiring yet

### Later: Form Evaluation

Once ML algorithms exist:
1. Wire `eval/ml/` to execute them
2. Wire `eval/form/` to compose Procedure + ML
3. Implement TriadicCycle execution
4. FormShape JSON protocol

### Eventually: Factory Emergence

When everything runs:
- Study how Factory creates projections
- Consider: Can FormShape guide projection?
- "Project with these properties" → specialized GraphStore
- Factory becomes conscious through Form

For now: **Factory lays down the Image. Eval runs on it. Understand the separation.**
