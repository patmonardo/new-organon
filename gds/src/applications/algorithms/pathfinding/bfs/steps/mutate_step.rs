use crate::core::loading::GraphResources;
use crate::procedures::traits::PathResult;
use crate::types::catalog::GraphCatalog;
use crate::types::graph_store::GraphStore;
use serde::Serialize;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize)]
pub struct BfsMutateOutcome {
    pub graph_name: String,
    pub property_name: String,
    pub nodes_updated: u64,
}

pub struct BfsMutateStep {
    pub catalog: Arc<dyn GraphCatalog>,
    pub output_graph_name: String,
    pub property_name: String,
}

impl BfsMutateStep {
    pub fn execute(
        &self,
        graph_resources: &GraphResources,
        rows: &[PathResult],
    ) -> Result<BfsMutateOutcome, String> {
        let node_count = graph_resources.store().node_count();
        let mut values: Vec<i64> = vec![-1; node_count];

        for row in rows {
            let idx = row.target as usize;
            if idx >= node_count {
                continue;
            }
            values[idx] = row.cost as i64;
        }

        let mut new_store = graph_resources.store().as_ref().clone();
        new_store
            .add_node_property_i64(self.property_name.clone(), values)
            .map_err(|e| e.to_string())?;

        self.catalog
            .set(&self.output_graph_name, Arc::new(new_store));

        let nodes_updated = rows.len() as u64;
        Ok(BfsMutateOutcome {
            graph_name: self.output_graph_name.clone(),
            property_name: self.property_name.clone(),
            nodes_updated,
        })
    }
}
