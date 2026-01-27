use crate::algo::embeddings::gat::GATStorageRuntime;
use crate::algo::embeddings::gat::GATConfig;
use crate::algo::embeddings::gat::GATResult;
use crate::define_algorithm_spec;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::projection::Orientation;
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

        config
            .validate()
            .map_err(|e| AlgorithmError::Execution(e.to_string()))?;

        // Load graph
        let rel_types = std::collections::HashSet::new();
        let graph = graph_store
            .get_graph_with_types_and_orientation(&rel_types, Orientation::Natural)
            .map_err(|e| AlgorithmError::Graph(e.to_string()))?;

        // Run computation
        Ok(GATStorageRuntime::new()
            .compute(graph.as_ref(), &config))
    }
}
