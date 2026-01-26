use crate::projection::eval::algorithm::AlgorithmError;
use crate::types::graph_store::GraphStore;
use crate::types::prelude::DefaultGraphStore;
use crate::types::properties::relationship::DefaultRelationshipPropertyValues;
use crate::types::properties::relationship::RelationshipPropertyValues;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

pub fn build_similarity_relationship_store(
    graph_store: &DefaultGraphStore,
    property_name: &str,
    pairs: &[(u64, u64, f64)],
) -> Result<Arc<DefaultGraphStore>, AlgorithmError> {
    let mut similarity_by_pair: HashMap<(u64, u64), f64> = HashMap::new();
    for (source, target, similarity) in pairs {
        let key = (*source, *target);
        let entry = similarity_by_pair.entry(key).or_insert(*similarity);
        if *similarity > *entry {
            *entry = *similarity;
        }
    }

    let mut new_store = graph_store.clone();

    for rel_type in graph_store.relationship_types() {
        let mut rel_types = HashSet::new();
        rel_types.insert(rel_type.clone());

        let graph = graph_store.get_graph_with_types(&rel_types).map_err(|e| {
            AlgorithmError::Execution(format!(
                "Similarity mutate failed to read relationships: {e}"
            ))
        })?;

        let fallback = graph.default_property_value();
        let mut values: Vec<f64> = Vec::with_capacity(graph.relationship_count());

        for node_id in 0..graph.node_count() {
            let source = node_id as i64;
            for cursor in graph.stream_relationships(source, fallback) {
                let target = cursor.target_id();
                if target < 0 {
                    continue;
                }
                let key = (source as u64, target as u64);
                let value = similarity_by_pair.get(&key).copied().unwrap_or(0.0);
                values.push(value);
            }
        }

        if values.is_empty() {
            continue;
        }

        let element_count = values.len();
        let pv: Arc<dyn RelationshipPropertyValues> = Arc::new(
            DefaultRelationshipPropertyValues::with_values(values, 0.0, element_count),
        );

        new_store
            .add_relationship_property(rel_type, property_name, pv)
            .map_err(|e| {
                AlgorithmError::Execution(format!("Similarity mutate failed to add property: {e}"))
            })?;
    }

    Ok(Arc::new(new_store))
}
