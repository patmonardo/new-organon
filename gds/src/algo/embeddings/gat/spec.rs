use crate::algo::embeddings::GATConfig;
use crate::algo::embeddings::GATResult;
use crate::define_algorithm_spec;
use crate::projection::eval::procedure::AlgorithmError;
use crate::projection::orientation::Orientation;
use serde_json;

// ============================================================================
// Algorithm Spec
// ============================================================================

define_algorithm_spec! {
    name: "gat",
    output_type: GATResult,
    projection_hint: Dense,
    modes: [Stream],

    execute: |_self, graph_store, config_input, _context| {
        let config: GATConfig = serde_json::from_value(config_input.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Failed to parse GAT config: {e}")))?;

        if config.embedding_dimension == 0 {
            return Err(AlgorithmError::Execution("embedding_dimension must be > 0".to_string()));
        }
        if config.num_layers == 0 {
            return Err(AlgorithmError::Execution("num_layers must be > 0".to_string()));
        }
        if config.num_heads == 0 {
            return Err(AlgorithmError::Execution("num_heads must be > 0".to_string()));
        }

        // Load graph
        let rel_types = std::collections::HashSet::new();
        let graph = graph_store
            .get_graph_with_types_and_orientation(&rel_types, Orientation::Natural)
            .map_err(|e| AlgorithmError::Graph(e.to_string()))?;

        // Run computation
        Ok(crate::algo::embeddings::gat::storage::GATStorageRuntime::new()
            .compute(graph.as_ref(), &config))
    }
}
