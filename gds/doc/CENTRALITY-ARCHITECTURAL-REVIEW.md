# Centrality Algorithms - Architectural Review

## Overview

This document reviews the architectural compliance of centrality algorithms with the universal `compute_{algo}` controller pattern. The pattern requires:

- **Storage Runtime**: Acts as controller, orchestrates algorithm loops, handles graph access
- **Computation Runtime**: Pure state management, no graph access, only ephemeral state operations
- **Procedure Layer**: Top-level compute entrypoint; creates both runtimes and calls `storage.compute_{algo}(&mut computation, ...)`

## Current Status Summary

**Total Centrality Algorithms**: 9
- ✅ **Fully Architecturally Sound** (9 algorithms): articulation_points, betweenness, bridges, celf, closeness, degree_centrality, harmonic, hits, pagerank
- ⚠️ **Partially Compliant** (0 algorithms)
- ❌ **Non-Compliant** (0 algorithms)

**Reviewed**: 9/9 algorithms (100% complete)
**Remaining Issues**: 0 algorithms need restructuring to follow the Procedure-First Controller Pattern

## articulation_points - ✅ **Architecturally Sound**

### Layered Architecture

**1. Procedure Layer** (`procedures/centrality/articulation_points.rs`):
```rust
// Creates both runtimes (factory pattern)
let storage = ArticulationPointsStorageRuntime::new(&*self.graph_store)?;
let mut computation = ArticulationPointsComputationRuntime::new(node_count);

// Calls storage.compute_articulation_points() - Applications talk only to procedures
let result = storage.compute_articulation_points(&mut computation, Some(graph_view.as_ref()), &mut progress_tracker)?;
```

**2. Storage Runtime** (`algo/articulation_points/storage.rs`) - **Controller Pattern**:
```rust
pub fn compute_articulation_points(
    &self,
    computation: &mut ArticulationPointsComputationRuntime,
    graph: Option<&dyn Graph>,
    progress_tracker: &mut dyn ProgressTracker,
) -> Result<ArticulationPointsComputationResult, AlgorithmError>
```
- **Orchestrates algorithm loop**: Iterative DFS traversal with stack-based events
- **Graph access**: `self.neighbors()` method for undirected neighbor retrieval
- **State delegation**: Calls `computation.is_visited()`, `computation.set_visited()`, `computation.get_tin()`, etc.

**3. Computation Runtime** (`algo/articulation_points/computation.rs`) - **Pure State Management**:
```rust
pub fn initialize(&mut self, node_count: usize)
pub fn is_visited(&self, node: usize) -> bool
pub fn set_visited(&mut self, node: usize)
pub fn get_tin(&self, node: usize) -> i64
pub fn set_tin(&mut self, node: usize, value: i64)
// ... additional state management methods
```
- **No graph access** - only manages ephemeral state (visited nodes, discovery times, low values, articulation points)
- **Stateless operations** - pure functions on internal data structures

### Pattern Compliance
✅ **Storage acts as controller** - orchestrates DFS traversal, calls computation methods
✅ **Computation is pure state** - no graph access, only state management
✅ **Procedure creates both** - factory pattern with proper initialization
✅ **Applications use procedures** - procedures layer prevents direct ::algo:: calls

## betweenness - ✅ **Architecturally Sound**

### Layered Architecture

**1. Procedure Layer** (`procedures/centrality/betweenness.rs`):
```rust
// Creates both runtimes (factory pattern)
let storage = BetweennessCentralityStorageRuntime::new(&*self.graph_store, self.orientation(), self.relationship_weight_property.as_deref())?;
let mut computation = BetweennessCentralityComputationRuntime::new(node_count);

// Select sources using storage
let sources = storage.select_sources(&self.sampling_strategy, Some(requested), self.random_seed);

// Calls storage.compute_betweenness() - Applications talk only to procedures
let result = storage.compute_betweenness(&mut computation, &sources, divisor, self.concurrency, &termination, on_source_done)?;
```

**2. Storage Runtime** (`algo/betweenness/storage.rs`) - **Controller Pattern**:
```rust
pub fn compute_betweenness(
    &self,
    computation: &mut BetweennessCentralityComputationRuntime,
    sources: &[usize],
    divisor: f64,
    concurrency: usize,
    termination: &TerminationFlag,
    on_source_done: Arc<dyn Fn() + Send + Sync>,
) -> Result<BetweennessCentralityComputationResult, TerminatedException>
```
- **Orchestrates algorithm loop**: Parallel Brandes algorithm across selected sources
- **Graph access**: `self.neighbors()` and `self.neighbors_weighted()` for traversal
- **Source selection**: `self.select_sources()` for sampling strategies (all, random_degree)
- **State delegation**: Calls `computation.compute_parallel_unweighted/weighted()` for parallel execution

**3. Computation Runtime** (`algo/betweenness/computation.rs`) - **Pure State Management**:
```rust
pub struct BetweennessCentralityComputationRuntime {
    centrality: HugeAtomicDoubleArray,
    node_count: usize,
}

pub fn finalize_result(&self) -> BetweennessCentralityComputationResult
pub fn compute_parallel_unweighted(&self, sources: &[usize], divisor: f64, concurrency: usize, termination: &TerminationFlag, on_source_done: Arc<dyn Fn() + Send + Sync>, get_neighbors: &(impl Fn(usize) -> Vec<usize> + Send + Sync)) -> Result<(), TerminatedException>
// ... similar for weighted
```
- **No graph access** - only manages global centrality scores (HugeAtomicDoubleArray)
- **Parallel coordination** - handles worker threads and accumulates results atomically
- **Stateless operations** - pure functions on internal state

### Pattern Compliance
✅ **Storage acts as controller** - orchestrates Brandes algorithm, handles source selection and parallel execution
✅ **Computation is pure state** - no graph access, only manages centrality accumulation
✅ **Procedure creates both** - factory pattern with proper initialization
✅ **Applications use procedures** - procedures layer prevents direct ::algo:: calls

### Key Architectural Notes
- **Parallel Complexity**: Betweenness uses parallel execution with worker contexts, but storage still controls the high-level orchestration
- **Source Sampling**: Storage handles sampling strategies (full vs degree-weighted), providing clean separation
- **Weighted/Unweighted Dispatch**: Storage determines traversal type and delegates to appropriate computation method

## bridges - ✅ **Architecturally Sound**

### Layered Architecture

**1. Procedure Layer** (`procedures/centrality/bridges.rs`):
```rust
// Creates both runtimes (factory pattern)
let storage = BridgesStorageRuntime::new(&*self.graph_store)?;
let mut computation = BridgesComputationRuntime::new(storage.node_count());

// Calls storage.compute_bridges() - Applications talk only to procedures
let result = storage.compute_bridges(&mut computation, &termination, on_node_scanned)?;
```

**2. Storage Runtime** (`algo/bridges/storage.rs`) - **Controller Pattern**:
```rust
pub fn compute_bridges(
    &self,
    computation: &mut BridgesComputationRuntime,
    termination: &TerminationFlag,
    on_node_scanned: Arc<dyn Fn() + Send + Sync>,
) -> Result<BridgesComputationResult, TerminatedException>
```
- **Orchestrates algorithm loop**: Iterative DFS traversal across all nodes
- **Graph access**: `self.neighbors()` for undirected neighbor retrieval
- **State delegation**: Calls `computation.is_visited()`, `computation.dfs_component()` for DFS state management

**3. Computation Runtime** (`algo/bridges/computation.rs`) - **Pure State Management**:
```rust
pub struct BridgesComputationRuntime {
    visited: BitSet,
    tin: HugeLongArray,
    low: HugeLongArray,
    timer: i64,
    stack: Vec<StackEvent>,
}

pub fn reset(&mut self, node_count: usize)
pub fn is_visited(&self, node: usize) -> bool
pub fn dfs_component(&mut self, start_node: usize, get_neighbors: &impl Fn(usize) -> Vec<usize>, bridges: &mut Vec<Bridge>)
```
- **No graph access** - only manages DFS state (visited nodes, discovery times, low values, stack)
- **Pure state operations** - all methods operate on internal data structures
- **Reusable stack** - DFS stack can be sized by relationship count for Java parity

### Pattern Compliance
✅ **Storage acts as controller** - orchestrates DFS traversal, handles termination checks
✅ **Computation is pure state** - no graph access, only manages DFS state and bridge collection
✅ **Procedure creates both** - factory pattern with proper initialization
✅ **Applications use procedures** - procedures layer prevents direct ::algo:: calls

### Key Architectural Notes
- **Single-threaded Algorithm**: Bridges uses iterative DFS with explicit stack, no concurrency needed
- **Stateful DFS**: Computation runtime maintains DFS state across the entire algorithm execution
- **Bridge Collection**: Storage orchestrates traversal, computation collects bridges in provided vector

## celf - ✅ **Architecturally Sound**

### Layered Architecture

**1. Procedure Layer** (`procedures/centrality/celf.rs`):
```rust
// Creates both runtimes (factory pattern)
let storage = CELFStorageRuntime::new(&*self.graph_store)?;
let runtime = CELFComputationRuntime::new(self.config.clone(), node_count);

// Calls storage.compute_celf() with termination flag
let seed_set = storage.compute_celf(&runtime, &termination)?;
```

**2. Storage Runtime** (`algo/celf/storage.rs`) - **Controller Pattern**:
```rust
pub fn compute_celf(
    &self,
    computation: &CELFComputationRuntime,
    termination: &TerminationFlag,
) -> Result<HashMap<u64, f64>, TerminatedException>
```
- **Orchestrates algorithm loop**: owns graph access and delegates CELF greedy steps to computation
- **Graph access**: `neighbors()` iterates relationships with proper orientation
- **Termination-aware**: passes shared `TerminationFlag` into computation parallel loops

**3. Computation Runtime** (`algo/celf/computation.rs`) - **Pure State Management**:
```rust
pub fn compute(
    &self,
    get_neighbors: impl Fn(usize) -> Vec<usize> + Send + Sync,
    termination: &TerminationFlag,
) -> HashMap<u64, f64>
```
- **No graph access** - receives neighbor callback from storage
- **State-only operations** - manages priority queue, Monte Carlo simulation state, and spread tracking
- **Termination-aware** - threads `TerminationFlag` through greedy init and lazy-forward phases

### Pattern Compliance
✅ **Storage acts as controller** - handles graph access, calls computation with neighbor provider and termination
✅ **Computation is pure state** - no direct graph access; encapsulates CELF state and sampling
✅ **Procedure creates both** - factory pattern with termination flag
✅ **Applications use procedures** - controller boundary enforced

## closeness - ✅ **Architecturally Sound**

### Layered Architecture

**1. Procedure Layer** (`procedures/centrality/closeness.rs`):
```rust
// Creates both runtimes (factory pattern)
let storage = ClosenessCentralityStorageRuntime::new(self.graph_store.as_ref(), self.orientation())?;
let computation = ClosenessCentralityComputationRuntime::new();

// Calls storage.compute_parallel() with neighbor callback and termination flag
let centralities = storage.compute_parallel(
    &computation,
    self.wasserman_faust,
    self.concurrency,
    &termination,
    on_sources_done,
    on_closeness_done,
)?;
```

**2. Storage Runtime** (`algo/closeness/storage.rs`) - **Controller Pattern**:
```rust
pub fn compute_parallel(
    &self,
    computation: &ClosenessCentralityComputationRuntime,
    wasserman_faust: bool,
    concurrency: usize,
    termination: &TerminationFlag,
    on_farness_sources_done: Arc<dyn Fn(usize) + Send + Sync>,
    on_closeness_nodes_done: Arc<dyn Fn(usize) + Send + Sync>,
) -> Result<Vec<f64>, TerminatedException>
```
- **Orchestrates pipeline**: builds oriented graph view, supplies `neighbors()`, drives farness then closeness phases
- **Graph access**: `neighbors()` streams relationships via graph view; computation stays callback-only
- **Termination-aware**: threads shared `TerminationFlag` through both phases

**3. Computation Runtime** (`algo/closeness/computation.rs`) - **Pure State Management**:
```rust
pub struct ClosenessCentralityComputationRuntime;

pub fn compute_farness_parallel(&self, node_count, concurrency, termination, on_sources_done, get_neighbors) -> Result<(Vec<u64>, Vec<u64>), TerminatedException>
pub fn compute_closeness_parallel(&self, node_count, wasserman_faust, concurrency, termination, farness, component, on_nodes_done) -> Result<Vec<f64>, TerminatedException>
```
- **No graph access** - consumes neighbor callback provided by storage
- **State-only** - owns farness/component accumulation and score computation using HugeAtomic arrays
- **Termination-aware** - executor loops check `TerminationFlag`

### Pattern Compliance
✅ **Storage acts as controller** - graph access + pipeline orchestration, progress hooks
✅ **Computation is pure state** - receives neighbor callback; no graph I/O
✅ **Procedure creates both** - factory pattern; passes termination and progress callbacks
✅ **Applications use procedures** - controller boundary enforced

## degree_centrality - ✅ **Architecturally Sound**

### Layered Architecture

**1. Procedure Layer** (`procedures/centrality/degree_centrality.rs`):
```rust
let storage = DegreeCentralityStorageRuntime::with_settings(&*graph_store, orientation, weighted)?;
let computation = DegreeCentralityComputationRuntime::new();

// Applications talk only to procedures
let scores = storage.compute_parallel(&computation, concurrency, &termination, on_nodes_done)?;
```

**2. Storage Runtime** (`algo/degree_centrality/storage.rs`) - **Controller Pattern**:
```rust
pub fn compute_parallel(
    &self,
    computation: &DegreeCentralityComputationRuntime,
    concurrency: usize,
    termination: &TerminationFlag,
    on_nodes_done: Arc<dyn Fn(usize) + Send + Sync>,
) -> Result<Vec<f64>, TerminatedException>
```
- **Orchestrates algorithm loop**: partitions nodes, drives virtual-thread executor
- **Graph access**: delegates degree lookups via `degree_unweighted` / `degree_weighted`
- **State delegation**: calls `computation.compute_range(...)` for batch writes

**3. Computation Runtime** (`algo/degree_centrality/computation.rs`) - **Pure State Management**:
```rust
pub struct DegreeCentralityComputationRuntime;

pub fn compute_range(&self, start, end, termination, out, degree_fn)
pub fn normalize_scores(&self, scores: &mut [f64])
```
- **No graph access** - only writes to provided output buffer
- **State-only** - normalization and per-batch accumulation
- **Termination-aware** - early exit when `TerminationFlag` stops

### Pattern Compliance
✅ **Storage acts as controller** - batching, graph access, executor, progress callbacks
✅ **Computation is pure state** - no graph I/O; writes via supplied callbacks
✅ **Procedure creates both** - factory pattern; passes termination/progress
✅ **Applications use procedures** - controller boundary enforced

## harmonic - ✅ **Architecturally Sound**

### Layered Architecture

**1. Procedure Layer** (`procedures/centrality/harmonic.rs`):
```rust
let storage = HarmonicStorageRuntime::with_orientation(&*graph_store, orientation)?;
let computation = HarmonicComputationRuntime::new(storage.node_count());

let scores = storage.compute_parallel(&computation, concurrency, &termination, on_sources_done)?;
```

**2. Storage Runtime** (`algo/harmonic/storage.rs`) - **Controller Pattern**:
```rust
pub fn compute_parallel(
    &self,
    computation: &HarmonicComputationRuntime,
    concurrency: usize,
    termination: &TerminationFlag,
    on_sources_done: Arc<dyn Fn(usize) + Send + Sync>,
) -> Result<Vec<f64>, TerminatedException>
```
- **Orchestrates algorithm loop**: partitions sources into `OMEGA`-sized MSBFS batches, drives virtual-thread executor
- **Graph access**: oriented neighbors resolved via storage `neighbors()`
- **State delegation**: invokes `computation.run_batch(...)` per batch and returns `computation.finalize()`

**3. Computation Runtime** (`algo/harmonic/computation.rs`) - **Pure State Management**:
```rust
pub struct HarmonicComputationRuntime { inverse_farness: HugeAtomicDoubleArray, node_count: usize }

pub fn run_batch(&self, msbfs, source_offset, source_len, termination, get_neighbors)
pub fn finalize(&self) -> Vec<f64>
```
- **No graph access** - consumes neighbor callback supplied by storage
- **State-only** - accumulates inverse farness and normalizes by `(node_count - 1)`
- **Termination-aware** - exits batches early if `TerminationFlag` stops

### Pattern Compliance
✅ **Storage acts as controller** - batching, graph I/O, executor, progress callbacks
✅ **Computation is pure state** - accumulation + normalization only
✅ **Procedure creates both** - factory pattern with termination/progress wiring
✅ **Applications use procedures** - controller boundary enforced

## hits - ✅ **Architecturally Sound**

### Layered Architecture

**1. Procedure Layer** (`procedures/centrality/hits.rs`):
```rust
let storage = HitsStorageRuntime::with_default_projection(self.graph_store.as_ref())?;
let computation = HitsComputationRuntime::new(self.tolerance);

let result = storage.run(
    &computation,
    self.max_iterations,
    self.concurrency,
    &mut progress_tracker,
);
```

**2. Storage Runtime** (`algo/hits/storage.rs`) - **Controller Pattern**:
```rust
pub fn run(
    &self,
    computation: &HitsComputationRuntime,
    max_iterations: usize,
    concurrency: usize,
    progress_tracker: &mut dyn ProgressTracker,
) -> HitsRunResult
```
- **Orchestrates Pregel**: builds runtime config (supersteps = 1 + 4 * iterations), picks messenger, and drives Pregel execution
- **Graph access**: owns projected graph handle; computation stays graph-free
- **State delegation**: wires computation schema/init/compute/master callbacks, then calls `computation.finalize()` to collect scores

**3. Computation Runtime** (`algo/hits/computation.rs`) - **Pure State Management**:
```rust
pub struct HitsComputationRuntime { tolerance: f64 }

pub fn schema(&self) -> PregelSchema
pub fn init_fn(&self) -> Arc<...InitContext...>
pub fn compute_fn(&self) -> Arc<...ComputeContext...>
pub fn master_compute_fn(&self) -> impl Fn(&mut MasterComputeContext<...>) -> bool + Send + Sync + 'static
pub fn finalize(&self, result: &PregelResult, node_count: usize) -> HitsRunResult
```
- **No graph access**: only manipulates Pregel node values provided by storage
- **State-only**: maintains hub/authority temp/prev values; master computes norms and convergence via captured tolerance
- **Iteration mapping**: translates Pregel supersteps back to HITS iterations in `finalize`

## pagerank - ✅ **Architecturally Sound**

### Layered Architecture

**1. Procedure Layer** (`procedures/centrality/pagerank.rs`):
```rust
let storage = PageRankStorageRuntime::with_orientation(self.graph_store.as_ref(), self.orientation())?;
let computation = PageRankComputationRuntime::new(
    pr_config.max_iterations,
    pr_config.damping_factor,
    pr_config.tolerance,
    source_set,
);

let run = storage.run(&computation, self.concurrency, &mut progress_tracker);
```

**2. Storage Runtime** (`algo/pagerank/storage.rs`) - **Controller Pattern**:
```rust
pub fn run(
    &self,
    computation: &PageRankComputationRuntime,
    concurrency: usize,
    progress_tracker: &mut dyn ProgressTracker,
) -> PageRankRunResult
```
- **Orchestrates power iteration**: projects graph with orientation/types, precomputes out-degree, streams neighbors, and drives computation run loop
- **Graph access**: owns graph handle; computation stays graph-free
- **Progress**: logs iterations via provided progress tracker

**3. Computation Runtime** (`algo/pagerank/computation.rs`) - **Pure State Management**:
```rust
pub struct PageRankComputationRuntime { max_iterations, damping_factor, tolerance, source_nodes }

pub fn run(
    &self,
    node_count: usize,
    out_degree: &[usize],
    concurrency: usize,
    stream_neighbors: &(impl Fn(usize, &mut dyn FnMut(usize)) + Sync),
) -> PageRankRunResult
```
- **No graph access**: consumes degree vector and neighbor-stream callback supplied by storage
- **State-only**: initializes teleport vector, applies damping/dangling mass, and checks convergence via max delta
- **Concurrency-aware**: uses provided concurrency for parallel contribution accumulation

## Universal Pattern Summary

All centrality algorithms now follow the required controller pattern:

```
Applications → Procedures → Storage Runtime (Controller + Graph Access) → Computation Runtime (Pure State)
```
