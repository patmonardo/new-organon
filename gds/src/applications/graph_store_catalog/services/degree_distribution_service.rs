use std::collections::HashMap;

use crate::concurrency::TerminationFlag;
use crate::types::graph::degrees::Degrees;
use crate::types::graph::IdMap as _;
use crate::types::graph_store::DefaultGraphStore;

/// Service for computing degree distributions.
///
/// Java’s `DegreeDistributionService` defers to `DegreeDistribution.compute` on the union graph.
/// Here we compute a simple histogram (degree -> count) over the in-memory graph store.
pub struct DegreeDistributionService;

impl DegreeDistributionService {
    pub fn new() -> Self {
        Self
    }

    /// Compute a degree histogram. If `termination_flag` is triggered, the partial histogram is returned.
    pub fn compute(
        &self,
        graph_store: &DefaultGraphStore,
        termination_flag: &TerminationFlag,
    ) -> HashMap<u32, u64> {
        let graph = graph_store.graph();
        let node_count = graph.node_count();
        let mut hist: HashMap<u32, u64> = HashMap::new();

        for mapped in 0..node_count {
            if !termination_flag.running() {
                break;
            }

            let deg = Degrees::degree(graph.as_ref(), mapped as i64) as u32;
            *hist.entry(deg).or_insert(0) += 1;
        }

        hist
    }
}

impl Default for DegreeDistributionService {
    fn default() -> Self {
        Self::new()
    }
}
