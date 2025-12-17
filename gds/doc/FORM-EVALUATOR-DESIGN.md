# Form Evaluator Design Specification

## Purpose
Document the discovery process for building the **Form Evaluator** - the third evaluator that unifies Procedure and ML execution through transcendental projection.

## Current State

### What We Know

1. **Procedure Evaluator exists** ([projection/eval/procedure/](../src/projection/eval/procedure/))
   - ProcedureExecutor with ExecutionMode (stream, stats, mutate, write)
   - AlgorithmSpec trait for algorithm implementations
   - Works with f64 weight streams
   - Example: PageRank, Dijkstra, Louvain

2. **ML Evaluator exists** ([projection/eval/ml/](../src/projection/eval/ml/))
   - PipelineExecutor with TrainingPipeline/PredictPipeline
   - Works with Tensor streams (Matrix/Vector/Scalar)
   - Models: DecisionTree, LogisticRegression, etc.
   - Link prediction and node classification pipelines

3. **Form Evaluator exists** ([projection/eval/form/](../src/projection/eval/form/))
    - `FormProcessor` that projects a **returned GraphStore** (ResultStore)
    - `FormOperator` contract (operator selected from a `FormShape`)
    - Consumes `FormArtifacts` (bridge from Procedure/ML outputs)
    - Modality/concept kernel lives in `triadic_cycle.rs`

### What We Don't Know

**What does the Form Evaluator DO?**

We need to discover this by building incrementally.

## Update (2025-12-17): The Seed Is Certainty

The guiding prompt is: **the origin of certainty is self-evident in use but obscure in its ground**.
We’re treating the Form ISA as the place where that ground starts to become explicit in code.

Runtime handoff (working mapping):

- **Procedure** → Assertion (universal / immediate)
- **ML** → Problematic (particular / mediate)
- **Form** → Apodictic (singular / result-producing)

Concretely, the minimal requirement for “apodictic” right now is:

- Given a base `GraphStore` plus artifacts, Form must be able to **return a projected GraphStore**.

Live code entry points:

- `gds/src/projection/eval/form/executor.rs`
- `gds/src/projection/eval/form/form_spec.rs`
- `gds/src/projection/eval/form/triadic_cycle.rs`

## Design Philosophy: Discovery by Building

We're not designing from scratch - we're **discovering** what the Form Evaluator must be by:
1. Creating concrete examples of Application Forms
2. Implementing projection logic
3. Observing what patterns emerge
4. Refining based on actual needs

## Proposed Architecture

### Level 1: Application Form Protocol

```rust
/// ApplicationForm - What clients submit to GDS
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationForm {
    /// Unique identifier for this application
    pub id: String,
    
    /// The graph operation being requested
    pub operation: GraphOperation,
    
    /// Configuration parameters
    pub config: ApplicationConfig,
    
    /// Execution preferences
    pub execution: ExecutionPreferences,
}

/// What kind of graph operation?
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GraphOperation {
    /// Run a single procedure
    Procedure(ProcedureSpec),
    
    /// Run an ML pipeline
    MachineLearning(MLSpec),
    
    /// Run a hybrid (Procedure + ML)
    Hybrid(HybridSpec),
    
    /// Run a GDSL program
    GDSL(GDSLProgram),
}

/// Configuration for the operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationConfig {
    /// Graph to operate on
    pub graph_name: String,
    
    /// Node/relationship filters
    pub filters: Option<GraphFilters>,
    
    /// Projection configuration
    pub projection: Option<ProjectionConfig>,
}

/// How should this execute?
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionPreferences {
    /// Execution mode (stream, stats, mutate, write)
    pub mode: ExecutionMode,
    
    /// Concurrency settings
    pub concurrency: Option<ConcurrencyConfig>,
    
    /// Result format preferences
    pub output: OutputFormat,
}
```

### Level 2: Form Evaluator Core

**Location**: [projection/eval/form/executor.rs](../src/projection/eval/form/executor.rs)

```rust
/// FormExecutor - The main executor for forms
///
/// This struct is the **fixed singularity** that executes **FormSpec** implementations
/// through the **triadic cycle**.
pub struct FormExecutor<F: FormStore> {
    /// The form store (like GraphStore for procedures)
    form_store: F,
    /// Execution context
    execution_context: ExecutionContext,
    /// Executor configuration
    config: ExecutorConfig,
}

impl<F: FormStore> FormExecutor<F> {
    /// Execute a FormSpec through the triadic cycle
    pub fn execute<S: FormSpec>(
        &self,
        form_spec: &S,
        config: &FormConfig,
    ) -> Result<FormResult<S::Output>, FormError> {
        // Step 1: Validate the form specification
        self.validate_form_spec(form_spec, config)?;
        
        // Step 2: Create the triadic cycle
        let triadic_cycle = TriadicCycle::new(
            form_spec.thesis().clone(),      // Procedure aspect
            form_spec.antithesis().clone(),  // ML aspect
            form_spec.synthesis().clone(),   // Union aspect
            config.cycle_config.clone(),
        );
        
        // Step 3: Execute the triadic cycle
        let cycle_result = triadic_cycle.execute(&self.execution_context)?;
        
        // Step 4: Return completed form result
        Ok(FormResult::new(
            output,
            execution_time,
            cycle_metadata,
        ))
    }
    
    /// Project to Procedure (Thesis)
    async fn project_procedure(
        &self,
        thesis: &Thesis,
    ) -> Result<ProcedureResult, FormError> {
        // Delegate to projection/eval/procedure
        // Uses AlgorithmSpec execution
    }
    
    /// Project to ML (Antithesis)
    async fn project_ml(
        &self,
        antithesis: &Antithesis,
    ) -> Result<MLResult, FormError> {
        // Delegate to projection/eval/ml
        // Uses Pipeline execution
    }
    
    /// Project to BOTH (Synthesis - the Union!)
    async fn project_synthesis(
        &self,
        synthesis: &Synthesis,
    ) -> Result<UnionResult, FormError> {
        // Execute both procedure and ML
        // Synthesize results through Form
    }
}
```

### Level 3: Integration with GDSL

```rust
/// Connect Form Evaluator to GDSL execution
impl FormEvaluator {
    /// Execute a GDSL program through Form
    async fn project_gdsl(
        &self,
        program: &GDSLProgram,
        cycle: &TriadicCycle,
    ) -> Result<ApplicationResult, EvaluationError> {
        // Parse GDSL IR
        let ir = gdsl::parse(program)?;
        
        // Analyze what operations are needed
        let operations = self.analyze_ir(&ir)?;
        
        // Project each operation
        let mut results = Vec::new();
        for op in operations {
            let result = match op {
                IROperation::Algorithm(algo) => {
                    let spec = ProcedureSpec::from_ir(algo);
                    self.project_procedure(&spec, cycle).await?
                }
                IROperation::MLPipeline(pipeline) => {
                    let spec = MLSpec::from_ir(pipeline);
                    self.project_ml(&spec, cycle).await?
                }
                IROperation::Hybrid(hybrid) => {
                    let spec = HybridSpec::from_ir(hybrid);
                    self.project_hybrid(&spec, cycle).await?
                }
            };
            results.push(result);
        }
        
        // GDSL as "Form in Execution"
        // The ShapeStream is the Form streaming itself
        Ok(ApplicationResult::GDSL(GDSLResult {
            ir,
            results,
            shape_stream: self.create_shape_stream(&results)?,
        }))
    }
    
    /// Create ShapeStream from results
    fn create_shape_stream(
        &self,
        results: &[ApplicationResult],
    ) -> Result<ShapeStream, EvaluationError> {
        // ShapeStream = Form "mid-execution"
        // Not data ABOUT results, but results AS stream
        let stream = ShapeStream::new();
        
        for result in results {
            match result {
                ApplicationResult::Procedure(r) => {
                    // f64 weight stream
                    stream.append_scalar_stream(r.values())?;
                }
                ApplicationResult::ML(r) => {
                    // Tensor stream
                    stream.append_tensor_stream(r.tensors())?;
                }
                ApplicationResult::Hybrid(r) => {
                    // Union stream
                    stream.append_union_stream(r)?;
                }
                _ => {}
            }
        }
        
        Ok(stream)
    }
}
```

## Implementation Phases

### Phase 1: Foundation (Week 1)
- [ ] Complete FormSpec trait in [projection/eval/form/form_spec.rs](../src/projection/eval/form/form_spec.rs)
- [ ] Implement FormExecutor core in [projection/eval/form/executor.rs](../src/projection/eval/form/executor.rs)
- [ ] Connect FormExecutor to ProcedureExecutor ([projection/eval/procedure/](../src/projection/eval/procedure/))
- [ ] Write tests with PageRank as example FormSpec

**Goal**: Prove we can execute existing Procedures through Form (Thesis projection)

### Phase 2: ML Integration (Week 2)
- [ ] Add ML projection to FormEvaluator
- [ ] Implement tensor stream handling
- [ ] Create examples with Decision Trees
- [ ] Test separate Procedure and ML paths

**Goal**: Prove Form can project to BOTH evaluators

### Phase 3: The Union (Week 3)
- [ ] Implement Hybrid projection (Procedure + ML)
- [ ] Build synthesis logic for combined results
- [ ] Create practical hybrid examples:
  - PageRank → Node Classification
  - Community Detection → Link Prediction
  - Centrality → Feature Engineering

**Goal**: Prove Form is the UNION that combines both

### Phase 4: GDSL Integration (Week 4)
- [ ] Connect Form Evaluator to GDSL IR
- [ ] Implement ShapeStream creation
- [ ] Test with existing GDSL programs in `fixtures/gdsl/`
- [ ] Document Form as "Form in Execution"

**Goal**: Prove GDSL Graph systems ARE Forms

### Phase 5: Application Protocol (Week 5)
- [ ] Design JSON schema for Application Forms
- [ ] Build HTTP/gRPC API endpoints
- [ ] Implement daemon-style form submission
- [ ] Create client examples (Python, TypeScript)

**Goal**: Prove Form works as client-facing protocol

## Test Strategy

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_procedure_projection() {
        let evaluator = FormEvaluator::new();
        
        let form = ApplicationForm {
            id: "test-1".into(),
            operation: GraphOperation::Procedure(
                ProcedureSpec::pagerank()
                    .iterations(20)
                    .damping_factor(0.85)
            ),
            config: ApplicationConfig {
                graph_name: "test-graph".into(),
                filters: None,
                projection: None,
            },
            execution: ExecutionPreferences {
                mode: ExecutionMode::Stream,
                concurrency: None,
                output: OutputFormat::JSON,
            },
        };
        
        let result = evaluator.evaluate(form).await.unwrap();
        assert!(matches!(result, ApplicationResult::Procedure(_)));
    }
    
    #[tokio::test]
    async fn test_hybrid_projection() {
        let evaluator = FormEvaluator::new();
        
        let form = ApplicationForm {
            operation: GraphOperation::Hybrid(HybridSpec {
                procedure: ProcedureSpec::pagerank(),
                ml: MLSpec::node_classification(),
            }),
            // ... config
        };
        
        let result = evaluator.evaluate(form).await.unwrap();
        
        // Verify BOTH executed
        if let ApplicationResult::Hybrid(hybrid) = result {
            assert!(hybrid.has_procedure_result());
            assert!(hybrid.has_ml_result());
            assert!(hybrid.is_synthesized());
        }
    }
}
```

### Integration Tests
```rust
#[tokio::test]
async fn test_gdsl_as_form_in_execution() {
    // Load GDSL program from fixtures
    let gdsl = fs::read_to_string(
        "fixtures/gdsl/run.pagerank.stream.json"
    ).unwrap();
    
    let program = serde_json::from_str(&gdsl).unwrap();
    
    let form = ApplicationForm {
        operation: GraphOperation::GDSL(program),
        // ... config
    };
    
    let result = evaluator.evaluate(form).await.unwrap();
    
    // Verify ShapeStream created
    if let ApplicationResult::GDSL(gdsl_result) = result {
        let stream = gdsl_result.shape_stream;
        
        // ShapeStream IS the Form mid-execution
        assert!(stream.is_streaming());
        assert_eq!(stream.operation_count(), 1);
    }
}
```

## Open Questions

### Q1: Application Form Schema
**Question**: Should Application Forms be JSON, Rust structs, or GDSL IR directly?

**Options**:
- A) JSON schema (most compatible, client-friendly)
- B) Rust structs (type-safe, internal only)
- C) GDSL IR (consistent with existing IR)
- D) All three (JSON for external, Rust for internal, IR as target)

**Recommendation**: Start with D - JSON for clients, convert to Rust, compile to IR

### Q2: Execution Mode
**Question**: Should Form Evaluator be sync or async?

**Analysis**:
- Procedures: Currently sync (but could be async)
- ML: Mix of sync (training) and async (inference)
- GDSL: Should be async (streaming nature)

**Recommendation**: Async by default (`async fn evaluate()`)

### Q3: Result Streaming
**Question**: Should results stream or return complete?

**Options**:
- A) Return complete results (simple)
- B) Stream results as available (efficient)
- C) Hybrid: stream by default, complete on request

**Recommendation**: C - ShapeStream for streaming, collect() for complete

### Q4: Error Handling
**Question**: How do projection errors propagate?

**Recommendation**:
```rust
#[derive(Debug, thiserror::Error)]
pub enum EvaluationError {
    #[error("Procedure projection failed: {0}")]
    ProcedureError(#[from] ProcedureError),
    
    #[error("ML projection failed: {0}")]
    MLError(#[from] MLError),
    
    #[error("Invalid Application Form: {0}")]
    InvalidForm(String),
    
    #[error("Projection not supported: {0}")]
    UnsupportedProjection(String),
}
```

## Success Criteria

We'll know the Form Evaluator is complete when:

1. ✅ Can execute any existing Procedure through Form
2. ✅ Can execute any existing ML pipeline through Form
3. ✅ Can execute Hybrid (Procedure + ML) operations
4. ✅ GDSL programs compile to Application Forms
5. ✅ ShapeStream represents "Form in Execution"
6. ✅ Client can submit Application Forms via API
7. ✅ All tests pass (unit + integration)
8. ✅ Documentation explains the philosophical basis

## Next Actions

1. **Uncomment Form module** in [lib.rs](../src/lib.rs#L17)
2. **Create** `src/form/eval/` directory
3. **Implement** `application_form.rs` with basic protocol
4. **Implement** `form_evaluator.rs` with Phase 1 functionality
5. **Write** first test: PageRank through Form
6. **Run** test, observe what breaks, fix
7. **Iterate** based on what we discover

## Philosophical Note

We're not inventing the Form Evaluator - we're **discovering** it by building.

The Form Evaluator emerges as the **necessary mediator** between:
- Pure Form (FormShape, Container, Triadic Cycle)
- Concrete Execution (Procedures, ML, GDSL)

It's transcendental because it doesn't add new functionality - it reveals the **condition of possibility** for the functionality that already exists.

Form is to Procedures/ML as **Categories are to Intuitions** in Kant: the structure that makes experience (computation) possible.

---

*"The Form Evaluator is not built, it is discovered by building what it must coordinate."*
