# Pathfinding Algorithms - Architectural Review

## Overview

This document reviews the architectural compliance of pathfinding algorithms with the universal `compute_{algo}` controller pattern. The pattern requires:

- **Storage Runtime**: Acts as controller, orchestrates algorithm loops, handles graph access
- **Computation Runtime**: Pure state management, no graph access, only ephemeral state operations
- **Procedure Layer**: Top-level compute entrypoint; creates both runtimes and calls `storage.compute_{algo}()`

## Current Status Summary

**Total Pathfinding Algorithms**: 15
- ✅ **Fully Architecturally Sound** (9 algorithms): BFS, DFS, A*, Bellman-Ford, Dijkstra, Delta Stepping, Yen's, steiner_tree, prize_collecting_steiner_tree
- ⚠️ **Partially Compliant** (3 algorithms): all_shortest_paths, kspanningtree, spanning_tree (have compute_ methods but don't use computation runtime properly)
- ❌ **Non-Compliant** (3 algorithms): dag_longest_path, random_walk, topological_sort

**Reviewed**: 9/9 fully compliant algorithms (100% complete)
**Remaining Issues**: 3 partially compliant + 3 non-compliant algorithms to address

## BFS Algorithm - ✅ **Architecturally Sound**

### Layered Architecture

**1. Procedure Layer** (`procedures/pathfinding/bfs.rs`):
```rust
// Creates both runtimes (factory pattern)
let storage = BfsStorageRuntime::new(source_node, target_nodes, max_depth, track_paths);
let mut computation = BfsComputationRuntime::new(source_node, track_paths, concurrency, node_count);

// Calls storage.compute_bfs() - Applications talk only to procedures
let result = storage.compute_bfs(&mut computation, Some(graph_view.as_ref()), &mut progress_tracker)?;
```

**2. Storage Runtime** (`algo/bfs/storage.rs`) - **Controller Pattern**:
```rust
pub fn compute_bfs(
    &self,                                    // Storage runtime (controller)
    computation: &mut BfsComputationRuntime, // Computation runtime (state)
    graph: Option<&dyn Graph>,
    progress_tracker: &mut dyn ProgressTracker,
) -> Result<BfsResult, AlgorithmError>
```
- **Orchestrates algorithm loop**: Queue management, neighbor iteration
- **Graph access**: `get_neighbors()` method for data access
- **State delegation**: Calls `computation.is_visited()`, `computation.set_visited()`, etc.

**3. Computation Runtime** (`algo/bfs/computation.rs`) - **Pure State Management**:
```rust
pub fn is_visited(&self, node: NodeId) -> bool
pub fn set_visited(&mut self, node: NodeId)
pub fn initialize(&mut self, source_node: NodeId, max_depth: Option<u32>, node_count: usize)
pub fn check_max_depth(&self, current_depth: f64) -> bool
```
- **No graph access** - only manages ephemeral state (visited array, distances)
- **Stateless operations** - pure functions on internal state

### Pattern Compliance
✅ **Storage acts as controller** - orchestrates BFS loop, calls computation methods
✅ **Computation is pure state** - no graph access, only state management
✅ **Procedure creates both** - factory pattern with proper initialization
✅ **Applications use procedures** - procedures layer prevents direct ::algo:: calls

## DFS Algorithm - ✅ **Architecturally Sound**

### Layered Architecture

**1. Procedure Layer** (`procedures/pathfinding/dfs.rs`):
```rust
// Creates both runtimes (factory pattern)
let storage = DfsStorageRuntime::new(source_node, target_nodes, max_depth, track_paths, concurrency);
let mut computation = DfsComputationRuntime::new(source_node, track_paths, concurrency, node_count);

// Calls storage.compute_dfs() - Applications never call ::algo:: directly
let result = storage.compute_dfs(&mut computation, Some(graph_view.as_ref()), &mut progress_tracker)?;
```

**2. Storage Runtime** (`algo/dfs/storage.rs`) - **Controller Pattern**:
```rust
pub fn compute_dfs(
    &self,                                    // Storage runtime (controller)
    computation: &mut DfsComputationRuntime, // Computation runtime (state)
    graph: Option<&dyn Graph>,
    progress_tracker: &mut dyn ProgressTracker,
) -> Result<DfsResult, AlgorithmError>
```
- **Orchestrates algorithm loop**: Stack management (VecDeque), depth-first traversal
- **Graph access**: Direct relationship streaming from graph
- **State delegation**: Calls `computation.is_visited()`, `computation.set_visited()`, `computation.check_max_depth()`

**3. Computation Runtime** (`algo/dfs/computation.rs`) - **Pure State Management**:
```rust
pub fn is_visited(&self, node: NodeId) -> bool
pub fn set_visited(&mut self, node: NodeId)
pub fn initialize(&mut self, source_node: NodeId, max_depth: Option<u32>, node_count: usize)
pub fn check_max_depth(&self, current_depth: f64) -> bool
```
- **No graph access** - only manages ephemeral state (visited array)
- **Stateless operations** - pure functions on internal state

### Pattern Compliance
✅ **Storage acts as controller** - orchestrates DFS stack-based traversal, calls computation methods
✅ **Computation is pure state** - no graph access, only state management
✅ **Procedure creates both** - factory pattern with proper initialization
✅ **Applications use procedures** - procedures layer prevents direct ::algo:: calls

### Key Differences from BFS
- **Traversal Strategy**: DFS uses stacks (VecDeque) vs BFS queues
- **Progress Tracking**: DFS tracks progress per node degree vs BFS per relationship
- **Memory Pattern**: DFS can have deeper recursion patterns but same architectural separation

## A* Algorithm - ✅ **Architecturally Sound**

### Layered Architecture

**1. Procedure Layer** (`procedures/pathfinding/astar.rs`):
```rust
// Creates both runtimes (factory pattern)
let mut storage = AStarStorageRuntime::new(source_node, target_node, "latitude", "longitude");
let mut computation = AStarComputationRuntime::new();

// Calls storage.compute_astar_path() - Applications never call ::algo:: directly
let result = storage.compute_astar_path(&mut computation, Some(graph_view.as_ref()), direction_byte, &mut progress_tracker)?;
```

**2. Storage Runtime** (`algo/astar/storage.rs`) - **Controller Pattern**:
```rust
pub fn compute_astar_path(
    &mut self,                                         // Storage runtime (controller)
    computation: &mut AStarComputationRuntime,        // Computation runtime (state)
    graph: Option<&dyn Graph>,
    direction: u8,
    progress_tracker: &mut dyn ProgressTracker,
) -> Result<AStarComputationResult, String>
```
- **Orchestrates algorithm loop**: Priority queue management, neighbor expansion with f(n) = g(n) + h(n)
- **Graph access**: Relationship streaming with property weights
- **Heuristic computation**: `compute_haversine_distance()` for geographic coordinates
- **State delegation**: Calls `computation.is_visited()`, `computation.update_g_cost()`, `computation.update_f_cost()`, etc.

**3. Computation Runtime** (`algo/astar/computation.rs`) - **Pure State Management**:
```rust
pub fn initialize(&mut self, source: NodeId, _target: NodeId)
pub fn add_to_open_set(&mut self, node: NodeId)
pub fn remove_from_open_set(&mut self, node: NodeId)
pub fn get_lowest_f_cost_node(&self) -> Option<NodeId>
pub fn mark_visited(&mut self, node: NodeId)
pub fn is_visited(&self, node: NodeId) -> bool
pub fn update_g_cost(&mut self, node: NodeId, cost: f64)
pub fn get_g_cost(&self, node: NodeId) -> f64
pub fn update_f_cost(&mut self, node: NodeId, cost: f64)
pub fn set_parent(&mut self, child: NodeId, parent: NodeId)
pub fn reconstruct_path(&self, source: NodeId, target: NodeId) -> Option<Vec<NodeId>>
```
- **No graph access** - only manages ephemeral state (open set, g-costs, f-costs, parent pointers)
- **Complex state operations** - priority queue management, path reconstruction
- **Stateless operations** - pure functions on internal state

### Pattern Compliance
✅ **Storage acts as controller** - orchestrates A* priority queue algorithm, calls computation methods
✅ **Computation is pure state** - no graph access, only state management
✅ **Procedure creates both** - factory pattern with proper initialization
✅ **Applications use procedures** - procedures layer prevents direct ::algo:: calls

### Key Differences from BFS/DFS
- **Priority Queue**: Uses f-cost (g + h) ordering vs FIFO/LIFO
- **Heuristic Integration**: Storage computes h(n), computation manages g(n) and f(n)
- **Path Reconstruction**: Computation runtime handles parent pointer tracking and path building
- **Cost Tracking**: More complex state (g-costs, f-costs, parent relationships)

## Bellman-Ford Algorithm - ✅ **Architecturally Sound**

### Layered Architecture

**1. Procedure Layer** (`procedures/pathfinding/bellman_ford.rs`):
```rust
// Creates both runtimes (factory pattern)
let mut storage = BellmanFordStorageRuntime::new(
    source_node,
    self.track_negative_cycles,
    self.track_paths,
    self.concurrency,
);

let mut computation = BellmanFordComputationRuntime::new(
    source_node,
    self.track_negative_cycles,
    self.track_paths,
    self.concurrency,
);

// Calls storage.compute_bellman_ford() - Applications never call ::algo:: directly
let result = storage.compute_bellman_ford(&mut computation, Some(graph_view.as_ref()), direction_byte, &mut progress_tracker)?;
```

**2. Storage Runtime** (`algo/bellman_ford/storage.rs`) - **Controller Pattern**:
```rust
pub fn compute_bellman_ford(
    &mut self,                                              // Storage runtime (controller)
    computation: &mut BellmanFordComputationRuntime,      // Computation runtime (state)
    graph: Option<&dyn Graph>,
    direction: u8,
    progress_tracker: &mut dyn ProgressTracker,
) -> Result<BellmanFordResult, AlgorithmError>
```
- **Orchestrates algorithm loop**: Iterative relaxation with frontier management, V-1 iterations max
- **Graph access**: `get_neighbors_with_weights()` for weighted edge access
- **Negative cycle detection**: Tracks path lengths > V to detect negative cycles
- **State delegation**: Calls `computation.distance()`, `computation.set_distance()`, `computation.set_predecessor()`, etc.

**3. Computation Runtime** (`algo/bellman_ford/computation.rs`) - **Pure State Management**:
```rust
pub fn initialize(&mut self, source_node: NodeId, track_negative_cycles: bool, track_paths: bool, node_count: usize)
pub fn distance(&self, node_id: NodeId) -> f64
pub fn set_distance(&mut self, node_id: NodeId, distance: f64)
pub fn predecessor(&self, node_id: NodeId) -> Option<NodeId>
pub fn set_predecessor(&mut self, node_id: NodeId, predecessor: Option<NodeId>)
pub fn length(&self, node_id: NodeId) -> u32
pub fn set_length(&mut self, node_id: NodeId, length: u32)
pub fn add_negative_cycle_node(&mut self, node_id: NodeId)
pub fn has_negative_cycles(&self) -> bool
pub fn get_negative_cycle_nodes(&self) -> &[NodeId]
pub fn compare_and_exchange(&mut self, node_id: NodeId, new_distance: f64, new_predecessor: Option<NodeId>) -> bool
```
- **No graph access** - only manages ephemeral state (distances, predecessors, path lengths, negative cycle tracking)
- **Complex state operations** - atomic distance updates, path reconstruction, negative cycle detection
- **Stateless operations** - pure functions on internal state

### Pattern Compliance
✅ **Storage acts as controller** - orchestrates Bellman-Ford relaxation algorithm, calls computation methods
✅ **Computation is pure state** - no graph access, only state management
✅ **Procedure creates both** - factory pattern with proper initialization
✅ **Applications use procedures** - procedures layer prevents direct ::algo:: calls

### Key Differences from Other Pathfinding Algorithms
- **Relaxation Algorithm**: Iterative edge relaxation vs priority queue (A*) or simple traversal (BFS/DFS)
- **Negative Cycle Detection**: Advanced cycle detection with path length tracking
- **Convergence Guarantee**: V-1 iterations maximum vs potentially unbounded (A*) or complete traversal (BFS/DFS)
- **State Complexity**: Distance arrays, predecessor arrays, path length tracking

## Dijkstra Algorithm - ✅ **Architecturally Sound**

### Layered Architecture

**1. Procedure Layer** (`procedures/pathfinding/dijkstra.rs`):
```rust
// Creates both runtimes (factory pattern)
let mut storage = DijkstraStorageRuntime::new(source_node, self.track_relationships, self.concurrency, false);
let mut computation = DijkstraComputationRuntime::new(source_node, self.track_relationships, self.concurrency, false);

// Calls storage.compute_dijkstra() - Applications never call ::algo:: directly
let result = storage.compute_dijkstra(&mut computation, targets, Some(graph_view.as_ref()), direction_byte, &mut progress_tracker)?;
```

**2. Storage Runtime** (`algo/dijkstra/storage.rs`) - **Controller Pattern**:
```rust
pub fn compute_dijkstra(
    &mut self,
    computation: &mut DijkstraComputationRuntime,
    targets: crate::algo::dijkstra::targets::TargetSet,
    graph: Option<&dyn Graph>,
    direction: u8,
    progress_tracker: &mut dyn ProgressTracker,
) -> Result<DijkstraResult, AlgorithmError>
```
- **Orchestrates algorithm loop**: Priority queue management with decrease-key operations
- **Graph access**: Weighted relationship streaming with property selectors
- **State delegation**: Calls `computation` methods for distance updates and path tracking

**3. Computation Runtime** (`algo/dijkstra/computation.rs`) - **Pure State Management**:
- **Priority Queue Management**: Binary heap with node distances and decrease-key operations
- **Distance Tracking**: Maintains g-scores (actual path costs from source)
- **Path Reconstruction**: Predecessor pointers for optimal path building
- **No graph access** - only manages ephemeral state

### Pattern Compliance
✅ **Storage acts as controller** - orchestrates Dijkstra's priority queue algorithm
✅ **Computation is pure state** - no graph access, only state management
✅ **Procedure creates both** - factory pattern with proper initialization
✅ **Applications use procedures** - procedures layer prevents direct ::algo:: calls

### Key Differences from BFS/DFS
- **Priority Queue**: Uses decrease-key operations vs FIFO/LIFO queues
- **Weighted Edges**: Considers edge weights vs unit weights
- **Optimality Guarantee**: Guaranteed shortest paths in non-negative graphs
- **Complexity**: O((V + E) log V) vs O(V + E) for unweighted graphs

## Delta Stepping Algorithm - ✅ **Architecturally Sound**

### Layered Architecture

**1. Procedure Layer** (`procedures/pathfinding/delta_stepping.rs`):
```rust
// Creates both runtimes (factory pattern)
let mut storage = DeltaSteppingStorageRuntime::new(source_node, self.delta, self.concurrency, self.store_predecessors);
let mut computation = DeltaSteppingComputationRuntime::new(source_node, self.delta, self.concurrency, self.store_predecessors);

// Calls storage.compute_delta_stepping() - Applications never call ::algo:: directly
let result = storage.compute_delta_stepping(&mut computation, Some(graph_view.as_ref()), direction_byte, &mut progress_tracker)?;
```

**2. Storage Runtime** (`algo/delta_stepping/storage.rs`) - **Controller Pattern**:
```rust
pub fn compute_delta_stepping(
    &mut self,
    computation: &mut DeltaSteppingComputationRuntime,
    graph: Option<&dyn Graph>,
    direction: u8,
    progress_tracker: &mut dyn ProgressTracker,
) -> Result<DeltaSteppingResult, AlgorithmError>
```
- **Orchestrates algorithm loop**: Bucket-based parallel shortest path computation
- **Graph access**: Weighted relationship streaming with delta-binning optimization
- **State delegation**: Calls `computation` methods for bucket management and distance updates

**3. Computation Runtime** (`algo/delta_stepping/computation.rs`) - **Pure State Management**:
- **Bucket Management**: Parallel processing with delta-binned distance ranges
- **Distance Arrays**: Maintains tentative distances with atomic updates
- **Light Edges/Heavy Edges**: Separates edges by weight for parallel processing
- **No graph access** - only manages ephemeral state

### Pattern Compliance
✅ **Storage acts as controller** - orchestrates delta-stepping bucket algorithm
✅ **Computation is pure state** - no graph access, only state management
✅ **Procedure creates both** - factory pattern with proper initialization
✅ **Applications use procedures** - procedures layer prevents direct ::algo:: calls

### Key Differences from Dijkstra
- **Parallel Processing**: Bucket-based parallelism vs sequential priority queue
- **Delta Parameter**: Bins edges by weight ranges for efficiency
- **Memory vs Time Tradeoff**: More memory for faster computation on large graphs
- **Scalability**: Better parallelization for high-degree graphs

## Yen's Algorithm - ✅ **Architecturally Sound**

### Layered Architecture

**1. Procedure Layer** (`procedures/pathfinding/yens.rs`):
```rust
// Creates both runtimes (factory pattern)
let storage = YensStorageRuntime::new(source_node, target_node, self.k, self.track_relationships, self.concurrency);
let mut computation = YensComputationRuntime::new(source_node, target_node, self.k, self.track_relationships, self.concurrency);

// Calls storage.compute_yens() - Applications never call ::algo:: directly
let result = storage.compute_yens(&mut computation, Some(graph_view.as_ref()), direction_byte, &mut progress_tracker)?;
```

**2. Storage Runtime** (`algo/yens/storage.rs`) - **Controller Pattern**:
```rust
pub fn compute_yens(
    &self,
    computation: &mut YensComputationRuntime,
    graph: Option<&dyn Graph>,
    direction: u8,
    progress_tracker: &mut dyn ProgressTracker,
) -> Result<YensResult, AlgorithmError>
```
- **Orchestrates algorithm loop**: K-shortest paths using candidate path elimination
- **Graph access**: Dijkstra-based shortest path computations with edge removals
- **State delegation**: Calls `computation` methods for path set management and candidate evaluation

**3. Computation Runtime** (`algo/yens/computation.rs`) - **Pure State Management**:
- **Path Set Management**: Maintains k shortest paths found so far
- **Candidate Evaluation**: Tracks potential path extensions and loop elimination
- **Deviation Points**: Manages spur node tracking for path diversification
- **No graph access** - only manages ephemeral path state

### Pattern Compliance
✅ **Storage acts as controller** - orchestrates Yen's k-shortest path algorithm
✅ **Computation is pure state** - no graph access, only state management
✅ **Procedure creates both** - factory pattern with proper initialization
✅ **Applications use procedures** - procedures layer prevents direct ::algo:: calls

### Key Differences from Single Shortest Path Algorithms
- **K Paths**: Computes multiple diverse paths vs single optimal path
- **Path Diversity**: Ensures paths don't overlap too much (deviation points)
- **Candidate Elimination**: Removes invalid path extensions
- **Complexity**: O(K * Dijkstra) where K is number of paths requested

## Partially Compliant Algorithms ⚠️

### Issues Found

**all_shortest_paths**: Has `compute_shortest_paths` and `compute_all_shortest_paths_streaming` methods, but procedure only creates storage runtime and calls streaming method without computation runtime parameter.

**kspanningtree**: Has `compute_kspanningtree` method, but procedure calls `computation.compute()` directly instead of `storage.compute_kspanningtree(&mut computation)`.

**spanning_tree**: Has multiple `compute_spanning_tree*` methods, but procedure only creates storage runtime without separate computation runtime.

## Non-Compliant Algorithms ❌

### Missing compute_{algo} Methods
- `dag_longest_path/storage.rs` - no compute_ methods
- `random_walk/storage.rs` - no compute_ methods
- `topological_sort/storage.rs` - no compute_ methods

## Universal Pattern Summary

All reviewed pathfinding algorithms follow the `compute_{algo}` controller pattern:

```
Applications → Procedures (Top-level compute) → Storage Runtime (Controller + Data) → Computation Runtime (State)
```

### Storage Runtime Responsibilities
- Algorithm orchestration (loops, traversal logic)
- Graph data access and neighbor iteration
- Progress tracking and result construction
- Calling computation runtime for state operations

### Computation Runtime Responsibilities
- Pure state management (visited nodes, distances, costs, etc.)
- No graph access - only internal state operations
- Ephemeral data that resets between runs
- Stateless functions on internal data structures

### Procedure Layer Responsibilities
- Configuration validation and parameter checking
- Creating both storage and computation runtimes
- Calling `storage.compute_{algo}()` method
- Result transformation and return

This architecture ensures clean separation of concerns, testability, and maintainability across all pathfinding algorithms.
