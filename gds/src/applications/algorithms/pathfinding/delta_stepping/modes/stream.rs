use crate::applications::algorithms::machinery::{
    AlgorithmMachinery, AlgorithmProcessingTemplateConvenience, DefaultAlgorithmProcessingTemplate,
    ProgressTrackerCreator, RequestScopedDependencies,
};
use crate::applications::algorithms::pathfinding::shared::{PathFindingStreamResultBuilder, TraversalResult};
use crate::applications::algorithms::pathfinding::delta_stepping::request::DeltaSteppingRequest;
use crate::applications::algorithms::pathfinding::shared::err;
use crate::concurrency::TerminationFlag;
use crate::core::loading::GraphResources;
use crate::core::utils::progress::{JobId, ProgressTracker, TaskRegistryFactories, Tasks};
use crate::graph_store::GraphStore;
use crate::procedures::traits::PathResult;
use serde_json::{json, Value};

pub fn run(op: &str, request: &DeltaSteppingRequest, graph_resources: &GraphResources) -> Value {
    let deps = RequestScopedDependencies::new(
        JobId::new(),
        TaskRegistryFactories::empty(),
        TerminationFlag::running_true(),
    );
    let creator = ProgressTrackerCreator::new(deps);
    let template = DefaultAlgorithmProcessingTemplate::new(creator);
    let convenience = AlgorithmProcessingTemplateConvenience::new(template);

    let task = Tasks::leaf("DeltaStepping::stream".to_string()).base().clone();

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
        let mut storage = crate::algo::delta_stepping::DeltaSteppingStorageRuntime::new(
            source_node,
            request.delta,
            1, // concurrency
            request.store_predecessors,
        );

        let mut computation = crate::algo::delta_stepping::DeltaSteppingComputationRuntime::new(
            source_node,
            request.delta,
            1, // concurrency
            request.store_predecessors,
        );
        let node_count = graph_view.node_count();
        computation.initialize(source_node, request.delta, request.store_predecessors, node_count);

        let direction = if request.direction == "incoming" { 1 } else { 0 };

        let result = AlgorithmMachinery::run_algorithms_and_manage_progress_tracker(
            tracker,
            false, // release_progress_tracker
            crate::concurrency::Concurrency::of(request.common.concurrency.value()),
            |tracker| {
                storage.compute_delta_stepping(&mut computation, Some(graph_view.as_ref()), direction, tracker)
                    .map(|_| computation.get_visited_nodes().into_iter().map(|n| n as i64).collect())
                    .map_err(|e| format!("Delta Stepping algorithm failed: {:?}", e))
            },
        )?;

        Ok(Some(result))
    };

    let result_builder = PathFindingStreamResultBuilder::new(true);

    match convenience.process_stream(graph_resources, request.common.concurrency, task, compute, result_builder) {
        Ok(stream) => {
            let rows: Vec<PathResult> = stream.collect();
            json!({
                "ok": true,
                "op": op,
                "mode": "stream",
                "data": rows
            })
        },
        Err(e) => err(op, "EXECUTION_ERROR", &format!("Delta-Stepping stream failed: {e}")),
    }
}
