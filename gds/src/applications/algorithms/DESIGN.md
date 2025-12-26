# Business Facade Design

## Control Flow

```
TS Client
    │
    ▼
tsjson_napi.rs::handle_algorithms()
    │
    ▼
BusinessFacade::handle(catalog, request)
    │
    ├─ Extract: op, mode, graphName
    ├─ Lookup graph from catalog
    ├─ Create procedures::facades::Graph
    │
    ▼
handle_{algorithm}(graph, mode, config)
    │
    ├─ mode == "stream" → builder.stream() → JSON rows
    └─ mode == "stats"  → builder.stats()  → JSON stats
```

## Example: PathfindingBusinessFacade

```rust
pub struct PathfindingBusinessFacade;

impl PathfindingBusinessFacade {
    pub fn handle(catalog: &dyn GraphCatalog, request: &Value) -> Value {
        let op = request["op"].as_str();      // "bfs", "dijkstra", etc.
        let mode = request["mode"].as_str();  // "stream", "stats"
        let graph_name = request["graphName"].as_str();
        
        let store = catalog.get(graph_name)?;
        let graph = Graph::new(store);
        
        match op {
            "bfs" => Self::handle_bfs(&graph, mode, request),
            "dfs" => Self::handle_dfs(&graph, mode, request),
            // ...
        }
    }
    
    fn handle_bfs(graph: &Graph, mode: &str, config: &Value) -> Result<Value, String> {
        let builder = graph.bfs()
            .source(config["sourceNode"])
            .targets(config["targetNodes"]);
        
        match mode {
            "stream" => Ok(json!({ "rows": builder.stream()? })),
            "stats"  => Ok(json!(builder.stats()?)),
        }
    }
}
```

## Key Points

1. **One facade per algorithm family** (pathfinding, centrality, community, etc.)
2. **Single `handle()` method** dispatches by algorithm name and mode
3. **Delegates to `procedures::facades`** for actual computation
4. **Returns JSON** - no intermediate types needed

