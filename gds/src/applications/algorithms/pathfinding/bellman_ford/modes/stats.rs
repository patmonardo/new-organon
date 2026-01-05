use crate::applications::algorithms::machinery::{
    AlgorithmMachinery, AlgorithmProcessingTemplateConvenience, DefaultAlgorithmProcessingTemplate,
    ProgressTrackerCreator, RequestScopedDependencies,
};
use crate::applications::algorithms::pathfinding::shared::{PathFindingStatsResultBuilder, TraversalResult};
use crate::applications::algorithms::pathfinding::bellman_ford::request::BellmanFordRequest;
use crate::applications::algorithms::pathfinding::shared::err;
use crate::concurrency::TerminationFlag;
use crate::core::loading::GraphResources;
use crate::core::utils::progress::{JobId, ProgressTracker, TaskRegistryFactories, Tasks};
use crate::graph_store::GraphStore;
use serde_json::Value;

pub fn run(op: &str, request: &BellmanFordRequest, graph_resources: &GraphResources) -> Value {
    let deps = RequestScopedDependencies::new(
        JobId::new(),
        TaskRegistryFactories::empty(),
        TerminationFlag::running_true(),
    );
    let creator = ProgressTrackerCreator::new(deps);
    let template = DefaultAlgorithmProcessingTemplate::new(creator);
    let convenience = AlgorithmProcessingTemplateConvenience::new(template);

    let task = Tasks::leaf("BellmanFord::stats".to_string()).base().clone();

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
        let mut storage = crate::algo::bellman_ford::BellmanFordStorageRuntime::new(
            source_node,
            request.track_negative_cycles,
            request.track_paths,
            1, // concurrency
        );

        let mut computation = crate::algo::bellman_ford::BellmanFordComputationRuntime::new(
            source_node,
            request.track_negative_cycles,
            request.track_paths,
            1, // concurrency
        );

        let direction = if request.direction == "incoming" { 1 } else { 0 };

        let result = AlgorithmMachinery::run_algorithms_and_manage_progress_tracker(
            tracker,
            false, // release_progress_tracker
            crate::concurrency::Concurrency::of(request.common.concurrency.value()),
            |tracker| {
                storage.compute_bellman_ford(&mut computation, Some(graph_view.as_ref()), direction, tracker)
                    .map(|_| computation.get_visited_nodes().into_iter().map(|n| n as i64).collect())
                    .map_err(|e| format!("Bellman-Ford algorithm failed: {:?}", e))
            },
        )?;

        Ok(Some(result))
    };

    let result_builder = PathFindingStatsResultBuilder::new();

    match convenience.process_stats(graph_resources, request.common.concurrency, task, compute, result_builder) {
        Ok(v) => v,
        Err(e) => err(op, "EXECUTION_ERROR", &format!("Bellman-Ford stats failed: {e}")),
    }
}
