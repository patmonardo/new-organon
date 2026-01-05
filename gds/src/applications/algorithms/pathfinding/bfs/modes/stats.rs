use crate::applications::algorithms::machinery::{
    AlgorithmMachinery, AlgorithmProcessingTemplateConvenience, DefaultAlgorithmProcessingTemplate,
    ProgressTrackerCreator, RequestScopedDependencies,
};
use crate::applications::algorithms::pathfinding::shared::{PathFindingStatsResultBuilder, TraversalResult};
use crate::applications::algorithms::pathfinding::bfs::request::BfsRequest;
use crate::applications::algorithms::pathfinding::shared::err;
use crate::concurrency::TerminationFlag;
use crate::core::loading::GraphResources;
use crate::core::utils::progress::{JobId, ProgressTracker, TaskRegistryFactories, Tasks};
use crate::graph_store::GraphStore;
use serde_json::Value;

pub fn run(op: &str, request: &BfsRequest, graph_resources: &GraphResources) -> Value {
    let deps = RequestScopedDependencies::new(
        JobId::new(),
        TaskRegistryFactories::empty(),
        TerminationFlag::running_true(),
    );
    let creator = ProgressTrackerCreator::new(deps);
    let template = DefaultAlgorithmProcessingTemplate::new(creator);
    let convenience = AlgorithmProcessingTemplateConvenience::new(template);

    let task = Tasks::leaf("BFS::stats".to_string()).base().clone();

    let compute = |gr: &GraphResources,
                   tracker: &mut dyn ProgressTracker,
                   _termination: &TerminationFlag|
     -> Result<Option<TraversalResult>, String> {
        // Get the graph view for algorithm
        let rel_types: std::collections::HashSet<crate::projection::RelationshipType> = std::collections::HashSet::new();
        let graph_view = gr.graph_store.get_graph_with_types_and_orientation(&rel_types, crate::projection::orientation::Orientation::Natural)
            .map_err(|e| format!("Failed to get graph view: {}", e))?;

        let source_node = request.source as i64;

        // Create algorithm runtime
        let storage = crate::algo::bfs::BfsStorageRuntime::new(
            source_node,
            vec![], // targets - empty for full traversal
            request.max_depth,
            false, // track_paths - not needed for stats
        );

        let node_count = graph_view.node_count() as usize;
        let mut computation = crate::algo::bfs::BfsComputationRuntime::new(
            source_node,
            false, // track_paths
            1, // concurrency
            node_count,
        );

        let result = AlgorithmMachinery::run_and_manage_progress_tracker(
            |tracker| {
                storage.compute_bfs(&mut computation, Some(graph_view.as_ref()), tracker)
                    .map(|r| r.visited_nodes)
                    .map_err(|e| format!("BFS algorithm failed: {:?}", e))
            },
            tracker,
        )?;

        Ok(Some(result))
    };

    let result_builder = PathFindingStatsResultBuilder::new();

    match convenience.process_stats(graph_resources, request.common.concurrency, task, compute, result_builder) {
        Ok(v) => v,
        Err(e) => err(op, "EXECUTION_ERROR", &format!("BFS stats failed: {e}")),
    }
}
