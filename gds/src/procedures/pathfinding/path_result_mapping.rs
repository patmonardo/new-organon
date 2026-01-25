use crate::algo::common::result_builders::PathResult as CorePathResult;
use crate::collections::backends::vec::VecDouble;
use crate::procedures::{PathResult as ProcedurePathResult, Result};
use crate::projection::RelationshipType;
use crate::types::graph_store::GraphStore;
use crate::types::prelude::DefaultGraphStore;
use crate::types::properties::relationship::DefaultDoubleRelationshipPropertyValues;
use crate::types::properties::relationship::RelationshipPropertyValues;
use crate::types::schema::Direction;
use std::sync::Arc;

// Additional import for error handling
use crate::projection::eval::algorithm::AlgorithmError;

pub(crate) fn core_to_procedure_path_result(path: CorePathResult) -> ProcedurePathResult {
    ProcedurePathResult {
        source: path.source,
        target: path.target,
        path: path.path,
        cost: path.cost,
    }
}

pub(crate) fn build_path_relationship_store(
    graph_store: &DefaultGraphStore,
    relationship_type: &str,
    paths: &[ProcedurePathResult],
) -> Result<Arc<DefaultGraphStore>> {
    let node_count = graph_store.node_count();
    let mut outgoing: Vec<Vec<i64>> = vec![Vec::new(); node_count];
    let mut costs_by_source: Vec<Vec<f64>> = vec![Vec::new(); node_count];

    for path in paths {
        let source = path.source as usize;
        let target = path.target as usize;
        if source >= node_count || target >= node_count {
            continue;
        }
        outgoing[source].push(path.target as i64);
        costs_by_source[source].push(path.cost);
    }

    let rel_type = RelationshipType::of(relationship_type);

    let mut updated = graph_store
        .with_added_relationship_type_preserve_name(rel_type.clone(), outgoing, Direction::Directed)
        .map_err(|e| AlgorithmError::Execution(format!("Failed to add path relationships: {e}")))?;

    let mut flat_costs: Vec<f64> = Vec::new();
    for costs in costs_by_source {
        flat_costs.extend(costs);
    }

    let pv: Arc<dyn RelationshipPropertyValues> = Arc::new(
        DefaultDoubleRelationshipPropertyValues::<VecDouble>::from_collection(
            VecDouble::from(flat_costs),
            updated.relationship_count(),
        ),
    );

    updated
        .add_relationship_property(rel_type, "totalCost".to_string(), pv)
        .map_err(|e| {
            AlgorithmError::Execution(format!(
                "Failed to add totalCost relationship property: {e}"
            ))
        })?;

    Ok(Arc::new(updated))
}
