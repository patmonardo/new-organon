use std::collections::{HashMap, HashSet};

use crate::applications::graph_store_catalog::results::GraphStreamRelationshipPropertiesResult;
use crate::projection::RelationshipType;
use crate::types::graph_store::{DefaultGraphStore, GraphStore};
use crate::types::ValueType;

/// StreamRelationshipPropertiesApplication
///
/// Java parity:
/// - For each (relationshipType, propertyKey) pair, build a Graph view with that property selected.
/// - Stream relationships per node; emit (sourceOriginalId, targetOriginalId, relationshipType, propertyKey, propertyValue).
pub struct StreamRelationshipPropertiesApplication;

impl StreamRelationshipPropertiesApplication {
    pub fn compute(
        &self,
        graph_store: &DefaultGraphStore,
        relationship_properties: &[String],
        relationship_types: &[RelationshipType],
    ) -> Result<Vec<GraphStreamRelationshipPropertiesResult>, String> {
        let mut out: Vec<GraphStreamRelationshipPropertiesResult> = Vec::new();

        // Java parity: ElementProjection.PROJECT_ALL is "*".
        let has_wildcard = relationship_types.iter().any(|t| t.name() == "*");
        let types: Vec<RelationshipType> = if relationship_types.is_empty() || has_wildcard {
            graph_store.relationship_types().into_iter().collect()
        } else {
            relationship_types.to_vec()
        };

        for rel_type in types.into_iter() {
            for prop_key in relationship_properties.iter() {
                if !graph_store.has_relationship_property(&rel_type, prop_key.as_str()) {
                    continue;
                }

                let mut type_filter: HashSet<RelationshipType> = HashSet::new();
                type_filter.insert(rel_type.clone());
                let mut selectors: HashMap<RelationshipType, String> = HashMap::new();
                selectors.insert(rel_type.clone(), prop_key.clone());

                let graph = graph_store
                    .get_graph_with_types_and_selectors(&type_filter, &selectors)
                    .map_err(|e| e.to_string())?;

                let value_type = graph_store
                    .relationship_property_type(prop_key.as_str())
                    .map_err(|e| e.to_string())?;

                let node_count = graph.node_count() as i64;
                for mapped_source in 0..node_count {
                    let original_source = graph
                        .to_original_node_id(mapped_source)
                        .unwrap_or(mapped_source);
                    for cursor in graph.stream_relationships(mapped_source, f64::NAN) {
                        let mapped_target = cursor.target_id();
                        let original_target = graph
                            .to_original_node_id(mapped_target)
                            .unwrap_or(mapped_target);
                        let pv = cursor.property();

                        let value = match value_type {
                            ValueType::Double => serde_json::Number::from_f64(pv)
                                .map(serde_json::Value::Number)
                                .unwrap_or(serde_json::Value::Null),
                            _ => serde_json::Value::Number(serde_json::Number::from(pv as i64)),
                        };

                        out.push(GraphStreamRelationshipPropertiesResult::new(
                            original_source,
                            original_target,
                            rel_type.name().to_string(),
                            prop_key.clone(),
                            value,
                        ));
                    }
                }
            }
        }

        Ok(out)
    }
}
