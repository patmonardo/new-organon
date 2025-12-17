# Projection Eval Architecture: The Three ISA

## Overview

The **Projection Eval system** (`gds/src/projection/eval/`) is where GDS Form, ML, and Procedures actually **execute**. This is not speculative architecture - this is the **running system**.

In the repo’s two-stage meaning of “Projection”, Eval is the **Revealing power** (Image → derived result). The corresponding **Concealing power** is Projection/Factory (Source → Image). Canonical note:

- [gds/doc/PROJECTION-CONCEAL-REVEAL.md](PROJECTION-CONCEAL-REVEAL.md)

## The Three ISA (Instruction Set Architectures)

```
gds/src/projection/eval/
├── procedure/   ← Computation ISA (AlgorithmSpec)
├── ml/          ← Pipeline ISA (Pipeline trait)
└── form/        ← Form ISA (FormProcessor + FormOperator)
```

Each ISA is a complete **evaluation system** with its own:
- **Executor**: The runtime engine (ProcedureExecutor, PipelineExecutor, FormProcessor)
- **Contract**: The specification trait (AlgorithmSpec, Pipeline, FormOperator)
- **Program**: The submitted spec/program (AlgorithmSpec impls, Pipeline plans, FormShape)

## Update (2025-12-17): Form ISA = Certainty Projection

The Form ISA is being rewritten to match the insight:

- Procedure corresponds to **Assertion** (universal / immediate)
- ML corresponds to **Problematic** (particular / mediate)
- Form corresponds to **Apodictic** (singular / result-producing)

In code, this is currently expressed as:

- A minimal `FormProcessor` that loads a base graph from `ExecutionContext`, selects an operator from a `FormShape`, and returns a derived `GraphStore`.
- A modality/concept mapping in `triadic_cycle.rs` (conceptual kernel; implementation continues).

Primary entry points:

- `gds/src/projection/eval/form/executor.rs` (`FormProcessor`, `FormCatalog`)
- `gds/src/projection/eval/form/form_spec.rs` (`FormRequest`, `FormOperator`, `FormArtifacts`)
- `gds/src/projection/eval/form/triadic_cycle.rs` (modal/concept moments)

## The Form ISA: Projection in Action

From [projection/eval/form/mod.rs](../src/projection/eval/form/mod.rs):

```rust
//! Form Evaluator - Fixed Singularity
//!
//! This module implements the **Form Evaluator** as a **fixed singularity** that executes
//! the **Form infrastructure**. It's the **third ISA** consisting of the **Triads of Hegel**.
//!
//! ## The Three ISA
//!
//! ```
//! eval/procedure (Computation ISA)  ← AlgorithmSpec implementations
//! eval/ml (ML ISA)                 ← Pipeline implementations  
//! eval/form (Form ISA)             ← FormProcessor + FormOperator
//! ```
```

### Form as “Returned Graph” (Apodictic)

The current Form ISA surface is intentionally minimal and grounded:

```rust
use crate::projection::eval::form::{FormCatalog, FormProcessor, FormRequest};
use crate::projection::eval::procedure::ExecutionContext;

let context = ExecutionContext::mock();
let catalog = FormCatalog::with_default_operators();
let processor = FormProcessor::new(context, catalog);

let request = FormRequest {
    graph_name: "graph".to_string(),
    program: /* FormShape */, // selects an operator via morph.patterns[0]
    artifacts: Default::default(),
    output_graph_name: None,
};

let result = processor.evaluate(&request)?;
let projected_graph = result.graph; // this is the ResultStore
```

The older “FormSpec + TriadicCycle” design remains in this document below as historical context,
but the live implementation has moved to an operator-based FormProcessor.

## Architecture Diagram

```
┌────────────────────────────────────────────────────────────────┐
│                    CLIENT / GDSL                               │
│              (Application Form / GDSL IR)                      │
└────────────────────────┬───────────────────────────────────────┘
                         │
                         │ Submits Form/IR
                         │
┌────────────────────────▼───────────────────────────────────────┐
│              gds/src/projection/eval/                          │
│                  (THE THREE ISA)                               │
│                                                                │
│  ┌──────────────────────────────────────────────────────────┐ │
│  │              eval/form/ (Form ISA)                       │ │
│  │                                                          │ │
│  │  FormExecutor {                                         │ │
│  │    fn execute<S: FormSpec>(                             │ │
│  │      form_spec: &S                                      │ │
│  │    ) -> FormResult<S::Output> {                         │ │
│  │      // Create triadic cycle                            │ │
│  │      let cycle = TriadicCycle::new(                     │ │
│  │        form_spec.thesis(),      // Procedure aspect     │ │
│  │        form_spec.antithesis(),  // ML aspect            │ │
│  │        form_spec.synthesis(),   // Union aspect         │ │
│  │      );                                                  │ │
│  │                                                          │ │
│  │      // Execute cycle                                   │ │
│  │      cycle.execute(context)                             │ │
│  │    }                                                     │ │
│  │  }                                                       │ │
│  │                                                          │ │
│  │  FormSpec trait {                                       │ │
│  │    fn thesis(&self) -> &Thesis;                         │ │
│  │    fn antithesis(&self) -> &Antithesis;                 │ │
│  │    fn synthesis(&self) -> &Synthesis;                   │ │
│  │  }                                                       │ │
│  └──────────────┬───────────────┬───────────────────────────┘ │
│                 │               │                             │
│                 │ Projects to:  │                             │
│                 │               │                             │
│     ┌───────────▼────┐     ┌───▼────────────┐                │
│     │                │     │                 │                │
│  ┌──▼──────────────┐ │  ┌──▼───────────────┐ │               │
│  │ eval/procedure/ │ │  │    eval/ml/      │ │               │
│  │  (Computation)  │ │  │  (Pipeline)      │ │               │
│  │                 │ │  │                  │ │               │
│  │ ProcedureExecutor│ │  │ PipelineExecutor│ │               │
│  │ AlgorithmSpec   │ │  │ Pipeline trait   │ │               │
│  │ f64 streams     │ │  │ Tensor streams   │ │               │
│  └────────┬────────┘ │  └────────┬─────────┘ │               │
│           │          │           │           │               │
│           │          │           │           │               │
└───────────┼──────────┴───────────┼───────────┴───────────────┘
            │                      │
            │                      │
            ▼                      ▼
    ┌───────────────┐      ┌──────────────┐
    │ procedures/   │      │    ml/       │
    │ (Algorithm    │      │  (Model      │
    │  impls)       │      │   impls)     │
    └───────────────┘      └──────────────┘
```

## The Procedure ISA

**Location**: [projection/eval/procedure/](../src/projection/eval/procedure/)

**Purpose**: Execute graph algorithms with various modes (stream, stats, mutate, write)

### Key Components

1. **ProcedureExecutor** ([executor.rs](../src/projection/eval/procedure/executor.rs))
   ```rust
   pub struct ProcedureExecutor {
       execution_context: ExecutionContext,
       execution_mode: ExecutionMode,
   }
   
   impl ProcedureExecutor {
       pub fn execute<S: AlgorithmSpec>(
           &mut self,
           algorithm_spec: &S,
           config: &AlgorithmConfig,
       ) -> Result<ComputationResult<S::Result>> {
           // 1. Validate configuration
           // 2. Load graph
           // 3. Execute algorithm
           // 4. Process result according to mode
       }
   }
   ```

2. **AlgorithmSpec** ([algorithm_spec.rs](../src/projection/eval/procedure/algorithm_spec.rs))
   ```rust
   pub trait AlgorithmSpec: Send + Sync {
       type Result: Send + Sync;
       
       fn name(&self) -> &str;
       fn compute(&self, graph: &Graph, config: &AlgorithmConfig) 
           -> Result<Self::Result>;
   }
   ```

3. **ExecutionMode** ([execution_mode.rs](../src/projection/eval/procedure/execution_mode.rs))
   ```rust
   pub enum ExecutionMode {
       Stream,   // Return result stream
       Stats,    // Return statistics only
       Mutate,   // Mutate graph properties
       Write,    // Write to external store
   }
   ```

### Example: PageRank

```rust
// procedures/pagerank/pagerank_spec.rs
pub struct PageRankSpec {
    max_iterations: usize,
    damping_factor: f64,
}

impl AlgorithmSpec for PageRankSpec {
    type Result = Vec<f64>;
    
    fn name(&self) -> &str { "pagerank" }
    
    fn compute(&self, graph: &Graph, config: &AlgorithmConfig) 
        -> Result<Vec<f64>> 
    {
        // PageRank implementation
        // Returns f64 weight stream
    }
}

// Usage through eval/procedure
let mut executor = ProcedureExecutor::new(context, ExecutionMode::Stream);
let result = executor.execute(&PageRankSpec { ... }, config)?;
```

## The ML ISA

**Location**: [projection/eval/ml/](../src/projection/eval/ml/)

**Purpose**: Execute ML pipelines for node/link prediction and classification

### Key Components

1. **PipelineExecutor** ([pipeline_executor.rs](../src/projection/eval/ml/pipeline_executor.rs))
   ```rust
   pub struct PipelineExecutor {
       graph_procedure_registry: GraphProcedureRegistry,
   }
   
   impl PipelineExecutor {
       pub fn execute(
           &self,
           pipeline: &PipelineDescriptor,
           graph: Arc<Graph>,
       ) -> Result<PipelineResult> {
           // 1. Execute node property steps (features)
           // 2. Assemble features
           // 3. Train/predict with model
       }
   }
   ```

2. **Pipeline** ([pipeline/pipeline_trait.rs](../src/projection/eval/ml/pipeline/pipeline_trait.rs))
   ```rust
   pub trait Pipeline {
       type FeatureStep: FeatureStep;
       
       fn node_property_steps(&self) -> &[ExecutableNodePropertyStep];
       fn feature_steps(&self) -> &[Self::FeatureStep];
   }
   ```

3. **LinkPredictionTrainingPipeline** ([pipeline/link_pipeline/](../src/projection/eval/ml/pipeline/link_pipeline/))
   ```rust
   pub struct LinkPredictionTrainingPipeline {
       node_property_steps: Vec<ExecutableNodePropertyStep>,
       link_feature_steps: Vec<LinkFeatureStep>,
       split_config: LinkPredictionSplitConfig,
   }
   
   impl Pipeline for LinkPredictionTrainingPipeline {
       type FeatureStep = LinkFeatureStep;
       // ...
   }
   ```

### Example: Link Prediction

```rust
// Create pipeline
let pipeline = LinkPredictionTrainingPipeline::new()
    .add_node_property_step(FastRPStep::new())
    .add_link_feature_step(HadamardFeatureStep::new())
    .add_link_feature_step(CosineFeatureStep::new())
    .build();

// Execute through eval/ml
let executor = PipelineExecutor::new(registry);
let result = executor.execute(&pipeline, graph)?;
// Returns tensor streams for model training
```

## The Form ISA: The Union

**Location**: [projection/eval/form/](../src/projection/eval/form/)

**Purpose**: Unite Procedure and ML execution through transcendental projection

### Key Components

1. **FormExecutor** ([executor.rs](../src/projection/eval/form/executor.rs))
   ```rust
   pub struct FormExecutor<F: FormStore> {
       form_store: F,
       execution_context: ExecutionContext,
       config: ExecutorConfig,
   }
   
   impl<F: FormStore> FormExecutor<F> {
       pub fn execute<S: FormSpec>(
           &self,
           form_spec: &S,
           config: &FormConfig,
       ) -> Result<FormResult<S::Output>, FormError> {
           // Create triadic cycle
           let cycle = TriadicCycle::new(
               form_spec.thesis(),
               form_spec.antithesis(),
               form_spec.synthesis(),
               config.cycle_config.clone(),
           );
           
           // Execute cycle (projects to both ISA!)
           cycle.execute(&self.execution_context)
       }
   }
   ```

2. **FormSpec** ([form_spec.rs](../src/projection/eval/form/form_spec.rs))
   ```rust
   pub trait FormSpec: Send + Sync {
       type Output: Send + Sync;
       
       fn name(&self) -> &str;
       fn thesis(&self) -> &Thesis;        // Procedure aspect
       fn antithesis(&self) -> &Antithesis; // ML aspect
       fn synthesis(&self) -> &Synthesis;   // Union aspect
   }
   ```

3. **TriadicCycle** ([triadic_cycle.rs](../src/projection/eval/form/triadic_cycle.rs))
   ```rust
   pub struct TriadicCycle {
       thesis: Thesis,          // Projects to eval/procedure
       antithesis: Antithesis,  // Projects to eval/ml
       synthesis: Synthesis,    // Combines both
       config: CycleConfig,
   }
   
   impl TriadicCycle {
       pub fn execute(&self, context: &ExecutionContext) 
           -> Result<TriadicCycleResult> 
       {
           // Execute all three moments
           // Return unified result
       }
   }
   ```

### Example: Hybrid Form

```rust
// Define a FormSpec that uses BOTH
pub struct GraphEmbeddingFormSpec {
    // Thesis: PageRank (Procedure)
    pagerank: PageRankSpec,
    
    // Antithesis: Node Classification (ML)
    classifier: NodeClassificationPipeline,
    
    // Synthesis: How to combine them
    synthesis_strategy: SynthesisStrategy,
}

impl FormSpec for GraphEmbeddingFormSpec {
    type Output = GraphEmbedding;
    
    fn name(&self) -> &str { "graph_embedding" }
    
    fn thesis(&self) -> &Thesis {
        &Thesis::Procedure(self.pagerank)
    }
    
    fn antithesis(&self) -> &Antithesis {
        &Antithesis::ML(self.classifier)
    }
    
    fn synthesis(&self) -> &Synthesis {
        &Synthesis::Strategy(self.synthesis_strategy)
    }
}

// Execute through eval/form
let executor = FormExecutor::new(form_store, context, config);
let result = executor.execute(&GraphEmbeddingFormSpec { ... }, config)?;
// Returns BOTH f64 streams AND tensor streams, unified!
```

## How Projection Works

### 1. Client Submits Request

Client can submit through:
- Direct Rust API (FormSpec instances)
- GDSL IR (compiled to FormSpec)
- JSON API (parsed to FormSpec)

### 2. FormExecutor Routes to ISA

```rust
// Form analyzes the request
let form_spec = parse_request(request)?;

// Create triadic cycle
let cycle = TriadicCycle::new(
    form_spec.thesis(),      // → eval/procedure
    form_spec.antithesis(),  // → eval/ml
    form_spec.synthesis(),   // → Both!
    config,
);

// Execute projects to appropriate ISA
cycle.execute(context)?
```

### 3. ISA Executes Content

```rust
// Thesis projects to eval/procedure
impl TriadicCycle {
    fn execute_thesis(&self) -> ThesisResult {
        match &self.thesis {
            Thesis::Procedure(spec) => {
                // Delegate to ProcedureExecutor
                let executor = ProcedureExecutor::new(...);
                executor.execute(spec, config)
            }
        }
    }
    
    fn execute_antithesis(&self) -> AntithesisResult {
        match &self.antithesis {
            Antithesis::ML(pipeline) => {
                // Delegate to PipelineExecutor
                let executor = PipelineExecutor::new(...);
                executor.execute(pipeline, graph)
            }
        }
    }
    
    fn synthesize(&self, thesis: ThesisResult, antithesis: AntithesisResult) 
        -> SynthesisResult 
    {
        // Combine results from BOTH ISA
        // This is the Union!
        match &self.synthesis {
            Synthesis::Strategy(strategy) => {
                strategy.combine(thesis, antithesis)
            }
        }
    }
}
```

### 4. Results Flow Back

```
FormExecutor
  └─> TriadicCycle
       ├─> ProcedureExecutor → f64 streams (Thesis)
       ├─> PipelineExecutor → Tensor streams (Antithesis)
       └─> Synthesis → UnionStream (both!)
```

## The Form Infrastructure Connection

The **Form Core** (`gds/src/form/core/`) provides the transcendental infrastructure:
- FormShape (Shape + Context + Morph)
- Container (manages FormShapes)
- Triadic relations (Membership, Consequence, Inherence)

The **Form Evaluator** (`gds/src/projection/eval/form/`) executes that infrastructure:
- FormExecutor (runtime engine)
- FormSpec (execution contract)
- TriadicCycle (projection mechanism)

```
form/core/              →  The Infrastructure (Transcendental Logic)
projection/eval/form/   →  The Execution (Projection into Actuality)
```

## Summary: The Complete Picture

GDS is built on **three parallel ISA**:

1. **Procedure ISA** - Discrete computation (f64 streams)
2. **ML ISA** - Continuous learning (Tensor streams)
3. **Form ISA** - Transcendental union (both!)

All three exist **right now** in `projection/eval/`:
- They're not speculative
- They're not "to be built"
- They **ARE** the running system

**Form is the Union** that makes both Procedures and ML possible - not by containing them, but by **projecting** them through the Triadic Cycle.

This is Pure Reason enabling Empirical Reason.
This is Buddhi individualizing Mahat.
This is Transcendental Logic making Ordinary Logic possible.

---

*"Projection is not a metaphor - it's the actual mechanism by which Form becomes execution in `projection/eval/`."*
