//! Embedding utilities for ML in GDS.
//!
//! Translated from Java GDS ml-core EmbeddingUtils.java.
//! This is a literal 1:1 translation following repository translation policy.

use crate::concurrency::Concurrency;
use crate::types::graph::Graph;

/// Retrieve a checked double array node property.
pub fn get_checked_double_array_node_property(
    graph: &dyn Graph,
    property_key: &str,
    node_id: u64,
) -> Vec<f64> {
    let values = graph.node_properties(property_key).unwrap_or_else(|| {
        panic!(
            "Missing node property `{}`. Consider using a default value in the property projection.",
            property_key
        )
    });

    values.double_array_value(node_id).unwrap_or_else(|e| {
        panic!(
            "Failed reading double array property `{}` for node {}: {}",
            property_key, node_id, e
        )
    })
}

/// Retrieve a checked long array node property.
pub fn get_checked_long_array_node_property(
    graph: &dyn Graph,
    property_key: &str,
    node_id: u64,
) -> Vec<i64> {
    let values = graph.node_properties(property_key).unwrap_or_else(|| {
        panic!(
            "Missing node property `{}`. Consider using a default value in the property projection.",
            property_key
        )
    });

    values.long_array_value(node_id).unwrap_or_else(|e| {
        panic!(
            "Failed reading long array property `{}` for node {}: {}",
            property_key, node_id, e
        )
    })
}

/// Retrieve a checked long array node property with expected length validation.
pub fn get_checked_long_array_node_property_with_length(
    graph: &dyn Graph,
    property_key: &str,
    node_id: u64,
    expected_length: usize,
) -> Vec<i64> {
    let property_value = get_checked_long_array_node_property(graph, property_key, node_id);
    if property_value.len() != expected_length {
        panic!(
            "The property `{}` contains arrays of differing lengths `{}` and `{}`.",
            property_key,
            property_value.len(),
            expected_length
        );
    }
    property_value
}

/// Validate relationship weight property values using the default validator.
pub fn validate_relationship_weight_property_value(graph: &dyn Graph, concurrency: Concurrency) {
    validate_relationship_weight_property_value_with_validator(
        graph,
        concurrency,
        |weight| weight.is_finite(),
        "Consider using `defaultValue` when loading the graph.",
    );
}

/// Validate relationship weight property values with a custom validator.
pub fn validate_relationship_weight_property_value_with_validator<F>(
    graph: &dyn Graph,
    _concurrency: Concurrency,
    validator: F,
    error_details: &str,
) where
    F: Fn(f64) -> bool + Send + Sync,
{
    // Single-threaded scan for now; hook into parallel utilities once available.
    let fallback = graph.default_property_value();
    for node_id in 0..graph.node_count() {
        for rel in graph.stream_relationships_weighted(node_id as i64, fallback) {
            let weight = rel.weight();
            if !validator(weight) {
                panic!(
                    "Invalid relationship weight `{}` on edge {} -> {}. {}",
                    weight,
                    rel.source_id(),
                    rel.target_id(),
                    error_details
                );
            }
        }
    }
}
