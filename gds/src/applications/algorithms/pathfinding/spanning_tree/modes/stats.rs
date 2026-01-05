use crate::applications::algorithms::machinery::{
    AlgorithmMachinery, AlgorithmProcessingTemplateConvenience, DefaultAlgorithmProcessingTemplate,
    FnStatsResultBuilder, ProgressTrackerCreator, RequestScopedDependencies,
};
use crate::applications::algorithms::pathfinding::shared::{err, timings_json};
use crate::applications::algorithms::pathfinding::spanning_tree::request::SpanningTreeRequest;
use crate::concurrency::TerminationFlag;
use crate::core::loading::GraphResources;
use crate::core::utils::progress::{JobId, ProgressTracker, TaskRegistryFactories, Tasks};
use crate::procedures::pathfinding::SpanningTreeStats;
use crate::projection::{Orientation, RelationshipType};
use crate::types::prelude::GraphStore;
use serde_json::{json, Value};

pub fn run(op: &str, request: &SpanningTreeRequest, graph_resources: &GraphResources) -> Value {
    let deps = RequestScopedDependencies::new(
        JobId::new(),
        TaskRegistryFactories::empty(),
        TerminationFlag::running_true(),
    );
    let creator = ProgressTrackerCreator::new(deps);
    let template = DefaultAlgorithmProcessingTemplate::new(creator);
    let convenience = AlgorithmProcessingTemplateConvenience::new(template);

    let task = Tasks::leaf("SpanningTree::stats".to_string()).base().clone();

    let compute = |gr: &GraphResources,
                   tracker: &mut dyn ProgressTracker,
                   _termination: &TerminationFlag|
     -> Result<Option<crate::algo::spanning_tree::SpanningTree>, String> {
        // Get the graph view for algorithm
        let rel_types: std::collections::HashSet<RelationshipType> = if request.relationship_types.is_empty() {
            gr.graph_store.relationship_types()
        } else {
            RelationshipType::list_of(request.relationship_types.clone()).into_iter().collect()
        };

        let orientation = match request.direction.to_ascii_lowercase().as_str() {
            "outgoing" => Orientation::Natural,
            "incoming" => Orientation::Reverse,
            "undirected" => Orientation::Undirected,
            _ => return Err("Invalid direction".to_string()),
        };

        let selectors: std::collections::HashMap<RelationshipType, String> = rel_types
            .iter()
            .map(|t: &RelationshipType| (t.clone(), request.weight_property.clone()))
            .collect();

        let graph_view = gr.graph_store.get_graph_with_types_selectors_and_orientation(&rel_types, &selectors, orientation)
            .map_err(|e| format!("Failed to get graph view: {}", e))?;

        let start_node = request.start_node as u32;
        let direction = match request.direction.to_ascii_lowercase().as_str() {
            "outgoing" => 0u8,
            "incoming" => 1u8,
            "undirected" => 2u8,
            _ => return Err("Invalid direction".to_string()),
        };

        // Create algorithm runtime
        let storage = crate::algo::spanning_tree::SpanningTreeStorageRuntime::new(
            start_node,
            request.compute_minimum,
            request.common.concurrency.value() as usize,
        );

        // Run algorithm via machinery
        let result = AlgorithmMachinery::run_algorithms_and_manage_progress_tracker(
            tracker,
            false, // release_progress_tracker
            crate::concurrency::Concurrency::of(request.common.concurrency.value()),
            |tracker| {
                storage.compute_spanning_tree_with_graph(graph_view.as_ref(), direction, tracker)
                    .map_err(|e| format!("SpanningTree algorithm failed: {:?}", e))
            },
        )?;

        Ok(Some(result))
    };

    let builder = FnStatsResultBuilder(|_gr: &GraphResources,
                                       spanning_tree: Option<crate::algo::spanning_tree::SpanningTree>,
                                       timings: crate::applications::algorithms::machinery::AlgorithmProcessingTimings| {
        let stats = if let Some(tree) = spanning_tree {
            SpanningTreeStats {
                effective_node_count: tree.effective_node_count as u64,
                total_weight: tree.total_weight,
                computation_time_ms: timings.compute_millis as u64,
            }
        } else {
            SpanningTreeStats {
                effective_node_count: 0,
                total_weight: 0.0,
                computation_time_ms: timings.compute_millis as u64,
            }
        };

        json!({
            "ok": true,
            "op": op,
            "mode": "stats",
            "data": stats,
            "timings": timings_json(timings)
        })
    });

    match convenience.process_stats(
        graph_resources,
        request.common.concurrency,
        task,
        compute,
        builder,
    ) {
        Ok(v) => v,
        Err(e) => err(op, "EXECUTION_ERROR", &format!("SpanningTree stats failed: {e}")),
    }
}
