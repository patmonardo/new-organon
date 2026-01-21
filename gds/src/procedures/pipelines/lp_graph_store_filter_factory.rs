use std::collections::{HashMap, HashSet};

use crate::applications::services::logging::Log;
use crate::projection::eval::pipeline::link_pipeline::train::LinkPredictionTrainConfig;
use crate::projection::{NodeLabel, RelationshipType};
use crate::types::graph_store::{DefaultGraphStore, GraphStore};
use crate::types::schema::Direction;

use super::{LPGraphStoreFilter, LinkPredictionPredictPipelineConfig};

pub struct LPGraphStoreFilterFactory;

impl LPGraphStoreFilterFactory {
    pub fn generate(
        log: &Log,
        train_config: &LinkPredictionTrainConfig,
        predict_config: &dyn LinkPredictionPredictPipelineConfig,
        graph_store: &DefaultGraphStore,
    ) -> Result<LPGraphStoreFilter, String> {
        let source_node_labels = match predict_config.source_node_label() {
            Some(label) => vec![NodeLabel::of(label.to_string())],
            None => vec![NodeLabel::of(train_config.source_node_label().to_string())],
        };

        let target_node_labels = match predict_config.target_node_label() {
            Some(label) => vec![NodeLabel::of(label.to_string())],
            None => vec![NodeLabel::of(train_config.target_node_label().to_string())],
        };

        let predict_relationship_types: Vec<RelationshipType> =
            if !predict_config.relationship_types().is_empty() {
                predict_config
                    .relationship_types()
                    .iter()
                    .map(|t| RelationshipType::of(t.to_string()))
                    .collect()
            } else {
                vec![RelationshipType::of(
                    train_config.target_relationship_type().to_string(),
                )]
            };

        Self::validate_graph_filter(graph_store, &predict_relationship_types)?;

        let node_property_steps_base_labels = source_node_labels
            .iter()
            .chain(target_node_labels.iter())
            .cloned()
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();

        let filter = LPGraphStoreFilter::new(
            source_node_labels,
            target_node_labels,
            predict_relationship_types,
            node_property_steps_base_labels,
        );

        log.info(&format!(
            "The graph filters used for filtering in prediction is {:?}",
            filter
        ));

        Ok(filter)
    }

    fn validate_graph_filter(
        graph_store: &DefaultGraphStore,
        predicted_relationships: &[RelationshipType],
    ) -> Result<(), String> {
        let predicted_set: HashSet<RelationshipType> =
            predicted_relationships.iter().cloned().collect();
        let directions: HashMap<RelationshipType, Direction> =
            graph_store.schema().relationship_schema().directions();

        let directed: Vec<String> = directions
            .into_iter()
            .filter(|(rel_type, direction)| {
                predicted_set.contains(rel_type) && *direction != Direction::Undirected
            })
            .map(|(rel_type, _): (RelationshipType, Direction)| rel_type.name().to_string())
            .collect();

        if directed.is_empty() {
            return Ok(());
        }

        let predicted_names: Vec<String> = predicted_relationships
            .iter()
            .map(|rel| rel.name().to_string())
            .collect();

        Err(format!(
            "Procedure requires all relationships of {} to be UNDIRECTED, but found {} to be directed.",
            predicted_names.join(", "),
            directed.join(", ")
        ))
    }
}
