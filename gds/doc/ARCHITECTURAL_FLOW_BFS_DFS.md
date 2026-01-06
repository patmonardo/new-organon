# GDS Pathfinding Architecture: BFS & DFS

## Three-Layer Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                    APPLICATIONS LAYER                       │
│  (Orchestration & Result Transformation)                    │
├─────────────────────────────────────────────────────────────┤
│                    PROCEDURES LAYER                         │
│  (Fluent API & Configuration)                               │
├─────────────────────────────────────────────────────────────┤
│                    ALGORITHMS LAYER                         │
│  (Core Computation & Raw Results)                           │
└─────────────────────────────────────────────────────────────┘
```

## Layer Responsibilities

### Applications Layer
- **Purpose**: High-level orchestration and result formatting
- **Components**: AlgorithmMachinery, ResultBuilders, Request handling
- **Java Parity**: Matches `org.neo4j.gds.applications.*` packages
- **Key Pattern**: Orchestrates algorithm execution, transforms raw results

### Procedures Layer
- **Purpose**: User-facing fluent API and configuration
- **Components**: Builder patterns, validation, execution modes
- **Java Parity**: Matches `org.neo4j.gds.procedures.*` packages
- **Key Pattern**: Fluent configuration → execution modes (stream/stats/mutate/write)

### Algorithms Layer
- **Purpose**: Pure computational logic
- **Components**: StorageRuntime, ComputationRuntime, algorithm specs
- **Java Parity**: Matches `org.neo4j.gds.algo.*` packages
- **Key Pattern**: Raw computation → minimal result structures

---

## BFS Complete Flow

### 1. Applications Layer (BFS Stream Mode)

```rust
pub fn run(op: &str, request: &BfsRequest, graph_resources: &GraphResources) -> Value {
    // Setup orchestration infrastructure
    let deps = RequestScopedDependencies::new(/*...*/);
    let template = DefaultAlgorithmProcessingTemplate::new(/*...*/);
    let convenience = AlgorithmProcessingTemplateConvenience::new(template);

    // Define computation closure (calls algorithm directly)
    let compute = |gr: &GraphResources, tracker: &mut dyn ProgressTracker, _termination| {
        // Get graph view
        let graph_view = gr.graph_store.get_graph_with_types_and_orientation(/*...*/)?;

        // Create algorithm runtimes
        let storage = BfsStorageRuntime::new(source_node, targets, max_depth, track_paths);
        let mut computation = BfsComputationRuntime::new(source_node, track_paths, concurrency, node_count);

        // Execute via AlgorithmMachinery (Java parity)
        let result = AlgorithmMachinery::run_algorithms_and_manage_progress_tracker(
            tracker, false, concurrency,
            |tracker| {
                storage.compute_bfs(&mut computation, Some(graph_view.as_ref()), tracker)
                    .map(|r| r.visited_nodes)  // Extract raw TraversalResult
                    .map_err(|e| format!("BFS algorithm failed: {:?}", e))
            }
        )?;

        Ok(Some(result))  // Returns: TraversalResult = Vec<i64>
    };

    // Use ResultBuilder for transformation
    let result_builder = PathFindingStreamResultBuilder::new(request.track_paths);

    // Process with convenience wrapper
    convenience.process_stream(graph_resources, concurrency, task, compute, result_builder)
}
```

**Key Points:**
- Calls algorithm directly via `AlgorithmMachinery`
- Gets raw `TraversalResult` (Vec<i64>)
- Uses `ResultBuilder` to transform into `PathResult` stream
- Matches Java GDS Applications layer pattern

### 2. Procedures Layer (BFS Builder)

```rust
pub struct BfsBuilder {
    graph_store: Arc<DefaultGraphStore>,
    source: Option<u64>,
    targets: Vec<u64>,
    max_depth: Option<u32>,
    track_paths: bool,
    concurrency: usize,
}

impl BfsBuilder {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self { /*...*/ }

    pub fn source(mut self, source: u64) -> Self { self.source = Some(source); self }
    pub fn targets(mut self, targets: Vec<u64>) -> Self { self.targets = targets; self }
    pub fn max_depth(mut self, max_depth: u32) -> Self { self.max_depth = Some(max_depth); self }
    pub fn track_paths(mut self, track_paths: bool) -> Self { self.track_paths = track_paths; self }

    pub fn stream(self) -> Result<Box<dyn Iterator<Item = PathResult>>> {
        let source_u64 = self.source.expect("validate() ensures source is set");
        let traversal_order = self.compute()?;  // Calls compute()

        // Transform raw TraversalResult into PathResult stream
        let paths = traversal_order.into_iter().enumerate().map(move |(index, node_id)| PathResult {
            source: source_u64,
            target: node_id as u64,
            path: vec![node_id as u64],
            cost: index as f64,
        });
        Ok(Box::new(paths))
    }

    pub fn stats(self) -> Result<BfsStats> {
        let traversal_order = self.compute()?;
        Ok(BfsStats {
            nodes_visited: traversal_order.len() as u64,
            max_depth_reached: 0,
            execution_time_ms: 0,
            targets_found: 0,
            all_targets_reached: false,
            avg_branching_factor: 0.0,
        })
    }

    fn compute(self) -> Result<TraversalResult> {
        // Setup algorithm runtimes
        let storage = BfsStorageRuntime::new(source_node, target_nodes, max_depth, track_paths);
        let mut computation = BfsComputationRuntime::new(source_node, track_paths, concurrency, node_count);

        // Execute algorithm directly
        let result = storage.compute_bfs(&mut computation, Some(graph_view.as_ref()), &mut progress_tracker)?;

        // Return raw traversal order
        Ok(result.visited_nodes.into_iter().map(|node_id| node_id).collect())
    }
}
```

**Key Points:**
- Fluent builder API for configuration
- `compute()` returns raw `TraversalResult`
- `stream()`/`stats()` transform raw results into user-friendly formats
- Acts as facade over algorithm layer

### 3. Algorithms Layer (BFS Core)

```rust
// spec.rs - Algorithm specification
pub struct BfsConfig {
    pub source_node: NodeId,
    pub target_nodes: Vec<NodeId>,
    pub max_depth: Option<u32>,
    pub track_paths: bool,
    pub concurrency: usize,
}

pub struct BfsResult {
    pub visited_nodes: Vec<NodeId>,  // Raw traversal order
    pub computation_time_ms: u64,
}

// storage.rs - Graph orchestration
impl BfsStorageRuntime {
    pub fn compute_bfs(
        &self,
        computation: &mut BfsComputationRuntime,
        graph: Option<&dyn Graph>,
        tracker: &mut dyn ProgressTracker,
    ) -> Result<BfsResult, AlgorithmError> {
        // BFS traversal logic using queue
        let mut queue = VecDeque::new();
        queue.push_back(self.source_node);

        while let Some(current_node) = queue.pop_front() {
            // Visit neighbors, update computation state
            if let Some(graph) = graph {
                for neighbor in graph.neighbors(current_node) {
                    if computation.visit_neighbor(current_node, neighbor, graph) {
                        queue.push_back(neighbor);
                    }
                }
            }
        }

        // Return raw result
        Ok(BfsResult {
            visited_nodes: computation.get_visited_nodes(),
            computation_time_ms: 0,  // Simplified
        })
    }
}

// computation.rs - State management
impl BfsComputationRuntime {
    pub fn new(source_node: NodeId, track_paths: bool, concurrency: usize, node_count: usize) -> Self {
        Self {
            visited: vec![false; node_count],  // BitSet pattern
            visit_order: Vec::new(),
            // ... other state
        }
    }

    pub fn visit_neighbor(&mut self, from: NodeId, to: NodeId, graph: &dyn Graph) -> bool {
        if !self.visited[to as usize] {
            self.visited[to as usize] = true;
            self.visit_order.push(to);
            return true;
        }
        false
    }

    pub fn get_visited_nodes(&self) -> Vec<NodeId> {
        self.visit_order.clone()
    }
}
```

**Key Points:**
- Pure computational logic
- Returns minimal `BfsResult` with `visited_nodes: Vec<NodeId>`
- Separation of concerns: storage (graph access) vs computation (state management)
- No result formatting - just raw data

---

## DFS Complete Flow

### Current State: Mixed Architecture (Needs Update)

**Applications Layer (DFS Stream - OLD PATTERN):**
```rust
let compute = |gr: &GraphResources, _tracker, _termination| {
    // Calls procedure instead of algorithm directly
    let mut builder = gr.facade().dfs().source(request.source);
    let iter = builder.stream().map_err(|e| e.to_string())?;
    let rows: Vec<PathResult> = iter.collect();
    Ok(Some(rows))
};
```

**Procedures Layer (DFS - UPDATED):**
- Similar to BFS: `compute()` returns `TraversalResult`
- `stream()` transforms raw results into `PathResult`

**Algorithms Layer (DFS - UPDATED):**
- Returns `DfsResult { visited_nodes: Vec<NodeId> }`
- Uses stack-based traversal with `Vec<bool>` visited tracking

### Required: Update DFS Applications Layer

To achieve full Java parity, DFS Applications layer needs same pattern as BFS:

```rust
// NEW: AlgorithmMachinery + ResultBuilder pattern
let compute = |gr: &GraphResources, tracker, _termination| {
    let graph_view = gr.graph_store.get_graph_with_types_and_orientation(/*...*/)?;
    let storage = DfsStorageRuntime::new(/*...*/);
    let mut computation = DfsComputationRuntime::new(/*...*/);

    let result = AlgorithmMachinery::run_algorithms_and_manage_progress_tracker(
        tracker, false, concurrency,
        |tracker| {
            storage.compute_dfs(&mut computation, Some(graph_view.as_ref()), tracker)
                .map(|r| r.visited_nodes)
                .map_err(|e| format!("DFS algorithm failed: {:?}", e))
        }
    )?;
    Ok(Some(result))
};

let result_builder = PathFindingStreamResultBuilder::new(request.track_paths);
convenience.process_stream(graph_resources, concurrency, task, compute, result_builder)
```

---

## Data Flow Summary

### BFS Complete Flow:
```
User Request → Applications Layer → AlgorithmMachinery → StorageRuntime.compute_bfs()
    ↓              ↓                        ↓
BfsRequest → PathFindingStreamResultBuilder → ComputationRuntime
    ↓              ↓                        ↓
    ↓              ↓                     TraversalResult (Vec<i64>)
    ↓              ↓                        ↓
    ↓           PathResult Stream           ↓
    ↓              ↓                        ↓
JSON Response ← ResultBuilder ← Raw Algorithm Result
```

### Key Architectural Principles:
1. **Applications Layer**: Orchestrates execution, handles result transformation
2. **Procedures Layer**: User API, configuration, basic result formatting
3. **Algorithms Layer**: Pure computation, minimal result structures
4. **ResultBuilders**: Transform raw algorithm outputs into procedure-friendly formats
5. **AlgorithmMachinery**: Manages progress tracking, concurrency, lifecycle

### Java Parity Achieved:
- ✅ Applications layer uses `AlgorithmMachinery.runAlgorithmsAndManageProgressTracker`
- ✅ Algorithms return raw results (`TraversalResult` = `HugeLongArray` in Java)
- ✅ `ResultBuilders` handle transformation to final formats
- ✅ Clean separation between orchestration, API, and computation

This architecture enables maximum flexibility, testability, and reusability across different execution modes and result formats.
