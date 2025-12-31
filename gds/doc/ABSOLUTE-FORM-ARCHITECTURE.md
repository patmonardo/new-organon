# Absolute Form Architecture

## The Fundamental Architecture

The **Entire Organon Project** is the **Absolute Form**. It does not derive from anything - it is the foundational reality.

```
┌─────────────────────────────────────────────────────────┐
│           ABSOLUTE FORM                                  │
│        (Entire Organon Project)                          │
│        - Does not derive from anything                  │
│        - The foundational reality                       │
└───────────────────┬─────────────────────────────────────┘
                    │
        ┌───────────┼───────────┐
        │           │           │
        ▼           ▼           ▼
    ┌────────┐  ┌────────┐  ┌────────┐
    │CODEGEN │  │FACTORY │  │  EVAL  │
    │        │  │        │  │        │
    │Presup- │  │Projec- │  │Emerges │
    │posed   │  │tion    │  │Third   │
    │Reality │  │Proper  │  │        │
    └────────┘  └────────┘  └───┬────┘
                                 │
                    ┌────────────▼────────────┐
                    │    PURE FORM            │
                    │  (GDS Kernel)           │
                    │  Absolute Logic /       │
                    │  Absolute Science       │
                    └────────────┬────────────┘
                                  │
                    ┌─────────────▼─────────────┐
                    │    IMPURE FORM            │
                    │  (Relative Logic)         │
                    │  TS Agential Logic Layer │
                    └──────────────────────────┘
```

## The Three Moments of Absolute Form

### 1. Codegen - Presupposed Reality

**Location**: `gds/src/projection/codegen/`

**Role**: The presupposed reality - the code generation infrastructure that makes everything else possible.

**What it does**:
- Generates algorithm specifications
- Creates algorithm configs and validation
- Defines property value types
- Generates factory macros
- Provides the foundational code structure

**Philosophical Role**: This is the **presupposed** moment - it exists before projection and evaluation. It is the reality that must be in place for anything to happen.

### 2. Factory - Projection Proper

**Location**: `gds/src/projection/factory/`

**Role**: Projection Proper - the act of projecting native data into GraphStores.

**What it does**:
- Creates GraphStores from native data sources (Arrow, Polars, etc.)
- Handles schema inference
- Maps native types to GDS types
- Performs ID mapping
- Constructs the graph topology

**Key Components**:
- `ArrowNativeFactory` - Arrow → GraphStore projection
- `GraphStoreFactory` trait - Contract for all factory implementations
- `ArrowProjectionConfig` - Configuration for projection operations

**Philosophical Role**: This is **Projection Proper** - the actual act of projecting reality (native data) into form (GraphStore). This is where the "concealing power" operates - concealing the contingent details of the source to produce a stable, abstract Image.

### 3. Eval - Emerges Third

**Location**: `gds/src/projection/eval/`

**Role**: The evaluation moment - where Pure Form emerges.

**What it does**:
- Executes Procedures (graph algorithms)
- Executes ML Pipelines (training and prediction)
- Executes Forms (the unifying layer)
- Manages computation and storage runtimes
- Orchestrates the complete execution lifecycle

**Key Components**:
- `procedure/` - Procedure Executor (spec.rs execution)
- `ml/` - ML Pipeline Executor
- `form/` - Form Processor (Pure Form execution)

**Philosophical Role**: This is the **third moment** - it emerges after Codegen and Factory. It is in the Eval moment that **Pure Form emerges**.

## Pure Form Emergence

**Pure Form** emerges from the **Eval moment**, specifically from `gds/src/projection/eval/form/`.

### Pure Form Structure

```
Pure Form (GDS Kernel)
  ├─ Program (top-level moment)
  ├─ Procedures (within Program)
  └─ Sees Given Form within it
       ↓
Given Form (Relative Logic)
  ├─ Shape (Concept - Genus/Species)
  ├─ Context (Judgment)
  └─ Morph (Syllogism - recursive, can invoke GDS procedures)
```

### Pure Form Components

- **FormProcessor** (`gds/src/projection/eval/form/executor.rs`)
  - Executes Form evaluation
  - Returns ResultStore (GraphStore)
  - Uses ExecutionContext from procedure catalog

- **PureFormProcessor** (`gds/src/projection/eval/form/pure_executor.rs`)
  - Substrate-facing entrypoint
  - Does not depend on procedure graph catalog
  - Base graph provided directly

- **Form Operators**:
  - `EssenceFormOperator` - First moment (essentiality / presupposed)
  - `ShineFormOperator` - Second moment (positedness)
  - `ReflectionFormOperator` - Third moment (reflection)
  - `PassThroughFormOperator` - Identity operator
  - `CommitSubgraphOperator` - Commits subgraph
  - `MaterializeNodePropertiesOperator` - Materializes node properties
  - `MaterializeRelationshipPropertiesOperator` - Materializes relationship properties

### Pure Form as Absolute Logic / Absolute Science

**Absolute Science** is the Science of the Absolute Form.

Pure Form is:
- **Absolute Logic**: The logical structure that makes reasoning possible
- **Absolute Science**: The Science of the Absolute Form
- **GDS Kernel**: The core computational substrate

## The Division via Projection

**Absolute Form divides via Projection** into:

1. **Pure Form** (within GDS)
2. **Impure Form** (which presupposes Application Form)

```
Absolute Form
  │
  │ (via Projection)
  │
  ├─→ Pure Form (GDS Kernel)
  │   └─ Absolute Logic / Absolute Science
  │
  └─→ Impure Form (Relative Logic)
      └─ Presupposes Application Form
          └─ Receives (via Receptivity) objects from Relative FormProcessor
```

## Impure Form (Relative Logic)

**Impure Form** emerges from **Pure Form** via the division of Absolute Form through Projection. This is the **TS Agential Logic Layer** (Relative Logic).

**Location**: `logic/src/relative/form/`

**Role**: Discursive form evaluation - what can be said, traced, explained.

**What it does**:
- Presupposes **Application Form**
- Receives Principle from Pure Form
- Recursively descends into Shape:Context:Morph
- Maintains unity throughout descent
- Produces aspectual determinations

**Key Components**:
- `Shape` → Concept (Genus/Species processing)
- `Context` → Judgment
- `Morph` → Syllogism (recursive structure, can invoke GDS procedures)

## Application Form

**Application Form** is presupposed by **Impure Form**.

**Role**: Receives (via **Receptivity**) objects from the **Relative FormProcessor**.

**What it does**:
- Acts as the interface between Impure Form and Relative FormProcessor
- Receives objects through Receptivity (the capacity to receive)
- Provides the form structure that Impure Form presupposes

**Key Relationship**:
- **Impure Form** presupposes **Application Form**
- **Application Form** receives (via Receptivity) objects from **Relative FormProcessor**
- This creates the connection between the discursive layer (Impure Form) and the processor layer (Relative FormProcessor)

## The Projection Machine

The **Projection Machine** is the manifestation of the **Absolute Form**. It consists of:

1. **Codegen** - Presupposed reality
2. **Factory** - Projection Proper (concealing power)
3. **Eval** - Evaluation (revealing power)

### Factory as Concealing Power

Factory **conceals** the contingent details of the source:
- File formats
- Physical storage
- Raw IDs
- Native data structures

To produce a stable, abstract **Image** (GraphStore).

### Eval as Revealing Power

Eval **reveals** determinate structures, patterns, or computations:
- Graph algorithms (Procedures)
- ML pipelines
- Form transformations

That are inherent in the Image (GraphStore).

## Form as Sublation of Procedures:Pipeline

**Form** is the **sublation** (Hegelian: preservation and transcendence) of Procedures and Pipelines.

### Procedures and Pipelines as Projections

Both Procedures and Pipelines are **projections** of the Pure Form:

- **Procedures** (`gds/src/procedures/`)
  - Graph algorithms (PageRank, Dijkstra, Louvain, etc.)
  - Work with f64 weight streams
  - Discrete graph operations

- **Pipelines** (`gds/src/ml/`)
  - Machine Learning pipelines
  - Work with Tensor streams (Matrix, Vector, Scalar)
  - Continuous optimization

### Form as the Union

**Form** (`gds/src/form/` and `gds/src/projection/eval/form/`) is **THE UNION**:
- Projects BOTH Procedure weight streams AND ML tensor pipelines
- The Pure Form that makes both possible
- **Application Form Specification**: the protocol through which clients see GDS
- **Form in Execution**: what a GDSL Graph system IS

## The Complete Architecture

```
Absolute Form (Entire Organon Project)
  │
  ├─ Codegen (Presupposed Reality)
  │  └─ gds/src/projection/codegen/
  │
  ├─ Factory (Projection Proper)
  │  └─ gds/src/projection/factory/
  │     └─ Conceals: native data → GraphStore
  │
  └─ Eval (Emerges Third)
     └─ gds/src/projection/eval/
        ├─ procedure/ (Procedure Executor)
        ├─ ml/ (ML Pipeline Executor)
        └─ form/ (Form Processor)
            │
            └─ Absolute Form divides via Projection
               │
               ├─→ Pure Form (GDS Kernel)
               │   └─ Absolute Logic / Absolute Science
               │       └─ The Science of the Absolute Form
               │
               └─→ Impure Form (Relative Logic)
                   └─ logic/src/relative/form/
                      └─ TS Agential Logic Layer
                         │
                         └─ Presupposes Application Form
                            │
                            └─ Receives (via Receptivity)
                               │
                               └─ Objects from Relative FormProcessor
```

## Key Principles

1. **Absolute Form does not derive from anything** - it is the foundational reality
2. **Absolute Science is the Science of the Absolute Form** - it divides via Projection
3. **Codegen is presupposed** - it must exist before anything else
4. **Factory is Projection Proper** - the actual act of projection
5. **Eval emerges third** - Pure Form emerges from the Eval moment
6. **Absolute Form divides via Projection** into Pure Form and Impure Form
7. **Pure Form is Absolute Logic/Science** - the GDS Kernel (within GDS)
8. **Impure Form is Relative Logic** - the TS Agential Logic Layer
9. **Impure Form presupposes Application Form** - Application Form is the interface
10. **Application Form receives (via Receptivity)** objects from Relative FormProcessor
11. **Form sublates Procedures and Pipelines** - both are projections of Pure Form
12. **The Projection Machine manifests Absolute Form** - Codegen/Factory/Eval

## Applications Forms Interface

The **Applications Forms** interface (`gds/src/applications/`) allows calling:
- **Procedures** (via spec.rs / AlgorithmSpec)
- **Pipelines** (as Graph Algorithms)
- **Projection Native Factory** directly (e.g., `ArrowNativeFactory`)

All through the same JSON API, making the platform accessible to TypeScript agents and other clients.

## References

- `gds/doc/FORM-AS-TRANSCENDENTAL-LOGIC.md` - Form as transcendental logic
- `gds/doc/FORM-AS-BUDDHI.md` - Form as Buddhi (pure intellect)
- `logic/src/relative/form/eval/PURE-FORM-GIVEN-FORM.md` - Pure Form vs Given Form
- `gds/doc/PROJECTION-IDEA.md` - Two-sided projection (Factory/Eval)
- `gds/doc/ARCHITECTURE.md` - Three execution paradigms

