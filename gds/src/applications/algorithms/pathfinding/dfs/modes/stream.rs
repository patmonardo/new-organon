use crate::applications::algorithms::machinery::{
    AlgorithmMachinery, AlgorithmProcessingTemplateConvenience, DefaultAlgorithmProcessingTemplate,
    ProgressTrackerCreator, RequestScopedDependencies,
};
use crate::applications::algorithms::pathfinding::shared::{PathFindingStreamResultBuilder, TraversalResult};
use crate::applications::algorithms::pathfinding::dfs::request::DfsRequest;
use crate::applications::algorithms::pathfinding::shared::err;
use crate::concurrency::TerminationFlag;
use crate::core::loading::GraphResources;
use crate::core::utils::progress::{JobId, ProgressTracker, TaskRegistryFactories, Tasks};
use crate::graph_store::GraphStore;
use crate::procedures::traits::PathResult;
use serde_json::{json, Value};

pub fn run(op: &str, request: &DfsRequest, graph_resources: &GraphResources) -> Value {
    let deps = RequestScopedDependencies::new(
        JobId::new(),
        TaskRegistryFactories::empty(),
        TerminationFlag::running_true(),
    );
    let creator = ProgressTrackerCreator::new(deps);
    let template = DefaultAlgorithmProcessingTemplate::new(creator);
    let convenience = AlgorithmProcessingTemplateConvenience::new(template);

    let task = Tasks::leaf("DFS::stream".to_string()).base().clone();

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
        let storage = crate::algo::dfs::DfsStorageRuntime::new(
            source_node,
            vec![], // targets - empty for full traversal
            request.max_depth,
            request.track_paths,
            1, // concurrency
        );

        let node_count = graph_view.node_count() as usize;
        let mut computation = crate::algo::dfs::DfsComputationRuntime::new(
            source_node,
            request.track_paths,
            1, // concurrency
            node_count,
        );

        // Run algorithm via machinery
        let result = AlgorithmMachinery::run_algorithms_and_manage_progress_tracker(
            tracker,
            false, // release_progress_tracker
            crate::concurrency::Concurrency::of(request.common.concurrency.value()),
            |tracker| {
                storage.compute_dfs(&mut computation, Some(graph_view.as_ref()), tracker)
                    .map(|r| r.visited_nodes)
                    .map_err(|e| format!("DFS algorithm failed: {:?}", e))
            },
        )?;

        Ok(Some(result))
    };

    let result_builder = PathFindingStreamResultBuilder::new(request.track_paths);

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
        Err(e) => err(op, "EXECUTION_ERROR", &format!("DFS stream failed: {e}")),
    }
}
